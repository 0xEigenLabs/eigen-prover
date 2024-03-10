pub mod aggregator {
    tonic::include_proto!("aggregator.v1"); // The string specified here must match the proto package name
}
pub mod executor {
    tonic::include_proto!("executor.v1");
}

pub mod state {
    tonic::include_proto!("state.v1");
}
