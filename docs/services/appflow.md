# winterbaume-appflow

AWS AppFlow service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | AppFlow |
| AWS model | `appflow` |
| Protocol | restJson1 |
| winterbaume coverage | 9/25 operations (36.0%) |
| stubs (routed, returns empty/default) | 0/25 operations (0.0%) |
| moto coverage | 0/25 operations (0.0%) |
| floci coverage | 0/25 operations (0.0%) |
| kumo coverage | 0/25 operations (0.0%) |
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
aws appflow help
```

## Example

```rust
use aws_sdk_appflow::config::BehaviorVersion;
use winterbaume_appflow::AppFlowService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AppFlowService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appflow::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_appflow::Client::new(&config);

    let resp = client
        .list_flows()
        .send()
        .await
        .expect("list_flows should succeed");
    println!("AppFlow flows: {}", resp.flows().len());
}
```

## Implemented APIs (9)

- `CreateFlow`
- `DeleteFlow`
- `DescribeFlow`
- `ListFlows`
- `StartFlow`
- `StopFlow`
- `TagResource`
- `UntagResource`
- `UpdateFlow`

<details><summary>Not yet implemented APIs (16)</summary>

- `CancelFlowExecutions`
- `CreateConnectorProfile`
- `DeleteConnectorProfile`
- `DescribeConnector`
- `DescribeConnectorEntity`
- `DescribeConnectorProfiles`
- `DescribeConnectors`
- `DescribeFlowExecutionRecords`
- `ListConnectorEntities`
- `ListConnectors`
- `ListTagsForResource`
- `RegisterConnector`
- `ResetConnectorMetadataCache`
- `UnregisterConnector`
- `UpdateConnectorProfile`
- `UpdateConnectorRegistration`

</details>
