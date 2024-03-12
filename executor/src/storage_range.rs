use ethers_providers::{Http, Middleware, Provider};
use futures::future::join_all;
use std::collections::HashMap;
use std::sync::Arc;

use ethers_core::types::{Address, BlockId, GethDebugTracingOptions, H256, U256};
/*
// It's not all node support debug_traceTransaction
pub async fn debug_storage_range_at(client: Arc<Provider<Http>>, txid: H256) {
    let options = GethDebugTracingOptions {
        disable_storage: Some(false),
        ..Default::default()
    };
    let trace_tx = client.debug_trace_transaction(txid, options).await.unwrap();
    log::trace!("trace: {:?}", trace_tx);
}
*/
pub fn get_storage(
    client: Arc<Provider<Http>>,
    address: Address,
    block_id: Option<BlockId>,
    max_slot_num: u64,
) -> HashMap<H256, H256> {
    let max_slot_num = if max_slot_num == 0 { 64 } else { max_slot_num };
    let batch: Vec<_> = (0..max_slot_num)
        .into_iter()
        .map(|i| {
            let pos = H256::from_low_u64_be(i);
            let f = async {
                let slot = client.get_storage_at(address, pos, block_id);
                tokio::join!(slot)
            };
            let result = futures::executor::block_on(f);
            result.0.unwrap()
        })
        .collect();
    (0..max_slot_num)
        .into_iter()
        .zip(batch.iter())
        .map(|(k, v)| (H256::from_low_u64_be(k), *v))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    /*
    #[tokio::test]
    async fn test_trace_tx() {
        let url = "https://polygon.llamarpc.com";
        let client = Provider::<Http>::try_from(url).unwrap();
        let client = Arc::new(client);

        let txid = "0xbb73f9712191cd05b995a3e0068e3e3aae0d2a598c8031e07c6518ec20f3543d"
            .parse()
            .unwrap();
        debug_storage_range_at(client, txid).await;
    }
    */

    #[tokio::test]
    async fn test_storage_range() {
        let url = "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27";
        let client = Provider::<Http>::try_from(url).unwrap();
        let client = Arc::new(client);

        let from: Address = "0xE592427A0AEce92De3Edee1F18E0157C05861564"
            .parse()
            .unwrap();
        let res = get_storage(client, from, None, 2);
        println!("get storage: {:?}", res);
    }
}
