use serde::{Deserialize, Serialize};

use super::ProveDataCache;
use crate::args::CircomCompileArgs;
use crate::args::FinalProveArgs;
use crate::args::StarkProveArgs;
use crate::stage::Stage;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FinalContext {
    pub basedir: String,

    pub final_circom: CircomCompileArgs,
    pub final_snark: FinalProveArgs,
    pub final_stark: StarkProveArgs,
    pub final_stark_struct: String,

    pub prover_addr: String,
    pub recursive2_circom: CircomCompileArgs,
    pub recursive2_stark: StarkProveArgs,

    pub task_id: String,
    pub task_name: String,
    pub prove_data_cache: Arc<Mutex<ProveDataCache>>,
}

impl FinalContext {
    pub fn new(
        basedir: String,
        task_id: String,
        task_name: String,
        curve: String,
        prover_addr: String,
        prove_data_cache: Arc<Mutex<ProveDataCache>>,
    ) -> Self {
        let id = format!("{}_agg", task_id);
        let prev_task_path = Stage::Aggregate(id, "".into(), "".into()).path();
        let task_path = Stage::Final(task_id.clone(), curve.clone(), prover_addr.clone()).path();
        let r2_task_name = format!("{}.recursive2", task_name);
        let final_task_name = format!("{}.final", task_name);
        FinalContext {
            basedir: basedir.clone(),
            task_id,
            prover_addr,
            task_name: task_name.clone(),
            final_stark_struct: format!("{}/{}/final.stark_struct.json", basedir, task_name),
            final_stark: StarkProveArgs::new(&basedir, &prev_task_path, &final_task_name, &curve),
            recursive2_stark: StarkProveArgs::new(&basedir, &prev_task_path, &r2_task_name, &curve),
            final_circom: CircomCompileArgs::new(
                "",
                &basedir,
                &final_task_name,
                &prev_task_path,
                "",
                0,
                &curve,
            ),
            recursive2_circom: CircomCompileArgs::new(
                "",
                &basedir,
                &r2_task_name,
                &prev_task_path,
                "",
                0,
                "GL",
            ),
            final_snark: FinalProveArgs {
                curve_type: curve,
                pk_file: format!("{basedir}/{task_path}/g16.key"),
                vk_file: format!("{basedir}/{task_path}/verification_key.json"),
                public_input_file: format!("{basedir}/{task_path}/public_input.json"),
                proof_file: format!("{basedir}/{task_path}/proof.json"),
            },
            prove_data_cache,
        }
    }
}
