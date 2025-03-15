mod agg_prover;
pub use agg_prover::AggProver;

mod batch_prover;
pub use batch_prover::BatchProver;

mod final_prover;
pub use final_prover::FinalProver;

use anyhow::Result;

// mod sp1_prover;
// pub use sp1_prover::Sp1BatchProver;
// pub use sp1_prover::Sp1AggProver;

// /// Prover trait
// pub trait Prover<T,P> {
//     fn prove(&self, ctx: &T) -> Result<()>;
// }
