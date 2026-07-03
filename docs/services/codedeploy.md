# winterbaume-codedeploy

CodeDeploy service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | CodeDeploy |
| AWS model | `codedeploy` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 15/47 operations (31.9%) |
| stubs (routed, returns empty/default) | 0/47 operations (0.0%) |
| moto coverage | 14/47 operations (29.8%) |
| floci coverage | 0/47 operations (0.0%) |
| kumo coverage | 0/47 operations (0.0%) |
| fakecloud coverage | 0/47 operations (0.0%) |
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
aws deploy list-applications
```

## Example

```rust
use aws_sdk_codedeploy::config::BehaviorVersion;
use winterbaume_codedeploy::CodeDeployService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CodeDeployService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codedeploy::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_codedeploy::Client::new(&config);

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");
    println!("Applications: {}", resp.applications().len());
}
```

## Implemented APIs (15)

- `BatchGetApplications`
- `BatchGetDeployments`
- `CreateApplication`
- `CreateDeployment`
- `CreateDeploymentGroup`
- `DeleteApplication`
- `GetApplication`
- `GetDeployment`
- `GetDeploymentGroup`
- `ListApplications`
- `ListDeploymentGroups`
- `ListDeployments`
- `ListTagsForResource`
- `TagResource`
- `UntagResource`

<details><summary>Not yet implemented APIs (32)</summary>

- `AddTagsToOnPremisesInstances`
- `BatchGetApplicationRevisions`
- `BatchGetDeploymentGroups`
- `BatchGetDeploymentInstances`
- `BatchGetDeploymentTargets`
- `BatchGetOnPremisesInstances`
- `ContinueDeployment`
- `CreateDeploymentConfig`
- `DeleteDeploymentConfig`
- `DeleteDeploymentGroup`
- `DeleteGitHubAccountToken`
- `DeleteResourcesByExternalId`
- `DeregisterOnPremisesInstance`
- `GetApplicationRevision`
- `GetDeploymentConfig`
- `GetDeploymentInstance`
- `GetDeploymentTarget`
- `GetOnPremisesInstance`
- `ListApplicationRevisions`
- `ListDeploymentConfigs`
- `ListDeploymentInstances`
- `ListDeploymentTargets`
- `ListGitHubAccountTokenNames`
- `ListOnPremisesInstances`
- `PutLifecycleEventHookExecutionStatus`
- `RegisterApplicationRevision`
- `RegisterOnPremisesInstance`
- `RemoveTagsFromOnPremisesInstances`
- `SkipWaitTimeForInstanceTermination`
- `StopDeployment`
- `UpdateApplication`
- `UpdateDeploymentGroup`

</details>
