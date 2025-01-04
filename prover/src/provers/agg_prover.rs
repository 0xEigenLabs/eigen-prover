use super::Prover;
use crate::contexts::AggContext;
use crate::contexts::BatchContext;
use crate::contexts::{CacheStage, StarkFileType};

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

const EVM_ELF: &[u8] = include_elf!("evm");

/// A program that aggregates the proofs of the simple program.
const AGGREGATION_ELF: &[u8] = include_elf!("aggregation-program");

use anyhow::Result;
use dsl_compile::circom_compiler;
use recursion::{compressor12_exec::exec, compressor12_setup::setup};

use metrics::{Function, Step};
use starky::prove::stark_prove;
use starky::zkin_join::join_zkin;

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
        let prove_start = std::time::Instant::now();


        // 1. Compile circom circuit to r1cs, and generate witness
        // FIXME: extract all the proof and verifier from the request.
        let task_id_slice: Vec<_> = ctx.input.split("_chunk_").collect();
        let task_id2_slice: Vec<_> = ctx.input2.split("_chunk_").collect();
        assert_eq!(task_id_slice[0], task_id2_slice[0]);
        let start: usize = task_id_slice[1].parse::<usize>().unwrap();
        let end: usize = task_id2_slice[1].parse::<usize>().unwrap();

        let (_, evm_vk) = client.setup(EVM_ELF);
        // TODO: 1. replace with proof path 2. traverse all chunks and aggregate every two chunks together
        let proof_1 = SP1ProofWithPublicValues::load(format!(
            "{}/proof/{}/batch_proof_{}/{}.proof",
            ctx.basedir, task_id_slice[0], start, ctx.task_name,
        ))?;
        let proof_2 = SP1ProofWithPublicValues::load(format!(
            "{}/proof/{}/batch_proof_{}/{}.proof",
            ctx.basedir, task_id2_slice[0], end, ctx.task_name,
        ))?;
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
        let agg_proof = client.prove(&aggregation_pk, &stdin).plonk().run().expect("proving failed");
        agg_proof.save(format!(
            "{}/proof/{}/agg_proof_{}/{}.proof.bin",
            ctx.basedir, task_id_slice[0], start, ctx.task_name,
        )).expect("saving proof failed");        ;

        log::info!("end aggregate prove");
        let prove_elapsed = prove_start.elapsed();
        metrics::PROMETHEUS_METRICS
            .lock()
            .unwrap()
            .observe_prover_processing_time_gauge(
                Step::Agg,
                Function::Total,
                prove_elapsed.as_secs_f64(),
            );
        Ok(())
    }
}
