pub use crate::prelude::*;

/// Admin job queue response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct JobsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<AdminJob>>,
}

impl JobsResponse {
    pub fn builder() -> JobsResponseBuilder {
        <JobsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JobsResponseBuilder {
    jobs: Option<Vec<AdminJob>>,
}

impl JobsResponseBuilder {
    pub fn jobs(mut self, value: Vec<AdminJob>) -> Self {
        self.jobs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JobsResponse`].
    pub fn build(self) -> Result<JobsResponse, BuildError> {
        Ok(JobsResponse { jobs: self.jobs })
    }
}
