use crate::prover::Prover;
use crate::contexts::FinalContext;
use crate::contexts::{CacheStage, SnarkFileType, StarkFileType};

use anyhow::Result;
use dsl_compile::circom_compiler;
use groth16::api::{groth16_prove, groth16_setup, groth16_verify};
use metrics::{Function, Step};
use recursion::{compressor12_exec::exec, compressor12_setup::setup};
use starky::prove::stark_prove;

#[derive(Default)]
pub struct FinalProver {}

impl FinalProver {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Prover<FinalContext> for FinalProver {
    fn prove(&self, ctx: &FinalContext) -> Result<()> {
        log::info!("start final_stark prove, ctx: {:?}", ctx);
        let prove_start = std::time::Instant::now();
        let mut prove_data_cache = ctx.prove_data_cache.lock().unwrap();

        // 1. compress setup
        let rc2 = &ctx.recursive2_circom;
        let r2 = ctx.recursive2_stark.clone();
        let sp = &ctx.final_stark;
        let cc = &ctx.final_circom;

        let mut cached_files = vec![];

        let setup_start = std::time::Instant::now();
        if !prove_data_cache.final_cache.already_cached {
            circom_compiler(
                rc2.circom(true),
                "goldilocks".to_string(),
                "full".to_string(),
                rc2.link_directories.clone(),
                rc2.zkin(true),
                false,
                false,
            )?;

            cached_files.extend_from_slice(&[
                (r2.r1cs_file.clone(), CacheStage::Final(StarkFileType::R1cs)),
                (r2.wasm_file.clone(), CacheStage::Final(StarkFileType::Wasm)),
            ]);

            setup(
                &r2.r1cs_file,
                &r2.pil_file,
                &r2.const_file,
                &r2.exec_file,
                0,
            )?;
            let _ = std::fs::copy(r2.pil_file.clone(), r2.piljson.clone());
            cached_files.extend_from_slice(&[
                (r2.pil_file.clone(), CacheStage::Final(StarkFileType::Pil)),
                (
                    r2.const_file.clone(),
                    CacheStage::Final(StarkFileType::Const),
                ),
                (r2.exec_file.clone(), CacheStage::Final(StarkFileType::Exec)),
                (
                    format!("{}.json", r2.pil_file),
                    CacheStage::Final(StarkFileType::PilJson),
                ),
            ]);
            prove_data_cache.batch_add(cached_files.clone())?;
        }
        let setup_elapsed = setup_start.elapsed();
        metrics::PROMETHEUS_METRICS
            .lock()
            .unwrap()
            .observe_prover_processing_time_gauge(
                Step::Final,
                Function::Setup,
                setup_elapsed.as_secs_f64(),
            );

        log::info!("2. compress exec");
        // let wasm_file = format!(
        //     "{}/{}.recursive2_js/{}.recursive2.wasm",
        //     cc.output, ctx.task_name, ctx.task_name
        // );
        let exec_start = std::time::Instant::now();
        exec(
            &rc2.zkin(true),
            &prove_data_cache.final_cache.wasm_file,
            &prove_data_cache.final_cache.pil_file,
            &prove_data_cache.final_cache.exec_file,
            &r2.commit_file,
        )?;
        let exec_elapsed = exec_start.elapsed();
        metrics::PROMETHEUS_METRICS
            .lock()
            .unwrap()
            .observe_prover_processing_time_gauge(
                Step::Final,
                Function::Exec,
                exec_elapsed.as_secs_f64(),
            );

        log::info!("3. generate final proof");
        let stark_prove_start = std::time::Instant::now();
        stark_prove(
            &ctx.final_stark_struct,
            &prove_data_cache.final_cache.piljson_file,
            false,
            false,
            false,
            &prove_data_cache.final_cache.const_file,
            &r2.commit_file,
            &cc.circom(true),
            &cc.zkin(true),
            &ctx.prover_addr,
        )?;
        let stark_prove_elapsed = stark_prove_start.elapsed();
        metrics::PROMETHEUS_METRICS
            .lock()
            .unwrap()
            .observe_prover_processing_time_gauge(
                Step::Final,
                Function::StarkProve,
                stark_prove_elapsed.as_secs_f64(),
            );

        log::info!("end final stark prove");
        let args = &ctx.final_snark;

        if !prove_data_cache.snark_cache.already_cached {
            circom_compiler(
                cc.circom(true),
                args.curve_type.to_lowercase(),
                "full".to_string(),
                cc.link_directories.clone(),
                cc.zkin(true),
                false,
                false,
            )?;
            groth16_setup(
                &args.curve_type,
                &sp.r1cs_file,
                &args.pk_file,
                &args.vk_file,
                false,
            )?;

            cached_files.extend_from_slice(&[
                (sp.wasm_file.clone(), CacheStage::Snark(SnarkFileType::Wasm)),
                (sp.r1cs_file.clone(), CacheStage::Snark(SnarkFileType::R1cs)),
                (args.pk_file.clone(), CacheStage::Snark(SnarkFileType::PK)),
                (args.vk_file.clone(), CacheStage::Snark(SnarkFileType::VK)),
            ]);
            prove_data_cache.batch_add(cached_files)?;
        }

        let curve_cache = &prove_data_cache.snark_cache;

        groth16_prove(
            &args.curve_type,
            &curve_cache.r1cs_file,
            &curve_cache.wasm_file,
            &curve_cache.pk_file,
            &cc.zkin(true),
            &args.public_input_file,
            &args.proof_file,
            false,
        )?;

        groth16_verify(
            &args.curve_type,
            &curve_cache.vk_file,
            &args.public_input_file,
            &args.proof_file,
        )?;

        let prove_elapsed = prove_start.elapsed();
        metrics::PROMETHEUS_METRICS
            .lock()
            .unwrap()
            .observe_prover_processing_time_gauge(
                Step::Final,
                Function::Total,
                prove_elapsed.as_secs_f64(),
            );
        log::info!("end snark prove");
        Ok(())
    }
}
