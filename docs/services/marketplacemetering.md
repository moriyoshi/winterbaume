# winterbaume-marketplacemetering

AWS Marketplace Metering service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Marketplace Metering |
| AWS model | `marketplace-metering` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 4/4 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/4 operations (0.0%) |
| moto coverage | 0/4 operations (0.0%) |
| floci coverage | 0/4 operations (0.0%) |
| kumo coverage | 0/4 operations (0.0%) |
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
aws meteringmarketplace resolve-customer --registration-token dummy-token
```

## Example

```rust
use aws_sdk_marketplacemetering::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_marketplacemetering::MarketplaceMeteringService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(MarketplaceMeteringService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_marketplacemetering::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_marketplacemetering::Client::new(&config);

    // Marketplace Metering requires a valid product code and customer token.
    // This example demonstrates client setup for the MeteringMarketplace service.
    println!("MeteringMarketplace client ready. Use meter_usage() to report usage.");
    let _client = client;
}
```

## Implemented APIs (4)

- `BatchMeterUsage`
- `MeterUsage`
- `RegisterUsage`
- `ResolveCustomer`
