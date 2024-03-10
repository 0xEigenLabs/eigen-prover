#![allow(clippy::all)]
#![allow(unknown_lints)]

use anyhow::{anyhow, Result};
use proto::aggregator::{
    aggregator_message::Request::*,
    aggregator_service_client::AggregatorServiceClient,
    get_status_response,
    prover_message,
    prover_message::Response::*,
    CancelResponse,
    GenAggregatedProofResponse,
    GenBatchProofResponse,
    GenFinalProofResponse,
    GetProofResponse,
    GetStatusResponse,
    ProverMessage,
    // PublicInputs, InputProver,
};
use std::env::var;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::Request;

use prover::pipeline::Pipeline;

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

pub async fn run_prover() -> Result<()> {
    PIPELINE.lock().unwrap().prove()
}

pub async fn run_client() -> Result<()> {
    let addr = std::env::var("NODE_ADDR").unwrap_or("http://[::1]:50051".to_string());
    let mut client = AggregatorServiceClient::connect(addr.clone())
        .await
        .map_err(|e| anyhow!("Connect {}, error: {:?}", addr, e))?;

    log::debug!("streaming aggregator:");

    let (tx, rx) = mpsc::channel(128);

    let out_stream = ReceiverStream::new(rx);

    let response = client
        .channel(Request::new(out_stream))
        .await
        .map_err(|e| anyhow!(format!("receive channel: {:?}", e)))?;

    let mut resp_stream = response.into_inner();

    while let Some(received) = resp_stream.next().await {
        let received = received.map_err(|e| anyhow!(format!("client close socket {}", e)))?;
        let req_id = received.id.clone();
        log::debug!("debug new req {}", req_id.clone());
        if let Some(request) = received.request {
            let resp = match request {
                GetStatusRequest(_req) => {
                    // step 1: get prover status
                    let status = match PIPELINE.lock().unwrap().get_status() {
                        Ok(_) => get_status_response::Status::Idle,
                        _ => get_status_response::Status::Unspecified,
                    };
                    // TODO: cpu and mem usage: https://github.com/GuillaumeGomez/sysinfo
                    GetStatusResponse(GetStatusResponse {
                        status: status.into(),
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
                        fork_id: *PROVER_FORK_ID,
                    })
                }
                GenBatchProofRequest(req) => {
                    // step 2: submit input to prover, and get task id
                    let input = req.input.unwrap();
                    let _public_input = input.public_inputs.unwrap();
                    let _contract_bytecode = input.contracts_bytecode;
                    let _db = input.db;
                    //let req_id = received.id.clone(); // execute_task_id_chunk_chunk_id
                    let execute_task_id = req.execute_task_id;
                    let chunk_id = req.chunk_id;
                    let task_id = format!("{}_chunk_{}", execute_task_id, chunk_id);
                    let result = match PIPELINE
                        .lock()
                        .unwrap()
                        .batch_prove(execute_task_id, chunk_id)
                    {
                        Ok(_) => proto::aggregator::Result::Ok,
                        _ => proto::aggregator::Result::Error,
                    };
                    GenBatchProofResponse(GenBatchProofResponse {
                        id: task_id.to_string(),
                        result: result.into(),
                    })
                }

                GenAggregatedProofRequest(req) => {
                    // step 4: submit 2 proofs to aggregate, and goto step 3 again
                    let (id, result) = match PIPELINE.lock().unwrap().aggregate_prove(
                        req.recursive_proof_1.clone(),
                        req.recursive_proof_2.clone(),
                    ) {
                        Ok(id) => (id, proto::aggregator::Result::Ok),
                        _ => ("".into(), proto::aggregator::Result::Error),
                    };
                    GenAggregatedProofResponse(GenAggregatedProofResponse {
                        id: id,
                        result: result.into(),
                    })
                }
                GenFinalProofRequest(req) => {
                    // step 5: wrap the stark proof to snark, and goto step 3 again
                    let task_id = uuid::Uuid::new_v4();
                    let req_id = received.id.clone();
                    let result = match PIPELINE.lock().unwrap().final_prove(
                        req_id.clone(),
                        req.recursive_proof.clone(),
                        req.aggregator_addr.clone(),
                    ) {
                        Ok(_) => proto::aggregator::Result::Ok,
                        _ => proto::aggregator::Result::Error,
                    };
                    GenFinalProofResponse(GenFinalProofResponse {
                        id: task_id.to_string(),
                        result: result.into(),
                    })
                }
                CancelRequest(req) => {
                    let result = match PIPELINE.lock().unwrap().cancel(req.id.clone()) {
                        Ok(_) => proto::aggregator::Result::Ok,
                        _ => proto::aggregator::Result::Error,
                    };
                    prover_message::Response::CancelResponse(CancelResponse {
                        result: result.into(),
                    })
                }
                GetProofRequest(req) => {
                    // step 3: fetch proving progress by task id, and get the proof data
                    let (_proof, res) = match PIPELINE
                        .lock()
                        .unwrap()
                        .get_proof(req.id.clone(), req.timeout)
                    {
                        Ok(proof) => (proof, proto::aggregator::Result::Ok),
                        _ => ("".to_string(), proto::aggregator::Result::Error),
                    };

                    // TODO: check if it a final proof or recursive proof by status file
                    // get_proof_response::Proof::RecursiveProof()
                    // get_proof_response::Proof::FinalProof()

                    GetProofResponse(GetProofResponse {
                        id: req.id.clone(),
                        result: res.into(),
                        result_string: "".into(),
                        proof: None,
                    })
                }
            };
            tx.send(ProverMessage {
                id: received.id.clone(),
                response: Some(resp),
            })
            .await
            .map_err(|e| anyhow!(format!("send message, {:?}", e)))?;
        } else {
            log::debug!("Sleep for next message");
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    Ok(())
}
