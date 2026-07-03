# winterbaume-kinesisanalyticsv2

Kinesis Data Analytics V2 service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Kinesis Analytics V2 |
| AWS model | `kinesis-analytics-v2` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 32/33 operations (97.0%) |
| stubs (routed, returns empty/default) | 1/33 operations (3.0%) |
| moto coverage | 0/33 operations (0.0%) |
| floci coverage | 0/33 operations (0.0%) |
| kumo coverage | 0/33 operations (0.0%) |
| fakecloud coverage | 0/33 operations (0.0%) |
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
aws kinesisanalyticsv2 list-applications
```

## Current Network Resource Stub Semantics

Kinesis Analytics V2 currently has a minimal VPC-configuration placeholder.

- `AddApplicationVpcConfiguration` is routed and returns a VPC configuration description, but the state path does not use EC2 to derive VPC ID or validate the supplied subnets and security groups.
- `DeleteApplicationVpcConfiguration` removes or acknowledges the local placeholder by application name and VPC configuration ID.
- Application descriptions expose whatever local VPC configuration description the Kinesis Analytics state has retained.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_kinesisanalyticsv2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_kinesisanalyticsv2::KinesisAnalyticsV2Service;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(KinesisAnalyticsV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_kinesisanalyticsv2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_kinesisanalyticsv2::Client::new(&config);

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");
    println!(
        "Kinesis Analytics V2 applications: {}",
        resp.application_summaries().len()
    );
}
```

## Implemented APIs (32)

- `AddApplicationCloudWatchLoggingOption`
- `AddApplicationInput`
- `AddApplicationInputProcessingConfiguration`
- `AddApplicationOutput`
- `AddApplicationReferenceDataSource`
- `AddApplicationVpcConfiguration`
- `CreateApplication`
- `CreateApplicationPresignedUrl`
- `CreateApplicationSnapshot`
- `DeleteApplication`
- `DeleteApplicationCloudWatchLoggingOption`
- `DeleteApplicationInputProcessingConfiguration`
- `DeleteApplicationOutput`
- `DeleteApplicationReferenceDataSource`
- `DeleteApplicationSnapshot`
- `DeleteApplicationVpcConfiguration`
- `DescribeApplication`
- `DescribeApplicationOperation`
- `DescribeApplicationSnapshot`
- `DescribeApplicationVersion`
- `ListApplicationOperations`
- `ListApplicationSnapshots`
- `ListApplicationVersions`
- `ListApplications`
- `ListTagsForResource`
- `RollbackApplication`
- `StartApplication`
- `StopApplication`
- `TagResource`
- `UntagResource`
- `UpdateApplication`
- `UpdateApplicationMaintenanceConfiguration`

<details><summary>Stubbed APIs (1) &mdash; routed but return an empty/default response</summary>

- `DiscoverInputSchema`

</details>
