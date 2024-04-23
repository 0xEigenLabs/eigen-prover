// TODO: Fixme
#![allow(clippy::all)]
#![allow(unknown_lints)]

use std::env::var;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::prover_service::prover_service::get_status_response::Status::Idle;
use crate::prover_service::prover_service::prover_request::RequestType;
use crate::prover_service::prover_service::prover_response::ResponseType;
use crate::prover_service::prover_service::{
    get_status_response, BatchProofResult, ChunkProof, GenAggregatedProofRequest,
    GenAggregatedProofResponse, GenBatchProofRequest, GenBatchProofResponse, GenFinalProofRequest,
    GenFinalProofResponse, GetStatusRequest, GetStatusResponse, GetStatusResultCode,
    ProofResultCode, ProverRequest, ProverResponse, ProverStatus,
};
use anyhow::{bail, Result};
use ethers_providers::{Http, Provider};
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
                                        id: "".to_string(),
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
                                        id: "".to_string(),
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
                                        id: "".to_string(),
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
    // TODO: refactor
    curve_type: String,
}

impl ProverRequestHandler {
    pub fn new() -> Self {
        ProverRequestHandler {
            curve_type: var("CURVE_TYPE").unwrap_or(String::from("BN128")),
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

        // TODO: get from request?
        let task = var("TASK").unwrap_or(String::from("evm"));

        let base_dir = var("BASEDIR").unwrap_or(String::from("/tmp/prover/data/proof"));
        let execute_task_id = uuid::Uuid::new_v4();
        let chain_id = var("CHAINID").unwrap_or(String::from("1"));

        log::info!(
            "generate chunks for Block: {:?}, request id {:?}",
            block_number,
            msg_id
        );
        // gen chunk
        let (_res, cnt_chunks) = batch_process(
            client,
            block_number,
            chain_id.parse::<u64>().unwrap(),
            &task,
            execute_task_id.to_string().as_str(),
            base_dir.as_str(),
        )
        .await;

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
            match PIPELINE
                .lock()
                .unwrap()
                .batch_prove(execute_task_id.to_string(), chunk_id.to_string())
            {
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
        let mut results = vec![];
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
                        pending_tasks.remove(index);
                    }
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
                proof: "".to_string(),
            };

            batch_proof_result.chunk_proofs.push(chunk_proof);
        }

        Ok(ProverResponse {
            id: msg_id,
            response_type: Some(ResponseType::GenBatchProof(GenBatchProofResponse {
                id: "".to_string(),
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

        let result_key: String;
        loop {
            tokio::select! {
                _ = polling_ticker.tick() => {
                    let proof_result = PIPELINE.lock().unwrap().get_proof(task_id.clone(), 0);
                    match proof_result {
                        Ok(task_key) => {
                            result_key = task_key;
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
                    id: task_id,
                    result_code: ProofResultCode::CompletedOk as i32,
                    result_string: result_key,
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
            // request.curve_name.clone(),
            self.curve_type.clone(),
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

        loop {
            tokio::select! {
                _ = polling_ticker.tick() => {
                    // TODO: Read the proof and public_input from disk
                    let proof_result = PIPELINE.lock().unwrap().get_proof(task_id.clone(), 0);
                    match proof_result {
                        Ok(_task_key) => {
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

        // TODO: Get the proof data from disk
        // TODO: Get the public_input from disk
        Ok(ProverResponse {
            id: msg_id,
            response_type: Some(ResponseType::GenFinalProof(GenFinalProofResponse {
                id: task_id,
                result_code: ProofResultCode::CompletedOk as i32,
                result_string: "".to_string(),
                final_proof: None,
                error_message: "".to_string(),
            })),
        })
    }
}
