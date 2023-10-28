use crate::traits::Executor;
use algebraic::errors::{EigenError, Result};

pub struct C12Prover {}
impl C12Prover {
    pub fn new() -> Self { C12Prover{} }
}

impl Executor for C12Prover {
    fn execute(&self, basedir: &str, task_id: &str) -> Result<()> {
        log::info!("start c12 prove");
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

        // 3. stark prove

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
        log::info!("end c12 prove");
        Ok(())
    }
}
