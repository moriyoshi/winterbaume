# winterbaume-neptune

Neptune service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Neptune |
| AWS model | `neptune` |
| Protocol | awsQuery |
| winterbaume coverage | 64/70 operations (91.4%) |
| stubs (routed, returns empty/default) | 6/70 operations (8.6%) |
| moto coverage | 47/70 operations (67.1%) |
| floci coverage | 0/70 operations (0.0%) |
| kumo coverage | 6/70 operations (8.6%) |
| Coverage report date | 2026-05-16 |

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
aws neptune describe-db-clusters
```

## Current Network Resource Stub Semantics

Neptune currently stores DB subnet groups and VPC security group references locally.

- DB subnet groups keep raw subnet IDs and an optional or unset VPC ID field in Neptune state.
- DB instances and clusters store DB subnet group names and VPC security group IDs without EC2 resolution.
- Deletion and describe paths check Neptune-local resource maps, not EC2 dependencies.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_neptune::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_neptune::NeptuneService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(NeptuneService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_neptune::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_neptune::Client::new(&config);

    let resp = client
        .describe_db_clusters()
        .send()
        .await
        .expect("describe_db_clusters should succeed");
    println!("Neptune DB clusters: {:?}", resp.db_clusters());
}
```

## Implemented APIs (64)

- `AddRoleToDBCluster`
- `AddSourceIdentifierToSubscription`
- `AddTagsToResource`
- `CopyDBClusterParameterGroup`
- `CopyDBClusterSnapshot`
- `CopyDBParameterGroup`
- `CreateDBCluster`
- `CreateDBClusterEndpoint`
- `CreateDBClusterParameterGroup`
- `CreateDBClusterSnapshot`
- `CreateDBInstance`
- `CreateDBParameterGroup`
- `CreateDBSubnetGroup`
- `CreateEventSubscription`
- `CreateGlobalCluster`
- `DeleteDBCluster`
- `DeleteDBClusterEndpoint`
- `DeleteDBClusterParameterGroup`
- `DeleteDBClusterSnapshot`
- `DeleteDBInstance`
- `DeleteDBParameterGroup`
- `DeleteDBSubnetGroup`
- `DeleteEventSubscription`
- `DeleteGlobalCluster`
- `DescribeDBClusterEndpoints`
- `DescribeDBClusterParameterGroups`
- `DescribeDBClusterParameters`
- `DescribeDBClusterSnapshotAttributes`
- `DescribeDBClusterSnapshots`
- `DescribeDBClusters`
- `DescribeDBEngineVersions`
- `DescribeDBInstances`
- `DescribeDBParameterGroups`
- `DescribeDBParameters`
- `DescribeDBSubnetGroups`
- `DescribeEventSubscriptions`
- `DescribeGlobalClusters`
- `DescribeOrderableDBInstanceOptions`
- `DescribeValidDBInstanceModifications`
- `FailoverDBCluster`
- `FailoverGlobalCluster`
- `ListTagsForResource`
- `ModifyDBCluster`
- `ModifyDBClusterEndpoint`
- `ModifyDBClusterParameterGroup`
- `ModifyDBClusterSnapshotAttribute`
- `ModifyDBInstance`
- `ModifyDBParameterGroup`
- `ModifyDBSubnetGroup`
- `ModifyEventSubscription`
- `ModifyGlobalCluster`
- `PromoteReadReplicaDBCluster`
- `RebootDBInstance`
- `RemoveFromGlobalCluster`
- `RemoveRoleFromDBCluster`
- `RemoveSourceIdentifierFromSubscription`
- `RemoveTagsFromResource`
- `ResetDBClusterParameterGroup`
- `ResetDBParameterGroup`
- `RestoreDBClusterFromSnapshot`
- `RestoreDBClusterToPointInTime`
- `StartDBCluster`
- `StopDBCluster`
- `SwitchoverGlobalCluster`

<details><summary>Stubbed APIs (6) &mdash; routed but return an empty/default response</summary>

- `ApplyPendingMaintenanceAction`
- `DescribeEngineDefaultClusterParameters`
- `DescribeEngineDefaultParameters`
- `DescribeEventCategories`
- `DescribeEvents`
- `DescribePendingMaintenanceActions`

</details>
