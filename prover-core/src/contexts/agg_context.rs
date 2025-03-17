use serde::{Deserialize, Serialize};

use super::ProveDataCache;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AggContext {
    pub basedir: String,
    pub input: String,
    pub input2: String,
    pub task_name: String,
    pub prove_data_cache: Arc<Mutex<ProveDataCache>>,
    pub task_id: String,

    pub elf_path: String,
    pub aggregate_elf_path: String,
}

impl AggContext {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        basedir: &str,
        task_id: &str,
        task_name: &str,
        input: String,
        input2: String,
        prove_data_cache: Arc<Mutex<ProveDataCache>>,
        elf_path: &str,
        aggregate_elf_path: &str,
    ) -> Self {
        AggContext {
            task_id: task_id.to_string(),
            basedir: basedir.to_string(),
            task_name: task_name.to_string(),
            input,
            input2,
            prove_data_cache,
            elf_path: elf_path.to_string(),
            aggregate_elf_path: aggregate_elf_path.to_string(),
        }
    }
}
