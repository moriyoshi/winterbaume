# Architecture

## Request flow

The diagram below traces a complete round-trip for an `aws-sdk-rust` call in library mode:

```
aws_sdk_sts::Client::get_caller_identity()
  │
  ▼  aws-sdk-rust serializes the call to HTTP
     awsQuery: POST / with URL-encoded body
     Action=GetCallerIdentity&Version=2011-06-15
  │
  ▼  MockAwsClient.call(Request)
     implements HttpConnector / HttpClient
  │
  ▼  URL pattern matching  →  StsService
     regex on request host
  │
  ▼  StsService.handle(MockRequest)
     1. parse Action from URL-encoded body
     2. dispatch to get_caller_identity handler
     3. handler reads StsState via BackendState<StsState>
     4. return MockResponse { status: 200, body: XML }
  │
  ▼  MockAwsClient converts MockResponse → smithy Response
  │
  ▼  aws-sdk-rust deserializes XML
  │
  ▼  caller receives GetCallerIdentityOutput
```

In server mode (`winterbaume-server`) the same `MockService` objects sit behind a hyper HTTP server instead of the in-process `MockAwsClient`. Everything from `StsService.handle()` downward is identical.

## Core traits

### `MockService`

Defined in `winterbaume-core/src/service.rs`. Every AWS service implements:

```rust
pub trait MockService: Send + Sync + 'static {
    fn service_name(&self) -> &str;
    fn url_patterns(&self) -> &[Regex];
    fn handle(&self, req: MockRequest)
        -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>>;
}
```

`service_name()` is used as a fallback lookup key when the host does not match any `url_patterns()` regex.

### `StatefulService`

Layered on top of `MockService`. Provides a typed, serde-compatible state view:

```rust
pub trait StatefulService: MockService {
    type StateView: Serialize + DeserializeOwned + Send + Sync;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView;
    async fn restore(&self, account_id: &str, region: &str, view: Self::StateView);
    async fn merge(&self, account_id: &str, region: &str, view: Self::StateView);
    async fn notify_state_changed(&self, account_id: &str, region: &str);
}
```

`restore()` is a full replacement; `merge()` is additive and must not delete resources that are absent from the incoming view. `notify_state_changed()` snapshots the current state and publishes it through `StateChangeNotifier`.

## `BackendState<B>`

`winterbaume-core/src/state.rs`

```rust
pub struct BackendState<B> {
    inner: RwLock<HashMap<(String, String), Arc<RwLock<B>>>>,
}
```

Keys are `(account_id, region)` tuples. State is lazily initialized — the inner `B` is created via `B::default()` on first access. Read and write locks are taken at the entry level so concurrent requests for different accounts/regions do not block each other.

This mirrors moto's `BackendDict` pattern.

## `MockAwsClient`

`winterbaume-core/src/mock_client.rs`

Implements both `HttpConnector` and `HttpClient` from `aws-smithy-runtime-api`. On each request it:

1. Tries each registered service's `url_patterns()` regexes against the request URI.
2. Falls back to extracting the service name from the subdomain (e.g. `sqs.us-east-1.amazonaws.com` → `"sqs"`).
3. Converts the smithy `Request` to a `MockRequest` and calls the matched service's `handle()`.
4. Converts the returned `MockResponse` back to a smithy `Response`.

## `MockAws` builder

`winterbaume-core/src/mock_aws.rs`

```rust
let mock = MockAws::builder()
    .with_service(StsService::new())
    .with_service(S3Service::new())
    .build();

let config = mock.sdk_config("us-east-1").await;       // convenience
let http   = mock.http_client();                        // SharedHttpClient
let creds  = mock.credentials_provider();              // mock creds (account 123456789012)
let interceptor = mock.mock_interceptor(mode, rules);  // aws-smithy-mocks bridge (feature-gated)
```

## Protocol helpers

`winterbaume-core/src/protocol/`

| Module | Purpose |
|---|---|
| `xml.rs` | Response wrappers and error shapes for awsQuery and REST-XML |
| `json.rs` | Response helpers for awsJson and REST-JSON |
| `common.rs` | Shared utilities and protocol-neutral pagination helpers |

Service crates use these helpers instead of building XML/JSON strings by hand.

## Vfs and BlobStore

`winterbaume-core/src/vfs.rs`, `blob_store.rs`

Services that handle large payloads (S3 objects, EBS snapshots, Glacier archives) store raw bytes outside their in-memory state via the `Vfs` abstraction:

```rust
pub trait Vfs: Send + Sync + 'static {
    async fn put(&self, key: &str, data: Bytes) -> Result<()>;
    async fn get(&self, key: &str) -> Result<Bytes>;
    async fn delete(&self, key: &str) -> Result<()>;
    async fn list(&self, prefix: &str) -> Result<Vec<String>>;
    async fn stat(&self, key: &str) -> Result<BlobMeta>;
}
```

`MemVfs` stores blobs in a `HashMap`. `FsVfs` writes one file per key on disk.

`BlobStore` wraps `Arc<dyn Vfs>` with a namespace and tracks current blobs, versioned blobs, and composite manifests (multi-part uploads, etc.) separately.

## Per-service crate layout

```
crates/winterbaume-{svc}/
  src/
    lib.rs          re-exports
    handlers.rs     protocol routing, request parsing, operation dispatch
    state.rs        in-memory state, validation, state transitions
    types.rs        internal domain types
    views.rs        StatefulService StateView + snapshot/restore/merge impl
    backend.rs      object-safe async backend trait + in-memory default  (optional)
    model.rs        ← generated: Smithy-derived request/response structs
    wire.rs         ← generated: serialisers + pub use super::model::*
  tests/
    integration_test.rs   SDK integration tests, moto parity ports, regressions
```

`model.rs` and `wire.rs` carry a `//! Do not edit manually` header. All fixes to their contents belong in `tools/smithy-codegen`.

### `state.rs` vs `views.rs`

`state.rs` owns the mutable runtime representation and may contain transient fields that are convenient for request handling (caches, receipt handles, in-flight uploads). `views.rs` owns the stable serde-facing contract for snapshots. Conversions between the two are explicit so `restore()` / `merge()` can reinitialise transient fields rather than leak runtime internals into the serialized format.

## Smithy codegen subsystem

`tools/smithy-codegen` parses AWS Smithy model directories and emits per-service `model.rs` / `wire.rs` modules. It handles:

- collecting operations from both `service.operations` and nested Smithy resources
- protocol-specific serialiser behaviour (XML container wrappers, JSON/XML timestamp defaults)
- keeping generated model types separate from serialiser code

When generated outputs are wrong, the durable fix is to patch `tools/smithy-codegen`, rebuild it, and regenerate. See [Smithy Codegen](./smithy-codegen).

## DynamoDB PartiQL subsystem

Parsing and execution are split across two crates:

- `winterbaume-partiql` — parser: converts PartiQL text into an intermediate `DdbOperation` representation. SELECT uses the upstream `partiql-parser`; INSERT/UPDATE/DELETE use a custom lightweight DML parser.
- `winterbaume-dynamodb/src/partiql_exec.rs` — executor: maps `DdbOperation` onto `DynamoDbBackend` calls.

Positional `?` parameters are substituted into PartiQL literals before parsing. PartiQL remains above the backend trait — it is protocol-layer orchestration, not a backend method.

## Query execution backends (Athena, Redshift Data)

Athena and Redshift Data apply the same pluggable-backend pattern to SQL query execution:

- `winterbaume-athena` exposes `AthenaQueryBackend`
- `winterbaume-redshiftdata` exposes `RedshiftQueryBackend`
- `winterbaume-sqlengine-duckdb` provides the concrete DuckDB engine (excluded from the default workspace path due to heavy native dependencies)

## Stub services

`winterbaume-stubs` registers every unimplemented service name and returns `501 Not Implemented` for all operations. It is a catchall so that correctly-spelled but unimplemented service endpoints get a proper HTTP error rather than a connection refused.

## Supported wire protocols

| Protocol | Format | Used by |
|---|---|---|
| `awsQuery` | POST, `application/x-www-form-urlencoded`, XML response | STS, IAM, SNS, Auto Scaling, CloudFormation, ELB / ELBv2 |
| `ec2Query` | Same as awsQuery with EC2 envelope quirks | EC2 |
| `awsJson1.0` | POST, JSON body, `X-Amz-Target` header, JSON response | DynamoDB, SQS, CloudWatch, Step Functions, SWF |
| `awsJson1.1` | Same as 1.0 with minor differences | KMS, ECS, Secrets Manager, CloudWatch Logs, most modern awsJson services |
| `restJson1` | Method + path routing, JSON body, JSON response | Lambda, S3 (management), API Gateway, EKS, EFS |
| `restXml` | Method + path routing, XML body, XML response | S3 (objects), Route 53, CloudFront |
| `rpc-v2-cbor` | CBOR framing over HTTP/2 | limited coverage |
