pub use crate::prelude::*;

/// Get record response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetRecordResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<RecordOutput>,
}

impl GetRecordResponse {
    pub fn builder() -> GetRecordResponseBuilder {
        <GetRecordResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetRecordResponseBuilder {
    record: Option<RecordOutput>,
}

impl GetRecordResponseBuilder {
    pub fn record(mut self, value: RecordOutput) -> Self {
        self.record = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetRecordResponse`].
    pub fn build(self) -> Result<GetRecordResponse, BuildError> {
        Ok(GetRecordResponse {
            record: self.record,
        })
    }
}
