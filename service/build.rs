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
            &["proto/src/proto/executor/v1/executor.proto"],
            &["proto/src/proto/executor/v1", "proto/include"],
        )?;

    tonic_build::configure().build_server(true).build_client(true).compile(
        &["proto/src/proto/prover/v1/prover.proto"],
        &["proto/src/proto/prover/v1", "proto/include"],
    )?;
    Ok(())
}
