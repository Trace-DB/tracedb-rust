pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecordScanRequest {
    /// Opaque cursor returned by the previous scan page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}

impl RecordScanRequest {
    pub fn builder() -> RecordScanRequestBuilder {
        <RecordScanRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordScanRequestBuilder {
    cursor: Option<String>,
    limit: Option<i64>,
    table: Option<String>,
    tenant_id: Option<String>,
}

impl RecordScanRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
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

    /// Consumes the builder and constructs a [`RecordScanRequest`].
    pub fn build(self) -> Result<RecordScanRequest, BuildError> {
        Ok(RecordScanRequest {
            cursor: self.cursor,
            limit: self.limit,
            table: self.table,
            tenant_id: self.tenant_id,
        })
    }
}
