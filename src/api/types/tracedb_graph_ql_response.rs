pub use crate::prelude::*;

/// Native GraphQL response envelope.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GraphQlResponse {
    /// GraphQL data object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<GraphQlError>>,
}

impl GraphQlResponse {
    pub fn builder() -> GraphQlResponseBuilder {
        <GraphQlResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraphQlResponseBuilder {
    data: Option<HashMap<String, serde_json::Value>>,
    errors: Option<Vec<GraphQlError>>,
}

impl GraphQlResponseBuilder {
    pub fn data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn errors(mut self, value: Vec<GraphQlError>) -> Self {
        self.errors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GraphQlResponse`].
    pub fn build(self) -> Result<GraphQlResponse, BuildError> {
        Ok(GraphQlResponse {
            data: self.data,
            errors: self.errors,
        })
    }
}
