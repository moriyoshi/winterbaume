# winterbaume-bcmdashboards

AWS BCM Dashboards service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | BCM Dashboards |
| AWS model | `bcm-dashboards` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 9/15 operations (60.0%) |
| stubs (routed, returns empty/default) | 0/15 operations (0.0%) |
| moto coverage | 0/15 operations (0.0%) |
| floci coverage | 0/15 operations (0.0%) |
| kumo coverage | 0/15 operations (0.0%) |
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
aws bcmdashboards help
```

## Example

```rust
use aws_sdk_bcmdashboards::config::BehaviorVersion;
use winterbaume_bcmdashboards::BcmDashboardsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BcmDashboardsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_bcmdashboards::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_bcmdashboards::Client::new(&config);

    let resp = client
        .list_dashboards()
        .send()
        .await
        .expect("list_dashboards should succeed");
    println!("BCM dashboards: {}", resp.dashboards().len());
}
```

## Implemented APIs (9)

- `CreateDashboard`
- `DeleteDashboard`
- `GetDashboard`
- `GetResourcePolicy`
- `ListDashboards`
- `ListTagsForResource`
- `TagResource`
- `UntagResource`
- `UpdateDashboard`

<details><summary>Not yet implemented APIs (6)</summary>

- `CreateScheduledReport`
- `DeleteScheduledReport`
- `ExecuteScheduledReport`
- `GetScheduledReport`
- `ListScheduledReports`
- `UpdateScheduledReport`

</details>
