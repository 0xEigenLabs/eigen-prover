# Testing

test zkvm_evm_generate_chunks
```
RUST_LOG=info cargo test --release test_zkvm_evm_generate_chunks -- --nocapture
```

test state_merkle_trie_root
```
RUST_LOG=info cargo test --release test_state_merkle_trie_root -- --nocapture
```

test example exec_block
```
NO=1 RUST_LOG=info cargo run --example exec_block -- --nocapture
```

test example batch_process
```
NO=1 RUST_LOG=info cargo run --example batch_process -- --nocapture
```