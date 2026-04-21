# Shuriken API Types (Rust)

Shared payload types for the Shuriken platform — consumed by `shuriken-sdk-rs` and the private monorepo.

## Releasing

Use `cargo release` — **never** run `cargo publish` manually (CI handles publishing when it sees a new tag).

```bash
cargo release patch   # bug fixes, field doc updates
cargo release minor   # new types, new fields, new modules
cargo release major   # breaking changes (renamed/removed types, changed field types)
```

This will: bump `Cargo.toml` version → commit → create git tag → push to origin. CI then publishes to crates.io.

**Update `README.md`** if new modules are added.

## Build & check

- Check: `cargo check`
- Test: `cargo test`
- Lint: `cargo clippy -- -D warnings`
- Format: `cargo fmt --check`

## Architecture

- `src/svm.rs` — Solana stream event types (swaps, pool info, balances, bonding curves)
- `src/evm.rs` — EVM stream event types (swaps, pool info, balances)
- `src/wallet.rs` — Wallet balance event types (native + token, SVM + EVM)
- `src/alpha.rs` — Alpha signal payloads
- `src/analytics.rs` — Analytics event payloads
- `src/automation.rs` — Automation/trigger event payloads
- `src/notification.rs` — Notification payloads
- `src/network.rs` — `Network` enum with `FromStr`/`Display` impls
- `src/platform.rs` — `Platform` enum

## Type sync

- Types must stay in sync with the TypeScript SDK's `src/streams/` types (`shuriken-sdk-ts`)
- Types must match the backend event payloads from `web-event-proxy`
- Downstream consumers (`shuriken-sdk-rs`) pin to git tags — bump the tag after changes
