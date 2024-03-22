use ethers_core::types::{Block, Transaction, H160, H256, U256, U64};
use ethers_core::utils::hex;
use ethers_core::utils::rlp::{Decodable, Rlp};
use ethers_providers::{Http, Provider};
use executor::batch_process;
use std::env as stdenv;
use std::sync::Arc;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::try_init().unwrap_or_default();
    let l2_batch_data = String::from("f86a3a0882520894617b3a3528f9cdd6630fd3301b9c8911f7bf063d890363408b48148fc03780820a95a06a9801037bd94ab48f3fc383d605d29c9f6407be3d26c7bc70108c609cabb2f7a052a067f0abd92441b40380997a76056bc00f274c372c0102d50244544f42d2b9");
    let l2_batch_data = hex::decode(&l2_batch_data).unwrap();
    let raw_tx = Rlp::new(&l2_batch_data);
    let mut tx = Transaction::decode(&raw_tx).unwrap();
    let author: Option<H160> = match hex::decode("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266") {
        Ok(x) => Some(H160::from_slice(&x)),
        _ => None,
    };

    tx.from = match hex::decode("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266") {
        Ok(x) => H160::from_slice(&x),
        _ => panic!("Invalid from address"),
    };
    println!("tx: {:?}", tx);
    let block = Block {
        author,
        number: Some(U64::from(0)),
        transactions: vec![tx.clone()],
        timestamp: U256::from(0),
        gas_limit: U256::from(80_000_000u128),
        parent_hash: H256::default(),
        gas_used: U256::from(0),
        difficulty: U256::from(0),
        ..Default::default()
    };

    let task = "lr";
    let task_id = "0";
    // base_dir is eigen_prover
    let base_dir = "/tmp";
    let url = stdenv::var("URL").unwrap_or(String::from("http://localhost:8123"));
    let client = Provider::<Http>::try_from(url).unwrap();
    let client = Arc::new(client);
    let (_res, cnt_chunks) = batch_process(
        client,
        tx.chain_id.unwrap().as_u64(),
        block,
        task,
        task_id,
        base_dir,
    )
    .await;
    println!("Generated {} chunks", cnt_chunks);
    Ok(())
}
