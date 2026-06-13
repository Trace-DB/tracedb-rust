pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RecordPatchRequest {
    /// Patch field map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}

impl RecordPatchRequest {
    pub fn builder() -> RecordPatchRequestBuilder {
        <RecordPatchRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordPatchRequestBuilder {
    fields: Option<HashMap<String, serde_json::Value>>,
    id: Option<String>,
    table: Option<String>,
    tenant_id: Option<String>,
}

impl RecordPatchRequestBuilder {
    pub fn fields(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.fields = Some(value);
        self
    }

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

    /// Consumes the builder and constructs a [`RecordPatchRequest`].
    pub fn build(self) -> Result<RecordPatchRequest, BuildError> {
        Ok(RecordPatchRequest {
            fields: self.fields,
            id: self.id,
            table: self.table,
            tenant_id: self.tenant_id,
        })
    }
}
