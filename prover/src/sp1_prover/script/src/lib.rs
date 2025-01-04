use prover::contexts::BatchContext;
use prover::provers::Prover;

use anyhow::Result;
use models::TestUnit;

use sp1_sdk::{include_elf, utils, ProverClient, SP1ProofWithPublicValues, SP1Stdin};

const ELF: &[u8] = include_elf!("evm");

#[derive(Default)]
pub struct Sp1Prover {}

impl Sp1Prover {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Prover<BatchContext> for Sp1Prover {
    /// Generate stark proof and generate its verifier circuit in circom
    fn prove(&self, ctx: &BatchContext) -> Result<()> {
        log::info!("start batch prove, ctx: {:?}", ctx);
        let prove_start = std::time::Instant::now();
        // 1. stark prove: generate `.circom` file.
                                              // given that the l2batch data has been stored in ctx.l2_data.
        let serde_data = ctx.l2_batch_data.clone();
        let suite: TestUnit = serde_json::from_str(serde_data.as_str()).map_err(|e| e).unwrap();

        utils::setup_logger();
        let mut stdin = SP1Stdin::new();
        stdin.write(&suite);

        let client = ProverClient::new();
        let (pk, vk) = client.setup(ELF);
        let mut proof = client.prove(&pk, stdin).run().unwrap();

        // let is_prime = proof.public_values.read::<bool>();
        client.verify(&proof, &vk).expect("verification failed");
        proof.save("proof-with-is-prime.bin").expect("saving proof failed");

        let prove_elapsed = prove_start.elapsed();
        log::info!("prove_elapsed: {:?}", prove_elapsed);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env as stdenv;
    use std::fs;

    #[test]
    fn test_sp1_prove() {
        // env_logger::try_init().unwrap_or_default();
        //let test_file = "test-vectors/blockInfo.json";
        let sp1_prover = Sp1Prover::new();
        let test_file =
            stdenv::var("SUITE_JSON").unwrap_or(String::from("../../../../executor/test-vectors/solidityExample.json"));
        let suite_json = fs::read_to_string(test_file).unwrap();
        let mut batch_context = BatchContext::default();
        batch_context.l2_batch_data = suite_json;
        let _ = sp1_prover.prove(&batch_context);
    }
}