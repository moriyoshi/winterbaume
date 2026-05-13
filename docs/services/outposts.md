# winterbaume-outposts

AWS Outposts service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Outposts |
| AWS model | `outposts` |
| Protocol | restJson1 |
| winterbaume coverage | 13/37 operations (35.1%) |
| stubs (routed, returns empty/default) | 0/37 operations (0.0%) |
| moto coverage | 0/37 operations (0.0%) |
| floci coverage | 0/37 operations (0.0%) |
| kumo coverage | 0/37 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws outposts help
```

## Example

```rust
use aws_sdk_outposts::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_outposts::OutpostsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(OutpostsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_outposts::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_outposts::Client::new(&config);

    let resp = client
        .list_sites()
        .send()
        .await
        .expect("list_sites should succeed");
    println!("Outposts sites: {}", resp.sites().len());
}
```

## Implemented APIs (13)

- `CreateOutpost`
- `CreateSite`
- `DeleteOutpost`
- `DeleteSite`
- `GetOutpost`
- `GetSite`
- `ListOutposts`
- `ListSites`
- `ListTagsForResource`
- `TagResource`
- `UntagResource`
- `UpdateOutpost`
- `UpdateSite`

<details><summary>Not yet implemented APIs (24)</summary>

- `CancelCapacityTask`
- `CancelOrder`
- `CreateOrder`
- `CreateRenewal`
- `GetCapacityTask`
- `GetCatalogItem`
- `GetConnection`
- `GetOrder`
- `GetOutpostBillingInformation`
- `GetOutpostInstanceTypes`
- `GetOutpostSupportedInstanceTypes`
- `GetRenewalPricing`
- `GetSiteAddress`
- `ListAssetInstances`
- `ListAssets`
- `ListBlockingInstancesForCapacityTask`
- `ListCapacityTasks`
- `ListCatalogItems`
- `ListOrders`
- `StartCapacityTask`
- `StartConnection`
- `StartOutpostDecommission`
- `UpdateSiteAddress`
- `UpdateSiteRackPhysicalProperties`

</details>
