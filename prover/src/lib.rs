pub(crate) mod agg_prove;
pub(crate) mod batch_prove;
pub(crate) mod final_prove;
mod integration_test;
pub(crate) mod traits;

use crate::agg_prove::AggProver;
use crate::batch_prove::BatchProver;
use crate::final_prove::FinalProver;
use crate::traits::StageProver;
use algebraic::errors::{EigenError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env::var;
use std::path::Path;
use std::sync::Mutex;
use uuid::Uuid;

fn load_link(curve_type: &str) -> Vec<String> {
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProveStage {
    BatchProve(String),
    AggProve(String, String, String),
    FinalProve(String, String, String),
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CircomCompileArgs {
    circom_file: String,
    output: String,
    link_directories: Vec<String>, // setup the library path
}

impl CircomCompileArgs {
    pub fn new(basedir: &str, task_path: &str, task_name: &str, curve: &str) -> Self {
        CircomCompileArgs {
            circom_file: format!("{basedir}/{task_path}/{task_name}.circom",),
            output: format!("{basedir}/{task_path}/",),
            link_directories: load_link(curve), // setup the library path
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct StarkProveArgs {
    r1cs_file: String,
    pil_file: String,
    piljson: String,
    const_file: String,
    exec_file: String,
    commit_file: String,
    zkin: String,
    curve_type: String,
}

impl StarkProveArgs {
    pub fn new(basedir: &str, task_path: &str, task_name: &str, curve: &str) -> Self {
        StarkProveArgs {
            r1cs_file: format!("{basedir}/{task_path}/{task_name}.r1cs",),
            pil_file: format!("{basedir}/{task_path}/{task_name}.pil",),
            piljson: format!("{basedir}/{task_path}/{task_name}.pil.json",),
            const_file: format!("{basedir}/{task_path}/{task_name}.const",),
            commit_file: format!("{basedir}/{task_path}/{task_name}.cm",),
            exec_file: format!("{basedir}/{task_path}/{task_name}.exec",),
            zkin: format!("{basedir}/{task_path}/{task_name}.zkin.json",),
            curve_type: curve.to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FinalProveArgs {
    curve_type: String,
    circuit_file: String,
    wasm_file: String,
    input_file: String,

    pk_file: String,
    vk_file: String,
    public_input_file: String,
    proof_file: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct FinalContext {
    basedir: String,
    task_id: String,
    task_name: String,
    prover_addr: String,

    final_stark: StarkProveArgs,
    final_circom: CircomCompileArgs,
    final_stark_struct: String,

    final_snark: FinalProveArgs,
}

impl FinalContext {
    pub fn new(
        basedir: String,
        task_id: String,
        task_name: String,
        curve: String,
        prover_addr: String,
    ) -> Self {
        let prev_task_path =
            ProveStage::FinalProve(task_id.clone(), curve.clone(), prover_addr.clone()).path();
        let task_path =
            ProveStage::FinalProve(task_id.clone(), curve.clone(), prover_addr.clone()).path();
        FinalContext {
            basedir: basedir.clone(),
            task_id,
            prover_addr,
            task_name: task_name.clone(),
            final_stark_struct: "data/final.stark_struct.json".to_string(),
            final_stark: StarkProveArgs::new(&basedir, &task_path, &task_name, &curve),
            final_circom: CircomCompileArgs::new(&basedir, &task_path, &task_name, &curve),
            final_snark: FinalProveArgs{
                curve_type: curve,
                circuit_file: format!("{basedir}/{prev_task_path}/{task_name}.recursive2.r1cs"),
                wasm_file: format!("{basedir}/{prev_task_path}/{task_name}.recursive2_js/{task_name}_recursive2.r1cs"),
                input_file: format!("{basedir}/{prev_task_path}/{task_name}.recursive2.zkin.json"),
                pk_file: format!("{basedir}/{task_path}/g16.key"),
                vk_file: format!("{basedir}/{task_path}/verification_key.json"),
                public_input_file: format!("{basedir}/{task_path}/public_input.json"),
                proof_file: format!("{basedir}/{task_path}/proof.json"),
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BatchContext {
    basedir: String,
    task_id: String,
    task_name: String,

    batch_stark: StarkProveArgs,
    c12_stark: StarkProveArgs,

    batch_circom: CircomCompileArgs,
    // c12_circom: CircomCompileArgs,
    c12_struct: String,
    batch_struct: String,
}

impl BatchContext {
    pub fn new(basedir: String, task_id: String, task_name: String) -> Self {
        let executor_dir = format!("{}/executor/{}", basedir.clone(), task_id.clone());
        let task_path = ProveStage::BatchProve(task_id.clone()).path();
        BatchContext {
            basedir: basedir.clone(),
            task_id: task_id.clone(),
            task_name: task_name.clone(),
            batch_struct: "data/batch.stark_struct.json".to_string(),
            c12_struct: "data/c12.stark_struct.json".to_string(),

            batch_stark: StarkProveArgs {
                r1cs_file: "".to_string(),
                pil_file: "".to_string(),
                piljson: format!("{}/{}.pil.json", executor_dir, task_name.clone()),
                const_file: format!("{}/{}.const", executor_dir, task_name.clone()),
                commit_file: format!("{}/{}.cm", executor_dir, task_name.clone()),
                exec_file: "".to_string(),
                zkin: "".to_string(),
                curve_type: "GL".to_string(),
            },
            batch_circom: CircomCompileArgs::new(&basedir, &task_path, &task_name, "GL"),

            c12_stark: StarkProveArgs::new(&basedir, &task_path, &task_name, "GL"),
            // c12_circom: CircomCompileArgs::new(&basedir, &task_path, &task_name, "GL"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AggContext {
    basedir: String,
    task_name: String,
    input: String,
    input2: String,

    agg_stark: StarkProveArgs,
    agg_circom: CircomCompileArgs,
    agg_struct: String,
}

impl AggContext {
    pub fn new(
        basedir: String,
        task_id: String,
        task_name: String,
        input: String,
        input2: String,
    ) -> Self {
        let task_path = ProveStage::AggProve(task_id.clone(), input.clone(), input2.clone()).path();
        AggContext {
            basedir: basedir.clone(),
            task_name: task_name.clone(),
            input,
            input2,
            agg_struct: "data/agg.stark_struct.json".to_string(),
            agg_stark: StarkProveArgs::new(&basedir, &task_path, &task_name, "GL"),
            agg_circom: CircomCompileArgs::new(&basedir, &task_path, &task_name, "GL"),
        }
    }
}

impl ProveStage {
    fn path(&self) -> String {
        let stage = match self {
            Self::BatchProve(task_id) => format!("proof/{task_id}/batch_proof"),
            Self::AggProve(task_id, _, _) => format!("proof/{task_id}/agg_proof"),
            Self::FinalProve(task_id, _, _) => format!("proof/{task_id}/snark_proof"),
        };
        stage.to_string()
    }

    /// keep track of task status
    pub fn to_string(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

pub struct Pipeline {
    pub basedir: String,
    pub task_name: String,
    queue: VecDeque<String>,
    task_map: Mutex<HashMap<String, ProveStage>>,
}

impl Pipeline {
    pub fn new(basedir: String, task_name: String) -> Self {
        // TODO: recover tasks from basedir
        Pipeline {
            basedir,
            task_name,
            queue: VecDeque::new(),
            task_map: Mutex::new(HashMap::new()),
        }
    }

    fn save_checkpoint(&self, task_id: String, finished: bool) -> Result<String> {
        let binding = self.task_map.lock().unwrap();
        let task = binding.get(&task_id);

        if let Some(status) = task {
            // mkdir
            let workdir = Path::new(&self.basedir).join(status.path());
            let _ = std::fs::create_dir_all(workdir);

            if !finished {
                let p = Path::new(&self.basedir)
                    .join("proof")
                    .join(task_id.clone())
                    .join("status");
                std::fs::write(p, status.to_string()?)?;
            }

            let p = Path::new(&self.basedir)
                .join("proof")
                .join(task_id.clone())
                .join("status.finished");
            std::fs::write(p, if finished { "1" } else { "0" })?;
        }
        Ok(task_id)
    }

    fn load_checkpoint(&self, task_id: String) -> Result<bool> {
        //let p = Path::new(&self.basedir).join(status.path()).join("status");
        //std::fs::read(p, status.to_string()?)?;

        let p = Path::new(&self.basedir)
            .join("proof")
            .join(task_id.clone())
            .join("status.finished");
        let status: bool = std::fs::read_to_string(p)?.parse().map_err(|e| {
            log::error!("load_checkpoint: {:?}", e);
            EigenError::InvalidValue("load checkpoint failed".to_string())
        })?;
        Ok(status)
    }

    /// FIXME: receive input data
    pub fn batch_prove(&mut self, _batch_l2_data: Vec<u8>) -> Result<String> {
        let task_id = Uuid::new_v4().to_string();
        match self.task_map.get_mut() {
            Ok(w) => {
                self.queue.push_back(task_id.clone());
                w.insert(task_id.clone(), ProveStage::BatchProve(task_id.clone()));
                self.save_checkpoint(task_id, false)
            }
            _ => Err(EigenError::Unknown("Task queue is full".to_string())),
        }
    }

    /// Add a new task into task queue
    pub fn aggregate_prove(&mut self, input: String, input2: String) -> Result<String> {
        let task_id = Uuid::new_v4().to_string();
        match self.task_map.get_mut() {
            Ok(w) => {
                self.queue.push_back(task_id.clone());
                w.insert(
                    task_id.clone(),
                    ProveStage::AggProve(task_id.clone(), input, input2),
                );
                self.save_checkpoint(task_id, false)
            }
            _ => Err(EigenError::Unknown("Task queue is full".to_string())),
        }
    }

    /// Add a new task into task queue
    pub fn final_prove(&mut self, curve_name: String, prover_addr: String) -> Result<String> {
        let task_id = Uuid::new_v4().to_string();
        match self.task_map.get_mut() {
            Ok(w) => {
                self.queue.push_back(task_id.clone());
                w.insert(
                    task_id.clone(),
                    ProveStage::FinalProve(task_id.clone(), curve_name, prover_addr),
                );
                self.save_checkpoint(task_id, false)
            }
            _ => Err(EigenError::Unknown("Task queue is full".to_string())),
        }
    }

    pub fn cancel(&mut self, task_id: String) -> Result<()> {
        if let Ok(w) = self.task_map.get_mut() {
            let _ = w.remove(&task_id);
        }
        Ok(())
    }

    /// Return prover status
    pub fn get_status(&mut self) -> Result<()> {
        Ok(())
    }

    /// TODO: Return proof
    pub fn get_proof(&mut self, task_id: String, _timeout: u64) -> Result<(i32, String)> {
        match self.load_checkpoint(task_id) {
            Ok(true) => Ok((1, "".to_string())),
            _ => Err(EigenError::InvalidValue("get_proof failed".to_string())),
        }
    }

    pub fn prove(&mut self) -> Result<()> {
        if let Some(task_id) = self.queue.pop_front() {
            match self.task_map.get_mut().unwrap().get(&task_id) {
                Some(v) => match v {
                    ProveStage::BatchProve(task_id) => {
                        let ctx = BatchContext::new(
                            self.basedir.clone(),
                            task_id.clone(),
                            self.task_name.clone(),
                        );
                        BatchProver::new().batch_prove(&ctx)?;
                    }
                    ProveStage::AggProve(task_id, input, input2) => {
                        let ctx = AggContext::new(
                            self.basedir.clone(),
                            task_id.clone(),
                            self.task_name.clone(),
                            input.clone(),
                            input2.clone(),
                        );
                        AggProver::new().agg_prove(&ctx)?;
                    }
                    ProveStage::FinalProve(task_id, curve_name, prover_addr) => {
                        let ctx = FinalContext::new(
                            self.basedir.clone(),
                            task_id.clone(),
                            self.task_name.clone(),
                            curve_name.clone(),
                            prover_addr.clone(),
                        );
                        FinalProver::new().final_prove(&ctx)?;
                    }
                },
                _ => {
                    log::info!("Task queue is empty...");
                }
            };
            self.save_checkpoint(task_id, true)?;
        }
        Ok(())
    }
}
