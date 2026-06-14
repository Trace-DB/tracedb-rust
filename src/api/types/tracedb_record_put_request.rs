pub use crate::prelude::*;

/// Full replacement record write. The server also accepts RecordInput directly.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RecordPutRequest {
    /// Optional managed-routing branch identifier injected by SDKs and gateways.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<String>,
    /// Optional managed-routing database identifier injected by SDKs and gateways.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<RecordInput>,
}

impl RecordPutRequest {
    pub fn builder() -> RecordPutRequestBuilder {
        <RecordPutRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordPutRequestBuilder {
    branch_id: Option<String>,
    database_id: Option<String>,
    record: Option<RecordInput>,
}

impl RecordPutRequestBuilder {
    pub fn branch_id(mut self, value: impl Into<String>) -> Self {
        self.branch_id = Some(value.into());
        self
    }

    pub fn database_id(mut self, value: impl Into<String>) -> Self {
        self.database_id = Some(value.into());
        self
    }

    pub fn record(mut self, value: RecordInput) -> Self {
        self.record = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecordPutRequest`].
    pub fn build(self) -> Result<RecordPutRequest, BuildError> {
        Ok(RecordPutRequest {
            branch_id: self.branch_id,
            database_id: self.database_id,
            record: self.record,
        })
    }
}
