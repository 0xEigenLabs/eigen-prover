use std::env as stdenv;
use executor::execute_one;
use revm::primitives::address;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_block_number = stdenv::var("NO").unwrap_or(String::from("0"));
    let block_number: u64 = env_block_number.parse().unwrap();
    let addr = address!("a94f5374fce5edbc8e2a8697c15331677e6ebf0b");
    let res = execute_one(block_number, addr, 1).await;
    Ok(())
}