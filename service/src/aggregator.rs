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
    aggregator_message, prover_message, AggregatorMessage, CancelRequest,
    GenAggregatedProofRequest, GenBatchProofRequest, GenFinalProofRequest, GetProofRequest,
    GetStatusRequest, InputProver, ProverMessage, PublicInputs,
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
    type ChannelStream = Pin<Box<dyn Stream<Item = Result<AggregatorMessage, Status>> + Send>>;

    async fn channel(
        &self,
        request: Request<tonic::Streaming<ProverMessage>>, // Accept request of type HelloRequest
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
                        let req = match v.response {
                            Some(req) => match req {
                                prover_message::Response::GetStatusResponse(resp) => {
                                    let _status = resp.status;
                                    /*
                                    let _status = match PIPELINE.lock().unwrap().get_status() {
                                        Ok(_) => 1,
                                        _ => 2,
                                    };
                                    */
                                    log::info!("GetStatusRequest");
                                    Some(aggregator_message::Request::GetStatusRequest(
                                        GetStatusRequest {},
                                    ))
                                }
                                prover_message::Response::GenBatchProofResponse(resp) => {
                                    let id = &resp.id;
                                    let _result = &resp.result;
                                    /*
                                    let _result =
                                        PIPELINE.lock().unwrap().batch_prove(id.clone()).unwrap();
                                    */
                                    log::info!("GenBatchProofRequest");

                                    let input_prover = InputProver {
                                        public_inputs: Some(PublicInputs::default()),
                                        db: Default::default(),
                                        contracts_bytecode: Default::default(),
                                    };
                                    Some(aggregator_message::Request::GenBatchProofRequest(
                                        GenBatchProofRequest {
                                            input: Some(input_prover),
                                        },
                                    ))
                                }
                                prover_message::Response::GenAggregatedProofResponse(resp) => {
                                    let id = &resp.id;
                                    let _result = &resp.result;
                                    /*
                                    let _result = PIPELINE
                                        .lock()
                                        .unwrap()
                                        .aggregate_prove(id.clone(), "".into());
                                    */
                                    log::info!("GenAggregatedProofRequest");
                                    Some(aggregator_message::Request::GenAggregatedProofRequest(
                                        GenAggregatedProofRequest {
                                            recursive_proof_1: "".into(),
                                            recursive_proof_2: "".into(),
                                        },
                                    ))
                                }
                                prover_message::Response::GenFinalProofResponse(resp) => {
                                    let id = &resp.id;
                                    let _result = &resp.result;
                                    /*
                                    let _result = PIPELINE.lock().unwrap().final_prove(
                                        id.clone(),
                                        "BN128".into(),
                                        "ABC".into(),
                                    );
                                    */
                                    log::info!("GenFinalProofRequest");
                                    Some(aggregator_message::Request::GenFinalProofRequest(
                                        GenFinalProofRequest {
                                            recursive_proof: "".into(),
                                            aggregator_addr: "".into(),
                                        },
                                    ))
                                }
                                prover_message::Response::CancelResponse(resp) => {
                                    let _result = &resp.result;
                                    /*
                                    let _result = match PIPELINE.lock().unwrap().cancel("".into()) {
                                        Ok(_) => 1,
                                        _ => 2,
                                    };
                                    */
                                    log::info!("CancelRequest");
                                    Some(aggregator_message::Request::CancelRequest(
                                        CancelRequest { id: "".into() },
                                    ))
                                }
                                prover_message::Response::GetProofResponse(resp) => {
                                    let id = &resp.id;
                                    let _result = &resp.result;
                                    let _result_string = &resp.result_string;
                                    let _proof = &resp.proof;
                                    /*
                                    let (_res, _str_res) = match PIPELINE
                                        .lock()
                                        .unwrap()
                                        .get_proof(resp.id.clone(), 0)
                                    {
                                        Ok((res, str_res)) => (res, str_res),
                                        _ => (2, "".to_string()),
                                    };
                                    */
                                    log::info!("GetProofRequest");
                                    Some(aggregator_message::Request::GetProofRequest(
                                        GetProofRequest {
                                            id: id.clone(),
                                            timeout: 0,
                                        },
                                    ))
                                }
                            },
                            None => None,
                        };

                        tx.send(Ok(AggregatorMessage {
                            id: v.id,
                            request: req,
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
