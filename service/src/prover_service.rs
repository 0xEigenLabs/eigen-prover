// TODO: Fixme
#![allow(clippy::all)]
#![allow(unknown_lints)]

use std::collections::HashMap;
use std::env::var;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::prover_service::prover_service::get_status_response::Status::Idle;
use crate::prover_service::prover_service::prover_request::RequestType;
use crate::prover_service::prover_service::prover_response::ResponseType;
use crate::prover_service::prover_service::FinalProof;
use crate::prover_service::prover_service::{
    get_status_response, BatchProofResult, ChunkProof, GenAggregatedProofRequest,
    GenAggregatedProofResponse, GenBatchProofRequest, GenBatchProofResponse, GenFinalProofRequest,
    GenFinalProofResponse, GetStatusRequest, GetStatusResponse, GetStatusResultCode,
    ProofResultCode, ProverRequest, ProverResponse, ProverStatus,
};
use anyhow::{anyhow, bail, Result};
use ethers_providers::{Http, Middleware, Provider};
use executor::batch_process;
use prover::contexts::BatchContext;
use prover::pipeline::Pipeline;
use prover_service::prover_service_server::ProverService;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, watch};
use tokio::time;
use tokio_stream::{Stream, StreamExt};
use tonic::{async_trait, Request, Response, Status, Streaming};

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
    static ref PIPELINE: Mutex<Pipeline> = Mutex::new(Pipeline::new(
        var("WORKSPACE").unwrap_or("/tmp/prover/data".to_string()),
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
                        RequestType::GenBatchProof(req) => handler_clone
                            .handle_gen_batch_proof_request(
                                request_id.clone(),
                                req,
                                eth_client_clone.clone(),
                            )
                            .await
                            .unwrap_or_else(|e| ProverResponse {
                                id: request_id.clone(),
                                response_type: Some(ResponseType::GenBatchProof(
                                    GenBatchProofResponse {
                                        batch_id: "".to_string(),
                                        result_code: ProofResultCode::CompletedError as i32,
                                        batch_proof_result: None,
                                        error_message: e.to_string(),
                                    },
                                )),
                            }),
                        RequestType::GenAggregatedProof(r) => handler_clone
                            .handle_gen_aggregated_proof_request(request_id.clone(), r)
                            .await
                            .unwrap_or_else(|e| ProverResponse {
                                id: request_id.clone(),
                                response_type: Some(ResponseType::GenAggregatedProof(
                                    GenAggregatedProofResponse {
                                        batch_id: "".to_string(),
                                        result_code: ProofResultCode::CompletedError as i32,
                                        result_string: "".to_string(),
                                        error_message: e.to_string(),
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
                                        batch_id: "".to_string(),
                                        result_code: ProofResultCode::CompletedError as i32,
                                        result_string: "".to_string(),
                                        final_proof: None,
                                        error_message: e.to_string(),
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
    async fn handle_gen_batch_proof_request(
        &self,
        msg_id: String,
        request: GenBatchProofRequest,
        client: Arc<Provider<Http>>,
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
    batch_state_root: Arc<Mutex<HashMap<String, BatchStateRoot>>>,
}

pub struct BatchStateRoot {
    pub prev_state_root: [u8; 32],
    pub post_state_root: [u8; 32],
}

impl ProverRequestHandler {
    pub fn new(executor_base_dir: String) -> Self {
        ProverRequestHandler {
            executor_base_dir,
            batch_state_root: Arc::new(Mutex::new(HashMap::new())),
        }
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

    async fn handle_gen_batch_proof_request(
        &self,
        msg_id: String,
        request: GenBatchProofRequest,
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

        let execute_task_id = uuid::Uuid::new_v4();

        log::info!(
            "generate chunks for Block: {:?}, request id {:?}",
            block_number,
            msg_id
        );
        // gen chunk
        let (_res, l2_batch_data, cnt_chunks) = batch_process(
            client.clone(),
            block_number,
            request.chain_id,
            &request.program_name,
            execute_task_id.to_string().as_str(),
            self.executor_base_dir.as_str(),
        )
        .await;

        // TODO: refactor to batch
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

        let block_state_root = BatchStateRoot {
            prev_state_root: <[u8; 32]>::from(pre_state_root),
            post_state_root: *post_state_hash,
        };

        // Reduce the lifecycle of the mutex and release it as soon as possible
        {
            self.batch_state_root
                .lock()
                .map_err(|e| anyhow!("get state root's lock failed: {:?}", e))?
                .insert(request.batch_id.clone(), block_state_root);
        }

        log::info!(
            "put the task to pipline, Block: {:?} request id {:?}",
            block_number,
            msg_id
        );
        // gen proof
        // distribute tasks according to the number of chunks
        // put the task into the pipeline
        let mut pending_tasks = Vec::<String>::new();
        for chunk_id in 0..cnt_chunks {
            // FIXME: don't clone the l2 batch data
            match PIPELINE.lock().unwrap().batch_prove(
                execute_task_id.to_string(),
                chunk_id.to_string(),
                l2_batch_data.clone(),
            ) {
                Ok(key) => pending_tasks.push(key),
                Err(e) => {
                    bail!("Failed to generate batch proof: {:?}", e);
                }
            };
        }

        // waiting for the proof result
        let mut polling_ticker = time::interval(DEFAULT_BATCH_PROOF_POLLING_INTERVAL);
        let timeout_start = time::Instant::now() + DEFAULT_BATCH_PROOF_POLLING_TIMEOUT;
        let mut timeout_ticker =
            time::interval_at(timeout_start, DEFAULT_BATCH_PROOF_POLLING_TIMEOUT);
        let (finish_tx, mut finish_rx) = watch::channel::<()>(());
        let mut finished_tasks = vec![];
        let mut results = vec![String::new(); cnt_chunks];
        log::info!(
            "polling the batch proof of Block: {:?}, request id {:?}",
            block_number,
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
                            }
                            Err(_e) => {
                                // false, continue
                                continue;
                                // TODO: other error, stop and return error
                            }
                        }
                    }

                    // remove finished tasks from pending tasks
                    for &index in finished_tasks.iter() {
                        if index < pending_tasks.len() {
                            pending_tasks.remove(index);
                        }
                    }
                    finished_tasks.clear();
                    if pending_tasks.is_empty() {
                        // finished
                        finish_tx.send(()).unwrap();
                        continue;
                    }
                }
                _ = finish_rx.changed() => {
                    break;
                }
                _ = timeout_ticker.tick() => {
                    log::info!("generate the proof timeout: {:?}, request id {:?}", block_number, msg_id);
                    bail!("generate batch proof timeout");
                }
            }
        }

        log::info!(
            "Finished the task of generate batch proof, Block: {:?}, request id {:?}",
            block_number,
            msg_id
        );

        let mut batch_proof_result = BatchProofResult::default();
        batch_proof_result.task_id = execute_task_id.to_string();

        for chunk_id in 0..cnt_chunks {
            let chunk_proof = ChunkProof {
                chunk_id: chunk_id as u64,
                proof_key: results[chunk_id].clone(),
                // we don't need to return the proof data
                // just return the proof key, key: {task_id}_chunk_{chunk_id}
                proof: format!("{}_chunk_{}", execute_task_id, chunk_id),
            };

            batch_proof_result.chunk_proofs.push(chunk_proof);
        }

        Ok(ProverResponse {
            id: msg_id,
            response_type: Some(ResponseType::GenBatchProof(GenBatchProofResponse {
                batch_id: request.batch_id,
                result_code: ProofResultCode::CompletedOk as i32,
                batch_proof_result: Some(batch_proof_result),
                error_message: "".to_string(),
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

        let block_state_root = self
            .batch_state_root
            .lock()
            .map_err(|e| anyhow!("get state root's lock failed: {:?}", e))?
            .remove(&request.batch_id)
            .ok_or_else(|| anyhow!("Failed to get the block state root, key: {}", msg_id))?;

        Ok(ProverResponse {
            id: msg_id,
            response_type: Some(ResponseType::GenFinalProof(GenFinalProofResponse {
                batch_id: request.batch_id,
                result_code: ProofResultCode::CompletedOk as i32,
                result_string: "".to_string(),
                final_proof: Some(FinalProof {
                    proof,
                    public_input,
                    pre_state_root: Vec::from(block_state_root.prev_state_root),
                    post_state_root: Vec::from(block_state_root.post_state_root),
                }),
                error_message: "".to_string(),
            })),
        })
    }
}
