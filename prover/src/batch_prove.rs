use crate::traits::Executor;
use crate::Context;
use algebraic::errors::Result;
use dsl_compile::circom_compiler;
use starky::prove::stark_prove;

#[derive(Default)]
pub struct BatchProver {}

impl BatchProver {
    pub fn new() -> Self {
        BatchProver {}
    }
}

impl Executor for BatchProver {
    /// Generate stark proof and generate its verifier circuit in circom
    fn execute(&self, ctx: &Context) -> Result<()> {
        log::info!("start batch prove");
        // 1. stark prove: generate `.circom` file.
        let sp = &ctx.batch_stark;
        let cc = &ctx.batch_circom;
        stark_prove(
            &ctx.batch_struct,
            &sp.piljson,
            true,
            true,
            &sp.const_file,
            &sp.commit_file,
            &cc.circom_file,
            &sp.zkin,
            "", // prover address
        )?;

        // 2. Compile circom circuit to r1cs, and generate witness
        let output_dir = format!("{}/{}/recursive1", ctx.basedir, ctx.task_id);
        circom_compiler(
            cc.circom_file.clone(),
            "goldilocks".to_string(), // prime
            "full".to_string(),       // full_simplification
            cc.link_directories.clone(),
            output_dir,
            true, // no_simplification
            true, // reduced_simplification
        )
        .unwrap();
        log::info!("end batch prove");
        Ok(())
    }
}
