# eigen-prover

This repo aims to build components:

* executor
* statedb
* prover

## Conf
executor: 50071
statedb: 50061
prover_server listen at 50081

## Server

Use [grpc server](https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md)

```
cd service
DATABASE_URL="postgresql://prover_user:prover_pass@127.0.0.1:5432/prover_db" \
CIRCOMLIB=../executor/node_modules/circomlib/circuits \
STARK_VERIFIER_GL=../executor/node_modules/pil-stark/circuits.gl \
STARK_VERIFIER_BN128=../executor/node_modules/pil-stark/circuits.bn128  \
PROVER_FORK_ID=4  NODE_ADDR=http://127.0.0.1:50081 TASK_NAME=XXXXX RUST_LOG=debug cargo run --release
```
Change the TASK\_NAME to your task name.

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

