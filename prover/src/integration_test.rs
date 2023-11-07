use crate::Pipeline;
use std::env;
use std::fs;

#[test]
#[ignore]
fn integration_test() -> algebraic::errors::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::try_init().unwrap_or_default();

    // init pipeline.
    let mut pipeline = Pipeline::new(
        env::var("WORKSPACE").unwrap_or("data".to_string()),
        env::var("TASK_NAME").unwrap_or("fibonacci".to_string()),
    );
    let task1 = pipeline.batch_prove(0, "".into()).unwrap();
    pipeline.prove().unwrap();
    log::info!("task: {task1}");

    let task2 = pipeline.batch_prove(1, "".into()).unwrap();
    pipeline.prove().unwrap();
    log::info!("task2: {task2}");

    let recursive1_zkin = fs::read_to_string(format!(
        "{}/proof/{}/batch_proof/fibonacci.zkin.json",
        pipeline.basedir, task1
    ))
    .unwrap();
    let recursive2_zkin = fs::read_to_string(format!(
        "{}/proof/{}/batch_proof/fibonacci.zkin.json",
        pipeline.basedir, task2
    ))
    .unwrap();

    let task3 = pipeline
        .aggregate_prove(recursive1_zkin, recursive2_zkin)
        .unwrap();
    pipeline.prove().unwrap();
    log::info!("agg task: {task3}");

    let task4 = pipeline
        .final_prove(
            "BN128".into(),
            "273030697313060285579891744179749754319274977764".into(),
        )
        .unwrap();
    pipeline.prove().unwrap();
    log::info!("final task: {task4}");

    Ok(())
}
