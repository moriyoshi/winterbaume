# winterbaume-opensearch

Amazon OpenSearch service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | OpenSearch |
| AWS model | `opensearch` |
| Protocol | restJson1 |
| winterbaume coverage | 44/88 operations (50.0%) |
| stubs (routed, returns empty/default) | 0/88 operations (0.0%) |
| moto coverage | 11/88 operations (12.5%) |
| floci coverage | 0/88 operations (0.0%) |
| kumo coverage | 0/88 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws opensearch list-domain-names
```

## Current Network Resource Stub Semantics

OpenSearch currently stores VPC endpoints and access relationships inside OpenSearch state.

- VPC endpoint records keep the domain name, owner, VPC ID, subnet IDs, and security group IDs supplied by the request.
- Domain endpoint access principals are stored as domain-local lists.
- The service does not provision EC2 VPC endpoints or check subnet and security group membership.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_opensearch::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_opensearch::OpenSearchService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(OpenSearchService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_opensearch::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_opensearch::Client::new(&config);

    let resp = client
        .list_domain_names()
        .send()
        .await
        .expect("list_domain_names should succeed");
    println!("OpenSearch domains: {}", resp.domain_names().len());
}
```

## Implemented APIs (44)

- `AcceptInboundConnection`
- `AddDataSource`
- `AddDirectQueryDataSource`
- `AddTags`
- `AssociatePackage`
- `CancelDomainConfigChange`
- `CreateDomain`
- `CreateOutboundConnection`
- `CreatePackage`
- `CreateVpcEndpoint`
- `DeleteDataSource`
- `DeleteDirectQueryDataSource`
- `DeleteDomain`
- `DeleteInboundConnection`
- `DeleteOutboundConnection`
- `DeletePackage`
- `DeleteVpcEndpoint`
- `DescribeDomain`
- `DescribeDomainConfig`
- `DescribeDomains`
- `DescribeInboundConnections`
- `DescribeOutboundConnections`
- `DescribePackages`
- `DescribeReservedInstances`
- `DescribeVpcEndpoints`
- `DissociatePackage`
- `GetCompatibleVersions`
- `GetDataSource`
- `GetDirectQueryDataSource`
- `ListDataSources`
- `ListDirectQueryDataSources`
- `ListDomainNames`
- `ListDomainsForPackage`
- `ListPackagesForDomain`
- `ListTags`
- `ListVpcEndpoints`
- `ListVpcEndpointsForDomain`
- `PurchaseReservedInstanceOffering`
- `RejectInboundConnection`
- `RemoveTags`
- `UpdateDataSource`
- `UpdateDirectQueryDataSource`
- `UpdateDomainConfig`
- `UpdateVpcEndpoint`

<details><summary>Not yet implemented APIs (44)</summary>

- `AssociatePackages`
- `AuthorizeVpcEndpointAccess`
- `CancelServiceSoftwareUpdate`
- `CreateApplication`
- `CreateIndex`
- `DeleteApplication`
- `DeleteIndex`
- `DeregisterCapability`
- `DescribeDomainAutoTunes`
- `DescribeDomainChangeProgress`
- `DescribeDomainHealth`
- `DescribeDomainNodes`
- `DescribeDryRunProgress`
- `DescribeInsightDetails`
- `DescribeInstanceTypeLimits`
- `DescribeReservedInstanceOfferings`
- `DissociatePackages`
- `GetApplication`
- `GetCapability`
- `GetDefaultApplicationSetting`
- `GetDomainMaintenanceStatus`
- `GetIndex`
- `GetPackageVersionHistory`
- `GetUpgradeHistory`
- `GetUpgradeStatus`
- `ListApplications`
- `ListDomainMaintenances`
- `ListInsights`
- `ListInstanceTypeDetails`
- `ListScheduledActions`
- `ListVersions`
- `ListVpcEndpointAccess`
- `PutDefaultApplicationSetting`
- `RegisterCapability`
- `RevokeVpcEndpointAccess`
- `RollbackServiceSoftwareUpdate`
- `StartDomainMaintenance`
- `StartServiceSoftwareUpdate`
- `UpdateApplication`
- `UpdateIndex`
- `UpdatePackage`
- `UpdatePackageScope`
- `UpdateScheduledAction`
- `UpgradeDomain`

</details>
