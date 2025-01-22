# Test

## batch prove

```bash
cd batch_prover/script
RUST_LOG=info cargo test test_sp1_prove --release -- --nocapture
```

## aggregator prove

```bash
cd agg_prover/script
RUST_LOG=info cargo test test_agg_prove --release -- --nocapture
```

## ark groth16 verify
```bash
cd agg_prover/script
cargo test test_ark_groth16 --features "ark" --release -- --nocapture
```