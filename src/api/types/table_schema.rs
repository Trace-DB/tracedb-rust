pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TableSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_id_column: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalar_columns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id_column: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_indexed_columns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_columns: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl TableSchema {
    pub fn builder() -> TableSchemaBuilder {
        <TableSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TableSchemaBuilder {
    name: Option<String>,
    primary_id_column: Option<String>,
    scalar_columns: Option<Vec<String>>,
    tenant_id_column: Option<String>,
    text_indexed_columns: Option<Vec<String>>,
    vector_columns: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl TableSchemaBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn primary_id_column(mut self, value: impl Into<String>) -> Self {
        self.primary_id_column = Some(value.into());
        self
    }

    pub fn scalar_columns(mut self, value: Vec<String>) -> Self {
        self.scalar_columns = Some(value);
        self
    }

    pub fn tenant_id_column(mut self, value: impl Into<String>) -> Self {
        self.tenant_id_column = Some(value.into());
        self
    }

    pub fn text_indexed_columns(mut self, value: Vec<String>) -> Self {
        self.text_indexed_columns = Some(value);
        self
    }

    pub fn vector_columns(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.vector_columns = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TableSchema`].
    pub fn build(self) -> Result<TableSchema, BuildError> {
        Ok(TableSchema {
            name: self.name,
            primary_id_column: self.primary_id_column,
            scalar_columns: self.scalar_columns,
            tenant_id_column: self.tenant_id_column,
            text_indexed_columns: self.text_indexed_columns,
            vector_columns: self.vector_columns,
        })
    }
}
