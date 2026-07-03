# winterbaume-fsx

FSx service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | FSx |
| AWS model | `fsx` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 9/48 operations (18.8%) |
| stubs (routed, returns empty/default) | 0/48 operations (0.0%) |
| moto coverage | 9/48 operations (18.8%) |
| floci coverage | 0/48 operations (0.0%) |
| kumo coverage | 0/48 operations (0.0%) |
| fakecloud coverage | 0/48 operations (0.0%) |
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
aws fsx describe-file-systems
```

## Current Network Resource Stub Semantics

FSx currently treats file-system networking fields as local file-system metadata.

- Create paths store subnet IDs, security group IDs, preferred subnet IDs, endpoint IP address ranges, and route-table IDs where the implemented file-system type supports them.
- The service mints FSx lifecycle and endpoint-style data internally and returns the stored network fields through describe calls.
- There is no EC2 subnet, security group, route table, or ENI reconciliation.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_fsx::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_fsx::FsxService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(FsxService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_fsx::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_fsx::Client::new(&config);

    let resp = client
        .describe_file_systems()
        .send()
        .await
        .expect("describe_file_systems should succeed");
    println!("FSx file systems: {}", resp.file_systems().len());
}
```

## Implemented APIs (9)

- `CreateBackup`
- `CreateFileSystem`
- `DeleteBackup`
- `DeleteFileSystem`
- `DescribeBackups`
- `DescribeFileSystems`
- `ListTagsForResource`
- `TagResource`
- `UntagResource`

<details><summary>Not yet implemented APIs (39)</summary>

- `AssociateFileSystemAliases`
- `CancelDataRepositoryTask`
- `CopyBackup`
- `CopySnapshotAndUpdateVolume`
- `CreateAndAttachS3AccessPoint`
- `CreateDataRepositoryAssociation`
- `CreateDataRepositoryTask`
- `CreateFileCache`
- `CreateFileSystemFromBackup`
- `CreateSnapshot`
- `CreateStorageVirtualMachine`
- `CreateVolume`
- `CreateVolumeFromBackup`
- `DeleteDataRepositoryAssociation`
- `DeleteFileCache`
- `DeleteSnapshot`
- `DeleteStorageVirtualMachine`
- `DeleteVolume`
- `DescribeDataRepositoryAssociations`
- `DescribeDataRepositoryTasks`
- `DescribeFileCaches`
- `DescribeFileSystemAliases`
- `DescribeS3AccessPointAttachments`
- `DescribeSharedVpcConfiguration`
- `DescribeSnapshots`
- `DescribeStorageVirtualMachines`
- `DescribeVolumes`
- `DetachAndDeleteS3AccessPoint`
- `DisassociateFileSystemAliases`
- `ReleaseFileSystemNfsV3Locks`
- `RestoreVolumeFromSnapshot`
- `StartMisconfiguredStateRecovery`
- `UpdateDataRepositoryAssociation`
- `UpdateFileCache`
- `UpdateFileSystem`
- `UpdateSharedVpcConfiguration`
- `UpdateSnapshot`
- `UpdateStorageVirtualMachine`
- `UpdateVolume`

</details>
