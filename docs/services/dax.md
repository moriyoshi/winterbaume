# winterbaume-dax

DAX service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | DAX |
| AWS model | `dax` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 6/21 operations (28.6%) |
| stubs (routed, returns empty/default) | 0/21 operations (0.0%) |
| moto coverage | 8/21 operations (38.1%) |
| floci coverage | 0/21 operations (0.0%) |
| kumo coverage | 0/21 operations (0.0%) |
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
aws dax describe-clusters
```

## Current Network Resource Stub Semantics

DAX currently has partial placeholders for subnet groups and security groups.

- Subnet group shapes and views include subnet IDs and an optional VPC ID, but `CreateSubnetGroup`, `DescribeSubnetGroups`, `UpdateSubnetGroup`, and `DeleteSubnetGroup` return `501 NotImplemented`.
- `CreateCluster` returns local cluster data using the requested subnet group name when present, but the mocked response falls back to default networking such as `default` subnet group and a fixed `sg-00000001` style security group.
- Cluster state is not tied to EC2 subnet membership or security group existence.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_dax::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_dax::DaxService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(DaxService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dax::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_dax::Client::new(&config);

    let resp = client
        .describe_clusters()
        .send()
        .await
        .expect("describe_clusters should succeed");
    println!("DAX clusters: {}", resp.clusters().len());
}
```

## Implemented APIs (6)

- `CreateCluster`
- `DecreaseReplicationFactor`
- `DeleteCluster`
- `DescribeClusters`
- `IncreaseReplicationFactor`
- `ListTags`

<details><summary>Not yet implemented APIs (15)</summary>

- `CreateParameterGroup`
- `CreateSubnetGroup`
- `DeleteParameterGroup`
- `DeleteSubnetGroup`
- `DescribeDefaultParameters`
- `DescribeEvents`
- `DescribeParameterGroups`
- `DescribeParameters`
- `DescribeSubnetGroups`
- `RebootNode`
- `TagResource` (implemented by moto)
- `UntagResource` (implemented by moto)
- `UpdateCluster`
- `UpdateParameterGroup`
- `UpdateSubnetGroup`

</details>
