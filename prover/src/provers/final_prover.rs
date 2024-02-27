use super::Prover;
use crate::contexts::FinalContext;

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

        // 1. compress setup
        let rc2 = &ctx.recursive2_circom;
        let r2 = &ctx.recursive2_stark;
        let sp = &ctx.final_stark;
        let cc = &ctx.final_circom;
        log::info!("rc2: {:?}", rc2);
        log::info!("r2: {:?}", r2);
        log::info!("sp: {:?}", sp);
        log::info!("cc: {:?}", cc);
        circom_compiler(
            rc2.circom_file.clone(),
            "goldilocks".to_string(),
            "full".to_string(),
            rc2.link_directories.clone(),
            cc.output.clone(),
            false,
            false,
        )?;

        log::info!("setup");

        setup(
            &r2.r1cs_file,
            &r2.pil_file,
            &r2.const_file,
            &r2.exec_file,
            0,
        )?;

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
            true,
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

        circom_compiler(
            cc.circom_file.clone(),
            args.curve_type.to_lowercase(),
            "full".to_string(),
            cc.link_directories.clone(),
            cc.output.clone(),
            false,
            false,
        )?;
        let wasm_file = format!(
            "{}/{}.final_js/{}.final.wasm",
            cc.output, ctx.task_name, ctx.task_name
        );

        log::info!("start snark prove");
        groth16_setup(
            &args.curve_type,
            &sp.r1cs_file,
            &args.pk_file,
            &args.vk_file,
            false,
        )?;
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
