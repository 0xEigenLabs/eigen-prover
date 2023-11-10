#![allow(clippy::all)]
#![allow(unknown_lints)]
use algebraic::errors::{EigenError, Result};
use std::env::var;
use std::sync::Mutex;
use std::time;
use tokio_stream::{StreamExt};
use tonic::{self, Request};

use crate::time::{interval, Instant};

use aggregator_service::aggregator_service_client::AggregatorServiceClient;
use aggregator_service::{
    get_status_response,
    aggregator_message, prover_message, CancelResponse,
    GenAggregatedProofResponse,
    GenBatchProofResponse, GenFinalProofResponse,
    GetProofResponse, GetStatusResponse, ProverMessage,
    // PublicInputs, InputProver,
};

use prover::Pipeline;

pub mod aggregator_service {
    tonic::include_proto!("aggregator.v1"); // The string specified here must match the proto package name
}

lazy_static! {
    static ref PIPELINE: Mutex<Pipeline> = Mutex::new(Pipeline::new(
        var("WORKSPACE").unwrap_or("/tmp/prover".to_string()),
        var("TASK_NAME").unwrap_or("fib".to_string())
    ));
}

pub async fn prove() -> Result<()> {
    // PIPELINE.lock().unwrap().prove()

    let addr = std::env::var("NODE_ADDR").unwrap_or("http://[::1]:50051".to_string());
    let mut client = AggregatorServiceClient::connect(addr.clone())
        .await
        .map_err(|e| EigenError::from(format!("Connect {}, error: {:?}", addr, e)))?;

    log::debug!("streaming aggregator:");

    let start = Instant::now();

    let outbound = async_stream::stream! {
        let mut intval = interval(time::Duration::from_secs(1));

        loop {
            let time = intval.tick().await;
            let elapsed = time.duration_since(start);

            yield ProverMessage::default();
        }
    };

    // TODO: how to initialize the first message?
    let response = client.channel(Request::new(outbound)).await.unwrap();

    let mut resp_stream = response.into_inner();

    while let Some(received) = resp_stream.next().await {
        let received = received.unwrap();
        let resp = match received.request {
            Some(req) => match req {
                aggregator_message::Request::GetStatusRequest(req) => {
                    // step 1: get prover status
                    let status = match PIPELINE.lock().unwrap().get_status() {
                        Ok(_) => get_status_response::Status::Booting,
                        _ => get_status_response::Status::Unspecified,
                    };
                    Some(prover_message::Response::GetStatusResponse(
                        GetStatusResponse {
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
                            fork_id: 0,
                        },
                    ))
                }
                aggregator_message::Request::GenBatchProofRequest(req) => {
                    // step 2: submit input to prover, and get task id
                    let result = PIPELINE.lock().unwrap().batch_prove(
                        /*
                        req.input
                            .unwrap()
                            .public_inputs
                            .unwrap()
                            .batch_l2_data
                            .clone(),
                        */
                        "".into()
                    );
                    let (id, res) = match result {
                        Ok(i) => (i, 1),
                        _ => ("".to_string(), 2),
                    };
                    Some(prover_message::Response::GenBatchProofResponse(
                        GenBatchProofResponse {
                            id: id,
                            result: res,
                        },
                    ))
                }

                aggregator_message::Request::GenAggregatedProofRequest(req) => {
                    // step 4: submit 2 proofs to aggregate, and goto step 3 again
                    let result = PIPELINE.lock().unwrap().aggregate_prove(
                        req.recursive_proof_1.clone(),
                        req.recursive_proof_2.clone(),
                    );
                    let (id, res) = match result {
                        Ok(i) => (i, 1),
                        _ => ("".to_string(), 2),
                    };
                    Some(prover_message::Response::GenAggregatedProofResponse(
                        GenAggregatedProofResponse {
                            id: id,
                            result: res,
                        },
                    ))
                }
                aggregator_message::Request::GenFinalProofRequest(req) => {
                    // step 5: wrap the stark proof to snark, and goto step 3 again
                    let result = PIPELINE.lock().unwrap().final_prove(
                        "".into(),
                        req.recursive_proof.clone(),
                        req.aggregator_addr.clone(),
                    );
                    let (id, res) = match result {
                        Ok(i) => (i, 1),
                        _ => ("".to_string(), 2),
                    };
                    Some(prover_message::Response::GenFinalProofResponse(
                        GenFinalProofResponse {
                            id: id,
                            result: res,
                        },
                    ))
                }
                aggregator_message::Request::CancelRequest(req) => {
                    let result = match PIPELINE.lock().unwrap().cancel(req.id.clone()) {
                        Ok(_) => 1,
                        _ => 2,
                    };
                    Some(prover_message::Response::CancelResponse(CancelResponse {
                        result,
                    }))
                }
                aggregator_message::Request::GetProofRequest(req) => {
                    // step 3: fetch proving progress by task id, and get the proof data
                    let (res, str_res) = match PIPELINE
                        .lock()
                        .unwrap()
                        .get_proof(req.id.clone(), req.timeout)
                    {
                        Ok((res, str_res)) => (res, str_res),
                        _ => (2, "".to_string()),
                    };
                    Some(prover_message::Response::GetProofResponse(
                        GetProofResponse {
                            id: req.id.clone(),
                            result: res,
                            result_string: str_res,
                            proof: Default::default(), //FIXME
                        },
                    ))
                }
            },
            _ => {
                log::info!("Request is empty");
                None
            }
        };

        //TODO send response
    }

    Ok(())
}
