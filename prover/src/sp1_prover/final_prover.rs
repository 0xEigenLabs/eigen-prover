use prover_core::prover::Prover;
use prover_core::contexts::FinalContext;
use anyhow::Result;

use std::path::Path;
use recursion_gnark_ffi::json_util::{Groth16Proof, JsonPublicInput, JsonVerificationKey};
use recursion_gnark_ffi::ffi::build_groth16;
use ark_bls12_381::Bls12_381;
use ark_groth16::Groth16;
use ark_groth16::Proof;
use ark_groth16::VerifyingKey;
use std::fs;

#[derive(Default)]
pub struct Sp1FinalProver {}

impl Prover<FinalContext> for Sp1FinalProver {
    fn prove(&self, ctx: &FinalContext) -> Result<()> {

        let input_path= format!("{}/public_inputs.json", &ctx.basedir);
        let vk_path= format!("~/.sp1/circuits/groth16/v4.0.0-rc.3/groth16_vk.bin");
        let proof_path = format!("{}/proof.bin", &ctx.basedir);

        build_groth16(&vk_path, &ctx.basedir, &proof_path, &input_path);

        let input_file_bls12381 = Path::new(&ctx.basedir).join("public_inputs_bls12381.json"); 
        let vk_file_bls12381 = Path::new(&ctx.basedir).join("groth16_vk_bls12381.json"); 
        let proof_file_bls12381 = Path::new(&ctx.basedir).join("proof_bls12381.json"); 

        let public_input =
            fs::read_to_string(input_file_bls12381).expect("Failed to read public inputs JSON file");
        let public_input =
            serde_json::from_str::<JsonPublicInput<ark_bls12_381::Fr>>(&public_input)
                .expect("Failed to parse JSON public input");
        println!("Public Input: {:?}", public_input);
        let vk_string =
            fs::read_to_string(vk_file_bls12381).expect("Failed to read public inputs JSON file");
        let vk = serde_json::from_str::<JsonVerificationKey<Bls12_381>>(&vk_string).unwrap();
        println!("vk: {:?}", vk);
        let proof_string =
            fs::read_to_string(proof_file_bls12381).expect("Failed to read public inputs JSON file");
        let proof = serde_json::from_str::<Groth16Proof<Bls12_381>>(&proof_string).unwrap();
        println!("proof: {:?}", proof);

        let vk = VerifyingKey::<Bls12_381> {
            alpha_g1: vk.alpha_1,
            beta_g2: vk.beta_2,
            gamma_g2: vk.gamma_2,
            delta_g2: vk.delta_2,
            gamma_abc_g1: vk.ic[..vk.ic.len() - 1].to_vec(),
        };
        let proof = Proof { a: proof.pi_a, b: proof.pi_b, c: proof.pi_c };
        let public_inputs: &[ark_bls12_381::Fr] = &public_input.values;
        println!("Expected public inputs: {}", vk.gamma_abc_g1.len());
        println!("Provided public inputs: {}", public_inputs.len());
        let vk = ark_groth16::prepare_verifying_key(&vk);

        let res = Groth16::<Bls12_381>::verify_proof(&vk, &proof, public_inputs);
        assert!(res.is_ok(), "Groth16 proof verification failed: {:?}", res.err());

        Ok(())
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_sp1_final_prove() {
        env_logger::try_init().unwrap_or_default();
        let sp1_prover = Sp1FinalProver::default();
        
        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let mut agg_context = FinalContext::default();
        agg_context.basedir = format!("{}/test_vectors", &manifest_dir.display());

        let _ = sp1_prover.prove(&agg_context);
    }
}
