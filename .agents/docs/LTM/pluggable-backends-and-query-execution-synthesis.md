# Pluggable Backends and Query Execution Synthesis

## Summary

Winterbaume now has a reusable pattern for decoupling protocol handlers from storage and execution engines. The stable architecture is object-safe async backend traits in the service crate, lightweight in-memory adapters for the default path, and optional external engine crates for Redis-backed persistence or DuckDB-backed query execution. The main caveat is no longer that PartiQL bypasses the abstraction; instead, the durable boundary is that higher-level execution flows such as PartiQL should stay above the backend trait and decompose into backend primitives rather than being forced into the storage interface itself. As of 2026-04-30, the PartiQL parser is hand-rolled ( replacing `partiql-parser` 0.14 + lalrpop ), runtime validation aligns previously-permissive arithmetic and EXISTS paths with AWS behaviour, SQL comments lex correctly, and the DuckDB engine is wired into `winterbaume-server` behind paired prebuilt / bundled feature flags.

## Included Documents

| Document | Focus |
|----------|-------|
| [pluggable-service-backends-and-redis-storage.md](./pluggable-service-backends-and-redis-storage.md) | Backend-trait pattern for SQS, SNS, and DynamoDB, plus Redis storage design |
| [query-service-sql-engine-backends.md](./query-service-sql-engine-backends.md) | Query-execution backends for Athena and Redshift Data using DuckDB and papera |
| [dynamodb-partiql-integration.md](./dynamodb-partiql-integration.md) | PartiQL parser, typed `AttributeValue` model, backend-aware execution, and stream capture |

## Stable Knowledge

- Backend traits should stay object-safe and async through `Pin<Box<dyn Future<...> + Send>>`, not by making service handlers own a specific runtime or storage implementation.
- The default path should remain simple: core service crates keep an in-memory backend and avoid optional Redis or SQL-engine dependencies.
- External engines belong in separate crates such as `winterbaume-sqs-redis`, `winterbaume-dynamodb-redis`, and `winterbaume-sqlengine-duckdb`.
- `with_backend(...)` or `with_query_backend(...)` constructors are the stable integration points for swapping implementations.
- Backend ownership matters for state fidelity. Snapshot, restore, and merge must route through the backend once alternative persistence is supported.
- Storage backends and query backends are the same architectural move at different layers: one replaces persistence, the other replaces execution.
- PartiQL is still the clearest abstraction boundary, but now in a positive sense: execution runs through `execute_partiql_via_backend(...)`, which decomposes a higher-level language into existing backend primitives without making PartiQL itself a backend-trait method.
- Typed DynamoDB `AttributeValue` storage and real stream change capture made the DynamoDB boundary more coherent: the same typed item model now drives normal APIs, PartiQL, and DynamoDB Streams.
- DynamoDB secondary-index support is now part of the backend contract. `create_table()` carries GSI/LSI definitions, `query()` accepts `index_name: Option<String>`, and custom backends such as `winterbaume-dynamodb-redis` must persist secondary-index definitions with backward-compatible defaults.
- The PartiQL parser is now hand-rolled inside `winterbaume-partiql/src/parser/` — `partiql-parser` 0.14 and `partiql-ast` were dropped in 2026-04-28. Removing them eliminated the lalrpop build script ( ~30 minutes per fresh `CARGO_TARGET_DIR` ) and ~50 transitive crates. Direct deps are now `serde_json` and `thiserror`. The IR is `Expression` ( Literal / Path / BinaryOp / Neg ) + `ArithOp` + `CmpOp` + `Condition::Compare(Expr, CmpOp, Expr)`, validated empirically against real AWS DynamoDB. The lexer skips both `-- ...` line comments and `/* ... */` block comments. Do not re-introduce the upstream crates.
- `EXISTS(SELECT …)` is a top-level `DdbOperation::Exists(Box<SelectOp>)`, **not** a Condition variant — AWS rejects EXISTS in any expression position ( SELECT projection, WHERE clause ). Only `handle_execute_transaction` accepts it; `handle_execute_statement` and `handle_batch_execute_statement` reject it with `ValidationException` ( exact wording: "EXISTS can only be used in ExecuteTransaction write requests." ). The inner SELECT must specify the full primary key by Eq AND at least one non-key predicate; key-only EXISTS is rejected with "must contain a single item read with additional condition".
- MockAws keeps the parser intentionally permissive for general IR coverage, but DynamoDB handlers run `winterbaume-partiql/src/validate.rs` before execution. That runtime walk rejects arithmetic in condition contexts, unary `-path`, and misplaced EXISTS with AWS-exact messages while preserving the parser's ability to represent the broader syntax.
- Query-engine crates should be opt-in when they compile heavy native dependencies. The DuckDB crate is intentionally excluded from workspace default-members for this reason.
- Cross-service catalogue resolution is a known architectural boundary. Athena's `AthenaQueryBackend` trait and the DuckDB backend both operate without awareness of Glue metadata. In real AWS, the default Athena catalogue is the Glue Data Catalogue, so bridging this gap requires a resolution layer above or alongside the backend trait that consults `winterbaume-glue` state for database and table schemas.
- The DuckDB backend structs are now injectable: both `DuckDbAthenaQueryBackend` and `DuckDbRedshiftQueryBackend` store `Mutex<Connection>` and expose `new(conn: Connection)`. Per-query execution clones the connection via `Connection::try_clone()`, which opens a new lightweight handle to the **same underlying in-memory database** — all handles share tables and data. The mutex is held only during the clone, not during the query itself. This enables seeding the database with schema or data before constructing the backend.
- DuckDB is opt-in at three layers and they must move together: ( 1 ) the workspace dep is `duckdb = { version = "1" }` ( no `bundled` ); ( 2 ) the wrapper crate `winterbaume-sqlengine-duckdb` re-exposes the toggle as its own `bundled` default feature; ( 3 ) the `winterbaume-server` crate exposes paired `backend-sqlengine-duckdb` ( prebuilt libduckdb via `DUCKDB_LIB_DIR` / `DUCKDB_INCLUDE_DIR` ) and `backend-sqlengine-duckdb-bundled` features, neither on by default. CI uses prebuilt libduckdb pinned by SHA-256 in `.github/actions/setup-duckdb`; the libduckdb URL+digest must be bumped together when the `duckdb` crate version moves.
- Cargo workspace `default-features = false` propagation gotcha worth remembering: a workspace dependency entry that omits `default-features` is treated as implicit `true`, and Cargo currently forbids member crates from flipping it to `false`. The fix is to set `default-features = false` at the workspace level so member crates explicitly opt in. The wrapper crate's own `default = ["bundled"]` is unaffected for direct `cargo build -p winterbaume-sqlengine-duckdb` — only inheritors via `workspace = true` see `default-features = false`. If a second default feature is ever added, opt-out paths will silently drop it.
- Server wire-up shares a single `Arc<Mutex<Connection>>` between Athena and Redshift Data so `CREATE TABLE` issued through one is visible to the other — closer to a real Glue catalog tying them together than two isolated databases would be. New config: `[backends] sqlengine_duckdb`, env var `WB_SQLENGINE_DUCKDB`, CLI flag `--sqlengine-duckdb PATH` ( `:memory:` or empty for in-memory; filesystem path for persistence ). When the option is set but the feature is off, the server logs a `tracing::warn!` and falls back to in-memory, matching the SQS / DynamoDB / VFS warn-and-degrade pattern.

## Operational Guidance

When adding a pluggable backend to a service:

1. Put the trait in the service crate and keep it object-safe.
2. Keep the service's default constructor on an in-memory adapter that preserves current behaviour.
3. Add `with_backend(...)` or `with_query_backend(...)` rather than wiring external engines into `new()`.
4. If the service also implements `StatefulService`, make snapshots and merges backend-facing instead of reading hidden in-memory state.
5. Put Redis, DuckDB, or other heavy integrations in separate crates so workspace defaults stay fast.
6. Keep request-language or orchestration logic above the backend trait when it can be expressed in terms of existing backend primitives.
7. When the in-memory backend trait grows, update every custom backend in the same pass and add serde defaults for newly persisted fields so older stored data remains readable.

When choosing the right kind of backend:

- Use a storage backend when the service's main problem is durable resource state.
- Use a query backend when the protocol surface is mostly a thin wrapper over a compute or SQL engine.
- Use both only if the service truly has separate persistence and execution concerns.

## Files

- `crates/winterbaume-sqs/src/backend.rs`: SQS backend trait and in-memory adapter
- `crates/winterbaume-sns/src/backend.rs`: SNS backend trait and in-memory adapter
- `crates/winterbaume-dynamodb/src/backend.rs`: DynamoDB backend trait and in-memory adapter
- `crates/winterbaume-dynamodb/src/views.rs`: typed table and secondary-index view shapes
- `crates/winterbaume-athena/src/backend.rs`: Athena query backend trait
- `crates/winterbaume-redshiftdata/src/backend.rs`: Redshift Data query backend trait
- `crates/winterbaume-sqs-redis/src/lib.rs`: Redis-backed SQS implementation
- `crates/winterbaume-dynamodb-redis/src/lib.rs`: Redis-backed DynamoDB implementation
- `crates/winterbaume-sqlengine-duckdb/src/exec.rs`: shared DuckDB execution logic
- `crates/winterbaume-dynamodb/src/partiql_exec.rs`: backend-aware PartiQL execution bridge
- `crates/winterbaume-partiql/src/*.rs`: parser, validator, and intermediate representation for DynamoDB PartiQL

## Tests

Representative validation points:

```bash
cargo check -p winterbaume-sqs -p winterbaume-sns
cargo test -p winterbaume-sqs -p winterbaume-sns
cargo check -p winterbaume-dynamodb
cargo check -p winterbaume-dynamodb-redis
cargo test -p winterbaume-dynamodb
cargo test -p winterbaume-dynamodb-redis
cargo test -p winterbaume-dynamodbstreams
cargo test -p winterbaume-athena
cargo test -p winterbaume-redshiftdata
cargo test -p winterbaume-sqlengine-duckdb
```

Use service-level integration tests for backend extraction regressions and focused engine tests for external execution crates.

## Pitfalls

- Backend injection does not automatically make `views.rs` or `StatefulService` backend-aware.
- GET -> modify -> SET Redis mutation flows are acceptable for mocks but are not concurrency-hardened.
- Heavy execution engines should not be pulled into the default workspace build.
- Do not force higher-level query languages into a backend trait just because they eventually touch storage.
- If a custom backend needs the same semantics as the in-memory path, identify hidden state couplings before assuming the trait surface is sufficient.
- Do not update the in-memory backend trait without updating Redis-backed or other custom backends; otherwise workspace check fails after the service tests pass.
- Do not store secondary-index definitions as opaque JSON when the backend and Terraform converter need semantic access to key schemas and projections.
- Do not treat Athena's `DataCatalogType::Glue` support as functional Glue integration. The catalogue type is stored as metadata but query execution does not resolve Glue-managed schemas.
- Do not re-introduce `partiql-parser` / `partiql-ast` for any new SQL surface in `winterbaume-partiql`. The lalrpop build script ( ~30 minutes per fresh `CARGO_TARGET_DIR` ) is what motivated the rewrite; new tokens / productions go in `parser/lexer.rs` + `parser/expr.rs` + `parser/stmt.rs`.
- Do not rely on old references to `crates/winterbaume-partiql/src/translate.rs` or `value.rs`; those files are no longer in the module tree or on disk after the hand-rolled parser cleanup.
- Do not try to use `EXISTS` inside `WHERE` or `SELECT` projection. AWS rejects it ( "Unexpected path component" ) and the parser will not emit it from an expression position.
- Do not bump the `duckdb` crate version in isolation; the libduckdb URL and SHA-256 in `.github/actions/setup-duckdb` must move in lockstep — version mismatch surfaces as a runtime symbol error, not a build error.
- Do not set `default-features = false` on a workspace dependency entry that previously omitted it ( implicit `true` ); Cargo treats the override as forbidden. Set it at the workspace level instead, and let inheritors opt in explicitly.
- Do not mix bundled and prebuilt-libduckdb feature sets in the same local session; the sccache-wrapper does not always discriminate cache entries between two builds of the same crate where the active feature set differs, and you can resurrect stale `chrono` / `serde` artefacts producing `E0463: can't find crate for serde`. Wipe `.agents-workspace/tmp/winterbaume-rustc-cache/` and the session `target-*` directory when switching feature sets locally.
