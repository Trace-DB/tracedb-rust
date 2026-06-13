pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SnapshotRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

impl SnapshotRequest {
    pub fn builder() -> SnapshotRequestBuilder {
        <SnapshotRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SnapshotRequestBuilder {
    target: Option<String>,
}

impl SnapshotRequestBuilder {
    pub fn target(mut self, value: impl Into<String>) -> Self {
        self.target = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SnapshotRequest`].
    pub fn build(self) -> Result<SnapshotRequest, BuildError> {
        Ok(SnapshotRequest {
            target: self.target,
        })
    }
}
