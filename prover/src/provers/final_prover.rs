use super::Prover;
use crate::contexts::FinalContext;
use crate::contexts::{CacheStage, Curve, SnarkFileType, StarkFileType};

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
        let mut r2 = ctx.recursive2_stark.clone();
        let sp = &ctx.final_stark;
        let cc = &ctx.final_circom;
        log::info!("rc2: {:?}", rc2);
        log::info!("r2: {:?}", r2);
        log::info!("sp: {:?}", sp);
        log::info!("cc: {:?}", cc);
        if prove_data_cache.final_cache.already_cached {
            r2.r1cs_file
                .clone_from(&prove_data_cache.final_cache.r1cs_file);
            r2.pil_file
                .clone_from(&prove_data_cache.final_cache.pil_file);
            r2.const_file
                .clone_from(&prove_data_cache.final_cache.const_file);
            r2.exec_file
                .clone_from(&prove_data_cache.final_cache.exec_file);
        }

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
        }

        log::info!("setup");

        if !prove_data_cache.final_cache.already_cached {
            setup(
                &r2.r1cs_file,
                &r2.pil_file,
                &r2.const_file,
                &r2.exec_file,
                0,
            )?;

            prove_data_cache.update_cache_flag(CacheStage::Final(StarkFileType::default()));
            let _ =
                prove_data_cache.add(r2.r1cs_file.clone(), CacheStage::Final(StarkFileType::R1cs));
            let _ =
                prove_data_cache.add(r2.pil_file.clone(), CacheStage::Final(StarkFileType::Pil));
            let _ = prove_data_cache.add(
                r2.const_file.clone(),
                CacheStage::Final(StarkFileType::Const),
            );
            let _ =
                prove_data_cache.add(r2.exec_file.clone(), CacheStage::Final(StarkFileType::Exec));
        }

        log::info!("2. compress exec");
        let wasm_file = format!(
            "{}/{}.recursive2_js/{}.recursive2.wasm",
            cc.output, ctx.task_name, ctx.task_name
        );
        exec(
            &r2.zkin,
            &wasm_file,
            &r2.pil_file,
            &r2.exec_file,
            &r2.commit_file,
        )?;

        log::info!("3. generate final proof");
        stark_prove(
            &ctx.final_stark_struct,
            &r2.piljson,
            false,
            false,
            false,
            &r2.const_file,
            &r2.commit_file,
            &cc.circom_file,
            &sp.zkin,
            &ctx.prover_addr,
        )?;

        log::info!("end final stark prove");
        let args = &ctx.final_snark;

        log::info!("start snark prove");
        let snark_already_cached = match args.curve_type.as_str() {
            "BN128" => prove_data_cache.snark_cache.bn128_data.already_cached,
            "BLS12381" => prove_data_cache.snark_cache.bls12381_data.already_cached,
            _ => false,
        };

        if !snark_already_cached {
            log::info!("compile");
            circom_compiler(
                cc.circom_file.clone(),
                args.curve_type.to_lowercase(),
                "full".to_string(),
                cc.link_directories.clone(),
                cc.output.clone(),
                false,
                false,
            )?;
        }
        let wasm_file = format!(
            "{}/{}.final_js/{}.final.wasm",
            cc.output, ctx.task_name, ctx.task_name
        );

        if !snark_already_cached {
            log::info!("setup");
            groth16_setup(
                &args.curve_type,
                &sp.r1cs_file,
                &args.pk_file,
                &args.vk_file,
                false,
            )?;

            match args.curve_type.as_str() {
                "BN128" => {
                    prove_data_cache.update_cache_flag(CacheStage::Snark(Curve::BN128(
                        SnarkFileType::default(),
                    )));
                    let _ = prove_data_cache.add(
                        sp.r1cs_file.clone(),
                        CacheStage::Snark(Curve::BN128(SnarkFileType::R1cs)),
                    );
                    let _ = prove_data_cache.add(
                        args.pk_file.clone(),
                        CacheStage::Snark(Curve::BN128(SnarkFileType::PK)),
                    );
                    let _ = prove_data_cache.add(
                        args.vk_file.clone(),
                        CacheStage::Snark(Curve::BN128(SnarkFileType::VK)),
                    );
                }
                "BLS12381" => {
                    prove_data_cache.update_cache_flag(CacheStage::Snark(Curve::BLS12381(
                        SnarkFileType::default(),
                    )));
                    let _ = prove_data_cache.add(
                        sp.r1cs_file.clone(),
                        CacheStage::Snark(Curve::BLS12381(SnarkFileType::R1cs)),
                    );
                    let _ = prove_data_cache.add(
                        args.pk_file.clone(),
                        CacheStage::Snark(Curve::BLS12381(SnarkFileType::PK)),
                    );
                    let _ = prove_data_cache.add(
                        args.vk_file.clone(),
                        CacheStage::Snark(Curve::BLS12381(SnarkFileType::VK)),
                    );
                }
                _ => {
                    log::warn!("unsupport cache: {}", args.curve_type);
                }
            }
        }

        log::info!("prove");
        groth16_prove(
            &args.curve_type,
            &sp.r1cs_file,
            &wasm_file,
            &args.pk_file,
            &sp.zkin,
            &args.public_input_file,
            &args.proof_file,
            false,
        )?;

        log::info!("verify");
        groth16_verify(
            &args.curve_type,
            &args.vk_file,
            &args.public_input_file,
            &args.proof_file,
        )?;

        log::info!("end snark prove");
        Ok(())
    }
}
