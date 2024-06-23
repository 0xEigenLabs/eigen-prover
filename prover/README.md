# Remote Prover

Remove Proving allows developers to submit their proof generation request to Eigen Proving Service.

## Testing
TASK=evm
```bash
export STARKJS=/zkp/eigen-zkvm/starkjs
FORCE_BIT=18 RUST_MIN_STACK=2073741821 RUST_BACKTRACE=1 RUST_LOG=debug \
    CIRCOMLIB=$STARKJS/node_modules/circomlib/circuits \
    STARK_VERIFIER_GL=$STARKJS/node_modules/pil-stark/circuits.gl \
    STARK_VERIFIER_BN128=$STARKJS/node_modules/pil-stark/circuits.bn128 \
    cargo test --release integration_test -- --nocapture
```
TASK=lr
```bash
export STARKJS=/zkp/eigen-zkvm/starkjs
FORCE_BIT=18 RUST_MIN_STACK=2073741821 RUST_BACKTRACE=1 RUST_LOG=debug \
    CIRCOMLIB=$STARKJS/node_modules/circomlib/circuits \
    STARK_VERIFIER_GL=$STARKJS/node_modules/pil-stark/circuits.gl \
    STARK_VERIFIER_BN128=$STARKJS/node_modules/pil-stark/circuits.bn128 \
    cargo test --release integration_test_lr -- --nocapture
```
Note that the `FORCE_BIT` can be adjusted as per to different circuits.
Taking Fibonacci as an example, the recursive proof process is shown in the figure below.

If you intend to enable the avx acceleration:

```
# for avx512
RUSTFLAGS='-C target-cpu=native' FORCE_BIT=18 RUST_MIN_STACK=2073741821 RUST_BACKTRACE=1 RUST_LOG=debug \
    CIRCOMLIB=$STARKJS/node_modules/circomlib/circuits \
    STARK_VERIFIER_GL=$STARKJS/node_modules/pil-stark/circuits.gl \
    STARK_VERIFIER_BN128=$STARKJS/node_modules/pil-stark/circuits.bn128 \
    cargo test --release integration_test --features avx512

# for avx2 only (without avx512)
RUSTFLAGS='-C target-cpu=native' FORCE_BIT=18 RUST_MIN_STACK=2073741821 RUST_BACKTRACE=1 RUST_LOG=debug \
    CIRCOMLIB=$STARKJS/node_modules/circomlib/circuits \
    STARK_VERIFIER_GL=$STARKJS/node_modules/pil-stark/circuits.gl \
    STARK_VERIFIER_BN128=$STARKJS/node_modules/pil-stark/circuits.bn128 \
    cargo test --release integration_test
```

![proving-architecture](../docs/proving-architecture.png)
