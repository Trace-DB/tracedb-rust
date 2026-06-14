pub use crate::prelude::*;

/// Access path explain entry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccessPathExplain {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_path_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidates: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opened: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_checked_before_open: Option<bool>,
}

impl AccessPathExplain {
    pub fn builder() -> AccessPathExplainBuilder {
        <AccessPathExplainBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccessPathExplainBuilder {
    access_path_id: Option<String>,
    candidates: Option<i64>,
    opened: Option<bool>,
    visibility_checked_before_open: Option<bool>,
}

impl AccessPathExplainBuilder {
    pub fn access_path_id(mut self, value: impl Into<String>) -> Self {
        self.access_path_id = Some(value.into());
        self
    }

    pub fn candidates(mut self, value: i64) -> Self {
        self.candidates = Some(value);
        self
    }

    pub fn opened(mut self, value: bool) -> Self {
        self.opened = Some(value);
        self
    }

    pub fn visibility_checked_before_open(mut self, value: bool) -> Self {
        self.visibility_checked_before_open = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccessPathExplain`].
    pub fn build(self) -> Result<AccessPathExplain, BuildError> {
        Ok(AccessPathExplain {
            access_path_id: self.access_path_id,
            candidates: self.candidates,
            opened: self.opened,
            visibility_checked_before_open: self.visibility_checked_before_open,
        })
    }
}
