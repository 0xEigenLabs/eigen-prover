use serde::{Deserialize, Serialize};

use crate::args::CircomCompileArgs;
use crate::args::StarkProveArgs;
use crate::stage::Stage;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BatchContext {
    pub basedir: String,
    pub l2_batch_data: String,
    pub batch_struct: String,
    pub chunk_id: String,
    pub program_output: String,
    pub task_id: String,
    pub task_name: String,
    
    pub elf_path: String,

    pub c12_task_name: String,
    pub c12_struct: String,
    pub r1_task_name: String,
    pub task_path: String,
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
        elf_path: &str,
    ) -> Self {
        //TODO : don't clone the l2 batch data
        let task_path =
            Stage::Batch(task_id.to_string(), chunk_id.to_string(), l2_batch_data.clone()).path();
        let c12_task_name = format!("{}.c12", task_name);

        let r1_task_name = format!("{}.recursive1", task_name);
        let program_output = format!("{basedir}/{task_path}/../{task_name}",);

        BatchContext {
            basedir: basedir.to_string(),
            l2_batch_data,
            task_id: task_id.to_string(),
            batch_struct: format!("{}/{}/batch.stark_struct.json", basedir, task_name),
            c12_struct: format!("{}/{}/c12.stark_struct.json", basedir, task_name),


            task_path: task_path.clone(),

            program_output,
            chunk_id: chunk_id.to_string(),

            task_name: task_name.to_string(),
            c12_task_name,
            r1_task_name,

            //c12_stark: StarkProveArgs::new(basedir, &task_path, &c12_task_name, "GL"),
            //c12_circom: CircomCompileArgs::new(basedir, basedir, &c12_task_name, &task_path, chunk_id, "GL"),

            //recursive1_stark: StarkProveArgs::new(basedir, &task_path, &r1_task_name, "GL"),
            //recursive1_circom: CircomCompileArgs::new(basedir, basedir, &r1_task_name, &task_path,chunk_id, &"GL"),
            force_bits,

            elf_path: elf_path.to_string(),
        }
    }

    pub fn new_sp1(elf_path: String) -> Self {
        Self {
            elf_path,
            ..Default::default()
        }
    }

    pub fn get_circom(&self, name: &str, submachine_id: usize) -> CircomCompileArgs {
        CircomCompileArgs::new(
            &self.program_output,
            &self.basedir,
            name,
            &self.task_path,
            &self.chunk_id,
            submachine_id,
            "GL",
        )
    }

    pub fn get_stark(&self, name: &str, submachine_id: usize) -> StarkProveArgs {
        StarkProveArgs {
            commit_file: format!(
                "{}/{}_chunk_{}/commits.bin",
                self.program_output, name, self.chunk_id
            ),
            const_file: format!("{}/constants.bin", self.program_output),
            curve_type: "GL".to_string(),
            exec_file: format!(
                "{}/{}/{}_chunk_{}_submachine_{}.exec",
                self.basedir, self.task_path, name, self.chunk_id, submachine_id
            ),
            pil_file: format!(
                "{}/{}/{}_chunk_{}_submachine_{}.pil",
                self.basedir, self.task_path, name, self.chunk_id, submachine_id
            ),
            piljson: format!(
                "{}/{}/{}_chunk_{}_submachine_{}.pil.json",
                self.basedir, self.task_path, name, self.chunk_id, submachine_id
            ),
            r1cs_file: format!(
                "{}/{}/{}_chunk_{}_submachine_{}.r1cs",
                self.basedir, self.task_path, name, self.chunk_id, submachine_id
            ),
            wasm_file: format!(
                "{}/{}/{}_chunk_{}_submachine_{}_js/{}_chunk_{}_submachine_{}.wasm",
                self.basedir,
                self.task_path,
                name,
                self.chunk_id,
                submachine_id,
                name,
                self.chunk_id,
                submachine_id
            ),
            //zkin: format!("{}/{}_chunk_{}_submachine_{}/{}_proof.bin",self.program_output, name, self.chunk_id, submachine_id, name),
        }
    }
}
