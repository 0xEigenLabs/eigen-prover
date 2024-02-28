mod agg_prover;
pub use agg_prover::AggProver;

mod batch_prover;
pub use batch_prover::BatchProver;

mod final_prover;
pub use final_prover::FinalProver;

use anyhow::Result;

/// Prover trait
pub trait Prover<T> {
    fn prove(&self, ctx: &T) -> Result<()>;
}
