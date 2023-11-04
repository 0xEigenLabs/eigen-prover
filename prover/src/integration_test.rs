use crate::batch_prove::BatchProver;
use crate::traits::StageProver;
use crate::{BatchContext, Pipeline, ProveStage};
use std::env;
use std::fs::File;

#[test]
fn integration_test() -> algebraic::errors::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::try_init().unwrap_or_default();

    let task_id = "0";

    // init pipeline.
    let mut pipeline = Pipeline::new(
        env::var("WORKSPACE").unwrap_or("data".to_string()),
        env::var("TASK_NAME").unwrap_or("fibonacci".to_string()),
    );
    pipeline.queue.push_back(task_id.to_string());
    pipeline.task_map.lock().unwrap().insert(task_id.to_string(), ProveStage::BatchProve(task_id.to_string()));
    pipeline.save_checkpoint(task_id.to_string(), false);

    // File::create("data/proof/0/agg_proof/fibonacci.circom").unwrap();

    // // data/proof/0/agg_proof/fibonacci.circom
    let ctx = BatchContext::new(
        pipeline.basedir.clone(),
        task_id.to_string(),
        pipeline.task_name.clone(),
    );

    println!("zkin: {:?}", ctx.c12_stark.zkin);
    println!("zkin: {:?}", ctx.batch_stark.zkin);// null str
    BatchProver::new().batch_prove(&ctx)?;

    Ok(())
}
