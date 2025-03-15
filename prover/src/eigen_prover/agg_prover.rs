// use super::Prover;
use prover_core::contexts::AggContext;
use prover_core::contexts::BatchContext;
use prover_core::contexts::{CacheStage, StarkFileType};
use prover_core::prover::Prover;

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
        let mut prove_data_cache = ctx.prove_data_cache.lock().unwrap();

        // 1. Compile circom circuit to r1cs, and generate witness
        // FIXME: extract all the proof and verifier from the request.
        let task_id_slice: Vec<_> = ctx.input.split("_chunk_").collect();
        let task_id2_slice: Vec<_> = ctx.input2.split("_chunk_").collect();
        assert_eq!(task_id_slice[0], task_id2_slice[0]);
        let start: usize = task_id_slice[1].parse::<usize>().unwrap();
        let end: usize = task_id2_slice[1].parse::<usize>().unwrap();

        let mut ids = vec![];
        for i in start..=end {
            let mut f = std::fs::File::open(format!("{}/{}.ids", ctx.task_path, i))?;
            let mut submachine_id_vec: Vec<usize> = serde_json::from_reader(&f)?;
            ids.push(submachine_id_vec);
        }

        let mut batch_ctx = vec![];
        for i in start..=end {
            let sub_ids = &ids[i - start];
            for j in sub_ids {
                batch_ctx.push(BatchContext::new(
                    &ctx.basedir,
                    task_id_slice[0],
                    &ctx.task_name,
                    &format!("{}", i),
                    "".to_string(), // don't have to init the l2_batch_data when aggregate proof
                    ctx.force_bits,
                    ..Default::default()
                ));
                log::info!("batch_ctx[{}]: {:?}", i, batch_ctx[i]);
            }
        }

        let cc = &ctx.agg_circom;

        let r1_stark = batch_ctx[0].get_stark(&batch_ctx[0].r1_task_name, 0);
        let r1_circom = batch_ctx[0].get_circom(&batch_ctx[0].r1_task_name, 0);

        log::info!("agg_circom: {:?}", cc);

        let mut cached_files = vec![];
        if !prove_data_cache.agg_cache.already_cached {
            circom_compiler(
                r1_circom.circom(true),
                "goldilocks".to_string(),
                "full".to_string(),
                cc.link_directories.clone(),
                r1_circom.zkin(true),
                false,
                false,
            )?;

            cached_files.extend_from_slice(&[
                (r1_stark.r1cs_file.clone(), CacheStage::Agg(StarkFileType::R1cs)),
                (r1_stark.wasm_file.clone(), CacheStage::Agg(StarkFileType::Wasm)),
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
        let setup_start = std::time::Instant::now();
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
                (r1_stark.pil_file.clone(), CacheStage::Agg(StarkFileType::Pil)),
                (r1_stark.const_file.clone(), CacheStage::Agg(StarkFileType::Const)),
                (r1_stark.exec_file.clone(), CacheStage::Agg(StarkFileType::Exec)),
                (
                    format!("{}.json", r1_stark.pil_file.clone()),
                    CacheStage::Agg(StarkFileType::PilJson),
                ),
            ]);
            prove_data_cache.batch_add(cached_files)?;
        }
        let setup_elapsed = setup_start.elapsed();
        metrics::PROMETHEUS_METRICS.lock().unwrap().observe_prover_processing_time_gauge(
            Step::Agg,
            Function::Setup,
            setup_elapsed.as_secs_f64(),
        );

        // 4. compress exec
        log::info!("wasm_file: {}", prove_data_cache.agg_cache.wasm_file);
        let exec_start = std::time::Instant::now();
        exec(
            &ctx.agg_zkin,
            &prove_data_cache.agg_cache.wasm_file,
            &prove_data_cache.agg_cache.pil_file,
            &prove_data_cache.agg_cache.exec_file,
            &r1_stark.commit_file,
        )?;

        let exec_elapsed = exec_start.elapsed();
        metrics::PROMETHEUS_METRICS.lock().unwrap().observe_prover_processing_time_gauge(
            Step::Agg,
            Function::Exec,
            exec_elapsed.as_secs_f64(),
        );

        // 5. stark prove
        log::info!("recursive2: {:?} -> {:?}", r1_stark, cc);
        let prev_zkin_out = format!(
            "{}/proof/{}/batch_proof_{}/{}_input.recursive1.zkin.json",
            ctx.basedir, task_id_slice[0], 0, ctx.task_name,
        );

        let stark_prove_start = std::time::Instant::now();
        stark_prove(
            &ctx.agg_struct,
            &prove_data_cache.agg_cache.piljson_file,
            true,
            false,
            false,
            &prove_data_cache.agg_cache.const_file,
            &r1_stark.commit_file,
            &cc.circom(true),
            &prev_zkin_out,
            "",
        )?;

        let stark_prove_elapsed = stark_prove_start.elapsed();
        metrics::PROMETHEUS_METRICS.lock().unwrap().observe_prover_processing_time_gauge(
            Step::Agg,
            Function::StarkProve,
            stark_prove_elapsed.as_secs_f64(),
        );

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
            // FIXME
            let r_stark = &batch_ctx[i].get_stark(&batch_ctx[i].r1_task_name, 0);

            log::info!("join {} {} -> {}", prev_zkin_out, zkin, zkin_out);
            join_zkin(&prev_zkin_out, &zkin, &zkin_out)?;

            let exec_start = std::time::Instant::now();
            exec(
                &zkin_out,
                &prove_data_cache.agg_cache.wasm_file,
                &prove_data_cache.agg_cache.pil_file,
                &prove_data_cache.agg_cache.exec_file,
                &r_stark.commit_file,
            )?;
            let exec_elapsed = exec_start.elapsed();
            metrics::PROMETHEUS_METRICS.lock().unwrap().observe_prover_processing_time_gauge(
                Step::Agg,
                Function::Exec,
                exec_elapsed.as_secs_f64(),
            );

            let stark_prove_start = std::time::Instant::now();
            stark_prove(
                &ctx.agg_struct,
                &prove_data_cache.agg_cache.piljson_file,
                true,
                false,
                false,
                &prove_data_cache.agg_cache.const_file,
                &r_stark.commit_file,
                &cc.circom(true),
                &prev_zkin_out,
                "",
            )?;
            let stark_prove_elapsed = stark_prove_start.elapsed();
            metrics::PROMETHEUS_METRICS.lock().unwrap().observe_prover_processing_time_gauge(
                Step::Agg,
                Function::StarkProve,
                stark_prove_elapsed.as_secs_f64(),
            );
        }
        std::fs::copy(prev_zkin_out, cc.zkin(true))?;

        log::info!("end aggregate prove");
        let prove_elapsed = prove_start.elapsed();
        metrics::PROMETHEUS_METRICS.lock().unwrap().observe_prover_processing_time_gauge(
            Step::Agg,
            Function::Total,
            prove_elapsed.as_secs_f64(),
        );
        Ok(())
    }
}
