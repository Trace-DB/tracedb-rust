pub use crate::prelude::*;

/// Public-safe metrics response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MetricsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_epoch: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_epoch: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_requests: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

impl MetricsResponse {
    pub fn builder() -> MetricsResponseBuilder {
        <MetricsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MetricsResponseBuilder {
    durable_epoch: Option<i64>,
    gateway: Option<bool>,
    index_count: Option<i64>,
    latest_epoch: Option<i64>,
    module_count: Option<i64>,
    rate_limit_enabled: Option<bool>,
    rate_limit_requests: Option<i64>,
    recovery_state: Option<String>,
    requests: Option<i64>,
    schema_count: Option<i64>,
    segment_count: Option<i64>,
    service: Option<String>,
}

impl MetricsResponseBuilder {
    pub fn durable_epoch(mut self, value: i64) -> Self {
        self.durable_epoch = Some(value);
        self
    }

    pub fn gateway(mut self, value: bool) -> Self {
        self.gateway = Some(value);
        self
    }

    pub fn index_count(mut self, value: i64) -> Self {
        self.index_count = Some(value);
        self
    }

    pub fn latest_epoch(mut self, value: i64) -> Self {
        self.latest_epoch = Some(value);
        self
    }

    pub fn module_count(mut self, value: i64) -> Self {
        self.module_count = Some(value);
        self
    }

    pub fn rate_limit_enabled(mut self, value: bool) -> Self {
        self.rate_limit_enabled = Some(value);
        self
    }

    pub fn rate_limit_requests(mut self, value: i64) -> Self {
        self.rate_limit_requests = Some(value);
        self
    }

    pub fn recovery_state(mut self, value: impl Into<String>) -> Self {
        self.recovery_state = Some(value.into());
        self
    }

    pub fn requests(mut self, value: i64) -> Self {
        self.requests = Some(value);
        self
    }

    pub fn schema_count(mut self, value: i64) -> Self {
        self.schema_count = Some(value);
        self
    }

    pub fn segment_count(mut self, value: i64) -> Self {
        self.segment_count = Some(value);
        self
    }

    pub fn service(mut self, value: impl Into<String>) -> Self {
        self.service = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MetricsResponse`].
    pub fn build(self) -> Result<MetricsResponse, BuildError> {
        Ok(MetricsResponse {
            durable_epoch: self.durable_epoch,
            gateway: self.gateway,
            index_count: self.index_count,
            latest_epoch: self.latest_epoch,
            module_count: self.module_count,
            rate_limit_enabled: self.rate_limit_enabled,
            rate_limit_requests: self.rate_limit_requests,
            recovery_state: self.recovery_state,
            requests: self.requests,
            schema_count: self.schema_count,
            segment_count: self.segment_count,
            service: self.service,
        })
    }
}
