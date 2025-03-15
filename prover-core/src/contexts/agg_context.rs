use serde::{Deserialize, Serialize};

use super::ProveDataCache;
use crate::args::CircomCompileArgs;
use crate::args::StarkProveArgs;
use crate::stage::Stage;
use std::default;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AggContext {
    pub agg_circom: CircomCompileArgs,
    pub agg_stark: StarkProveArgs,
    pub agg_struct: String,
    pub agg_zkin: String,

    pub basedir: String,
    pub input: String,
    pub input2: String,
    pub task_name: String,
    pub r2_task_name: String,
    pub prove_data_cache: Arc<Mutex<ProveDataCache>>,
    pub force_bits: usize,
    pub task_path: String,

    pub elf_path: String,
    pub aggregate_elf_path: String,
}

impl AggContext {
    pub fn new(
        basedir: &str,
        task_id: &str,
        task_name: &str,
        input: String,
        input2: String,
        force_bits: usize,
        prove_data_cache: Arc<Mutex<ProveDataCache>>,
    ) -> Self {
        let task_path = Stage::Aggregate(task_id.to_string(), input.clone(), input2.clone()).path();
        let r2_task_name = format!("{}.recursive2", task_name);

        AggContext {
            task_path: task_path.clone(),
            basedir: basedir.to_string(),
            task_name: task_name.to_string(),
            input,
            input2,
            agg_stark: StarkProveArgs::new(basedir, &task_path, &r2_task_name, "GL"),
            agg_circom: CircomCompileArgs::new("", basedir, &r2_task_name, &task_path, "", 0, "GL"),

            agg_zkin: format!("{}/proof/{}/agg_zkin.json", basedir, task_id),
            agg_struct: format!("{}/{}/c12.stark_struct.json", basedir, task_name), // should be same as c12
            r2_task_name,
            prove_data_cache,
            force_bits,
            ..Default::default()
        }
    }

    pub fn new_sp1(elf_path: String, aggregate_elf_path: String) -> Self {
        Self {
            elf_path,
            aggregate_elf_path,
            ..Default::default()
        }
    }
}
