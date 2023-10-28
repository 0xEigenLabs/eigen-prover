use crate::traits::Executor;
use algebraic::errors::Result;
use starky::{compressor12_exec::exec, compressor12_setup::setup, zkin_join::join_zkin};

pub struct FinalStarkProver {}
impl FinalStarkProver {
    pub fn new() -> Self { FinalStarkProver{} }
}

impl Executor for FinalStarkProver {
    fn execute(&self, basedir: &str, task_id: &str) -> Result<()> {
        log::info!("start final_stark prove");

        // 1. compress setup
        // setup(
        // &args.r1cs_file,
        // &args.pil_file,
        // &args.const_file,
        // &args.exec_file,
        // args.force_n_bits,
        // )?;

        // 2. compress exec
        // exec(
        //     &args.input_file,
        //     &args.wasm_file,
        //     &args.pil_file,
        //     &args.exec_file,
        //     &args.commit_file,
        // )?;

        // 3. generate final proof

        // stark_prove(
        //     &args.stark_struct,
        //     &args.piljson,
        //     args.norm_stage,
        //     args.agg_stage,
        //     &args.const_pols,
        //     &args.cm_pols,
        //     &args.circom_file,
        //     &args.zkin,
        //     &args.prover_addr,
        // )?;

        log::info!("end final stark prove");
        Ok(())
    }
}
