pub use crate::prelude::*;

/// Database catalog response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DatabasesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub databases: Option<Vec<DatabaseSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

impl DatabasesResponse {
    pub fn builder() -> DatabasesResponseBuilder {
        <DatabasesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatabasesResponseBuilder {
    databases: Option<Vec<DatabaseSummary>>,
    gateway: Option<bool>,
    mode: Option<String>,
}

impl DatabasesResponseBuilder {
    pub fn databases(mut self, value: Vec<DatabaseSummary>) -> Self {
        self.databases = Some(value);
        self
    }

    pub fn gateway(mut self, value: bool) -> Self {
        self.gateway = Some(value);
        self
    }

    pub fn mode(mut self, value: impl Into<String>) -> Self {
        self.mode = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DatabasesResponse`].
    pub fn build(self) -> Result<DatabasesResponse, BuildError> {
        Ok(DatabasesResponse {
            databases: self.databases,
            gateway: self.gateway,
            mode: self.mode,
        })
    }
}
