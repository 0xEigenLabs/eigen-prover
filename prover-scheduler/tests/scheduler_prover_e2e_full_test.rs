use std::env;
use std::sync::Arc;

use prover::pipeline::Pipeline;
use prover::scheduler::Scheduler;
use prover_scheduler::scheduler_server::{SchedulerServerHandler, SchedulerServiceSVC};

#[tokio::test]
#[ignore = "slow"]
async fn prover_scheduler_e2e_full_test() {
    // init log
    env::set_var("RUST_LOG", "info");
    env::set_var("PROVER_MODEL", "grpc");
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
    let addr = "0.0.0.0:50051".to_string();
    let server_future = tokio::spawn(async move {
        scheduler_service_svc.launch_server(addr).await.unwrap();
    });

    log::info!("====================2. Start scheduler====================");
    tokio::spawn(async move {
        scheduler.run().await;
    });
    log::info!("====================3. Start pipeline====================");

    let l2_batch_data = std::fs::read_to_string(
        env::var("SUITE_JSON")
            .unwrap_or("../executor/test-vectors/solidityExample.json".to_string()),
    )
    .unwrap();
    log::info!("====================4. Task incoming ====================");
    let task1 = pipeline
        .batch_prove("0".into(), "0".into(), l2_batch_data.clone())
        .unwrap();
    pipeline.prove().unwrap();
    log::info!("task: {task1}");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let task2 = pipeline
        .batch_prove("0".into(), "1".into(), l2_batch_data.clone())
        .unwrap();
    pipeline.prove().unwrap();
    log::info!("task2: {task2}");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let task3 = pipeline
        .batch_prove("0".into(), "2".into(), l2_batch_data)
        .unwrap();
    pipeline.prove().unwrap();
    log::info!("task3: {task3}");
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // Give the server a little time to start
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    log::info!("====================5. Wait service ====================");
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    log::info!("====================6. Start Batch Proof... ====================");

    log::info!("====================7. Wait Batch Proof result... ====================");
    tokio::time::sleep(tokio::time::Duration::from_secs(5400)).await;

    log::info!("====================8. Start 2 rounds of Agg and Final... ====================");
    let task4 = pipeline
        .aggregate_prove("0_chunk_0".to_string(), "0_chunk_2".to_string())
        .unwrap();
    pipeline.prove().unwrap();
    log::info!("agg task: {task4}");

    let task5 = pipeline
        .final_prove(
            task4,
            "BN128".into(),
            "273030697313060285579891744179749754319274977764".into(),
        )
        .unwrap();
    pipeline.prove().unwrap();
    log::info!("final task: {task5}");

    let task6 = pipeline
        .aggregate_prove("0_chunk_0".to_string(), "0_chunk_2".to_string())
        .unwrap();
    pipeline.prove().unwrap();
    log::info!("agg task: {task6}");

    let task7 = pipeline
        .final_prove(
            task6,
            "BN128".into(),
            "273030697313060285579891744179749754319274977764".into(),
        )
        .unwrap();
    pipeline.prove().unwrap();
    log::info!("final task: {task7}");

    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

    // Wait for the client to finish
    server_future.abort();
    log::info!("====================6. Done ====================");
}
