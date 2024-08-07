use std::env;

use prover::pipeline::Pipeline;

#[test]
#[ignore = "slow"]
fn integration_test() -> anyhow::Result<()> {
    env_logger::try_init().unwrap_or_default();
    let curve_name = env::var("CURVE_NAME").unwrap_or("BN128".to_string());

    // init pipeline.
    let mut pipeline = Pipeline::new(
        env::var("BASEDIR").unwrap_or("data".to_string()),
        env::var("TASK_NAME").unwrap_or("evm".to_string()),
    );
    let l2_batch_data = std::fs::read_to_string(
        env::var("SUITE_JSON")
            .unwrap_or("../executor/test-vectors/solidityExample.json".to_string()),
    )
    .unwrap();
    let task1 = pipeline
        .batch_prove("0".into(), "0".into(), l2_batch_data.clone())
        .unwrap();
    pipeline.prove().unwrap();
    log::info!("task: {task1}");

    let task2 = pipeline
        .batch_prove("0".into(), "1".into(), l2_batch_data.clone())
        .unwrap();
    pipeline.prove().unwrap();
    log::info!("task2: {task2}");

    let task3 = pipeline
        .batch_prove("0".into(), "2".into(), l2_batch_data)
        .unwrap();
    pipeline.prove().unwrap();
    log::info!("task3: {task3}");

    let task4 = pipeline
        .aggregate_prove("0_chunk_0".to_string(), "0_chunk_2".to_string())
        .unwrap();
    pipeline.prove().unwrap();
    log::info!("agg task: {task4}");

    let task5 = pipeline
        .final_prove(
            task4,
            curve_name.clone(),
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
            curve_name,
            "273030697313060285579891744179749754319274977764".into(),
        )
        .unwrap();
    pipeline.prove().unwrap();
    log::info!("final task: {task7}");
    Ok(())
}

#[test]
#[ignore = "slow"]
fn integration_test_lr() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::try_init().unwrap_or_default();

    // init pipeline.
    let mut pipeline = Pipeline::new(
        env::var("BASEDIR").unwrap_or("data".to_string()),
        env::var("TASK_NAME").unwrap_or("lr".to_string()),
    );
    let l2_batch_data =
        std::fs::read_to_string(env::var("SUIT_JSON").unwrap_or("data/test.json".to_string()))
            .unwrap();
    let task1 = pipeline
        .batch_prove("0".into(), "0".into(), l2_batch_data)
        .unwrap();
    pipeline.prove().unwrap();
    log::info!("task: {task1}");
    Ok(())
}
