use prover::provers::Prover;
use prover::{contexts, provers};
use scheduler_service::scheduler_service_client::SchedulerServiceClient;
use scheduler_service::{
    batch_prover_message, scheduler_message, BatchProverMessage, GetProofResponse,
    GetStatusResponse,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub mod scheduler_service {
    tonic::include_proto!("scheduler.v1");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "http://localhost:8545";
    let mut client = SchedulerServiceClient::connect(addr).await?;

    let (tx, rx) = mpsc::channel(10);

    // send request to registry service

    let request = ReceiverStream::new(rx);
    let response = client.scheduler_stream(request).await?;
    let mut stream = response.into_inner();
    while let Some(recv_msg) = stream.message().await? {
        if let Some(msg) = recv_msg.response {
            let send_msg = match msg {
                scheduler_message::Response::GetStatusRequest(_r) => {
                    // ready
                    BatchProverMessage {
                        id: "".to_string(),
                        request: Some(batch_prover_message::Request::GetStatusResponse(
                            GetStatusResponse {
                                status: 0,
                                prover_name: "".to_string(),
                                prover_id: "".to_string(),
                            },
                        )),
                    }
                }
                scheduler_message::Response::GenBatchProofRequest(r) => {
                    // running
                    // TODO: parameters fo batch_prover
                    let ctx = contexts::BatchContext::new(
                        "basedir",
                        &r.execute_task_id.clone(),
                        "task_name",
                        &r.chunk_id.clone(),
                    );
                    // TODO: async prover execution and return immediately
                    // or block until prover finish?
                    provers::BatchProver::new().prove(&ctx)?;
                    // TODO: save checkpoint
                    // save_checkpoint()
                    BatchProverMessage {
                        id: "".to_string(),
                        request: None,
                    }
                }
                scheduler_message::Response::GetProofRequest(_r) => {
                    // TODO: load proof, send to scheduler
                    // get_proof
                    // load_checkpoint
                    BatchProverMessage {
                        id: "".to_string(),
                        request: Some(batch_prover_message::Request::GetProofResponse(
                            GetProofResponse {
                                id: "".to_string(),
                                recursive_proof: "".to_string(),
                                result: 0,
                                result_string: "".to_string(),
                            },
                        )),
                    }
                }
                scheduler_message::Response::CancelResponse(_r) => {
                    // TODO: remove, one task running, cannot remove
                    BatchProverMessage {
                        id: "".to_string(),
                        request: None,
                    }
                }
            };

            tx.send(send_msg).await?;
        }
    }

    Ok(())
}
