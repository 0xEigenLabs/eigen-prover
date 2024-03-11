use serde::{Deserialize, Serialize};

use crate::args::CircomCompileArgs;
use crate::args::StarkProveArgs;
use crate::stage::Stage;

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
}

impl AggContext {
    pub fn new(
        basedir: &str,
        task_id: &str,
        task_name: &str,
        input: String,
        input2: String,
    ) -> Self {
        let task_path = Stage::Aggregate(task_id.to_string(), input.clone(), input2.clone()).path();
        let r2_task_name = format!("{}.recursive2", task_name);

        AggContext {
            basedir: basedir.to_string(),
            task_name: task_name.to_string(),
            input,
            input2,
            agg_zkin: format!("{}/proof/{}/agg_zkin.json", basedir, task_id),
            agg_struct: format!("{}/c12.stark_struct.json", basedir), // should be same as c12
            agg_stark: StarkProveArgs::new(basedir, &task_path, &r2_task_name, "GL"),
            agg_circom: CircomCompileArgs::new(basedir, &task_path, &r2_task_name, "GL"),
        }
    }
}
