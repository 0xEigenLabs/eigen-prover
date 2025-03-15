use recursion_gnark_ffi;
use prover_core::prover::Prover;
use prover_core::contexts::FinalContext;
use anyhow::Result;

#[derive(Default)]
pub struct FinalProver {}

impl Prover<FinalContext> for FinalProver {
    fn prove(&self, ctx: &FinalContext) -> Result<()> {

        Ok(())
    }
}