use prover::contexts::BatchContext;
use prover::provers;
use prover::provers::Prover;
use scheduler_service::scheduler_service_client::SchedulerServiceClient;
use scheduler_service::TakeBatchProofTaskResponse;
use scheduler_service::{batch_prover_message, scheduler_message, BatchProverMessage};
use scheduler_service::{BatchProofResult, Registry};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::async_trait;
use uuid::Uuid;

pub mod scheduler_service {
    tonic::include_proto!("scheduler.v1");
}

pub struct BatchProverService {
    addr: String,
    pub batch_prover_handler: Arc<dyn BatchProverHandler + Send + Sync>,
}

impl BatchProverService {
    pub fn new(
        addr: String,
        batch_prover_handler: Arc<dyn BatchProverHandler + Send + Sync>,
    ) -> Self {
        BatchProverService {
            addr,
            batch_prover_handler,
        }
    }

    pub async fn launch_service(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = SchedulerServiceClient::connect(self.addr.clone()).await?;

        let batch_prover_id = Uuid::new_v4().to_string();
        let batch_prover_name = format!("BatchProverService-{}", batch_prover_id);

        log::info!(
            "[Batch Prover Service: {}] connect to {}",
            batch_prover_name.clone(),
            self.addr.clone()
        );

        let (tx, rx) = mpsc::channel(10);

        // send request to registry service
        tx.send(BatchProverMessage {
            id: "".to_string(),
            message_type: Some(batch_prover_message::MessageType::Registry(Registry {
                prover_name: batch_prover_id,
                prover_id: batch_prover_name,
            })),
        })
        .await?;

        let request = ReceiverStream::new(rx);
        let response = (client.scheduler_stream(request)).await?;
        let mut stream = response.into_inner();
        while let Some(recv_msg) = stream.message().await? {
            if let Some(msg_type) = recv_msg.message_type {
                let send_msg = match msg_type {
                    scheduler_message::MessageType::TakeBatchProofTaskResponse(r) => {
                        self.batch_prover_handler
                            .handle_take_batch_proof_task_response(r)
                            .await
                    }
                };
                tx.send(send_msg).await?;
            }
        }

        Ok(())
    }
}

#[async_trait]
pub trait BatchProverHandler {
    async fn handle_take_batch_proof_task_response(
        &self,
        take_batch_proof_task_response: TakeBatchProofTaskResponse,
    ) -> BatchProverMessage;
}

#[derive(Default)]
pub struct BatchProverServiceHandler {}

#[async_trait]
impl BatchProverHandler for BatchProverServiceHandler {
    async fn handle_take_batch_proof_task_response(
        &self,
        take_batch_proof_task_response: TakeBatchProofTaskResponse,
    ) -> BatchProverMessage {
        let ctx = serde_json::from_slice::<BatchContext>(
            &take_batch_proof_task_response
                .batch_context_bytes
                .unwrap()
                .data,
        )
        .unwrap();
        // TODO: async service execution and return immediately
        // or block until service finish?
        log::debug!(
            "[batch-prover] handles task: {}-{}",
            ctx.task_id,
            ctx.chunk_id
        );
        match provers::BatchProver::new().prove(&ctx) {
            Ok(_) => {
                log::info!("batch prove success, task id: {}", ctx.task_id.clone());
                // Return Result and Trigger next task
                BatchProverMessage {
                    id: "".to_string(),
                    message_type: Some(batch_prover_message::MessageType::BatchProofResult(
                        BatchProofResult {
                            prover_id: take_batch_proof_task_response.prover_id,
                            task_id: ctx.task_id.clone(),
                            chunk_id: ctx.chunk_id.clone(),
                            result: 1,
                        },
                    )),
                }
            }
            Err(e) => {
                log::error!("batch prove({}) error: {:?}", ctx.task_id, e);
                // Return a failure message
                BatchProverMessage {
                    id: "".to_string(),
                    message_type: Some(batch_prover_message::MessageType::BatchProofResult(
                        BatchProofResult {
                            prover_id: take_batch_proof_task_response.prover_id,
                            task_id: ctx.task_id.clone(),
                            chunk_id: ctx.chunk_id.clone(),
                            result: 0, // Indicate failure
                        },
                    )),
                }
            }
        }
        // TODO: save checkpoint
        // save_checkpoint()
    }
}
