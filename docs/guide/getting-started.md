# Getting Started

Winterbäume (stylised as Winterbaume) is a stateful AWS service mock for `aws-sdk-rust`. It intercepts HTTP calls made by the SDK in-process and routes them to in-memory service backends that simulate real AWS behaviour — no network I/O required.

## Installation

Add the crates you need to your `Cargo.toml`. At minimum you need `winterbaume-core`, plus one crate per AWS service you want to mock:

```toml
[dev-dependencies]
winterbaume-core = { path = "..." }
winterbaume-sts   = { path = "..." }
winterbaume-s3    = { path = "..." }
winterbaume-sqs   = { path = "..." }
```

Replace `path = "..."` with the appropriate `git` or registry source for your setup.

## Quickstart

```rust
use winterbaume_core::MockAws;
use winterbaume_sts::StsService;
use winterbaume_s3::S3Service;

#[tokio::test]
async fn test_s3_and_sts() {
    let mock = MockAws::builder()
        .with_service(StsService::new())
        .with_service(S3Service::new())
        .build();

    // sdk_config() returns a fully configured SdkConfig for the given region
    let config = mock.sdk_config("us-east-1").await;

    let sts = aws_sdk_sts::Client::new(&config);
    let resp = sts.get_caller_identity().send().await.unwrap();
    assert_eq!(resp.account(), Some("123456789012"));

    let s3 = aws_sdk_s3::Client::new(&config);
    s3.create_bucket()
        .bucket("my-test-bucket")
        .send()
        .await
        .unwrap();

    let buckets = s3.list_buckets().send().await.unwrap();
    assert_eq!(buckets.buckets().len(), 1);
}
```

`MockAws::builder()` registers the services you want. `sdk_config(region)` wires the mock HTTP client and fake credentials into an `aws_config::SdkConfig` so every SDK client you create from it routes to the mock.

## What happens to unimplemented operations?

Services that are listed as stubs in the [coverage table](/reference/services) return `501 Not Implemented`. Partially implemented services only handle the operations listed in that table; any other operation also returns `501`.

## Standalone server

If you are testing non-Rust clients, CLI scripts, or Terraform configurations, run `winterbaume-server` instead:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

Then point any AWS client at it:

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
export AWS_ACCESS_KEY_ID=test
export AWS_SECRET_ACCESS_KEY=test
export AWS_DEFAULT_REGION=us-east-1

aws s3 mb s3://my-bucket
aws s3 ls
```

No extra configuration is needed — every service in winterbaume is pre-registered in the server. See [Server Mode](/guide/server-mode) for the full usage guide, including boto3 and Terraform examples.

## Next steps

- [Library Mode](/guide/library-mode) — full in-process usage reference for Rust tests
- [Server Mode](/guide/server-mode) — standalone HTTP server for non-Rust clients, CLIs, and Terraform
- [State Management](/guide/state-management) — snapshots, restore, and merge
- [Smithy Mocks Integration](/guide/smithy-mocks) — combining winterbaume with per-operation rule overrides
