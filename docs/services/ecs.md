# winterbaume-ecs

ECS service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | ECS |
| AWS model | `ecs` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 63/76 operations (82.9%) |
| stubs (routed, returns empty/default) | 1/76 operations (1.3%) |
| moto coverage | 45/76 operations (59.2%) |
| floci coverage | 0/76 operations (0.0%) |
| kumo coverage | 12/76 operations (15.8%) |
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
aws ecs list-clusters
```

## Current Network Resource Stub Semantics

ECS currently stores `awsvpc`-style placement inputs as task and service metadata rather than provisioning EC2 networking.

- Task and service request paths preserve network configuration values such as subnets, security groups, and public-IP assignment when the implemented handler stores the surrounding task or service.
- ECS resources do not allocate ENIs, attach them to container instances, or track subnet capacity.
- Load balancer, service discovery, and network configuration references are service-local and are not reconciled with ELB, Cloud Map, or EC2 state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_ecs::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ecs::EcsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(EcsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ecs::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_ecs::Client::new(&config);

    let resp = client
        .list_clusters()
        .send()
        .await
        .expect("list_clusters should succeed");
    println!("ECS clusters: {}", resp.cluster_arns().len());
}
```

## Implemented APIs (63)

- `CreateCapacityProvider`
- `CreateCluster`
- `CreateExpressGatewayService`
- `CreateService`
- `CreateTaskSet`
- `DeleteAccountSetting`
- `DeleteAttributes`
- `DeleteCapacityProvider`
- `DeleteCluster`
- `DeleteExpressGatewayService`
- `DeleteService`
- `DeleteTaskDefinitions`
- `DeleteTaskSet`
- `DeregisterContainerInstance`
- `DeregisterTaskDefinition`
- `DescribeCapacityProviders`
- `DescribeClusters`
- `DescribeContainerInstances`
- `DescribeExpressGatewayService`
- `DescribeServiceDeployments`
- `DescribeServiceRevisions`
- `DescribeServices`
- `DescribeTaskDefinition`
- `DescribeTaskSets`
- `DescribeTasks`
- `DiscoverPollEndpoint`
- `GetTaskProtection`
- `ListAccountSettings`
- `ListAttributes`
- `ListClusters`
- `ListContainerInstances`
- `ListServiceDeployments`
- `ListServices`
- `ListServicesByNamespace`
- `ListTagsForResource`
- `ListTaskDefinitionFamilies`
- `ListTaskDefinitions`
- `ListTasks`
- `PutAccountSetting`
- `PutAccountSettingDefault`
- `PutAttributes`
- `PutClusterCapacityProviders`
- `RegisterContainerInstance`
- `RegisterTaskDefinition`
- `RunTask`
- `StartTask`
- `StopServiceDeployment`
- `StopTask`
- `SubmitAttachmentStateChanges`
- `SubmitContainerStateChange`
- `SubmitTaskStateChange`
- `TagResource`
- `UntagResource`
- `UpdateCapacityProvider`
- `UpdateCluster`
- `UpdateClusterSettings`
- `UpdateContainerAgent`
- `UpdateContainerInstancesState`
- `UpdateExpressGatewayService`
- `UpdateService`
- `UpdateServicePrimaryTaskSet`
- `UpdateTaskProtection`
- `UpdateTaskSet`

<details><summary>Stubbed APIs (1) &mdash; routed but return an empty/default response</summary>

- `ExecuteCommand`

</details>

<details><summary>Not yet implemented APIs (12)</summary>

- `CreateDaemon`
- `DeleteDaemon`
- `DeleteDaemonTaskDefinition`
- `DescribeDaemon`
- `DescribeDaemonDeployments`
- `DescribeDaemonRevisions`
- `DescribeDaemonTaskDefinition`
- `ListDaemonDeployments`
- `ListDaemonTaskDefinitions`
- `ListDaemons`
- `RegisterDaemonTaskDefinition`
- `UpdateDaemon`

</details>
