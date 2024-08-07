use super::Prover;
use crate::contexts::BatchContext;

use anyhow::Result;
use powdr::number::{FieldElement, GoldilocksField};

use dsl_compile::circom_compiler;
use metrics::Batch::BatchStark;
use metrics::{Batch, Function, Step};
use recursion::{compressor12_exec::exec, compressor12_setup::setup};
use starky::prove::stark_prove;
use std::{fs, io::Read};
use zkvm::zkvm_prove_only;

#[derive(Default)]
pub struct BatchProver {}

impl BatchProver {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Prover<BatchContext> for BatchProver {
    /// Generate stark proof and generate its verifier circuit in circom
    fn prove(&self, ctx: &BatchContext) -> Result<()> {
        log::info!("start batch prove, ctx: {:?}", ctx);
        let prove_start = std::time::Instant::now();
        // 1. stark prove: generate `.circom` file.
        let batch_stark = &ctx.batch_stark;
        let batch_circom = &ctx.batch_circom;
        let c12_circom = &ctx.c12_circom;
        let c12_stark = &ctx.c12_stark;
        let r1_circom = &ctx.recursive1_circom; // output
        let r1_stark = &ctx.recursive1_stark; // output
                                              // given that the l2batch data has been stored in ctx.l2_data.
        let serde_data = ctx.l2_batch_data.clone();
        // the circom: $output/main_proof.bin_1
        // the zkin(stark proof): $output/main_proof.bin_0
        let bootloader_input_path = format!(
            "{}/proof/{}/{}/{}_chunks_{}.data",
            &ctx.basedir, ctx.task_id, ctx.task_name, ctx.task_name, ctx.chunk_id
        );
        log::info!("bootloader_input_path: {}", bootloader_input_path);
        let mut f = fs::File::open(bootloader_input_path.clone())?;
        let metadata = fs::metadata(bootloader_input_path)?;
        let file_size = metadata.len() as usize;
        assert!(file_size % 8 == 0);
        // read the start_of_shutdown_routine
        let mut buffer = [0u8; 8];
        f.read_exact(&mut buffer).unwrap();
        let start_of_shutdown_routine: u64 = u64::from_le_bytes(buffer);
        log::debug!("start_of_shutdown_routine: {start_of_shutdown_routine}");

        let file_size = file_size - 8;
        let mut buffer = vec![0; file_size];
        f.read_exact(&mut buffer)?;
        let mut bi = vec![GoldilocksField::default(); file_size / 8];
        bi.iter_mut().zip(buffer.chunks(8)).for_each(|(out, bin)| {
            *out = GoldilocksField::from_bytes_le(bin);
        });
        log::debug!("read bootstrap input done");

        zkvm_prove_only(
            &ctx.task_name,
            &serde_data,
            bi,
            start_of_shutdown_routine,
            ctx.chunk_id.parse()?,
            &ctx.evm_output,
        )?;
        log::debug!("zkvm_prove_only done");
        /*
        stark_prove(
            &ctx.batch_struct,
            &sp.piljson,
            false,
            false,
            &sp.const_file,
            &sp.commit_file,
            &c12_circom.circom_file,
            &c12_stark.zkin,
            "", // prover address
        )?;
        */

        log::debug!("circom_compiler: {:?}", batch_circom);
        // 2. Compile circom circuit to r1cs, and generate witness
        circom_compiler(
            batch_circom.circom_file.clone(),
            "goldilocks".to_string(), // prime
            "full".to_string(),       // full_simplification
            batch_circom.link_directories.clone(),
            batch_circom.output.clone(),
            false, // no_simplification
            false, // reduced_simplification
        )?;

        log::info!("batch proof: compress setup");
        let setup_start = std::time::Instant::now();
        setup(
            &batch_stark.r1cs_file,
            &batch_stark.pil_file,
            &batch_stark.const_file,
            &batch_stark.exec_file,
            0,
        )?;
        let setup_elapsed = setup_start.elapsed();
        metrics::PROMETHEUS_METRICS
            .lock()
            .unwrap()
            .observe_prover_processing_time_gauge(
                Step::Batch(Batch::BatchStark),
                Function::Setup,
                setup_elapsed.as_secs_f64(),
            );

        log::info!("batch proof. compress exec");
        let exec_start = std::time::Instant::now();
        exec(
            &batch_stark.zkin,
            &batch_stark.wasm_file,
            &batch_stark.pil_file,
            &batch_stark.exec_file,
            &batch_stark.commit_file,
        )?;
        let exec_elapsed = exec_start.elapsed();
        metrics::PROMETHEUS_METRICS
            .lock()
            .unwrap()
            .observe_prover_processing_time_gauge(
                Step::Batch(BatchStark),
                Function::Exec,
                exec_elapsed.as_secs_f64(),
            );

        // 3. stark prove
        let stark_prove_start = std::time::Instant::now();
        stark_prove(
            &ctx.batch_struct,
            &batch_stark.piljson,
            false,
            false,
            false,
            &batch_stark.const_file,
            &batch_stark.commit_file,
            &c12_circom.circom_file,
            &c12_stark.zkin,
            "",
        )?;
        log::info!("end batch prove");
        let stark_prove_elapsed = stark_prove_start.elapsed();
        metrics::PROMETHEUS_METRICS
            .lock()
            .unwrap()
            .observe_prover_processing_time_gauge(
                Step::Batch(BatchStark),
                Function::StarkProve,
                stark_prove_elapsed.as_secs_f64(),
            );

        log::info!("start c12 prove: {:?}", c12_stark);

        // 2. Compile circom circuit to r1cs, and generate witness
        circom_compiler(
            c12_circom.circom_file.clone(),
            "goldilocks".to_string(), // prime
            "full".to_string(),       // full_simplification
            c12_circom.link_directories.clone(),
            c12_circom.output.clone(),
            false, // no_simplification
            false, // reduced_simplification
        )?;

        let c12_setup_start = std::time::Instant::now();
        setup(
            &c12_stark.r1cs_file,
            &c12_stark.pil_file,
            &c12_stark.const_file,
            &c12_stark.exec_file,
            ctx.force_bits,
        )?;
        let c12_setup_elapsed = c12_setup_start.elapsed();
        metrics::PROMETHEUS_METRICS
            .lock()
            .unwrap()
            .observe_prover_processing_time_gauge(
                Step::Batch(Batch::C12Stark),
                Function::Setup,
                c12_setup_elapsed.as_secs_f64(),
            );

        log::info!("c12 proof: compress exec");
        let c12_exec_start = std::time::Instant::now();
        exec(
            &c12_stark.zkin,
            &c12_stark.wasm_file,
            &c12_stark.pil_file,
            &c12_stark.exec_file,
            &c12_stark.commit_file,
        )?;
        let c12_exec_elapsed = c12_exec_start.elapsed();
        log::info!("c12 proof: compress exec elapsed: {:?}", c12_exec_elapsed);
        metrics::PROMETHEUS_METRICS
            .lock()
            .unwrap()
            .observe_prover_processing_time_gauge(
                Step::Batch(Batch::C12Stark),
                Function::Exec,
                c12_exec_elapsed.as_secs_f64(),
            );

        // 3. stark prove
        let c12_stark_prove_start = std::time::Instant::now();
        stark_prove(
            &ctx.c12_struct,
            &c12_stark.piljson,
            true,
            false,
            true,
            &c12_stark.const_file,
            &c12_stark.commit_file,
            &r1_circom.circom_file,
            &r1_stark.zkin,
            "",
        )?;
        let c12_stark_prove_elapsed = c12_stark_prove_start.elapsed();
        metrics::PROMETHEUS_METRICS
            .lock()
            .unwrap()
            .observe_prover_processing_time_gauge(
                Step::Batch(Batch::C12Stark),
                Function::StarkProve,
                c12_stark_prove_elapsed.as_secs_f64(),
            );

        log::info!("end c12 prove");
        let prove_elapsed = prove_start.elapsed();
        metrics::PROMETHEUS_METRICS
            .lock()
            .unwrap()
            .observe_prover_processing_time_gauge(
                Step::Batch(BatchStark),
                Function::Total,
                prove_elapsed.as_secs_f64(),
            );
        Ok(())
    }
}
