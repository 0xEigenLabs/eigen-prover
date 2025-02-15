use prover_core::stage::Stage;
use std::env::var;
use std::path::Path;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::prover_service::prover_service::gen_batch_proof_request::Step;
use crate::prover_service::prover_service::get_status_response::Status::Idle;
use crate::prover_service::prover_service::prover_request::RequestType;
use crate::prover_service::prover_service::prover_response::ResponseType;
use crate::prover_service::prover_service::{
    gen_batch_proof_response, FinalProof, GenBatchChunks, GenBatchChunksResult, GenChunkProof,
    GenChunkProofResult,
};
use crate::prover_service::prover_service::{
    get_status_response, BatchProofResult, ChunkProof, GenAggregatedProofRequest,
    GenAggregatedProofResponse, GenBatchProofResponse, GenFinalProofRequest, GenFinalProofResponse,
    GetStatusRequest, GetStatusResponse, GetStatusResultCode, ProofResultCode, ProverRequest,
    ProverResponse, ProverStatus,
};
use anyhow::{anyhow, bail, Result};
use ethers_providers::{Http, Middleware, Provider};
use executor::{batch_process, gen_block_json};
use prover::pipeline::{Pipeline, ProverType};
use prover_core::contexts::BatchContext;
use prover_service::prover_service_server::ProverService;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, watch};
use tokio::time;
use tokio_stream::{Stream, StreamExt};
use tonic::{async_trait, Request, Response, Status, Streaming};

#[allow(clippy::module_inception)]
pub mod prover_service {
    tonic::include_proto!("prover.v1"); // The string specified here must match the proto package name
}

const DEFAULT_BATCH_PROOF_POLLING_INTERVAL: Duration = Duration::from_secs(60);
const DEFAULT_BATCH_PROOF_POLLING_TIMEOUT: Duration = Duration::from_secs(60 * 60);

const DEFAULT_AGGREGATED_PROOF_POLLING_INTERVAL: Duration = Duration::from_secs(10);
const DEFAULT_AGGREGATED_PROOF_POLLING_TIMEOUT: Duration = Duration::from_secs(60 * 30);

const DEFAULT_FINAL_PROOF_POLLING_INTERVAL: Duration = Duration::from_secs(10);
const DEFAULT_FINAL_PROOF_POLLING_TIMEOUT: Duration = Duration::from_secs(60 * 30);

lazy_static! {
    static ref BASE_DIR: String = var("BASEDIR").unwrap_or_else(|_| "/tmp/prover/data".to_string());
}

// FIXME: Since each pipeline handles one task, we should create a pipeline set to handle different tasks
lazy_static! {
    static ref PIPELINE: Mutex<Pipeline> = Mutex::new(Pipeline::new(
        var("BASEDIR").unwrap_or("/tmp/prover/data".to_string()),
        var("TASK_NAME").unwrap_or("fibonacci".to_string())
    ));
    static ref PROVER_FORK_ID: u64 = {
        let fork_id = var("PROVER_FORK_ID").unwrap_or("0".into());
        fork_id.parse().unwrap_or(0)
    };
}

pub async fn run_prover(task_sender: Sender<BatchContext>) -> Result<()> {
    PIPELINE.lock().unwrap().set_task_sender(task_sender);
    PIPELINE.lock().unwrap().prove()
}

pub struct ProverServiceSVC {
    handler: Arc<dyn ProverHandler + Send + Sync>,
    eth_client: Arc<Provider<Http>>,
}

impl ProverServiceSVC {
    pub fn new(handler: Arc<dyn ProverHandler + Send + Sync>) -> Self {
        let url = std::env::var("URL").unwrap_or(String::from("http://localhost:8545"));
        let client = Provider::<Http>::try_from(url).unwrap();
        ProverServiceSVC {
            handler,
            eth_client: Arc::new(client),
        }
    }
}

#[tonic::async_trait]
impl ProverService for ProverServiceSVC {
    type ProverStreamStream =
        Pin<Box<dyn Stream<Item = Result<ProverResponse, Status>> + Send + Sync + 'static>>;

    async fn prover_stream(
        &self,
        request: Request<Streaming<ProverRequest>>,
    ) -> Result<Response<Self::ProverStreamStream>, Status> {
        let mut stream = request.into_inner();
        let (tx, rx) = mpsc::channel(10);
        let handler_clone = self.handler.clone();
        let eth_client_clone = self.eth_client.clone();

        tokio::spawn(async move {
            while let Some(req_result) = stream.next().await {
                // let request = req_result.map_err(|e| anyhow!(format!("Error: {:?}", e)))?;
                let request = match req_result {
                    Ok(request) => request,
                    Err(e) => {
                        log::error!("Failed to receive message, close: {}", e);
                        break;
                    }
                };

                let request_id = request.id.clone();
                log::info!(
                    "receive the request from eigen-zeth, request: {:?}",
                    request
                );

                if let Some(req_type) = request.request_type {
                    let resp = match req_type {
                        RequestType::GetStatus(req) => handler_clone
                            .handle_get_status_request(request_id.clone(), req)
                            .await
                            .unwrap_or_else(|e| ProverResponse {
                                id: request_id.clone(),
                                response_type: Some(ResponseType::GetStatus(GetStatusResponse {
                                    id: "".to_string(),
                                    result_code: GetStatusResultCode::Fail as i32,
                                    status: Idle as i32,
                                    prover_status: None,
                                    error_message: e.to_string(),
                                })),
                            }),
                        RequestType::GenBatchProof(req) => {
                            if let Some(batch_req) = req.step {
                                match batch_req {
                                    Step::GenBatchChunks(gen_chunk_req) => {
                                        handler_clone
                                            .handle_gen_batch_chunks_request(
                                                request_id.clone(), gen_chunk_req, eth_client_clone.clone()
                                            ).await.unwrap_or_else(|e| ProverResponse {
                                            id: request_id.clone(),
                                            response_type: Some(ResponseType::GenBatchProof(
                                                GenBatchProofResponse {
                                                    step: Some(gen_batch_proof_response::Step::GenBatchChunks(GenBatchChunksResult{
                                                        result_code: ProofResultCode::CompletedError as i32,
                                                        error_message: e.to_string(),
                                                        ..Default::default()
                                                    })),
                                                }
                                            )),
                                        })
                                    }
                                    Step::GenChunkProof(gen_proof_req) => {
                                        handler_clone.
                                            handle_gen_chunks_proof_request(
                                                request_id.clone(), gen_proof_req
                                            ).await.unwrap_or_else(|e| ProverResponse {
                                            id: request_id.clone(),
                                            response_type: Some(ResponseType::GenBatchProof(
                                                GenBatchProofResponse {
                                                    step: Some(gen_batch_proof_response::Step::GenChunkProof(GenChunkProofResult{
                                                        result_code: ProofResultCode::CompletedError as i32,
                                                        error_message: e.to_string(),
                                                        ..Default::default()
                                                    })),
                                                }
                                            )),
                                        })
                                    }
                                }
                            } else {
                                ProverResponse {
                                    id: request_id.clone(),
                                    // TODO: update pb, return error
                                    response_type: None,
                                }
                            }
                        }
                        RequestType::GenAggregatedProof(r) => handler_clone
                            .handle_gen_aggregated_proof_request(request_id.clone(), r)
                            .await
                            .unwrap_or_else(|e| ProverResponse {
                                id: request_id.clone(),
                                response_type: Some(ResponseType::GenAggregatedProof(
                                    GenAggregatedProofResponse {
                                        result_code: ProofResultCode::CompletedError as i32,
                                        error_message: e.to_string(),
                                        ..Default::default()
                                    },
                                )),
                            }),
                        RequestType::GenFinalProof(r) => handler_clone
                            .handle_gen_final_proof_request(request_id.clone(), r)
                            .await
                            .unwrap_or_else(|e| ProverResponse {
                                id: request_id.clone(),
                                response_type: Some(ResponseType::GenFinalProof(
                                    GenFinalProofResponse {
                                        result_code: ProofResultCode::CompletedError as i32,
                                        final_proof: None,
                                        error_message: e.to_string(),
                                        ..Default::default()
                                    },
                                )),
                            }),
                    };

                    log::info!("send the response to eigen-zeth, response: {:?}", resp);
                    if let Err(e) = tx.send(Ok(resp)).await {
                        log::error!("Failed to send response: {}", e);
                        break;
                    }
                }
            }
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }
}

#[async_trait]
pub trait ProverHandler {
    async fn handle_get_status_request(
        &self,
        msg_id: String,
        _request: GetStatusRequest,
    ) -> Result<ProverResponse>;

    async fn handle_gen_batch_chunks_request(
        &self,
        msg_id: String,
        request: GenBatchChunks,
        client: Arc<Provider<Http>>,
    ) -> Result<ProverResponse>;

    async fn handle_gen_chunks_proof_request(
        &self,
        msg_id: String,
        request: GenChunkProof,
    ) -> Result<ProverResponse>;

    async fn handle_gen_aggregated_proof_request(
        &self,
        msg_id: String,
        request: GenAggregatedProofRequest,
    ) -> Result<ProverResponse>;

    async fn handle_gen_final_proof_request(
        &self,
        msg_id: String,
        request: GenFinalProofRequest,
    ) -> Result<ProverResponse>;
}

#[derive(Default, Clone)]
pub struct ProverRequestHandler {
    executor_base_dir: String,
}

pub struct BatchStateRoot {
    pub prev_state_root: [u8; 32],
    pub post_state_root: [u8; 32],
}

impl ProverRequestHandler {
    pub fn new(executor_base_dir: String) -> Self {
        ProverRequestHandler { executor_base_dir }
    }
}

#[async_trait]
impl ProverHandler for ProverRequestHandler {
    async fn handle_get_status_request(
        &self,
        msg_id: String,
        _request: GetStatusRequest,
    ) -> Result<ProverResponse> {
        let status = match PIPELINE.lock().unwrap().get_status() {
            Ok(_) => Idle,
            _ => get_status_response::Status::Unspecified,
        };

        Ok(ProverResponse {
            id: msg_id,
            response_type: Some(ResponseType::GetStatus(GetStatusResponse {
                id: "".to_string(),
                result_code: GetStatusResultCode::Ok as i32,
                status: status.into(),
                prover_status: Some(ProverStatus {
                    last_computed_request_id: "".to_string(),
                    last_computed_end_time: 0,
                    current_computing_request_id: "".to_string(),
                    current_computing_start_time: 0,
                    version_proto: "".to_string(),
                    version_server: "".to_string(),
                    pending_request_queue_ids: vec![],
                    prover_name: "".to_string(),
                    prover_id: "".to_string(),
                    number_of_cores: 0,
                    total_memory: 0,
                    free_memory: 0,
                    fork_id: 0,
                }),
                error_message: "".to_string(),
            })),
        })
    }

    async fn handle_gen_batch_chunks_request(
        &self,
        msg_id: String,
        request: GenBatchChunks,
        client: Arc<Provider<Http>>,
    ) -> Result<ProverResponse> {
        // parse the block number from the request
        let block_number = match request.batch {
            None => {
                log::error!("Batch is empty, request id: {:?}", msg_id);
                bail!("Batch is empty");
            }
            Some(batch) => {
                if batch.block_number.is_empty() {
                    log::error!("Block List is empty, request id: {:?}", msg_id);
                    bail!("Block List is empty");
                } else {
                    // currently, batch only contains one block
                    batch.block_number[0]
                }
            }
        };

        let execute_task_id = format!("{:010}", block_number);

        log::info!(
            "generate chunks for Block: {:?}, request id {:?}",
            block_number,
            msg_id
        );

        let prover_type: ProverType = std::env::var("PROVER_TYPE")
            .unwrap_or("eigen".to_string())
            .into();

        let (_res, l2_batch_data, cnt_chunks) = match prover_type {
            ProverType::Eigen => {
                batch_process(
                    client.clone(),
                    block_number,
                    request.chain_id,
                    &request.program_name,
                    execute_task_id.to_string().as_str(),
                    self.executor_base_dir.as_str(),
                )
                .await
            }
            ProverType::SP1 => {
                let (_res, l2_batch_data) = gen_block_json(
                    client.clone(),
                    block_number,
                    request.chain_id,
                )
                .await;
                (_res, l2_batch_data, 1)
            }
        };
        

        // get previous block state root
        let previous_block_number = block_number - 1;
        let previous_block = match client.get_block_with_txs(previous_block_number).await {
            Ok(Some(block)) => block,
            Ok(None) => bail!("Previous block:{} not found", previous_block_number),
            Err(error) => bail!(
                "Failed to get previous block:{} err: {:?}",
                previous_block_number,
                error
            ),
        };

        let pre_state_root = previous_block.state_root;

        let block_test_unit: models::TestUnit = serde_json::from_str(&l2_batch_data)
            .map_err(|e| anyhow!("Failed to parse test unit: {:?}", e))?;

        let block_test = block_test_unit
            .post
            .get(&models::SpecName::Shanghai)
            .ok_or_else(|| anyhow!("Failed to get block test"))?;

        let post_state_hash = block_test
            .last()
            .ok_or_else(|| anyhow!("Failed to get last block test"))?
            .hash;

        let pre_state_root = <[u8; 32]>::from(pre_state_root);
        let post_state_root = *post_state_hash;

        Ok(ProverResponse {
            id: msg_id,
            response_type: Some(ResponseType::GenBatchProof(GenBatchProofResponse {
                step: Some(gen_batch_proof_response::Step::GenBatchChunks(
                    GenBatchChunksResult {
                        batch_id: request.batch_id,
                        task_id: execute_task_id,
                        result_code: 0,
                        chunk_count: cnt_chunks as u64,
                        batch_data: l2_batch_data,
                        pre_state_root: Vec::from(pre_state_root),
                        post_state_root: Vec::from(post_state_root),
                        error_message: "".to_string(),
                    },
                )),
            })),
        })
    }

    async fn handle_gen_chunks_proof_request(
        &self,
        msg_id: String,
        request: GenChunkProof,
    ) -> Result<ProverResponse> {
        let execute_task_id = request.task_id.clone();
        let cnt_chunk = request.chunk_count as usize;
        let l2_batch_data = request.batch_data;

        // gen chunks proof
        // distribute tasks according to the number of chunks

        // key:  format!("{}_{}", task_id, chunk_id)
        // pending task
        let mut pending_tasks = Vec::<String>::new();
        for chunk_id in 0..cnt_chunk {
            pending_tasks.push(format!("{}_{}", execute_task_id, chunk_id))
        }

        let mut finished_tasks = vec![];
        let mut results = vec![String::new(); cnt_chunk];

        // put the task into the pipeline, skip the finished tasks
        for (index, key) in pending_tasks.iter().enumerate() {
            // let proof_result = PIPELINE.lock().unwrap().get_proof(key.clone(), 0);
            let tmp_stage = Stage::Batch(
                execute_task_id.clone(),
                index.to_string(),
                l2_batch_data.clone(),
            );
            let task_result_dir = Path::new(&*BASE_DIR)
                .join(tmp_stage.path())
                .join("status.finished");
            log::info!("check the task status: {}", task_result_dir.display());
            let status = match std::fs::read_to_string(task_result_dir) {
                Ok(flag) => {
                    log::info!("task({}) status: {}", key, flag);
                    match flag.trim() {
                        "1" => true,
                        "0" => false,
                        _ => false,
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to read task status, gen proof again, err: {:?}, ",
                        e
                    );
                    false
                }
            };

            match status {
                true => {
                    // already finished, skip the task
                    log::info!("task: {:?}, index: {}, already finished, skip", key, index);
                    finished_tasks.push(index);
                    results.insert(index, key.clone());
                }
                false => {
                    // not finished, put the task to the pipeline
                    log::info!("task: {:?} not finished, put the task to pipeline", key);
                    match PIPELINE.lock().unwrap().batch_prove(
                        execute_task_id.to_string(),
                        index.to_string(),
                        l2_batch_data.clone(),
                    ) {
                        Ok(_) => continue,
                        Err(err) => {
                            bail!("Failed to generate batch proof: {:?}", err);
                        }
                    };
                }
            }
        }

        // remove finished tasks from pending tasks
        for &index in finished_tasks.iter().rev() {
            pending_tasks.remove(index);
        }
        finished_tasks.clear();
        log::info!("pending tasks: {:?}", pending_tasks);
        log::info!("load results of the task list: {:?}", results);

        // waiting for the proof result
        let mut polling_ticker = time::interval(DEFAULT_BATCH_PROOF_POLLING_INTERVAL);
        let timeout_start = time::Instant::now() + DEFAULT_BATCH_PROOF_POLLING_TIMEOUT;
        let mut timeout_ticker =
            time::interval_at(timeout_start, DEFAULT_BATCH_PROOF_POLLING_TIMEOUT);
        let (finish_tx, mut finish_rx) = watch::channel::<()>(());

        log::info!(
            "polling the batch proof of task id: {:?}, request id {:?}",
            execute_task_id,
            msg_id
        );
        loop {
            tokio::select! {
                _ = polling_ticker.tick() => {
                    // get proof result
                    for (index, key) in pending_tasks.iter().enumerate() {
                        let proof_result = PIPELINE.lock().unwrap().get_proof(key.clone(), 0);
                        match proof_result {
                            Ok(task_key) => {
                                // do nothing
                                finished_tasks.push(index);
                                results.insert(index, task_key);
                                log::info!("task: {:?} is finished", key);
                            }
                            Err(e) => {
                                log::info!("task: {:?} is not finished, check again later, check result: {:#?}", key, e);

                                // false, continue
                                continue;
                                // TODO: other error, stop and return error
                            }
                        }
                    }

                    // remove finished tasks from pending tasks
                    for &index in finished_tasks.iter().rev() {
                        if index < pending_tasks.len() {
                            pending_tasks.remove(index);
                        }
                    }
                    finished_tasks.clear();
                    if pending_tasks.is_empty() {
                        // finished
                        log::info!("all tasks are finished");
                        finish_tx.send(()).unwrap();
                        continue;
                    }
                    log::info!("end of polling, try again later, pending tasks: {:?}", pending_tasks);
                }
                _ = finish_rx.changed() => {
                    break;
                }
                _ = timeout_ticker.tick() => {
                    log::info!("generate the proof timeout, task id: {:?}, request id {:?}", execute_task_id, msg_id);
                    bail!("generate batch proof timeout");
                }
            }
        }

        log::info!(
            "Finished the task of generate batch proof, task id: {:?}, request id {:?}",
            execute_task_id,
            msg_id
        );

        let mut batch_proof_result = BatchProofResult {
            task_id: execute_task_id.to_string(),
            ..Default::default()
        };

        for (chunk_id, chunk_proof_key) in results.iter().enumerate().take(cnt_chunk) {
            let chunk_proof = ChunkProof {
                chunk_id: chunk_id as u64,
                proof_key: chunk_proof_key.clone(),
                proof: format!("{}_chunk_{}", execute_task_id, chunk_id),
            };

            batch_proof_result.chunk_proofs.push(chunk_proof);
        }

        Ok(ProverResponse {
            id: msg_id,
            response_type: Some(ResponseType::GenBatchProof(GenBatchProofResponse {
                step: Some(gen_batch_proof_response::Step::GenChunkProof(
                    GenChunkProofResult {
                        batch_id: request.batch_id.clone(),
                        task_id: execute_task_id,
                        result_code: 0,
                        batch_proof_result: Some(batch_proof_result),
                        error_message: "".to_string(),
                    },
                )),
            })),
        })
    }

    async fn handle_gen_aggregated_proof_request(
        &self,
        msg_id: String,
        request: GenAggregatedProofRequest,
    ) -> Result<ProverResponse> {
        // put the task into the pipeline
        let task_id = match PIPELINE.lock().unwrap().aggregate_prove(
            request.recursive_proof_1.clone(),
            request.recursive_proof_2.clone(),
        ) {
            Ok(id) => id,
            Err(e) => bail!("Failed to generate aggregated proof: {:?}", e.to_string()),
        };

        log::info!(
            "generate agg proof, task_id: {:?}, request id {:?}",
            task_id,
            msg_id
        );

        // waiting for the proof result
        let mut polling_ticker = time::interval(DEFAULT_AGGREGATED_PROOF_POLLING_INTERVAL);
        let timeout_start = time::Instant::now() + DEFAULT_AGGREGATED_PROOF_POLLING_TIMEOUT;
        let mut timeout_ticker =
            time::interval_at(timeout_start, DEFAULT_AGGREGATED_PROOF_POLLING_TIMEOUT);

        log::info!(
            "polling the agg proof of agg_task: {:?}, request id {:?}",
            task_id,
            msg_id
        );

        let checkpoint_key = format!("{}_agg", task_id.clone());
        // let result_key: String;
        loop {
            tokio::select! {
                _ = polling_ticker.tick() => {
                    let proof_result = PIPELINE.lock().unwrap().get_proof(checkpoint_key.clone(), 0);
                    match proof_result {
                        Ok(_) => {
                            // result_key = task_key;
                            break;
                        }
                        Err(_) => {
                                // false, continue
                                continue;
                                // TODO: other error, stop and return error
                        }
                    }
                }
                _ = timeout_ticker.tick() => {
                    log::info!("generate agg proof timeout, task_id: {:?}, request id {:?}", task_id, msg_id);
                    bail!("generate aggregated proof timeout");
                }
            }
        }

        Ok(ProverResponse {
            id: msg_id,
            response_type: Some(ResponseType::GenAggregatedProof(
                GenAggregatedProofResponse {
                    batch_id: request.batch_id,
                    result_code: ProofResultCode::CompletedOk as i32,
                    result_string: task_id,
                    error_message: "".to_string(),
                },
            )),
        })
    }

    async fn handle_gen_final_proof_request(
        &self,
        msg_id: String,
        request: GenFinalProofRequest,
    ) -> Result<ProverResponse> {
        let task_id = match PIPELINE.lock().unwrap().final_prove(
            request.recursive_proof.clone(),
            request.curve_name.clone(),
            request.aggregator_addr.clone(),
        ) {
            Ok(id) => id,
            Err(e) => bail!("Failed to generate final proof: {:?}", e.to_string()),
        };

        log::info!(
            "generate final proof, task_id: {:?}, request id {:?}",
            task_id,
            msg_id
        );

        // waiting for the proof result
        let mut polling_ticker = time::interval(DEFAULT_FINAL_PROOF_POLLING_INTERVAL);
        let timeout_start = time::Instant::now() + DEFAULT_FINAL_PROOF_POLLING_TIMEOUT;
        let mut timeout_ticker =
            time::interval_at(timeout_start, DEFAULT_FINAL_PROOF_POLLING_TIMEOUT);

        log::info!(
            "polling the final proof of agg_task: {:?}, request id {:?}",
            task_id,
            msg_id
        );

        let checkpoint_key = format!("{}_final", task_id.clone());

        loop {
            tokio::select! {
                _ = polling_ticker.tick() => {
                    let proof_result = PIPELINE.lock().unwrap().get_proof(checkpoint_key.clone(), 0);
                    match proof_result {
                        Ok(_task_key) => {
                            log::info!("finished the final stage!");
                            break;
                        }
                        Err(_) => {
                                // false, continue
                                continue;
                                // TODO: other error, stop and return error
                        }
                    }
                }
                _ = timeout_ticker.tick() => {
                    bail!("generate final proof timeout");
                }
            }
        }

        let (proof, public_input) = PIPELINE
            .lock()
            .unwrap()
            .load_final_proof_and_input(&checkpoint_key)?;

        Ok(ProverResponse {
            id: msg_id,
            response_type: Some(ResponseType::GenFinalProof(GenFinalProofResponse {
                batch_id: request.batch_id,
                result_code: ProofResultCode::CompletedOk as i32,
                result_string: "".to_string(),
                final_proof: Some(FinalProof {
                    proof,
                    public_input,
                }),
                error_message: "".to_string(),
            })),
        })
    }
}
