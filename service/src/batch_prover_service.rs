use prover_scheduler::service::batch_prover_service::{
    BatchProverService, BatchProverServiceHandler,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::var("SCHEDULER_ADDR").unwrap_or("http://localhost:50061".to_string());
    let batch_prover_handler = Arc::new(BatchProverServiceHandler::default());
    let batch_prover_service = BatchProverService::new(addr, batch_prover_handler);
    batch_prover_service.launch_service().await
}
