pub use crate::prelude::*;

/// Admin job queue entry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdminJob {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl AdminJob {
    pub fn builder() -> AdminJobBuilder {
        <AdminJobBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdminJobBuilder {
    queue: Option<String>,
    state: Option<String>,
}

impl AdminJobBuilder {
    pub fn queue(mut self, value: impl Into<String>) -> Self {
        self.queue = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdminJob`].
    pub fn build(self) -> Result<AdminJob, BuildError> {
        Ok(AdminJob {
            queue: self.queue,
            state: self.state,
        })
    }
}
