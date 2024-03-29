use prover::contexts::BatchContext;
use prover::provers;
use prover::provers::Prover;
use scheduler_service::scheduler_service_client::SchedulerServiceClient;
use scheduler_service::{batch_prover_message, scheduler_message, BatchProverMessage};
use scheduler_service::{BatchProofResult, Registry};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use uuid::Uuid;

pub mod scheduler_service {
    tonic::include_proto!("scheduler.v1");
}

pub async fn launch_batch_prover_service_with_addr(
    addr: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SchedulerServiceClient::connect(addr).await?;
    let batch_prover_id = Uuid::new_v4().to_string();
    let batch_prover_name = format!("BatchProverService-{}", batch_prover_id);

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
    let response = client.scheduler_stream(request).await?;
    let mut stream = response.into_inner();
    while let Some(recv_msg) = stream.message().await? {
        if let Some(msg_type) = recv_msg.message_type {
            let send_msg = match msg_type {
                scheduler_message::MessageType::TakeBatchProofTaskResponse(r) => {
                    let ctx = serde_json::from_slice::<BatchContext>(
                        &r.batch_context_bytes.unwrap().data,
                    )
                    .unwrap();
                    // TODO: async service execution and return immediately
                    // or block until service finish?
                    provers::BatchProver::new().prove(&ctx)?;
                    // TODO: save checkpoint
                    // save_checkpoint()

                    // Return Result and Trigger next task
                    BatchProverMessage {
                        id: "".to_string(),
                        message_type: Some(batch_prover_message::MessageType::BatchProofResult(
                            BatchProofResult {
                                prover_id: r.prover_id,
                                result: 1,
                            },
                        )),
                    }
                }
            };
            tx.send(send_msg).await?;
        }
    }

    Ok(())
}
