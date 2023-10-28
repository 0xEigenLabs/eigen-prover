use crate::traits::Executor;
use crate::Context;
use algebraic::errors::Result;
use dsl_compile::circom_compiler;
use starky::{
    compressor12_exec::exec, compressor12_setup::setup, prove::stark_prove, zkin_join::join_zkin,
};

pub struct AggProver {}
impl AggProver {
    pub fn new() -> Self {
        AggProver {}
    }
}

impl Executor for AggProver {
    fn execute(&self, ctx: &Context) -> Result<()> {
        log::info!("start aggregate prove");

        // 1. Compile circom circuit to r1cs, and generate witness
        let sp = &ctx.agg_stark;
        let sp_next = &ctx.final_stark;
        let cc = &ctx.agg_circom;
        circom_compiler(
            cc.circom_file.clone(),
            "goldilocks".to_string(),
            "full".to_string(),
            cc.link_directories.clone(),
            sp.zkin.clone(),
            true,
            true,
        )
        .unwrap();

        // 2. compress inputs
        join_zkin(&sp.zkin, &sp.zkin2, &sp_next.zkin).unwrap();

        // 3. compress setup
        setup(
            &sp.r1cs_file,
            &sp.pil_file,
            &sp.const_file,
            &sp.exec_file,
            0,
        )?;

        // 4. compress exec
        exec(
            &sp.zkin,
            &cc.wasm_file,
            &sp.pil_file,
            &sp.exec_file,
            &sp.commit_file,
        )?;

        // 5. stark prove
        stark_prove(
            &ctx.final_stark_struct,
            &sp.piljson,
            true,
            false,
            &sp.const_file,
            &sp.commit_file,
            &cc.circom_file,
            &sp.zkin,
            "",
        )?;

        log::info!("end aggregate prove");
        Ok(())
    }
}
