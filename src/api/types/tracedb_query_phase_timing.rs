pub use crate::prelude::*;

/// Query phase timing entry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryPhaseTiming {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub elapsed_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

impl QueryPhaseTiming {
    pub fn builder() -> QueryPhaseTimingBuilder {
        <QueryPhaseTimingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryPhaseTimingBuilder {
    elapsed_ms: Option<f64>,
    phase: Option<String>,
}

impl QueryPhaseTimingBuilder {
    pub fn elapsed_ms(mut self, value: f64) -> Self {
        self.elapsed_ms = Some(value);
        self
    }

    pub fn phase(mut self, value: impl Into<String>) -> Self {
        self.phase = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryPhaseTiming`].
    pub fn build(self) -> Result<QueryPhaseTiming, BuildError> {
        Ok(QueryPhaseTiming {
            elapsed_ms: self.elapsed_ms,
            phase: self.phase,
        })
    }
}
