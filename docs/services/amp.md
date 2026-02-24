# winterbaume-amp

Amazon Managed Prometheus service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | AMP/Prometheus |
| AWS model | `amp` |
| Protocol | restJson1 |
| winterbaume coverage | 17/44 operations (38.6%) |
| stubs (routed, returns empty/default) | 0/44 operations (0.0%) |
| moto coverage | 17/44 operations (38.6%) |
| floci coverage | 0/44 operations (0.0%) |
| kumo coverage | 0/44 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws amp list-workspaces
```

## Example

```rust
use aws_sdk_amp::config::BehaviorVersion;
use winterbaume_amp::AmpService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(AmpService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_amp::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_amp::Client::new(&config);

    let resp = client
        .list_workspaces()
        .send()
        .await
        .expect("list_workspaces should succeed");
    println!("Workspaces: {}", resp.workspaces().len());
}
```

## Implemented APIs (17)

- `CreateLoggingConfiguration`
- `CreateRuleGroupsNamespace`
- `CreateWorkspace`
- `DeleteLoggingConfiguration`
- `DeleteRuleGroupsNamespace`
- `DeleteWorkspace`
- `DescribeLoggingConfiguration`
- `DescribeRuleGroupsNamespace`
- `DescribeWorkspace`
- `ListRuleGroupsNamespaces`
- `ListTagsForResource`
- `ListWorkspaces`
- `PutRuleGroupsNamespace`
- `TagResource`
- `UntagResource`
- `UpdateLoggingConfiguration`
- `UpdateWorkspaceAlias`

<details><summary>Not yet implemented APIs (27)</summary>

- `CreateAlertManagerDefinition`
- `CreateAnomalyDetector`
- `CreateQueryLoggingConfiguration`
- `CreateScraper`
- `DeleteAlertManagerDefinition`
- `DeleteAnomalyDetector`
- `DeleteQueryLoggingConfiguration`
- `DeleteResourcePolicy`
- `DeleteScraper`
- `DeleteScraperLoggingConfiguration`
- `DescribeAlertManagerDefinition`
- `DescribeAnomalyDetector`
- `DescribeQueryLoggingConfiguration`
- `DescribeResourcePolicy`
- `DescribeScraper`
- `DescribeScraperLoggingConfiguration`
- `DescribeWorkspaceConfiguration`
- `GetDefaultScraperConfiguration`
- `ListAnomalyDetectors`
- `ListScrapers`
- `PutAlertManagerDefinition`
- `PutAnomalyDetector`
- `PutResourcePolicy`
- `UpdateQueryLoggingConfiguration`
- `UpdateScraper`
- `UpdateScraperLoggingConfiguration`
- `UpdateWorkspaceConfiguration`

</details>
