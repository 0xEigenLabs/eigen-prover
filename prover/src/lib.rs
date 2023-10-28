mod aggregate_prove;
mod batch_prove;
mod c12_prove;
mod final_stark_prove;
mod snark_prove;
mod traits;

use async_channel::{bounded, Receiver, Sender};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Condvar, Mutex,
};
use std::thread;
use crate::traits::Executor;

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

///
/// basedir/{task_id}/status
///                 _/batch_proof/
///                 _/c12_proof/
///                 _/agg_proof/
///                 _/final_stark_proof/
///                 _/snark_proof/
///
impl ProveStage {
    pub fn execute(&self, basedir: &str) -> Result<()> {
        match self {
            Self::AggProve(task_id) => aggregate_prove::AggProver::new().execute(basedir, task_id),
            Self::BatchProve(task_id) => batch_prove::BatchProver::new().execute(basedir, task_id),
            Self::C12Prove(task_id) => c12_prove::C12Prover::new().execute(basedir, task_id),
            Self::FinalStarkProve(task_id) => {
                final_stark_prove::FinalStarkProver::new().execute(basedir, task_id)
            }
            Self::SnarkProve(task_id) => snark_prove::SnarkProver::new().execute(basedir, task_id),
        }
    }

    fn path(&self) -> String {
        let stage = match self {
            Self::AggProve(task_id) => "agg_proof",
            Self::BatchProve(task_id) => "batch_proof",
            Self::C12Prove(task_id) => "c12_proof",
            Self::FinalStarkProve(task_id) => "final_stark_proof",
            Self::SnarkProve(task_id) => "snark_proof",
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
            Ok(w) => Ok(w.push_back(ProveStage::BatchProve(task_id))),
            _ => Err(EigenError::Unknown("Task queue is full".to_string())),
        }
    }

    fn prove(&self) -> Result<()> {
        let mut inner = self.queue.lock().unwrap();
        loop {
            match inner.pop_front() {
                Some(v) => {
                    v.execute(&self.basedir)?;
                    v.next_stage(&self.basedir);
                }
                _ => {
                    std::thread::sleep_ms(1000);
                }
            }
        }
        Ok(())
    }
}
