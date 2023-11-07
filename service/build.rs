fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        // .out_dir("proto")
        .compile(
            &["proto/src/proto/statedb/v1/statedb.proto"],
            &["proto/src/proto/statedb/v1", "proto/include"],
        )?;
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        // .out_dir("proto")
        .compile(
            &["proto/src/proto/aggregator/v1/aggregator.proto"],
            &["proto/src/proto/aggregator/v1", "proto/include"],
        )?;
    Ok(())
}
