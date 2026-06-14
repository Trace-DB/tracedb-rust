pub use crate::prelude::*;

/// Branch catalog response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BranchesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<BranchSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<bool>,
}

impl BranchesResponse {
    pub fn builder() -> BranchesResponseBuilder {
        <BranchesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BranchesResponseBuilder {
    branches: Option<Vec<BranchSummary>>,
    gateway: Option<bool>,
}

impl BranchesResponseBuilder {
    pub fn branches(mut self, value: Vec<BranchSummary>) -> Self {
        self.branches = Some(value);
        self
    }

    pub fn gateway(mut self, value: bool) -> Self {
        self.gateway = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BranchesResponse`].
    pub fn build(self) -> Result<BranchesResponse, BuildError> {
        Ok(BranchesResponse {
            branches: self.branches,
            gateway: self.gateway,
        })
    }
}
