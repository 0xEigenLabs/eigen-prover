fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        // .out_dir("proto")
        .compile(
            &["proto/src/proto/scheduler/v1/scheduler.proto"],
            &["proto/src/proto/scheduler/v1", "proto/include"],
        )?;
    Ok(())
}
