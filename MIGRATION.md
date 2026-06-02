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

The standalone crate still references local model crates from the sibling core
checkout, including `../tracedb/crates/tracedb-query` and
`../tracedb/crates/tracedb-features`. This is acceptable during migration but
blocks crates.io publishing. See `ROADMAP.md` for the packaging cleanup plan.

## Validation

Fresh local validation on 2026-06-02:

- `cargo test --no-run` passed.
- `cargo test` passed with 62 HTTP-client tests and 6 quickstart-example tests.

```bash
cargo test
cargo run --example quickstart -- --url http://127.0.0.1:8090 --token dev-token
```

These commands are local SDK evidence for the standalone Rust SDK lane. They
are not crates.io publication, hosted-alpha, managed-cloud, benchmark, or Go SDK
evidence.
