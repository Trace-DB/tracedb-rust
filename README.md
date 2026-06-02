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

## Install

```bash
cargo add tracedb-sdk
```

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

The crate owns its HTTP contract wire models directly and has no normal or
dev-dependency path links to the sibling core repo. Local integration tests
still start the sibling `tracedb-server` process through Cargo so the SDK can
exercise the real local HTTP product path without linking core crates. Those
real-server tests skip when the sibling core checkout is not present.

`cargo package` passes locally. A crates.io release still needs a release
review and versioned protocol-lock signoff; this repository does not claim
publication until that release is performed.
