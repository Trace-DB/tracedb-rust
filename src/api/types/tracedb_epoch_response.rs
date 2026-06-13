pub use crate::prelude::*;

/// Epoch allocation response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EpochResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<i64>,
}

impl EpochResponse {
    pub fn builder() -> EpochResponseBuilder {
        <EpochResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EpochResponseBuilder {
    epoch: Option<i64>,
}

impl EpochResponseBuilder {
    pub fn epoch(mut self, value: i64) -> Self {
        self.epoch = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EpochResponse`].
    pub fn build(self) -> Result<EpochResponse, BuildError> {
        Ok(EpochResponse { epoch: self.epoch })
    }
}
