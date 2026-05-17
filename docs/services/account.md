# winterbaume-account

Account Management service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Account |
| AWS model | `account` |
| Protocol | restJson1 |
| winterbaume coverage | 14/15 operations (93.3%) |
| stubs (routed, returns empty/default) | 1/15 operations (6.7%) |
| moto coverage | 3/15 operations (20.0%) |
| floci coverage | 0/15 operations (0.0%) |
| kumo coverage | 0/15 operations (0.0%) |
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
aws account list-regions
```

## Example

```rust
use aws_sdk_account::config::BehaviorVersion;
use winterbaume_account::AccountService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AccountService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_account::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_account::Client::new(&config);

    let resp = client
        .list_regions()
        .send()
        .await
        .expect("list_regions should succeed");
    println!("Regions: {}", resp.regions().len());
}
```

## Implemented APIs (14)

- `AcceptPrimaryEmailUpdate`
- `DeleteAlternateContact`
- `DisableRegion`
- `EnableRegion`
- `GetAccountInformation`
- `GetAlternateContact`
- `GetContactInformation`
- `GetPrimaryEmail`
- `GetRegionOptStatus`
- `ListRegions`
- `PutAccountName`
- `PutAlternateContact`
- `PutContactInformation`
- `StartPrimaryEmailUpdate`

<details><summary>Stubbed APIs (1) &mdash; routed but return an empty/default response</summary>

- `GetGovCloudAccountInformation`

</details>
