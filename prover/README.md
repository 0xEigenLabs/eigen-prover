# Prover
The work space is organized as below.
```
basedir/
      _/executor/{task_id}/status
                         _/fib.pil.json
                         _/fib.cm
                         _/fib.const
      _/proof/{task_id}/status
                      _/status.finished
                      _/batch_proof/
                                  _/fib.circom
                                  _/fib.zkin.json
                                  _/fib.r1cs
                                  _/fib_js/fib.wasm
                                  _/fib.pil
                                  _/fib.pil.json
                                  _/fib.exec
                                  _/fib.cm
                                  _/fib.const
                      _/agg_proof/
                                _/fib.recursive1.circom
                                _/fib.recursive1.zkin.json
                                _/fib.recursive1.r1cs
                                _/fib.recursive1_js/fib.recursive1.wasm
                                _/fib.recursive1.cm
                                _/fib.recursive1.const
                                _/fib.recursive1.exec
                                _/fib.recursive1.pil
                                _/fib.recursive1.pil.json

                      _/final_proof/
                                _/fib.recursive2.circom
                                _/fib.recursive2.zkin.json
                                _/fib.recursive2.r1cs
                                _/fib.recursive2_js/fib.recursive2.wasm
                                _/fib.recursive2.cm
                                _/fib.recursive2.const
                                _/fib.recursive2.exec
                                _/fib.recursive2.pil
                                _/fib.recursive2.pil.json
                                _/g16.zkey
                                _/verification_key.json
                                _/proof.json
                                _/public_input.json

```

## Testing

```bash
RUST_BACKTRACE=1 RUST_LOG=info \
CIRCOMLIB=../executor/node_modules/circomlib/circuits \
STARK_VERIFIER_GL=../executor/node_modules/pil-stark/circuits.gl \
STARK_VERIFIER_BN128=../executor/node_modules/pil-stark/circuits.bn128  \
cargo test --release -- --nocapture
```
