use crate::traits::StageProver;
use crate::BatchContext;
use anyhow::Result;
use powdr_backend::BackendType;
use powdr_number::{FieldElement, GoldilocksField};
use powdr_pipeline::{Pipeline, Stage};
use powdr_riscv::continuations::{
    bootloader::default_input, rust_continuations, rust_continuations_dry_run,
};
use powdr_riscv::{compile_rust, CoProcessors};

use dsl_compile::circom_compiler;
use recursion::{compressor12_exec::exec, compressor12_setup::setup};
use starky::prove::stark_prove;
use std::path::Path;
use std::{fs, io::Read, io::Write};
use zkvm::{zkvm_evm_generate_chunks, zkvm_evm_prove_only};
#[derive(Default)]
pub struct BatchProver {}

impl BatchProver {
    pub fn new() -> Self {
        BatchProver {}
    }
}

impl StageProver for BatchProver {
    /// Generate stark proof and generate its verifier circuit in circom
    fn batch_prove(&self, ctx: &BatchContext) -> Result<()> {
        log::info!("start batch prove");
        // 1. stark prove: generate `.circom` file.
        let sp = &ctx.batch_stark;
        let c12_circom = &ctx.c12_circom;
        let c12_stark = &ctx.c12_stark;
        let r1_circom = &ctx.recursive1_circom; // output
        let r1_stark = &ctx.recursive1_stark; // output
        log::info!("batch_context: {:?}", ctx);
        // given that the l2batch data has been stored in sp.zkin.
        let serde_data = std::fs::read_to_string(sp.zkin.clone())?;
        // the circom: $output/main_proof.bin_1
        // the zkin(stark proof): $output/main_proof.bin_0
        let output_path = &ctx.evm_output;
        let task = &ctx.task_name;
        let bootloader_inputs = zkvm_evm_generate_chunks(&task, &serde_data, output_path).unwrap();
        log::info!(
            "====================bootloader_inputs length: {:?}",
            bootloader_inputs.len()
        );
        let bi_files: Vec<_> = (0..bootloader_inputs.len())
            .map(|i| Path::new(output_path).join(format!("{task}_chunks_{i}.data")))
            .collect();
        bootloader_inputs
            .iter()
            .zip(&bi_files)
            .for_each(|(data, filename)| {
                let mut f = fs::File::create(filename).unwrap();
                for d in data {
                    f.write_all(&d.to_bytes_le()[0..8]).unwrap();
                }
            });
        bi_files.iter().enumerate().for_each(|(i, filename)| {
            let mut f = fs::File::open(filename).unwrap();
            let metadata = fs::metadata(filename).unwrap();
            let file_size = metadata.len() as usize;
            assert!(file_size % 8 == 0);
            let mut buffer = vec![0; file_size];
            f.read_exact(&mut buffer).unwrap();
            let mut bi = vec![GoldilocksField::default(); file_size / 8];
            bi.iter_mut().zip(buffer.chunks(8)).for_each(|(out, bin)| {
                *out = GoldilocksField::from_bytes_le(bin);
            });

            //zkvm_evm_prove_only(&ctx.task_name, &serde_data, bi, i, output_path).unwrap();
            log::info!(
                "circom file path: {:?}",
                format!(
                    "{}/{}_chunk_{}_proof.bin_1",
                    ctx.evm_output, ctx.task_name, i
                )
            );
            log::info!(
                "zkin file path: {:?}",
                format!(
                    "{}/{}_chunk_{}_proof.bin_0",
                    ctx.evm_output, ctx.task_name, i
                )
            );
            std::fs::copy(
                format!(
                    "{}/{}_chunk_{}_proof.bin_1",
                    ctx.evm_output, ctx.task_name, i
                ),
                c12_circom.circom_file.clone(),
            );
            std::fs::copy(
                format!(
                    "{}/{}_chunk_{}_proof.bin_0",
                    ctx.evm_output, ctx.task_name, i
                ),
                c12_stark.zkin.clone(),
            );

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

            // 2. Compile circom circuit to r1cs, and generate witness
            circom_compiler(
                c12_circom.circom_file.clone(),
                "goldilocks".to_string(), // prime
                "full".to_string(),       // full_simplification
                c12_circom.link_directories.clone(),
                c12_circom.output.clone(),
                false, // no_simplification
                false, // reduced_simplification
            )
            .unwrap();
            log::info!("end batch prove");

            log::info!("start c12 prove: {:?}", c12_stark);
            log::info!("1. compress setup");
            setup(
                &c12_stark.r1cs_file,
                &c12_stark.pil_file,
                &c12_stark.const_file,
                &c12_stark.exec_file,
                0,
            );

            let wasm_file = format!(
                "{}/{}.c12_js/{}.c12.wasm",
                c12_circom.output, ctx.task_name, ctx.task_name
            );
            log::info!("2. compress exec: {wasm_file}");
            exec(
                &c12_stark.zkin,
                &wasm_file,
                &c12_stark.pil_file,
                &c12_stark.exec_file,
                &c12_stark.commit_file,
            );

            // 3. stark prove
            stark_prove(
                &ctx.c12_struct,
                &c12_stark.piljson,
                true,
                true,
                &c12_stark.const_file,
                &c12_stark.commit_file,
                &r1_circom.circom_file,
                &r1_stark.zkin,
                "",
            );
            log::info!("end c12 prove");
        });
        Ok(())
    }
}
