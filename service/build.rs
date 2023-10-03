fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/src/proto/statedb/v1/statedb.proto")?;
    tonic_build::compile_protos("proto/src/proto/aggregator/v1/aggregator.proto")?;
    Ok(())
}
