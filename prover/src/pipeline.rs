//use crate::eigen_prover::{AggProver, BatchProver, FinalProver};
use crate::sp1_prover::final_prover::Sp1FinalProver;
use prover_core::contexts::{AggContext, BatchContext, FinalContext, ProveDataCache};
use prover_core::prover::Prover;
use prover_core::stage::Stage;

use crate::sp1_prover::agg_prover::Sp1AggProver;
use crate::sp1_prover::batch_prover::Sp1BatchProver;

use anyhow::{anyhow, bail, Result};
use std::collections::{HashMap, VecDeque};
use std::env;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Sender;
//use uuid::Uuid;

/// Each task handled by one pipeline
pub struct Pipeline {
    basedir: String,
    queue: VecDeque<String>, // task_id
    task_map: Mutex<HashMap<String, Stage>>,
    task_name: String,
    /// Cache the reusable data of rec2 during the final stage.
    /// include: R1CS, pil, exec, wasm, const
    prove_data_cache: Arc<Mutex<ProveDataCache>>,
    task_sender: Option<Sender<BatchContext>>,
    prover_model: ProverModel,
    prover_type: ProverType,

    elf_path: String,
    aggregation_elf_path: String,
}

#[derive(Debug)]
pub enum ProverModel {
    Local,
    GRPC,
}

impl From<String> for ProverModel {
    fn from(value: String) -> Self {
        match value.as_str() {
            "local" => ProverModel::Local,
            "grpc" => ProverModel::GRPC,
            // invalid env value, use default local model
            _ => {
                log::error!("invalid prover model: {}, please set the env PROVER_MODEL to local or grpc, use default local model", value);
                ProverModel::Local
            }
        }
    }
}

#[derive(Debug)]
pub enum ProverType {
    Eigen,
    SP1,
}

impl From<String> for ProverType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "eigen" => ProverType::Eigen,
            "sp1" => ProverType::SP1,
            // invalid env value, use default local type
            _ => {
                log::error!("invalid prover type: {}, please set the env PROVER_TYPE to local or sp1, use default local model", value);
                ProverType::Eigen
            }
        }
    }
}

impl Pipeline {
    pub fn new(basedir: String, task_name: String) -> Self {
        // TODO move those codes out of Pipeline::new.
        let default_cache_dir = env::var("CACHE_DIR").unwrap_or(String::from(""));
        let prover_model: ProverModel =
            env::var("PROVER_MODEL").unwrap_or("local".to_string()).into();
        log::info!("start pipeline with prover model: {:?}", prover_model);
        let prover_type: ProverType = env::var("PROVER_TYPE").unwrap_or("eigen".to_string()).into();
        log::info!("start pipeline with prover type: {:?}", prover_type);

        let force_bits = std::env::var("FORCE_BIT").unwrap_or("0".to_string());
        let force_bits = force_bits
            .parse::<usize>()
            .unwrap_or_else(|_| panic!("Can not parse {} to usize", force_bits));
        log::info!("pipeline: compress setup force_bits {force_bits}");

        let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let elf_path = env::var("ELF_PATH").unwrap_or(format!(
            "{}/../target/elf-compilation/riscv32im-succinct-zkvm-elf/release/evm",
            manifest_dir.display()
        ));
        let aggregation_elf_path = env::var("ELF_PATH").unwrap_or(format!(
            "{}/../target/elf-compilation/riscv32im-succinct-zkvm-elf/release/aggregation",
            manifest_dir.display()
        ));

        Pipeline {
            basedir: basedir.clone(),
            queue: VecDeque::new(),
            task_map: Mutex::new(HashMap::new()),
            task_name: task_name.clone(),
            prove_data_cache: Arc::new(Mutex::new(ProveDataCache::new(
                task_name,
                basedir,
                default_cache_dir,
            ))),
            task_sender: None,
            prover_model,
            prover_type,
            elf_path,
            aggregation_elf_path,
        }
    }

    pub fn set_task_sender(&mut self, task_sender: Sender<BatchContext>) {
        self.task_sender = Some(task_sender);
    }

    pub fn get_key(&self, task_id: &str, phase: &str) -> String {
        format!("{}_{}", task_id, phase)
    }

    fn save_checkpoint(&self, key: &String, finished: bool) -> Result<String> {
        let binding = self.task_map.lock().unwrap();
        let task = binding.get(key);
        log::debug!("task: {:?}", task);
        if let Some(status) = task {
            // mkdir
            let workdir = Path::new(&self.basedir).join(status.path());
            log::info!("save_checkpoint, mkdir: {:?}", workdir);
            std::fs::create_dir_all(workdir.clone())?;

            if !finished {
                let p = workdir.join("status");
                std::fs::write(p, status.to_string()?)?;
            }

            let p = workdir.join("status.finished");
            std::fs::write(p, if finished { "1" } else { "0" })?;
        }
        Ok(key.clone())
    }

    fn load_checkpoint(&self, key: &String) -> Result<bool> {
        let binding = self.task_map.lock().unwrap();
        let task = binding.get(key);

        if let Some(stage) = task {
            // mkdir
            let workdir = Path::new(&self.basedir).join(stage.path()).join("status.finished");
            log::info!("load_checkpoint, check file: {:?}", workdir);

            let status = match std::fs::read_to_string(workdir)?.trim() {
                "1" => true,
                "0" => false,
                _ => return Err(anyhow!("Invalid value. Expected '0' or '1'.")),
            };

            Ok(status)
        } else {
            Ok(false)
        }
    }

    pub fn load_final_proof_and_input(&self, key: &str) -> Result<(String, String)> {
        let binding = self.task_map.lock().unwrap();
        let task = binding.get(key);

        if let Some(stage) = task {
            // mkdir
            let workdir: std::path::PathBuf = Path::new(&self.basedir).join(stage.path());
            log::info!("load_final_proof_and_input, workdir: {:?}", workdir);

            let proof_path: std::path::PathBuf = match self.prover_type {
                ProverType::Eigen => workdir.clone().join("../proof.json"),
                ProverType::SP1 => workdir.clone().join("../proof_bls12381.json"),
            };

            let proof = std::fs::read_to_string(proof_path.clone()).map_err(|e| {
                anyhow!("Failed to load the proof.json: {:?}, err: {}", proof_path, e)
            })?;

            let input_path: std::path::PathBuf = match self.prover_type {
                ProverType::Eigen => workdir.clone().join("../public_input.json"),
                ProverType::SP1 => workdir.clone().join("../public_inputs_bls12381.json"),
            };
            let input = std::fs::read_to_string(input_path.clone()).map_err(|e| {
                anyhow!("Failed to load the public_input.json: {:?}, err: {}", input_path, e)
            })?;

            Ok((proof, input))
        } else {
            Err(anyhow!("can not find task: {}", key))
        }
    }

    pub fn batch_prove(&mut self, task_id: String, l2_batch_data: String) -> Result<String> {
        let key = task_id.clone();
        match self.task_map.get_mut() {
            Ok(w) => {
                self.queue.push_back(key.clone());
                w.insert(key.clone(), Stage::Batch(task_id.clone(), l2_batch_data));
                self.save_checkpoint(&key, false)
            }
            _ => bail!("Task queue is full".to_string()),
        }
    }

    /// Add a new task into task queue
    pub fn aggregate_prove(&mut self, task: String, task2: String) -> Result<String> {
        let key = self.get_key(&task, &task2);
        match self.task_map.get_mut() {
            Ok(w) => {
                self.queue.push_back(key.clone());
                w.insert(key.clone(), Stage::Aggregate(key.clone(), task, task2));
                self.save_checkpoint(&key, false)?;
                Ok(key)
            }
            _ => bail!("Task queue is full".to_string()),
        }
    }

    /// Add a new task into task queue
    pub fn final_prove(&mut self, agg_task_id: String, prover_addr: String) -> Result<String> {
        // key := ${agg task id}_final
        let key = self.get_key(&agg_task_id, "final");
        match self.task_map.get_mut() {
            Ok(w) => {
                self.queue.push_back(key.clone());
                w.insert(
                    key.clone(),
                    Stage::Final(agg_task_id.clone(), prover_addr), // use task_id first, then compute the right task_name in final context
                );
                self.save_checkpoint(&key, false)?;
                Ok(agg_task_id)
            }
            _ => bail!("Task queue is full".to_string()),
        }
    }

    pub fn cancel(&mut self, task_id: String) -> Result<()> {
        // TODO find all the tasks with prefix `task_id`
        if let Ok(w) = self.task_map.get_mut() {
            let _ = w.remove(&task_id);
        }
        Ok(())
    }

    /// Return prover status
    pub fn get_status(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn get_proof(&mut self, key: String, _timeout: u64) -> Result<String> {
        match self.load_checkpoint(&key) {
            Ok(true) => Ok(key),
            Ok(false) => bail!("can not find task: {}", key),
            Err(e) => bail!("load checkpoint failed, {:#?}", e),
        }
    }

    pub fn prove(&mut self) -> Result<()> {
        if let Some(key) = self.queue.pop_front() {
            match self.task_map.get_mut().unwrap().get(&key) {
                Some(v) => match v {
                    Stage::Batch(task_id, l2_batch_data) => {
                        let ctx = BatchContext::new(
                            &self.basedir,
                            task_id,
                            &self.task_name.clone(),
                            l2_batch_data.clone(),
                            &self.elf_path,
                        );

                        match self.prover_model {
                            ProverModel::Local => {
                                match self.prover_type {
                                    ProverType::Eigen => {
                                        //BatchProver::new().prove(&ctx)?;
                                        todo!();
                                    }
                                    ProverType::SP1 => {
                                        Sp1BatchProver::new().prove(&ctx)?;
                                    }
                                }

                                self.save_checkpoint(&key, true)?;
                            }
                            ProverModel::GRPC => {
                                // send the task's ctx to scheduler
                                log::info!(
                                    "send task to scheduler: [id:{}], [name:{}]",
                                    ctx.task_id,
                                    ctx.task_name
                                );
                                if let Err(e) = self.task_sender.as_ref().unwrap().try_send(ctx) {
                                    log::error!("send task to scheduler failed, {:?}", e);
                                }
                            }
                        }
                    }
                    Stage::Aggregate(task_id, input, input2) => {
                        let ctx = AggContext::new(
                            &self.basedir,
                            task_id,
                            &self.task_name,
                            input.clone(),
                            input2.clone(),
                            self.prove_data_cache.clone(),
                            &self.elf_path,
                            &self.aggregation_elf_path,
                        );
                        match self.prover_type {
                            ProverType::Eigen => {
                                //AggProver::new().prove(&ctx)?;
                                todo!();
                            }
                            ProverType::SP1 => {
                                Sp1AggProver::new().prove(&ctx)?;
                            }
                        }

                        self.save_checkpoint(&key, true)?;
                    }
                    Stage::Final(agg_task_id, prover_addr) => {
                        let ctx = FinalContext::new(
                            self.basedir.clone(),
                            agg_task_id.clone(),
                            self.task_name.clone(),
                            prover_addr.clone(),
                            self.prove_data_cache.clone(),
                        );
                        match self.prover_type {
                            ProverType::Eigen => {
                                //FinalProver::default().prove(&ctx)?;
                                todo!();
                            }
                            ProverType::SP1 => {
                                Sp1FinalProver::default().prove(&ctx)?;
                            }
                        }
                        self.save_checkpoint(&key, true)?;
                    }
                },
                _ => {
                    log::info!("Task queue is empty...");
                }
            };
        }
        Ok(())
    }
}
