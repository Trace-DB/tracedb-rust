pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RestoreRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_record: Option<RecordGetRequest>,
}

impl RestoreRequest {
    pub fn builder() -> RestoreRequestBuilder {
        <RestoreRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RestoreRequestBuilder {
    source: Option<String>,
    target: Option<String>,
    verify_record: Option<RecordGetRequest>,
}

impl RestoreRequestBuilder {
    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn target(mut self, value: impl Into<String>) -> Self {
        self.target = Some(value.into());
        self
    }

    pub fn verify_record(mut self, value: RecordGetRequest) -> Self {
        self.verify_record = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RestoreRequest`].
    pub fn build(self) -> Result<RestoreRequest, BuildError> {
        Ok(RestoreRequest {
            source: self.source,
            target: self.target,
            verify_record: self.verify_record,
        })
    }
}
