#![allow(clippy::all)]
#![allow(unknown_lints)]
use std::env::var;
use std::pin::Pin;
use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{self, Request, Response, Status};

use aggregator_service::aggregator_service_server::AggregatorService;
use aggregator_service::{
    aggregator_message, prover_message, AggregatorMessage, CancelResponse,
    GenAggregatedProofResponse, GenBatchProofResponse, GenFinalProofResponse, GetProofResponse,
    GetStatusResponse, ProverMessage,
};
use prover::Pipeline;

pub mod aggregator_service {
    tonic::include_proto!("aggregator.v1"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct AggregatorServiceSVC {}

fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {
    let mut err: &(dyn std::error::Error + 'static) = err_status;

    loop {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }

        // h2::Error do not expose std::io::Error with `source()`
        // https://github.com/hyperium/h2/pull/462
        /*
        if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
            if let Some(io_err) = h2_err.get_io() {
                return Some(io_err);
            }
        }
        */

        err = match err.source() {
            Some(err) => err,
            None => return None,
        };
    }
}

lazy_static! {
    static ref PIPELINE: Mutex<Pipeline> = Mutex::new(Pipeline::new(
        var("WORKSPACE").unwrap_or("/tmp/prover".to_string()),
        var("TASK_NAME").unwrap_or("fib".to_string())
    ));
}

pub fn prove() -> algebraic::errors::Result<()> {
    PIPELINE.lock().unwrap().prove()
}

#[tonic::async_trait]
impl AggregatorService for AggregatorServiceSVC {
    type ChannelStream = Pin<Box<dyn Stream<Item = Result<ProverMessage, Status>> + Send>>;

    async fn channel(
        &self,
        request: Request<tonic::Streaming<AggregatorMessage>>, // Accept request of type HelloRequest
    ) -> Result<Response<Self::ChannelStream>, Status> {
        // Return an instance of type HelloReply
        log::info!("Got a request: {:?}", request);

        let mut in_stream = request.into_inner();
        // spawn and channel are required if you want handle "disconnect" functionality
        // the `out_stream` will not be polled after client disconnect
        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(item) = in_stream.next().await {
                match item {
                    Ok(v) => {
                        let resp = match v.request {
                            Some(req) => match req {
                                aggregator_message::Request::GetStatusRequest(_req) => {
                                    // TODO: return prover info
                                    let status = match PIPELINE.lock().unwrap().get_status() {
                                        Ok(_) => 1,
                                        _ => 2,
                                    };
                                    Some(prover_message::Response::GetStatusResponse(
                                        GetStatusResponse {
                                            status,
                                            ..Default::default()
                                        },
                                    ))
                                }
                                aggregator_message::Request::GenBatchProofRequest(req) => {
                                    let result = PIPELINE.lock().unwrap().batch_prove(
                                        req.input
                                            .unwrap()
                                            .public_inputs
                                            .unwrap()
                                            .batch_l2_data
                                            .clone(),
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
                                    //let id = resp.current_computing_request_id;
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
                                    //let id = resp.current_computing_request_id;
                                    let result = PIPELINE.lock().unwrap().final_prove(
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
                                    let result =
                                        match PIPELINE.lock().unwrap().cancel(req.id.clone()) {
                                            Ok(_) => 1,
                                            _ => 2,
                                        };
                                    Some(prover_message::Response::CancelResponse(CancelResponse {
                                        result,
                                    }))
                                }
                                aggregator_message::Request::GetProofRequest(req) => {
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
                            None => None,
                        };

                        tx.send(Ok(ProverMessage {
                            id: v.id,
                            response: resp,
                        }))
                        .await
                        .expect("working rx")
                    }
                    Err(err) => {
                        if let Some(io_err) = match_for_io_error(&err) {
                            if io_err.kind() == std::io::ErrorKind::BrokenPipe {
                                // here you can handle special case when client
                                // disconnected in unexpected way
                                eprintln!("\tclient disconnected: broken pipe");
                                break;
                            }
                        }

                        match tx.send(Err(err)).await {
                            Ok(_) => (),
                            Err(_err) => break, // response was droped
                        }
                    }
                }
            }
            log::info!("\tclient disconnected");
        });

        let reply = ReceiverStream::new(rx);

        Ok(Response::new(Box::pin(reply) as Self::ChannelStream))
    }
}
