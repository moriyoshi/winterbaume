# winterbaume-cloudfront

CloudFront service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | CloudFront |
| AWS model | `cloudfront` |
| Protocol | restXml |
| winterbaume coverage | 156/167 operations (93.4%) |
| stubs (routed, returns empty/default) | 11/167 operations (6.6%) |
| moto coverage | 25/167 operations (15.0%) |
| floci coverage | 0/167 operations (0.0%) |
| kumo coverage | 8/167 operations (4.8%) |
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
aws cloudfront list-distributions
```

## Current Network Resource Stub Semantics

CloudFront currently models VPC origins as CloudFront-owned records.

- `CreateVpcOrigin` stores the supplied `VpcOriginEndpointConfig` inside `CloudFrontState.vpc_origins` with generated CloudFront VPC origin ID, ARN, timestamps, status, and ETag.
- `UpdateVpcOrigin` replaces the stored endpoint config after optional ETag matching, and `DeleteVpcOrigin` removes the local record.
- The endpoint config is not resolved against EC2, ELBv2, or security group state; the status is CloudFront-local metadata rather than the result of endpoint provisioning.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_cloudfront::config::BehaviorVersion;
use winterbaume_cloudfront::CloudFrontService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CloudFrontService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudfront::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cloudfront::Client::new(&config);

    let resp = client
        .list_distributions()
        .send()
        .await
        .expect("list_distributions should succeed");
    let count = resp
        .distribution_list()
        .map(|l| l.items().len())
        .unwrap_or(0);
    println!("Distributions: {}", count);
}
```

## Implemented APIs (156)

- `AssociateAlias`
- `AssociateDistributionTenantWebACL`
- `AssociateDistributionWebACL`
- `CopyDistribution`
- `CreateAnycastIpList`
- `CreateCachePolicy`
- `CreateCloudFrontOriginAccessIdentity`
- `CreateConnectionFunction`
- `CreateConnectionGroup`
- `CreateContinuousDeploymentPolicy`
- `CreateDistribution`
- `CreateDistributionTenant`
- `CreateDistributionWithTags`
- `CreateFieldLevelEncryptionConfig`
- `CreateFieldLevelEncryptionProfile`
- `CreateFunction`
- `CreateInvalidation`
- `CreateInvalidationForDistributionTenant`
- `CreateKeyGroup`
- `CreateKeyValueStore`
- `CreateMonitoringSubscription`
- `CreateOriginAccessControl`
- `CreateOriginRequestPolicy`
- `CreatePublicKey`
- `CreateRealtimeLogConfig`
- `CreateResponseHeadersPolicy`
- `CreateStreamingDistribution`
- `CreateStreamingDistributionWithTags`
- `CreateTrustStore`
- `CreateVpcOrigin`
- `DeleteAnycastIpList`
- `DeleteCachePolicy`
- `DeleteCloudFrontOriginAccessIdentity`
- `DeleteConnectionFunction`
- `DeleteConnectionGroup`
- `DeleteContinuousDeploymentPolicy`
- `DeleteDistribution`
- `DeleteDistributionTenant`
- `DeleteFieldLevelEncryptionConfig`
- `DeleteFieldLevelEncryptionProfile`
- `DeleteFunction`
- `DeleteKeyGroup`
- `DeleteKeyValueStore`
- `DeleteMonitoringSubscription`
- `DeleteOriginAccessControl`
- `DeleteOriginRequestPolicy`
- `DeletePublicKey`
- `DeleteRealtimeLogConfig`
- `DeleteResourcePolicy`
- `DeleteResponseHeadersPolicy`
- `DeleteStreamingDistribution`
- `DeleteTrustStore`
- `DeleteVpcOrigin`
- `DescribeConnectionFunction`
- `DescribeFunction`
- `DescribeKeyValueStore`
- `DisassociateDistributionTenantWebACL`
- `DisassociateDistributionWebACL`
- `GetAnycastIpList`
- `GetCachePolicy`
- `GetCachePolicyConfig`
- `GetCloudFrontOriginAccessIdentity`
- `GetCloudFrontOriginAccessIdentityConfig`
- `GetConnectionFunction`
- `GetConnectionGroup`
- `GetConnectionGroupByRoutingEndpoint`
- `GetContinuousDeploymentPolicy`
- `GetContinuousDeploymentPolicyConfig`
- `GetDistribution`
- `GetDistributionConfig`
- `GetDistributionTenant`
- `GetDistributionTenantByDomain`
- `GetFieldLevelEncryption`
- `GetFieldLevelEncryptionConfig`
- `GetFieldLevelEncryptionProfile`
- `GetFieldLevelEncryptionProfileConfig`
- `GetFunction`
- `GetInvalidation`
- `GetInvalidationForDistributionTenant`
- `GetKeyGroup`
- `GetKeyGroupConfig`
- `GetMonitoringSubscription`
- `GetOriginAccessControl`
- `GetOriginAccessControlConfig`
- `GetOriginRequestPolicy`
- `GetOriginRequestPolicyConfig`
- `GetPublicKey`
- `GetPublicKeyConfig`
- `GetRealtimeLogConfig`
- `GetResourcePolicy`
- `GetResponseHeadersPolicy`
- `GetResponseHeadersPolicyConfig`
- `GetStreamingDistribution`
- `GetStreamingDistributionConfig`
- `GetTrustStore`
- `GetVpcOrigin`
- `ListAnycastIpLists`
- `ListCachePolicies`
- `ListCloudFrontOriginAccessIdentities`
- `ListConnectionFunctions`
- `ListConnectionGroups`
- `ListContinuousDeploymentPolicies`
- `ListDistributionTenants`
- `ListDistributionTenantsByCustomization`
- `ListDistributions`
- `ListDistributionsByAnycastIpListId`
- `ListDistributionsByCachePolicyId`
- `ListDistributionsByKeyGroup`
- `ListDistributionsByOriginRequestPolicyId`
- `ListDistributionsByResponseHeadersPolicyId`
- `ListDistributionsByVpcOriginId`
- `ListDistributionsByWebACLId`
- `ListFieldLevelEncryptionConfigs`
- `ListFieldLevelEncryptionProfiles`
- `ListFunctions`
- `ListInvalidations`
- `ListInvalidationsForDistributionTenant`
- `ListKeyGroups`
- `ListKeyValueStores`
- `ListOriginAccessControls`
- `ListOriginRequestPolicies`
- `ListPublicKeys`
- `ListRealtimeLogConfigs`
- `ListResponseHeadersPolicies`
- `ListStreamingDistributions`
- `ListTagsForResource`
- `ListTrustStores`
- `ListVpcOrigins`
- `PublishConnectionFunction`
- `PublishFunction`
- `PutResourcePolicy`
- `TagResource`
- `UntagResource`
- `UpdateAnycastIpList`
- `UpdateCachePolicy`
- `UpdateCloudFrontOriginAccessIdentity`
- `UpdateConnectionFunction`
- `UpdateConnectionGroup`
- `UpdateContinuousDeploymentPolicy`
- `UpdateDistribution`
- `UpdateDistributionTenant`
- `UpdateDistributionWithStagingConfig`
- `UpdateDomainAssociation`
- `UpdateFieldLevelEncryptionConfig`
- `UpdateFieldLevelEncryptionProfile`
- `UpdateFunction`
- `UpdateKeyGroup`
- `UpdateKeyValueStore`
- `UpdateOriginAccessControl`
- `UpdateOriginRequestPolicy`
- `UpdatePublicKey`
- `UpdateRealtimeLogConfig`
- `UpdateResponseHeadersPolicy`
- `UpdateStreamingDistribution`
- `UpdateTrustStore`
- `UpdateVpcOrigin`

<details><summary>Stubbed APIs (11) &mdash; routed but return an empty/default response</summary>

- `GetManagedCertificateDetails`
- `ListConflictingAliases`
- `ListDistributionsByConnectionFunction`
- `ListDistributionsByConnectionMode`
- `ListDistributionsByOwnedResource`
- `ListDistributionsByRealtimeLogConfig`
- `ListDistributionsByTrustStore`
- `ListDomainConflicts`
- `TestConnectionFunction`
- `TestFunction`
- `VerifyDnsConfiguration`

</details>
