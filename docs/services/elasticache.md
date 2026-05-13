# winterbaume-elasticache

ElastiCache service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | ElastiCache |
| AWS model | `elasticache` |
| Protocol | awsQuery |
| winterbaume coverage | 24/75 operations (32.0%) |
| stubs (routed, returns empty/default) | 0/75 operations (0.0%) |
| moto coverage | 17/75 operations (22.7%) |
| floci coverage | 0/75 operations (0.0%) |
| kumo coverage | 7/75 operations (9.3%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws elasticache describe-cache-clusters
```

## Current Network Resource Stub Semantics

ElastiCache currently keeps subnet groups and security group references inside ElastiCache state.

- Cache subnet groups store supplied subnet IDs and a local VPC ID field; describe operations return that stored metadata.
- Cache clusters and replication groups store subnet group names and security group references without resolving them through EC2.
- There is no cross-service check that subnet group subnets share a VPC, that security groups exist, or that a subnet group is free of dependent clusters before every mutation path.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_elasticache::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_elasticache::ElastiCacheService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ElastiCacheService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_elasticache::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_elasticache::Client::new(&config);

    client
        .create_cache_cluster()
        .cache_cluster_id("example-cluster")
        .engine("redis")
        .cache_node_type("cache.t3.micro")
        .num_cache_nodes(1)
        .send()
        .await
        .expect("create_cache_cluster should succeed");

    let resp = client
        .describe_cache_clusters()
        .send()
        .await
        .expect("describe_cache_clusters should succeed");
    println!("ElastiCache clusters: {}", resp.cache_clusters().len());
}
```

## Implemented APIs (24)

- `AddTagsToResource`
- `CreateCacheCluster`
- `CreateCacheParameterGroup`
- `CreateCacheSecurityGroup`
- `CreateCacheSubnetGroup`
- `CreateReplicationGroup`
- `CreateSnapshot`
- `CreateUser`
- `DeleteCacheCluster`
- `DeleteCacheParameterGroup`
- `DeleteCacheSecurityGroup`
- `DeleteCacheSubnetGroup`
- `DeleteReplicationGroup`
- `DeleteSnapshot`
- `DeleteUser`
- `DescribeCacheClusters`
- `DescribeCacheParameterGroups`
- `DescribeCacheSecurityGroups`
- `DescribeCacheSubnetGroups`
- `DescribeReplicationGroups`
- `DescribeSnapshots`
- `DescribeUsers`
- `ListTagsForResource`
- `RemoveTagsFromResource`

<details><summary>Not yet implemented APIs (51)</summary>

- `AuthorizeCacheSecurityGroupIngress`
- `BatchApplyUpdateAction`
- `BatchStopUpdateAction`
- `CompleteMigration`
- `CopyServerlessCacheSnapshot`
- `CopySnapshot`
- `CreateGlobalReplicationGroup`
- `CreateServerlessCache`
- `CreateServerlessCacheSnapshot`
- `CreateUserGroup`
- `DecreaseNodeGroupsInGlobalReplicationGroup`
- `DecreaseReplicaCount`
- `DeleteGlobalReplicationGroup`
- `DeleteServerlessCache`
- `DeleteServerlessCacheSnapshot`
- `DeleteUserGroup`
- `DescribeCacheEngineVersions`
- `DescribeCacheParameters`
- `DescribeEngineDefaultParameters`
- `DescribeEvents`
- `DescribeGlobalReplicationGroups`
- `DescribeReservedCacheNodes`
- `DescribeReservedCacheNodesOfferings`
- `DescribeServerlessCacheSnapshots`
- `DescribeServerlessCaches`
- `DescribeServiceUpdates`
- `DescribeUpdateActions`
- `DescribeUserGroups`
- `DisassociateGlobalReplicationGroup`
- `ExportServerlessCacheSnapshot`
- `FailoverGlobalReplicationGroup`
- `IncreaseNodeGroupsInGlobalReplicationGroup`
- `IncreaseReplicaCount`
- `ListAllowedNodeTypeModifications`
- `ModifyCacheCluster` (implemented by kumo)
- `ModifyCacheParameterGroup`
- `ModifyCacheSubnetGroup`
- `ModifyGlobalReplicationGroup`
- `ModifyReplicationGroup`
- `ModifyReplicationGroupShardConfiguration`
- `ModifyServerlessCache`
- `ModifyUser`
- `ModifyUserGroup`
- `PurchaseReservedCacheNodesOffering`
- `RebalanceSlotsInGlobalReplicationGroup`
- `RebootCacheCluster`
- `ResetCacheParameterGroup`
- `RevokeCacheSecurityGroupIngress`
- `StartMigration`
- `TestFailover`
- `TestMigration`

</details>
