# Remote Prover

<font color="red">Note: Eigen Prover is sunset, use SP1 prover</font>

Remove Proving allows developers to submit their proof generation request to Eigen Proving Service.

## Testing
```bash
PROVER_TYPE=sp1 RUST_LOG=debug cargo test -r integration_sp1_test
```

![proving-architecture](../docs/proving-architecture.png)
