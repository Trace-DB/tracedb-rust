pub use crate::prelude::*;

/// Hybrid query score components.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct HybridScoreComponents {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub final_score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freshness_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexical: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector: Option<f64>,
}

impl HybridScoreComponents {
    pub fn builder() -> HybridScoreComponentsBuilder {
        <HybridScoreComponentsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HybridScoreComponentsBuilder {
    final_score: Option<f64>,
    freshness_penalty: Option<f64>,
    lexical: Option<f64>,
    relational: Option<f64>,
    vector: Option<f64>,
}

impl HybridScoreComponentsBuilder {
    pub fn final_score(mut self, value: f64) -> Self {
        self.final_score = Some(value);
        self
    }

    pub fn freshness_penalty(mut self, value: f64) -> Self {
        self.freshness_penalty = Some(value);
        self
    }

    pub fn lexical(mut self, value: f64) -> Self {
        self.lexical = Some(value);
        self
    }

    pub fn relational(mut self, value: f64) -> Self {
        self.relational = Some(value);
        self
    }

    pub fn vector(mut self, value: f64) -> Self {
        self.vector = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`HybridScoreComponents`].
    pub fn build(self) -> Result<HybridScoreComponents, BuildError> {
        Ok(HybridScoreComponents {
            final_score: self.final_score,
            freshness_penalty: self.freshness_penalty,
            lexical: self.lexical,
            relational: self.relational,
            vector: self.vector,
        })
    }
}
