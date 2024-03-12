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
    #[tokio::test]
    async fn test_trace_tx() {
        let url = "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27";
        let client = Provider::<Http>::try_from(url).unwrap();
        let client = Arc::new(client);

        let txid = "0x215d5a2aa47abaa65f4a81c8b38b5157dee66608a9442c82516925a3b6bb0d84"
            .parse()
            .unwrap();
        debug_storage_range_at(client, txid);
    }
}
