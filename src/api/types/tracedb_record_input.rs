pub use crate::prelude::*;

/// TraceDB record input.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RecordInput {
    /// Record field map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<String, serde_json::Value>>,
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

impl RecordInput {
    pub fn builder() -> RecordInputBuilder {
        <RecordInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordInputBuilder {
    fields: Option<HashMap<String, serde_json::Value>>,
    id: Option<String>,
    table: Option<String>,
    tenant_id: Option<String>,
}

impl RecordInputBuilder {
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

    /// Consumes the builder and constructs a [`RecordInput`].
    pub fn build(self) -> Result<RecordInput, BuildError> {
        Ok(RecordInput {
            fields: self.fields,
            id: self.id,
            table: self.table,
            tenant_id: self.tenant_id,
            extra: Default::default(),
        })
    }
}
