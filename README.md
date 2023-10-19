# eigen-prover

This repo aims to build components:

* executor
* statedb
* prover

## Server

Use [grpc server](https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md)

```
cargo run
# run in another terminal
cargo run --example set
```

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

