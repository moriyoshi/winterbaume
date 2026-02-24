# State Management

Every service crate implements the `StatefulService` trait, which provides a typed, serde-compatible contract for extracting and injecting service state.

## The `StatefulService` trait

```rust
pub trait StatefulService: MockService {
    type StateView: Serialize + DeserializeOwned;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView;
    async fn restore(&self, account_id: &str, region: &str, view: Self::StateView);
    async fn merge(&self, account_id: &str, region: &str, view: Self::StateView);
}
```

Each service exposes a `StateView` type that can be serialized to JSON (or any serde format). The view contains only the data that belongs in a snapshot — runtime-only internals are excluded.

## Operations

### `snapshot`

Returns the current state for a given account/region pair as a typed view. Useful for asserting on state after a sequence of SDK calls, or for saving and restoring test fixtures.

```rust
let s3 = S3Service::new();
let mock = MockAws::builder().with_service(s3.clone()).build();
let config = mock.sdk_config("us-east-1").await;

aws_sdk_s3::Client::new(&config)
    .create_bucket()
    .bucket("my-bucket")
    .send()
    .await
    .unwrap();

let view = s3.snapshot("123456789012", "us-east-1").await;
// view.buckets contains "my-bucket"
```

### `restore`

Replaces the entire state for an account/region with the supplied view. Any resources that existed before the restore are discarded.

```rust
// Save state before a destructive test sequence
let saved = s3.snapshot("123456789012", "us-east-1").await;

// ... run tests that modify state ...

// Reset to the saved baseline
s3.restore("123456789012", "us-east-1", saved).await;
```

### `merge`

Additively overlays the supplied view onto the current state. Resources already in the live state are preserved; the view only adds to them.

```rust
// Pre-seed some buckets without disturbing any existing state
let seed = S3StateView {
    buckets: vec![BucketView { name: "pre-seeded".into(), .. }],
    ..Default::default()
};
s3.merge("123456789012", "us-east-1", seed).await;
```

## State isolation

State is keyed by `(account_id, region)`. Each `MockAws` instance owns its own `BackendState` map, so two `MockAws` instances never share state even when they use the same service implementation object.

## Blob-backed state

Services that store large payloads (S3 object bodies, EBS snapshots, Glacier archives) route data through the shared `Vfs` / `BlobStore` layer instead of keeping bytes in the in-memory state struct. Snapshots taken via `snapshot()` include blob references but not the raw bytes. Restoring or merging such a view on a fresh `MockAws` instance requires that the same `Vfs` be shared, or that you re-upload the objects.

## JSON serialization example

State views are plain Rust structs that derive `serde::Serialize` and `serde::Deserialize`. You can serialize them to JSON for golden-file tests or fixture files:

```rust
let view = dynamodb.snapshot("123456789012", "us-east-1").await;
let json = serde_json::to_string_pretty(&view).unwrap();
// Save to a fixture file, compare on next run, etc.
```

## State change notifications

Services emit a `StateChangeNotifier` event after any mutating operation. Listeners registered on the notifier receive a fresh snapshot synchronously. Any async work triggered by a state change must be spawned by the listener — the notifier callback itself is synchronous.
