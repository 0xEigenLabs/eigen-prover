# eigen-prover

This repo aims to build components:

* executor
* statedb
* prover

## Conf
executor: 50071

statedb: 50061

prover_server listen at 50081

## StateDB

```
# create
cd statedb
bash -x install_db.sh create state root password

sudo apt install libpq-dev
export DATABASE_URL="postgresql://root:password@127.0.0.1:5432/state"

diesel migration run
cargo run --example nodes

# delete
bash -x install_db.sh delete state root
```

## Server

### Use [grpc server](https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md)

> 💡 We offer two modes for you to use the eigen-prover.

#### 1. Single node mode

All services are running in a single process.
Convenient for you to test or quickly experience eigen-prover.

```
cd service
RUST_LOG=info cargo run --bin service -- --nocapture --release
```

#### 2. Distributed node mode

Expanding any number of remote services to provide batch-proof distributed parallel computing in this mode.
If you want to use it in production or experience the ultimate performance of Engen Pro, we recommend using this mode.

first, start the Server
> ⚠️ Remember the IP and port of your server, We will use it later.
```shell
cd service
PROVER_MODEL=grpc RUST_LOG=info cargo run --bin service -- --nocapture --release
```

then, start any number of batch-proof computing nodes.
> 🚀 The speed of the eigen-prover depends on the number of computing nodes you initiate.
```shell
cd service
SCHEDULER_ADDR="http://server_ip:server_port" cargo run --bin batch-prover -- --nocapture --release
```

### Executor Test

If you want to test the executor, you need to run a hardhat node locally, and the number of blocks is greater than or equal to the block_number in /service/examples/exec.rs

open another terminal
```
cd service
RUST_LOG=info cargo run --example exec -- --nocapture
```

You can also use `CONF_PATH` environment variable to setup config path, and make sure the config file in that is named `base_config.toml`.

### Sqlite

Not support

```
sudo apt install libsqlite3-dev
cargo install diesel_cli --no-default-features --features sqlite
cargo build --release --features sqlite

export DATABASE_URL=/tmp/database.sql
diesel migration run
cargo run --example nodes
```

