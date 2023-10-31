#![allow(clippy::all)]
#![allow(unknown_lints)]
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{self, Request, Response, Status};

use aggregator_service::aggregator_service_server::AggregatorService;
use aggregator_service::{
    aggregator_message, prover_message, AggregatorMessage, CancelRequest,
    GenAggregatedProofRequest, GenBatchProofRequest, GenFinalProofRequest, GetProofRequest,
    GetStatusRequest, ProverMessage,
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

#[tonic::async_trait]
impl AggregatorService for AggregatorServiceSVC {
    type ChannelStream = Pin<Box<dyn Stream<Item = Result<AggregatorMessage, Status>> + Send>>;

    async fn channel(
        &self,
        request: Request<tonic::Streaming<ProverMessage>>, // Accept request of type HelloRequest
    ) -> Result<Response<Self::ChannelStream>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let mut in_stream = request.into_inner();
        // spawn and channel are required if you want handle "disconnect" functionality
        // the `out_stream` will not be polled after client disconnect
        let (tx, rx) = mpsc::channel(128);
        let pipeline = Pipeline::new("/tmp/prover".to_string(), "fib".to_string());
        tokio::spawn(async move {
            while let Some(item) = in_stream.next().await {
                match item {
                    Ok(v) => {
                        let resp = match v.response {
                            Some(resp) => match resp {
                                prover_message::Response::GetStatusResponse(resp) => {
                                    let id = resp.current_computing_request_id;
                                    let result = pipeline.get_status(id);
                                    Some(aggregator_message::Request::GetStatusRequest(
                                        GetStatusRequest::default(),
                                    ))
                                }
                                prover_message::Response::GenBatchProofResponse(resp) => {
                                    //let id = resp.current_computing_request_id;
                                    Some(aggregator_message::Request::GenBatchProofRequest(
                                        GenBatchProofRequest::default(),
                                    ))
                                }
                                prover_message::Response::GenAggregatedProofResponse(resp) => {
                                    //let id = resp.current_computing_request_id;
                                    Some(aggregator_message::Request::GenAggregatedProofRequest(
                                        GenAggregatedProofRequest::default(),
                                    ))
                                }
                                prover_message::Response::GenFinalProofResponse(resp) => {
                                    //let id = resp.current_computing_request_id;
                                    Some(aggregator_message::Request::GenFinalProofRequest(
                                        GenFinalProofRequest::default(),
                                    ))
                                }
                                prover_message::Response::CancelResponse(resp) => {
                                    //let id = resp.current_computing_request_id;
                                    Some(aggregator_message::Request::CancelRequest(
                                        CancelRequest::default(),
                                    ))
                                }
                                prover_message::Response::GetProofResponse(resp) => {
                                    //let id = resp.current_computing_request_id;
                                    Some(aggregator_message::Request::GetProofRequest(
                                        GetProofRequest::default(),
                                    ))
                                }
                            },
                            None => None,
                        };

                        tx.send(Ok(AggregatorMessage {
                            id: v.id,
                            request: None,
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
            println!("\tclient disconnected");
        });

        let reply = ReceiverStream::new(rx);

        Ok(Response::new(Box::pin(reply) as Self::ChannelStream))
    }
}
