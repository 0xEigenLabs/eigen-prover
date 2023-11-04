use crate::batch_prove::BatchProver;
use crate::traits::StageProver;
use crate::{BatchContext, Pipeline};
use std::env;
use std::fs::File;

#[test]
fn integration_test() -> algebraic::errors::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env_logger::try_init().unwrap_or_default();

    let pipeline = Pipeline::new(
        env::var("WORKSPACE").unwrap_or("data".to_string()),
        env::var("TASK_NAME").unwrap_or("fibonacci".to_string()),
    );
    let ctx = BatchContext::new(
        pipeline.basedir.clone(),
        "0".to_string(),
        pipeline.task_name.clone(),
    );

    BatchProver::new().batch_prove(&ctx)?;

    Ok(())
}
