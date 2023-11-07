use crate::traits::StageProver;
use crate::BatchContext;
use algebraic::errors::Result;
use dsl_compile::circom_compiler;
use log::{debug, info};
use starky::prove::stark_prove;
use starky::{compressor12_exec::exec, compressor12_setup::setup};

// Generate stark proof and generate its verifier circuit in circom
// 1. batch prove
// 2. compile circom to r1cs
// 3. compress12
// 4. c12 prove
#[derive(Default)]
pub struct BatchProver {}

impl BatchProver {
    pub fn new() -> Self {
        BatchProver {}
    }
}

impl StageProver for BatchProver {
    fn batch_prove(&self, ctx: &BatchContext) -> Result<()> {
        info!("start batch_prove");
        let batch_stark = &ctx.batch_stark;
        let cc = &ctx.batch_circom;
        let c12_stark = &ctx.c12_stark; // output

        debug!("start stark_prove");
        // 1. batch stark prove: generate `.circom` file.
        stark_prove(
            &ctx.batch_struct,
            &batch_stark.piljson,
            false,
            false,
            &batch_stark.const_file,
            &batch_stark.commit_file,
            &cc.circom_file,
            &c12_stark.zkin,
            "", // prover address
        )?;
        debug!("end stark_prove");

        // todo debug. skip first
        //  diff aggregation_BN128_fibonacci/0/fibonacci.c12.r1cs tests/proof/0/batch_proof/fibonacci.r1cs
        //  diff aggregation_BN128_fibonacci/0/fibonacci.c12_js/fibonacci.c12.wasm tests/proof/0/batch_proof/fibonacci_js/fibonacci.wasm
        ////fixed  diff aggregation_BN128_fibonacci/0/fibonacci.c12.sym tests/proof/0/batch_proof/fibonacci.sym
        //
        // 2. Compile circom circuit to r1cs, and generate witness
        debug!("start circom_compiler");
        circom_compiler(
            cc.circom_file.clone(),
            "goldilocks".to_string(), // prime
            "full".to_string(),       // full_simplification
            cc.link_directories.clone(), // seems like here meet error.
            cc.output.clone(),
            false, // no_simplification
            false, // reduced_simplification
        )
        .unwrap();
        // debug!("end circom_compiler");

        debug!("start compress_setup");
        // todo debug
        // diff  aggregation_BN128_fibonacci/0/fibonacci.const tests/proof/0/batch_proof/fibonacci.const
        // 3.1. compress setup
        setup(
            &c12_stark.r1cs_file,
            &c12_stark.pil_file,
            &c12_stark.const_file,// pil.json meet error.
            &c12_stark.exec_file,// pil.json meet error.
            0,
        )?;
        // debug!("end compress_setup");

        // 3.2. compress exec
        debug!("start compress_exec");
        exec(
            &c12_stark.zkin,
            &format!("{}/{}_js/{}.wasm", cc.output, ctx.task_name, ctx.task_name),
            &c12_stark.pil_file,
            &c12_stark.exec_file,
            &c12_stark.commit_file,
        )?;
        debug!("end compress_exec");

        // 4. c12 prove
        debug!("start c12 prove");
        stark_prove(
            &ctx.c12_struct,
            &c12_stark.piljson,
            true,
            false,
            &c12_stark.const_file,
            &c12_stark.commit_file,
            &cc.circom_file,// todo use a new var. add agg_circom path.
            &c12_stark.zkin,
            "",
        )?;
        debug!("end c12 prove");

        info!("end batch_prove");
        Ok(())
    }
}
