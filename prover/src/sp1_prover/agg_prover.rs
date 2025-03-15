use anyhow::Result;
use prover_core::contexts::AggContext;
use prover_core::prover::Prover;

use sp1_sdk::{
    include_elf, HashableKey, ProverClient, SP1Proof, SP1ProofWithPublicValues, SP1Stdin,
    SP1VerifyingKey,
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

        // A program that aggregates the proofs of the simple program.
        let agg_elf = std::fs::read(&ctx.aggregate_elf_path).unwrap();
        let program  = std::fs::read(&ctx.elf_path).unwrap();

        let client: ProverClient = ProverClient::new();
        let (aggregation_pk, aggregation_vk) = client.setup(&agg_elf);
        log::info!("aggregation_vk: {:?}", aggregation_vk.bytes32());
        let (_, evm_vk) = client.setup(&program);

        log::info!("evm_vk: {:?}", evm_vk.bytes32());
        let task_id_slice: Vec<_> = ctx.input.split("_chunk_").collect();
        let task_id2_slice: Vec<_> = ctx.input2.split("_chunk_").collect();

        let proof_1_path = format!("{}/proof/{}/sp1_proof.bin", ctx.basedir, task_id_slice[0]);
        let proof_2_path = format!("{}/proof/{}/sp1_proof.bin", ctx.basedir, task_id2_slice[0]);

        let proof_1 = SP1ProofWithPublicValues::load(proof_1_path)?;
        let proof_2 = SP1ProofWithPublicValues::load(proof_2_path)?;
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

        // Generate the plonk bn254 proof.
        let agg_proof =
            client.prove(&aggregation_pk, stdin).groth16().run().expect("proving failed");

        let agg_proof_path = format!("{}/{}/agg_proof.bin", ctx.basedir, ctx.task_path);
        agg_proof.save(agg_proof_path).expect("saving proof failed");

        log::info!("end aggregate prove");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    #[test]
    fn test_agg_prove() {
        env_logger::try_init().unwrap_or_default();
        let agg_prover = Sp1AggProver::new();
        let mut agg_context = AggContext::default();
        agg_context.input = "../../test_vectors/proof/0/sp1_proof_0.bin".to_string();
        agg_context.input2 = "../../test_vectors/proof/0/sp1_proof_1.bin".to_string();

        agg_context.task_path = "../../test_vectors/proof/0/agg_proof.bin".to_string();
        log::info!("task_path: {:?}", agg_context.task_path);
        let _ = agg_prover.prove(&agg_context);
    }

    #[test]
    fn test_load_and_prove() {
        use sp1_sdk::SP1ProofWithPublicValues;

        let proof_path =
            "/mnt/nfs/sy/eigen-prover/prover/src/sp1_prover/test_vectors/proof/0/agg_proof.bin";
        let proof = SP1ProofWithPublicValues::load(proof_path).unwrap();

        let core_proof = proof.clone().proof.try_as_groth_16().unwrap();

        let mut proof_file = std::fs::File::create(
            "/mnt/nfs/sy/eigen-prover/prover/src/sp1_prover/test_vectors/proof/0/proof.bin",
        )
        .unwrap();
        // proof_file
        //     .write_all(serde_json::to_string_pretty(&core_proof.encoded_proof).unwrap().as_bytes())
        //     .expect("Failed to write proof.bin");
        bincode::serialize_into(&mut proof_file, &core_proof).unwrap();
        // let mut public_inputs_file = std::fs::File::create("/mnt/nfs/sy/eigen-prover/prover/src/sp1_prover/test_vectors/proof/0/public_inputs.json").unwrap();
        // let public_inputs_str = serde_json::to_string_pretty(&core_proof.public_inputs).unwrap();
        // public_inputs_file.write_all(public_inputs_str.as_bytes()).unwrap();
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
        eprintln!("proof: {:?}", proof);
        let vkey = load_ark_groth16_verifying_key_from_bytes(&GROTH16_VK_BYTES).unwrap();
        eprintln!("vkey: {:?}", vkey);

        let public_inputs = load_ark_public_inputs_from_bytes(
            &decode_sp1_vkey_hash(vkey_hash).unwrap(),
            &hash_public_inputs(&public_inputs),
        );

        let res =
            Groth16::<Bn254, LibsnarkReduction>::verify_proof(&vkey.into(), &proof, &public_inputs)
                .unwrap();
        eprintln!("res: {:?}", res);
    }

    #[test]
    #[cfg(feature = "ark")]
    fn test_ark_groth16_bls12381() {
        use ark_bn254::Bn254;

        use ark_groth16::{r1cs_to_qap::LibsnarkReduction, Groth16};

        use sp1_verifier::{decode_sp1_vkey_hash, groth16::ark_converter::*, hash_public_inputs};

        let GROTH16_VK_BYTES: &'static [u8] =
            include_bytes!("/mnt/nfs/sy/eigen-zkvm/recursion-gnark/ffi/data/groth16_vk.bin");
        // Location of the serialized SP1ProofWithPublicValues. See README.md for more information.
        let proof_file = "/mnt/nfs/sy/eigen-zkvm/recursion-gnark/ffi/data/proof_bls12381.bin";

        // Load the saved proof and extract the proof and public inputs.
        let sp1_proof_with_public_values = SP1ProofWithPublicValues::load(proof_file).unwrap();

        let proof = sp1_proof_with_public_values.bytes();
        let public_inputs = sp1_proof_with_public_values.public_values.to_vec();

        // This vkey hash was derived by calling `vk.bytes32()` on the verifying key.
        let vkey_hash = "0x00b67552933cee925e1daad47ccdf0402561f3962d92629311dfa0aae7fb54bb";

        let proof = load_ark_proof_from_bytes(&proof[4..]).unwrap();
        eprintln!("proof: {:?}", proof);
        let vkey = load_ark_groth16_verifying_key_from_bytes(&GROTH16_VK_BYTES).unwrap();
        eprintln!("vkey: {:?}", vkey);

        let public_inputs = load_ark_public_inputs_from_bytes(
            &decode_sp1_vkey_hash(vkey_hash).unwrap(),
            &hash_public_inputs(&public_inputs),
        );

        let res =
            Groth16::<Bn254, LibsnarkReduction>::verify_proof(&vkey.into(), &proof, &public_inputs)
                .unwrap();
        eprintln!("res: {:?}", res);
    }
}
