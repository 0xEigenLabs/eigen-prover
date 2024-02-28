use crate::contexts::{AggContext, BatchContext, FinalContext};
use crate::provers::{AggProver, BatchProver, FinalProver, Prover};
use crate::stage::Stage;

use anyhow::{anyhow, bail, Result};
use std::collections::{HashMap, VecDeque};
use std::path::Path;
use std::sync::Mutex;
use uuid::Uuid;

pub struct Pipeline {
    basedir: String,
    queue: VecDeque<String>, // task_id
    task_map: Mutex<HashMap<String, Stage>>,
    task_name: String,
}

impl Pipeline {
    pub fn new(basedir: String, task_name: String) -> Self {
        // TODO: recover tasks from basedir
        Pipeline {
            basedir,
            queue: VecDeque::new(),
            task_map: Mutex::new(HashMap::new()),
            task_name,
        }
    }

    fn save_checkpoint(&self, task_id: String, finished: bool) -> Result<String> {
        let binding = self.task_map.lock().unwrap();
        let task = binding.get(&task_id);

        if let Some(status) = task {
            // mkdir
            let workdir = Path::new(&self.basedir).join(status.path());
            log::info!("save_checkpoint, mkdir: {:?}", workdir);
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
            .join(task_id)
            .join("status.finished");
        let status: bool = std::fs::read_to_string(p)?.parse().map_err(|e| {
            log::error!("load_checkpoint");
            anyhow!("load checkpoint failed, {:?}", e)
        })?;
        Ok(status)
    }

    pub fn batch_prove(&mut self, task_id: String, chunk_id: String) -> Result<String> {
        match self.task_map.get_mut() {
            Ok(w) => {
                self.queue.push_back(task_id.clone());
                w.insert(task_id.clone(), Stage::Batch(task_id.clone(), chunk_id));
                self.save_checkpoint(task_id, false)
            }
            _ => bail!("Task queue is full".to_string()),
        }
    }

    /// Add a new task into task queue
    pub fn aggregate_prove(&mut self, task: String, task2: String) -> Result<String> {
        let task_id = Uuid::new_v4().to_string();
        match self.task_map.get_mut() {
            Ok(w) => {
                self.queue.push_back(task_id.clone());
                w.insert(
                    task_id.clone(),
                    Stage::Aggregate(task_id.clone(), task, task2),
                );
                self.save_checkpoint(task_id, false)
            }
            _ => bail!("Task queue is full".to_string()),
        }
    }

    /// Add a new task into task queue
    pub fn final_prove(
        &mut self,
        task_id: String,
        curve_name: String,
        prover_addr: String,
    ) -> Result<String> {
        match self.task_map.get_mut() {
            Ok(w) => {
                self.queue.push_back(task_id.clone());
                w.insert(
                    task_id.clone(),
                    Stage::Final(task_id.clone(), curve_name, prover_addr),
                );
                self.save_checkpoint(task_id, false)
            }
            _ => bail!("Task queue is full".to_string()),
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
    pub fn get_proof(&mut self, task_id: String, _timeout: u64) -> Result<String> {
        match self.load_checkpoint(task_id) {
            Ok(true) => Ok("".to_string()),
            _ => bail!("get_proof failed".to_string()),
        }
    }

    pub fn prove(&mut self) -> Result<()> {
        if let Some(task_id) = self.queue.pop_front() {
            match self.task_map.get_mut().unwrap().get(&task_id) {
                Some(v) => match v {
                    Stage::Batch(task_id, chunk_id) => {
                        let ctx = BatchContext::new(
                            &self.basedir,
                            task_id,
                            &self.task_name.clone(),
                            chunk_id,
                        );
                        BatchProver::new().prove(&ctx)?;
                    }
                    Stage::Aggregate(task_id, input, input2) => {
                        let ctx = AggContext::new(
                            &self.basedir,
                            task_id,
                            &self.task_name,
                            input.clone(),
                            input2.clone(),
                        );
                        AggProver::new().prove(&ctx)?;
                    }
                    Stage::Final(task_id, curve_name, prover_addr) => {
                        let ctx = FinalContext::new(
                            self.basedir.clone(),
                            task_id.clone(),
                            self.task_name.clone(),
                            curve_name.clone(),
                            prover_addr.clone(),
                        );
                        FinalProver::new().prove(&ctx)?;
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
