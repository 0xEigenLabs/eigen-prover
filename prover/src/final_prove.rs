use algebraic::errors::Result;
use groth16::api::{groth16_prove, groth16_setup, groth16_verify};

use crate::traits::StageProver;
use crate::FinalContext;
use starky::{compressor12_exec::exec, compressor12_setup::setup, prove::stark_prove};

pub struct FinalProver {}
impl FinalProver {
    pub fn new() -> Self {
        FinalProver {}
    }
}

impl StageProver for FinalProver {
    fn final_prove(&self, ctx: &FinalContext) -> Result<()> {
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
            &format!("{}/{}_js/{}.wasm", cc.output, ctx.task_name, ctx.task_name),
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
            &ctx.prover_addr,
        )?;

        log::info!("end final stark prove");

        log::info!("start snark prove");
        // snark_verifier.sh
        let args = &ctx.final_snark;
        groth16_setup(
            &args.curve_type,
            &args.circuit_file,
            &args.pk_file,
            &args.vk_file,
        )?;
        groth16_prove(
            &args.curve_type,
            &args.circuit_file,
            &args.wasm_file,
            &args.pk_file,
            &args.input_file,
            &args.public_input_file,
            &args.proof_file,
        )?;
        groth16_verify(
            &args.curve_type,
            &args.vk_file,
            &args.public_input_file,
            &args.proof_file,
        )?;

        log::info!("end snark prove");
        Ok(())
    }
}
