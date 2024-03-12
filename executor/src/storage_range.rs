use ethers_providers::{Http, Middleware, Provider};
use std::sync::Arc;

use ethers_core::types::{GethDebugTracingOptions, H256};
pub async fn debug_storage_range_at(client: Arc<Provider<Http>>, txid: H256) {
    let options = GethDebugTracingOptions {
        disable_storage: Some(false),
        ..Default::default()
    };
    let trace_tx = client.debug_trace_transaction(txid, options).await.unwrap();
    log::trace!("trace: {:?}", trace_tx);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ethers_core::types::Address;
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

    #[tokio::test]
    async fn test_storage_range() {
        let url = "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27";
        let client = Provider::<Http>::try_from(url).unwrap();
        let client = Arc::new(client);

        let from: Address = "0xE592427A0AEce92De3Edee1F18E0157C05861564".parse().unwrap();
        //let txid = "0x37d934bac59d3bcaac491a565d8015c9b27793898ec93e653d509113c68bdceb".parse().unwrap();
        let pos = H256::from_low_u64_be(0u64);

        let storage_slots = client.get_storage_at(from, pos, None).await.unwrap();
        println!("11 {:?}", storage_slots);
    }
}
