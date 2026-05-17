# winterbaume-efs

EFS service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | EFS |
| AWS model | `efs` |
| Protocol | restJson1 |
| winterbaume coverage | 31/31 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/31 operations (0.0%) |
| moto coverage | 19/31 operations (61.3%) |
| floci coverage | 0/31 operations (0.0%) |
| kumo coverage | 0/31 operations (0.0%) |
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
aws efs describe-file-systems
```

## Current Network Resource Stub Semantics

EFS currently models mount targets with service-local network metadata.

- `CreateMountTarget` stores the supplied subnet ID and security groups, mints a synthetic mount target ID and network interface ID, and returns a VPC ID from local EFS logic rather than EC2 subnet state.
- Mount target lifecycle and file-system mount target counts are maintained inside EFS state.
- `DescribeMountTargetSecurityGroups` and `ModifyMountTargetSecurityGroups` read and replace the stored security group list only.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_efs::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_efs::EfsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(EfsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_efs::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_efs::Client::new(&config);

    let resp = client
        .describe_file_systems()
        .send()
        .await
        .expect("describe_file_systems should succeed");
    println!("EFS file systems: {}", resp.file_systems().len());
}
```

## Implemented APIs (31)

- `CreateAccessPoint`
- `CreateFileSystem`
- `CreateMountTarget`
- `CreateReplicationConfiguration`
- `CreateTags`
- `DeleteAccessPoint`
- `DeleteFileSystem`
- `DeleteFileSystemPolicy`
- `DeleteMountTarget`
- `DeleteReplicationConfiguration`
- `DeleteTags`
- `DescribeAccessPoints`
- `DescribeAccountPreferences`
- `DescribeBackupPolicy`
- `DescribeFileSystemPolicy`
- `DescribeFileSystems`
- `DescribeLifecycleConfiguration`
- `DescribeMountTargetSecurityGroups`
- `DescribeMountTargets`
- `DescribeReplicationConfigurations`
- `DescribeTags`
- `ListTagsForResource`
- `ModifyMountTargetSecurityGroups`
- `PutAccountPreferences`
- `PutBackupPolicy`
- `PutFileSystemPolicy`
- `PutLifecycleConfiguration`
- `TagResource`
- `UntagResource`
- `UpdateFileSystem`
- `UpdateFileSystemProtection`
