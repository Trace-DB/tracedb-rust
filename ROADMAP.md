# TraceDB Rust SDK Roadmap

This is the standalone Rust SDK lane pinned to `platform-contract-v0` by
`tracedb-protocol.lock`.

## Before crates.io publishing

- Keep SDK-owned wire models in lockstep with `platform-contract-v0`, or move
  them to generated protocol output once `tracedb-protocol` owns generation.
- Add package release metadata review and version bump discipline for the first
  crates.io publish.
- Add a `rustls` TLS feature if remote TLS endpoints become part of the Rust
  SDK lane.
- Clearly document that the blocking raw TCP transport is local-HTTP only unless
  TLS support is added.
- Keep `User-Agent` versioning aligned with crate releases.

## Compatibility

The SDK currently targets `platform-contract-v0` pinned by
`tracedb-protocol.lock`.

## Current Checkpoint

The sibling core path-dependency blocker is closed. Local integration tests use
the sibling `tracedb-server` process as test infrastructure without linking core
crates into this SDK package.

This roadmap does not claim crates.io publication, hosted-alpha, managed-cloud,
benchmark, or Go SDK support.
