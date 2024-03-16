use serde::{Deserialize, Serialize};
use storage_layout_extractor as sle;
use storage_layout_extractor::{
    extractor::{
        chain::{
            version::{ChainVersion, EthereumVersion},
            Chain,
        },
        contract::Contract,
        InitialExtractor,
    },
    tc, vm,
    watchdog::DynWatchdog,
};

/// A wrapper for the parts of the JSON representation of the compiled contract
/// on disk that we care about to enable easy deserialization.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompiledContract {
    deployed_bytecode: DeployedBytecode,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeployedBytecode {
    object: String,
}

/// Constructs a new extractor to analyze the hex-encoded (with or without the
/// `0x` prefix) contract bytecode provided in `code`.
///
/// It uses the default configurations for the extractor.
#[allow(unused)] // It is actually
pub fn new_extractor_from_bytecode(
    bytecode: Vec<u8>,
    watchdog: DynWatchdog,
) -> anyhow::Result<InitialExtractor> {
    // Generally unsafe but fine for ASCII so we do it here.
    let contract = Contract::new(
        bytecode,
        Chain::Ethereum {
            version: EthereumVersion::latest(),
        },
    );

    let vm_config = vm::Config::default();
    let unifier_config = tc::Config::default();

    Ok(sle::new(contract, vm_config, unifier_config, watchdog))
}
