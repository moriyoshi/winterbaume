# Architecture

## Request flow

The diagram below traces a single SDK call from user code through winterbaume and back:

```
aws_sdk_sts::Client::get_caller_identity()
  -> aws-sdk-rust serializes to HTTP request
     (awsQuery: POST with URL-encoded form body)
  -> MockAwsClient intercepts
     (implements HttpClient / HttpConnector)
  -> Regex URL pattern matching finds StsService
  -> Request converted to MockRequest
     (method, URI, headers, body bytes)
  -> StsService.handle() dispatches:
     1. Parses Action parameter from form body
     2. Routes to get_caller_identity handler
     3. Handler reads/writes StsState via BackendState
     4. Returns MockResponse with XML body
  -> MockAwsClient converts MockResponse to smithy Response
  -> aws-sdk-rust deserializes XML
  -> Caller receives GetCallerIdentityOutput
```

## Core components

### `MockAws` builder

Entry point. Registers service backends, builds the HTTP client, and provides fake credentials.

Key methods:

| Method | Description |
|---|---|
| `MockAws::builder()` | Start building a mock |
| `.with_service(svc)` | Register a service backend |
| `.build()` | Finalize and return `MockAws` |
| `mock.sdk_config(region)` | Return a fully-wired `SdkConfig` |
| `mock.http_client()` | Return `SharedHttpClient` for manual SDK config |
| `mock.credentials_provider()` | Return mock credentials |
| `mock.mock_interceptor(mode, rules)` | Return an `aws_smithy_mocks` interceptor (requires `smithy-mocks` feature) |

### `MockAwsClient`

Implements `HttpClient` and `HttpConnector` from `aws-smithy-runtime-api`. Performs regex-based URL routing to dispatch requests to the correct service. Falls back to extracting the service name from the request host when no pattern matches.

### `MockService` trait

Every AWS service backend implements:

- `service_name()` — e.g. `"sts"`, `"s3"`
- `url_patterns()` — regex patterns used for routing
- `handle(MockRequest) -> MockResponse` — async request handler

### `BackendState<B>`

Per-`(account_id, region)` state container. Uses a `RwLock<HashMap<..., Arc<RwLock<B>>>>` with lazy initialisation. Mirrors moto's `BackendDict` pattern. All state mutation goes through this container so concurrent test runs remain isolated.

### Protocol helpers

`winterbaume-core/src/protocol/` provides shared wire-format utilities so service crates focus on business logic:

| Module | Purpose |
|---|---|
| `protocol/xml.rs` | XML serialization for awsQuery and REST-XML services |
| `protocol/json.rs` | JSON response helpers for awsJson and REST-JSON services |
| `protocol/common.rs` | Shared utilities, pagination, error response shapes |

### `StatefulService` trait

Layered on top of `MockService`. Provides `snapshot()`, `restore()`, and `merge()` with a typed `StateView` per service. See [State Management](/guide/state-management).

### `Vfs` and `BlobStore`

Object-safe async storage abstraction for large payloads (S3 objects, EBS snapshots, Glacier archives). Two built-in implementations:

- `MemVfs` — in-memory, for tests and library mode
- `FsVfs` — filesystem-backed, for server mode persistence

### Smithy codegen

`tools/smithy-codegen` reads AWS Smithy models and emits `model.rs` (typed request/response structs) and `wire.rs` (serialisers) for each mapped service. These files carry a `//! Do not edit manually` header. Fixes go in the generator, not in the generated output.

## Per-service crate layout

```
crates/winterbaume-<service>/
  src/
    lib.rs           # crate exports
    handlers.rs      # protocol routing and operation handlers
    state.rs         # in-memory state and validation logic
    types.rs         # internal domain types
    views.rs         # StatefulService StateView implementation
    model.rs         # generated — Smithy-derived request/response types
    wire.rs          # generated — serialisers and model re-exports
  tests/
    integration_test.rs   # SDK integration tests and moto parity ports
```

## Protocols supported

| AWS protocol | Used by |
|---|---|
| `awsQuery` | STS, IAM, CloudWatch, SNS, SQS (legacy), ELBv2 |
| `ec2Query` | EC2 |
| `awsJson1.0` | DynamoDB, DynamoDB Streams, Step Functions, SWF |
| `awsJson1.1` | Lambda, KMS, Secrets Manager, ECS, many others |
| `restJson1` | S3 (management), API Gateway, EKS, EFS, and most modern services |
| `restXml` | S3 (object operations), Route 53, CloudFront |
| `rpc-v2-cbor` | — (supported, limited service coverage) |
