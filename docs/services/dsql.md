# winterbaume-dsql

Amazon Aurora DSQL service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Aurora DSQL |
| AWS model | `dsql` |
| Protocol | restJson1 |
| winterbaume coverage | 12/12 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/12 operations (0.0%) |
| moto coverage | 5/12 operations (41.7%) |
| floci coverage | 0/12 operations (0.0%) |
| kumo coverage | 0/12 operations (0.0%) |
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
aws dsql list-clusters
```

## Current Network Resource Stub Semantics

DSQL currently exposes a VPC endpoint service-name API shape but does not implement the network path.

- `GetVpcEndpointServiceName` is routed as not implemented and does not return a service name derived from cluster state.
- Cluster records do not allocate or track VPC endpoints, subnets, security groups, or PrivateLink attachments.
- The generated model contains `clusterVpcEndpoint`, but current handlers do not populate it from EC2 or DSQL state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_dsql::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_dsql::DsqlService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(DsqlService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dsql::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_dsql::Client::new(&config);

    let resp = client
        .list_clusters()
        .send()
        .await
        .expect("list_clusters should succeed");
    println!("DSQL clusters: {}", resp.clusters().len());
}
```

## Implemented APIs (12)

- `CreateCluster`
- `DeleteCluster`
- `DeleteClusterPolicy`
- `GetCluster`
- `GetClusterPolicy`
- `GetVpcEndpointServiceName`
- `ListClusters`
- `ListTagsForResource`
- `PutClusterPolicy`
- `TagResource`
- `UntagResource`
- `UpdateCluster`
