use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient};

pub mod admin;
pub use admin::AdminClient;
pub mod catalog;
pub use catalog::CatalogClient;
pub mod query;
pub use query::QueryClient;
pub mod health;
pub use health::HealthClient;
pub mod records;
pub use records::RecordsClient;
pub mod metrics;
pub use metrics::MetricsClient;
pub mod readiness;
pub use readiness::ReadinessClient;
pub mod schema;
pub use schema::SchemaClient;
pub struct TracedbClient {
    pub http_client: HttpClient,
    pub admin: AdminClient,
    pub catalog: CatalogClient,
    pub query: QueryClient,
    pub health: HealthClient,
    pub records: RecordsClient,
    pub metrics: MetricsClient,
    pub readiness: ReadinessClient,
    pub schema: SchemaClient,
}

impl TracedbClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            admin: AdminClient::new(config.clone())?,
            catalog: CatalogClient::new(config.clone())?,
            query: QueryClient::new(config.clone())?,
            health: HealthClient::new(config.clone())?,
            records: RecordsClient::new(config.clone())?,
            metrics: MetricsClient::new(config.clone())?,
            readiness: ReadinessClient::new(config.clone())?,
            schema: SchemaClient::new(config.clone())?,
        })
    }
}
