pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecordDeleteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tombstone: Option<String>,
}

impl RecordDeleteRequest {
    pub fn builder() -> RecordDeleteRequestBuilder {
        <RecordDeleteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordDeleteRequestBuilder {
    id: Option<String>,
    table: Option<String>,
    tenant_id: Option<String>,
    tombstone: Option<String>,
}

impl RecordDeleteRequestBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn table(mut self, value: impl Into<String>) -> Self {
        self.table = Some(value.into());
        self
    }

    pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    pub fn tombstone(mut self, value: impl Into<String>) -> Self {
        self.tombstone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecordDeleteRequest`].
    pub fn build(self) -> Result<RecordDeleteRequest, BuildError> {
        Ok(RecordDeleteRequest {
            id: self.id,
            table: self.table,
            tenant_id: self.tenant_id,
            tombstone: self.tombstone,
        })
    }
}
