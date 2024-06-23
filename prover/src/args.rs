use serde::{Deserialize, Serialize};
use std::env::var;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CircomCompileArgs {
    /// circom file
    pub circom_file: String,
    /// setup the library path
    pub link_directories: Vec<String>,
    /// output directory
    pub output: String,
}

impl CircomCompileArgs {
    pub fn new(basedir: &str, task_path: &str, task_name: &str, curve: &str) -> Self {
        CircomCompileArgs {
            circom_file: format!("{basedir}/{task_path}/{task_name}.circom",),
            link_directories: load_link(curve),
            output: format!("{basedir}/{task_path}"),
        }
    }

    pub fn new_batch(
        evm_output: &str,
        basedir: &str,
        task_path: &str,
        task_name: &str,
        chunk_id: &str,
        curve: &str,
    ) -> Self {
        CircomCompileArgs {
            circom_file: format!("{evm_output}/{task_name}_chunk_{chunk_id}.circom",),
            link_directories: load_link(curve),
            output: format!("{basedir}/{task_path}"),
        }
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
    pub zkin: String,
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
            zkin: format!("{basedir}/{task_path}/{task_name}.zkin.json",),
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
