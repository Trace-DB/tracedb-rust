use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct QueryClient {
    pub http_client: HttpClient,
}

impl QueryClient {
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
    pub async fn post_explain(
        &self,
        request: &HybridQuery,
        options: Option<RequestOptions>,
    ) -> Result<HybridExplain, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/explain",
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
    pub async fn post_graphql(
        &self,
        request: &GraphQlQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<GraphQlResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/graphql",
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
    pub async fn post_graphql_bounded(
        &self,
        request: &GraphQlQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/graphql/bounded",
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
    pub async fn get_graphql_schema(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<GraphQlSchemaResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/graphql/schema", None, None, options)
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
    pub async fn post_query(
        &self,
        request: &HybridQuery,
        options: Option<RequestOptions>,
    ) -> Result<QueryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/query",
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
    pub async fn post_traceql(
        &self,
        request: &TraceQlQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "v1/traceql",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
