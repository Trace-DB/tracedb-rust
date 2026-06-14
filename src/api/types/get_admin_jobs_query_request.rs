pub use crate::prelude::*;

/// Query parameters for getAdminJobs
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAdminJobsQueryRequest {
    /// Canonical managed-routing database identifier for bodyless routes. SDKs must use this parameter name (not db_id, databaseId, or similar variants) so all SDKs target the same gateway routing key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<String>,
    /// Canonical managed-routing branch identifier for bodyless routes. SDKs must use this parameter name (not br_id, branchId, or similar variants) so all SDKs target the same gateway routing key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
}

impl GetAdminJobsQueryRequest {
    pub fn builder() -> GetAdminJobsQueryRequestBuilder {
        <GetAdminJobsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAdminJobsQueryRequestBuilder {
    database_id: Option<String>,
    branch_id: Option<String>,
}

impl GetAdminJobsQueryRequestBuilder {
    pub fn database_id(mut self, value: impl Into<String>) -> Self {
        self.database_id = Some(value.into());
        self
    }

    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetAdminJobsQueryRequest`].
    pub fn build(self) -> Result<GetAdminJobsQueryRequest, BuildError> {
        Ok(GetAdminJobsQueryRequest {
            database_id: self.database_id,
            branch_id: self.branch_id,
        })
    }
}
