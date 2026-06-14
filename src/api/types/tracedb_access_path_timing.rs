pub use crate::prelude::*;

/// Access path timing entry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccessPathTiming {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_path_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub build_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub open_ms: Option<f64>,
}

impl AccessPathTiming {
    pub fn builder() -> AccessPathTimingBuilder {
        <AccessPathTimingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccessPathTimingBuilder {
    access_path_id: Option<String>,
    build_ms: Option<f64>,
    open_ms: Option<f64>,
}

impl AccessPathTimingBuilder {
    pub fn access_path_id(mut self, value: impl Into<String>) -> Self {
        self.access_path_id = Some(value.into());
        self
    }

    pub fn build_ms(mut self, value: f64) -> Self {
        self.build_ms = Some(value);
        self
    }

    pub fn open_ms(mut self, value: f64) -> Self {
        self.open_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccessPathTiming`].
    pub fn build(self) -> Result<AccessPathTiming, BuildError> {
        Ok(AccessPathTiming {
            access_path_id: self.access_path_id,
            build_ms: self.build_ms,
            open_ms: self.open_ms,
        })
    }
}
