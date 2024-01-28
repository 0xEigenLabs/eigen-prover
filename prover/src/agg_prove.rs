use crate::traits::StageProver;
use crate::{AggContext, BatchContext};
use anyhow::Result;
use dsl_compile::circom_compiler;
use recursion::{compressor12_exec::exec, compressor12_setup::setup};

use starky::prove::stark_prove;
use starky::zkin_join::join_zkin;

pub struct AggProver {}
impl AggProver {
    pub fn new() -> Self {
        AggProver {}
    }
}

impl StageProver for AggProver {
    fn agg_prove(&self, ctx: &AggContext) -> Result<()> {
        log::info!("start aggregate prove");

        // 1. Compile circom circuit to r1cs, and generate witness
        let task_id_slice: Vec<_> = ctx.input.split("_chunk_").collect();
        let task_id2_slice: Vec<_> = ctx.input2.split("_chunk_").collect();
        let batch_ctx =
            BatchContext::new(&ctx.basedir, &ctx.input, &ctx.task_name, task_id_slice[1]);
        let batch2_ctx =
            BatchContext::new(&ctx.basedir, &ctx.input2, &ctx.task_name, task_id2_slice[1]);

        log::info!("batch_ctx: {:?}", batch_ctx);
        log::info!("batch2_ctx: {:?}", batch2_ctx);

        let sp = &ctx.agg_stark;
        let cc = &ctx.agg_circom;

        let r1_stark = &batch_ctx.recursive1_stark;
        let r1_circom = &batch_ctx.recursive1_circom;

        log::info!("agg_stark: {:?}", sp);
        log::info!("agg_circom: {:?}", cc);
        circom_compiler(
            r1_circom.circom_file.clone(),
            "goldilocks".to_string(),
            "full".to_string(),
            cc.link_directories.clone(),
            r1_circom.output.clone(),
            false,
            false,
        )?;

        // 2. compress inputs
        let zkin = format!(
            "{}/proof/{}/batch_proof/{}.recursive1.zkin.json",
            ctx.basedir, ctx.input, ctx.task_name,
        );
        let zkin2 = format!(
            "{}/proof/{}/batch_proof/{}.recursive1.zkin.json",
            ctx.basedir, ctx.input2, ctx.task_name,
        );

        log::info!("join {} {} -> {}", zkin, zkin2, ctx.agg_zkin);
        join_zkin(&zkin, &zkin2, &ctx.agg_zkin)?;

        // 3. compress setup
        setup(
            &r1_stark.r1cs_file,
            &r1_stark.pil_file,
            &r1_stark.const_file,
            &r1_stark.exec_file,
            0,
        )?;

        // 4. compress exec
        // TODO: place it in StarkProveArgs?
        let wasm_file = format!(
            "{}/{}.recursive1_js/{}.recursive1.wasm",
            r1_circom.output, ctx.task_name, ctx.task_name
        );
        log::info!("wasm_file: {}", wasm_file);
        exec(
            &ctx.agg_zkin,
            &wasm_file,
            &r1_stark.pil_file,
            &r1_stark.exec_file,
            &r1_stark.commit_file,
        )?;

        // 5. stark prove
        log::info!("recursive2: {:?} -> {:?}", r1_stark, cc);
        stark_prove(
            &ctx.agg_struct,
            &r1_stark.piljson,
            true,
            false,
            &r1_stark.const_file,
            &r1_stark.commit_file,
            &cc.circom_file,
            &sp.zkin,
            "",
        )?;

        log::info!("end aggregate prove");
        Ok(())
    }
}
