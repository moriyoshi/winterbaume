# winterbaume-cloudhsmv2

AWS CloudHSM V2 service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | CloudHSM v2 |
| AWS model | `cloudhsm-v2` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 18/18 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/18 operations (0.0%) |
| moto coverage | 0/18 operations (0.0%) |
| floci coverage | 0/18 operations (0.0%) |
| kumo coverage | 0/18 operations (0.0%) |
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
aws cloudhsmv2 describe-clusters
```

## Current Network Resource Stub Semantics

CloudHSM V2 currently synthesises cluster network state from request metadata.

- `CreateCluster` records the supplied subnet IDs but mints a synthetic VPC ID of the form `vpc-<uuid-prefix>` instead of deriving it from EC2 subnet state.
- Cluster filters can match the stored synthetic VPC ID, and later describe calls return that local value.
- HSM and cluster lifecycle is not tied to subnet reachability, security groups, route tables, or ENIs.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_cloudhsmv2::config::BehaviorVersion;
use winterbaume_cloudhsmv2::CloudHsmV2Service;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CloudHsmV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudhsmv2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cloudhsmv2::Client::new(&config);

    let resp = client
        .describe_clusters()
        .send()
        .await
        .expect("describe_clusters should succeed");
    println!("Clusters: {}", resp.clusters().len());
}
```

## Implemented APIs (18)

- `CopyBackupToRegion`
- `CreateCluster`
- `CreateHsm`
- `DeleteBackup`
- `DeleteCluster`
- `DeleteHsm`
- `DeleteResourcePolicy`
- `DescribeBackups`
- `DescribeClusters`
- `GetResourcePolicy`
- `InitializeCluster`
- `ListTags`
- `ModifyBackupAttributes`
- `ModifyCluster`
- `PutResourcePolicy`
- `RestoreBackup`
- `TagResource`
- `UntagResource`
