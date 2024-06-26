# eigen-prover

This repo aims to build components:

* executor
* prover


## Server

### Use [grpc server](https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md)

> 💡 We provide two modes for using the Eigen-Prover.

#### 1. Single Node Mode

In this mode, all services run within a single process.
It's convenient for testing or quickly getting started with the Eigen-Prover.
```
cd service
RUST_LOG=info cargo run --release --bin service
```

#### 2. Distributed Node Mode

This mode allows the expansion of any number of remote services to provide batch-proof distributed parallel computing.
If you're aiming for production deployment or want to experience the peak performance of Eigen-Prover, this mode is recommended.

First, start the Server:
> ⚠️ Remember to note the IP and Port of your server, as they will be needed later.
```shell
cd service
PROVER_BASE=<your_prover_base_directory_path> PROVER_MODEL=grpc CACHE_DIR=${PROVER_BASE}/eigen-prover/prover/cache WORK_BASE=${PROVER_BASE}/eigen-zkvm/starkjs FORCE_BIT=18 RUSTFLAGS="-C target-cpu=native" RUST_MIN_STACK=2073741821 RUST_LOG=info CIRCOMLIB=${WORK_BASE}/node_modules/circomlib/circuits STARK_VERIFIER_GL=${WORK_BASE}/node_modules/pil-stark/circuits.gl STARK_VERIFIER_BN128=${WORK_BASE}/node_modules/pil-stark/circuits.bn128 URL=<http://zeth_ip:zeth_port> TASK_NAME=evm BASEDIR=${PROVER_BASE}/eigen-prover/prover/data/proof TASK=evm RUST_BACKTRACE=full nohup cargo run --release --bin service >> nohup.out 2>&1 &
```

Next, initiate any number of batch-proof computing nodes:
> 🚀 The speed of the Eigen-Prover depends on the number of computing nodes you initiate.
```shell
cd service
BATCH_BASE=<your_batch_prover_base_directory_path> SCHEDULER_ADDR=<http://server_ip:server_port> RUST_LOG=debug CACHE_DIR=${BATCH_BASE}/eigen-prover/prover/cache BASEDIR=${BATCH_BASE}/eigen-prover/prover/data/proof WORK_BASE="${BATCH_BASE}/eigen-zkvm/starkjs" FORCE_BIT=18 RUSTFLAGS="-C target-cpu=native" RUST_MIN_STACK=2073741821 CIRCOMLIB=${WORK_BASE}/node_modules/circomlib/circuits STARK_VERIFIER_GL=${WORK_BASE}/node_modules/pil-stark/circuits.gl STARK_VERIFIER_BN128=${WORK_BASE}/node_modules/pil-stark/circuits.bn128 nohup cargo run --bin batch-prover --release >> nohup-batch-prover.out 2>&1 &
```

### Executor Test

If you want to test the executor, you need to run a hardhat node locally, and the number of blocks is greater than or equal to the block_number in /service/examples/exec.rs

open another terminal
```
cd service
RUST_LOG=info cargo run --example exec -- --nocapture
```

You can also use `CONF_PATH` environment variable to setup config path, and make sure the config file in that is named `base_config.toml`.


## Generate the solidity verifier

Take zkEVM for instance, run the commands below. 

```bash
SUITE_JSON="/tmp/reth2.block.json" CHAINID=12345 URL=http://localhost:8546 NO=1 TASK=evm BASEDIR="prover/data/proof" RUST_LOG=debug cargo run --example batch_process -- --nocapture

export STARKJS=/zkp/eigen-zkvm/starkjs
TASK_NAME=evm SUITE_JSON="/tmp/reth2.block.json" FORCE_BIT=18 RUST_MIN_STACK=2073741821 RUST_LOG=debug \
    CIRCOMLIB=$STARKJS/node_modules/circomlib/circuits \
    STARK_VERIFIER_GL=$STARKJS/node_modules/pil-stark/circuits.gl \
    STARK_VERIFIER_BN128=$STARKJS/node_modules/pil-stark/circuits.bn128 \
    cargo test --release integration_test -- --nocapture

eigen-zkit generate_verifier -v $vk -p groth16 -s /tmp/verifier.sol
```
