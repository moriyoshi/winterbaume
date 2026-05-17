# winterbaume-connectcontactlens

AWS Connect Contact Lens service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Connect Contact Lens |
| AWS model | `connect-contact-lens` |
| Protocol | restJson1 |
| winterbaume coverage | 0/1 operations (0.0%) |
| stubs (routed, returns empty/default) | 0/1 operations (0.0%) |
| moto coverage | 0/1 operations (0.0%) |
| floci coverage | 0/1 operations (0.0%) |
| kumo coverage | 0/1 operations (0.0%) |
| Coverage report date | 2026-05-16 |

## Server-mode usage

Install `winterbaume-server` from crates.io or run it from a workspace checkout, then point the AWS CLI at it:

```sh
# Installed binary ( from crates.io ):
cargo install winterbaume-server
winterbaume-server --host 127.0.0.1 --port 5555

# Or, from a workspace checkout:
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws connectcontactlens help
```

## Example

```rust
use aws_sdk_connectcontactlens::config::BehaviorVersion;
use winterbaume_connectcontactlens::ConnectContactLensService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ConnectContactLensService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_connectcontactlens::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_connectcontactlens::Client::new(&config);
    let resp = client
        .list_realtime_contact_analysis_segments()
        .instance_id("11111111-1111-1111-1111-111111111111")
        .contact_id("22222222-2222-2222-2222-222222222222")
        .send()
        .await
        .expect("list");
    println!("Segments: {}", resp.segments().len());
}
```

## Implemented APIs (0)

No modeled operations are currently detected as implemented.

<details><summary>Not yet implemented APIs (1)</summary>

- `ListRealtimeContactAnalysisSegments`

</details>
