pub use crate::prelude::*;

/// Readiness response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReadyResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_databases: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durable_epoch: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_health_checked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_status_code: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_epoch: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metered_requests: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

impl ReadyResponse {
    pub fn builder() -> ReadyResponseBuilder {
        <ReadyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadyResponseBuilder {
    catalog_databases: Option<i64>,
    durable_epoch: Option<i64>,
    engine_health_checked: Option<bool>,
    engine_status_code: Option<i64>,
    engine_url: Option<String>,
    error: Option<String>,
    latest_epoch: Option<i64>,
    metered_requests: Option<i64>,
    ok: Option<bool>,
    ready: Option<bool>,
    recovery_state: Option<String>,
    service: Option<String>,
}

impl ReadyResponseBuilder {
    pub fn catalog_databases(mut self, value: i64) -> Self {
        self.catalog_databases = Some(value);
        self
    }

    pub fn durable_epoch(mut self, value: i64) -> Self {
        self.durable_epoch = Some(value);
        self
    }

    pub fn engine_health_checked(mut self, value: bool) -> Self {
        self.engine_health_checked = Some(value);
        self
    }

    pub fn engine_status_code(mut self, value: i64) -> Self {
        self.engine_status_code = Some(value);
        self
    }

    pub fn engine_url(mut self, value: impl Into<String>) -> Self {
        self.engine_url = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn latest_epoch(mut self, value: i64) -> Self {
        self.latest_epoch = Some(value);
        self
    }

    pub fn metered_requests(mut self, value: i64) -> Self {
        self.metered_requests = Some(value);
        self
    }

    pub fn ok(mut self, value: bool) -> Self {
        self.ok = Some(value);
        self
    }

    pub fn ready(mut self, value: bool) -> Self {
        self.ready = Some(value);
        self
    }

    pub fn recovery_state(mut self, value: impl Into<String>) -> Self {
        self.recovery_state = Some(value.into());
        self
    }

    pub fn service(mut self, value: impl Into<String>) -> Self {
        self.service = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReadyResponse`].
    pub fn build(self) -> Result<ReadyResponse, BuildError> {
        Ok(ReadyResponse {
            catalog_databases: self.catalog_databases,
            durable_epoch: self.durable_epoch,
            engine_health_checked: self.engine_health_checked,
            engine_status_code: self.engine_status_code,
            engine_url: self.engine_url,
            error: self.error,
            latest_epoch: self.latest_epoch,
            metered_requests: self.metered_requests,
            ok: self.ok,
            ready: self.ready,
            recovery_state: self.recovery_state,
            service: self.service,
        })
    }
}
