use std::collections::HashMap;
// use std::sync::Mutex;
use super::event::Event;
use crate::contexts::BatchContext;
use crate::scheduler::{AddServiceResult, ProofResult, TakeTaskResult};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

pub struct Scheduler {
    // Service <-> Scheduler
    pub service_table: HashMap<ServiceId, Service>,

    pub event_receiver: mpsc::Receiver<Event>,

    // Pipeline <-> Scheduler
    // Pipeline send task to the channel
    pub task_receiver: mpsc::Receiver<BatchContext>,
    pub retry_to: Sender<BatchContext>,
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
        event_receiver: mpsc::Receiver<Event>,
        task_receiver: mpsc::Receiver<BatchContext>,
        retry_to: Sender<BatchContext>,
    ) -> Self {
        Scheduler {
            service_table: HashMap::new(),
            event_receiver,
            task_receiver,
            retry_to,
        }
    }

    pub async fn run(&mut self) {
        loop {
            match self.event_receiver.recv().await {
                Some(event) => match event {
                    Event::AddService {
                        service_id,
                        relay_to,
                    } => {
                        self.handle_add_service(service_id, relay_to).await;
                    }
                    Event::RemoveService { service_id } => {
                        self.handle_remove_service(service_id).await;
                    }
                    Event::TakeTask {
                        service_id,
                        relay_to,
                    } => {
                        self.handle_take_task(service_id, relay_to).await;
                    }
                    Event::TaskResult {
                        service_id,
                        recursive_proof,
                    } => {
                        self.handle_task_result(service_id, recursive_proof).await;
                    }
                },
                None => {
                    log::error!("Failed to receive event, all sender have been dropped, exit the scheduler loop");
                }
            }
        }
    }

    pub async fn take_task(&mut self) -> Option<BatchContext> {
        self.task_receiver.recv().await
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
            status: ServiceStatus::Idle,
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
    ) {
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
            if let Some(task) = self.task_receiver.recv().await {
                // record the task, so that we can retry it when an error occurs during the proof generation
                service.status = ServiceStatus::Running;
                service.current_task_id = Some(task.task_id.clone());
                service.current_task = Some(task.clone());
                if let Err(e) = relay_to.send(TakeTaskResult::Success(task.clone())).await {
                    // retry the task
                    service.status = ServiceStatus::Idle;
                    service.current_task_id = None;
                    service.current_task = None;
                    if let Err(e) = self.retry_to.send(task).await {
                        // TODO: in the case, is it allowed to discard the task?
                        log::error!(
                            "Failed to retry task: {}, the task will be discarded here, err: {}",
                            service_id,
                            e
                        );
                    }
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
    }

    pub async fn handle_remove_service(&mut self, service_id: ServiceId) {
        self.service_table.remove(&service_id);
    }

    pub async fn handle_task_result(
        &mut self,
        service_id: ServiceId,
        _recursive_proof: ProofResult,
    ) {
        if let Some(service) = self.service_table.get_mut(&service_id) {
            service.status = ServiceStatus::Idle;
            service.current_task = None;
            service.current_task_id = None;
        }
        // TODO: save checkpoint here or batch_prover client?
        // TODO: save_checkpoint
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
