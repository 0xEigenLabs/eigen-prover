use prover_core::contexts::BatchContext;
use prover_core::prover::Prover;
use anyhow::Result;

use sp1_sdk::{include_elf, utils, HashableKey, ProverClient, SP1Stdin};

const ELF: &[u8] = include_elf!("evm");

#[derive(Default)]
pub struct Sp1BatchProver {}

impl Sp1BatchProver {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Prover<BatchContext> for Sp1BatchProver {
    /// Generate stark proof and generate its verifier circuit in circom
    fn prove(&self, ctx: &BatchContext) -> Result<()> {
        log::info!("start batch prove, ctx: {:?}", ctx);
        let prove_start = std::time::Instant::now();
        // given that the l2batch data has been stored in ctx.l2_data.
        let serde_data = ctx.l2_batch_data.clone();
        // let suite: TestUnit = serde_json::from_str(serde_data.as_str()).map_err(|e| e).unwrap();

        let mut stdin = SP1Stdin::new();
        stdin.write(&serde_data);

        let client = ProverClient::new();
        let (pk, vk) = client.setup(ELF);
        log::info!("vk: {:?}", vk.bytes32());
        let proof = client.prove(&pk, stdin).compressed().run().unwrap();

        client.verify(&proof, &vk).expect("verification failed");
        log::info!("ctx.basedir: {:?}", ctx.basedir);
        let proof_path = format!("{}/proof/{}/sp1_proof.bin", ctx.basedir, ctx.task_id);
        proof.save(proof_path).expect("saving proof failed");

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
        let sp1_prover = Sp1BatchProver::new();
        let test_file = stdenv::var("SUITE_JSON").unwrap_or(String::from(
            "../../../../../executor/test-vectors/solidityExample.json",
        ));
        let suite_json = fs::read_to_string(test_file).unwrap();
        let mut batch_context = BatchContext::default();
        batch_context.l2_batch_data = suite_json;
        batch_context.basedir = "../../test_vectors".to_string();
        batch_context.task_id = "0".to_string();
        let _ = sp1_prover.prove(&batch_context);
    }
}
