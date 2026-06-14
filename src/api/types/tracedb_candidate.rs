pub use crate::prelude::*;

/// Planner candidate explain row.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Candidate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freshness: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_components: Option<HybridScoreComponents>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_upper_bound: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_checked: Option<bool>,
}

impl Candidate {
    pub fn builder() -> CandidateBuilder {
        <CandidateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CandidateBuilder {
    freshness: Option<String>,
    record_id: Option<String>,
    score_components: Option<HybridScoreComponents>,
    score_upper_bound: Option<f64>,
    source: Option<String>,
    version_id: Option<i64>,
    visibility_checked: Option<bool>,
}

impl CandidateBuilder {
    pub fn freshness(mut self, value: impl Into<String>) -> Self {
        self.freshness = Some(value.into());
        self
    }

    pub fn record_id(mut self, value: impl Into<String>) -> Self {
        self.record_id = Some(value.into());
        self
    }

    pub fn score_components(mut self, value: HybridScoreComponents) -> Self {
        self.score_components = Some(value);
        self
    }

    pub fn score_upper_bound(mut self, value: f64) -> Self {
        self.score_upper_bound = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: i64) -> Self {
        self.version_id = Some(value);
        self
    }

    pub fn visibility_checked(mut self, value: bool) -> Self {
        self.visibility_checked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Candidate`].
    pub fn build(self) -> Result<Candidate, BuildError> {
        Ok(Candidate {
            freshness: self.freshness,
            record_id: self.record_id,
            score_components: self.score_components,
            score_upper_bound: self.score_upper_bound,
            source: self.source,
            version_id: self.version_id,
            visibility_checked: self.visibility_checked,
        })
    }
}
