use crate::traits::Executor;
use crate::Context;
use algebraic::errors::Result;
use starky::prove::stark_prove;
use starky::{compressor12_exec::exec, compressor12_setup::setup};

pub struct FinalStarkProver {}
impl FinalStarkProver {
    pub fn new() -> Self {
        FinalStarkProver {}
    }
}

impl Executor for FinalStarkProver {
    fn execute(&self, ctx: &Context) -> Result<()> {
        log::info!("start final_stark prove");

        // 1. compress setup
        let sp = &ctx.final_stark;
        let cc = &ctx.final_circom;
        setup(
            &sp.r1cs_file,
            &sp.pil_file,
            &sp.const_file,
            &sp.exec_file,
            0,
        )?;

        // 2. compress exec
        exec(
            &sp.zkin,
            &cc.wasm_file,
            &sp.pil_file,
            &sp.exec_file,
            &sp.commit_file,
        )?;

        // 3. generate final proof

        stark_prove(
            &ctx.final_stark_struct,
            &sp.piljson,
            true,
            false,
            &sp.const_file,
            &sp.commit_file,
            &cc.circom_file,
            &sp.zkin,
            &sp.prover_addr,
        )?;

        log::info!("end final stark prove");
        Ok(())
    }
}
