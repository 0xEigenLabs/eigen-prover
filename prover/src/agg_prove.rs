use crate::traits::StageProver;
use crate::AggContext;
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

impl StageProver for AggProver {
    fn agg_prove(&self, ctx: &AggContext) -> Result<()> {
        log::info!("start aggregate prove");

        // 1. Compile circom circuit to r1cs, and generate witness
        let sp = &ctx.agg_stark;
        let sp_next = &ctx.agg_stark.clone();
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
        join_zkin(&ctx.input, &ctx.input2, &sp_next.zkin).unwrap();

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
            &format!("{}/{}_js/{}.wasm", cc.output, ctx.task_name, ctx.task_name),
            &sp.pil_file,
            &sp.exec_file,
            &sp.commit_file,
        )?;

        // 5. stark prove
        stark_prove(
            &ctx.agg_struct,
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
