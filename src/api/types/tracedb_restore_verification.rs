pub use crate::prelude::*;

/// Optional restored-target record verification.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RestoreVerification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<RecordOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<RecordGetRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RestoreVerificationStatus>,
}

impl RestoreVerification {
    pub fn builder() -> RestoreVerificationBuilder {
        <RestoreVerificationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RestoreVerificationBuilder {
    record: Option<RecordOutput>,
    record_visible: Option<bool>,
    request: Option<RecordGetRequest>,
    status: Option<RestoreVerificationStatus>,
}

impl RestoreVerificationBuilder {
    pub fn record(mut self, value: RecordOutput) -> Self {
        self.record = Some(value);
        self
    }

    pub fn record_visible(mut self, value: bool) -> Self {
        self.record_visible = Some(value);
        self
    }

    pub fn request(mut self, value: RecordGetRequest) -> Self {
        self.request = Some(value);
        self
    }

    pub fn status(mut self, value: RestoreVerificationStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RestoreVerification`].
    pub fn build(self) -> Result<RestoreVerification, BuildError> {
        Ok(RestoreVerification {
            record: self.record,
            record_visible: self.record_visible,
            request: self.request,
            status: self.status,
        })
    }
}
