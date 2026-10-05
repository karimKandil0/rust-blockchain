# toy-blockchain

A minimal blockchain implementation in Rust, written as a learning exercise.

## Features

- Block with SHA-256 hashing
- Linked chain (genesis + `prev_hash` chaining)
- Validation (link integrity + tamper detection)
- Proof-of-work (nonce / difficulty) — (pending)
- Transactions instead of raw string data — (pending)

## Project structure

```
src/
  main.rs        entry point / demo
  block.rs       Block struct + hashing
  blockchain.rs  BlockChain struct + validation
```

## Dependencies

- `serde` / `serde_json` — serialization
- `sha2` — SHA-256 hashing
- `hex` — hex encoding
- `chrono` — timestamps

## Build & run

```
cargo run
```

## License

TBD
