# winterbaume-s3files

Amazon S3 Files service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | S3 Files |
| AWS model | `s3files` |
| Protocol | restJson1 |
| winterbaume coverage | 21/21 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/21 operations (0.0%) |
| moto coverage | 0/21 operations (0.0%) |
| floci coverage | 0/21 operations (0.0%) |
| kumo coverage | 0/21 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws s3files help
```

## Current Network Resource Stub Semantics

S3Files currently has one of the more detailed local network stubs for mount targets.

- `CreateMountTarget` requires a subnet ID, stores security groups, derives a VPC ID deterministically from the subnet ID, derives an availability-zone ID from the subnet ID, and mints an `eni-` network interface ID.
- File-system mount targets are constrained inside S3Files state to one derived VPC per file system and one mount target per derived availability zone.
- `ListMountTargets` can filter by stored VPC ID, and `UpdateMountTarget` replaces the stored security group list.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_s3files::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_s3files::S3FilesService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(S3FilesService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3files::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_s3files::Client::new(&config);

    let resp = client
        .list_file_systems()
        .send()
        .await
        .expect("list_file_systems should succeed");
    println!("S3 Files: {:?}", resp.file_systems());
}
```

## Implemented APIs (21)

- `CreateAccessPoint`
- `CreateFileSystem`
- `CreateMountTarget`
- `DeleteAccessPoint`
- `DeleteFileSystem`
- `DeleteFileSystemPolicy`
- `DeleteMountTarget`
- `GetAccessPoint`
- `GetFileSystem`
- `GetFileSystemPolicy`
- `GetMountTarget`
- `GetSynchronizationConfiguration`
- `ListAccessPoints`
- `ListFileSystems`
- `ListMountTargets`
- `ListTagsForResource`
- `PutFileSystemPolicy`
- `PutSynchronizationConfiguration`
- `TagResource`
- `UntagResource`
- `UpdateMountTarget`
