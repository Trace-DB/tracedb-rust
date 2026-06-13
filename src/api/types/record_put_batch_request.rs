pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RecordPutBatchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_write_timing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<RecordInput>>,
}

impl RecordPutBatchRequest {
    pub fn builder() -> RecordPutBatchRequestBuilder {
        <RecordPutBatchRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordPutBatchRequestBuilder {
    include_write_timing: Option<bool>,
    records: Option<Vec<RecordInput>>,
}

impl RecordPutBatchRequestBuilder {
    pub fn include_write_timing(mut self, value: bool) -> Self {
        self.include_write_timing = Some(value);
        self
    }

    pub fn records(mut self, value: Vec<RecordInput>) -> Self {
        self.records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecordPutBatchRequest`].
    pub fn build(self) -> Result<RecordPutBatchRequest, BuildError> {
        Ok(RecordPutBatchRequest {
            include_write_timing: self.include_write_timing,
            records: self.records,
        })
    }
}
