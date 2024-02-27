use serde::{Deserialize, Serialize};

use crate::args::CircomCompileArgs;
use crate::args::StarkProveArgs;
use crate::stage::Stage;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BatchContext {
    pub basedir: String,
    pub batch_stark: StarkProveArgs,
    pub batch_struct: String,
    pub c12_circom: CircomCompileArgs,
    pub c12_stark: StarkProveArgs,
    pub c12_struct: String,
    pub chunk_id: String,
    pub evm_output: String,
    pub recursive1_circom: CircomCompileArgs,
    pub recursive1_stark: StarkProveArgs,
    pub task_id: String,
    pub task_name: String,
}

impl BatchContext {
    pub fn new(basedir: &str, task_id: &str, task_name: &str, chunk_id: &str) -> Self {
        let executor_dir = format!("{}/executor/{}", basedir, task_id);
        let task_path = Stage::Batch(task_id.to_string(), chunk_id.to_string()).path();
        let c12_task_name = format!("{}.c12", task_name);

        let r1_task_name = format!("{}.recursive1", task_name);

        BatchContext {
            basedir: basedir.to_string(),
            task_id: task_id.to_string(),
            task_name: task_name.to_string(),
            batch_struct: format!("{}/batch.stark_struct.json", basedir),
            c12_struct: format!("{}/c12.stark_struct.json", basedir),

            batch_stark: StarkProveArgs {
                commit_file: format!("{}/{}.cm", executor_dir, task_name),
                const_file: format!("{}/{}.const", executor_dir, task_name),
                curve_type: "GL".to_string(),
                exec_file: "".to_string(),
                pil_file: "".to_string(),
                piljson: format!("{}/{}.pil.json", executor_dir, task_name),
                r1cs_file: "".to_string(),
                zkin: format!("{}/test.json", basedir),
            },

            evm_output: format!("{basedir}/{task_path}/../{task_name}",),
            chunk_id: chunk_id.to_string(),
            c12_stark: StarkProveArgs::new(basedir, &task_path, &c12_task_name, "GL"),
            c12_circom: CircomCompileArgs::new(basedir, &task_path, &c12_task_name, "GL"),

            recursive1_stark: StarkProveArgs::new(basedir, &task_path, &r1_task_name, "GL"),
            recursive1_circom: CircomCompileArgs::new(basedir, &task_path, &r1_task_name, "GL"),
        }
    }
}
