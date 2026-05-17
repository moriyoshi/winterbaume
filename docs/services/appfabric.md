# winterbaume-appfabric

AWS AppFabric service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | AppFabric |
| AWS model | `appfabric` |
| Protocol | restJson1 |
| winterbaume coverage | 6/26 operations (23.1%) |
| stubs (routed, returns empty/default) | 0/26 operations (0.0%) |
| moto coverage | 0/26 operations (0.0%) |
| floci coverage | 0/26 operations (0.0%) |
| kumo coverage | 0/26 operations (0.0%) |
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
aws appfabric help
```

## Example

```rust
use aws_sdk_appfabric::config::BehaviorVersion;
use winterbaume_appfabric::AppFabricService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AppFabricService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appfabric::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_appfabric::Client::new(&config);

    let resp = client
        .list_app_bundles()
        .send()
        .await
        .expect("list_app_bundles should succeed");
    println!(
        "AppFabric bundles: {}",
        resp.app_bundle_summary_list().len()
    );
}
```

## Implemented APIs (6)

- `CreateAppBundle`
- `DeleteAppBundle`
- `GetAppBundle`
- `ListAppBundles`
- `TagResource`
- `UntagResource`

<details><summary>Not yet implemented APIs (20)</summary>

- `BatchGetUserAccessTasks`
- `ConnectAppAuthorization`
- `CreateAppAuthorization`
- `CreateIngestion`
- `CreateIngestionDestination`
- `DeleteAppAuthorization`
- `DeleteIngestion`
- `DeleteIngestionDestination`
- `GetAppAuthorization`
- `GetIngestion`
- `GetIngestionDestination`
- `ListAppAuthorizations`
- `ListIngestionDestinations`
- `ListIngestions`
- `ListTagsForResource`
- `StartIngestion`
- `StartUserAccessTasks`
- `StopIngestion`
- `UpdateAppAuthorization`
- `UpdateIngestionDestination`

</details>
