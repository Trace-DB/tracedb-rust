//! # TraceDB v1 HTTP API SDK
//!
//! The official Rust SDK for the TraceDB v1 HTTP API.
//!
//! ## Getting Started
//!
//! ```rust
//! use trace_db_api::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ClientConfig {
//!         token: Some("<token>".to_string()),
//!         ..Default::default()
//!     };
//!     let client = ApiClient::new(config).expect("Failed to build client");
//!     client
//!         .tracedb
//!         .admin
//!         .post_admin_compact(
//!             &EmptyObject(HashMap::from([(
//!                 "key".to_string(),
//!                 serde_json::json!("value"),
//!             )])),
//!             None,
//!         )
//!         .await;
//! }
//! ```
//!
//! ## Modules
//!
//! - [`api`] - Core API types and models
//! - [`client`] - Client implementations
//! - [`config`] - Configuration options
//! - [`core`] - Core utilities and infrastructure
//! - [`error`] - Error types and handling
//! - [`prelude`] - Common imports for convenience

// Generated client code from Fern triggers several clippy lints that are safe
// to allow for a generated SDK crate. These are reviewed upstream with Fern.
#![allow(
    unused_imports,
    dead_code,
    clippy::derivable_impls,
    clippy::field_reassign_with_default,
    clippy::redundant_closure,
    clippy::type_complexity,
    clippy::while_let_on_iterator,
    clippy::needless_return,
    clippy::approx_constant,
    clippy::manual_range_contains
)]
// Fern generates doc comments containing Vec<T> / Vec<Option<bool>> which
// rustdoc interprets as unclosed HTML tags.
#![allow(rustdoc::invalid_html_tags)]

pub mod api;
pub mod client;
pub mod config;
pub mod core;
pub mod environment;
pub mod error;
pub mod prelude;

pub use api::*;
pub use client::*;
pub use config::*;
pub use core::*;
pub use environment::*;
pub use error::{ApiError, BuildError};
