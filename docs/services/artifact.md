# winterbaume-artifact

AWS Artifact service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Artifact |
| AWS model | `artifact` |
| Protocol | restJson1 |
| winterbaume coverage | 8/8 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/8 operations (0.0%) |
| moto coverage | 0/8 operations (0.0%) |
| floci coverage | 0/8 operations (0.0%) |
| kumo coverage | 0/8 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws artifact help
```

## Example

```rust
use aws_sdk_artifact::config::BehaviorVersion;
use winterbaume_artifact::ArtifactService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ArtifactService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_artifact::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_artifact::Client::new(&config);

    let resp = client
        .list_reports()
        .send()
        .await
        .expect("list_reports should succeed");
    println!("Artifact reports: {}", resp.reports().len());
}
```

## Implemented APIs (8)

- `GetAccountSettings`
- `GetReport`
- `GetReportMetadata`
- `GetTermForReport`
- `ListCustomerAgreements`
- `ListReportVersions`
- `ListReports`
- `PutAccountSettings`
