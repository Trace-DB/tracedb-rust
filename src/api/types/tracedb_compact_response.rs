pub use crate::prelude::*;

/// Compact response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompactResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compacted: Option<bool>,
}

impl CompactResponse {
    pub fn builder() -> CompactResponseBuilder {
        <CompactResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompactResponseBuilder {
    compacted: Option<bool>,
}

impl CompactResponseBuilder {
    pub fn compacted(mut self, value: bool) -> Self {
        self.compacted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompactResponse`].
    pub fn build(self) -> Result<CompactResponse, BuildError> {
        Ok(CompactResponse {
            compacted: self.compacted,
        })
    }
}
