use super::Prover;
use crate::contexts::AggContext;
use crate::contexts::BatchContext;

use anyhow::Result;
use dsl_compile::circom_compiler;
use recursion::{compressor12_exec::exec, compressor12_setup::setup};

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
        log::info!("start aggregate prove");

        // 1. Compile circom circuit to r1cs, and generate witness
        let task_id_slice: Vec<_> = ctx.input.split("_chunk_").collect();
        let task_id2_slice: Vec<_> = ctx.input2.split("_chunk_").collect();
        assert_eq!(task_id_slice[0], task_id2_slice[0]);

        let start: usize = task_id_slice[1].parse::<usize>().unwrap();
        let end: usize = task_id2_slice[1].parse::<usize>().unwrap();
        let mut batch_ctx = vec![];
        for i in start..=end {
            batch_ctx.push(BatchContext::new(
                &ctx.basedir,
                task_id_slice[0],
                &ctx.task_name,
                &format!("{}", i),
            ));
            log::info!("batch_ctx[{}]: {:?}", i, batch_ctx[i]);
        }

        let sp = &ctx.agg_stark;
        let cc = &ctx.agg_circom;

        let r1_stark = &batch_ctx[0].recursive1_stark;
        let r1_circom = &batch_ctx[0].recursive1_circom;

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
            "{}/proof/{}/batch_proof_{}/{}.recursive1.zkin.json",
            ctx.basedir, task_id_slice[0], start, ctx.task_name,
        );
        let zkin2 = format!(
            "{}/proof/{}/batch_proof_{}/{}.recursive1.zkin.json",
            ctx.basedir,
            task_id_slice[0],
            start + 1,
            ctx.task_name,
        );

        log::info!("join {} {} -> {}", zkin, zkin2, ctx.agg_zkin);
        join_zkin(&zkin, &zkin2, &ctx.agg_zkin)?;
        let force_bits = std::env::var("FORCE_BIT").unwrap_or("0".to_string());
        let force_bits = force_bits
            .parse::<usize>()
            .unwrap_or_else(|_| panic!("Can not parse {} to usize", force_bits));
        // 3. compress setup
        setup(
            &r1_stark.r1cs_file,
            &r1_stark.pil_file,
            &r1_stark.const_file,
            &r1_stark.exec_file,
            force_bits,
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
        let prev_zkin_out = format!(
            "{}/proof/{}/batch_proof_{}/{}_input.recursive1.zkin.json",
            ctx.basedir, task_id_slice[0], 0, ctx.task_name,
        );
        stark_prove(
            &ctx.agg_struct,
            &r1_stark.piljson,
            true,
            false,
            false,
            &r1_stark.const_file,
            &r1_stark.commit_file,
            &cc.circom_file,
            &prev_zkin_out,
            "",
        )?;

        #[allow(clippy::needless_range_loop)]
        for i in 2..=end {
            let zkin = format!(
                "{}/proof/{}/batch_proof_{}/{}.recursive1.zkin.json",
                ctx.basedir, task_id_slice[0], i, ctx.task_name,
            );
            let zkin_out = format!(
                "{}/proof/{}/batch_proof_{}/{}_input.recursive1.zkin.json",
                ctx.basedir, task_id_slice[0], i, ctx.task_name,
            );
            let r_stark = &batch_ctx[i].recursive1_stark;

            log::info!("join {} {} -> {}", prev_zkin_out, zkin, zkin_out);
            join_zkin(&prev_zkin_out, &zkin, &zkin_out)?;

            exec(
                &zkin_out,
                &wasm_file,
                &r1_stark.pil_file,
                &r1_stark.exec_file,
                &r_stark.commit_file,
            )?;

            stark_prove(
                &ctx.agg_struct,
                &r1_stark.piljson,
                true,
                false,
                false,
                &r1_stark.const_file,
                &r_stark.commit_file,
                &cc.circom_file,
                &prev_zkin_out,
                "",
            )?;
        }
        std::fs::copy(prev_zkin_out, sp.zkin.clone())?;

        log::info!("end aggregate prove");
        Ok(())
    }
}
