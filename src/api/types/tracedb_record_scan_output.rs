pub use crate::prelude::*;

/// Scan output.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RecordScanOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<RecordOutput>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_count: Option<i64>,
}

impl RecordScanOutput {
    pub fn builder() -> RecordScanOutputBuilder {
        <RecordScanOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordScanOutputBuilder {
    next_cursor: Option<String>,
    records: Option<Vec<RecordOutput>>,
    returned_count: Option<i64>,
}

impl RecordScanOutputBuilder {
    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn records(mut self, value: Vec<RecordOutput>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn returned_count(mut self, value: i64) -> Self {
        self.returned_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecordScanOutput`].
    pub fn build(self) -> Result<RecordScanOutput, BuildError> {
        Ok(RecordScanOutput {
            next_cursor: self.next_cursor,
            records: self.records,
            returned_count: self.returned_count,
        })
    }
}
