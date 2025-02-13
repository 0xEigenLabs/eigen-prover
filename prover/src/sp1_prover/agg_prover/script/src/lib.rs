use anyhow::Result;
use prover::contexts::AggContext;
// use prover::provers::Prover;
use prover::prover::Prover;
use prover::eigen_prover::Sp1AggProver;

use sp1_sdk::{
    include_elf, HashableKey, ProverClient, SP1Proof, SP1ProofWithPublicValues, SP1Stdin,
    SP1VerifyingKey,
};

/// A program that aggregates the proofs of the simple program.
const AGGREGATION_ELF: &[u8] = include_elf!("aggregation-program");

const EVM_ELF: &[u8] = include_elf!("evm");

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

        let client = ProverClient::new();
        let (aggregation_pk, aggregation_vk) = client.setup(AGGREGATION_ELF);
        log::info!("aggregation_vk: {:?}", aggregation_vk.bytes32());
        let (_, evm_vk) = client.setup(EVM_ELF);
        log::info!("evm_vk: {:?}", evm_vk.bytes32());
        let proof_1 = SP1ProofWithPublicValues::load(ctx.input.clone())?;
        let proof_2 = SP1ProofWithPublicValues::load(ctx.input2.clone())?;
        let agg_input1 = AggregationInput {
            proof: proof_1,
            vk: evm_vk.clone(),
        };
        let agg_input2 = AggregationInput {
            proof: proof_2,
            vk: evm_vk,
        };
        let inputs = vec![agg_input1, agg_input2];

        // Aggregate the proofs.
        let mut stdin = SP1Stdin::new();

        // Write the verification keys.
        let vkeys = inputs
            .iter()
            .map(|input| input.vk.hash_u32())
            .collect::<Vec<_>>();
        stdin.write::<Vec<[u32; 8]>>(&vkeys);

        // Write the public values.
        let public_values = inputs
            .iter()
            .map(|input| input.proof.public_values.to_vec())
            .collect::<Vec<_>>();
        stdin.write::<Vec<Vec<u8>>>(&public_values);

        // Write the proofs.
        //
        // Note: this data will not actually be read by the aggregation program, instead it will be
        // witnessed by the prover during the recursive aggregation process inside SP1 itself.
        for input in inputs {
            let SP1Proof::Compressed(proof) = input.proof.proof else {
                panic!()
            };
            stdin.write_proof(*proof, input.vk.vk);
        }

        // Generate the plonk bn254 proof.
        let agg_proof = client
            .prove(&aggregation_pk, stdin)
            .groth16()
            .run()
            .expect("proving failed");
        agg_proof
            .save(ctx.task_path.clone())
            .expect("saving proof failed");

        log::info!("end aggregate prove");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agg_prove() {
        env_logger::try_init().unwrap_or_default();
        let agg_prover = SP1AggProver::new();
        let mut agg_context = AggContext::default();
        agg_context.input = "../../test_vectors/proof/0/sp1_proof_0.bin".to_string();
        agg_context.input2 = "../../test_vectors/proof/0/sp1_proof_1.bin".to_string();

        agg_context.task_path = "../../test_vectors/proof/0/agg_proof.bin".to_string();
        log::info!("task_path: {:?}", agg_context.task_path);
        let _ = agg_prover.prove(&agg_context);
    }

    #[test]
    #[cfg(feature = "ark")]
    fn test_ark_groth16() {
        use ark_bn254::Bn254;

        use ark_groth16::{r1cs_to_qap::LibsnarkReduction, Groth16};

        use sp1_verifier::{decode_sp1_vkey_hash, groth16::ark_converter::*, hash_public_inputs};

        let GROTH16_VK_BYTES: &'static [u8] =
            include_bytes!("../../../test_vectors/groth16_vk.bin");
        // Location of the serialized SP1ProofWithPublicValues. See README.md for more information.
        let proof_file = "../../test_vectors/proof/0/agg_proof.bin";

        // Load the saved proof and extract the proof and public inputs.
        let sp1_proof_with_public_values = SP1ProofWithPublicValues::load(proof_file).unwrap();

        let proof = sp1_proof_with_public_values.bytes();
        let public_inputs = sp1_proof_with_public_values.public_values.to_vec();

        // This vkey hash was derived by calling `vk.bytes32()` on the verifying key.
        let vkey_hash = "0x00b67552933cee925e1daad47ccdf0402561f3962d92629311dfa0aae7fb54bb";

        let proof = load_ark_proof_from_bytes(&proof[4..]).unwrap();
        let vkey = load_ark_groth16_verifying_key_from_bytes(&GROTH16_VK_BYTES).unwrap();

        let public_inputs = load_ark_public_inputs_from_bytes(
            &decode_sp1_vkey_hash(vkey_hash).unwrap(),
            &hash_public_inputs(&public_inputs),
        );

        let res =
            Groth16::<Bn254, LibsnarkReduction>::verify_proof(&vkey.into(), &proof, &public_inputs)
                .unwrap();
        println!("res: {:?}", res);
    }
}
