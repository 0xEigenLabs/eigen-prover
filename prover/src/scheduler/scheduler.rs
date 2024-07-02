use super::event::{Event, ResultStatus, TaskResult};
use crate::contexts::BatchContext;
use crate::scheduler::{AddServiceResult, ProofResult, TakeTaskResult};
use crate::stage::Stage;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, Mutex as TokioMutex};

pub struct Scheduler {
    pub service_table: HashMap<ServiceId, Service>,

    // it's concurrency safe
    // handle_take_task will put the task into the pending_results
    // handle_task_result will take the task from the pending_results
    // but they are in different branches of the same select
    pub pending_results: HashMap<ServiceId, BatchContext>,

    pub retry_to: Sender<BatchContext>,

    pub event_handler: Option<EventHandler>,
    pub result_handler: ResultHandler,
}

pub struct EventHandler {
    // Service <-> Scheduler
    pub event_receiver: Arc<TokioMutex<mpsc::Receiver<Event>>>,
    // Pipeline <-> Scheduler
    // Pipeline send task to the channel
    pub task_receiver: Arc<TokioMutex<mpsc::Receiver<BatchContext>>>,
    pub retry_to: Sender<BatchContext>,
}

impl EventHandler {
    fn new(
        event_receiver: Arc<TokioMutex<mpsc::Receiver<Event>>>,
        task_receiver: Arc<TokioMutex<mpsc::Receiver<BatchContext>>>,
        retry_to: Sender<BatchContext>,
    ) -> Self {
        EventHandler {
            event_receiver,
            task_receiver,
            retry_to,
        }
    }
    pub async fn recv_both(&self) -> (Event, BatchContext) {
        let event = self.event_receiver.lock().await.recv().await.unwrap();
        match event {
            Event::TakeTask { .. } => {
                // wait for the task
                log::info!("[scheduler] wait for task");
                let task = self.task_receiver.lock().await.recv().await.unwrap();
                (event, task)
            }
            // don't need task
            Event::AddService { .. } | Event::RemoveService { .. } => {
                (event, BatchContext::default())
            }
        }
    }
}

pub struct ResultHandler {
    pub result_receiver: Arc<TokioMutex<mpsc::Receiver<TaskResult>>>,
    pub retry_to: Sender<BatchContext>,
}

impl ResultHandler {
    fn new(
        result_receiver: Arc<TokioMutex<mpsc::Receiver<TaskResult>>>,
        retry_to: Sender<BatchContext>,
    ) -> Self {
        ResultHandler {
            result_receiver,
            retry_to,
        }
    }

    pub async fn take_result(&self) -> TaskResult {
        self.result_receiver.lock().await.recv().await.unwrap()
    }
}

pub type ServiceId = String;

#[derive(Clone)]
pub struct Service {
    pub service_id: String,
    pub task_name: Option<String>,
    pub service_type: BatchProver,
    pub status: ServiceStatus,
    pub current_task_id: Option<String>,
    pub current_task: Option<BatchContext>,

    // Service will send batch_proof result to this channel
    pub proof_receiver: Option<Arc<mpsc::Receiver<BatchContext>>>,
}

impl Scheduler {
    pub fn new(
        result_receiver: mpsc::Receiver<TaskResult>,
        event_receiver: mpsc::Receiver<Event>,
        task_receiver: mpsc::Receiver<BatchContext>,
        retry_to: Sender<BatchContext>,
    ) -> Self {
        Scheduler {
            service_table: HashMap::new(),
            retry_to: retry_to.clone(),
            pending_results: Default::default(),
            result_handler: ResultHandler::new(
                Arc::new(TokioMutex::new(result_receiver)),
                retry_to.clone(),
            ),
            event_handler: Some(EventHandler::new(
                Arc::new(TokioMutex::new(event_receiver)),
                Arc::new(TokioMutex::new(task_receiver)),
                retry_to.clone(),
            )),
        }
    }

    // pub async fn run(&mut self) {
    //     loop {
    //         tokio::select! {
    //             // listen the event from the scheduler serve and task from the pipeline
    //             (event, ctx) = self.event_handler.recv_both()  => {
    //                      self.handle_event(event, ctx).await;
    //                  },
    //             // listen the result from the scheduler server
    //             result = self.result_handler.take_result() => {
    //                 self.handle_result(result).await;
    //             },
    //         }
    //     }
    // }
    pub async fn run(&mut self) {
        let (interval_event_tx, mut interval_event_rx) =
            mpsc::channel::<(Event, BatchContext)>(128);
        // take the event_handler, and spawn a new coroutine to listen the event from the event_receiver and task_receiver
        // the self.event_handler will be None after this
        let event_handler = self.event_handler.take().unwrap();

        tokio::spawn(async move {
            loop {
                let (event, ctx) = event_handler.recv_both().await;
                if let Err(e) = interval_event_tx.send((event, ctx)).await {
                    log::error!("Failed to send event to interval_event_tx, err: {}", e);
                }
            }
        });

        loop {
            tokio::select! {
                // listen the event from the scheduler serve and task from the pipeline
                interval_event = interval_event_rx.recv() => {
                    let (event, ctx) = interval_event.unwrap();
                    self.handle_event(event, ctx).await;
                },
                // listen the result from the scheduler server
                result = self.result_handler.take_result() => {
                    self.handle_result(result).await;
                },
            }
        }
    }
    pub async fn handle_event(&mut self, event: Event, ctx: BatchContext) {
        match event {
            Event::AddService {
                service_id,
                relay_to,
            } => {
                log::info!("[scheduler] add service: {}", service_id);
                self.handle_add_service(service_id, relay_to).await
            }
            Event::RemoveService { service_id } => {
                log::info!("[scheduler] remove service: {}", service_id);
                self.handle_remove_service(service_id).await
            }
            Event::TakeTask {
                service_id,
                relay_to,
            } => {
                log::info!("[scheduler] [service:{}] take a task", service_id);
                self.handle_take_task(service_id, relay_to, ctx).await
            }
        }
    }

    pub async fn handle_result(&mut self, result: TaskResult) {
        log::info!(
            "[service:{}] task result: {:?}",
            &result.service_id,
            &result.recursive_proof
        );
        self.handle_task_result(result.service_id, result.recursive_proof)
            .await;
    }

    pub async fn handle_add_service(
        &mut self,
        service_id: ServiceId,
        relay_to: Sender<AddServiceResult>,
    ) {
        let new_service = Service {
            service_id: service_id.clone(),
            task_name: None,
            service_type: BatchProver::GRPC,
            status: ServiceStatus::Prepare,
            current_task_id: None,
            current_task: None,
            proof_receiver: None,
        };
        self.service_table.insert(service_id.clone(), new_service);

        if let Err(e) = relay_to
            .send(AddServiceResult::Success(service_id.clone()))
            .await
        {
            log::error!("Failed to add service: {}, err: {}", service_id, e);
            self.service_table.remove(&service_id);
        }
    }

    pub async fn handle_take_task(
        &mut self,
        service_id: ServiceId,
        relay_to: Sender<TakeTaskResult>,
        task: BatchContext,
    ) {
        // record the task, so that we can retry it when an error occurs during the proof generation
        if let Err(e) = relay_to.send(TakeTaskResult::Success(task.clone())).await {
            if let Err(e) = self.retry_to.send(task.clone()).await {
                // TODO: in the case, is it allowed to discard the task?
                log::error!(
                    "Failed to retry task: {}, the task will be discarded here, err: {}",
                    service_id,
                    e
                );
            }

            // retry the task
            log::error!("Failed to take task: {}, err: {}", service_id, e);
        }

        // put task to pending_results
        log::info!(
            "put task to pending_results: {}, task: {:?}",
            service_id,
            task.clone()
        );
        let task_key = self.construct_task_key(&task.task_id, &task.chunk_id);
        self.pending_results.insert(task_key, task.clone());

        /*
        if let Some(service) = self.service_table.get_mut(&service_id) {
            if service.status != ServiceStatus::Idle {
                // the service is not idle, can't take task
                if let Err(e) = relay_to
                    .send(TakeTaskResult::Fail(service_id.clone()))
                    .await
                {
                    log::error!("Failed to take task: {}, err: {}", service_id, e);
                }
            }
            } else {
            // the service has not been added to the scheduler, please add it first
            if let Err(e) = relay_to
                .send(TakeTaskResult::Fail(service_id.clone()))
                .await
            {
                log::error!("Failed to take task: {}, err: {}", service_id, e);
            }
        }
         */
    }

    pub async fn handle_remove_service(&mut self, service_id: ServiceId) {
        self.service_table.remove(&service_id);
    }

    pub async fn handle_task_result(
        &mut self,
        service_id: ServiceId,
        recursive_proof_result: ProofResult,
    ) {
        let finished = match recursive_proof_result.result_code {
            ResultStatus::Success => true,
            ResultStatus::Fail => false,
        };

        let key = self.construct_task_key(
            &recursive_proof_result.task_id,
            &recursive_proof_result.chunk_id,
        );
        if let Some(task_ctx) = self.pending_results.remove(&key) {
            let task_stage =
                Stage::Batch(task_ctx.task_id, task_ctx.chunk_id, task_ctx.l2_batch_data);
            let workdir = Path::new(&task_ctx.basedir).join(task_stage.path());

            log::info!("save_checkpoint, mkdir: {:?}", workdir);
            if let Err(e) = std::fs::create_dir_all(workdir.clone()) {
                log::error!("Failed to create checkpoint dir: {:?}, err: {}", workdir, e);
                return;
            }

            if !finished {
                let p = workdir.join("status");
                let stage_str = match task_stage.to_string() {
                    Ok(str) => str,
                    Err(e) => {
                        log::error!("Failed to serialize stage: {:?}, err: {}", task_stage, e);
                        return;
                    }
                };

                if let Err(e) = std::fs::write(p.clone(), stage_str) {
                    log::error!("Failed to write status file: {:?}, err: {}", p, e);
                    return;
                }
            }

            let p = workdir.join("status.finished");
            let proof_result = if finished { "1" } else { "0" };
            log::info!(
                "batch proof finished! save_checkpoint with result: {}, dir: {:?}",
                proof_result,
                workdir
            );
            if let Err(e) = std::fs::write(p.clone(), proof_result) {
                log::error!("Failed to write status.finished file: {:?}, err: {}", p, e);
            }
        } else {
            log::error!(
                "Failed to find task in pending_results, discard the result from service: {}",
                service_id
            );
        }
    }

    fn construct_task_key(&self, task_id: &String, chunk_id: &String) -> String {
        format!("{}_{}", task_id, chunk_id)
    }
}
#[derive(Clone)]
pub enum BatchProver {
    InMemory,
    GRPC,
}

#[derive(Clone, Eq, PartialEq)]
pub enum ServiceStatus {
    Prepare,
    Idle,
    Running,
    Exit,
}
