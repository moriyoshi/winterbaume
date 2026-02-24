# Pluggable Service Backends and Redis Storage

## Summary

Winterbaume's backend abstraction layer separates protocol handlers from persistence details. The durable pattern is object-safe async backend traits, default in-memory adapters, and optional Redis-backed crates that live outside the core service crate so storage dependencies remain opt-in. The later refinement is just as important: the backend must be the authoritative owner of service state, so `with_backend(...)` does not leave snapshot, restore, merge, or higher-level execution paths pointing at a disconnected in-memory shadow copy.

## Key Facts

- SQS and SNS introduced the first broad pluggable backend traits for high-traffic protocol services.
- DynamoDB later adopted the same pattern with a larger trait surface and a paired Redis implementation.
- Backend traits use `Pin<Box<dyn Future<...> + Send>>` rather than `async-trait`, preserving object safety for `Arc<dyn Backend>`.
- Core service crates still default to in-memory backends and do not take Redis dependencies directly.
- `with_backend(...)` constructors are the stable injection point for alternative storage backends.
- SQS, SNS, and DynamoDB now route snapshot, restore, and merge through the backend trait, so custom backends remain authoritative.
- DynamoDB's PartiQL support now runs as protocol-layer logic via `execute_partiql_via_backend(...)` rather than as a backend-trait method or a direct `DynamoDbState` escape hatch.

## Details

### Backend trait pattern

The shared design is:

- define a `*Backend` trait in the service crate
- keep method parameters owned where necessary so futures can be `'static`
- store `Arc<dyn *Backend>` on the service struct
- provide `new()` for the in-memory default and `with_backend(...)` for injection

This keeps handler code backend-agnostic while preserving a simple local development path.

### SQS and SNS backend rollout

`winterbaume-sqs` and `winterbaume-sns` were the first services to adopt the backend-trait pattern.

Durable choices from that rollout:

- `InMemorySqsBackend` and `InMemorySnsBackend` remain the default storage implementation
- `StatefulService` methods are now backend operations, not special-case reads of a separate service-owned state field
- handler methods became async and now delegate to the backend instead of locking state directly
- the service struct no longer keeps a second disconnected `state` copy when a custom backend is injected

This is the reference pattern for stateful services that want pluggable persistence without rewriting the state-view layer.

### Redis-backed SQS backend

`winterbaume-sqs-redis` is the model for keeping external storage optional:

- the Redis implementation lives in a separate crate
- `winterbaume-sqs` itself stays free of Redis features or dependencies
- the service can be wired with `SqsService::with_backend(Arc::new(RedisSqsBackend::new(...).await?))`

Important Redis design choices:

- queue metadata, messages, ordering, deleted receipt handles, and move-task state are stored under explicit namespaced keys
- `receive_messages` uses a Lua script so visibility checks and receipt-handle assignment happen atomically
- most other mutations use a simple GET -> modify -> SET pattern, which is acceptable for a mock server but not a strong concurrency guarantee
- serde-friendly stored structs bridge domain types that should not be burdened with storage-specific derives

### DynamoDB backend rollout

`winterbaume-dynamodb` adopted the same pattern later, but at a larger scale.

The `DynamoDbBackend` trait covers:

- table lifecycle
- item APIs
- backups
- TTL and PITR
- resource policies
- global tables
- Kinesis streaming destinations
- Contributor Insights
- export APIs
- stream metadata needed by DynamoDB Streams (`update_stream_specification`, `list_streams`, `describe_stream_by_arn`)
- state-view operations (`snapshot`, `restore`, `merge`)

The in-memory adapter delegates to the existing synchronous `DynamoDbState` methods. The Redis implementation in `winterbaume-dynamodb-redis` stores tables, items, backups, tags, policy state, global tables, streaming destinations, Contributor Insights metadata, and exports under namespaced Redis keys.

### Backend ownership and protocol-layer boundaries

The first DynamoDB backend rollout still had a dual `backend + state` pattern. That left three broken seams when `with_backend(...)` was used:

- PartiQL handlers bypassed the backend and read `self.state` directly
- `delete_table` synthesized `DELETING` at the handler layer instead of inside the backend or state implementation
- `StatefulService` snapshot, restore, and merge read the service-owned in-memory state instead of the injected backend

The durable correction is:

- make the backend the only owner of mutable service state
- move snapshot, restore, and merge onto the backend trait
- keep protocol-layer orchestration outside the backend trait when it is really decomposition of higher-level requests into backend primitives

For DynamoDB that means PartiQL now runs through a public helper:

```rust
execute_partiql_via_backend(backend.clone(), account_id, region, op).await
```

This helper belongs above the storage boundary because PartiQL is a query language and request-decomposition layer, not a persistence primitive. Trying to put it on the trait would have forced awkward lifetime workarounds and blurred the layering.

### Redis key-schema lessons

Two storage lessons are worth keeping:

- Namespace every key by account and region so one backend instance can safely host multiple logical AWS partitions.
- Encode composite logical keys in a reversible, transport-safe format. DynamoDB uses URL-safe base64 of JSON key values for composite hash or range keys so all DynamoDB key types remain representable.

### Redis Backend E2E Sweeps ( 2026-04-30 / 2026-05-01 )

Live Redis black-box testing against `winterbaume-server` found and later verified fixes for the DynamoDB and SQS Redis backends. The durable lesson is that optional backends need real-server verification, not only in-memory regression tests, because some bugs sit at the handler-to-backend boundary and others sit in storage-specific ordering or pagination.

Observed DynamoDB Redis schema:

- `ddb:<account>:<region>:tables` - set of table names.
- `ddb:<account>:<region>:table:<name>` - JSON table metadata.
- `ddb:<account>:<region>:table:<name>:items` - Redis hash of composite item keys to JSON-encoded typed DynamoDB items.

The DynamoDB fixes covered sort-key conditions, `ScanIndexForward` ordering for `Query`, `Limit` / `LastEvaluatedKey` pagination for `Query`, PartiQL sort-key equality, LSI persistence and `DescribeTable` emission, update-expression actions on nested paths and collections, and projection expressions.

The SQS fixes covered default visibility-timeout handling, per-message delay and delayed/not-visible counters, redrive policy enforcement, FIFO content-based and explicit deduplication, and long-poll wakeup when a message arrives during the wait window. A subtle handler-level rule emerged: awsJson clients can serialise omitted numeric options as explicit `0`, so handlers must treat `VisibilityTimeout=0` and `DelaySeconds=0` as "not specified" where AWS semantics require queue defaults.

One SQS Redis edge remains worth tracking: dead-lettered entries in the Lua receive path consume the caller's `max_n` budget, so a receive that redrives two messages may return fewer visible messages than requested even when more are available.

## Files

- `crates/winterbaume-sqs/src/backend.rs` - `SqsBackend` and `InMemorySqsBackend`
- `crates/winterbaume-sns/src/backend.rs` - `SnsBackend` and `InMemorySnsBackend`
- `crates/winterbaume-dynamodb/src/backend.rs` - `DynamoDbBackend` and `InMemoryDynamoDbBackend`
- `crates/winterbaume-sqs/src/handlers.rs` - backend-driven async SQS handlers
- `crates/winterbaume-sns/src/handlers.rs` - backend-driven async SNS handlers
- `crates/winterbaume-dynamodb/src/handlers.rs` - backend-driven async DynamoDB handlers
- `crates/winterbaume-sqs/src/views.rs`, `crates/winterbaume-sns/src/views.rs`, `crates/winterbaume-dynamodb/src/views.rs` - `StatefulService` delegation through backend traits
- `crates/winterbaume-dynamodb/src/partiql_exec.rs` - backend-driven PartiQL execution helper
- `crates/winterbaume-sqs-redis/src/lib.rs` - Redis-backed SQS backend
- `crates/winterbaume-dynamodb-redis/src/lib.rs` - Redis-backed DynamoDB backend

## Test Coverage

- `cargo check -p winterbaume-sqs -p winterbaume-sns` passed after the SQS and SNS backend extraction
- `cargo test -p winterbaume-sqs -p winterbaume-sns` passed after the in-memory adapters were introduced
- `cargo check -p winterbaume-dynamodb` passed after the DynamoDB backend trait rollout
- `cargo check -p winterbaume-dynamodb-redis` passed for the Redis implementation crate
- the backend-ownership cleanup later passed `winterbaume-sqs` (72 tests), `winterbaume-sns` (54 tests), and `winterbaume-dynamodb` (129 tests, including PartiQL coverage)
- both Redis backend crates compiled cleanly after snapshot, restore, and merge were moved onto the backend traits
- live Redis E2E verification confirmed the DynamoDB and SQS Redis fixes through AWS CLI calls against `winterbaume-server`

## Pitfalls

- Do not keep a hidden service-owned state field once a backend trait exists. Snapshot, restore, merge, and any protocol-layer helpers will drift out of sync.
- GET -> modify -> SET mutation patterns are acceptable for mocks but are not a substitute for strong concurrency control.
- Keep Redis dependencies out of the core service crate when possible; use a separate backend crate.
- If a higher-level query engine can be expressed as calls to existing backend primitives, keep it above the backend trait rather than forcing it into the storage interface.
- **Redis backend crates drift silently.** Because `winterbaume-sqs-redis` and `winterbaume-dynamodb-redis` are optional, they are not compiled in a default `cargo check` run and can fall behind the parent crate API. Known incidents (2026-04-06):
  - `refactor-state-errors` changed `SqsError` and `DynamoDbError` from plain structs with `{ error_type, message, status }` fields to `thiserror` enums. Both redis crates still constructed the old struct syntax, which caused `E0574` errors.
  - The `AttributeValue` enum refactor changed `Item = HashMap<String, AttributeValue>`. `winterbaume-dynamodb-redis` still used `serde_json::Value` in `encode_key_value`, `item_field`, `update_item`, and `transact_write_items`.
  - The DynamoDB Streams implementation added `update_stream_specification`, `list_streams`, `describe_stream_by_arn`, and `get_stream_records` to `DynamoDbBackend`. `winterbaume-dynamodb-redis` did not implement them.
  - `Table` gained four stream fields (`stream_enabled`, `stream_view_type`, `latest_stream_arn`, `latest_stream_label`). Both `StoredTable → Table` conversion paths in the Redis crate were incomplete.
  - Fix strategy: add `InternalError(String)` to each domain error enum so infrastructure errors ( Redis, JSON parse) map to a clean variant; then update all struct constructions to enum variants; then synchronise trait signatures and data type references.
- **Domain error enums need an `InternalError(String)` escape hatch** for external-dependency failures (Redis errors, JSON deserialization failures) that the main crate's enum does not anticipate. Without it, backend crates cannot implement the trait at all. `SqsError::InternalError` and `DynamoDbError::InternalError` were added in the 2026-04-06 fix.
- Do not assume the parent in-memory backend's test suite covers Redis ordering, pagination, or atomic receive semantics. Redis hashes are unordered and Lua receive paths need their own validation.
- Do not trust numeric `0` from awsJson request bodies as proof the caller explicitly overrode a default. Check the AWS operation semantics before forwarding `Some(0)` into a backend trait.
