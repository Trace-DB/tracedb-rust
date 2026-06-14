pub use crate::prelude::*;

/// Restore response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RestoreResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restored: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<RestoreVerification>,
}

impl RestoreResponse {
    pub fn builder() -> RestoreResponseBuilder {
        <RestoreResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RestoreResponseBuilder {
    restored: Option<bool>,
    source: Option<String>,
    target: Option<String>,
    verification: Option<RestoreVerification>,
}

impl RestoreResponseBuilder {
    pub fn restored(mut self, value: bool) -> Self {
        self.restored = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn target(mut self, value: impl Into<String>) -> Self {
        self.target = Some(value.into());
        self
    }

    pub fn verification(mut self, value: RestoreVerification) -> Self {
        self.verification = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RestoreResponse`].
    pub fn build(self) -> Result<RestoreResponse, BuildError> {
        Ok(RestoreResponse {
            restored: self.restored,
            source: self.source,
            target: self.target,
            verification: self.verification,
        })
    }
}
