pub use crate::prelude::*;

/// Hybrid query result row.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct HybridQueryRow {
    /// Record field map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<HybridScoreComponents>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<i64>,
}

impl HybridQueryRow {
    pub fn builder() -> HybridQueryRowBuilder {
        <HybridQueryRowBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HybridQueryRowBuilder {
    fields: Option<HashMap<String, serde_json::Value>>,
    record_id: Option<String>,
    score: Option<HybridScoreComponents>,
    tenant_id: Option<String>,
    version_id: Option<i64>,
}

impl HybridQueryRowBuilder {
    pub fn fields(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn record_id(mut self, value: impl Into<String>) -> Self {
        self.record_id = Some(value.into());
        self
    }

    pub fn score(mut self, value: HybridScoreComponents) -> Self {
        self.score = Some(value);
        self
    }

    pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: i64) -> Self {
        self.version_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`HybridQueryRow`].
    pub fn build(self) -> Result<HybridQueryRow, BuildError> {
        Ok(HybridQueryRow {
            fields: self.fields,
            record_id: self.record_id,
            score: self.score,
            tenant_id: self.tenant_id,
            version_id: self.version_id,
        })
    }
}
