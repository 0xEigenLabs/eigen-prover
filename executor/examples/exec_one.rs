use executor::execute_one;
use std::env as stdenv;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_block_number = stdenv::var("NO").unwrap_or(String::from("0"));
    let block_number: u64 = env_block_number.parse().unwrap();
    let _res = execute_one(block_number, 1, "/tmp/storage".to_string()).await;
    Ok(())
}
