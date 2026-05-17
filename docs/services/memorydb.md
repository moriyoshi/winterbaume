# winterbaume-memorydb

Amazon MemoryDB service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | MemoryDB |
| AWS model | `memorydb` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 13/45 operations (28.9%) |
| stubs (routed, returns empty/default) | 0/45 operations (0.0%) |
| moto coverage | 13/45 operations (28.9%) |
| floci coverage | 0/45 operations (0.0%) |
| kumo coverage | 10/45 operations (22.2%) |
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
aws memorydb describe-clusters
```

## Current Network Resource Stub Semantics

MemoryDB currently models subnet groups and security groups inside MemoryDB state.

- `CreateSubnetGroup` stores supplied subnet IDs and mints a synthetic VPC ID from the generated resource identifier rather than deriving it from EC2.
- Clusters store the selected subnet group name and supplied security group IDs as raw strings.
- Subnet group deletion checks local MemoryDB cluster references, but not EC2 subnet or security group state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_memorydb::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_memorydb::MemoryDbService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(MemoryDbService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_memorydb::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_memorydb::Client::new(&config);

    let resp = client
        .describe_clusters()
        .send()
        .await
        .expect("describe_clusters should succeed");
    println!("MemoryDB clusters: {}", resp.clusters().len());
}
```

## Implemented APIs (13)

- `CreateCluster`
- `CreateSnapshot`
- `CreateSubnetGroup`
- `DeleteCluster`
- `DeleteSnapshot`
- `DeleteSubnetGroup`
- `DescribeClusters`
- `DescribeSnapshots`
- `DescribeSubnetGroups`
- `ListTags`
- `TagResource`
- `UntagResource`
- `UpdateCluster`

<details><summary>Not yet implemented APIs (32)</summary>

- `BatchUpdateCluster`
- `CopySnapshot`
- `CreateACL` (implemented by kumo)
- `CreateMultiRegionCluster`
- `CreateParameterGroup`
- `CreateUser` (implemented by kumo)
- `DeleteACL` (implemented by kumo)
- `DeleteMultiRegionCluster`
- `DeleteParameterGroup`
- `DeleteUser` (implemented by kumo)
- `DescribeACLs` (implemented by kumo)
- `DescribeEngineVersions`
- `DescribeEvents`
- `DescribeMultiRegionClusters`
- `DescribeMultiRegionParameterGroups`
- `DescribeMultiRegionParameters`
- `DescribeParameterGroups`
- `DescribeParameters`
- `DescribeReservedNodes`
- `DescribeReservedNodesOfferings`
- `DescribeServiceUpdates`
- `DescribeUsers` (implemented by kumo)
- `FailoverShard`
- `ListAllowedMultiRegionClusterUpdates`
- `ListAllowedNodeTypeUpdates`
- `PurchaseReservedNodesOffering`
- `ResetParameterGroup`
- `UpdateACL`
- `UpdateMultiRegionCluster`
- `UpdateParameterGroup`
- `UpdateSubnetGroup`
- `UpdateUser`

</details>
