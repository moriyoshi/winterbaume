# Query Service SQL Engine Backends

## Summary

Athena and Redshift Data now support pluggable query-execution backends instead of hard-coded mock responses. The durable pattern is to keep service crates on object-safe async backend traits, then place real SQL execution in a separate driver crate such as `winterbaume-sqlengine-duckdb`, which uses `papera` for SQL transpilation and DuckDB for in-memory execution.

## Key Facts

- `winterbaume-athena` and `winterbaume-redshiftdata` now delegate query execution through backend traits.
- Default in-memory backends remain available for fast tests and simple behaviour.
- `winterbaume-sqlengine-duckdb` is a standalone crate so heavy SQL-engine dependencies stay opt-in.
- Athena uses `SourceDialect::Trino`; Redshift Data uses `SourceDialect::Redshift`.
- DuckDB metadata must be read after statement execution, not before.
- Large generated-model refreshes can expose stale handler assumptions in query-service crates quickly.

## Details

### Backend pattern for query services

The query-service pattern mirrors the SQS or SNS backend approach:

- backend traits return `Pin<Box<dyn Future<...> + Send>>`
- service handlers store `Arc<dyn *Backend>`
- `new()` uses the in-memory backend
- `with_query_backend(...)` injects an alternative execution engine

This keeps the protocol surface stable while allowing real query engines to evolve outside the service crate.

### DuckDB engine crate

`winterbaume-sqlengine-duckdb` is the concrete execution backend introduced for Athena and Redshift Data.

Architecture:

- `exec.rs` handles shared execution flow; takes `&Connection` — the caller owns the connection.
- `athena.rs` implements `DuckDbAthenaQueryBackend`; stores `Mutex<Connection>`.
- `redshift.rs` implements `DuckDbRedshiftQueryBackend`; stores `Mutex<Connection>`.
- `papera` transpiles incoming SQL to something DuckDB can execute.
- Each backend is constructed with `new(conn: Connection)` to allow the caller to seed the database.

The crate is intentionally excluded from default workspace builds because bundled DuckDB compilation is expensive.

### Injectable DuckDB Connection (2026-04-22)

Originally, both backend structs were zero-sized units and `execute_duckdb_sql` opened a fresh `Connection::open_in_memory()` per query. This made cross-query state impossible.

The backends now store `Mutex<Connection>`. Key design decisions:

- `duckdb::Connection` is `Send` but not `Sync`; `Mutex<Connection>` provides the required `Sync` bound.
- Each query clones the connection via `Connection::try_clone()`, which creates a lightweight handle to the **same underlying in-memory database** — all clones share tables and data. The mutex is held only for the duration of the clone, not the query.
- Callers can seed the database before constructing the backend:

```rust
let conn = Connection::open_in_memory().unwrap();
conn.execute_batch("CREATE TABLE users (id INT, name VARCHAR); INSERT INTO users VALUES (1, 'Alice');").unwrap();
let backend = DuckDbAthenaQueryBackend::new(conn);
```

### Metadata timing gotcha

The main implementation trap was DuckDB statement metadata. `column_count()` and related metadata access panic if called before execution. The stable pattern is:

1. prepare the statement
2. execute it with `query([])`
3. inspect metadata through the executed rows or statement reference

This is easy to forget if coming from drivers that expose metadata at prepare time.

### Regeneration fallout in query crates

The SQL-engine rollout happened while the workspace-wide codegen change was also landing. That exposed a useful maintenance pattern:

- handler breakage was mostly mechanical `Some(...)` wrapping for output structs whose required fields became optional-on-the-wire
- a few crates also depended on stale generated type names or shared custom response structs
- query-service tests are a good verification target because they exercise generated response construction heavily

Representative fixes:

- Athena wrapped `CapacityReservation`, `DataCatalog`, `NamedQuery`, and `WorkGroup` output fields in `Some(...)`
- Redshift Data wrapped list and result payload fields similarly
- the cancel-path test had to be updated because the default backend now completes queries immediately rather than leaving them in a cancelable running state

## Files

- `crates/winterbaume-athena/src/backend.rs` - `AthenaQueryBackend` and in-memory backend
- `crates/winterbaume-redshiftdata/src/backend.rs` - `RedshiftQueryBackend` and in-memory backend
- `crates/winterbaume-sqlengine-duckdb/src/exec.rs` - shared transpile-and-execute helper
- `crates/winterbaume-sqlengine-duckdb/src/athena.rs` - DuckDB Athena backend
- `crates/winterbaume-sqlengine-duckdb/src/redshift.rs` - DuckDB Redshift backend
- `crates/winterbaume-server/src/main.rs` - `--sqlengine-duckdb`, `WB_SQLENGINE_DUCKDB`, and `[backends] sqlengine_duckdb` server wiring
- `docs/guide/pluggable-backends.md` and `docs/developer/backends.md` - user and developer documentation for the DuckDB SQL-engine backend
- `Cargo.toml` - workspace `default-members` excludes the DuckDB crate from default builds

## Test Coverage

- `cargo test -p winterbaume-athena` passed after the backend extraction and model-wrap fixes
- `cargo test -p winterbaume-redshiftdata` passed after the same migration
- `cargo test -p winterbaume-sqlengine-duckdb` passed with focused SQL-engine backend tests
- Server-facing DuckDB documentation names were cross-checked against `winterbaume-server/src/main.rs` and `Cargo.toml`; end-to-end server startup with `WB_SQLENGINE_DUCKDB=:memory:` remains tracked in `TODO.md` as `duckdb-server-integration-test`.

### Glue catalogue metadata gap

In real AWS, Athena's default data catalogue is the Glue Data Catalogue. Most users create Glue databases and tables, then query them via Athena. Winterbaume does not yet emulate this relationship:

- `winterbaume-athena` accepts `DataCatalogType::Glue` when creating data catalogues but treats it as inert metadata.
- `StartQueryExecution` does not resolve database or table schemas from `winterbaume-glue` state, even when the target catalogue is of type `Glue`.
- The DuckDB backend (`winterbaume-sqlengine-duckdb`) executes SQL without any awareness of Glue-managed metadata.
- No cross-service integration tests exist that create Glue databases and tables, then verify Athena queries resolve schemas from them.

Closing this gap requires: (1) a catalogue-resolution layer that queries `winterbaume-glue` state when the target catalogue type is `Glue`, (2) a schema-injection mechanism so the query backend receives table definitions, and (3) cross-service integration tests.

### Prebuilt libduckdb in CI ( 2026-04-28 )

`winterbaume-sqlengine-duckdb` originally pinned `duckdb = { version = "1", features = ["bundled"] }` workspace-wide, so each CI run rebuilt DuckDB from source ( ~5-7 minutes per `clippy` / `test` job ). The shape that landed:

- Workspace dep is now `duckdb = { version = "1" }` ( no `bundled` ). The wrapper crate re-exposes the toggle as its own `bundled` default feature ( `bundled = ["duckdb/bundled"]` ), so `cargo build -p winterbaume-sqlengine-duckdb` keeps working out of the box for local devs.
- New composite action `.github/actions/setup-duckdb` downloads `libduckdb-linux-amd64.zip` from the upstream DuckDB release, verifies it against a pinned SHA-256 ( `21aec66a60eae1696270ba715a481ab066a88d99a62718d0577579ac1a7a4834` for v1.5.1, matching crate version `1.10501.0` ), unzips it, and exports `DUCKDB_LIB_DIR` / `DUCKDB_INCLUDE_DIR` / `LD_LIBRARY_PATH` via `$GITHUB_ENV`.
- `clippy` and `test` jobs in `.github/workflows/ci.yml` now run the workspace with `--exclude winterbaume-sqlengine-duckdb`, then run the wrapper crate separately with `--no-default-features` so duckdb-rs links against the prebuilt instead of compiling.

Local timings ( macOS, prebuilt `libduckdb-osx-universal.zip` ):

- `cargo test -p winterbaume-sqlengine-duckdb --no-run` baseline ( bundled ): 5m 58s
- `cargo test -p winterbaume-sqlengine-duckdb --no-default-features --no-run` ( prebuilt ): 44.72s

Bump procedure: when `duckdb` crate version moves, also bump the libduckdb URL ( and SHA-256 ) in `.github/actions/setup-duckdb`. The two are coupled — version mismatch will surface as a runtime symbol error, not a build error.

#### `--no-default-features` semantics on the wrapper

Cargo has no flag that disables a single feature once activated. `--features` only adds. The standard idiom for opting out of a default is `--no-default-features` ( drop them all ) followed by `--features ...` to re-add the keepers. Today the wrapper's only default is `bundled`, so `--no-default-features` is precisely "turn off `bundled`". Walking the resulting feature graph from the wrapper crate downward:

- `winterbaume-sqlengine-duckdb`: `bundled` OFF.
- `duckdb v1.10501.0`: `default = []` already, so unchanged. Without the wrapper enabling `duckdb/bundled`, no extra feature on duckdb.
- `libduckdb-sys`: `bundled` OFF → its `build.rs` reads `DUCKDB_LIB_DIR` / `DUCKDB_INCLUDE_DIR` and links the prebuilt.

`--no-default-features` does NOT propagate into dependencies' default features ( cargo only applies it to the `-p` selection ), so other crates' defaults are untouched. Footgun: if a second default feature is ever added to `winterbaume-sqlengine-duckdb`, `--no-default-features` in CI will silently drop it. Either keep `bundled` as the only default by convention, or spell the keepers out: `--no-default-features --features <other-defaults>`.

#### Why cargo-dist release builds still get bundled DuckDB

Verified by tracing the release path end to end:

- `dist-workspace.toml` lists exactly one dist member ( `cargo:.` ), the umbrella `winterbaume` crate. cargo-dist actually packages `winterbaume-server` ( the only crate with a `[[bin]]` ).
- `crates/winterbaume-server/Cargo.toml` has no dependency on `winterbaume-sqlengine-duckdb`, direct or transitive.
- `.github/workflows/release.yml` never passes `--no-default-features` or `--features` to cargo. `dist build` runs with default features.

Even if `winterbaume-server` ( or any future dist member ) starts pulling in the duckdb wrapper crate, the wrapper's `default = ["bundled"]` ensures bundled DuckDB is on by default for every consumer that does not explicitly opt out. Only the two CI jobs ( `clippy`, `test` ) opt out — release builds are unaffected.

### `winterbaume-server` Wire-Up ( 2026-04-29 )

Following the prebuilt-libduckdb CI work, the standalone server gained runtime access to DuckDB-backed Athena / Redshift Data services:

- `crates/winterbaume-server/Cargo.toml` defines two paired features mirroring the existing `backend-sqs-redis` / `backend-dynamodb-redis` / `backend-vfs-fs` pattern:
  - `backend-sqlengine-duckdb` — pulls in `winterbaume-sqlengine-duckdb` linked against a prebuilt libduckdb ( `DUCKDB_LIB_DIR` / `DUCKDB_INCLUDE_DIR` must be set at build time ).
  - `backend-sqlengine-duckdb-bundled` — supersets the above plus activates the wrapper's `bundled` feature so DuckDB is compiled from source. Build is several minutes slower but needs no system libraries.
  Neither is on by default; the binary stays light when nobody asks for SQL execution.
- `crates/winterbaume-server/src/main.rs` adds config knob `[backends] sqlengine_duckdb`, env var `WB_SQLENGINE_DUCKDB`, CLI flag `--sqlengine-duckdb PATH`. The value is either `:memory:` ( or empty ) for in-memory, or a filesystem path to a DuckDB file.
- `register_all_services` opens **one** DuckDB connection when the option is set + the feature is on, then injects it into both `AthenaService::with_query_backend` and `RedshiftDataService::with_query_backend`. A single shared `Arc<Mutex<Connection>>` means a `CREATE TABLE` issued through Athena is visible to Redshift Data and vice versa — closer to how a real Glue catalog ties them together than two isolated databases would be.
- When the option is set but the feature is off, prints a `tracing::warn!` and falls back to the in-memory backend, matching the SQS / DynamoDB / VFS warn-and-degrade behaviour.
- `crates/winterbaume-sqlengine-duckdb/src/lib.rs` gains `pub fn open_database(path: &str) -> DuckDbResult<Arc<Mutex<Connection>>>` plus re-exports of `Connection`, `Error`, `Result` so callers do not need their own `duckdb` dep.

#### Cargo workspace dep gotcha

Initial attempt declared the dep as `winterbaume-sqlengine-duckdb = { workspace = true, optional = true, default-features = false }`. Cargo rejected it:

```
`default-features = false` cannot override workspace's `default-features`
```

A workspace dependency entry that omits `default-features` is treated as an implicit `true`, and Cargo currently forbids member crates from flipping that to `false` ( the override is allowed only in the `false → true` direction ). The chosen fix: set `default-features = false` at the workspace level. Workspace inheritors then start with no defaults; each member crate explicitly opts in to whatever wrapper features it wants. The wrapper's own `default = ["bundled"]` is unaffected for direct `cargo build -p winterbaume-sqlengine-duckdb` — that path does not go through the workspace dependency entry. Only inheritors via `workspace = true` see `default-features = false`, which matches the desired behaviour: the server pins what it wants, the wrapper alone keeps working out of the box for local devs. If a second default feature is ever added to the wrapper, the server's feature wiring will need to enumerate the keepers explicitly.

### Binary E2E Findings and Failure Detail ( 2026-04-30 / 2026-05-01 )

Black-box AWS CLI testing against a prebuilt `winterbaume-server` binary confirmed that `--sqlengine-duckdb :memory:` drives real DuckDB execution through the public Athena API. A `CREATE TABLE` followed by `SELECT` returned real rows and DuckDB type names, and Trino-only syntax such as `CARDINALITY(ARRAY[...])` succeeded through `papera::SourceDialect::Trino`.

Two operational details matter for future verification:

- The rebuilt binary may live under `.agents-workspace/tmp/target-<session>/debug/winterbaume-server`, not the user's `target/debug/winterbaume-server`. Use the session build product when validating recent fixes.
- On macOS, a binary linked to prebuilt `libduckdb.dylib` may fail with `dyld: Library not loaded: @rpath/libduckdb.dylib` unless `DYLD_LIBRARY_PATH` points at the directory containing the dylib. Linux uses the analogous `LD_LIBRARY_PATH`. Distribution builds should prefer `backend-sqlengine-duckdb-bundled` if the dylib will not be shipped alongside the binary.

The same E2E run found that failed DuckDB queries were returned as `FAILED` without `Status.StateChangeReason` or `AthenaError`. The handler fix now maps stored backend error text into both fields, using an Athena user-error category for DuckDB query-shape failures. This keeps failure diagnostics visible through normal Athena polling.

## Pitfalls

- Heavy SQL-engine crates should stay opt-in; do not make default workspace builds depend on bundled DuckDB.
- Query-service tests may need expectation updates when the default backend changes lifecycle semantics, such as immediate completion versus long-running statements.
- Generated response types can shift underneath query handlers after codegen changes; treat those as generator-adoption tasks, not one-off local hacks.
- Do not assume Athena query execution is correct just because SQL runs against DuckDB. Without Glue catalogue resolution, queries that reference Glue-managed tables will fail or return wrong results compared to real AWS.
- Do not bump `duckdb` crate version in isolation. The libduckdb URL and SHA-256 in `.github/actions/setup-duckdb` must move in lockstep — version mismatch surfaces as a runtime symbol error, not a build error.
- Do not flip `default-features = false` at a workspace dependency entry that previously omitted it ( implicit `true` ); Cargo treats the override as forbidden. Set it at the workspace level instead, and let inheritors opt in explicitly.
- Do not mix bundled and prebuilt-libduckdb feature sets in the same local session — `tools/sccache-wrapper` does not always discriminate cache entries between two builds of the same crate where the active feature set differs, and you can resurrect stale `chrono` / `serde` artefacts producing `E0463: can't find crate for serde`. Wipe `.agents-workspace/tmp/winterbaume-rustc-cache/` and the session `target-*` directory when switching feature sets locally. CI is unaffected — each run starts from a clean cache.
- Do not assume `--no-default-features` will keep a future second default feature alive in CI. It silently drops every default. Either keep `bundled` as the wrapper's only default by convention, or spell the keepers out explicitly with `--no-default-features --features <keepers>`.
- Do not validate SQL-engine fixes against a stale `target/debug/winterbaume-server`; agent builds use per-session target directories.
- Do not hide query failure detail. If a backend stores an error string, Athena responses should surface it through `StateChangeReason` and `AthenaError`.
