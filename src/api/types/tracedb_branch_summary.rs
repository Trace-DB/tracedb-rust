pub use crate::prelude::*;

/// Branch catalog entry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BranchSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_epoch: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_branch_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl BranchSummary {
    pub fn builder() -> BranchSummaryBuilder {
        <BranchSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BranchSummaryBuilder {
    branch_id: Option<String>,
    database_id: Option<String>,
    endpoint: Option<String>,
    latest_epoch: Option<i64>,
    parent_branch_id: Option<String>,
    state: Option<String>,
}

impl BranchSummaryBuilder {
    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn database_id(mut self, value: impl Into<String>) -> Self {
        self.database_id = Some(value.into());
        self
    }

    pub fn endpoint(mut self, value: impl Into<String>) -> Self {
        self.endpoint = Some(value.into());
        self
    }

    pub fn latest_epoch(mut self, value: i64) -> Self {
        self.latest_epoch = Some(value);
        self
    }

    pub fn parent_branch_id(mut self, value: impl Into<String>) -> Self {
        self.parent_branch_id = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BranchSummary`].
    pub fn build(self) -> Result<BranchSummary, BuildError> {
        Ok(BranchSummary {
            branch_id: self.branch_id,
            database_id: self.database_id,
            endpoint: self.endpoint,
            latest_epoch: self.latest_epoch,
            parent_branch_id: self.parent_branch_id,
            state: self.state,
        })
    }
}
