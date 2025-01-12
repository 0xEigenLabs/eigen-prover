use prover::contexts::AggContext;
use prover::provers::Prover;
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
pub struct AggProver {}

impl AggProver {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Prover<AggContext> for AggProver {
    fn prove(&self, ctx: &AggContext) -> Result<()> {
        log::info!("start aggregate prove, ctx: {:?}", ctx);

        let client = ProverClient::new();
        let (aggregation_pk, _) = client.setup(AGGREGATION_ELF);
        let (_, evm_vk) = client.setup(EVM_ELF);
        let proof_1 = SP1ProofWithPublicValues::load("/mnt/nfs/sy/eigen-prover/prover/src/sp1_prover/script/sp1_proof_68855.bin")?;
        let proof_2 = SP1ProofWithPublicValues::load("/mnt/nfs/sy/eigen-prover/prover/src/sp1_prover/script/sp1_proof_68858.bin")?;
        let agg_input1 = AggregationInput { proof: proof_1, vk: evm_vk};
        let agg_input2 = AggregationInput{ proof: proof_2, vk: evm_vk};
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
        let agg_proof = client.prove(&aggregation_pk, stdin).plonk().run().expect("proving failed");
        agg_proof.save("../agg_proof.bin").expect("saving proof failed");

        log::info!("end aggregate prove");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env as stdenv;
    use std::fs;

    #[test]
    fn test_agg_prove() {
        let agg_prover = AggProver::new();
        let mut agg_context = AggContext::default();
        let _ = agg_prover.prove(&agg_context);
    }
}
