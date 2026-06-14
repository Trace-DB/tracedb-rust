pub use crate::prelude::*;

/// Query response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explain: Option<HybridExplain>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<HybridQueryRow>>,
}

impl QueryResponse {
    pub fn builder() -> QueryResponseBuilder {
        <QueryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryResponseBuilder {
    explain: Option<HybridExplain>,
    next_cursor: Option<String>,
    results: Option<Vec<HybridQueryRow>>,
}

impl QueryResponseBuilder {
    pub fn explain(mut self, value: HybridExplain) -> Self {
        self.explain = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    pub fn results(mut self, value: Vec<HybridQueryRow>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryResponse`].
    pub fn build(self) -> Result<QueryResponse, BuildError> {
        Ok(QueryResponse {
            explain: self.explain,
            next_cursor: self.next_cursor,
            results: self.results,
        })
    }
}
