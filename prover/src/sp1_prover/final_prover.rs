use anyhow::Result;
use prover_core::contexts::FinalContext;
use prover_core::prover::Prover;

use ark_bls12_381::Bls12_381;
use ark_groth16::Groth16;
use ark_groth16::Proof;
use ark_groth16::VerifyingKey;
use recursion_gnark_ffi::ffi::build_groth16;
use recursion_gnark_ffi::json_util::{Groth16Proof, JsonPublicInput, JsonVerificationKey};
use std::fs;
use std::path::Path;

#[derive(Default)]
pub struct Sp1FinalProver {}

impl Prover<FinalContext> for Sp1FinalProver {
    fn prove(&self, ctx: &FinalContext) -> Result<()> {
        let vk_path = format!(
            "{}/.sp1/circuits/groth16/v4.0.0-rc.3/groth16_vk.bin",
            std::env::var("HOME").unwrap(),
        );
        let proof_with_pis_path = std::path::Path::new(&ctx.basedir)
            .join(format!("{}/agg_proof.bin", ctx.agg_task_id));
        log::info!("read proof: {}", proof_with_pis_path.display());
        let sp1_proof = match sp1_sdk::SP1ProofWithPublicValues::load(&proof_with_pis_path) {
            Ok(proof) => proof,
            _ => panic!(),
        };

        let groth16_proof = match &sp1_proof.proof {
            sp1_sdk::SP1Proof::Groth16(x) => x.clone(),
            _ => panic!(),
        };
        log::debug!("load groth16 done");

        let inputs = serde_json::to_string(&groth16_proof.public_inputs)?;

        let output_dir =  format!("{}/{}_final", ctx.basedir, ctx.agg_task_id);
        std::fs::create_dir_all(&output_dir)?;
        
        log::debug!("build_groth16: {}", groth16_proof.encoded_proof.len());
        build_groth16(&vk_path, &output_dir, &groth16_proof.raw_proof, &inputs);

        let input_file_bls12381 = Path::new(&output_dir).join("public_inputs_bls12381.json");
        let vk_file_bls12381 = Path::new(&output_dir).join("groth16_vk_bls12381.json");
        let proof_file_bls12381 = Path::new(&output_dir).join("proof_bls12381.json");

        let public_input = fs::read_to_string(input_file_bls12381)
            .expect("Failed to read public inputs JSON file");
        let public_input =
            serde_json::from_str::<JsonPublicInput<ark_bls12_381::Fr>>(&public_input)
                .expect("Failed to parse JSON public input");
        log::debug!("Public Input: {:?}", public_input);
        let vk_string =
            fs::read_to_string(vk_file_bls12381).expect("Failed to read public inputs JSON file");
        let vk = serde_json::from_str::<JsonVerificationKey<Bls12_381>>(&vk_string).unwrap();
        log::debug!("vk: {:?}", vk);
        let proof_string = fs::read_to_string(proof_file_bls12381)
            .expect("Failed to read public inputs JSON file");
        let proof = serde_json::from_str::<Groth16Proof<Bls12_381>>(&proof_string).unwrap();
        log::debug!("proof: {:?}", proof);

        let vk = VerifyingKey::<Bls12_381> {
            alpha_g1: vk.alpha_1,
            beta_g2: vk.beta_2,
            gamma_g2: vk.gamma_2,
            delta_g2: vk.delta_2,
            gamma_abc_g1: vk.ic[..vk.ic.len() - 1].to_vec(),
        };
        let proof = Proof { a: proof.pi_a, b: proof.pi_b, c: proof.pi_c };
        let public_inputs: &[ark_bls12_381::Fr] = &public_input.values;
        log::debug!("Expected public inputs: {}", vk.gamma_abc_g1.len());
        log::debug!("Provided public inputs: {}", public_inputs.len());
        let vk = ark_groth16::prepare_verifying_key(&vk);

        let res = Groth16::<Bls12_381>::verify_proof(&vk, &proof, public_inputs);
        assert!(res.is_ok(), "Groth16 proof verification failed: {:?}", res.err());

        Ok(())
    }
}

#[allow(unused_imports)]
mod tests {
    use super::Sp1FinalProver;
    use prover_core::contexts::FinalContext;
    use prover_core::prover::Prover;
    #[test]
    #[ignore]
    fn test_sp1_final_prove() {
        env_logger::try_init().unwrap_or_default();
        let sp1_prover = Sp1FinalProver::default();

        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let final_context = FinalContext {
            basedir: format!("{}/test_vectors/proof", &manifest_dir.display()),
            agg_task_id: "2".to_string(),

            ..Default::default()
        };

        sp1_prover.prove(&final_context).unwrap();
    }
}
