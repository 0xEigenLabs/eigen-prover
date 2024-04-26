use serde::{Deserialize, Serialize};

use crate::args::CircomCompileArgs;
use crate::args::StarkProveArgs;
use crate::stage::Stage;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BatchContext {
    pub basedir: String,
    pub l2_data: String,
    pub batch_circom: CircomCompileArgs,
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
        let batch_task_name = format!("{}.verifier", task_name);
        let c12_task_name = format!("{}.c12", task_name);

        let r1_task_name = format!("{}.recursive1", task_name);
        let suite_json = std::env::var("JSON").unwrap_or(format!("{}/test.json", basedir));

        BatchContext {
            basedir: basedir.to_string(),
            task_id: task_id.to_string(),
            task_name: task_name.to_string(),
            batch_struct: format!("{}/{}/batch.stark_struct.json", basedir, task_name),
            c12_struct: format!("{}/{}/c12.stark_struct.json", basedir, task_name),
            batch_circom: CircomCompileArgs::new(basedir, &task_path, &batch_task_name, "GL"),
            l2_data: suite_json,

            batch_stark: StarkProveArgs {
                commit_file: format!("{}/{}.cm", executor_dir, task_name),
                const_file: format!("{}/{}.const", executor_dir, task_name),
                curve_type: "GL".to_string(),
                exec_file: format!("{}/{}.exec", executor_dir, batch_task_name),
                pil_file: format!("{}/{}.pil", executor_dir, batch_task_name),
                piljson: format!("{}/{}.pil.json", executor_dir, batch_task_name),
                r1cs_file: format!("{basedir}/{task_path}/{batch_task_name}.r1cs",),
                zkin: format!("{basedir}/{task_path}/{batch_task_name}.zkin",),
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
