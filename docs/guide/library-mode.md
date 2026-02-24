# Library Mode

Library mode is the primary way to use winterbaume in Rust test suites. The mock runs entirely in the same process as your code — no sockets, no ports, no network stack involved.

## How it works

`MockAws` implements the `HttpClient` and `HttpConnector` traits from `aws-smithy-runtime-api`. When you pass `mock.http_client()` to the AWS SDK config, the SDK sends every request through winterbaume instead of making real HTTP calls.

Requests are routed by URL pattern matching (regex on the host portion of the request URI) to the registered `MockService` implementation. Each service parses the request, updates its in-memory state, and returns a wire-format response.

## Building the mock

```rust
use winterbaume_core::MockAws;
use winterbaume_sts::StsService;
use winterbaume_iam::IamService;
use winterbaume_s3::S3Service;
use winterbaume_sqs::SqsService;

let mock = MockAws::builder()
    .with_service(StsService::new())
    .with_service(IamService::new())
    .with_service(S3Service::new())
    .with_service(SqsService::new())
    .build();
```

Add one `.with_service()` call for each AWS service your test exercises.

## Configuring the SDK client

### Option 1: `sdk_config()` (recommended)

```rust
let config = mock.sdk_config("us-east-1").await;
let s3 = aws_sdk_s3::Client::new(&config);
```

`sdk_config(region)` is a convenience wrapper that calls `aws_config::defaults(BehaviorVersion::latest())` and wires in the mock HTTP client, mock credentials, and the given region.

### Option 2: manual config

```rust
use aws_config::BehaviorVersion;

let config = aws_config::defaults(BehaviorVersion::latest())
    .http_client(mock.http_client())
    .credentials_provider(mock.credentials_provider())
    .region("us-east-1")
    .load()
    .await;

let s3 = aws_sdk_s3::Client::new(&config);
```

Use this form when you need to set additional config options that `sdk_config()` does not expose.

### Option 3: per-service client config

Some service clients accept a service-specific `Config` struct. You can pass the mock HTTP client and credentials there instead:

```rust
use aws_sdk_s3::config::{Builder, BehaviorVersion, Region};

let s3_config = aws_sdk_s3::Config::builder()
    .behavior_version(BehaviorVersion::latest())
    .http_client(mock.http_client())
    .credentials_provider(mock.credentials_provider())
    .region(Region::new("us-east-1"))
    .build();

let s3 = aws_sdk_s3::Client::from_conf(s3_config);
```

## State isolation

State is partitioned by account ID and region. The mock uses a fixed account ID (`123456789012`) for the built-in credentials provider. If you create multiple `MockAws` instances in the same test, each instance gets its own isolated state store — resources created through one instance are not visible to another.

## Multi-region tests

Create one config per region:

```rust
let config_east = mock.sdk_config("us-east-1").await;
let config_west = mock.sdk_config("us-west-2").await;

let s3_east = aws_sdk_s3::Client::new(&config_east);
let s3_west = aws_sdk_s3::Client::new(&config_west);

// Buckets created in us-east-1 are not visible in us-west-2
s3_east.create_bucket().bucket("east-bucket").send().await.unwrap();
let list = s3_west.list_buckets().send().await.unwrap();
assert!(list.buckets().is_empty());
```

## Parallel tests

`MockAws` is `Send + Sync`. Each test should construct its own `MockAws` instance to avoid shared state between parallel tests. The cost is low — `new()` allocates an empty state map, and state is only populated as requests come in.
