#![allow(clippy::large_enum_variant)]
#![allow(dead_code)]

use std::sync::Arc;
use tonic::transport::Server;
mod batch_prover_service;
mod config;
mod executor_service;

mod prover_service;

#[macro_use]
extern crate lazy_static;
use crate::prover_service::prover_service::prover_service_server::ProverServiceServer;
use crate::prover_service::ProverServiceSVC;
use executor_service::executor_service::executor_service_server::ExecutorServiceServer;
use prover::scheduler::Scheduler;
use prover_scheduler::scheduler_server::scheduler_service::scheduler_service_server::SchedulerServiceServer;
use prover_scheduler::scheduler_server::{SchedulerServerHandler, SchedulerServiceSVC};
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
    let conf_path = std::env::var("CONF_DIR").unwrap_or("conf".to_string());
    let executor_base_dir =
        std::env::var("BASEDIR").unwrap_or(String::from("/tmp/prover/data/proof"));
    let conf_path = std::path::Path::new(&conf_path).join("base_config.toml");
    let runtime_config = config::RuntimeConfig::from_toml(conf_path).expect("Config is missing");
    let addr = runtime_config.addr.as_str().parse()?;

    let executor = executor_service::ExecutorServiceSVC::new();

    log::info!("Launching sigterm handler");
    let (signal_tx, signal_rx) = oneshot::channel();
    let mut interval = time::interval(time::Duration::from_secs(1));
    let (send, mut recv) = watch::channel::<()>(());
    spawn(wait_for_sigterm(signal_tx, send));

    let (task_tx, task_rx) = tokio::sync::mpsc::channel(128);
    let (event_tx, event_rx) = tokio::sync::mpsc::channel(128);
    let (result_tx, result_rx) = tokio::sync::mpsc::channel(128);
    let task_tx_clone = task_tx.clone();

    // scheduler holds the event_rx, task_rx, task_tx
    //   - event_rx: receive the event from the SchedulerServiceSVC
    //   - task_rx: receive the task from the Pipeline
    //   - task_tx: used to retry tasks by itself
    let mut scheduler = Scheduler::new(result_rx, event_rx, task_rx, task_tx_clone);
    tokio::spawn(async move {
        // TODO: quit signal
        scheduler.run().await;
    });

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    // pipeline holds the task_tx to send tasks to the scheduler
                    match prover_service::run_prover(task_tx.clone()).await {
                        Ok(_) => {
                            log::debug!("prove one task");
                        }
                        Err(e) => {
                            log::warn!("Task error: {:?}", e)
                        }
                    };
                },
                _ = recv.changed() => {
                    log::info!("finished, break the prover loop, call it a day");
                    break;
                }
            }
        }

        log::info!("finished in the walking task");
    });

    log::info!("Executor service Listening on {}", addr);

    log::info!("Prover service Listening on {}", addr);

    log::info!("BatchProverScheduler service Listening on {}", addr);
    let prover_request_handler =
        Arc::new(prover_service::ProverRequestHandler::new(executor_base_dir));
    let prover_server = ProverServiceSVC::new(prover_request_handler);

    // SchedulerServiceSVC holds the event_tx
    // all client will connect to this instance
    // they will send events to the scheduler by the event_tx, such as AddService, TakeTask etc.
    let scheduler_handler = Arc::new(SchedulerServerHandler::default());
    let scheduler_server = SchedulerServiceSVC::new(event_tx, result_tx, scheduler_handler);
    Server::builder()
        .add_service(ExecutorServiceServer::new(executor))
        .add_service(SchedulerServiceServer::new(scheduler_server))
        .add_service(ProverServiceServer::new(prover_server))
        .serve_with_shutdown(addr, async {
            signal_rx.await.ok();
            log::info!("Graceful context shutdown");
        })
        .await?;
    Ok(())
}

async fn wait_for_sigterm(tx: Sender<()>, send: watch::Sender<()>) {
    // close prover, NOTE: should use terminate?
    let _ = signal(SignalKind::interrupt())
        .expect("failed to install signal handler")
        .recv()
        .await;
    let _ = tx.send(());
    let _ = send.send(());
    log::info!("SIGTERM received: shutting down");
}
