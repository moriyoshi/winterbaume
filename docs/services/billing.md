# winterbaume-billing

AWS Billing service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Billing |
| AWS model | `billing` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 12/12 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/12 operations (0.0%) |
| moto coverage | 0/12 operations (0.0%) |
| floci coverage | 0/12 operations (0.0%) |
| kumo coverage | 0/12 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws billing help
```

## Example

```rust
use aws_sdk_billing::config::BehaviorVersion;
use winterbaume_billing::BillingService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BillingService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_billing::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_billing::Client::new(&config);
    let resp = client
        .create_billing_view()
        .name("demo-view")
        .send()
        .await
        .expect("create");
    println!("BillingView: {}", resp.arn());
}
```

## Implemented APIs (12)

- `AssociateSourceViews`
- `CreateBillingView`
- `DeleteBillingView`
- `DisassociateSourceViews`
- `GetBillingView`
- `GetResourcePolicy`
- `ListBillingViews`
- `ListSourceViewsForBillingView`
- `ListTagsForResource`
- `TagResource`
- `UntagResource`
- `UpdateBillingView`
