use serde::{Deserialize, Serialize};

use crate::args::CircomCompileArgs;
use crate::args::StarkProveArgs;
use crate::stage::Stage;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BatchContext {
    pub basedir: String,
    pub l2_batch_data: String,
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
    pub force_bits: usize,
}

impl BatchContext {
    pub fn new(
        basedir: &str,
        task_id: &str,
        task_name: &str,
        chunk_id: &str,
        l2_batch_data: String,
        force_bits: usize,
    ) -> Self {
        let executor_dir = format!("{}/executor/{}", basedir, task_id);
        std::fs::create_dir_all(&executor_dir).unwrap();

        //TODO : don't clone the l2 batch data
        let task_path = Stage::Batch(
            task_id.to_string(),
            chunk_id.to_string(),
            l2_batch_data.clone(),
        )
        .path();
        let batch_task_name = format!("{}", task_name);
        let c12_task_name = format!("{}.c12", task_name);

        let r1_task_name = format!("{}.recursive1", task_name);
        let evm_output = format!("{basedir}/{task_path}/../{task_name}",);

        BatchContext {
            basedir: basedir.to_string(),
            l2_batch_data,
            task_id: task_id.to_string(),
            task_name: task_name.to_string(),
            batch_struct: format!("{}/{}/batch.stark_struct.json", basedir, task_name),
            c12_struct: format!("{}/{}/c12.stark_struct.json", basedir, task_name),
            batch_circom: CircomCompileArgs::new_batch(
                &evm_output,
                basedir,
                &task_path,
                &batch_task_name,
                chunk_id,
                "GL",
            ),

            batch_stark: StarkProveArgs {
                commit_file: format!("{}/{}.cm", executor_dir, task_name),
                const_file: format!("{}/{}.const", executor_dir, task_name),
                curve_type: "GL".to_string(),
                exec_file: format!("{}/{}.exec", executor_dir, batch_task_name),
                pil_file: format!("{}/{}.pil", executor_dir, batch_task_name),
                piljson: format!("{}/{}.pil.json", executor_dir, batch_task_name),
                r1cs_file: format!("{basedir}/{task_path}/{batch_task_name}.r1cs",),
                zkin: format!("{evm_output}/{task_name}_chunk_{chunk_id}/{task_name}_proof.bin",),
            },

            evm_output,
            chunk_id: chunk_id.to_string(),
            c12_stark: StarkProveArgs::new(basedir, &task_path, &c12_task_name, "GL"),
            c12_circom: CircomCompileArgs::new(basedir, &task_path, &c12_task_name, "GL"),

            recursive1_stark: StarkProveArgs::new(basedir, &task_path, &r1_task_name, "GL"),
            recursive1_circom: CircomCompileArgs::new(basedir, &task_path, &r1_task_name, "GL"),
            force_bits,
        }
    }
}
