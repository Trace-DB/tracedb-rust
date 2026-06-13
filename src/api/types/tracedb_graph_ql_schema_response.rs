pub use crate::prelude::*;

/// Bounded GraphQL adapter schema response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GraphQlSchemaResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<String>>,
}

impl GraphQlSchemaResponse {
    pub fn builder() -> GraphQlSchemaResponseBuilder {
        <GraphQlSchemaResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraphQlSchemaResponseBuilder {
    adapter: Option<String>,
    execution: Option<String>,
    schema: Option<String>,
    tables: Option<Vec<String>>,
}

impl GraphQlSchemaResponseBuilder {
    pub fn adapter(mut self, value: impl Into<String>) -> Self {
        self.adapter = Some(value.into());
        self
    }

    pub fn execution(mut self, value: impl Into<String>) -> Self {
        self.execution = Some(value.into());
        self
    }

    pub fn schema(mut self, value: impl Into<String>) -> Self {
        self.schema = Some(value.into());
        self
    }

    pub fn tables(mut self, value: Vec<String>) -> Self {
        self.tables = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GraphQlSchemaResponse`].
    pub fn build(self) -> Result<GraphQlSchemaResponse, BuildError> {
        Ok(GraphQlSchemaResponse {
            adapter: self.adapter,
            execution: self.execution,
            schema: self.schema,
            tables: self.tables,
        })
    }
}
