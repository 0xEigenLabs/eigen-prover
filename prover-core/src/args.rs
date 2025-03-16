use serde::{Deserialize, Serialize};
use std::env::var;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CircomCompileArgs {
    pub program_output: String,
    pub task_name: String,
    pub chunk_id: String,
    pub submachine_id: usize,
    pub base_dir: String,
    pub task_path: String,
    /// setup the library path
    pub link_directories: Vec<String>,
}

impl CircomCompileArgs {
    pub fn new(
        evm_output: &str,
        basedir: &str,
        task_name: &str,
        task_path: &str,
        chunk_id: &str,
        submachine_id: usize,
        curve: &str,
    ) -> Self {
        CircomCompileArgs {
            program_output: evm_output.to_string(),
            task_name: task_name.to_string(),
            chunk_id: chunk_id.to_string(),
            link_directories: load_link(curve),
            base_dir: basedir.to_string(),
            task_path: task_path.to_string(),
            submachine_id,
        }
    }

    pub fn circom(&self, use_base: bool) -> String {
        match use_base {
            false => format!(
                "{}/{}_chunk_{}_submachine_{}.circom",
                self.program_output, self.task_name, self.chunk_id, self.submachine_id
            ),
            true => format!(
                "{}/{}_chunk_{}_submachine_{}.circom",
                self.base_dir, self.task_name, self.chunk_id, self.submachine_id
            ),
        }
    }

    pub fn zkin(&self, use_base: bool) -> String {
        match use_base {
            false => format!(
                "{}/{}_chunk_{}_submachine_{}.json",
                self.program_output, self.task_name, self.chunk_id, self.submachine_id
            ),
            true => format!(
                "{}/{}_chunk_{}_submachine_{}.json",
                self.base_dir, self.task_name, self.chunk_id, self.submachine_id
            ),
        }
    }

    pub fn circom_output(&self) -> String {
        let path = format!("{}/{}", self.base_dir, self.task_path);
        std::fs::create_dir_all(&path).unwrap();
        path
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct StarkProveArgs {
    pub commit_file: String,
    pub const_file: String,
    pub curve_type: String,
    pub exec_file: String,
    pub pil_file: String,
    pub piljson: String,
    pub r1cs_file: String,
    pub wasm_file: String,
    //pub zkin: String,
}

impl StarkProveArgs {
    pub fn new(basedir: &str, task_path: &str, task_name: &str, curve: &str) -> Self {
        StarkProveArgs {
            commit_file: format!("{basedir}/{task_path}/{task_name}.cm",),
            const_file: format!("{basedir}/{task_path}/{task_name}.const",),
            curve_type: curve.to_string(),
            exec_file: format!("{basedir}/{task_path}/{task_name}.exec",),
            pil_file: format!("{basedir}/{task_path}/{task_name}.pil",),
            piljson: format!("{basedir}/{task_path}/{task_name}.pil.json",),
            r1cs_file: format!("{basedir}/{task_path}/{task_name}.r1cs",),
            wasm_file: format!("{basedir}/{task_path}/{task_name}_js/{task_name}.wasm",),
            //zkin: format!("{basedir}/{task_path}/{task_name}.zkin.json",),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FinalProveArgs {
    pub curve_type: String,
    pub pk_file: String,
    pub proof_file: String,
    pub public_input_file: String,
    pub vk_file: String,
}

pub fn load_link(curve_type: &str) -> Vec<String> {
    let mut links: Vec<String> = vec![];
    match curve_type {
        "GL" => {
            if let Ok(pilstark) = var("STARK_VERIFIER_GL") {
                links.push(pilstark);
            }
        }
        "BN128" => {
            if let Ok(pilstark) = var("STARK_VERIFIER_BN128") {
                links.push(pilstark);
            }
        }
        "BLS12381" => {
            if let Ok(pilstark) = var("STARK_VERIFIER_BLS12381") {
                links.push(pilstark);
            }
        }
        &_ => todo!(),
    }
    if let Ok(circomlib) = var("CIRCOMLIB") {
        links.push(circomlib);
    }
    links
}
