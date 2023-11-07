use crate::agg_prove::AggProver;
use crate::batch_prove::BatchProver;
use crate::traits::StageProver;
use crate::ProveStage::AggProve;
use crate::{AggContext, BatchContext, Pipeline, ProveStage};
use std::env;
use std::fs::File;

#[test]
fn integration_test() -> algebraic::errors::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::try_init().unwrap_or_default();
    env::set_var("STARK_VERIFIER_GL", "data/pil-stark/circuits.gl");

    let task_id = "0";

    // init pipeline.
    let mut pipeline = Pipeline::new(
        env::var("WORKSPACE").unwrap_or("tests".to_string()),
        env::var("TASK_NAME").unwrap_or("fibonacci".to_string()),
    );
    pipeline.queue.push_back(task_id.to_string());
    pipeline.task_map.lock().unwrap().insert(
        task_id.to_string(),
        ProveStage::BatchProve(task_id.to_string()),
    );
    pipeline.save_checkpoint(task_id.to_string(), false);

    // pipeline.batch_prove()

    // 1. batch prove
    let ctx = BatchContext::new(
        pipeline.basedir.clone(),
        task_id.to_string(),
        pipeline.task_name.clone(),
    );

    BatchProver::new().batch_prove(&ctx)?;

    // let agg_ctx = AggContext::new(
    //     pipeline.basedir.clone(),
    //     task_id.to_string(),
    //     pipeline.task_name.clone(),
    //
    // );
    // AggProver::new().agg_prove()?;

    Ok(())
}
