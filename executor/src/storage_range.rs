use ethers_providers::{GethDebugTracingOptions, Http, Provider, Middleware};
use std::sync::Arc;

use ethers_core::types::{H256, GethDebugTracingOptions};

pub fn debug_storage_range_at(client: Arc<Provider<Http>>, txid: H256) {
    let options = GethDebugTracingOptions {
        disable_storage: Some(false),
        ..Default::default()
    };
    let trace_tx = client.debug_trace_transaction(txid, options);
    log::trace!("trace: {:?}", trace_tx);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_trace_tx() {
        let url = "http://localhost:8545";
        let txid = "0xc265f68f4e5a4f83d2120cc5084c955b10241f26bb108a1857cdcdf0fce4f2da";
        let client = Provider::<Http>::try_from(url).unwrap();
        let client = Arc::new(client);
        debug_storage_range_at(client, txid);
    }
}
