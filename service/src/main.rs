#![allow(clippy::large_enum_variant)]
#![allow(dead_code)]

use tonic::transport::Server;
mod aggregator;
mod config;
mod statedb;

#[macro_use]
extern crate lazy_static;

use aggregator::aggregator_service::aggregator_service_server::AggregatorServiceServer;
use statedb::statedb_service::state_db_service_server::StateDbServiceServer;
use tokio::{
    signal::unix::{signal, SignalKind},
    spawn,
    sync::oneshot::{self, Sender},
    sync::watch,
    time,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let runtim_config = config::RuntimeConfig::from_toml("conf/base_config.toml").unwrap();
    let addr = runtim_config.addr.as_str().parse()?;
    let sdb = statedb::StateDBServiceSVC::default();
    let agg = aggregator::AggregatorServiceSVC::default();

    log::info!("Launching sigterm handler");
    let (signal_tx, signal_rx) = oneshot::channel();

    let mut interval = time::interval(time::Duration::from_secs(1));
    let (send, mut recv) = watch::channel::<()>(());
    spawn(wait_for_sigterm(signal_tx, send));

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    match aggregator::prove() {
                        Ok(_) => {
                            log::debug!("prove one task");
                        }
                        Err(e) => {
                            log::warn!("Task error: {:?}", e)
                        }
                    };
                },
                _ = recv.changed() => {
                    log::info!("finished, break the loop, call it a day");
                    break;
                }
            }
        }

        log::info!("finished in the walking task");
    });

    log::info!("Listening on {}", addr);
    Server::builder()
        .add_service(StateDbServiceServer::new(sdb))
        .add_service(AggregatorServiceServer::new(agg))
        .serve_with_shutdown(addr, async {
            signal_rx.await.ok();
            log::info!("Graceful context shutdown");
        })
        .await?;

    Ok(())
}

async fn wait_for_sigterm(tx: Sender<()>, send: tokio::sync::watch::Sender<()>) {
    // close prover, NOTE: should use terminate?
    let _ = signal(SignalKind::interrupt())
        .expect("failed to install signal handler")
        .recv()
        .await;
    let _ = tx.send(());
    let _ = send.send(());
    log::info!("SIGTERM received: shutting down");
}
