WORK_BASE="/data/stephen/eigen-zkvm/starkjs"
#RUSTFLAGS='-C target-feature=+avx512f,+avx512bw,+avx512cd,+avx512dq,+avx512vl'
RUSTFLAGS='-C target-cpu=native'
FORCE_BIT=18 RUSTFLAGS=$RUSTFLAGS SUITE_JSON=/tmp/reth.block.json RUST_MIN_STACK=2073741821 RUST_BACKTRACE=1 RUST_LOG=debug CIRCOMLIB=${WORK_BASE}/node_modules/circomlib/circuits STARK_VERIFIER_GL=$WORK_BASE/node_modules/pil-stark/circuits.gl STARK_VERIFIER_BN128=$WORK_BASE/node_modules/pil-stark/circuits.bn128 cargo test --release integration_test -- --nocapture
