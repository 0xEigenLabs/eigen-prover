use crate::traits::Executor;
use algebraic::errors::Result;
use starky::{compressor12_exec::exec, compressor12_setup::setup, zkin_join::join_zkin};

pub struct AggProver {}
impl AggProver {
    pub fn new() -> Self { AggProver{} }
}

impl Executor for AggProver {
    fn execute(&self, basedir: &str, task_id: &str) -> Result<()> {
        log::info!("start aggregate prove");

        // 1. Compile circom circuit to r1cs, and generate witness
        // circom_compiler(
        //     args.input,
        //     args.prime,
        //     args.full_simplification,
        //     args.link_directories,
        //     args.output,
        //     args.no_simplification,
        //     args.reduced_simplification,
        // )?;

        // 2. compress inputs
        // join_zkin(&args.zkin1, &args.zkin2, &args.zkinout)

        // 3. compress setup
        // setup(
        // &args.r1cs_file,
        // &args.pil_file,
        // &args.const_file,
        // &args.exec_file,
        // args.force_n_bits,
        // )?;

        // 4. compress exec
        // exec(
        //     &args.input_file,
        //     &args.wasm_file,
        //     &args.pil_file,
        //     &args.exec_file,
        //     &args.commit_file,
        // )?;

        // 5. stark prove

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

        log::info!("end aggregate prove");
        Ok(())
    }
}
