use anyhow::Result;
use prover_core::contexts::BatchContext;
use prover_core::prover::Prover;

use sp1_sdk::{EnvProver, HashableKey, SP1Stdin};

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
        let proof = client.prove(&pk, &stdin).compressed().run().unwrap();

        client.verify(&proof, &vk).expect("verification failed");

        let tmp_path = format!("{}/{}", ctx.basedir, ctx.task_id);
        std::fs::create_dir_all(&tmp_path)?;

        log::info!("ctx.basedir: {:?}", ctx.basedir);
        let proof_path = format!("{}/sp1_proof.bin", tmp_path);
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
        let test_file = stdenv::var("SUITE_JSON").unwrap_or(format!(
            "{}/../executor/test-vectors/solidityExample.json",
            &manifest_dir.display()
        ));
        let suite_json = fs::read_to_string(test_file).unwrap();

        let batch_context = BatchContext {
            l2_batch_data: suite_json.clone(),
            basedir: format!("{}/test_vectors/proof", &manifest_dir.display()),
            task_id: "0".to_string(),
            elf_path: format!(
                "{}/../target/elf-compilation/riscv32im-succinct-zkvm-elf/release/evm",
                manifest_dir.display()
            ),
            ..Default::default()
        };
        sp1_prover.prove(&batch_context).unwrap();

        let batch_context = BatchContext {
            l2_batch_data: suite_json,
            basedir: format!("{}/test_vectors/proof", &manifest_dir.display()),
            task_id: "1".to_string(),
            elf_path: format!(
                "{}/../target/elf-compilation/riscv32im-succinct-zkvm-elf/release/evm",
                manifest_dir.display()
            ),
            ..Default::default()
        };
        sp1_prover.prove(&batch_context).unwrap();
    }
}
