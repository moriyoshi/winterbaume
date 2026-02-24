# StatefulService, State Views, and VFS-Backed Blob Store

## Summary

The `StatefulService` trait, typed `views.rs` modules, `StateChangeNotifier`, the async `Vfs` abstraction, and `BlobStore` together provide Winterbaume's shared state export, import, merge, and blob-persistence layer. The durable shape after the April 2026 work is metadata-only state views, async snapshot and restore paths, `BlobBackedService` for services whose durable bytes live outside normal views, and a clear separation between durable configuration state and transient runtime state.

## Key Facts

- All service crates now implement the async `StatefulService` contract.
- `snapshot()`, `restore()`, `merge()`, and `notify_state_changed()` are async.
- State views should be metadata-only. Large payloads and backing-store contents should be referenced, not embedded.
- `BlobBackedService` exists for services that need to export or import backing-store blobs alongside metadata views.
- `BackendState` uses `tokio::sync::RwLock` for per-region state so async snapshot and blob workflows can stay consistent.
- `Vfs` remains object-safe and async through explicit boxed-future return types.
- `FsVfs` must implement string-prefix list semantics and reject path traversal.
- S3 now persists enough at-rest metadata in `BlobStore` to reconstruct bucket configuration, object history, and delete markers after in-memory state loss.
- Glacier now persists vault and archive metadata in `BlobStore` so empty snapshot restore can recover durable vault/archive state from blob metadata.
- Blob-backed services must isolate blobs by `(account_id, region)`. `BlobStoreMap` is the reference mechanism for scoped child stores over one shared VFS.
- `MemVfs` and `FsVfs` must enforce the same path-validation rules so tests do not mask traversal payloads that the filesystem backend would reject.
- S3, EBS, and Glacier are the reference blob-backed services.
- Persistence for SSM and Secrets Manager across server restarts remains an open design problem.

## Details

### StatefulService Contract

The trait lives in `crates/winterbaume-core/src/views.rs` and uses async methods end to end:

```rust
pub trait StatefulService: MockService {
    type StateView: Serialize + DeserializeOwned + Send + Sync;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView;
    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError>;
    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError>;
    fn notifier(&self) -> &StateChangeNotifier<Self::StateView>;
}
```

`restore()` is full replacement. `merge()` is additive and must not delete existing resources absent from the incoming view.

### Metadata-Only State Views

The 2026-04-06 snapshot audit established the repo-wide direction: state views should contain metadata and stable references, not bulk payload data.

That led to these durable rules:

- S3 objects are represented through blob keys and lengths, not inline bodies
- CodeCommit file content, AppConfig hosted configuration bytes, and SageMaker Runtime invocation bodies do not belong in views
- Kinesis and Firehose record payloads should not inflate snapshot cost
- services with genuinely small inline values can keep them in views when the serialisation cost is negligible

The point is not only smaller snapshots. It is also a clearer contract: views describe durable resource state, while backing stores or runtime APIs supply heavy payloads.

### StateChangeNotifier and Mutation Hooks

`StateChangeNotifier<V>` stores synchronous listener callbacks. The service pattern is:

- compute account and region
- handle the request
- if the mutation succeeded, call `notify_state_changed(...).await`

The callback itself stays synchronous. Listeners that need async work should spawn it themselves.

### Locking Rules

Two rules became critical once snapshotting and blob work went async:

- never await `notify_state_changed()` while holding a write lock
- never hold a lock guard across blob-store awaits

The safe pattern is:

- collect or clone the needed metadata while under the guard
- drop the guard
- perform async blob or notifier work afterwards

### BlobBackedService

`BlobBackedService` is the extension trait for services whose durable bytes live in `BlobStore`.

The important durable points are:

- metadata and blob movement are separate concerns
- exporting or importing blobs should stream through the store rather than buffering the whole service state in memory
- visitor or source abstractions are preferable to closure shapes that force `'static` futures or `Arc<Mutex<...>>` workarounds

The April redesign replaced `dyn FnMut` callback shapes with dyn-compatible traits that return futures tied to the borrow of `&mut self`. That fixed both ergonomics and lifetime soundness.

The same redesign also fixed a real restore bug: `restore_with_blobs()` had been iterating the old state instead of the incoming view-derived state, which meant a fresh restore could import zero blobs.

### BackendState and Async Locking

`BackendState` moved its per-region inner lock to `tokio::sync::RwLock`.

That change matters because:

- blob-aware snapshot and restore logic sometimes needs to await while keeping per-region consistency
- service handlers and `views.rs` implementations can now use a uniform async lock model
- the repo no longer needs awkward split designs just to avoid await-while-locked limitations of `std::sync::RwLock`

The outer map lock remains brief and synchronous in purpose. The inner region state is the async boundary.

### Async Vfs and BlobStore

`Vfs` lives in `crates/winterbaume-core/src/vfs.rs` and provides:

- `put`
- `get`
- `delete`
- `list`
- `stat`

Reference implementations:

- `MemVfs`
- `FsVfs`

Important behavioural rules:

- `list(prefix)` is string-prefix matching, not directory-boundary matching
- `FsVfs` must reject traversal attempts before touching the filesystem

`BlobStore` builds on `Vfs` and provides:

- versioned blob storage
- current-pointer indirection
- composite blobs for multipart assembly

S3 multipart uploads and other large-payload paths depend on those semantics.

The S3 durability work extended that storage layer so `BlobStore` can also persist structured metadata records under a namespaced `meta/` tree:

- `meta/version-meta/{blob_key}/{blob_version_id}` for stored object-version metadata
- `meta/delete-markers/{blob_key}/{dm_version_id}` for stored delete-marker metadata
- `meta/bucket-config/{bucket_name}` for stored bucket configuration

The important durable rule is that blob-backed services may need the store to hold both heavy payload bytes and the metadata required to recover higher-level state.

### BlobStoreMap and Scope Isolation

S3, EBS, and Glacier originally shared a flat service-level blob namespace such as `"s3"`. That collided with `BackendState`'s scoped in-memory model: two accounts or regions could have independent state buckets but still write blobs under identical VFS paths.

`BlobStoreMap` fixes that by managing child `BlobStore` instances keyed by `(account_id, region)`. Each scoped store uses namespace `{base}/{account_id}/{region}` while sharing the same underlying VFS.

Durable API shape:

- `BlobStoreMap::mem(namespace)` and `BlobStoreMap::new(vfs, namespace)` construct the scoped map
- `get(account_id, region)` lazily returns the scoped child store
- `base()` exposes the unscoped base store for legacy recovery
- `list_children()` scans VFS entries under the base namespace and returns stored `(account_id, region)` pairs, skipping legacy `data/` and `meta/` prefixes

Service patterns:

- service structs hold `Arc<BlobStoreMap>` rather than `Arc<BlobStore>`
- dispatch resolves the scoped store from request account and region, then passes `&BlobStore` to blob-using handlers
- `snapshot_with_blobs()` and `restore_with_blobs()` resolve the scoped store using the `StatefulService` arguments
- S3 recovery scans scoped children first and falls back to the legacy flat namespace only when no scoped children exist

The durable rule is that blob namespace scope must match `BackendState` scope. Metadata-only views and blob stores should not disagree about account or region boundaries.

### S3 Recovery from BlobStore Metadata

S3 is now the reference implementation for BlobStore-backed state recovery.

The durable shape is:

- `crates/winterbaume-s3/src/stored.rs` defines serde-friendly at-rest types such as `StoredObjectVersion`, `StoredDeleteMarker`, and `StoredBucketConfig`
- mutating handlers persist or delete the matching metadata record alongside the normal in-memory state update
- `S3Service::recover()` rebuilds `BucketState` from stored bucket configs, version metadata, and delete-marker metadata
- `restore_with_blobs()` falls back to `recover()` when the incoming snapshot is empty or missing bucket data

This changes the durability rule for S3: BlobStore is not only a payload backing store, it is also the recovery source of truth when process memory is lost.

### Glacier Recovery from BlobStore Metadata

Glacier adopted the same recovery idea for its durable archive surface.

The durable shape is:

- `crates/winterbaume-glacier/src/stored.rs` defines `StoredVaultConfig` and `StoredArchiveMeta`
- vault configuration persists vault name, ARN, creation time, tags, access policy, notification config, and vault lock data
- archive metadata persists vault name, archive ID, blob key, size, SHA-256, description, and creation date
- mutating handlers persist or delete matching metadata records after in-memory state updates
- `restore_with_blobs()` falls back to `recover_state_from_blobs()` when the incoming view is empty

Glacier deliberately does not persist jobs, multipart uploads, data retrieval policy, or provisioned capacity to the blob store. Those are ephemeral or transient surfaces rather than durable archive state.

One known fixture pitfall remains: a pre-existing Glacier blob-backed round-trip test seeded data into an unscoped `BlobStore` namespace while snapshot code read from the scoped `{service}/{account}/{region}` namespace. That is a test setup issue, not a Glacier recovery contract.

### Path-Validation Hardening

The VFS and BlobStore work also established stricter path-validation rules:

- both `MemVfs` and `FsVfs` call the same shared `validate_path` helper
- null bytes and any `..` path component are rejected before backend-specific handling
- `BlobStore::import_version` rejects caller-supplied version IDs containing `/` or `.`, because those values become path segments and the codebase only expects UUID-like or timestamp-like identifiers

The durable lesson is that the in-memory VFS must not be more permissive than the filesystem VFS on security-sensitive path handling.

### Durable State versus Derived State

Not every service should own its own durable resource state.

The DynamoDB Streams repair is the reference example:

- stream existence is derived from DynamoDB table configuration
- the streams service should not duplicate table-owned durable state
- only protocol-local transient state such as iterator positions belongs in the streams service itself

This is the durable rule for future services: if the real AWS surface is derived from another service's authoritative state, model it that way.

### State/View Disparity Audit ( 2026-04-30 )

The SES v2 `sent_emails` example exposed a broader state-view fidelity class: fields can be mutated in `state.rs`, used by behaviour, but omitted from `views.rs`, so snapshot / restore / merge silently drops durable state.

The audit found and fixed the same class in multiple crates:

| Crate | Fields |
|-------|--------|
| `winterbaume-sfn` | `task_tokens`, `execution_history`, `aliases` |
| `winterbaume-acm` | `idempotency_tokens`, `account_configuration` |
| `winterbaume-sqs` | `message_move_tasks` |
| `winterbaume-iotdataplane` | `published_messages` |
| `winterbaume-databasemigration` | `connections` |
| `winterbaume-ecr` | `layer_uploads` |
| `winterbaume-glue` | `job_runs` |
| `winterbaume-cloudwatch` | `metrics` |
| `winterbaume-kinesis` | `resource_tags`, `account_settings_commitment_status` |

Audit rule: grep both `state.rs` and `handlers.rs` for write sites. A first pass that only grepped handlers missed real mutations because handlers often call state methods that perform the actual writes.

Encoding rules from the fix pass:

- map-like state view fields merge by insert-overwrite.
- append-only histories merge by extend.
- singleton settings overwrite when the incoming view supplies a non-default value.
- nested maps should deep-merge per outer key.
- non-JSON-map keys, such as `HashMap<(String, String), JobRun>`, need flattened view keys and a reverse parser with a safe fallback.

`winterbaume-emrcontainers` `job_runs` was surfaced as a likely remaining candidate of the same shape.

### Persistence Follow-Up

The remaining open architectural question is persistence for configuration or secret-store services such as SSM and Secrets Manager.

The durable guidance from the audit is:

- do not assume BlobStore is the right answer for every persistence problem
- metadata-only views remain the right shape either way
- the persistence layer should be designed as persistence, not as a backdoor reason to put more payload bytes into `StateView`

## Files

- `crates/winterbaume-core/src/views.rs` - `StatefulService`, `StateChangeNotifier`, `BlobBackedService`, and `StateViewError`
- `crates/winterbaume-core/src/state.rs` - `BackendState` async lock model
- `crates/winterbaume-core/src/vfs.rs` - `Vfs`, `MemVfs`, and `FsVfs`
- `crates/winterbaume-core/src/blob_store.rs` - blob storage, versioning, composite blobs, and `BlobStoreMap` scoped child stores
- `crates/winterbaume-s3/src/stored.rs` - at-rest serde shapes for S3 bucket config, object versions, and delete markers
- `crates/winterbaume-s3/src/views.rs` - metadata-only blob-backed view pattern
- `crates/winterbaume-s3/src/handlers.rs` - recovery logic, metadata persistence hooks, and bucket-config persistence
- `crates/winterbaume-glacier/src/stored.rs` - at-rest serde shapes for Glacier vault configs and archive metadata
- `crates/winterbaume-glacier/src/handlers.rs` - Glacier metadata persistence and blob-backed recovery helpers
- `crates/winterbaume-ebs/src/views.rs` - reference `BlobBackedService` implementation
- `crates/winterbaume-glacier/src/views.rs` - reference `BlobBackedService` implementation
- `crates/winterbaume-dynamodb/src/backend.rs` and `state.rs` - authoritative upstream state for derived streams
- `crates/winterbaume-dynamodbstreams/src/views.rs` - derived-service state-view pattern

## Test Coverage

- core VFS and BlobStore units cover prefix semantics and traversal rejection
- `winterbaume-core` also covers parity between `MemVfs` and `FsVfs` path-traversal rejection
- S3, EBS, and Glacier gained blob export or import smoke coverage through the `BlobBackedService` redesign
- S3 scenario coverage now includes recovery from BlobStore-backed metadata, version-history reconstruction, and bucket-config recovery
- S3 tests cover scoped BlobStore recovery and shared-VFS recovery through `recover_with_vfs()`
- Glacier recovery work added archive and vault metadata persistence paths, with a known pre-existing scoped fixture issue in one blob-backed round-trip test
- the workspace-wide async lock migration was validated through successful crate-local build and test passes during the rollout
- the state/view disparity sweep fixed 13 durable fields across 9 crates and verified each touched crate with clippy, fmt, and tests

## Pitfalls

- Never await `notify_state_changed()` while holding a write lock.
- Never hold lock guards across blob-store awaits.
- Do not put large payload data back into `StateView` just to make export feel convenient.
- Do not duplicate durable state in a derived service when another backend already owns it.
- Do not assume an empty incoming snapshot means the durable state is empty for a blob-backed service. S3 can now recover from BlobStore metadata.
- Do not let `MemVfs` accept traversal-shaped paths that `FsVfs` would reject.
- Do not accept arbitrary caller-supplied blob version IDs as path segments without validation.
- Do not treat the persistence problem for SSM or Secrets Manager as solved. It remains open design work.
- Do not use one flat blob namespace for state that is otherwise scoped by account and region.
- Do not seed blob-backed tests into the unscoped base namespace when service recovery reads from scoped `(account_id, region)` stores.
- Do not audit state-view fidelity only from handlers. Mutations commonly live in `state.rs` methods.
