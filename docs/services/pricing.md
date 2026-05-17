# winterbaume-pricing

AWS Pricing service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Pricing |
| AWS model | `pricing` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 5/5 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/5 operations (0.0%) |
| moto coverage | 0/5 operations (0.0%) |
| floci coverage | 0/5 operations (0.0%) |
| kumo coverage | 0/5 operations (0.0%) |
| Coverage report date | 2026-05-17 |

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
aws pricing help
```

## Example

```rust
use aws_sdk_pricing::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_pricing::PricingService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(PricingService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_pricing::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_pricing::Client::new(&config);
    let resp = client
        .describe_services()
        .send()
        .await
        .expect("describe_services should succeed");
    for svc in resp.services() {
        println!("Service: {:?}", svc.service_code());
    }
}
```

## Implemented APIs (5)

- `DescribeServices`
- `GetAttributeValues`
- `GetPriceListFileUrl`
- `GetProducts`
- `ListPriceLists`
