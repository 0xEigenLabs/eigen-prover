use super::Prover;
use crate::contexts::FinalContext;
use crate::contexts::{CacheStage, SnarkFileType, StarkFileType};

use anyhow::Result;
use dsl_compile::circom_compiler;
use groth16::api::{groth16_prove, groth16_setup, groth16_verify};
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
        log::info!("start final_stark prove");
        let mut prove_data_cache = ctx.prove_data_cache.lock().unwrap();

        // 1. compress setup
        let rc2 = &ctx.recursive2_circom;
        let r2 = ctx.recursive2_stark.clone();
        let sp = &ctx.final_stark;
        let cc = &ctx.final_circom;

        let mut cached_files = vec![];
        if !prove_data_cache.final_cache.already_cached {
            circom_compiler(
                rc2.circom_file.clone(),
                "goldilocks".to_string(),
                "full".to_string(),
                rc2.link_directories.clone(),
                cc.output.clone(),
                false,
                false,
            )?;
            let wasm_file = format!(
                "{}/{}.recursive2_js/{}.recursive2.wasm",
                cc.output, ctx.task_name, ctx.task_name
            );
            
            cached_files.extend_from_slice(&[
                (r2.r1cs_file.clone(), CacheStage::Final(StarkFileType::R1cs)),
                (wasm_file, CacheStage::Final(StarkFileType::Wasm)),
            ]);

            setup(
                &r2.r1cs_file,
                &r2.pil_file,
                &r2.const_file,
                &r2.exec_file,
                0,
            )?;

            cached_files.extend_from_slice(&[
                (r2.pil_file.clone(), CacheStage::Final(StarkFileType::Pil)),
                (r2.const_file.clone(), CacheStage::Final(StarkFileType::Const)),
                (r2.exec_file.clone(), CacheStage::Final(StarkFileType::Exec)),
                (format!("{}.json", r2.pil_file), CacheStage::Final(StarkFileType::PilJson)),
            ]);
        }

        log::info!("2. compress exec");
        // let wasm_file = format!(
        //     "{}/{}.recursive2_js/{}.recursive2.wasm",
        //     cc.output, ctx.task_name, ctx.task_name
        // );
        exec(
            &r2.zkin,
            &prove_data_cache.final_cache.wasm_file,
            &prove_data_cache.final_cache.pil_file,
            &prove_data_cache.final_cache.exec_file,
            &r2.commit_file,
        )?;

        log::info!("3. generate final proof");
        stark_prove(
            &ctx.final_stark_struct,
            &prove_data_cache.final_cache.piljson_file,
            false,
            false,
            false,
            &prove_data_cache.final_cache.const_file,
            &r2.commit_file,
            &cc.circom_file,
            &sp.zkin,
            &ctx.prover_addr,
        )?;

        log::info!("end final stark prove");
        let args = &ctx.final_snark;

        if !prove_data_cache.snark_cache.already_cached {
            circom_compiler(
                cc.circom_file.clone(),
                args.curve_type.to_lowercase(),
                "full".to_string(),
                cc.link_directories.clone(),
                cc.output.clone(),
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

            let wasm_file = format!(
                "{}/{}.final_js/{}.final.wasm",
                cc.output, ctx.task_name, ctx.task_name
            );
            cached_files.extend_from_slice(&[
                (wasm_file, CacheStage::Snark(SnarkFileType::Wasm)),
                (sp.r1cs_file.clone(), CacheStage::Snark(SnarkFileType::R1cs)),
                (args.pk_file.clone(), CacheStage::Snark(SnarkFileType::PK)),
                (args.vk_file.clone(), CacheStage::Snark(SnarkFileType::VK)),
            ]
            );
            prove_data_cache.batch_add(cached_files)?;
        }

        let curve_cache = &prove_data_cache.snark_cache;

        groth16_prove(
            &args.curve_type,
            &curve_cache.r1cs_file,
            &curve_cache.wasm_file,
            &curve_cache.pk_file,
            &sp.zkin,
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

        log::info!("end snark prove");
        Ok(())
    }
}
