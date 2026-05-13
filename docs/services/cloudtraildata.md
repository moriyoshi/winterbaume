# winterbaume-cloudtraildata

AWS CloudTrail Data service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | CloudTrail Data |
| AWS model | `cloudtrail-data` |
| Protocol | restJson1 |
| winterbaume coverage | 1/1 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/1 operations (0.0%) |
| moto coverage | 0/1 operations (0.0%) |
| floci coverage | 0/1 operations (0.0%) |
| kumo coverage | 0/1 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws cloudtraildata help
```

## Example

```rust
use aws_sdk_cloudtraildata::config::BehaviorVersion;
use aws_sdk_cloudtraildata::types::AuditEvent;
use winterbaume_cloudtraildata::CloudTrailDataService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CloudTrailDataService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudtraildata::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cloudtraildata::Client::new(&config);
    let resp = client
        .put_audit_events()
        .channel_arn("arn:aws:cloudtrail:us-east-1:123:channel/demo")
        .audit_events(
            AuditEvent::builder()
                .id("client-1")
                .event_data(r#"{"eventName":"DemoEvent"}"#)
                .build()
                .expect("event"),
        )
        .send()
        .await
        .expect("put_audit_events should succeed");
    println!("Ingested {} events", resp.successful().len());
}
```

## Implemented APIs (1)

- `PutAuditEvents`
