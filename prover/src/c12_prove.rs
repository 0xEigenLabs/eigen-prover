use crate::traits::Executor;
use crate::Context;
use algebraic::errors::Result;
use starky::prove::stark_prove;
use starky::{compressor12_exec::exec, compressor12_setup::setup};

pub struct C12Prover {}
impl C12Prover {
    pub fn new() -> Self {
        C12Prover {}
    }
}

impl Executor for C12Prover {
    fn execute(&self, ctx: &Context) -> Result<()> {
        log::info!("start c12 prove");
        // 1. compress setup
        let sp = &ctx.c12_stark;
        setup(
            &sp.r1cs_file,
            &sp.pil_file,
            &sp.const_file,
            &sp.exec_file,
            0,
        )?;

        // 2. compress exec
        let cc = &ctx.c12_circom;
        exec(
            &sp.zkin,
            &cc.circom_file,
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
            &sp.zkin,
            "",
        )?;
        log::info!("end c12 prove");
        Ok(())
    }
}
