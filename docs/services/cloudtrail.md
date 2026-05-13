# winterbaume-cloudtrail

CloudTrail service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | CloudTrail |
| AWS model | `cloudtrail` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 21/60 operations (35.0%) |
| stubs (routed, returns empty/default) | 2/60 operations (3.3%) |
| moto coverage | 16/60 operations (26.7%) |
| floci coverage | 0/60 operations (0.0%) |
| kumo coverage | 8/60 operations (13.3%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws cloudtrail describe-trails
```

## Example

```rust
use aws_sdk_cloudtrail::config::BehaviorVersion;
use winterbaume_cloudtrail::CloudTrailService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CloudTrailService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudtrail::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cloudtrail::Client::new(&config);

    let resp = client
        .describe_trails()
        .send()
        .await
        .expect("describe_trails should succeed");
    println!("Trails: {}", resp.trail_list().len());
}
```

## Implemented APIs (21)

- `AddTags`
- `CreateEventDataStore`
- `CreateTrail`
- `DeleteEventDataStore`
- `DeleteTrail`
- `DescribeTrails`
- `GetEventDataStore`
- `GetEventSelectors`
- `GetInsightSelectors`
- `GetTrail`
- `GetTrailStatus`
- `ListEventDataStores`
- `ListTags`
- `ListTrails`
- `PutEventSelectors`
- `PutInsightSelectors`
- `RemoveTags`
- `StartLogging`
- `StopLogging`
- `UpdateEventDataStore`
- `UpdateTrail`

<details><summary>Stubbed APIs (2) &mdash; routed but return an empty/default response</summary>

- `ListPublicKeys`
- `LookupEvents`

</details>

<details><summary>Not yet implemented APIs (37)</summary>

- `CancelQuery`
- `CreateChannel`
- `CreateDashboard`
- `DeleteChannel`
- `DeleteDashboard`
- `DeleteResourcePolicy`
- `DeregisterOrganizationDelegatedAdmin`
- `DescribeQuery`
- `DisableFederation`
- `EnableFederation`
- `GenerateQuery`
- `GetChannel`
- `GetDashboard`
- `GetEventConfiguration`
- `GetImport`
- `GetQueryResults`
- `GetResourcePolicy`
- `ListChannels`
- `ListDashboards`
- `ListImportFailures`
- `ListImports`
- `ListInsightsData`
- `ListInsightsMetricData`
- `ListQueries`
- `PutEventConfiguration`
- `PutResourcePolicy`
- `RegisterOrganizationDelegatedAdmin`
- `RestoreEventDataStore`
- `SearchSampleQueries`
- `StartDashboardRefresh`
- `StartEventDataStoreIngestion`
- `StartImport`
- `StartQuery`
- `StopEventDataStoreIngestion`
- `StopImport`
- `UpdateChannel`
- `UpdateDashboard`

</details>
