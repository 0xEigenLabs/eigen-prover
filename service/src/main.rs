#![allow(clippy::large_enum_variant)]
#![allow(dead_code)]

use tonic::transport::Server;
mod aggregator;
mod config;
mod statedb;

use aggregator::aggregator_service::aggregator_service_server::AggregatorServiceServer;
use statedb::statedb_service::state_db_service_server::StateDbServiceServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runtim_config = config::RuntimeConfig::from_toml("conf/base_config.toml").unwrap();
    let addr = runtim_config.addr.as_str().parse()?;
    let sdb = statedb::StateDBServiceSVC::default();
    let agg = aggregator::AggregatorServiceSVC::default();

    Server::builder()
        .add_service(StateDbServiceServer::new(sdb))
        .add_service(AggregatorServiceServer::new(agg))
        .serve(addr)
        .await?;

    Ok(())
}
