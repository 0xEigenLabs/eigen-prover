use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use lazy_static::lazy_static;
use prometheus::{
    Encoder, Error as PrometheusError, GaugeVec, HistogramVec, Registry, TextEncoder,
};
use std::net::SocketAddr;
use std::sync::Mutex;

lazy_static! {
    pub static ref REGISTRY_INSTANCE: Registry = Registry::new();
    pub static ref PROMETHEUS_METRICS: Mutex<PrometheusMetrics> =
        Mutex::new(PrometheusMetrics::init().expect("Failed to initialize PrometheusMetrics"));
}

#[derive(Debug, Clone)]
pub struct PrometheusMetrics {
    pub prover_processing_time_gauge: GaugeVec,
    pub prover_processing_time_histogram: HistogramVec,
}

pub mod batch_processing_time_gauge_labels {
    pub const LABELS_BLOCK_NUMBER: &str = "block_number";
    pub const LABELS_CHUNK_ID: &str = "chunk_id";
    pub const LABELS_STEP: &str = "step";
    pub const LABELS_FUNCTION: &str = "function";
}

pub mod batch_processing_time_histogram_labels {
    pub const LABELS_BLOCK_NUMBER: &str = "block_number";
    pub const LABELS_CHUNK_ID: &str = "chunk_id";
    pub const LABELS_STEP: &str = "step";
    pub const LABELS_STEP_BATCH_STARK: &str = "step_batch_stark";
    pub const LABELS_STEP_BATCH_C12_STARK: &str = "step_batch_c12_stark";
    pub const LABELS_STEP_AGG: &str = "step_agg";
    pub const LABELS_STEP_FINAL: &str = "step_final";
    pub const LABELS_FUNCTION: &str = "function";
    pub const LABELS_FUNCTION_SETUP: &str = "function_setup";
    pub const LABELS_FUNCTION_EXEC: &str = "function_exec";
    pub const LABELS_FUNCTION_STARK_PROVE: &str = "function_stark_prove";
    pub const LABELS_FUNCTION_TOTAL: &str = "function_total";
}

pub enum BatchProcessingTimeHistogramLabels {
    BlockNumber,
    ChunkId,
    Step(Step),
    Function(Function),
}

pub enum Step {
    Batch(Batch),
    Agg,
    Final,
}

pub enum Batch {
    BatchStark,
    C12Stark,
}

impl From<Step> for String {
    fn from(value: Step) -> Self {
        match value {
            Step::Batch(value) => match value {
                Batch::BatchStark => {
                    batch_processing_time_histogram_labels::LABELS_STEP_BATCH_STARK.to_string()
                }
                Batch::C12Stark => {
                    batch_processing_time_histogram_labels::LABELS_STEP_BATCH_STARK.to_string()
                }
            },
            Step::Agg => batch_processing_time_histogram_labels::LABELS_STEP_AGG.to_string(),
            Step::Final => batch_processing_time_histogram_labels::LABELS_STEP_FINAL.to_string(),
        }
    }
}

pub enum Function {
    Setup,
    Exec,
    StarkProve,
    Total,
}

impl From<Function> for String {
    fn from(value: Function) -> Self {
        match value {
            Function::Setup => {
                batch_processing_time_histogram_labels::LABELS_FUNCTION_SETUP.to_string()
            }
            Function::Exec => {
                batch_processing_time_histogram_labels::LABELS_FUNCTION_EXEC.to_string()
            }
            Function::StarkProve => {
                batch_processing_time_histogram_labels::LABELS_FUNCTION_STARK_PROVE.to_string()
            }
            Function::Total => {
                batch_processing_time_histogram_labels::LABELS_FUNCTION_TOTAL.to_string()
            }
        }
    }
}

// TODO: add optional labels
#[allow(dead_code)]
impl PrometheusMetrics {
    pub fn init() -> Result<Self, PrometheusError> {
        let prover_processing_time_gauge = GaugeVec::new(
            prometheus::Opts::new(
                "prover_processing_time_gauge",
                "Prover processing time in seconds",
            )
            .namespace("eigen_prover")
            .const_label("type", "gauge"),
            &[
                batch_processing_time_gauge_labels::LABELS_STEP,
                batch_processing_time_gauge_labels::LABELS_FUNCTION,
            ],
        )
        .map_err(|e| PrometheusError::Msg(e.to_string()))?;

        let prover_processing_time_histogram = HistogramVec::new(
            prometheus::HistogramOpts::new(
                "prover_processing_time_histogram",
                "Prover processing time in seconds",
            )
            .namespace("eigen_prover")
            .const_label("type", "histogram"),
            &[
                batch_processing_time_histogram_labels::LABELS_STEP,
                batch_processing_time_histogram_labels::LABELS_FUNCTION,
            ],
        )
        .map_err(|e| PrometheusError::Msg(e.to_string()))?;

        REGISTRY_INSTANCE
            .register(Box::new(prover_processing_time_gauge.clone()))
            .map_err(|e| PrometheusError::Msg(e.to_string()))?;
        REGISTRY_INSTANCE
            .register(Box::new(prover_processing_time_histogram.clone()))
            .map_err(|e| PrometheusError::Msg(e.to_string()))?;

        Ok(Self {
            prover_processing_time_gauge,
            prover_processing_time_histogram,
        })
    }

    pub fn observe_prover_processing_time_gauge(&self, step: Step, function: Function, value: f64) {
        let step_string: String = step.into();
        let function_string: String = function.into();
        self.prover_processing_time_gauge
            .with_label_values(&[step_string.as_str(), function_string.as_str()])
            .set(value);
    }

    pub fn _observe_prover_processing_time_histogram(
        &self,
        step: Step,
        function: Function,
        value: f64,
    ) {
        let step_string: String = step.into();
        let function_string: String = function.into();
        self.prover_processing_time_histogram
            .with_label_values(&[step_string.as_str(), function_string.as_str()])
            .observe(value);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Hyper internal error.
    #[error(transparent)]
    Hyper(#[from] hyper::Error),

    /// Http request error.
    #[error(transparent)]
    Http(#[from] hyper::http::Error),

    #[error("Prometheus port {0} already in use.")]
    PortInUse(SocketAddr),
}

async fn request_metrics(req: Request<Body>, registry: Registry) -> Result<Response<Body>, Error> {
    if req.uri().path() == "/metrics" {
        let metric_families = registry.gather();
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();

        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", encoder.format_type())
            .body(Body::from(buffer))
            .map_err(Error::Http)
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not found."))
            .map_err(Error::Http)
    }
}

pub async fn launch_prometheus(prometheus_addr: SocketAddr) -> Result<(), Error> {
    let listener = tokio::net::TcpListener::bind(&prometheus_addr)
        .await
        .map_err(|_| Error::PortInUse(prometheus_addr))?;

    init_prometheus_with_listener(listener).await
}

async fn init_prometheus_with_listener(listener: tokio::net::TcpListener) -> Result<(), Error> {
    let listener = hyper::server::conn::AddrIncoming::from_listener(listener)?;
    log::info!("Prometheus exporter started at {}", listener.local_addr());

    let service = make_service_fn(move |_| {
        let registry = REGISTRY_INSTANCE.clone();

        async move {
            Ok::<_, hyper::Error>(service_fn(move |req: Request<Body>| {
                request_metrics(req, registry.clone())
            }))
        }
    });

    let server = Server::builder(listener).serve(service);

    server.await.map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::body::to_bytes;
    use std::str;
    use std::time::Duration;

    #[tokio::test]
    async fn prometheus_work() {
        // let registry = Registry::new();
        // let prometheus_metrics = PrometheusMetrics::init().unwrap();

        PROMETHEUS_METRICS
            .lock()
            .unwrap()
            .observe_prover_processing_time_gauge(
                Step::Batch(Batch::BatchStark),
                Function::Setup,
                1.0,
            );
        PROMETHEUS_METRICS
            .lock()
            .unwrap()
            ._observe_prover_processing_time_histogram(
                Step::Batch(Batch::BatchStark),
                Function::Setup,
                1.0,
            );

        let prometheus_port =
            std::env::var("PROMETHEUS_ADDR").unwrap_or("0.0.0.0:43032".to_string());
        let prometheus_addr: SocketAddr = prometheus_port.parse().expect("Invalid socket address");
        // let prometheus_addr = ([127, 0, 0, 1], 33032).into();
        tokio::spawn(async move {
            launch_prometheus(prometheus_addr)
                .await
                .expect("TODO: panic message");
        });
        tokio::time::sleep(Duration::from_secs(2)).await;

        let client = hyper::Client::new();
        let uri = format!(
            "http://{}:{}/metrics",
            prometheus_addr.ip(),
            prometheus_addr.port()
        )
        .parse()
        .unwrap();
        let res = client.get(uri).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        // Parse the body content
        let body_bytes = to_bytes(res.into_body()).await.unwrap();
        let body_content = str::from_utf8(&body_bytes).unwrap();

        println!("Response body: {}", body_content);
        // Response body: # HELP eigen_prover_prover_processing_time_gauge Prover processing time in seconds
        // # TYPE eigen_prover_prover_processing_time_gauge gauge
        // eigen_prover_prover_processing_time_gauge{function="function_setup",step="step_batch_stark",type="gauge"} 1
        // # HELP eigen_prover_prover_processing_time_histogram Prover processing time in seconds
        // # TYPE eigen_prover_prover_processing_time_histogram histogram
        // eigen_prover_prover_processing_time_histogram_bucket{function="function_setup",step="step_batch_stark",type="histogram",le="0.005"} 0
        // eigen_prover_prover_processing_time_histogram_bucket{function="function_setup",step="step_batch_stark",type="histogram",le="0.01"} 0
        // eigen_prover_prover_processing_time_histogram_bucket{function="function_setup",step="step_batch_stark",type="histogram",le="0.025"} 0
        // eigen_prover_prover_processing_time_histogram_bucket{function="function_setup",step="step_batch_stark",type="histogram",le="0.05"} 0
        // eigen_prover_prover_processing_time_histogram_bucket{function="function_setup",step="step_batch_stark",type="histogram",le="0.1"} 0
        // eigen_prover_prover_processing_time_histogram_bucket{function="function_setup",step="step_batch_stark",type="histogram",le="0.25"} 0
        // eigen_prover_prover_processing_time_histogram_bucket{function="function_setup",step="step_batch_stark",type="histogram",le="0.5"} 0
        // eigen_prover_prover_processing_time_histogram_bucket{function="function_setup",step="step_batch_stark",type="histogram",le="1"} 1
        // eigen_prover_prover_processing_time_histogram_bucket{function="function_setup",step="step_batch_stark",type="histogram",le="2.5"} 1
        // eigen_prover_prover_processing_time_histogram_bucket{function="function_setup",step="step_batch_stark",type="histogram",le="5"} 1
        // eigen_prover_prover_processing_time_histogram_bucket{function="function_setup",step="step_batch_stark",type="histogram",le="10"} 1
        // eigen_prover_prover_processing_time_histogram_bucket{function="function_setup",step="step_batch_stark",type="histogram",le="+Inf"} 1
        // eigen_prover_prover_processing_time_histogram_sum{function="function_setup",step="step_batch_stark",type="histogram"} 1
        // eigen_prover_prover_processing_time_histogram_count{function="function_setup",step="step_batch_stark",type="histogram"} 1

        // eigen_prover_prover_processing_time_gauge{function="function_setup",step="step_batch_stark",type="gauge"} 1
        let expect_gauge = "eigen_prover_prover_processing_time_gauge{function=\"function_setup\",step=\"step_batch_stark\",type=\"gauge\"} 1";
        assert!(body_content.contains(expect_gauge));

        // eigen_prover_prover_processing_time_histogram_sum{function="function_setup",step="step_batch_stark",type="histogram"} 1
        let expect_histogram_sum = "eigen_prover_prover_processing_time_histogram_sum{function=\"function_setup\",step=\"step_batch_stark\",type=\"histogram\"} 1";
        assert!(body_content.contains(expect_histogram_sum));
    }
}
