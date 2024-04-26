use ethers_providers::{Http, Provider};
use executor::batch_process;
use std::env as stdenv;
use std::sync::Arc;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::try_init().unwrap_or_default();
    let env_block_number = stdenv::var("NO").unwrap_or(String::from("1"));
    let block_number: u64 = env_block_number.parse().unwrap();
    let task = stdenv::var("TASK").unwrap_or("lr".to_string());
    let task_id = "0";
    // base_dir is eigen_prover
    let base_dir = stdenv::var("BASEDIR").unwrap_or("/tmp".to_string());
    let url = stdenv::var("URL").unwrap_or(String::from("http://localhost:8123"));
    let chain_id = stdenv::var("CHAINID").unwrap_or(String::from("1"));
    let suite_json = stdenv::var("SUITE_JSON").unwrap_or(String::from("/tmp/suite.json"));
    let client = Provider::<Http>::try_from(url).unwrap();
    let client = Arc::new(client);
    let (_res, json_string, cnt_chunks) = batch_process(
        client,
        block_number,
        chain_id.parse::<u64>().unwrap(),
        &task,
        task_id,
        &base_dir,
    )
    .await;
    std::fs::write(suite_json, json_string).expect("Unable to write file");
    println!("Generated {} chunks", cnt_chunks);
    Ok(())
}
