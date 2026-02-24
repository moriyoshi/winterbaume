# Pluggable Backends

Several winterbaume services separate their protocol handlers from their storage or query-execution engines behind an object-safe async backend trait. The default backend is always in-memory; alternative implementations live in separate crates and are injected at construction time.

## Design rules

- The backend trait lives in the service crate so the service owns its protocol-to-backend contract.
- The trait must be object-safe: async methods return `Pin<Box<dyn Future<...> + Send>>`.
- `new()` creates the service with the default in-memory adapter.
- `with_backend(impl Backend)` (or `with_query_backend(...)`) is the stable injection point.
- Heavy dependencies (Redis, DuckDB, native libs) live in separate crates. Core service crates stay dependency-light.
- When `StatefulService` is implemented, the backend is the authoritative owner of durable state. `snapshot()`, `restore()`, and `merge()` delegate through the backend — not through a hidden in-memory shadow.

## Storage backends

### SQS (`SqsBackend`)

Defined in `crates/winterbaume-sqs/src/backend.rs`.

```rust
pub trait SqsBackend: Send + Sync + 'static {
    async fn create_queue(&self, params: CreateQueueParams) -> Result<QueueInfo>;
    async fn get_queue(&self, url: &str) -> Result<Option<QueueInfo>>;
    async fn send_message(&self, url: &str, msg: Message) -> Result<String>;
    async fn receive_messages(&self, url: &str, max: u32, visibility_timeout: u32)
        -> Result<Vec<Message>>;
    async fn delete_message(&self, url: &str, receipt_handle: &str) -> Result<()>;
    // ... full queue management surface
}
```

In-memory adapter: `InMemorySqsBackend` (default, in `winterbaume-sqs`).
Redis adapter: `RedisSqsBackend` (in `winterbaume-sqs-redis`).

```rust
use winterbaume_sqs::SqsService;
use winterbaume_sqs_redis::RedisSqsBackend;

let backend = RedisSqsBackend::new("redis://127.0.0.1/").await?;
let sqs = SqsService::with_backend(Arc::new(backend));
```

### DynamoDB (`DynamoDbBackend`)

Defined in `crates/winterbaume-dynamodb/src/backend.rs`.

The DynamoDB backend owns item storage, table metadata, and GSI/LSI index state. PartiQL execution runs above the backend via `execute_partiql_via_backend(...)` — it is not a backend method itself.

```rust
use winterbaume_dynamodb::DynamoDbService;
use winterbaume_dynamodb_redis::RedisDynamoDbBackend;

let backend = RedisDynamoDbBackend::from_url("redis://127.0.0.1/").await?;
let ddb = DynamoDbService::with_backend(Arc::new(backend));
```

### VFS / BlobStore (S3, EBS, Glacier)

S3, EBS, and Glacier use the `Vfs` abstraction from `winterbaume-core` rather than a service-owned backend trait. Pass a shared `Arc<dyn Vfs>` at construction:

```rust
use winterbaume_core::FsVfs;
use winterbaume_s3::S3Service;
use winterbaume_glacier::GlacierService;
use winterbaume_ebs::EbsService;

let vfs = Arc::new(FsVfs::new("/var/lib/winterbaume/blobs")?);

let s3      = S3Service::with_vfs(Arc::clone(&vfs));
let glacier = GlacierService::with_vfs(Arc::clone(&vfs));
let ebs     = EbsService::with_vfs(Arc::clone(&vfs));
```

Sharing the `Arc<dyn Vfs>` is intentional: all three services can coexist in the same storage directory without key collisions because each service namespaces its blob keys internally.

## Query-execution backends

### Athena (`AthenaQueryBackend`)

Defined in `crates/winterbaume-athena/src/backend.rs`.

The default backend returns empty result sets. The DuckDB backend executes real SQL:

```rust
use winterbaume_athena::AthenaService;
use winterbaume_sqlengine_duckdb::DuckDbAthenaBackend;

let backend = DuckDbAthenaBackend::new()?;
let athena = AthenaService::with_query_backend(Arc::new(backend));
```

### Redshift Data (`RedshiftQueryBackend`)

Defined in `crates/winterbaume-redshiftdata/src/backend.rs`. Uses the same DuckDB crate:

```rust
use winterbaume_redshiftdata::RedshiftDataService;
use winterbaume_sqlengine_duckdb::DuckDbRedshiftBackend;

let backend = DuckDbRedshiftBackend::new()?;
let redshift_data = RedshiftDataService::with_query_backend(Arc::new(backend));
```

### Server wire-up ( shared DuckDB connection )

`winterbaume-server` exposes a single configuration knob that wires the DuckDB SQL engine into both Athena and Redshift Data simultaneously. The server opens the database once via `winterbaume_sqlengine_duckdb::open_database(path)`, which returns an `Arc<Mutex<Connection>>`, and clones the `Arc` into each service's query backend ( `DuckDbAthenaQueryBackend::new(conn)` and `DuckDbRedshiftQueryBackend::new(conn)` ). Both services therefore execute against the same catalogue and the same data files.

Configuration sources, highest precedence first:

| Source | Form |
|---|---|
| CLI flag | `--sqlengine-duckdb <PATH>` ( accepts `:memory:` ) |
| Environment | `WB_SQLENGINE_DUCKDB=<PATH>` |
| Config file | `[backends] sqlengine_duckdb = "<PATH>"` |

Cargo features on `winterbaume-server`:

- `backend-sqlengine-duckdb` — declares `dep:winterbaume-sqlengine-duckdb`. Links against a prebuilt `libduckdb` discovered via `DUCKDB_LIB_DIR` / `DUCKDB_INCLUDE_DIR`. Used by CI.
- `backend-sqlengine-duckdb-bundled` — implies `backend-sqlengine-duckdb` and additionally enables the wrapper crate's `bundled` feature, which compiles DuckDB from source. No system library required; significantly slower clean build.

If the value is set but neither feature was compiled in, the server emits a `tracing::warn!` and continues with the default in-memory query backends. Failing to open the database ( bad path, permissions ) is fatal and the server exits non-zero.

The injection mirrors the SQS / DynamoDB / VFS pattern: the CLI parser builds a `BackendOptions` struct, the wiring code feature-gates each backend behind `#[cfg(feature = "backend-...")]`, and a single `Option<Arc<Mutex<Connection>>>` is reused across both Athena and Redshift Data construction sites in `crates/winterbaume-server/src/main.rs`.

## Implementing a new backend

To add a new storage backend for an existing service:

1. **Implement the trait** from the service crate. The trait is in `crates/winterbaume-{svc}/src/backend.rs`.

2. **Create a new crate** (e.g. `crates/winterbaume-{svc}-mystore/`) with heavy dependencies isolated there. Add it to the workspace `[workspace.members]` list.

3. **Implement the trait methods.** All async methods must return `Pin<Box<dyn Future<...> + Send + 'static>>` — the object-safety constraint comes from the service crate's trait definition.

4. **Implement `StatefulService` delegation.** If the service implements `StatefulService`, the backend should implement snapshot, restore, and merge. The service's `StatefulService` impl will delegate to the backend rather than to an in-memory copy.

5. **Wire into the server (optional).** Add a feature flag to `winterbaume-server/Cargo.toml` and handle the new backend option in `main.rs`, following the SQS/DynamoDB patterns.

## State ownership with backends

When a backend is present, it is the single source of truth for durable state:

```
StatefulService::snapshot()
  → service.backend.snapshot(account, region)
  → backend reads from Redis / DuckDB / etc.
  → returns StateView

StatefulService::restore(view)
  → service.backend.restore(account, region, view)
  → backend writes to Redis / DuckDB / etc.
```

A service that maintains a separate in-memory shadow alongside a backend will have divergent state after a restore. Always route snapshot/restore/merge through the backend.
