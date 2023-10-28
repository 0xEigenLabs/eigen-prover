use algebraic::errors::Result;
use groth16::api::{groth16_prove, groth16_setup, groth16_verify};

use crate::traits::Executor;
use crate::Context;

pub struct SnarkProver {}
impl SnarkProver {
    pub fn new() -> Self {
        SnarkProver {}
    }
}

impl Executor for SnarkProver {
    fn execute(&self, ctx: &Context) -> Result<()> {
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
