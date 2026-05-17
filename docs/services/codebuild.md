# winterbaume-codebuild

CodeBuild service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | CodeBuild |
| AWS model | `codebuild` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 29/59 operations (49.2%) |
| stubs (routed, returns empty/default) | 0/59 operations (0.0%) |
| moto coverage | 9/59 operations (15.3%) |
| floci coverage | 0/59 operations (0.0%) |
| kumo coverage | 0/59 operations (0.0%) |
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
aws codebuild list-projects
```

## Current Network Resource Stub Semantics

CodeBuild currently has generated and snapshot fields for VPC configuration, but the implemented state does not create network resources.

- Project snapshots expose a `vpc_config` JSON slot, and current snapshot construction sets it to `None`.
- Builds and projects do not allocate ENIs, attach to subnets, or track security group membership.
- Networking-related Smithy fields are therefore effectively ignored by the implemented service state unless a future handler explicitly stores them.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_codebuild::config::BehaviorVersion;
use winterbaume_codebuild::CodeBuildService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CodeBuildService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codebuild::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_codebuild::Client::new(&config);

    let resp = client
        .list_projects()
        .send()
        .await
        .expect("list_projects should succeed");
    println!("Projects: {}", resp.projects().len());
}
```

## Implemented APIs (29)

- `BatchDeleteBuilds`
- `BatchGetBuilds`
- `BatchGetProjects`
- `BatchGetReportGroups`
- `CreateProject`
- `CreateReportGroup`
- `CreateWebhook`
- `DeleteProject`
- `DeleteReportGroup`
- `DeleteResourcePolicy`
- `DeleteSourceCredentials`
- `DeleteWebhook`
- `DescribeTestCases`
- `GetResourcePolicy`
- `ImportSourceCredentials`
- `InvalidateProjectCache`
- `ListBuilds`
- `ListBuildsForProject`
- `ListProjects`
- `ListReportGroups`
- `ListReportsForReportGroup`
- `ListSourceCredentials`
- `PutResourcePolicy`
- `RetryBuild`
- `StartBuild`
- `StopBuild`
- `UpdateProject`
- `UpdateReportGroup`
- `UpdateWebhook`

<details><summary>Not yet implemented APIs (30)</summary>

- `BatchGetBuildBatches`
- `BatchGetCommandExecutions`
- `BatchGetFleets`
- `BatchGetReports`
- `BatchGetSandboxes`
- `CreateFleet`
- `DeleteBuildBatch`
- `DeleteFleet`
- `DeleteReport`
- `DescribeCodeCoverages`
- `GetReportGroupTrend`
- `ListBuildBatches`
- `ListBuildBatchesForProject`
- `ListCommandExecutionsForSandbox`
- `ListCuratedEnvironmentImages`
- `ListFleets`
- `ListReports`
- `ListSandboxes`
- `ListSandboxesForProject`
- `ListSharedProjects`
- `ListSharedReportGroups`
- `RetryBuildBatch`
- `StartBuildBatch`
- `StartCommandExecution`
- `StartSandbox`
- `StartSandboxConnection`
- `StopBuildBatch`
- `StopSandbox`
- `UpdateFleet`
- `UpdateProjectVisibility`

</details>
