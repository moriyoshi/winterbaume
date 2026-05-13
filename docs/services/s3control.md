# winterbaume-s3control

S3 Control service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | S3 Control |
| AWS model | `s3-control` |
| Protocol | restXml |
| winterbaume coverage | 87/97 operations (89.7%) |
| stubs (routed, returns empty/default) | 10/97 operations (10.3%) |
| moto coverage | 0/97 operations (0.0%) |
| floci coverage | 0/97 operations (0.0%) |
| kumo coverage | 7/97 operations (7.2%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws s3control list-jobs --account-id 123456789012
```

## Current Network Resource Stub Semantics

S3 Control currently stores access point VPC configuration on the access point record.

- `CreateAccessPoint` records `VpcConfiguration.VpcId` when present.
- Access point network origin is inferred from that stored field: VPC when a VPC ID exists, otherwise Internet.
- No EC2 VPC endpoint, subnet, or security group resource is created for the access point.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_s3control::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_s3control::S3ControlService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(S3ControlService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3control::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_s3control::Client::new(&config);

    // Create an access point
    let resp = client
        .create_access_point()
        .account_id("123456789012")
        .name("my-ap")
        .bucket("my-bucket")
        .send()
        .await
        .expect("create_access_point should succeed");

    println!(
        "Created access point ARN: {}",
        resp.access_point_arn().unwrap_or_default()
    );

    // Get the access point
    let get_resp = client
        .get_access_point()
        .account_id("123456789012")
        .name("my-ap")
        .send()
        .await
        .expect("get_access_point should succeed");

    println!("Access point name: {}", get_resp.name().unwrap_or_default());
    println!("Bucket: {}", get_resp.bucket().unwrap_or_default());
    println!(
        "Network origin: {}",
        get_resp
            .network_origin()
            .map(|n| n.as_str())
            .unwrap_or("unknown")
    );
}
```

## Implemented APIs (87)

- `CreateAccessGrant`
- `CreateAccessGrantsInstance`
- `CreateAccessGrantsLocation`
- `CreateAccessPoint`
- `CreateAccessPointForObjectLambda`
- `CreateBucket`
- `CreateJob`
- `CreateMultiRegionAccessPoint`
- `CreateStorageLensGroup`
- `DeleteAccessGrant`
- `DeleteAccessGrantsInstance`
- `DeleteAccessGrantsInstanceResourcePolicy`
- `DeleteAccessGrantsLocation`
- `DeleteAccessPoint`
- `DeleteAccessPointForObjectLambda`
- `DeleteAccessPointPolicy`
- `DeleteAccessPointPolicyForObjectLambda`
- `DeleteAccessPointScope`
- `DeleteBucket`
- `DeleteBucketLifecycleConfiguration`
- `DeleteBucketPolicy`
- `DeleteBucketReplication`
- `DeleteBucketTagging`
- `DeleteJobTagging`
- `DeleteMultiRegionAccessPoint`
- `DeletePublicAccessBlock`
- `DeleteStorageLensConfiguration`
- `DeleteStorageLensConfigurationTagging`
- `DeleteStorageLensGroup`
- `DescribeJob`
- `GetAccessGrant`
- `GetAccessGrantsInstance`
- `GetAccessGrantsInstanceResourcePolicy`
- `GetAccessGrantsLocation`
- `GetAccessPoint`
- `GetAccessPointForObjectLambda`
- `GetAccessPointPolicy`
- `GetAccessPointPolicyForObjectLambda`
- `GetAccessPointPolicyStatus`
- `GetAccessPointPolicyStatusForObjectLambda`
- `GetBucket`
- `GetBucketPolicy`
- `GetBucketTagging`
- `GetBucketVersioning`
- `GetJobTagging`
- `GetMultiRegionAccessPoint`
- `GetMultiRegionAccessPointPolicy`
- `GetMultiRegionAccessPointPolicyStatus`
- `GetMultiRegionAccessPointRoutes`
- `GetPublicAccessBlock`
- `GetStorageLensConfiguration`
- `GetStorageLensConfigurationTagging`
- `GetStorageLensGroup`
- `ListAccessGrants`
- `ListAccessGrantsInstances`
- `ListAccessGrantsLocations`
- `ListAccessPoints`
- `ListAccessPointsForDirectoryBuckets`
- `ListAccessPointsForObjectLambda`
- `ListJobs`
- `ListMultiRegionAccessPoints`
- `ListRegionalBuckets`
- `ListStorageLensConfigurations`
- `ListStorageLensGroups`
- `ListTagsForResource`
- `PutAccessGrantsInstanceResourcePolicy`
- `PutAccessPointConfigurationForObjectLambda`
- `PutAccessPointPolicy`
- `PutAccessPointPolicyForObjectLambda`
- `PutAccessPointScope`
- `PutBucketLifecycleConfiguration`
- `PutBucketPolicy`
- `PutBucketReplication`
- `PutBucketTagging`
- `PutBucketVersioning`
- `PutJobTagging`
- `PutMultiRegionAccessPointPolicy`
- `PutPublicAccessBlock`
- `PutStorageLensConfiguration`
- `PutStorageLensConfigurationTagging`
- `SubmitMultiRegionAccessPointRoutes`
- `TagResource`
- `UntagResource`
- `UpdateAccessGrantsLocation`
- `UpdateJobPriority`
- `UpdateJobStatus`
- `UpdateStorageLensGroup`

<details><summary>Stubbed APIs (10) &mdash; routed but return an empty/default response</summary>

- `AssociateAccessGrantsIdentityCenter`
- `DescribeMultiRegionAccessPointOperation`
- `DissociateAccessGrantsIdentityCenter`
- `GetAccessGrantsInstanceForPrefix`
- `GetAccessPointConfigurationForObjectLambda`
- `GetAccessPointScope`
- `GetBucketLifecycleConfiguration`
- `GetBucketReplication`
- `GetDataAccess`
- `ListCallerAccessGrants`

</details>
