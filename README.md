# shuriken-api-types

Shared Rust types for the [Shuriken](https://app.shuriken.trade) API — WebSocket stream payloads and common enums used across SDKs.

> **Status:** Early development — types may change.

## Install

```toml
[dependencies]
shuriken-api-types = { git = "https://github.com/ShurikenTrade/shuriken-api-types-rs" }
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

## License

MIT
