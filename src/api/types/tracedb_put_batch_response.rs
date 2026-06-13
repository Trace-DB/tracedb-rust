pub use crate::prelude::*;

/// Batch write response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PutBatchResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_count: Option<i64>,
    /// Optional write timing attribution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_timing: Option<HashMap<String, f64>>,
}

impl PutBatchResponse {
    pub fn builder() -> PutBatchResponseBuilder {
        <PutBatchResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PutBatchResponseBuilder {
    epoch: Option<i64>,
    record_count: Option<i64>,
    write_timing: Option<HashMap<String, f64>>,
}

impl PutBatchResponseBuilder {
    pub fn epoch(mut self, value: i64) -> Self {
        self.epoch = Some(value);
        self
    }

    pub fn record_count(mut self, value: i64) -> Self {
        self.record_count = Some(value);
        self
    }

    pub fn write_timing(mut self, value: HashMap<String, f64>) -> Self {
        self.write_timing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PutBatchResponse`].
    pub fn build(self) -> Result<PutBatchResponse, BuildError> {
        Ok(PutBatchResponse {
            epoch: self.epoch,
            record_count: self.record_count,
            write_timing: self.write_timing,
        })
    }
}
