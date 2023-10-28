/// The work space is organized as below.
///`
/// basedir/
///       _/executor/{task_id}/status
///                          _/fib.pil.json
///                          _/fib.cm
///                          _/fib.const
///       _/proof/{task_id}/status
///                       _/batch_proof/
///                                   _/fib.circom
///                                   _/fib.zkin.json
///                                   _/fib.r1cs
///                                   _/fib_js/fib.wasm
///                                   _/fib.pil
///                                   _/fib.pil.json
///                                   _/fib.exec
///                                   _/fib.cm
///                                   _/fib.const
///                       _/agg_proof/
///                                 _/fib.recursive1.circom
///                                 _/fib.recursive1.zkin.json
///                                 _/fib.recursive1.r1cs
///                                 _/fib.recursive1_js/fib.recursive1.wasm
///                                 _/fib.recursive1.cm
///                                 _/fib.recursive1.const
///                                 _/fib.recursive1.exec
///                                 _/fib.recursive1.pil
///                                 _/fib.recursive1.pil.json
///
///                       _/snark_proof/
///                                 _/fib.recursive2.circom
///                                 _/fib.recursive2.zkin.json
///                                 _/fib.recursive2.r1cs
///                                 _/fib.recursive2_js/fib.recursive2.wasm
///                                 _/fib.recursive2.cm
///                                 _/fib.recursive2.const
///                                 _/fib.recursive2.exec
///                                 _/fib.recursive2.pil
///                                 _/fib.recursive2.pil.json
///                                 _/g16.zkey
///                                 _/verification_key.json
///                                 _/proof.json
///                                 _/public_input.json
///`
mod agg_prove;
mod batch_prove;
mod final_prove;
mod traits;

//use async_channel::{bounded, Receiver, Sender};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
//use std::fs::File;
//use std::io::Write;
use std::sync::Mutex;
//use std::thread;
use crate::agg_prove::AggProver;
use crate::batch_prove::BatchProver;
use crate::final_prove::FinalProver;
use crate::traits::StageProver;
use algebraic::errors::{EigenError, Result};

fn load_link(curve_type: &str) -> Vec<String> {
    let mut links: Vec<String> = vec![];
    match curve_type {
        "GL" => {
            if let Ok(pilstark) = std::env::var("STARK_VERIFIER_GL") {
                links.push(pilstark);
            }
        }
        "BN128" => {
            if let Ok(pilstark) = std::env::var("STARK_VERIFIER_BN128") {
                links.push(pilstark);
            }
        }
        "BLS12381" => {
            if let Ok(pilstark) = std::env::var("STARK_VERIFIER_BLS12381") {
                links.push(pilstark);
            }
        }
        &_ => todo!(),
    }
    if let Ok(circomlib) = std::env::var("CIRCOMLIB") {
        links.push(circomlib);
    }
    links
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProveStage {
    BatchProve(String),
    AggProve(String, String, String, String),
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
    c12_circom: CircomCompileArgs,

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
            c12_circom: CircomCompileArgs::new(&basedir, &task_path, &task_name, "GL"),
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
        curve: String,
        input: String,
        input2: String,
    ) -> Self {
        let task_path = ProveStage::AggProve(
            task_id.clone(),
            curve.clone(),
            input.clone(),
            input2.clone(),
        )
        .path();
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
            Self::BatchProve(task_id) => format!("proof/{task_id}/agg_proof"),
            Self::AggProve(task_id, _, _, _) => format!("proof/{task_id}/batch_proof"),
            Self::FinalProve(task_id, _, _) => format!("proof/{task_id}/snark_proof"),
        };
        stage.to_string()
    }

    /// keep track of task status
    pub fn checkpoint(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }
}

pub struct Pipeline {
    basedir: String,
    task_name: String,
    queue: Mutex<VecDeque<ProveStage>>,
}

const INIT_QUEUE_SIZE: usize = 32;

impl Pipeline {
    pub fn new(basedir: String, task_name: String) -> Self {
        // TODO: recover tasks from basedir
        Pipeline {
            basedir,
            task_name,
            queue: Mutex::new(VecDeque::with_capacity(INIT_QUEUE_SIZE)),
        }
    }

    /// Add a new task into task queue
    pub fn batch_prove(&mut self, task_id: String) -> Result<()> {
        match self.queue.get_mut() {
            Ok(w) => {
                w.push_back(ProveStage::BatchProve(task_id));
                Ok(())
            }
            _ => Err(EigenError::Unknown("Task queue is full".to_string())),
        }
    }

    /// Add a new task into task queue
    pub fn aggregate_prove(
        &mut self,
        task_id: String,
        curve_name: String,
        input: String,
        input2: String,
    ) -> Result<()> {
        match self.queue.get_mut() {
            Ok(w) => {
                w.push_back(ProveStage::AggProve(task_id, curve_name, input, input2));
                Ok(())
            }
            _ => Err(EigenError::Unknown("Task queue is full".to_string())),
        }
    }

    /// Add a new task into task queue
    pub fn snark_prove(
        &mut self,
        task_id: String,
        curve_name: String,
        prover_addr: String,
    ) -> Result<()> {
        match self.queue.get_mut() {
            Ok(w) => {
                w.push_back(ProveStage::FinalProve(task_id, curve_name, prover_addr));
                Ok(())
            }
            _ => Err(EigenError::Unknown("Task queue is full".to_string())),
        }
    }

    pub fn prove(&self) -> Result<()> {
        let mut inner = self.queue.lock().unwrap();
        loop {
            match inner.pop_front() {
                Some(v) => match v {
                    ProveStage::BatchProve(task_id) => {
                        let ctx = BatchContext::new(
                            self.basedir.clone(),
                            task_id.clone(),
                            self.task_name.clone(),
                        );
                        BatchProver::new().batch_prove(&ctx)?;
                    }
                    ProveStage::AggProve(task_id, curve, input, input2) => {
                        let ctx = AggContext::new(
                            self.basedir.clone(),
                            task_id.clone(),
                            self.task_name.clone(),
                            curve,
                            input,
                            input2,
                        );
                        AggProver::new().agg_prove(&ctx)?;
                    }
                    ProveStage::FinalProve(task_id, curve_name, prover_addr) => {
                        let ctx = FinalContext::new(
                            self.basedir.clone(),
                            task_id.clone(),
                            self.task_name.clone(),
                            curve_name,
                            prover_addr,
                        );
                        FinalProver::new().final_prove(&ctx)?;
                    }
                },
                _ => {
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                }
            }
        }
    }
}
