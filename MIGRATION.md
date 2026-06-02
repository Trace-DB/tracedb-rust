# TraceDB Rust SDK Migration Notes

This repository is the standalone home for the TraceDB Rust SDK lane pinned to
`platform-contract-v0`. The legacy crate under `tracedb/crates/tracedb-sdk/`
has been removed from the core repo.

## Cutover Status

SDK behavior now changes here first. The standalone Rust SDK tracks
`platform-contract-v0`; longer-term shared SDK code should come from
`tracedb-protocol` or published model crates rather than an in-tree core SDK
copy.
The protocol contract is pinned by `tracedb-protocol.lock`; update that lock
only when this SDK has been checked against the new `tracedb-protocol`
revision.

## Current Boundary

The standalone crate now owns its HTTP contract wire models directly. It no
longer references local model crates from the sibling core checkout, and local
integration tests start the sibling `tracedb-server` binary through Cargo
instead of linking it as a dev dependency. Real-server integration tests skip
when that sibling checkout is not present.

This closes the path-dependency packaging blocker. A crates.io release still
needs release review, version bump discipline, and protocol-lock signoff.

## Validation

Fresh local validation on 2026-06-02:

- `cargo check --all-targets` passed.
- `cargo package` passed.
- `cargo test --no-run` passed.
- `cargo test` passed with 62 HTTP-client tests and 6 quickstart-example tests.

```bash
cargo test
cargo run --example quickstart -- --url http://127.0.0.1:8090 --token dev-token
```

These commands are local SDK evidence for the standalone Rust SDK lane. They
are not crates.io publication, hosted-alpha, managed-cloud, benchmark, or Go SDK
evidence.
