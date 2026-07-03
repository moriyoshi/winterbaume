# winterbaume-emr

EMR (Elastic MapReduce) service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | EMR |
| AWS model | `emr` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 54/60 operations (90.0%) |
| stubs (routed, returns empty/default) | 2/60 operations (3.3%) |
| moto coverage | 26/60 operations (43.3%) |
| floci coverage | 0/60 operations (0.0%) |
| kumo coverage | 0/60 operations (0.0%) |
| fakecloud coverage | 0/60 operations (0.0%) |
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
aws emr list-clusters
```

## Current Network Resource Stub Semantics

EMR currently records VPC-related settings as EMR metadata.

- Security configurations can carry `block_public_security_group_rules`, but the value only affects the stored EMR security configuration document.
- Studio records include VPC ID, subnet IDs, workspace security group ID, and engine security group ID fields when created.
- Cluster placement, Studio lifecycle, and security configuration rules are not checked against EC2 subnet or security group state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_emr::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_emr::EmrService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(EmrService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_emr::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_emr::Client::new(&config);

    let resp = client
        .list_clusters()
        .send()
        .await
        .expect("list_clusters should succeed");
    println!("EMR clusters: {}", resp.clusters().len());
}
```

## Implemented APIs (54)

- `AddInstanceFleet`
- `AddInstanceGroups`
- `AddJobFlowSteps`
- `AddTags`
- `CancelSteps`
- `CreatePersistentAppUI`
- `CreateSecurityConfiguration`
- `CreateStudio`
- `CreateStudioSessionMapping`
- `DeleteSecurityConfiguration`
- `DeleteStudio`
- `DeleteStudioSessionMapping`
- `DescribeCluster`
- `DescribeJobFlows`
- `DescribeNotebookExecution`
- `DescribePersistentAppUI`
- `DescribeReleaseLabel`
- `DescribeSecurityConfiguration`
- `DescribeStep`
- `DescribeStudio`
- `GetAutoTerminationPolicy`
- `GetBlockPublicAccessConfiguration`
- `GetManagedScalingPolicy`
- `GetStudioSessionMapping`
- `ListBootstrapActions`
- `ListClusters`
- `ListInstanceFleets`
- `ListInstanceGroups`
- `ListInstances`
- `ListNotebookExecutions`
- `ListSecurityConfigurations`
- `ListSteps`
- `ListStudioSessionMappings`
- `ListStudios`
- `ModifyCluster`
- `ModifyInstanceFleet`
- `ModifyInstanceGroups`
- `PutAutoScalingPolicy`
- `PutAutoTerminationPolicy`
- `PutBlockPublicAccessConfiguration`
- `PutManagedScalingPolicy`
- `RemoveAutoScalingPolicy`
- `RemoveAutoTerminationPolicy`
- `RemoveManagedScalingPolicy`
- `RemoveTags`
- `RunJobFlow`
- `SetKeepJobFlowAliveWhenNoSteps`
- `SetTerminationProtection`
- `SetVisibleToAllUsers`
- `StartNotebookExecution`
- `StopNotebookExecution`
- `TerminateJobFlows`
- `UpdateStudio`
- `UpdateStudioSessionMapping`

<details><summary>Stubbed APIs (2) &mdash; routed but return an empty/default response</summary>

- `ListReleaseLabels`
- `ListSupportedInstanceTypes`

</details>

<details><summary>Not yet implemented APIs (4)</summary>

- `GetClusterSessionCredentials`
- `GetOnClusterAppUIPresignedURL`
- `GetPersistentAppUIPresignedURL`
- `SetUnhealthyNodeReplacement`

</details>
