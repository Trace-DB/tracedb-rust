pub use crate::prelude::*;

/// TraceDB visible record output.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RecordOutput {
    /// Record field map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<i64>,
}

impl RecordOutput {
    pub fn builder() -> RecordOutputBuilder {
        <RecordOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordOutputBuilder {
    fields: Option<HashMap<String, serde_json::Value>>,
    id: Option<String>,
    table: Option<String>,
    tenant_id: Option<String>,
    version_id: Option<i64>,
}

impl RecordOutputBuilder {
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

    pub fn version_id(mut self, value: i64) -> Self {
        self.version_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecordOutput`].
    pub fn build(self) -> Result<RecordOutput, BuildError> {
        Ok(RecordOutput {
            fields: self.fields,
            id: self.id,
            table: self.table,
            tenant_id: self.tenant_id,
            version_id: self.version_id,
        })
    }
}
