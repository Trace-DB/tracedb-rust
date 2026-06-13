pub use crate::prelude::*;

/// GraphQL request body. Native operations accept an input JSON string argument and standard variables/operationName fields.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GraphQlQueryRequest {
    #[serde(rename = "operationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// GraphQL variables map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, serde_json::Value>>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl GraphQlQueryRequest {
    pub fn builder() -> GraphQlQueryRequestBuilder {
        <GraphQlQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraphQlQueryRequestBuilder {
    operation_name: Option<String>,
    query: Option<String>,
    variables: Option<HashMap<String, serde_json::Value>>,
}

impl GraphQlQueryRequestBuilder {
    pub fn operation_name(mut self, value: impl Into<String>) -> Self {
        self.operation_name = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn variables(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.variables = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GraphQlQueryRequest`].
    pub fn build(self) -> Result<GraphQlQueryRequest, BuildError> {
        Ok(GraphQlQueryRequest {
            operation_name: self.operation_name,
            query: self.query,
            variables: self.variables,
            extra: Default::default(),
        })
    }
}
