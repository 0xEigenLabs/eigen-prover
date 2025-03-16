use anyhow::Result;
use prover_core::contexts::BatchContext;
use prover_core::prover::Prover;

use sp1_sdk::{include_elf, HashableKey, EnvProver, SP1Stdin};


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

        let elf_data = std::fs::read(&ctx.elf_path).unwrap();

        let client = EnvProver::new();
        let (pk, vk) = client.setup(&elf_data);
        log::info!("vk: {:?}", vk.bytes32());
        // let proof = client.prove(&pk, stdin).compressed().run().unwrap();
        let proof = client.prove(&pk, &stdin).compressed().run().unwrap();

        client.verify(&proof, &vk).expect("verification failed");
        log::info!("ctx.basedir: {:?}", ctx.basedir);
        let proof_path = format!("{}/proof/sp1_proof_{}.bin", ctx.basedir, ctx.task_id);
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
        env_logger::try_init().unwrap_or_default();
        let sp1_prover = Sp1BatchProver::new();
        
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let test_file = stdenv::var("SUITE_JSON")
            .unwrap_or(format!("{}/../executor/test-vectors/solidityExample.json", &manifest_dir.display()));
        log::info!("current: {}", std::env::current_dir().unwrap().display());
        log::info!("current: {}", std::env::current_exe().unwrap().display());
        let suite_json = fs::read_to_string(test_file).unwrap();
        let mut batch_context = BatchContext::default();
        batch_context.l2_batch_data = suite_json;
        batch_context.basedir = format!("{}/test_vectors", &manifest_dir.display());
        batch_context.task_id = "0".to_string();
        batch_context.elf_path = format!("{}/../target/elf-compilation/riscv32im-succinct-zkvm-elf/release/evm", manifest_dir.display());
        let _ = sp1_prover.prove(&batch_context);


        batch_context.task_id = "1".to_string();
        let _ = sp1_prover.prove(&batch_context);
    }
}
