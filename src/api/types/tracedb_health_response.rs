pub use crate::prelude::*;

/// Health response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HealthResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_databases: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metered_requests: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

impl HealthResponse {
    pub fn builder() -> HealthResponseBuilder {
        <HealthResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HealthResponseBuilder {
    catalog_databases: Option<i64>,
    engine_url: Option<String>,
    metered_requests: Option<i64>,
    ok: Option<bool>,
    service: Option<String>,
}

impl HealthResponseBuilder {
    pub fn catalog_databases(mut self, value: i64) -> Self {
        self.catalog_databases = Some(value);
        self
    }

    pub fn engine_url(mut self, value: impl Into<String>) -> Self {
        self.engine_url = Some(value.into());
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

    pub fn service(mut self, value: impl Into<String>) -> Self {
        self.service = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`HealthResponse`].
    pub fn build(self) -> Result<HealthResponse, BuildError> {
        Ok(HealthResponse {
            catalog_databases: self.catalog_databases,
            engine_url: self.engine_url,
            metered_requests: self.metered_requests,
            ok: self.ok,
            service: self.service,
        })
    }
}
