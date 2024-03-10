use ethers_providers::{Http, Provider};
use executor::batch_process;
use state::database::{Database, DEFAULT_ROOT_KEY};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::try_init().unwrap_or_default();
    let env_block_number = std::env::var("NO").unwrap_or(String::from("1"));
    let block_number: u64 = env_block_number.parse().unwrap();
    let task = "lr";
    let task_id = "0";
    // base_dir is eigen_prover
    let base_dir = "/tmp";
    let chain_id = std::env::var("CHAINID").unwrap_or(String::from("1"));

    // Create a new state database connection pool
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let root_key = std::env::var("ROOT_KEY").unwrap_or(DEFAULT_ROOT_KEY.to_string());
    let db = Arc::new(Database::new(&url, &root_key).await);

    // Create a new Ethereum [`Provider`] HTTP client with the given URL.
    let url = std::env::var("ETH_RPC_ENDPOINT").unwrap_or(String::from("http://localhost:8545"));
    let client = Provider::<Http>::try_from(url)
        .expect("Could not instantiate HTTP Provider to Ethereum JSON RPC API");
    let client = Arc::new(client);

    let (_res, cnt_chunks) = batch_process(
        &client,
        &db,
        block_number,
        chain_id.parse::<u64>().unwrap(),
        task,
        task_id,
        base_dir,
    )
    .await;
    println!("Generated {} chunks", cnt_chunks);
    Ok(())
}
