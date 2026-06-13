# TraceDb Rust Library

[![fern shield](https://img.shields.io/badge/%F0%9F%8C%BF-Built%20with%20Fern-brightgreen)](https://buildwithfern.com?utm_source=github&utm_medium=github&utm_campaign=readme&utm_source=https%3A%2F%2Fgithub.com%2FTrace-DB%2Ftracedb-rust)
[![crates.io shield](https://img.shields.io/crates/v/trace_db_api)](https://crates.io/crates/trace_db_api)

The TraceDb Rust library provides convenient access to the TraceDb APIs from Rust.

## Table of Contents

- [Install](#install)
- [Quickstart](#quickstart)
- [Current Packaging Boundary](#current-packaging-boundary)
- [Claim Boundary](#claim-boundary)
- [Installation](#installation)
- [Reference](#reference)
- [Usage](#usage)
- [Environments](#environments)
- [Errors](#errors)
- [Request Types](#request-types)
- [Advanced](#advanced)
  - [Retries](#retries)
  - [Timeouts](#timeouts)
  - [Additional Headers](#additional-headers)
  - [Additional Query String Parameters](#additional-query-string-parameters)
- [Contributing](#contributing)

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

The source crate version for this release wave is `tracedb-sdk = "0.1.1"`.

## Claim Boundary

`tracedb-sdk = "0.1.1"` is Rust SDK packaging for the current TraceDB HTTP
product surface. It does not claim managed-cloud
readiness, hosted-alpha readiness, SQL compatibility, benchmark wins,
production SLA, or Go SDK support.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
trace_db_api = "0.1.1"
```

Or install via cargo:

```sh
cargo add trace_db_api
```

## Reference

A full reference for this library is available [here](https://github.com/Trace-DB/tracedb-rust/blob/HEAD/./reference.md).

## Usage

Instantiate and use the client with the following:

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .tracedb
        .admin
        .post_admin_compact(
            &EmptyObject(HashMap::from([(
                "key".to_string(),
                serde_json::json!("value"),
            )])),
            None,
        )
        .await;
}
```

## Environments

This SDK allows you to configure different environments for API requests.

```rust
use trace_db_api::prelude::{*};

let config = ClientConfig {
    base_url: Environment::Default.url().to_string(),
    ..Default::default()
};
let client = Client::new(config).expect("Failed to build client");
```

## Errors

When the API returns a non-success status code (4xx or 5xx response), an error will be returned.

```rust
match client.tracedb.admin.post_admin_compact(None)?.await {
    Ok(response) => {
        println!("Success: {:?}", response);
    },
    Err(ApiError::HTTP { status, message }) => {
        println!("API Error {}: {:?}", status, message);
    },
    Err(e) => {
        println!("Other error: {:?}", e);
    }
}
```

## Request Types

The SDK exports all request types as Rust structs. Simply import them from the crate to access them:

```rust
use trace_db_api::prelude::{*};

let request = RestoreRequest {
    ...
};
```

## Advanced

### Retries

The SDK is instrumented with automatic retries with exponential backoff. A request will be retried as long
as the request is deemed retryable and the number of retry attempts has not grown larger than the configured
retry limit (default: 2).

A request is deemed retryable when any of the following HTTP status codes is returned:

- [408](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/408) (Timeout)
- [429](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/429) (Too Many Requests)
- [5XX](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#server_error_responses) (Internal Server Error)

The `retryStatusCodes` configuration controls which [5XX](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#server_error_responses) status codes are retried:

- `legacy` (default): Retries `408`, `429`, and all `>= 500`
- `recommended`: Retries `408`, `429`, `502`, `503`, `504` only (excludes `500 Internal Server Error` to avoid retrying non-idempotent failures)

Use the `max_retries` method to configure this behavior.

```rust
let response = client.tracedb.admin.post_admin_compact(
    Some(RequestOptions::new().max_retries(3))
)?.await;
```

### Timeouts

The SDK defaults to a 30 second timeout. Use the `timeout` method to configure this behavior.

```rust
let response = client.tracedb.admin.post_admin_compact(
    Some(RequestOptions::new().timeout_seconds(30))
)?.await;
```

### Additional Headers

You can add custom headers to requests using `RequestOptions`.

```rust
let response = client.tracedb.admin.post_admin_compact(
    Some(
        RequestOptions::new()
            .additional_header("X-Custom-Header", "custom-value")
            .additional_header("X-Another-Header", "another-value")
    )
)?
.await;
```

### Additional Query String Parameters

You can add custom query parameters to requests using `RequestOptions`.

```rust
let response = client.tracedb.admin.post_admin_compact(
    Some(
        RequestOptions::new()
            .additional_query_param("filter", "active")
            .additional_query_param("sort", "desc")
    )
)?
.await;
```

## Contributing

While we value open-source contributions to this SDK, this library is generated programmatically.
Additions made directly to this library would have to be moved over to our generation code,
otherwise they would be overwritten upon the next generated release. Feel free to open a PR as
a proof of concept, but know that we will not be able to merge it as-is. We suggest opening
an issue first to discuss with us!

On the other hand, contributions to the README are always very welcome!
