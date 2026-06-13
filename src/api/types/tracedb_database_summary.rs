pub use crate::prelude::*;

/// Database catalog entry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DatabaseSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl DatabaseSummary {
    pub fn builder() -> DatabaseSummaryBuilder {
        <DatabaseSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatabaseSummaryBuilder {
    database_id: Option<String>,
    endpoint: Option<String>,
    name: Option<String>,
    org_id: Option<String>,
    project_id: Option<String>,
    region: Option<String>,
}

impl DatabaseSummaryBuilder {
    pub fn database_id(mut self, value: impl Into<String>) -> Self {
        self.database_id = Some(value.into());
        self
    }

    pub fn endpoint(mut self, value: impl Into<String>) -> Self {
        self.endpoint = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn org_id(mut self, value: impl Into<String>) -> Self {
        self.org_id = Some(value.into());
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn region(mut self, value: impl Into<String>) -> Self {
        self.region = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DatabaseSummary`].
    pub fn build(self) -> Result<DatabaseSummary, BuildError> {
        Ok(DatabaseSummary {
            database_id: self.database_id,
            endpoint: self.endpoint,
            name: self.name,
            org_id: self.org_id,
            project_id: self.project_id,
            region: self.region,
        })
    }
}
