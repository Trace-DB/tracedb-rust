# Reference
## admin
<details><summary><code>client.tracedb().admin.<a href="/src/api/resources/tracedb/admin/client.rs">post_admin_compact</a>(request: EmptyObject) -> Result&lt;CompactResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .tracedb
        .admin
        .post_admin_compact(
            &EmptyObject(HashMap::from([(
                "key".to_string(),
                serde_json::json!("value"),
            )])),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tracedb().admin.<a href="/src/api/resources/tracedb/admin/client.rs">get_admin_jobs</a>(database_id: Option&lt;Option&lt;String&gt;&gt;, branch_id: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;JobsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .tracedb
        .admin
        .get_admin_jobs(
            &GetAdminJobsQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**database_id:** `Option<String>` — Canonical managed-routing database identifier for bodyless routes. SDKs must use this parameter name (not db_id, databaseId, or similar variants) so all SDKs target the same gateway routing key.
    
</dd>
</dl>

<dl>
<dd>

**branch_id:** `Option<String>` — Canonical managed-routing branch identifier for bodyless routes. SDKs must use this parameter name (not br_id, branchId, or similar variants) so all SDKs target the same gateway routing key.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tracedb().admin.<a href="/src/api/resources/tracedb/admin/client.rs">post_admin_restore</a>(request: RestoreRequest) -> Result&lt;RestoreResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .tracedb
        .admin
        .post_admin_restore(
            &RestoreRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**source:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**target:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**verify_record:** `Option<RecordGetRequest>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tracedb().admin.<a href="/src/api/resources/tracedb/admin/client.rs">post_admin_snapshot</a>(request: SnapshotRequest) -> Result&lt;SnapshotResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .tracedb
        .admin
        .post_admin_snapshot(
            &SnapshotRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**target:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## catalog
<details><summary><code>client.tracedb().catalog.<a href="/src/api/resources/tracedb/catalog/client.rs">get_branches</a>() -> Result&lt;BranchesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.tracedb.catalog.get_branches(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tracedb().catalog.<a href="/src/api/resources/tracedb/catalog/client.rs">get_databases</a>() -> Result&lt;DatabasesResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.tracedb.catalog.get_databases(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## query
<details><summary><code>client.tracedb().query.<a href="/src/api/resources/tracedb/query/client.rs">post_explain</a>(request: HybridQuery) -> Result&lt;HybridExplain, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .tracedb
        .query
        .post_explain(
            &HybridQuery {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tracedb().query.<a href="/src/api/resources/tracedb/query/client.rs">post_graphql</a>(request: GraphQlQueryRequest) -> Result&lt;GraphQlResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .tracedb
        .query
        .post_graphql(
            &GraphQlQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tracedb().query.<a href="/src/api/resources/tracedb/query/client.rs">post_graphql_bounded</a>(request: GraphQlQueryRequest) -> Result&lt;QueryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .tracedb
        .query
        .post_graphql_bounded(
            &GraphQlQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tracedb().query.<a href="/src/api/resources/tracedb/query/client.rs">get_graphql_schema</a>() -> Result&lt;GraphQlSchemaResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.tracedb.query.get_graphql_schema(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tracedb().query.<a href="/src/api/resources/tracedb/query/client.rs">post_query</a>(request: HybridQuery) -> Result&lt;QueryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .tracedb
        .query
        .post_query(
            &HybridQuery {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tracedb().query.<a href="/src/api/resources/tracedb/query/client.rs">post_traceql</a>(request: TraceQlQueryRequest) -> Result&lt;QueryResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .tracedb
        .query
        .post_traceql(
            &TraceQlQueryRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**query:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## health
<details><summary><code>client.tracedb().health.<a href="/src/api/resources/tracedb/health/client.rs">get_health</a>() -> Result&lt;HealthResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.tracedb.health.get_health(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## records
<details><summary><code>client.tracedb().records.<a href="/src/api/resources/tracedb/records/client.rs">post_insert</a>(request: RecordInput) -> Result&lt;EpochResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deprecated. Use POST /v1/records/put instead. This route remains for backwards compatibility and will be removed in a future release.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .tracedb
        .records
        .post_insert(
            &RecordInput {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tracedb().records.<a href="/src/api/resources/tracedb/records/client.rs">post_records_delete</a>(request: RecordDeleteRequest) -> Result&lt;DeleteResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .tracedb
        .records
        .post_records_delete(
            &RecordDeleteRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**table:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**tenant_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**tombstone:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tracedb().records.<a href="/src/api/resources/tracedb/records/client.rs">post_records_get</a>(request: RecordGetRequest) -> Result&lt;GetRecordResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .tracedb
        .records
        .post_records_get(
            &RecordGetRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tracedb().records.<a href="/src/api/resources/tracedb/records/client.rs">post_records_patch</a>(request: RecordPatchRequest) -> Result&lt;EpochResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .tracedb
        .records
        .post_records_patch(
            &RecordPatchRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**fields:** `Option<std::collections::HashMap<String, serde_json::Value>>` — Patch field map.
    
</dd>
</dl>

<dl>
<dd>

**id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**table:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**tenant_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tracedb().records.<a href="/src/api/resources/tracedb/records/client.rs">post_records_put</a>(request: RecordPutBody) -> Result&lt;EpochResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .tracedb
        .records
        .post_records_put(
            &RecordPutBody::RecordInput(RecordInput {
                ..Default::default()
            }),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tracedb().records.<a href="/src/api/resources/tracedb/records/client.rs">post_records_put_batch</a>(request: RecordPutBatchRequest) -> Result&lt;PutBatchResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .tracedb
        .records
        .post_records_put_batch(
            &RecordPutBatchRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**include_write_timing:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**records:** `Option<Vec<RecordInput>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.tracedb().records.<a href="/src/api/resources/tracedb/records/client.rs">post_records_scan</a>(request: RecordScanRequest) -> Result&lt;RecordScanOutput, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .tracedb
        .records
        .post_records_scan(
            &RecordScanRequest {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**cursor:** `Option<Option<String>>` — Opaque cursor returned by the previous scan page.
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**table:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**tenant_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## metrics
<details><summary><code>client.tracedb().metrics.<a href="/src/api/resources/tracedb/metrics/client.rs">get_metrics_public_safe</a>() -> Result&lt;MetricsResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.tracedb.metrics.get_metrics_public_safe(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## readiness
<details><summary><code>client.tracedb().readiness.<a href="/src/api/resources/tracedb/readiness/client.rs">get_ready</a>() -> Result&lt;ReadyResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.tracedb.readiness.get_ready(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## schema
<details><summary><code>client.tracedb().schema.<a href="/src/api/resources/tracedb/schema/client.rs">post_schema_apply</a>(request: TableSchema) -> Result&lt;EpochResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Current TraceDB v1 product route. This OpenAPI artifact is generated from the checked-in route manifest.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use trace_db_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        token: Some("<token>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .tracedb
        .schema
        .post_schema_apply(
            &TableSchema {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**name:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**primary_id_column:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**scalar_columns:** `Option<Vec<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**tenant_id_column:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**text_indexed_columns:** `Option<Vec<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**vector_columns:** `Option<Vec<std::collections::HashMap<String, serde_json::Value>>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

