use super::Prover;
use crate::contexts::AggContext;
use crate::contexts::BatchContext;
use crate::contexts::{CacheStage, StarkFileType};

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
        log::info!("start aggregate prove, ctx: {:?}", ctx);
        let mut prove_data_cache = ctx.prove_data_cache.lock().unwrap();

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
                "".to_string(), // don't have to init the l2_batch_data when aggregate proof
                ctx.force_bits,
            ));
            log::info!("batch_ctx[{}]: {:?}", i, batch_ctx[i]);
        }

        let sp = &ctx.agg_stark;
        let cc = &ctx.agg_circom;

        let r1_stark = batch_ctx[0].recursive1_stark.clone();
        let r1_circom = batch_ctx[0].recursive1_circom.clone();

        log::info!("agg_stark: {:?}", sp);
        log::info!("agg_circom: {:?}", cc);

        let mut cached_files = vec![];
        if !prove_data_cache.agg_cache.already_cached {
            circom_compiler(
                r1_circom.circom_file.clone(),
                "goldilocks".to_string(),
                "full".to_string(),
                cc.link_directories.clone(),
                r1_circom.output.clone(),
                false,
                false,
            )?;

            cached_files.extend_from_slice(&[
                (
                    r1_stark.r1cs_file.clone(),
                    CacheStage::Agg(StarkFileType::R1cs),
                ),
                (
                    r1_stark.wasm_file.clone(),
                    CacheStage::Agg(StarkFileType::Wasm),
                ),
            ])
        }

        // 2. compress inputs
        let zkin = format!(
            "{}/proof/{}/batch_proof_{}/{}.recursive1.zkin.json",
            ctx.basedir, task_id_slice[0], start, ctx.task_name,
        );

        // FIXME: if there is only one chunk for current block, just aggregate the chunk with itself.
        let zkin2 = if end > start {
            format!(
                "{}/proof/{}/batch_proof_{}/{}.recursive1.zkin.json",
                ctx.basedir,
                task_id_slice[0],
                start + 1,
                ctx.task_name,
            )
        } else {
            zkin.clone()
        };
        log::info!("aggregate chunks {start} -> {end}");

        log::info!("join {} {} -> {}", zkin, zkin2, ctx.agg_zkin);
        join_zkin(&zkin, &zkin2, &ctx.agg_zkin)?;
        // 3. compress setup
        if !prove_data_cache.agg_cache.already_cached {
            setup(
                &r1_stark.r1cs_file,
                &r1_stark.pil_file,
                &r1_stark.const_file,
                &r1_stark.exec_file,
                ctx.force_bits,
            )?;

            let _ = std::fs::copy(r1_stark.pil_file.clone(), r1_stark.piljson.clone());
            // add r1cs pil, const, exec to cache and update flag
            cached_files.extend_from_slice(&[
                (
                    r1_stark.pil_file.clone(),
                    CacheStage::Agg(StarkFileType::Pil),
                ),
                (
                    r1_stark.const_file.clone(),
                    CacheStage::Agg(StarkFileType::Const),
                ),
                (
                    r1_stark.exec_file.clone(),
                    CacheStage::Agg(StarkFileType::Exec),
                ),
                (
                    format!("{}.json", r1_stark.pil_file.clone()),
                    CacheStage::Agg(StarkFileType::PilJson),
                ),
            ]);
            prove_data_cache.batch_add(cached_files)?;
        }

        // 4. compress exec
        log::info!("wasm_file: {}", prove_data_cache.agg_cache.wasm_file);
        exec(
            &ctx.agg_zkin,
            &prove_data_cache.agg_cache.wasm_file,
            &prove_data_cache.agg_cache.pil_file,
            &prove_data_cache.agg_cache.exec_file,
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
            &prove_data_cache.agg_cache.piljson_file,
            true,
            false,
            false,
            &prove_data_cache.agg_cache.const_file,
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
                &prove_data_cache.agg_cache.wasm_file,
                &prove_data_cache.agg_cache.pil_file,
                &prove_data_cache.agg_cache.exec_file,
                &r_stark.commit_file,
            )?;

            stark_prove(
                &ctx.agg_struct,
                &prove_data_cache.agg_cache.piljson_file,
                true,
                false,
                false,
                &prove_data_cache.agg_cache.const_file,
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
