# shuriken-api-types

[![crates.io](https://img.shields.io/crates/v/shuriken-api-types)](https://crates.io/crates/shuriken-api-types)

Shared Rust types for the [Shuriken](https://app.shuriken.trade) API — WebSocket stream payloads and common enums used across SDKs.

> **Status:** Early development — types may change.

## Install

```toml
[dependencies]
shuriken-api-types = "0.4"
```

## Overview

This crate provides `serde`-compatible types for Shuriken's real-time stream events:

| Module | Contents |
|--------|----------|
| `svm` | Solana token swaps, pool info, balances, bonding curves |
| `evm` | EVM token swaps, pool info, balances |
| `wallet` | Wallet balance events (native + token, SVM + EVM) |
| `alpha` | Alpha signal payloads |
| `analytics` | Analytics event payloads |
| `automation` | Automation/trigger event payloads |
| `notification` | Notification payloads |
| `Network` | Typed network enum (`Solana`, `Ethereum`, `Base`, …) |
| `Platform` | Platform enum |

## Usage

```rust
use shuriken_api_types::svm::SwapEvent;
use shuriken_api_types::Network;

let event: SwapEvent = serde_json::from_str(&payload)?;
let network: Network = "solana".parse()?;
```

## Releasing

Use [`cargo-release`](https://github.com/crate-ci/cargo-release) — **never** run `cargo publish` manually (CI handles publishing when it sees a new tag).

```bash
cargo release patch   # bug fixes, doc updates
cargo release minor   # new types, new fields, new modules
cargo release major   # breaking changes
```

This bumps the version, commits, tags, and pushes. CI then publishes to crates.io.

## License

MIT
