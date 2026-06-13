pub use crate::prelude::*;

/// Snapshot response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SnapshotResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

impl SnapshotResponse {
    pub fn builder() -> SnapshotResponseBuilder {
        <SnapshotResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SnapshotResponseBuilder {
    snapshot: Option<bool>,
    target: Option<String>,
}

impl SnapshotResponseBuilder {
    pub fn snapshot(mut self, value: bool) -> Self {
        self.snapshot = Some(value);
        self
    }

    pub fn target(mut self, value: impl Into<String>) -> Self {
        self.target = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SnapshotResponse`].
    pub fn build(self) -> Result<SnapshotResponse, BuildError> {
        Ok(SnapshotResponse {
            snapshot: self.snapshot,
            target: self.target,
        })
    }
}
