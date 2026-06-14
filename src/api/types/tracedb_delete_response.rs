pub use crate::prelude::*;

/// Delete response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<i64>,
}

impl DeleteResponse {
    pub fn builder() -> DeleteResponseBuilder {
        <DeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteResponseBuilder {
    deleted: Option<bool>,
    epoch: Option<i64>,
}

impl DeleteResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn epoch(mut self, value: i64) -> Self {
        self.epoch = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteResponse`].
    pub fn build(self) -> Result<DeleteResponse, BuildError> {
        Ok(DeleteResponse {
            deleted: self.deleted,
            epoch: self.epoch,
        })
    }
}
