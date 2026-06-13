pub use crate::prelude::*;

/// Get record request.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RecordGetRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl RecordGetRequest {
    pub fn builder() -> RecordGetRequestBuilder {
        <RecordGetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordGetRequestBuilder {
    id: Option<String>,
    table: Option<String>,
    tenant_id: Option<String>,
}

impl RecordGetRequestBuilder {
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

    /// Consumes the builder and constructs a [`RecordGetRequest`].
    pub fn build(self) -> Result<RecordGetRequest, BuildError> {
        Ok(RecordGetRequest {
            id: self.id,
            table: self.table,
            tenant_id: self.tenant_id,
            extra: Default::default(),
        })
    }
}
