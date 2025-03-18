use prover::pipeline::Pipeline;
use std::env;
#[test]
#[ignore]
fn integration_sp1_test() -> anyhow::Result<()> {
    env_logger::try_init().unwrap_or_default();
    let basedir = env::var("BASEDIR").unwrap_or("data/".to_string());
    // init pipeline.
    let mut pipeline =
        Pipeline::new(basedir.clone(), env::var("TASK_NAME").unwrap_or("evm".to_string()));
    log::info!("read batch data");
    let task1_l2_batch_data = std::fs::read_to_string(
        env::var("SUITE_JSON")
            .unwrap_or("../executor/test-vectors/solidityExample.json".to_string()),
    )
    .unwrap();

    log::info!("run task1");
    let task1 = pipeline.batch_prove("10".into(), task1_l2_batch_data.clone()).unwrap();
    pipeline.prove().unwrap();
    log::info!("task: {task1}");

    log::info!("read batch data");
    let task2_l2_batch_data = std::fs::read_to_string(
        env::var("SUITE_JSON").unwrap_or("../executor/test-vectors/blockInfo.json".to_string()),
    )
    .unwrap();

    log::info!("run task2");
    let task2 = pipeline.batch_prove("11".into(), task2_l2_batch_data.clone()).unwrap();
    pipeline.prove().unwrap();
    log::info!("task: {task2}");

    let task3 = pipeline.aggregate_prove("10".to_string(), "11".to_string()).unwrap();
    pipeline.prove().unwrap();
    log::info!("agg task: {task3}");

    let task4 = pipeline
        .final_prove(task3, "273030697313060285579891744179749754319274977764".into())
        .unwrap();
    pipeline.prove().unwrap();
    log::info!("final task: {task4}");
    Ok(())
}
