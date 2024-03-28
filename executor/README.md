# Testing

test zkvm_evm_generate_chunks
```bash
TASK=evm RUST_LOG=info cargo test --release test_zkvm_evm_generate_chunks -- --nocapture
```
or
```bash
TASK=lr RUST_LOG=info cargo test --release test_zkvm_evm_generate_chunks -- --nocapture
```
test state_merkle_trie_root
```bash
RUST_LOG=info cargo test --release test_state_merkle_trie_root -- --nocapture
```

test example exec_block
```bash
NO=1 RUST_LOG=info cargo run --example exec_block -- --nocapture
```

test example batch_process
```bash
NO=1 RUST_LOG=info cargo run --example batch_process -- --nocapture
```