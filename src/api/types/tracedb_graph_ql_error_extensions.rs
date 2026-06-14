pub use crate::prelude::*;

/// GraphQL error extensions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GraphQlErrorExtensions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl GraphQlErrorExtensions {
    pub fn builder() -> GraphQlErrorExtensionsBuilder {
        <GraphQlErrorExtensionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GraphQlErrorExtensionsBuilder {
    code: Option<String>,
}

impl GraphQlErrorExtensionsBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GraphQlErrorExtensions`].
    pub fn build(self) -> Result<GraphQlErrorExtensions, BuildError> {
        Ok(GraphQlErrorExtensions {
            code: self.code,
            extra: Default::default(),
        })
    }
}
