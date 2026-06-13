pub use crate::prelude::*;

/// Full replacement record write. The server also accepts RecordInput directly.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(transparent)]
pub struct RecordPutRequest {
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
    record: Option<RecordInput>,
}

impl RecordPutRequestBuilder {
    pub fn record(mut self, value: RecordInput) -> Self {
        self.record = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecordPutRequest`].
    pub fn build(self) -> Result<RecordPutRequest, BuildError> {
        Ok(RecordPutRequest {
            record: self.record,
        })
    }
}
