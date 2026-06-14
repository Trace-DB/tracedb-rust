pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TraceQlQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl TraceQlQueryRequest {
    pub fn builder() -> TraceQlQueryRequestBuilder {
        <TraceQlQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TraceQlQueryRequestBuilder {
    query: Option<String>,
}

impl TraceQlQueryRequestBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TraceQlQueryRequest`].
    pub fn build(self) -> Result<TraceQlQueryRequest, BuildError> {
        Ok(TraceQlQueryRequest { query: self.query })
    }
}
