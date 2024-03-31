use prover::contexts::BatchContext;
use std::env;

use prover_scheduler::scheduler_server::{SchedulerServerHandler, SchedulerServiceSVC};

use prover_scheduler::service::batch_prover_service::scheduler_service::{
    batch_prover_message as client_batch_prover_message,
    BatchProofResult as ClientBatchProofResult, BatchProverMessage as ClientBatchProverMessage,
    TakeBatchProofTaskResponse as ClientTakeBatchProofTaskResponse,
};
use prover_scheduler::service::batch_prover_service::{BatchProverHandler, BatchProverService};

use prover::pipeline::Pipeline;
use prover::scheduler::Scheduler;
use std::sync::Arc;
use tonic::async_trait;

#[tokio::test]
async fn prover_scheduler_e2e_mock_test() {
    // init log
    env::set_var("RUST_LOG", "info");
    env_logger::try_init().unwrap_or_default();

    // init scheduler
    let (task_tx, task_rx) = tokio::sync::mpsc::channel(128);
    let (event_tx, event_rx) = tokio::sync::mpsc::channel(128);
    let (result_sender, result_receiver) = tokio::sync::mpsc::channel(128);
    let task_tx_clone = task_tx.clone();
    let mut scheduler = Scheduler::new(result_receiver, event_rx, task_rx, task_tx_clone);

    // init pipeline.
    let mut pipeline = Pipeline::new(
        env::var("WORKSPACE").unwrap_or("data".to_string()),
        env::var("TASK_NAME").unwrap_or("evm".to_string()),
    );
    pipeline.set_task_sender(task_tx);

    // Start the server
    log::info!("====================1. Start the server====================");
    // MOCK ServerHandler to test
    let scheduler_handler = Arc::new(SchedulerServerHandler::default());
    let scheduler_service_svc =
        SchedulerServiceSVC::new(event_tx, result_sender, scheduler_handler.clone());

    // [::1]:50051
    let addr = "[::1]:50051".to_string();
    let server_future = tokio::spawn(async move {
        scheduler_service_svc.launch_server(addr).await.unwrap();
    });

    log::info!("====================1. Start scheduler====================");
    tokio::spawn(async move {
        scheduler.run().await;
    });
    log::info!("====================1. Start pipeline====================");

    tokio::spawn(async move {
        log::info!("====================1. Task incoming ====================");
        let task1 = pipeline.batch_prove("0".into(), "0".into()).unwrap();
        log::info!("task: {task1}");

        let task2 = pipeline.batch_prove("0".into(), "1".into()).unwrap();
        log::info!("task2: {task2}");

        let task3 = pipeline.batch_prove("0".into(), "2".into()).unwrap();
        log::info!("task3: {task3}");

        loop {
            pipeline.prove().unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
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
    tokio::spawn(async move {
        let client_result1 = client_future1.await;
        assert!(client_result1.is_ok(), "Client 1 encountered an error");
    });

    tokio::spawn(async move {
        let client_result2 = client_future2.await;
        assert!(client_result2.is_ok(), "Client 2 encountered an error");
    });

    tokio::spawn(async move {
        let client_result3 = client_future3.await;
        assert!(client_result3.is_ok(), "Client 3 encountered an error");
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    // Wait for the client to finish
    server_future.abort();
    log::info!("====================6. Done ====================");
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
            &take_batch_proof_task_response
                .batch_context_bytes
                .unwrap()
                .data,
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
                    chunk_id: ctx.chunk_id.clone(),
                    result: 1,
                },
            )),
        }
    }
}
