pub use crate::prelude::*;

/// GraphQL error entry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GraphQlError {
    /// GraphQL error extensions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<GraphQlErrorExtensions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<String>>,
}

impl GraphQlError {
    pub fn builder() -> GraphQlErrorBuilder {
        <GraphQlErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraphQlErrorBuilder {
    extensions: Option<GraphQlErrorExtensions>,
    message: Option<String>,
    path: Option<Vec<String>>,
}

impl GraphQlErrorBuilder {
    pub fn extensions(mut self, value: GraphQlErrorExtensions) -> Self {
        self.extensions = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn path(mut self, value: Vec<String>) -> Self {
        self.path = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GraphQlError`].
    pub fn build(self) -> Result<GraphQlError, BuildError> {
        Ok(GraphQlError {
            extensions: self.extensions,
            message: self.message,
            path: self.path,
        })
    }
}
