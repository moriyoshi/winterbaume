# Pluggable Backends

Several high-traffic services decouple their protocol handlers from persistence behind an object-safe async backend trait. The default backend is always in-memory; alternative implementations (Redis, DuckDB) live in separate crates and are injected at construction time.

## Why pluggable backends?

The default in-memory backends work well for unit and integration tests where state does not need to survive process restarts. Pluggable backends let you:

- Persist state across server restarts in long-running `winterbaume-server` deployments
- Share state between multiple server instances
- Use a query engine (DuckDB) for SQL-style services like Athena

## Services with pluggable backends

| Service | Backend trait | Redis crate | Notes |
|---|---|---|---|
| SQS | `SqsBackend` | `winterbaume-sqs-redis` | Message persistence and visibility timeouts |
| SNS | `SnsBackend` | — | In-memory only currently |
| DynamoDB | `DynamoDbBackend` | `winterbaume-dynamodb-redis` | Full item storage |
| Athena | `QueryExecutionBackend` | — | DuckDB crate for real SQL execution |
| Redshift Data | `QueryExecutionBackend` | — | Same DuckDB crate |

## Injecting a Redis backend

```toml
[dependencies]
winterbaume-sqs-redis = { path = "..." }
```

```rust
use winterbaume_sqs::SqsService;
use winterbaume_sqs_redis::RedisSqsBackend;

let backend = RedisSqsBackend::new("redis://127.0.0.1/").await.unwrap();
let sqs = SqsService::with_backend(backend);

let mock = MockAws::builder()
    .with_service(sqs)
    .build();
```

## Injecting a DuckDB query backend

```toml
[dependencies]
winterbaume-sqlengine-duckdb = { path = "..." }
```

```rust
use winterbaume_athena::AthenaService;
use winterbaume_sqlengine_duckdb::DuckDbQueryBackend;

let query_backend = DuckDbQueryBackend::new().unwrap();
let athena = AthenaService::with_query_backend(query_backend);

let mock = MockAws::builder()
    .with_service(athena)
    .build();
```

## Wiring the DuckDB SQL engine into `winterbaume-server`

When you run the prebuilt server binary, the same DuckDB connection can be shared by both Athena and Redshift Data so that a `CREATE TABLE` issued through one service is visible to the other ( mirroring how a real Glue catalogue would behave ). The server holds the connection as an `Arc<Mutex<Connection>>` and hands a clone to each service's query backend.

### When to enable

Enable the DuckDB SQL engine when you want SQL submitted via Athena's `StartQueryExecution` or Redshift Data's `ExecuteStatement` to be executed for real ( rather than returning the default empty result sets ). It is also the right choice when an integration test needs Athena and Redshift Data to observe each other's schema changes within a single server process.

### How to enable

You can configure the path to the DuckDB database file ( or `:memory:` for an in-memory database ) through any of the three usual layers. CLI flag wins over environment variable, which wins over the config file.

CLI flag:

```
winterbaume-server --sqlengine-duckdb /var/lib/winterbaume/sqlengine.duckdb
winterbaume-server --sqlengine-duckdb :memory:
```

Environment variable:

```
WB_SQLENGINE_DUCKDB=:memory: winterbaume-server
```

Config file ( TOML ):

```toml
[backends]
sqlengine_duckdb = ":memory:"
```

### Cargo features

The DuckDB SQL engine is gated behind two cargo features on `winterbaume-server`. Pick at most one when building:

- `backend-sqlengine-duckdb` — links the server against a prebuilt `libduckdb` resolved through `DUCKDB_LIB_DIR` / `DUCKDB_INCLUDE_DIR`. Fast to compile; this is what CI uses.
- `backend-sqlengine-duckdb-bundled` — implies `backend-sqlengine-duckdb` and additionally turns on the wrapper crate's `bundled` feature, which compiles DuckDB from source. No system library required, at the cost of several extra minutes on a clean build.

If `--sqlengine-duckdb` ( or its env / config equivalent ) is set on a binary that was built without either feature, the server logs a warning and falls back to the in-memory query backend.

### Running the prebuilt binary

A binary built with `backend-sqlengine-duckdb` ( the non-bundled variant CI uses ) is dynamically linked against `libduckdb.dylib` ( macOS ) / `libduckdb.so` ( Linux ) and resolves the library at runtime through the dynamic-loader search path. The build environment exports `DUCKDB_LIB_DIR` so Cargo finds the library at link time, but the resulting binary does **not** have an embedded `LC_RPATH` / `RUNPATH` baked in -- launching it outside the build environment fails with:

```
dyld[…]: Library not loaded: @rpath/libduckdb.dylib
  Reason: no LC_RPATH's found
```

Two ways to resolve, in order of preference:

1. Use the `-bundled` build for distribution. `backend-sqlengine-duckdb-bundled` statically links libduckdb into the server binary so there is nothing to load at runtime.
2. Ship the dynamic library alongside the binary and set the loader env var before launch:

   ```sh
   # macOS
   DYLD_LIBRARY_PATH="$(pwd)/duckdb-lib" ./winterbaume-server …
   # Linux
   LD_LIBRARY_PATH="$(pwd)/duckdb-lib" ./winterbaume-server …
   ```

Without DuckDB enabled at build time, neither rule applies and the binary launches normally.

### Cross-service behaviour

When a DuckDB path is configured, the server opens the database once and passes the same `Arc<Mutex<Connection>>` clone to both `DuckDbAthenaQueryBackend` and `DuckDbRedshiftQueryBackend`. Practical consequences:

- A table created via Athena is queryable through Redshift Data, and vice versa.
- Concurrent SQL submissions are serialised through the shared mutex; DuckDB's own connection is not `Sync`, so this is intentional.
- Persisting to a filesystem path keeps the catalogue across restarts; `:memory:` resets it on every server start.

## Snapshot, restore, and merge with external backends

For services that use pluggable backends, `StatefulService::snapshot()`, `restore()`, and `merge()` delegate through the backend rather than through an in-memory shadow. This means:

- Snapshotting a Redis-backed SQS service reads live queue state from Redis.
- Restoring a view onto a Redis-backed service writes that state back to Redis.
- If the backend is not running, these operations will fail.

## Adding a new backend

Implement the service's backend trait (defined in `crates/winterbaume-{service}/src/backend.rs`) and construct the service with `ServiceName::with_backend(your_backend)`. The trait is object-safe and all methods are `async`.
