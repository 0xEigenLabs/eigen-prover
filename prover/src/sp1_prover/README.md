# Test

## batch prove

```bash
RUST_LOG=info cargo test test_sp1_prove --release -- --nocapture
```

## aggregator prove

```bash
RUST_LOG=info cargo test test_sp1_agg_prove --release -- --nocapture
```

## ark groth16 verify
```bash
RUST_LOG=info cargo test test_sp1_final_prove --release -- --nocapture
```