use tonic::transport::Server;
pub mod aggregator;
pub mod statedb;
use aggregator::aggregator_service::aggregator_service_server::{
    AggregatorServiceServer,
};
use statedb::statedb_service::state_db_service_server::{StateDbServiceServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let sdb = statedb::StateDBServiceSVC::default();
    let agg = aggregator::AggregatorServiceSVC::default();

    Server::builder()
        .add_service(StateDbServiceServer::new(sdb))
        .add_service(AggregatorServiceServer::new(agg))
        .serve(addr)
        .await?;

    Ok(())
}
