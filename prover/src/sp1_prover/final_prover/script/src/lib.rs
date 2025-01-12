//! A script that generates a Groth16 proof for the Fibonacci program, and verifies the
//! Groth16 proof in SP1.

use sp1_sdk::{include_elf, utils, HashableKey, ProverClient, SP1Stdin, SP1ProofWithPublicValues};

/// The ELF for the Groth16 verifier program.
const GROTH16_ELF: &[u8] = include_elf!("groth16-verifier-program");

/// The ELF for the Aggregation program.
const AGGREGATION_ELF : &[u8] = include_elf!("aggregation-program");


#[derive(Default)]
pub struct FinalProver {}

impl FinalProver {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Prover<FinalContext> for FinalProver {
    fn prove(&self, ctx: &FinalContext) -> Result<()> {
        log::info!("start final_stark prove, ctx: {:?}", ctx);
        let prove_start = std::time::Instant::now();
        
        // Setup logging.
        utils::setup_logger();

        // Create a `ProverClient`.
        let client = ProverClient::from_env();
        let (pk, vk) = client.setup(AGGREGATION_ELF);

        let agg_proof = SP1ProofWithPublicValues::load("../../agg_prover/script/agg_proof.bin")?;
        let agg_proof_bytes = agg_proof.bytes();
        let agg_public_values = agg_proof.public_values.to_vec();
        let vk_bytes32 = vk.bytes32();

        // Write the proof, public values, and vkey hash to the input stream.
        let mut stdin = SP1Stdin::new();
        stdin.write_vec(agg_proof);
        stdin.write_vec(agg_public_values);
        stdin.write(&vk_bytes32);

        // Create a `ProverClient`.
        let client = ProverClient::from_env();

        // Execute the program using the `ProverClient.execute` method, without generating a proof.
        let (_, report) = client.execute(GROTH16_ELF, &stdin).run().unwrap();
        println!("executed groth16 program with {} cycles", report.total_instruction_count());
        println!("{}", report);

        let prove_elapsed = prove_start.elapsed();
        metrics::PROMETHEUS_METRICS
            .lock()
            .unwrap()
            .observe_prover_processing_time_gauge(
                Step::Final,
                Function::Total,
                prove_elapsed.as_secs_f64(),
            );
        log::info!("end snark prove");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env as stdenv;
    use std::fs;

    #[test]
    fn test_final_prove() {
        let final_prover = FinalProver::new();
        let mut final_context = FinalContext::default();
        let _ = final_prover.prove(&final_context);
    }
}
