use anyhow::anyhow;
use prover::scheduler::{Event, TaskResult};
use prover_core::contexts::BatchContext;
use prover_scheduler::scheduler_server::scheduler_service::{
    scheduler_message as server_scheduler_message, BatchContextBytes as ServerBatchContextBytes,
    BatchProofResult as ServerBatchProofResult, Registry as ServerRegistry,
    SchedulerMessage as ServerSchedulerMessage,
    TakeBatchProofTaskResponse as ServerTakeBatchProofTaskResponse,
};
use prover_scheduler::scheduler_server::{SchedulerHandler, SchedulerServiceSVC};
use std::env;

use prover_scheduler::service::batch_prover_service::scheduler_service::{
    batch_prover_message as client_batch_prover_message,
    BatchProofResult as ClientBatchProofResult, BatchProverMessage as ClientBatchProverMessage,
    TakeBatchProofTaskResponse as ClientTakeBatchProofTaskResponse,
};
use prover_scheduler::service::batch_prover_service::{BatchProverHandler, BatchProverService};

use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tonic::async_trait;

#[tokio::test]
async fn scheduler_e2e_test() {
    // init log
    env::set_var("RUST_LOG", "info");
    env::set_var("PROVER_MODEL", "grpc");

    env_logger::try_init().unwrap_or_default();
    // Start the server
    log::info!("====================1. Start the server====================");
    let (scheduler_sender, _rx) = tokio::sync::mpsc::channel(100);
    let (result_sender, _rx) = tokio::sync::mpsc::channel(100);
    // MOCK ServerHandler to test
    let scheduler_handler = Arc::new(MockSchedulerServerHandler::default());
    let scheduler_service_svc =
        SchedulerServiceSVC::new(scheduler_sender, result_sender, scheduler_handler.clone());

    // [::1]:50051
    let addr = "[::1]:50051".to_string();
    let server_future = tokio::spawn(async move {
        scheduler_service_svc.launch_server(addr).await.unwrap();
    });

    // Give the server a little time to start
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    log::info!("====================2. Start service 1 ====================");
    // Run the batch_prover_service1
    let client_future1 = tokio::spawn(async move {
        let addr = "http://[::1]:50051".to_string();
        let batch_prover_handler = Arc::new(MockBatchProverHandler::default());
        let batch_prover_service = BatchProverService::new(addr, batch_prover_handler);
        batch_prover_service.launch_service().await.unwrap();
    });

    log::info!("====================3. Start service 2 ====================");
    // Run the batch_prover_service2
    let client_future2 = tokio::spawn(async move {
        let addr = "http://[::1]:50051".to_string();
        let batch_prover_handler = Arc::new(MockBatchProverHandler::default());
        let batch_prover_service = BatchProverService::new(addr, batch_prover_handler);
        batch_prover_service.launch_service().await.unwrap();
    });

    log::info!("====================4. Start service 3 ====================");
    // Run the batch_prover_service3
    let client_future3 = tokio::spawn(async move {
        let addr = "http://[::1]:50051".to_string();
        let batch_prover_handler = Arc::new(MockBatchProverHandler::default());
        let batch_prover_service = BatchProverService::new(addr, batch_prover_handler);
        batch_prover_service.launch_service().await.unwrap();
    });

    log::info!("====================5. Running... ====================");
    let client_result1 = client_future1.await;
    assert!(client_result1.is_ok(), "Client 1 encountered an error");
    let client_result2 = client_future2.await;
    assert!(client_result2.is_ok(), "Client 2 encountered an error");
    let client_result3 = client_future3.await;
    assert!(client_result3.is_ok(), "Client 3 encountered an error");

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Wait for the client to finish
    server_future.abort();
    log::info!("====================6. Done ====================");
}

#[derive(Default, Clone)]
pub struct MockSchedulerServerHandler {}

#[async_trait]
impl SchedulerHandler for MockSchedulerServerHandler {
    async fn handle_batch_prover_registry(
        &self,
        r: ServerRegistry,
        _scheduler_sender: Sender<Event>,
    ) -> anyhow::Result<ServerSchedulerMessage> {
        // here we don't need to send message to scheduler_sender,
        let basedir = "/tmp";
        let task_id = "task_id_1";
        let task_name = "task_name_1";
        let l2_batch_data = std::fs::read_to_string(
            env::var("SUITE_JSON")
                .unwrap_or("../executor/test-vectors/solidityExample.json".to_string()),
        )
        .unwrap();
        let first_task = BatchContext::new(
            basedir,
            task_id,
            task_name,
            l2_batch_data,
            std::env!("EVM_ELF_DATA"),
        );
        // just test the server and client communication
        // we will test the message sending in lib prover
        log::info!(
            "[Scheduler Server] receive the registry msg {:?}, from [Prover Service: {}]",
            r,
            r.prover_name.clone()
        );

        Ok(ServerSchedulerMessage {
            id: "".into(),
            message_type: Some(server_scheduler_message::MessageType::TakeBatchProofTaskResponse(
                ServerTakeBatchProofTaskResponse {
                    prover_id: r.prover_id.clone(),
                    batch_context_bytes: Some(ServerBatchContextBytes {
                        data: serde_json::to_vec(&first_task).unwrap(),
                    }),
                },
            )),
        })
    }

    async fn handle_gen_batch_proof_response(
        &self,
        _provider_id: String,
        _scheduler_sender: Sender<Event>,
    ) -> anyhow::Result<ServerSchedulerMessage> {
        // here we don't need to send message to scheduler_sender,
        todo!()
    }

    async fn handle_get_proof_response(
        &self,
        r: ServerBatchProofResult,
        _scheduler_sender: Sender<Event>,
        _result_sender: Sender<TaskResult>,
    ) -> anyhow::Result<ServerSchedulerMessage> {
        log::info!(
            "[Scheduler Server] receive the proof result msg {:?}, from [Prover Service: {}]",
            r,
            r.prover_id.clone()
        );
        log::info!("[Scheduler Server] finished! close the connection...");
        return Err(anyhow!("close the connection"));
    }

    async fn remove_service(&self, _service_id: String, _scheduler_sender: Sender<Event>) {
        todo!()
    }
}

#[derive(Default, Clone)]
pub struct MockBatchProverHandler {}

#[async_trait]
impl BatchProverHandler for MockBatchProverHandler {
    async fn handle_take_batch_proof_task_response(
        &self,
        take_batch_proof_task_response: ClientTakeBatchProofTaskResponse,
    ) -> ClientBatchProverMessage {
        let ctx = serde_json::from_slice::<BatchContext>(
            &take_batch_proof_task_response.batch_context_bytes.unwrap().data,
        )
        .unwrap();

        log::info!(
            "[Batch Prover Service: {}] receive the task {:?} from [Scheduler Server]]",
            take_batch_proof_task_response.prover_id.clone(),
            ctx.clone()
        );
        log::info!(
            "[Batch Prover Service: {}] done...",
            take_batch_proof_task_response.prover_id.clone()
        );
        ClientBatchProverMessage {
            id: "".to_string(),
            message_type: Some(client_batch_prover_message::MessageType::BatchProofResult(
                ClientBatchProofResult {
                    prover_id: take_batch_proof_task_response.prover_id,
                    task_id: ctx.task_id.clone(),
                    result: 1,
                },
            )),
        }
    }
}
