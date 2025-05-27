use anyhow::bail;
use anyhow::Result;
use prover::scheduler::{
    AddServiceResult, Event, ProofResult, ResultStatus, TakeTaskResult, TaskResult,
};
use scheduler_service::scheduler_service_server::SchedulerService;
use scheduler_service::scheduler_service_server::SchedulerServiceServer;
use scheduler_service::{
    batch_prover_message, scheduler_message, BatchProofResult, BatchProverMessage, Registry,
    SchedulerMessage,
};
use scheduler_service::{BatchContextBytes, TakeBatchProofTaskResponse};
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::{Stream, StreamExt};
use tonic::transport::Server;
use tonic::{async_trait, Request, Response, Status, Streaming};

// TODO: rename
#[allow(clippy::module_inception)]
pub mod scheduler_service {
    tonic::include_proto!("scheduler.v1");
}

#[allow(dead_code)]
pub struct SchedulerServiceSVC {
    scheduler_sender: mpsc::Sender<Event>,
    result_sender: mpsc::Sender<TaskResult>,
    handler: Arc<dyn SchedulerHandler + Send + Sync>,
}

impl SchedulerServiceSVC {
    pub fn new(
        scheduler_sender: mpsc::Sender<Event>,
        result_sender: mpsc::Sender<TaskResult>,
        handler: Arc<dyn SchedulerHandler + Send + Sync>,
    ) -> Self {
        SchedulerServiceSVC { scheduler_sender, result_sender, handler }
    }

    pub async fn launch_server(&self, addr: String) -> Result<(), Box<dyn std::error::Error>> {
        let socket_addr = addr.as_str().parse()?;
        log::info!("[Scheduler Server] listening on {}", socket_addr);
        let svc = SchedulerServiceSVC::new(
            self.scheduler_sender.clone(),
            self.result_sender.clone(),
            self.handler.clone(),
        );
        Server::builder().add_service(SchedulerServiceServer::new(svc)).serve(socket_addr).await?;
        Ok(())
    }
}

#[tonic::async_trait]
impl SchedulerService for SchedulerServiceSVC {
    type SchedulerStreamStream =
        Pin<Box<dyn Stream<Item = Result<SchedulerMessage, Status>> + Send + Sync + 'static>>;

    async fn scheduler_stream(
        &self,
        request: Request<Streaming<BatchProverMessage>>,
    ) -> Result<Response<Self::SchedulerStreamStream>, Status> {
        let mut stream = request.into_inner();
        let (tx, rx) = mpsc::channel(10);
        let scheduler_sender = self.scheduler_sender.clone();
        let result_sender = self.result_sender.clone();
        let handle_clone = self.handler.clone();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(result) = stream.next() => {
                        match result {
                            Ok(batch_prover_msg) => {
                                if let Some(msg) = batch_prover_msg.message_type {
                                    let resp = match msg {
                                        // update pb, we don't need too much information
                                        batch_prover_message::MessageType::Registry(r) => {
                                            log::debug!("[scheduler] register batch prover: {:?}", r);
                                            if let Ok(schdeduler_msg) = handle_clone.handle_batch_prover_registry(r, scheduler_sender.clone()).await {
                                                schdeduler_msg
                                            } else {
                                                // close the connection
                                                break;
                                            }
                                        }
                                        // update pb, we don't need GeneBatchProofResponse
                                        // just wait for the result, don't need to get again
                                        batch_prover_message::MessageType::TakeBatchProofTask(r) => {
                                            // TODO: id
                                            log::debug!("[scheduler] take batch proof: {:?}", r);
                                            if let Ok(scheduler_msg) = handle_clone.handle_gen_batch_proof_response(r.prover_id.clone(), scheduler_sender.clone()).await {
                                                scheduler_msg
                                            } else {
                                                // close the connection
                                                break;
                                            }
                                        }
                                        // receive proof, trigger next batch_proof task
                                        batch_prover_message::MessageType::BatchProofResult(r) => {
                                            log::debug!("[scheduler] return proof: {:?}", r);
                                            if let Ok(scheduler_msg) = handle_clone.handle_get_proof_response(r, scheduler_sender.clone(), result_sender.clone()).await {
                                                scheduler_msg
                                            } else {
                                                // close the connection
                                                break;
                                            }
                                        }
                                    };

                                    if let Err(e) = tx.send(Ok(resp)).await {
                                        log::error!("Failed to send message: {}", e);
                                        break;
                                    }
                                }
                            }
                            Err(e) => {
                                // some error occurred
                                // now, we choose to close the connection
                                // TODO: process according to the status code, eg. retry, close, etc.
                                // send Event::Shutdown to the scheduler, wait for the event result
                                // exit the loop
                                log::error!("Failed to receive message, close: {}", e);
                                // try to send error_message to client?
                                break;
                            }
                        }

                    }
                    else => {
                        // client already closed the connection
                        // send Event::Shutdown to the scheduler,
                        // to notify the scheduler to remove the service
                        // wait for the event result
                        // exit the loop
                        // TODO: put Event::Shutdown ant wait
                        break;
                    }
                }
            }
        });

        Ok(Response::new(Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx))))
    }
}

#[async_trait]
pub trait SchedulerHandler {
    async fn handle_batch_prover_registry(
        &self,
        r: Registry,
        scheduler_sender: mpsc::Sender<Event>,
    ) -> Result<SchedulerMessage>;
    async fn handle_gen_batch_proof_response(
        &self,
        provider_id: String,
        scheduler_sender: mpsc::Sender<Event>,
    ) -> Result<SchedulerMessage>;

    async fn handle_get_proof_response(
        &self,
        r: BatchProofResult,
        scheduler_sender: mpsc::Sender<Event>,
        result_sender: mpsc::Sender<TaskResult>,
    ) -> Result<SchedulerMessage>;

    async fn remove_service(&self, service_id: String, scheduler_sender: mpsc::Sender<Event>);
}

#[derive(Default)]
pub struct SchedulerServerHandler {}

#[async_trait]
impl SchedulerHandler for SchedulerServerHandler {
    async fn handle_batch_prover_registry(
        &self,
        r: Registry,
        scheduler_sender: mpsc::Sender<Event>,
    ) -> Result<SchedulerMessage> {
        // send Event::AddService to the scheduler, registry the service to the scheduler
        // wait for the event result from the relay channel
        let (relay_to, mut relay) = mpsc::channel::<AddServiceResult>(1);
        let event = Event::AddService { service_id: r.prover_id.clone(), relay_to };
        if let Err(e) = scheduler_sender.send(event.clone()).await {
            // can't send event to scheduler, close the connection
            log::error!("Failed to send Event: {:?}, receiver dropped: {}", event, e);
            bail!("Failed to send Event: {:?}, receiver dropped: {}", event, e)
        }

        if let Some(add_service_result) = relay.recv().await {
            match add_service_result {
                AddServiceResult::Success(_service_id) => {
                    if let Ok(scheduler_msg) =
                        self.handle_gen_batch_proof_response(r.prover_id, scheduler_sender).await
                    {
                        Ok(scheduler_msg)
                    } else {
                        // close the connection
                        bail!("Failed to handle GenBatchProofResponse")
                    }
                }
                AddServiceResult::Fail(service_id) => {
                    // close the connection
                    bail!("Failed to add service: {}", service_id)
                }
            }
        } else {
            // channel closed
            bail!("Failed to receive AddServiceResult, channel closed")
        }
    }

    async fn handle_gen_batch_proof_response(
        &self,
        provider_id: String,
        scheduler_sender: mpsc::Sender<Event>,
    ) -> Result<SchedulerMessage> {
        let (relay_to, mut relay) = mpsc::channel::<TakeTaskResult>(1);
        // then, send Event::TriggerTask to the scheduler
        let event = Event::TakeTask { service_id: provider_id.clone(), relay_to };

        if let Err(e) = scheduler_sender.send(event.clone()).await {
            // can't send event to scheduler, close the connection
            log::error!("Failed to send Event: {:?}, receiver dropped: {}", event, e);
            bail!("Failed to send Event: {:?}, receiver dropped: {}", event, e)
        }

        // wait for the event result
        if let Some(take_task_result) = relay.recv().await {
            match take_task_result {
                TakeTaskResult::Success(batch_ctx) => {
                    Ok(SchedulerMessage {
                        // TODO: received id
                        id: "".into(),
                        message_type: Some(
                            // TODO: impl into trait for BatchContext?
                            scheduler_message::MessageType::TakeBatchProofTaskResponse(
                                TakeBatchProofTaskResponse {
                                    prover_id: provider_id.clone(),
                                    batch_context_bytes: Some(BatchContextBytes {
                                        data: serde_cbor::to_vec(&batch_ctx).unwrap(),
                                    }),
                                },
                            ),
                        ),
                    })
                }
                TakeTaskResult::Fail(service_id) => {
                    bail!("Failed to take task for service: {}", service_id)
                }
            }
        } else {
            // channel closed
            bail!("Failed to receive TakeTaskResult, channel closed")
        }
    }

    async fn handle_get_proof_response(
        &self,
        r: BatchProofResult,
        scheduler_sender: mpsc::Sender<Event>,
        result_sender: mpsc::Sender<TaskResult>,
    ) -> Result<SchedulerMessage> {
        let task_result = if r.result == scheduler_service::Result::Ok as i32 {
            TaskResult {
                service_id: r.prover_id.clone(),
                recursive_proof: ProofResult {
                    task_id: r.task_id.clone(),
                    result_code: ResultStatus::Success,
                },
            }
        } else {
            TaskResult {
                service_id: r.prover_id.clone(),
                recursive_proof: ProofResult {
                    task_id: r.task_id.clone(),
                    result_code: ResultStatus::Fail,
                },
            }
        };

        if let Err(e) = result_sender.send(task_result.clone()).await {
            // can't send event to scheduler, close the connection
            log::error!("Failed to send Event: {:?}, receiver dropped: {}", task_result, e);
            bail!("Failed to send Event: {:?}, receiver dropped: {}", task_result, e)
        }

        if let Ok(scheduler_msg) =
            self.handle_gen_batch_proof_response(r.prover_id, scheduler_sender).await
        {
            Ok(scheduler_msg)
        } else {
            // close the connection
            bail!("Failed to handle GenBatchProofResponse")
        }
    }

    #[allow(dead_code)]
    async fn remove_service(&self, service_id: String, scheduler_sender: mpsc::Sender<Event>) {
        // send Event::RemoveService to the scheduler, remove the service from the scheduler
        let event = Event::RemoveService { service_id: service_id.clone() };
        if let Err(e) = scheduler_sender.send(event.clone()).await {
            // can't send event to scheduler, close the connection
            log::error!("Failed to send Event: {:?}, receiver dropped: {}", event, e);
        }
    }
}
