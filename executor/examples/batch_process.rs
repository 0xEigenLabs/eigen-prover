use ethers_providers::{Http, Provider};
use executor::batch_process;
use std::env as stdenv;
use std::sync::Arc;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::try_init().unwrap_or_default();
    let env_block_number = stdenv::var("NO").unwrap_or(String::from("1"));
    let block_number: u64 = env_block_number.parse().unwrap();
    // base_dir is eigen_prover
    let url = stdenv::var("URL").unwrap_or(String::from("http://localhost:8123"));
    let chain_id = stdenv::var("CHAINID").unwrap_or(String::from("1"));
    let suite_json = stdenv::var("SUITE_JSON").unwrap_or(String::from("/tmp/suite.json"));
    let client = Provider::<Http>::try_from(url).unwrap();
    let client = Arc::new(client);
    let (_res, json_string) =
        batch_process(client, block_number, chain_id.parse::<u64>().unwrap()).await;
    std::fs::write(suite_json, json_string).expect("Unable to write file");
    Ok(())
}
