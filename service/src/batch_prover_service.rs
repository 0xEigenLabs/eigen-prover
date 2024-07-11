use prover_scheduler::service::batch_prover_service::{
    BatchProverService, BatchProverServiceHandler,
};
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::try_init().unwrap_or_default();

    let prometheus_port = std::env::var("PROMETHEUS_ADDR").unwrap_or("0.0.0.0:3003".to_string());
    let prometheus_addr: SocketAddr = prometheus_port.parse().expect("Invalid socket address");

    tokio::spawn(async move { metrics::launch_prometheus(prometheus_addr).await });

    let addr = std::env::var("SCHEDULER_ADDR").unwrap_or("http://127.0.0.1:50051".to_string());
    let batch_prover_handler = Arc::new(BatchProverServiceHandler::default());
    let batch_prover_service = BatchProverService::new(addr, batch_prover_handler);
    batch_prover_service.launch_service().await
}
