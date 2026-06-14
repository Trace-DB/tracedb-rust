//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Tracedb**

use crate::{ApiError, ClientConfig};

pub mod tracedb;
pub struct ApiClient {
    pub config: ClientConfig,
    pub tracedb: TracedbClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            tracedb: TracedbClient::new(config.clone())?,
        })
    }
}

pub use tracedb::TracedbClient;
