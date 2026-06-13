use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct RecordsClient {
    pub http_client: HttpClient,
}

impl RecordsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Deprecated. Use POST /v1/records/put instead. This route remains for backwards compatibility and will be removed in a future release.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn post_insert(
        &self,
        request: &RecordInput,
        options: Option<RequestOptions>,
    ) -> Result<EpochResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/insert",
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
    pub async fn post_records_delete(
        &self,
        request: &RecordDeleteRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeleteResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/records/delete",
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
    pub async fn post_records_get(
        &self,
        request: &RecordGetRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetRecordResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/records/get",
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
    pub async fn post_records_patch(
        &self,
        request: &RecordPatchRequest,
        options: Option<RequestOptions>,
    ) -> Result<EpochResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/records/patch",
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
    pub async fn post_records_put(
        &self,
        request: &RecordPutBody,
        options: Option<RequestOptions>,
    ) -> Result<EpochResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/records/put",
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
    pub async fn post_records_put_batch(
        &self,
        request: &RecordPutBatchRequest,
        options: Option<RequestOptions>,
    ) -> Result<PutBatchResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/records/put-batch",
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
    pub async fn post_records_scan(
        &self,
        request: &RecordScanRequest,
        options: Option<RequestOptions>,
    ) -> Result<RecordScanOutput, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/records/scan",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
