use crate::scheduler::scheduler::ServiceId;
use prover_core::contexts::BatchContext;
use tokio::sync::mpsc::Sender;

/// Event is used to communicate between scheduler and scheduler_server
#[derive(Debug, Clone)]
pub enum Event {
    /// AddService is used to add a service to the scheduler
    AddService {
        /// service_id is the id of the service that used to generate batch proof
        service_id: ServiceId,
        /// relay_to is the channel that used to send the AddServiceResult back to the service
        /// service will wait for the result on the other side of the channel
        relay_to: Sender<AddServiceResult>,
    },

    /// Used to remove the service from the scheduler
    RemoveService { service_id: ServiceId },

    /// Used to take a task from the scheduler and wait for the result
    /// currently, there is only one type of task: batch_proof
    TakeTask { service_id: ServiceId, relay_to: Sender<TakeTaskResult> },
    // Used to send the proof result to the scheduler
    // TaskResult {
    //     service_id: ServiceId,
    //     recursive_proof: ProofResult,
    // },
}

#[derive(Debug, Default, Clone)]
pub struct TaskResult {
    pub service_id: ServiceId,
    pub recursive_proof: ProofResult,
}

#[derive(Debug, Clone, Default)]
pub struct ProofResult {
    pub task_id: String,
    pub result_code: ResultStatus,
}

#[derive(Debug, Clone, Default)]
pub enum ResultStatus {
    #[default]
    Success,
    Fail,
}

pub enum AddServiceResult {
    Success(ServiceId),
    Fail(ServiceId),
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub enum TakeTaskResult {
    Success(BatchContext),
    Fail(ServiceId),
}
