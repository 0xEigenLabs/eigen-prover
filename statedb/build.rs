fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        // .out_dir("proto")
        .compile(
            &["../service/proto/src/proto/statedb/v1/statedb.proto"],
            &["../service/proto/src/proto/statedb/v1", "../service/proto/include"],
        )?;
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        // .out_dir("proto")
        .compile(
            &["../service/proto/src/proto/aggregator/v1/aggregator.proto"],
            &["../service/proto/src/proto/aggregator/v1", "../service/proto/include"],
        )?;
    Ok(())
}
