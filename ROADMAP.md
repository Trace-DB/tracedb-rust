# TraceDB Rust SDK Roadmap

This is the standalone Rust SDK lane pinned to `platform-contract-v0` by
`tracedb-protocol.lock`.

## Before crates.io publishing

- Remove local path dependencies on `../tracedb/crates/tracedb-query` and
  `../tracedb/crates/tracedb-features`.
- Decide whether shared protocol models are generated into the SDK or published
  as separate versioned crates.
- Add a `rustls` TLS feature if remote TLS endpoints become part of the Rust
  SDK lane.
- Clearly document that the blocking raw TCP transport is local-HTTP only unless
  TLS support is added.
- Keep `User-Agent` versioning aligned with crate releases.

## Compatibility

The SDK currently targets `platform-contract-v0` pinned by
`tracedb-protocol.lock`.

## Current Checkpoint

Local tests pass, but crates.io publication remains blocked by sibling core path
dependencies. This roadmap does not claim crates.io publication, hosted-alpha,
managed-cloud, benchmark, or Go SDK support.
