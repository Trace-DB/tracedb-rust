# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.0 — 2025-05-30

### Added

- **Blocking client** (`TraceDbClient`) with HTTP/1.1 transport over raw TCP.
  - Supports all v1 routes: `ready`, `health`, `list_databases`, `list_branches`,
    `public_safe_metrics`, `apply_schema`, `put`, `put_batch`, `patch`, `delete`,
    `get`, `scan`, `query`, `traceql`, `graphql`, `bounded_graphql`,
    `graphql_schema`, `explain`, `compact`, `list_admin_jobs`, `snapshot`,
    `restore`.
  - Every route has a raw-JSON variant and a typed variant (e.g. `put` / `put_typed`).
  - Optional `TraceDbRequestOptions` for idempotency keys and actor context.

- **Async client** (`TraceDbAsyncClient`) behind the `async` feature flag (on by default).
  - Uses `reqwest` with connection pooling.
  - Mirrors the blocking client's typed API surface.
  - Constructable from config or from an existing `TraceDbClient`.

- **Configuration** (`TraceDbClientConfig`).
  - Builder-style methods: `with_database`, `with_branch`, `with_timeout`,
    `with_safe_retries`, `with_idempotency_retries`.
  - `from_env()` reads `TRACEDB_URL` (required), `TRACEDB_TOKEN`,
    `TRACEDB_DATABASE_ID`, `TRACEDB_BRANCH_ID`, `TRACEDB_TIMEOUT_MS`,
    `TRACEDB_SAFE_RETRIES`, `TRACEDB_IDEMPOTENCY_RETRIES`.

- **Retry policies**.
  - Safe retries for read-only routes (GET and read-only POST bodies).
  - Idempotency retries for keyed writes (requires an idempotency key header).

- **Query builder** (`QueryBuilder` / `TableHandle`).
  - Fluent API: `client.table("my_table").tenant("t1").match_text("body", "hello").limit(10).all()`.
  - Record CRUD helpers: `insert`, `insert_batch`, `insert_rows`, `patch_record`,
    `get_record`, `scan_typed`, `delete_record`.
  - `build()` produces a serializable `TraceQueryRequest`.

- **Feature flags**.
  - `default = ["async", "rustls-tls"]` — async support with rustls.
  - `native-tls` — switch to native TLS.
  - Disable `async` for a smaller, sync-only build.

- **Typed response models** for every endpoint (`ReadyResponse`, `HealthResponse`,
  `DatabasesResponse`, `BranchesResponse`, `MetricsResponse`, `EpochResponse`,
  `PutBatchResponse`, `DeleteResponse`, `GetRecordResponse`, `QueryResponse`,
  `GraphQlResponse`, `GraphQlSchemaResponse`, `CompactResponse`,
  `SnapshotResponse`, `RestoreResponse`, `JobsResponse`, etc.).

- **Error type** (`TraceDbClientError`) with variants for invalid URLs, config
  errors, I/O failures, JSON parse errors, timeouts, and HTTP status errors.
  - Helper methods: `error_response()`, `server_error()`, `server_error_code()`.

- **Crate identity** constants `NAME` / `VERSION`; all HTTP requests send
  `User-Agent: tracedb-sdk/<version>`.
