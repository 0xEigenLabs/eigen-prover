use prover_scheduler::service::batch_prover_service::launch_batch_prover_service_with_addr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "http://[::1]:8545".to_string();
    launch_batch_prover_service_with_addr(addr).await
}
