# eigen-prover

The whole process to produce a zkVM proof refers to [SM](https://github.com/0xEigenLabs/eigen-zkvm/tree/main/SM#sm).

This repo aims to build components as below.

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
bash -x statedb/create_db.sh create state root password

export DATABASE_URL="postgresql://root:password@127.0.0.1:5432/state"
sudo apt install libpq-dev
cargo run --example nodes

# delete
bash -x create_db.sh delete state root
```
