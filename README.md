# TraceDB Rust SDK

TraceDB Rust SDK for the standalone Rust SDK lane pinned to
`platform-contract-v0`.

The crate provides:

- Blocking `TraceDbClient` over HTTP/1.1 for local engine workflows.
- Async `TraceDbAsyncClient` over `reqwest`.
- Typed response models for the current TraceDB HTTP contract.
- Table/query builders for ergonomic record and hybrid-query calls.
- `from_env()` configuration support through `TraceDbClientConfig`.
- Safe retries for read-only routes and idempotency retries for keyed writes.
- Crate identity constants `NAME`/`VERSION`; sync and async HTTP requests send
  `User-Agent: {NAME}/{VERSION}`.

## Quickstart

```rust
use tracedb_sdk::{TraceDbClient, TraceDbClientConfig};

let config = TraceDbClientConfig::from_env()?;
let client = TraceDbClient::new(config);
let ready = client.ready_typed()?;
println!("ready: {}", ready.ready);
```

Against an existing TraceDB HTTP server, run the bundled example from this
repository:

```bash
cargo run --example quickstart -- --url http://127.0.0.1:8090 --token dev-token
```

## Current Packaging Boundary

This repository is the standalone Rust SDK lane for
`platform-contract-v0`, pinned by `tracedb-protocol.lock`.

Local tests pass, but crates.io publication remains blocked because the crate
still depends on sibling core path dependencies, including
`../tracedb/crates/tracedb-query` and
`../tracedb/crates/tracedb-features`. Before a crates.io release, the shared
protocol/model types should move to published versioned crates or be generated
into this crate from `tracedb-protocol`.
