use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AdminClient {
    pub http_client: HttpClient,
}

impl AdminClient {
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
    pub async fn post_admin_compact(
        &self,
        request: &EmptyObject,
        options: Option<RequestOptions>,
    ) -> Result<CompactResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/admin/compact",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
    ///
    /// # Arguments
    ///
    /// * `database_id` - Canonical managed-routing database identifier for bodyless routes. SDKs must use this parameter name (not db_id, databaseId, or similar variants) so all SDKs target the same gateway routing key.
    /// * `branch_id` - Canonical managed-routing branch identifier for bodyless routes. SDKs must use this parameter name (not br_id, branchId, or similar variants) so all SDKs target the same gateway routing key.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_admin_jobs(
        &self,
        request: &GetAdminJobsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<JobsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "v1/admin/jobs",
                None,
                QueryBuilder::new()
                    .string("database_id", request.database_id.clone())
                    .string("branch_id", request.branch_id.clone())
                    .build(),
                options,
            )
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
    pub async fn post_admin_restore(
        &self,
        request: &RestoreRequest,
        options: Option<RequestOptions>,
    ) -> Result<RestoreResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/admin/restore",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
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
    pub async fn post_admin_snapshot(
        &self,
        request: &SnapshotRequest,
        options: Option<RequestOptions>,
    ) -> Result<SnapshotResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/admin/snapshot",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
