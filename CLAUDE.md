# Shuriken API Types (Rust)

Shared payload types for the Shuriken platform — consumed by `shuriken-sdk-rs` and the private monorepo.

## After every change

1. **Bump the version** in `Cargo.toml` following semver:
   - **patch** (0.1.0 → 0.1.1): bug fixes, field doc updates
   - **minor** (0.1.0 → 0.2.0): new types, new fields, new modules
   - **major** (0.1.0 → 1.0.0): breaking changes (renamed/removed types, changed field types)
2. **Update `README.md`** if new modules are added.

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
