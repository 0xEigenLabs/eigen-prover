use crate::traits::StageProver;
use crate::BatchContext;
use algebraic::errors::Result;
use dsl_compile::circom_compiler;
use starky::prove::stark_prove;
use starky::{compressor12_exec::exec, compressor12_setup::setup};

#[derive(Default)]
pub struct BatchProver {}

impl BatchProver {
    pub fn new() -> Self {
        BatchProver {}
    }
}

impl StageProver for BatchProver {
    /// Generate stark proof and generate its verifier circuit in circom
    fn batch_prove(&self, ctx: &BatchContext) -> Result<()> {
        log::info!("start batch prove");
        // 1. stark prove: generate `.circom` file.
        let sp = &ctx.batch_stark;
        let cc = &ctx.batch_circom;
        let sp_next = &ctx.batch_stark.clone(); // output
        stark_prove(
            &ctx.batch_struct,
            &sp.piljson,
            true,
            true,
            &sp.const_file,
            &sp.commit_file,
            &cc.circom_file,
            &sp_next.zkin,
            "", // prover address
        )?;

        // 2. Compile circom circuit to r1cs, and generate witness
        circom_compiler(
            cc.circom_file.clone(),
            "goldilocks".to_string(), // prime
            "full".to_string(),       // full_simplification
            cc.link_directories.clone(),
            cc.output.clone(),
            true, // no_simplification
            true, // reduced_simplification
        )
        .unwrap();
        log::info!("end batch prove");

        log::info!("start c12 prove");
        // 1. compress setup
        setup(
            &sp.r1cs_file,
            &sp.pil_file,
            &sp.const_file,
            &sp.exec_file,
            0,
        )?;

        // 2. compress exec
        exec(
            &sp_next.zkin,
            &format!("{}/{}_js/{}.wasm", cc.output, ctx.task_name, ctx.task_name),
            &sp.pil_file,
            &sp.exec_file,
            &sp.commit_file,
        )?;

        // 3. stark prove
        stark_prove(
            &ctx.c12_struct,
            &sp.piljson,
            true,
            false,
            &sp.const_file,
            &sp.commit_file,
            &cc.circom_file,
            &sp_next.zkin,
            "",
        )?;
        log::info!("end c12 prove");
        Ok(())
    }
}
