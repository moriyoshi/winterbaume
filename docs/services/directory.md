# winterbaume-directory

AWS Directory Service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Directory Service |
| AWS model | `directory-service` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 4/80 operations (5.0%) |
| stubs (routed, returns empty/default) | 0/80 operations (0.0%) |
| moto coverage | 0/80 operations (0.0%) |
| floci coverage | 0/80 operations (0.0%) |
| kumo coverage | 6/80 operations (7.5%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws ds describe-directories
```

## Current Network Resource Stub Semantics

Directory Service stores directory networking settings as directory-local metadata.

- Microsoft AD creation records the supplied `VpcSettings.VpcId` and `SubnetIds`, then returns a `DirectoryVpcSettings` record with those raw IDs.
- AD Connector creation records the supplied connect settings VPC ID and subnet IDs in the same local directory state.
- The implementation synthesises directory security group IDs from the directory ID and derives availability-zone-looking values from subnet positions rather than querying EC2.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_directory::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_directory::DirectoryService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(DirectoryService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_directory::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_directory::Client::new(&config);

    let resp = client
        .describe_directories()
        .send()
        .await
        .expect("describe_directories should succeed");
    println!("Directories: {}", resp.directory_descriptions().len());
}
```

## Implemented APIs (4)

- `ConnectDirectory`
- `CreateDirectory`
- `DeleteDirectory`
- `DescribeDirectories`

<details><summary>Not yet implemented APIs (76)</summary>

- `AcceptSharedDirectory`
- `AddIpRoutes`
- `AddRegion`
- `AddTagsToResource`
- `CancelSchemaExtension`
- `CreateAlias`
- `CreateComputer`
- `CreateConditionalForwarder`
- `CreateHybridAD`
- `CreateLogSubscription`
- `CreateMicrosoftAD`
- `CreateSnapshot` (implemented by kumo)
- `CreateTrust`
- `DeleteADAssessment`
- `DeleteConditionalForwarder`
- `DeleteLogSubscription`
- `DeleteSnapshot` (implemented by kumo)
- `DeleteTrust`
- `DeregisterCertificate`
- `DeregisterEventTopic`
- `DescribeADAssessment`
- `DescribeCAEnrollmentPolicy`
- `DescribeCertificate`
- `DescribeClientAuthenticationSettings`
- `DescribeConditionalForwarders`
- `DescribeDirectoryDataAccess`
- `DescribeDomainControllers`
- `DescribeEventTopics`
- `DescribeHybridADUpdate`
- `DescribeLDAPSSettings`
- `DescribeRegions`
- `DescribeSettings`
- `DescribeSharedDirectories`
- `DescribeSnapshots` (implemented by kumo)
- `DescribeTrusts`
- `DescribeUpdateDirectory`
- `DisableCAEnrollmentPolicy`
- `DisableClientAuthentication`
- `DisableDirectoryDataAccess`
- `DisableLDAPS`
- `DisableRadius`
- `DisableSso`
- `EnableCAEnrollmentPolicy`
- `EnableClientAuthentication`
- `EnableDirectoryDataAccess`
- `EnableLDAPS`
- `EnableRadius`
- `EnableSso`
- `GetDirectoryLimits`
- `GetSnapshotLimits`
- `ListADAssessments`
- `ListCertificates`
- `ListIpRoutes`
- `ListLogSubscriptions`
- `ListSchemaExtensions`
- `ListTagsForResource`
- `RegisterCertificate`
- `RegisterEventTopic`
- `RejectSharedDirectory`
- `RemoveIpRoutes`
- `RemoveRegion`
- `RemoveTagsFromResource`
- `ResetUserPassword`
- `RestoreFromSnapshot`
- `ShareDirectory`
- `StartADAssessment`
- `StartSchemaExtension`
- `UnshareDirectory`
- `UpdateConditionalForwarder`
- `UpdateDirectorySetup`
- `UpdateHybridAD`
- `UpdateNumberOfDomainControllers`
- `UpdateRadius`
- `UpdateSettings`
- `UpdateTrust`
- `VerifyTrust`

</details>
