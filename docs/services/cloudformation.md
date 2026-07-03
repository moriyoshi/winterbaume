# winterbaume-cloudformation

CloudFormation service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | CloudFormation |
| AWS model | `cloudformation` |
| Protocol | awsQuery |
| winterbaume coverage | 40/90 operations (44.4%) |
| stubs (routed, returns empty/default) | 3/90 operations (3.3%) |
| moto coverage | 33/90 operations (36.7%) |
| floci coverage | 0/90 operations (0.0%) |
| kumo coverage | 8/90 operations (8.9%) |
| fakecloud coverage | 90/90 operations (100.0%) |
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
aws cloudformation list-stacks
```

## Example

```rust
use aws_sdk_cloudformation::config::BehaviorVersion;
use winterbaume_cloudformation::CloudFormationService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CloudFormationService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudformation::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cloudformation::Client::new(&config);

    let resp = client
        .list_stacks()
        .send()
        .await
        .expect("list_stacks should succeed");
    println!("CloudFormation stacks: {:?}", resp.stack_summaries());
}
```

## Implemented APIs (40)

- `CancelUpdateStack`
- `ContinueUpdateRollback`
- `CreateChangeSet`
- `CreateStack`
- `CreateStackInstances`
- `CreateStackSet`
- `DeleteChangeSet`
- `DeleteStack`
- `DeleteStackInstances`
- `DeleteStackSet`
- `DescribeChangeSet`
- `DescribeStackEvents`
- `DescribeStackInstance`
- `DescribeStackResource`
- `DescribeStackResources`
- `DescribeStackSet`
- `DescribeStackSetOperation`
- `DescribeStacks`
- `ExecuteChangeSet`
- `GetStackPolicy`
- `GetTemplate`
- `GetTemplateSummary`
- `ListChangeSets`
- `ListExports`
- `ListImports`
- `ListStackInstances`
- `ListStackResources`
- `ListStackSetOperationResults`
- `ListStackSetOperations`
- `ListStackSets`
- `ListStacks`
- `ListTypes`
- `RollbackStack`
- `SetStackPolicy`
- `SignalResource`
- `StopStackSetOperation`
- `UpdateStack`
- `UpdateStackInstances`
- `UpdateStackSet`
- `UpdateTerminationProtection`

<details><summary>Stubbed APIs (3) &mdash; routed but return an empty/default response</summary>

- `DescribeAccountLimits`
- `EstimateTemplateCost`
- `ValidateTemplate`

</details>

<details><summary>Not yet implemented APIs (47)</summary>

- `ActivateOrganizationsAccess` (implemented by fakecloud)
- `ActivateType` (implemented by fakecloud)
- `BatchDescribeTypeConfigurations` (implemented by fakecloud)
- `CreateGeneratedTemplate` (implemented by fakecloud)
- `CreateStackRefactor` (implemented by fakecloud)
- `DeactivateOrganizationsAccess` (implemented by fakecloud)
- `DeactivateType` (implemented by fakecloud)
- `DeleteGeneratedTemplate` (implemented by fakecloud)
- `DeregisterType` (implemented by fakecloud)
- `DescribeChangeSetHooks` (implemented by fakecloud)
- `DescribeEvents` (implemented by fakecloud)
- `DescribeGeneratedTemplate` (implemented by fakecloud)
- `DescribeOrganizationsAccess` (implemented by fakecloud)
- `DescribePublisher` (implemented by fakecloud)
- `DescribeResourceScan` (implemented by fakecloud)
- `DescribeStackDriftDetectionStatus` (implemented by fakecloud)
- `DescribeStackRefactor` (implemented by fakecloud)
- `DescribeStackResourceDrifts` (implemented by fakecloud)
- `DescribeType` (implemented by fakecloud)
- `DescribeTypeRegistration` (implemented by fakecloud)
- `DetectStackDrift` (implemented by fakecloud)
- `DetectStackResourceDrift` (implemented by fakecloud)
- `DetectStackSetDrift` (implemented by fakecloud)
- `ExecuteStackRefactor` (implemented by fakecloud)
- `GetGeneratedTemplate` (implemented by fakecloud)
- `GetHookResult` (implemented by fakecloud)
- `ImportStacksToStackSet` (implemented by fakecloud)
- `ListGeneratedTemplates` (implemented by fakecloud)
- `ListHookResults` (implemented by fakecloud)
- `ListResourceScanRelatedResources` (implemented by fakecloud)
- `ListResourceScanResources` (implemented by fakecloud)
- `ListResourceScans` (implemented by fakecloud)
- `ListStackInstanceResourceDrifts` (implemented by fakecloud)
- `ListStackRefactorActions` (implemented by fakecloud)
- `ListStackRefactors` (implemented by fakecloud)
- `ListStackSetAutoDeploymentTargets` (implemented by fakecloud)
- `ListTypeRegistrations` (implemented by fakecloud)
- `ListTypeVersions` (implemented by fakecloud)
- `PublishType` (implemented by fakecloud)
- `RecordHandlerProgress` (implemented by fakecloud)
- `RegisterPublisher` (implemented by fakecloud)
- `RegisterType` (implemented by fakecloud)
- `SetTypeConfiguration` (implemented by fakecloud)
- `SetTypeDefaultVersion` (implemented by fakecloud)
- `StartResourceScan` (implemented by fakecloud)
- `TestType` (implemented by fakecloud)
- `UpdateGeneratedTemplate` (implemented by fakecloud)

</details>
