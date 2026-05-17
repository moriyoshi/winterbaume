# winterbaume-arczonalshift

AWS ARC Zonal Shift service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | ARC Zonal Shift |
| AWS model | `arc-zonal-shift` |
| Protocol | restJson1 |
| winterbaume coverage | 14/15 operations (93.3%) |
| stubs (routed, returns empty/default) | 1/15 operations (6.7%) |
| moto coverage | 0/15 operations (0.0%) |
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
aws arczonalshift help
```

## Example

```rust
use aws_sdk_arczonalshift::config::BehaviorVersion;
use winterbaume_arczonalshift::ArcZonalShiftService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ArcZonalShiftService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_arczonalshift::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_arczonalshift::Client::new(&config);

    let resp = client
        .list_managed_resources()
        .send()
        .await
        .expect("list_managed_resources should succeed");
    println!("ARC managed resources: {}", resp.items().len());
}
```

## Implemented APIs (14)

- `CancelPracticeRun`
- `CancelZonalShift`
- `CreatePracticeRunConfiguration`
- `DeletePracticeRunConfiguration`
- `GetAutoshiftObserverNotificationStatus`
- `GetManagedResource`
- `ListManagedResources`
- `ListZonalShifts`
- `StartPracticeRun`
- `StartZonalShift`
- `UpdateAutoshiftObserverNotificationStatus`
- `UpdatePracticeRunConfiguration`
- `UpdateZonalAutoshiftConfiguration`
- `UpdateZonalShift`

<details><summary>Stubbed APIs (1) &mdash; routed but return an empty/default response</summary>

- `ListAutoshifts`

</details>
