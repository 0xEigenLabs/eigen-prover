use executor::execute_one;
use std::env as stdenv;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_block_number = stdenv::var("NO").unwrap_or(String::from("0"));
    let block_number: u64 = env_block_number.parse().unwrap();
    let slot_path = stdenv::var("SLOT").unwrap_or(String::from("/tmp/storage"));
    let _res = execute_one(block_number, 1, slot_path.as_str()).await;
    Ok(())
}
