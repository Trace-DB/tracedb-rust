use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CatalogClient {
    pub http_client: HttpClient,
}

impl CatalogClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_branches(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<BranchesResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/branches", None, None, options)
            .await
    }

    /// Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_databases(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<DatabasesResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/databases", None, None, options)
            .await
    }
}
