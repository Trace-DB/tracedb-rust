pub use crate::prelude::*;

/// Error response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ErrorResponse {
    /// Stable machine-readable error code when available; existing clients can continue to read error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl ErrorResponse {
    pub fn builder() -> ErrorResponseBuilder {
        <ErrorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorResponseBuilder {
    code: Option<String>,
    error: Option<String>,
}

impl ErrorResponseBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ErrorResponse`].
    pub fn build(self) -> Result<ErrorResponse, BuildError> {
        Ok(ErrorResponse {
            code: self.code,
            error: self.error,
        })
    }
}
