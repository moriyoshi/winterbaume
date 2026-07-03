# winterbaume-applicationdiscoveryservice

AWS Application Discovery Service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Application Discovery Service |
| AWS model | `application-discovery-service` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 28/28 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/28 operations (0.0%) |
| moto coverage | 0/28 operations (0.0%) |
| floci coverage | 0/28 operations (0.0%) |
| kumo coverage | 0/28 operations (0.0%) |
| fakecloud coverage | 0/28 operations (0.0%) |
| Coverage report date | 2026-07-03 |

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
aws applicationdiscoveryservice help
```

## Example

```rust
use aws_sdk_applicationdiscovery::config::BehaviorVersion;
use winterbaume_applicationdiscoveryservice::ApplicationDiscoveryService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ApplicationDiscoveryService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_applicationdiscovery::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_applicationdiscovery::Client::new(&config);

    let resp = client
        .describe_agents()
        .send()
        .await
        .expect("describe_agents should succeed");
    println!("Application Discovery agents: {}", resp.agents_info().len());
}
```

## Implemented APIs (28)

- `AssociateConfigurationItemsToApplication`
- `BatchDeleteAgents`
- `BatchDeleteImportData`
- `CreateApplication`
- `CreateTags`
- `DeleteApplications`
- `DeleteTags`
- `DescribeAgents`
- `DescribeBatchDeleteConfigurationTask`
- `DescribeConfigurations`
- `DescribeContinuousExports`
- `DescribeExportConfigurations`
- `DescribeExportTasks`
- `DescribeImportTasks`
- `DescribeTags`
- `DisassociateConfigurationItemsFromApplication`
- `ExportConfigurations`
- `GetDiscoverySummary`
- `ListConfigurations`
- `ListServerNeighbors`
- `StartBatchDeleteConfigurationTask`
- `StartContinuousExport`
- `StartDataCollectionByAgentIds`
- `StartExportTask`
- `StartImportTask`
- `StopContinuousExport`
- `StopDataCollectionByAgentIds`
- `UpdateApplication`
