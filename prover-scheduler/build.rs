fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        // .out_dir("proto")
        .compile(
            &["proto/src/proto/scheduler/v1/scheduler.proto"],
            &["proto/src/proto/scheduler/v1", "proto/include"],
        )?;
    println!("cargo:rustc-env=EVM_ELF_DATA=target/elf-compilation/riscv32im-succinct-zkvm-elf/release/evm");
    Ok(())
}
