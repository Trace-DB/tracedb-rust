pub use crate::prelude::*;

/// Hybrid lexical/vector/scalar query.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct HybridQuery {
    /// Opaque cursor returned by the previous query page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explain: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freshness: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_seed: Option<String>,
    /// Scalar equality predicates keyed by schema scalar column.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalar_eq: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_as_of: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Optional schema text-indexed column to search. When omitted, TraceDB searches all text-indexed columns for backwards-compatible fieldless queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector: Option<Vec<f64>>,
    /// Optional schema vector column to score. When omitted, TraceDB uses the first vector column for backwards-compatible fieldless queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_field: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl HybridQuery {
    pub fn builder() -> HybridQueryBuilder {
        <HybridQueryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HybridQueryBuilder {
    cursor: Option<String>,
    explain: Option<bool>,
    freshness: Option<String>,
    graph_seed: Option<String>,
    scalar_eq: Option<HashMap<String, serde_json::Value>>,
    table: Option<String>,
    temporal_as_of: Option<i64>,
    tenant_id: Option<String>,
    text: Option<String>,
    text_field: Option<String>,
    top_k: Option<i64>,
    vector: Option<Vec<f64>>,
    vector_field: Option<String>,
}

impl HybridQueryBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn explain(mut self, value: bool) -> Self {
        self.explain = Some(value);
        self
    }

    pub fn freshness(mut self, value: impl Into<String>) -> Self {
        self.freshness = Some(value.into());
        self
    }

    pub fn graph_seed(mut self, value: impl Into<String>) -> Self {
        self.graph_seed = Some(value.into());
        self
    }

    pub fn scalar_eq(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.scalar_eq = Some(value);
        self
    }

    pub fn table(mut self, value: impl Into<String>) -> Self {
        self.table = Some(value.into());
        self
    }

    pub fn temporal_as_of(mut self, value: i64) -> Self {
        self.temporal_as_of = Some(value);
        self
    }

    pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn text_field(mut self, value: impl Into<String>) -> Self {
        self.text_field = Some(value.into());
        self
    }

    pub fn top_k(mut self, value: i64) -> Self {
        self.top_k = Some(value);
        self
    }

    pub fn vector(mut self, value: Vec<f64>) -> Self {
        self.vector = Some(value);
        self
    }

    pub fn vector_field(mut self, value: impl Into<String>) -> Self {
        self.vector_field = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`HybridQuery`].
    pub fn build(self) -> Result<HybridQuery, BuildError> {
        Ok(HybridQuery {
            cursor: self.cursor,
            explain: self.explain,
            freshness: self.freshness,
            graph_seed: self.graph_seed,
            scalar_eq: self.scalar_eq,
            table: self.table,
            temporal_as_of: self.temporal_as_of,
            tenant_id: self.tenant_id,
            text: self.text,
            text_field: self.text_field,
            top_k: self.top_k,
            vector: self.vector,
            vector_field: self.vector_field,
            extra: Default::default(),
        })
    }
}
