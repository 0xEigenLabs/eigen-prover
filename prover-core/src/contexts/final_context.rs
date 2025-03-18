use serde::{Deserialize, Serialize};

use super::ProveDataCache;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FinalContext {
    pub basedir: String,
    pub prover_addr: String,
    pub agg_task_id: String,
    pub task_name: String,
    pub prove_data_cache: Arc<Mutex<ProveDataCache>>,
}

impl FinalContext {
    pub fn new(
        basedir: String,
        agg_task_id: String,
        task_name: String,
        prover_addr: String,
        prove_data_cache: Arc<Mutex<ProveDataCache>>,
    ) -> Self {
        FinalContext {
            basedir: basedir.clone(),
            agg_task_id,
            prover_addr,
            task_name: task_name.clone(),
            prove_data_cache,
        }
    }
}
