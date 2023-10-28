//! The work space is organized as below.
//!```
//! basedir/current
//! basedir/{task_id}/status
//!                 _/batch_proof/
//!                 _/c12_proof/
//!                 _/agg_proof/
//!                 _/final_stark_proof/
//!                 _/snark_proof/
//!                 _/recursive1_js/
//!                 _/recursive2_js/
//!```
//!
mod aggregate_prove;
mod batch_prove;
mod c12_prove;
mod final_stark_prove;
mod snark_prove;
mod traits;

use crate::traits::Executor;
//use async_channel::{bounded, Receiver, Sender};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
//use std::fs::File;
//use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
//use std::thread;

use algebraic::errors::{EigenError, Result};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProveStage {
    AggProve(String),
    BatchProve(String),
    C12Prove(String),
    FinalStarkProve(String),
    SnarkProve(String),
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CircomCompileArgs {
    circom_file: String,
    wasm_file: String,
    link_directories: Vec<String>, // setup the library path
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct StarkProveArgs {
    r1cs_file: String,
    pil_file: String,
    piljson: String,
    const_file: String,
    exec_file: String,
    commit_file: String,
    prover_addr: String,

    zkin: String,
    zkin2: String, // for aggregation
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SnarkProveArgs {
    curve_type: String,
    circuit_file: String,
    wasm_file: String,
    pk_file: String,
    vk_file: String,

    input_file: String,
    public_input_file: String,
    proof_file: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Context {
    basedir: String,
    task_id: String,

    batch_stark: StarkProveArgs,
    c12_stark: StarkProveArgs,
    agg_stark: StarkProveArgs,
    final_stark: StarkProveArgs,

    batch_circom: CircomCompileArgs,
    c12_circom: CircomCompileArgs,
    agg_circom: CircomCompileArgs,
    final_circom: CircomCompileArgs,

    c12_struct: String,
    batch_struct: String,
    agg_struct: String,
    final_stark_struct: String,

    final_snark: SnarkProveArgs,
}

impl Context {
    pub fn new(basedir: String, task_id: String) -> Self {
        // let workdir = format!("{}/{}", basedir, task_id);
        Context {
            basedir,
            task_id,
            batch_struct: "data/batch.stark_struct.json".to_string(),
            c12_struct: "data/c12.stark_struct.json".to_string(),
            agg_struct: "data/agg.stark_struct.json".to_string(),
            final_stark_struct: "data/final.stark_struct.json".to_string(),

            batch_stark: StarkProveArgs {
                r1cs_file: "".to_string(),
                pil_file: "".to_string(),
                piljson: "".to_string(),
                const_file: "".to_string(),
                exec_file: "".to_string(),
                commit_file: "".to_string(),
                prover_addr: "".to_string(),
                zkin: "".to_string(),
                zkin2: "".to_string(), // for aggregation
            },
            c12_stark: StarkProveArgs {
                r1cs_file: "".to_string(),
                pil_file: "".to_string(),
                piljson: "".to_string(),
                const_file: "".to_string(),
                exec_file: "".to_string(),
                commit_file: "".to_string(),
                prover_addr: "".to_string(),
                zkin: "".to_string(),
                zkin2: "".to_string(), // for aggregation
            },
            agg_stark: StarkProveArgs {
                r1cs_file: "".to_string(),
                pil_file: "".to_string(),
                piljson: "".to_string(),
                const_file: "".to_string(),
                exec_file: "".to_string(),
                commit_file: "".to_string(),
                prover_addr: "".to_string(),
                zkin: "".to_string(),
                zkin2: "".to_string(), // for aggregation
            },
            final_stark: StarkProveArgs {
                r1cs_file: "".to_string(),
                pil_file: "".to_string(),
                piljson: "".to_string(),
                const_file: "".to_string(),
                exec_file: "".to_string(),
                commit_file: "".to_string(),
                prover_addr: "".to_string(),
                zkin: "".to_string(),
                zkin2: "".to_string(), // for aggregation
            },
            ..Default::default()
        }
    }
}

impl ProveStage {
    pub fn execute(&self, ctx: &Context) -> Result<()> {
        match self {
            Self::AggProve(_task_id) => aggregate_prove::AggProver::new().execute(ctx),
            Self::BatchProve(_task_id) => batch_prove::BatchProver::new().execute(ctx),
            Self::C12Prove(_task_id) => c12_prove::C12Prover::new().execute(ctx),
            Self::FinalStarkProve(_task_id) => {
                final_stark_prove::FinalStarkProver::new().execute(ctx)
            }
            Self::SnarkProve(_task_id) => snark_prove::SnarkProver::new().execute(ctx),
        }
    }

    fn path(&self) -> String {
        let stage = match self {
            Self::AggProve(_task_id) => "agg_proof",
            Self::BatchProve(_task_id) => "batch_proof",
            Self::C12Prove(_task_id) => "c12_proof",
            Self::FinalStarkProve(_task_id) => "final_stark_proof",
            Self::SnarkProve(_task_id) => "snark_proof",
        };
        stage.to_string()
    }

    /// keep track of task status
    pub fn checkpoint(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn next_stage(&self, basedir: &str) -> Result<()> {
        let state = match self {
            Self::AggProve(task_id) => ProveStage::FinalStarkProve(task_id.clone()),
            Self::BatchProve(task_id) => ProveStage::C12Prove(task_id.clone()),
            Self::C12Prove(task_id) => ProveStage::AggProve(task_id.clone()),
            Self::FinalStarkProve(task_id) => ProveStage::SnarkProve(task_id.clone()),
            _ => panic!("Task done already"),
        };
        match state.checkpoint() {
            Ok(task) => {
                let p = Path::new(basedir).join(self.path());
                std::fs::write(p, task.as_str())?;
                Ok(())
            }
            Err(e) => {
                log::error!("next_stage: {:?}", e);
                Err(e)
            }
        }
    }
}

pub struct Pipeline {
    basedir: String,
    queue: Mutex<VecDeque<ProveStage>>,
}

const INIT_QUEUE_SIZE: usize = 32;

impl Pipeline {
    pub fn new(basedir: String) -> Self {
        // TODO: recover tasks from basedir
        Pipeline {
            basedir,
            queue: Mutex::new(VecDeque::with_capacity(INIT_QUEUE_SIZE)),
        }
    }

    /// Add a new task into task queue
    pub fn push(&mut self, task_id: String) -> Result<()> {
        match self.queue.get_mut() {
            Ok(w) => {
                w.push_back(ProveStage::BatchProve(task_id));
                Ok(())
            }
            _ => Err(EigenError::Unknown("Task queue is full".to_string())),
        }
    }

    pub fn prove(&self) -> Result<()> {
        let mut inner = self.queue.lock().unwrap();
        let ctx = Context {
            ..Default::default()
        };
        loop {
            match inner.pop_front() {
                Some(v) => {
                    v.execute(&ctx)?;
                    v.next_stage(&self.basedir)?;
                }
                _ => {
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                }
            }
        }
    }
}
