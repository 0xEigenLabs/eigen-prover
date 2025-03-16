use anyhow::Result;
use prover_core::contexts::AggContext;
use prover_core::prover::Prover;

use sp1_sdk::{
    EnvProver, HashableKey, SP1Proof, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey,
};

/// An input to the aggregation program.
///
/// Consists of a proof and a verification key.
struct AggregationInput {
    pub proof: SP1ProofWithPublicValues,
    pub vk: SP1VerifyingKey,
}

#[derive(Default)]
pub struct Sp1AggProver {}

impl Sp1AggProver {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Prover<AggContext> for Sp1AggProver {
    fn prove(&self, ctx: &AggContext) -> Result<()> {
        log::info!("start aggregate prove, ctx: {:?}", ctx);

        log::info!("ctx: {:?}", ctx);
        // A program that aggregates the proofs of the simple program.
        let agg_elf = std::fs::read(&ctx.aggregate_elf_path).unwrap();
        let program = std::fs::read(&ctx.elf_path).unwrap();

        let client: EnvProver = EnvProver::new();
        let (aggregation_pk, _aggregation_vk) = client.setup(&agg_elf);
        let (_, evm_vk) = client.setup(&program);

        let proof_1 = SP1ProofWithPublicValues::load(&ctx.input)?;
        let proof_2 = SP1ProofWithPublicValues::load(&ctx.input2)?;
        let agg_input1 = AggregationInput { proof: proof_1, vk: evm_vk.clone() };
        let agg_input2 = AggregationInput { proof: proof_2, vk: evm_vk };
        let inputs = vec![agg_input1, agg_input2];

        // Aggregate the proofs.
        let mut stdin = SP1Stdin::new();

        // Write the verification keys.
        let vkeys = inputs.iter().map(|input| input.vk.hash_u32()).collect::<Vec<_>>();
        stdin.write::<Vec<[u32; 8]>>(&vkeys);

        // Write the public values.
        let public_values =
            inputs.iter().map(|input| input.proof.public_values.to_vec()).collect::<Vec<_>>();
        stdin.write::<Vec<Vec<u8>>>(&public_values);

        // Write the proofs.
        //
        // Note: this data will not actually be read by the aggregation program, instead it will be
        // witnessed by the prover during the recursive aggregation process inside SP1 itself.
        for input in inputs {
            let SP1Proof::Compressed(proof) = input.proof.proof else { panic!() };
            stdin.write_proof(*proof, input.vk.vk);
        }

        log::info!("agg_proof prove");

        // Generate the plonk bn254 proof.
        let agg_proof =
            client.prove(&aggregation_pk, &stdin).groth16().run().expect("proving failed");

        let agg_proof_path = format!("{}/proof/agg_proof.bin", ctx.basedir);
        agg_proof.save(agg_proof_path).expect("saving proof failed");
        log::info!("end aggregate prove");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sp1_agg_prove() {
        env_logger::try_init().unwrap_or_default();
        let agg_prover = Sp1AggProver::new();
        let mut agg_context = AggContext::default();

        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        agg_context.basedir = format!("{}/test_vectors", manifest_dir.display());
        agg_context.input =
            format!("{}/test_vectors/proof/sp1_proof_0.bin", manifest_dir.display());
        println!("agg_context.input: {:?}", agg_context.input);
        agg_context.input2 =
            format!("{}/test_vectors/proof/sp1_proof_1.bin", manifest_dir.display());

        agg_context.task_path =
            format!("{}/test_vectors/proof/agg_proof.bin", manifest_dir.display());
        agg_context.elf_path = format!(
            "{}/../target/elf-compilation/riscv32im-succinct-zkvm-elf/release/evm",
            manifest_dir.display()
        );
        agg_context.aggregate_elf_path = format!(
            "{}/../target/elf-compilation/riscv32im-succinct-zkvm-elf/release/aggregation",
            manifest_dir.display()
        );

        log::info!("task_path: {:?}", agg_context.task_path);
        let _ = agg_prover.prove(&agg_context);
    }
}
