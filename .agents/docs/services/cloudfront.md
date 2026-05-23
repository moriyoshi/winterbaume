# Amazon CloudFront

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon CloudFront This is the Amazon CloudFront API Reference . This guide is for developers who need detailed information about CloudFront API actions, data types, and errors. For detailed information about CloudFront features, see the Amazon CloudFront Developer Guide.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon CloudFront where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon CloudFront by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon CloudFront by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon CloudFront workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Update` operation families, including `GetAnycastIpList`, `GetCachePolicy`, `GetCachePolicyConfig`, `GetCloudFrontOriginAccessIdentity`, `ListAnycastIpLists`, `ListCachePolicies`.

## Service Identity and Protocol

- AWS model slug: `cloudfront`
- AWS SDK for Rust slug: `cloudfront`
- Model version: `2020-05-31`
- Model file: `vendor/api-models-aws/models/cloudfront/service/2020-05-31/cloudfront-2020-05-31.json`
- SDK ID: `CloudFront`
- Endpoint prefix: `cloudfront`
- ARN namespace: `cloudfront`
- CloudFormation name: `CloudFront`
- CloudTrail event source: `cloudfront.amazonaws.com`
- Protocols: `restXml`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (39), `List` (39), `Create` (26), `Delete` (23), `Update` (23), `Associate` (3), `Describe` (3), `Disassociate` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateAlias`, `AssociateDistributionTenantWebACL`, `AssociateDistributionWebACL`, `CreateAnycastIpList`, `CreateCachePolicy`, `CreateCloudFrontOriginAccessIdentity`, `CreateConnectionFunction`, `CreateConnectionGroup`, `CreateContinuousDeploymentPolicy`, `CreateDistribution`, `CreateDistributionTenant`, `CreateDistributionWithTags`, `CreateFieldLevelEncryptionConfig`, `CreateFieldLevelEncryptionProfile`, `CreateFunction`, `CreateInvalidation`, `CreateInvalidationForDistributionTenant`, `CreateKeyGroup`, `CreateKeyValueStore`, `CreateMonitoringSubscription`, ... (+60).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeConnectionFunction`, `DescribeFunction`, `DescribeKeyValueStore`, `GetAnycastIpList`, `GetCachePolicy`, `GetCachePolicyConfig`, `GetCloudFrontOriginAccessIdentity`, `GetCloudFrontOriginAccessIdentityConfig`, `GetConnectionFunction`, `GetConnectionGroup`, `GetConnectionGroupByRoutingEndpoint`, `GetContinuousDeploymentPolicy`, `GetContinuousDeploymentPolicyConfig`, `GetDistribution`, `GetDistributionConfig`, `GetDistributionTenant`, `GetDistributionTenantByDomain`, `GetFieldLevelEncryption`, `GetFieldLevelEncryptionConfig`, `GetFieldLevelEncryptionProfile`, ... (+61).
- Pagination is modelled for 17 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 2 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 167 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `Lambda`, `EC2/VPC`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/DownloadDistValuesCacheBehavior.html
- https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/invalidation-by-tags.html
- https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html

Research outcomes:
- Cache behaviours are selected by path pattern, and the first matching behaviour wins. The default cache behaviour path pattern is `*`.
- Path patterns are case-sensitive and can use `*` and `?`. Leading slashes are optional.
- CloudFront normalises URI paths before matching cache behaviours, but sends the raw URI path to the origin.
- Viewer protocol policy controls whether HTTP is allowed, redirected to HTTPS, or rejected in favour of HTTPS only.
- Allowed methods can be GET/HEAD, GET/HEAD/OPTIONS, or all methods. CloudFront caches GET, HEAD, and optionally OPTIONS; other methods are forwarded but not cached.
- MinTTL can cause CloudFront to cache even when origin headers contain no-cache, no-store, or private directives.
- Cookie and query-string forwarding affect the cache key and can reduce cache hit rates.
- Cache tag invalidations use a `#tag` path value in CreateInvalidation, require cache-tag configuration, and can be mixed with normal path invalidations.

Parity implications:
- Distribution request handling needs ordered cache behaviour matching with path normalisation and raw-origin-path preservation.
- Separate viewer protocol policy, allowed methods, cached methods, origin forwarding, cache key, TTL calculation, signed access, and invalidation state.
- Invalidation should be asynchronous and able to target paths, wildcard paths, and cache tags where configured.

## Current Network Resource Stub Semantics

CloudFront currently models VPC origins as CloudFront-owned records.

- `CreateVpcOrigin` stores the supplied `VpcOriginEndpointConfig` inside `CloudFrontState.vpc_origins` with generated CloudFront VPC origin ID, ARN, timestamps, status, and ETag.
- `UpdateVpcOrigin` replaces the stored endpoint config after optional ETag matching, and `DeleteVpcOrigin` removes the local record.
- The endpoint config is not resolved against EC2, ELBv2, or security group state; the status is CloudFront-local metadata rather than the result of endpoint provisioning.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Get

- Operations: `GetAnycastIpList`, `GetCachePolicy`, `GetCachePolicyConfig`, `GetCloudFrontOriginAccessIdentity`, `GetCloudFrontOriginAccessIdentityConfig`, `GetConnectionFunction`, `GetConnectionGroup`, `GetConnectionGroupByRoutingEndpoint`, `GetContinuousDeploymentPolicy`, `GetContinuousDeploymentPolicyConfig`, `GetDistribution`, `GetDistributionConfig`, `GetDistributionTenant`, `GetDistributionTenantByDomain`, `GetFieldLevelEncryption`, `GetFieldLevelEncryptionConfig`, `GetFieldLevelEncryptionProfile`, `GetFieldLevelEncryptionProfileConfig`, `GetFunction`, `GetInvalidation`, `GetInvalidationForDistributionTenant`, `GetKeyGroup`, `GetKeyGroupConfig`, `GetManagedCertificateDetails`, `GetMonitoringSubscription`, `GetOriginAccessControl`, `GetOriginAccessControlConfig`, `GetOriginRequestPolicy`, `GetOriginRequestPolicyConfig`, `GetPublicKey`, `GetPublicKeyConfig`, `GetRealtimeLogConfig`, `GetResourcePolicy`, `GetResponseHeadersPolicy`, `GetResponseHeadersPolicyConfig`, `GetStreamingDistribution`, `GetStreamingDistributionConfig`, `GetTrustStore`, `GetVpcOrigin`
- Common required input members in this group: `Id`, `Identifier`, `DistributionId`

### List

- Operations: `ListAnycastIpLists`, `ListCachePolicies`, `ListCloudFrontOriginAccessIdentities`, `ListConflictingAliases`, `ListConnectionFunctions`, `ListConnectionGroups`, `ListContinuousDeploymentPolicies`, `ListDistributions`, `ListDistributionsByAnycastIpListId`, `ListDistributionsByCachePolicyId`, `ListDistributionsByConnectionFunction`, `ListDistributionsByConnectionMode`, `ListDistributionsByKeyGroup`, `ListDistributionsByOriginRequestPolicyId`, `ListDistributionsByOwnedResource`, `ListDistributionsByRealtimeLogConfig`, `ListDistributionsByResponseHeadersPolicyId`, `ListDistributionsByTrustStore`, `ListDistributionsByVpcOriginId`, `ListDistributionsByWebACLId`, `ListDistributionTenants`, `ListDistributionTenantsByCustomization`, `ListDomainConflicts`, `ListFieldLevelEncryptionConfigs`, `ListFieldLevelEncryptionProfiles`, `ListFunctions`, `ListInvalidations`, `ListInvalidationsForDistributionTenant`, `ListKeyGroups`, `ListKeyValueStores`, `ListOriginAccessControls`, `ListOriginRequestPolicies`, `ListPublicKeys`, `ListRealtimeLogConfigs`, `ListResponseHeadersPolicies`, `ListStreamingDistributions`, `ListTagsForResource`, `ListTrustStores`, `ListVpcOrigins`
- Traits: `paginated` (17)
- Common required input members in this group: `DistributionId`

### Create

- Operations: `CreateAnycastIpList`, `CreateCachePolicy`, `CreateCloudFrontOriginAccessIdentity`, `CreateConnectionFunction`, `CreateConnectionGroup`, `CreateContinuousDeploymentPolicy`, `CreateDistribution`, `CreateDistributionTenant`, `CreateDistributionWithTags`, `CreateFieldLevelEncryptionConfig`, `CreateFieldLevelEncryptionProfile`, `CreateFunction`, `CreateInvalidation`, `CreateInvalidationForDistributionTenant`, `CreateKeyGroup`, `CreateKeyValueStore`, `CreateMonitoringSubscription`, `CreateOriginAccessControl`, `CreateOriginRequestPolicy`, `CreatePublicKey`, `CreateRealtimeLogConfig`, `CreateResponseHeadersPolicy`, `CreateStreamingDistribution`, `CreateStreamingDistributionWithTags`, `CreateTrustStore`, `CreateVpcOrigin`
- Common required input members in this group: `Name`, `DistributionId`, `InvalidationBatch`

### Delete

- Operations: `DeleteAnycastIpList`, `DeleteCachePolicy`, `DeleteCloudFrontOriginAccessIdentity`, `DeleteConnectionFunction`, `DeleteConnectionGroup`, `DeleteContinuousDeploymentPolicy`, `DeleteDistribution`, `DeleteDistributionTenant`, `DeleteFieldLevelEncryptionConfig`, `DeleteFieldLevelEncryptionProfile`, `DeleteFunction`, `DeleteKeyGroup`, `DeleteKeyValueStore`, `DeleteMonitoringSubscription`, `DeleteOriginAccessControl`, `DeleteOriginRequestPolicy`, `DeletePublicKey`, `DeleteRealtimeLogConfig`, `DeleteResourcePolicy`, `DeleteResponseHeadersPolicy`, `DeleteStreamingDistribution`, `DeleteTrustStore`, `DeleteVpcOrigin`
- Traits: `idempotent` (1)
- Common required input members in this group: `Id`, `IfMatch`, `Name`

### Update

- Operations: `UpdateAnycastIpList`, `UpdateCachePolicy`, `UpdateCloudFrontOriginAccessIdentity`, `UpdateConnectionFunction`, `UpdateConnectionGroup`, `UpdateContinuousDeploymentPolicy`, `UpdateDistribution`, `UpdateDistributionTenant`, `UpdateDistributionWithStagingConfig`, `UpdateDomainAssociation`, `UpdateFieldLevelEncryptionConfig`, `UpdateFieldLevelEncryptionProfile`, `UpdateFunction`, `UpdateKeyGroup`, `UpdateKeyValueStore`, `UpdateOriginAccessControl`, `UpdateOriginRequestPolicy`, `UpdatePublicKey`, `UpdateRealtimeLogConfig`, `UpdateResponseHeadersPolicy`, `UpdateStreamingDistribution`, `UpdateTrustStore`, `UpdateVpcOrigin`
- Traits: `idempotent` (1)
- Common required input members in this group: `Id`, `IfMatch`, `Name`

### Associate

- Operations: `AssociateAlias`, `AssociateDistributionTenantWebACL`, `AssociateDistributionWebACL`
- Common required input members in this group: `Id`, `WebACLArn`

### Describe

- Operations: `DescribeConnectionFunction`, `DescribeFunction`, `DescribeKeyValueStore`
- Common required input members in this group: `Name`

### Disassociate

- Operations: `DisassociateDistributionTenantWebACL`, `DisassociateDistributionWebACL`
- Common required input members in this group: `Id`

### Publish

- Operations: `PublishConnectionFunction`, `PublishFunction`
- Common required input members in this group: `IfMatch`

### Test

- Operations: `TestConnectionFunction`, `TestFunction`
- Common required input members in this group: `IfMatch`

### Copy

- Operations: `CopyDistribution`
- Common required input members in this group: -

### Put

- Operations: `PutResourcePolicy`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Verify

- Operations: `VerifyDnsConfiguration`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateAlias` | `PUT /2020-05-31/distribution/{TargetDistributionId}/associate-alias` | - | `TargetDistributionId`, `Alias` | - | `Unit` | `AccessDenied`, `IllegalUpdate`, `InvalidArgument`, `NoSuchDistribution`, `TooManyDistributionCNAMEs` | The AssociateAlias API operation only supports standard distributions. To move domains between distribution tenants and/or standard distributions, we recommend that you use the UpdateDomainAssociation API operation i ... |
| `AssociateDistributionTenantWebACL` | `PUT /2020-05-31/distribution-tenant/{Id}/associate-web-acl` | - | `Id`, `WebACLArn` | - | `AssociateDistributionTenantWebACLResult` | `AccessDenied`, `EntityLimitExceeded`, `EntityNotFound`, `InvalidArgument`, `InvalidIfMatchVersion`, `PreconditionFailed` | Associates the WAF web ACL with a distribution tenant. |
| `AssociateDistributionWebACL` | `PUT /2020-05-31/distribution/{Id}/associate-web-acl` | - | `Id`, `WebACLArn` | - | `AssociateDistributionWebACLResult` | `AccessDenied`, `EntityLimitExceeded`, `EntityNotFound`, `InvalidArgument`, `InvalidIfMatchVersion`, `PreconditionFailed` | Associates the WAF web ACL with a distribution. |
| `CopyDistribution` | `POST /2020-05-31/distribution/{PrimaryDistributionId}/copy` | - | `PrimaryDistributionId`, `CallerReference` | - | `CopyDistributionResult` | `AccessDenied`, `CNAMEAlreadyExists`, `DistributionAlreadyExists`, `IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior`, `InconsistentQuantities`, `InvalidArgument`, `InvalidDefaultRootObject`, `InvalidErrorCode`, `InvalidForwardCookies`, `InvalidFunctionAssociation`, `InvalidGeoRestrictionParameter`, `InvalidHeadersForS3Origin`, `InvalidIfMatchVersion`, `InvalidLambdaFunctionAssociation`, `InvalidLocationCode`, `InvalidMinimumProtocolVersion`, `InvalidOrigin`, `InvalidOriginAccessControl`, `InvalidOriginAccessIdentity`, `InvalidOriginKeepaliveTimeout`, `InvalidOriginReadTimeout`, `InvalidProtocolSettings`, `InvalidQueryStringParameters`, `InvalidRelativePath`, `InvalidRequiredProtocol`, `InvalidResponseCode`, `InvalidTTLOrder`, `InvalidViewerCertificate`, `InvalidWebACLId`, `MissingBody`, `NoSuchCachePolicy`, `NoSuchDistribution`, `NoSuchFieldLevelEncryptionConfig`, `NoSuchOrigin`, `NoSuchOriginRequestPolicy`, `NoSuchRealtimeLogConfig`, `NoSuchResponseHeadersPolicy`, `PreconditionFailed`, `RealtimeLogConfigOwnerMismatch`, `TooManyCacheBehaviors`, `TooManyCertificates`, `TooManyCookieNamesInWhiteList`, `TooManyDistributionCNAMEs`, `TooManyDistributions`, `TooManyDistributionsAssociatedToCachePolicy`, `TooManyDistributionsAssociatedToFieldLevelEncryptionConfig`, `TooManyDistributionsAssociatedToKeyGroup`, `TooManyDistributionsAssociatedToOriginAccessControl`, `TooManyDistributionsAssociatedToOriginRequestPolicy`, `TooManyDistributionsAssociatedToResponseHeadersPolicy`, `TooManyDistributionsWithFunctionAssociations`, `TooManyDistributionsWithLambdaAssociations`, `TooManyDistributionsWithSingleFunctionARN`, `TooManyFunctionAssociations`, `TooManyHeadersInForwardedValues`, `TooManyKeyGroupsAssociatedToDistribution`, `TooManyLambdaFunctionAssociations`, `TooManyOriginCustomHeaders`, `TooManyOriginGroupsPerDistribution`, `TooManyOrigins`, `TooManyQueryStringParameters`, `TooManyTrustedSigners`, `TrustedKeyGroupDoesNotExist`, `TrustedSignerDoesNotExist` | Creates a staging distribution using the configuration of the provided primary distribution. A staging distribution is a copy of an existing distribution (called the primary distribution) that you can use in a contin ... |
| `CreateAnycastIpList` | `POST /2020-05-31/anycast-ip-list` | - | `Name`, `IpCount` | - | `CreateAnycastIpListResult` | `AccessDenied`, `EntityAlreadyExists`, `EntityLimitExceeded`, `InvalidArgument`, `InvalidTagging`, `UnsupportedOperation` | Creates an Anycast static IP list. |
| `CreateCachePolicy` | `POST /2020-05-31/cache-policy` | - | `CachePolicyConfig` | - | `CreateCachePolicyResult` | `AccessDenied`, `CachePolicyAlreadyExists`, `InconsistentQuantities`, `InvalidArgument`, `TooManyCachePolicies`, `TooManyCookiesInCachePolicy`, `TooManyHeadersInCachePolicy`, `TooManyQueryStringsInCachePolicy` | Creates a cache policy. After you create a cache policy, you can attach it to one or more cache behaviors. When it's attached to a cache behavior, the cache policy determines the following: The values that CloudFront ... |
| `CreateCloudFrontOriginAccessIdentity` | `POST /2020-05-31/origin-access-identity/cloudfront` | - | `CloudFrontOriginAccessIdentityConfig` | - | `CreateCloudFrontOriginAccessIdentityResult` | `CloudFrontOriginAccessIdentityAlreadyExists`, `InconsistentQuantities`, `InvalidArgument`, `MissingBody`, `TooManyCloudFrontOriginAccessIdentities` | Creates a new origin access identity. If you're using Amazon S3 for your origin, you can use an origin access identity to require users to access your content using a CloudFront URL instead of the Amazon S3 URL. For ... |
| `CreateConnectionFunction` | `POST /2020-05-31/connection-function` | - | `Name`, `ConnectionFunctionConfig`, `ConnectionFunctionCode` | - | `CreateConnectionFunctionResult` | `AccessDenied`, `EntityAlreadyExists`, `EntityLimitExceeded`, `EntitySizeLimitExceeded`, `InvalidArgument`, `InvalidTagging`, `UnsupportedOperation` | Creates a connection function. |
| `CreateConnectionGroup` | `POST /2020-05-31/connection-group` | - | `Name` | - | `CreateConnectionGroupResult` | `AccessDenied`, `EntityAlreadyExists`, `EntityLimitExceeded`, `EntityNotFound`, `InvalidArgument`, `InvalidTagging` | Creates a connection group. |
| `CreateContinuousDeploymentPolicy` | `POST /2020-05-31/continuous-deployment-policy` | - | `ContinuousDeploymentPolicyConfig` | - | `CreateContinuousDeploymentPolicyResult` | `AccessDenied`, `ContinuousDeploymentPolicyAlreadyExists`, `InconsistentQuantities`, `InvalidArgument`, `StagingDistributionInUse`, `TooManyContinuousDeploymentPolicies` | Creates a continuous deployment policy that distributes traffic for a custom domain name to two different CloudFront distributions. To use a continuous deployment policy, first use CopyDistribution to create a stagin ... |
| `CreateDistribution` | `POST /2020-05-31/distribution` | - | `DistributionConfig` | - | `CreateDistributionResult` | `AccessDenied`, `CNAMEAlreadyExists`, `ContinuousDeploymentPolicyInUse`, `DistributionAlreadyExists`, `EntityLimitExceeded`, `EntityNotFound`, `IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior`, `IllegalOriginAccessConfiguration`, `InconsistentQuantities`, `InvalidArgument`, `InvalidDefaultRootObject`, `InvalidDomainNameForOriginAccessControl`, `InvalidErrorCode`, `InvalidForwardCookies`, `InvalidFunctionAssociation`, `InvalidGeoRestrictionParameter`, `InvalidHeadersForS3Origin`, `InvalidLambdaFunctionAssociation`, `InvalidLocationCode`, `InvalidMinimumProtocolVersion`, `InvalidOrigin`, `InvalidOriginAccessControl`, `InvalidOriginAccessIdentity`, `InvalidOriginKeepaliveTimeout`, `InvalidOriginReadTimeout`, `InvalidProtocolSettings`, `InvalidQueryStringParameters`, `InvalidRelativePath`, `InvalidRequiredProtocol`, `InvalidResponseCode`, `InvalidTTLOrder`, `InvalidViewerCertificate`, `InvalidWebACLId`, `MissingBody`, `NoSuchCachePolicy`, `NoSuchContinuousDeploymentPolicy`, `NoSuchFieldLevelEncryptionConfig`, `NoSuchOrigin`, `NoSuchOriginRequestPolicy`, `NoSuchRealtimeLogConfig`, `NoSuchResponseHeadersPolicy`, `RealtimeLogConfigOwnerMismatch`, `TooManyCacheBehaviors`, `TooManyCertificates`, `TooManyCookieNamesInWhiteList`, `TooManyDistributionCNAMEs`, `TooManyDistributions`, `TooManyDistributionsAssociatedToCachePolicy`, `TooManyDistributionsAssociatedToFieldLevelEncryptionConfig`, `TooManyDistributionsAssociatedToKeyGroup`, `TooManyDistributionsAssociatedToOriginAccessControl`, `TooManyDistributionsAssociatedToOriginRequestPolicy`, `TooManyDistributionsAssociatedToResponseHeadersPolicy`, `TooManyDistributionsWithFunctionAssociations`, `TooManyDistributionsWithLambdaAssociations`, `TooManyDistributionsWithSingleFunctionARN`, `TooManyFunctionAssociations`, `TooManyHeadersInForwardedValues`, `TooManyKeyGroupsAssociatedToDistribution`, `TooManyLambdaFunctionAssociations`, `TooManyOriginCustomHeaders`, `TooManyOriginGroupsPerDistribution`, `TooManyOrigins`, `TooManyQueryStringParameters`, `TooManyTrustedSigners`, `TrustedKeyGroupDoesNotExist`, `TrustedSignerDoesNotExist` | Creates a CloudFront distribution. |
| `CreateDistributionTenant` | `POST /2020-05-31/distribution-tenant` | - | `DistributionId`, `Name`, `Domains` | - | `CreateDistributionTenantResult` | `AccessDenied`, `CNAMEAlreadyExists`, `EntityAlreadyExists`, `EntityLimitExceeded`, `EntityNotFound`, `InvalidArgument`, `InvalidAssociation`, `InvalidTagging` | Creates a distribution tenant. |
| `CreateDistributionWithTags` | `POST /2020-05-31/distribution?WithTags` | - | `DistributionConfigWithTags` | - | `CreateDistributionWithTagsResult` | `AccessDenied`, `CNAMEAlreadyExists`, `ContinuousDeploymentPolicyInUse`, `DistributionAlreadyExists`, `EntityNotFound`, `IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior`, `IllegalOriginAccessConfiguration`, `InconsistentQuantities`, `InvalidArgument`, `InvalidDefaultRootObject`, `InvalidDomainNameForOriginAccessControl`, `InvalidErrorCode`, `InvalidForwardCookies`, `InvalidFunctionAssociation`, `InvalidGeoRestrictionParameter`, `InvalidHeadersForS3Origin`, `InvalidLambdaFunctionAssociation`, `InvalidLocationCode`, `InvalidMinimumProtocolVersion`, `InvalidOrigin`, `InvalidOriginAccessControl`, `InvalidOriginAccessIdentity`, `InvalidOriginKeepaliveTimeout`, `InvalidOriginReadTimeout`, `InvalidProtocolSettings`, `InvalidQueryStringParameters`, `InvalidRelativePath`, `InvalidRequiredProtocol`, `InvalidResponseCode`, `InvalidTagging`, `InvalidTTLOrder`, `InvalidViewerCertificate`, `InvalidWebACLId`, `MissingBody`, `NoSuchCachePolicy`, `NoSuchContinuousDeploymentPolicy`, `NoSuchFieldLevelEncryptionConfig`, `NoSuchOrigin`, `NoSuchOriginRequestPolicy`, `NoSuchRealtimeLogConfig`, `NoSuchResponseHeadersPolicy`, `RealtimeLogConfigOwnerMismatch`, `TooManyCacheBehaviors`, `TooManyCertificates`, `TooManyCookieNamesInWhiteList`, `TooManyDistributionCNAMEs`, `TooManyDistributions`, `TooManyDistributionsAssociatedToCachePolicy`, `TooManyDistributionsAssociatedToFieldLevelEncryptionConfig`, `TooManyDistributionsAssociatedToKeyGroup`, `TooManyDistributionsAssociatedToOriginAccessControl`, `TooManyDistributionsAssociatedToOriginRequestPolicy`, `TooManyDistributionsAssociatedToResponseHeadersPolicy`, `TooManyDistributionsWithFunctionAssociations`, `TooManyDistributionsWithLambdaAssociations`, `TooManyDistributionsWithSingleFunctionARN`, `TooManyFunctionAssociations`, `TooManyHeadersInForwardedValues`, `TooManyKeyGroupsAssociatedToDistribution`, `TooManyLambdaFunctionAssociations`, `TooManyOriginCustomHeaders`, `TooManyOriginGroupsPerDistribution`, `TooManyOrigins`, `TooManyQueryStringParameters`, `TooManyTrustedSigners`, `TrustedKeyGroupDoesNotExist`, `TrustedSignerDoesNotExist` | Create a new distribution with tags. This API operation requires the following IAM permissions: CreateDistribution TagResource |
| `CreateFieldLevelEncryptionConfig` | `POST /2020-05-31/field-level-encryption` | - | `FieldLevelEncryptionConfig` | - | `CreateFieldLevelEncryptionConfigResult` | `FieldLevelEncryptionConfigAlreadyExists`, `InconsistentQuantities`, `InvalidArgument`, `NoSuchFieldLevelEncryptionProfile`, `QueryArgProfileEmpty`, `TooManyFieldLevelEncryptionConfigs`, `TooManyFieldLevelEncryptionContentTypeProfiles`, `TooManyFieldLevelEncryptionQueryArgProfiles` | Create a new field-level encryption configuration. |
| `CreateFieldLevelEncryptionProfile` | `POST /2020-05-31/field-level-encryption-profile` | - | `FieldLevelEncryptionProfileConfig` | - | `CreateFieldLevelEncryptionProfileResult` | `FieldLevelEncryptionProfileAlreadyExists`, `FieldLevelEncryptionProfileSizeExceeded`, `InconsistentQuantities`, `InvalidArgument`, `NoSuchPublicKey`, `TooManyFieldLevelEncryptionEncryptionEntities`, `TooManyFieldLevelEncryptionFieldPatterns`, `TooManyFieldLevelEncryptionProfiles` | Create a field-level encryption profile. |
| `CreateFunction` | `POST /2020-05-31/function` | - | `Name`, `FunctionConfig`, `FunctionCode` | - | `CreateFunctionResult` | `FunctionAlreadyExists`, `FunctionSizeLimitExceeded`, `InvalidArgument`, `TooManyFunctions`, `UnsupportedOperation` | Creates a CloudFront function. To create a function, you provide the function code and some configuration information about the function. The response contains an Amazon Resource Name (ARN) that uniquely identifies t ... |
| `CreateInvalidation` | `POST /2020-05-31/distribution/{DistributionId}/invalidation` | - | `DistributionId`, `InvalidationBatch` | - | `CreateInvalidationResult` | `AccessDenied`, `BatchTooLarge`, `InconsistentQuantities`, `InvalidArgument`, `MissingBody`, `NoSuchDistribution`, `TooManyInvalidationsInProgress` | Create a new invalidation. For more information, see Invalidating files in the Amazon CloudFront Developer Guide . |
| `CreateInvalidationForDistributionTenant` | `POST /2020-05-31/distribution-tenant/{Id}/invalidation` | - | `Id`, `InvalidationBatch` | - | `CreateInvalidationForDistributionTenantResult` | `AccessDenied`, `BatchTooLarge`, `EntityNotFound`, `InconsistentQuantities`, `InvalidArgument`, `MissingBody`, `TooManyInvalidationsInProgress` | Creates an invalidation for a distribution tenant. For more information, see Invalidating files in the Amazon CloudFront Developer Guide . |
| `CreateKeyGroup` | `POST /2020-05-31/key-group` | - | `KeyGroupConfig` | - | `CreateKeyGroupResult` | `InvalidArgument`, `KeyGroupAlreadyExists`, `TooManyKeyGroups`, `TooManyPublicKeysInKeyGroup` | Creates a key group that you can use with CloudFront signed URLs and signed cookies . To create a key group, you must specify at least one public key for the key group. After you create a key group, you can reference ... |
| `CreateKeyValueStore` | `POST /2020-05-31/key-value-store` | - | `Name` | - | `CreateKeyValueStoreResult` | `AccessDenied`, `EntityAlreadyExists`, `EntityLimitExceeded`, `EntitySizeLimitExceeded`, `InvalidArgument`, `UnsupportedOperation` | Specifies the key value store resource to add to your account. In your account, the key value store names must be unique. You can also import key value store data in JSON format from an S3 bucket by providing a valid ... |
| `CreateMonitoringSubscription` | `POST /2020-05-31/distributions/{DistributionId}/monitoring-subscription` | - | `DistributionId`, `MonitoringSubscription` | - | `CreateMonitoringSubscriptionResult` | `AccessDenied`, `MonitoringSubscriptionAlreadyExists`, `NoSuchDistribution`, `UnsupportedOperation` | Enables or disables additional Amazon CloudWatch metrics for the specified CloudFront distribution. The additional metrics incur an additional cost. For more information, see Viewing additional CloudFront distributio ... |
| `CreateOriginAccessControl` | `POST /2020-05-31/origin-access-control` | - | `OriginAccessControlConfig` | - | `CreateOriginAccessControlResult` | `InvalidArgument`, `OriginAccessControlAlreadyExists`, `TooManyOriginAccessControls` | Creates a new origin access control in CloudFront. After you create an origin access control, you can add it to an origin in a CloudFront distribution so that CloudFront sends authenticated (signed) requests to the o ... |
| `CreateOriginRequestPolicy` | `POST /2020-05-31/origin-request-policy` | - | `OriginRequestPolicyConfig` | - | `CreateOriginRequestPolicyResult` | `AccessDenied`, `InconsistentQuantities`, `InvalidArgument`, `OriginRequestPolicyAlreadyExists`, `TooManyCookiesInOriginRequestPolicy`, `TooManyHeadersInOriginRequestPolicy`, `TooManyOriginRequestPolicies`, `TooManyQueryStringsInOriginRequestPolicy` | Creates an origin request policy. After you create an origin request policy, you can attach it to one or more cache behaviors. When it's attached to a cache behavior, the origin request policy determines the values t ... |
| `CreatePublicKey` | `POST /2020-05-31/public-key` | - | `PublicKeyConfig` | - | `CreatePublicKeyResult` | `InvalidArgument`, `PublicKeyAlreadyExists`, `TooManyPublicKeys` | Uploads a public key to CloudFront that you can use with signed URLs and signed cookies , or with field-level encryption . |
| `CreateRealtimeLogConfig` | `POST /2020-05-31/realtime-log-config` | - | `EndPoints`, `Fields`, `Name`, `SamplingRate` | - | `CreateRealtimeLogConfigResult` | `AccessDenied`, `InvalidArgument`, `RealtimeLogConfigAlreadyExists`, `TooManyRealtimeLogConfigs` | Creates a real-time log configuration. After you create a real-time log configuration, you can attach it to one or more cache behaviors to send real-time log data to the specified Amazon Kinesis data stream. For more ... |
| `CreateResponseHeadersPolicy` | `POST /2020-05-31/response-headers-policy` | - | `ResponseHeadersPolicyConfig` | - | `CreateResponseHeadersPolicyResult` | `AccessDenied`, `InconsistentQuantities`, `InvalidArgument`, `ResponseHeadersPolicyAlreadyExists`, `TooLongCSPInResponseHeadersPolicy`, `TooManyCustomHeadersInResponseHeadersPolicy`, `TooManyRemoveHeadersInResponseHeadersPolicy`, `TooManyResponseHeadersPolicies` | Creates a response headers policy. A response headers policy contains information about a set of HTTP headers. To create a response headers policy, you provide some metadata about the policy and a set of configuratio ... |
| `CreateStreamingDistribution` | `POST /2020-05-31/streaming-distribution` | - | `StreamingDistributionConfig` | - | `CreateStreamingDistributionResult` | `AccessDenied`, `CNAMEAlreadyExists`, `InconsistentQuantities`, `InvalidArgument`, `InvalidOrigin`, `InvalidOriginAccessControl`, `InvalidOriginAccessIdentity`, `MissingBody`, `StreamingDistributionAlreadyExists`, `TooManyStreamingDistributionCNAMEs`, `TooManyStreamingDistributions`, `TooManyTrustedSigners`, `TrustedSignerDoesNotExist` | This API is deprecated. Amazon CloudFront is deprecating real-time messaging protocol (RTMP) distributions on December 31, 2020. For more information, read the announcement on the Amazon CloudFront discussion forum. |
| `CreateStreamingDistributionWithTags` | `POST /2020-05-31/streaming-distribution?WithTags` | - | `StreamingDistributionConfigWithTags` | - | `CreateStreamingDistributionWithTagsResult` | `AccessDenied`, `CNAMEAlreadyExists`, `InconsistentQuantities`, `InvalidArgument`, `InvalidOrigin`, `InvalidOriginAccessControl`, `InvalidOriginAccessIdentity`, `InvalidTagging`, `MissingBody`, `StreamingDistributionAlreadyExists`, `TooManyStreamingDistributionCNAMEs`, `TooManyStreamingDistributions`, `TooManyTrustedSigners`, `TrustedSignerDoesNotExist` | This API is deprecated. Amazon CloudFront is deprecating real-time messaging protocol (RTMP) distributions on December 31, 2020. For more information, read the announcement on the Amazon CloudFront discussion forum. |
| `CreateTrustStore` | `POST /2020-05-31/trust-store` | - | `Name`, `CaCertificatesBundleSource` | - | `CreateTrustStoreResult` | `AccessDenied`, `EntityAlreadyExists`, `EntityLimitExceeded`, `EntityNotFound`, `InvalidArgument`, `InvalidTagging` | Creates a trust store. |
| `CreateVpcOrigin` | `POST /2020-05-31/vpc-origin` | - | `VpcOriginEndpointConfig` | - | `CreateVpcOriginResult` | `AccessDenied`, `EntityAlreadyExists`, `EntityLimitExceeded`, `InconsistentQuantities`, `InvalidArgument`, `InvalidTagging`, `UnsupportedOperation` | Create an Amazon CloudFront VPC origin. |
| `DeleteAnycastIpList` | `DELETE /2020-05-31/anycast-ip-list/{Id}` | - | `Id`, `IfMatch` | - | `Unit` | `AccessDenied`, `CannotDeleteEntityWhileInUse`, `EntityNotFound`, `IllegalDelete`, `InvalidArgument`, `InvalidIfMatchVersion`, `PreconditionFailed`, `UnsupportedOperation` | Deletes an Anycast static IP list. |
| `DeleteCachePolicy` | `DELETE /2020-05-31/cache-policy/{Id}` | - | `Id` | - | `Unit` | `AccessDenied`, `CachePolicyInUse`, `IllegalDelete`, `InvalidIfMatchVersion`, `NoSuchCachePolicy`, `PreconditionFailed` | Deletes a cache policy. You cannot delete a cache policy if it's attached to a cache behavior. First update your distributions to remove the cache policy from all cache behaviors, then delete the cache policy. To del ... |
| `DeleteCloudFrontOriginAccessIdentity` | `DELETE /2020-05-31/origin-access-identity/cloudfront/{Id}` | - | `Id` | - | `Unit` | `AccessDenied`, `CloudFrontOriginAccessIdentityInUse`, `InvalidIfMatchVersion`, `NoSuchCloudFrontOriginAccessIdentity`, `PreconditionFailed` | Delete an origin access identity. |
| `DeleteConnectionFunction` | `DELETE /2020-05-31/connection-function/{Id}` | - | `Id`, `IfMatch` | - | `Unit` | `AccessDenied`, `CannotDeleteEntityWhileInUse`, `EntityNotFound`, `InvalidArgument`, `InvalidIfMatchVersion`, `PreconditionFailed`, `UnsupportedOperation` | Deletes a connection function. |
| `DeleteConnectionGroup` | `DELETE /2020-05-31/connection-group/{Id}` | - | `Id`, `IfMatch` | - | `Unit` | `AccessDenied`, `CannotDeleteEntityWhileInUse`, `EntityNotFound`, `InvalidIfMatchVersion`, `PreconditionFailed`, `ResourceNotDisabled` | Deletes a connection group. |
| `DeleteContinuousDeploymentPolicy` | `DELETE /2020-05-31/continuous-deployment-policy/{Id}` | - | `Id` | - | `Unit` | `AccessDenied`, `ContinuousDeploymentPolicyInUse`, `InvalidArgument`, `InvalidIfMatchVersion`, `NoSuchContinuousDeploymentPolicy`, `PreconditionFailed` | Deletes a continuous deployment policy. You cannot delete a continuous deployment policy that's attached to a primary distribution. First update your distribution to remove the continuous deployment policy, then you ... |
| `DeleteDistribution` | `DELETE /2020-05-31/distribution/{Id}` | - | `Id` | - | `Unit` | `AccessDenied`, `DistributionNotDisabled`, `InvalidIfMatchVersion`, `NoSuchDistribution`, `PreconditionFailed`, `ResourceInUse` | Delete a distribution. Before you can delete a distribution, you must disable it, which requires permission to update the distribution. Once deleted, a distribution cannot be recovered. |
| `DeleteDistributionTenant` | `DELETE /2020-05-31/distribution-tenant/{Id}` | - | `Id`, `IfMatch` | - | `Unit` | `AccessDenied`, `EntityNotFound`, `InvalidIfMatchVersion`, `PreconditionFailed`, `ResourceNotDisabled` | Deletes a distribution tenant. If you use this API operation to delete a distribution tenant that is currently enabled, the request will fail. To delete a distribution tenant, you must first disable the distribution ... |
| `DeleteFieldLevelEncryptionConfig` | `DELETE /2020-05-31/field-level-encryption/{Id}` | - | `Id` | - | `Unit` | `AccessDenied`, `FieldLevelEncryptionConfigInUse`, `InvalidIfMatchVersion`, `NoSuchFieldLevelEncryptionConfig`, `PreconditionFailed` | Remove a field-level encryption configuration. |
| `DeleteFieldLevelEncryptionProfile` | `DELETE /2020-05-31/field-level-encryption-profile/{Id}` | - | `Id` | - | `Unit` | `AccessDenied`, `FieldLevelEncryptionProfileInUse`, `InvalidIfMatchVersion`, `NoSuchFieldLevelEncryptionProfile`, `PreconditionFailed` | Remove a field-level encryption profile. |
| `DeleteFunction` | `DELETE /2020-05-31/function/{Name}` | - | `Name`, `IfMatch` | - | `Unit` | `FunctionInUse`, `InvalidIfMatchVersion`, `NoSuchFunctionExists`, `PreconditionFailed`, `UnsupportedOperation` | Deletes a CloudFront function. You cannot delete a function if it's associated with a cache behavior. First, update your distributions to remove the function association from all cache behaviors, then delete the func ... |
| `DeleteKeyGroup` | `DELETE /2020-05-31/key-group/{Id}` | - | `Id` | - | `Unit` | `InvalidIfMatchVersion`, `NoSuchResource`, `PreconditionFailed`, `ResourceInUse` | Deletes a key group. You cannot delete a key group that is referenced in a cache behavior. First update your distributions to remove the key group from all cache behaviors, then delete the key group. To delete a key ... |
| `DeleteKeyValueStore` | `DELETE /2020-05-31/key-value-store/{Name}` | `idempotent` | `Name`, `IfMatch` | - | `Unit` | `AccessDenied`, `CannotDeleteEntityWhileInUse`, `EntityNotFound`, `InvalidIfMatchVersion`, `PreconditionFailed`, `UnsupportedOperation` | Specifies the key value store to delete. |
| `DeleteMonitoringSubscription` | `DELETE /2020-05-31/distributions/{DistributionId}/monitoring-subscription` | - | `DistributionId` | - | `DeleteMonitoringSubscriptionResult` | `AccessDenied`, `NoSuchDistribution`, `NoSuchMonitoringSubscription`, `UnsupportedOperation` | Disables additional CloudWatch metrics for the specified CloudFront distribution. |
| `DeleteOriginAccessControl` | `DELETE /2020-05-31/origin-access-control/{Id}` | - | `Id` | - | `Unit` | `AccessDenied`, `InvalidIfMatchVersion`, `NoSuchOriginAccessControl`, `OriginAccessControlInUse`, `PreconditionFailed` | Deletes a CloudFront origin access control. You cannot delete an origin access control if it's in use. First, update all distributions to remove the origin access control from all origins, then delete the origin acce ... |
| `DeleteOriginRequestPolicy` | `DELETE /2020-05-31/origin-request-policy/{Id}` | - | `Id` | - | `Unit` | `AccessDenied`, `IllegalDelete`, `InvalidIfMatchVersion`, `NoSuchOriginRequestPolicy`, `OriginRequestPolicyInUse`, `PreconditionFailed` | Deletes an origin request policy. You cannot delete an origin request policy if it's attached to any cache behaviors. First update your distributions to remove the origin request policy from all cache behaviors, then ... |
| `DeletePublicKey` | `DELETE /2020-05-31/public-key/{Id}` | - | `Id` | - | `Unit` | `AccessDenied`, `InvalidIfMatchVersion`, `NoSuchPublicKey`, `PreconditionFailed`, `PublicKeyInUse` | Remove a public key you previously added to CloudFront. |
| `DeleteRealtimeLogConfig` | `POST /2020-05-31/delete-realtime-log-config` | - | - | - | `Unit` | `AccessDenied`, `InvalidArgument`, `NoSuchRealtimeLogConfig`, `RealtimeLogConfigInUse` | Deletes a real-time log configuration. You cannot delete a real-time log configuration if it's attached to a cache behavior. First update your distributions to remove the real-time log configuration from all cache be ... |
| `DeleteResourcePolicy` | `POST /2020-05-31/delete-resource-policy` | - | `ResourceArn` | - | `Unit` | `AccessDenied`, `EntityNotFound`, `IllegalDelete`, `InvalidArgument`, `PreconditionFailed`, `UnsupportedOperation` | Deletes the resource policy attached to the CloudFront resource. |
| `DeleteResponseHeadersPolicy` | `DELETE /2020-05-31/response-headers-policy/{Id}` | - | `Id` | - | `Unit` | `AccessDenied`, `IllegalDelete`, `InvalidIfMatchVersion`, `NoSuchResponseHeadersPolicy`, `PreconditionFailed`, `ResponseHeadersPolicyInUse` | Deletes a response headers policy. You cannot delete a response headers policy if it's attached to a cache behavior. First update your distributions to remove the response headers policy from all cache behaviors, the ... |
| `DeleteStreamingDistribution` | `DELETE /2020-05-31/streaming-distribution/{Id}` | - | `Id` | - | `Unit` | `AccessDenied`, `InvalidIfMatchVersion`, `NoSuchStreamingDistribution`, `PreconditionFailed`, `StreamingDistributionNotDisabled` | Delete a streaming distribution. To delete an RTMP distribution using the CloudFront API, perform the following steps. To delete an RTMP distribution using the CloudFront API : Disable the RTMP distribution. Submit a ... |
| `DeleteTrustStore` | `DELETE /2020-05-31/trust-store/{Id}` | - | `Id`, `IfMatch` | - | `Unit` | `AccessDenied`, `CannotDeleteEntityWhileInUse`, `EntityNotFound`, `InvalidArgument`, `InvalidIfMatchVersion`, `PreconditionFailed` | Deletes a trust store. |
| `DeleteVpcOrigin` | `DELETE /2020-05-31/vpc-origin/{Id}` | - | `Id`, `IfMatch` | - | `DeleteVpcOriginResult` | `AccessDenied`, `CannotDeleteEntityWhileInUse`, `EntityNotFound`, `IllegalDelete`, `InvalidArgument`, `InvalidIfMatchVersion`, `PreconditionFailed`, `UnsupportedOperation` | Delete an Amazon CloudFront VPC origin. |
| `DescribeConnectionFunction` | `GET /2020-05-31/connection-function/{Identifier}/describe` | - | `Identifier` | - | `DescribeConnectionFunctionResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument`, `UnsupportedOperation` | Describes a connection function. |
| `DescribeFunction` | `GET /2020-05-31/function/{Name}/describe` | - | `Name` | - | `DescribeFunctionResult` | `NoSuchFunctionExists`, `UnsupportedOperation` | Gets configuration information and metadata about a CloudFront function, but not the function's code. To get a function's code, use GetFunction . To get configuration information and metadata about a function, you mu ... |
| `DescribeKeyValueStore` | `GET /2020-05-31/key-value-store/{Name}` | - | `Name` | - | `DescribeKeyValueStoreResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument`, `UnsupportedOperation` | Specifies the key value store and its configuration. |
| `DisassociateDistributionTenantWebACL` | `PUT /2020-05-31/distribution-tenant/{Id}/disassociate-web-acl` | - | `Id` | - | `DisassociateDistributionTenantWebACLResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument`, `InvalidIfMatchVersion`, `PreconditionFailed` | Disassociates a distribution tenant from the WAF web ACL. |
| `DisassociateDistributionWebACL` | `PUT /2020-05-31/distribution/{Id}/disassociate-web-acl` | - | `Id` | - | `DisassociateDistributionWebACLResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument`, `InvalidIfMatchVersion`, `PreconditionFailed` | Disassociates a distribution from the WAF web ACL. |
| `GetAnycastIpList` | `GET /2020-05-31/anycast-ip-list/{Id}` | - | `Id` | - | `GetAnycastIpListResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument`, `UnsupportedOperation` | Gets an Anycast static IP list. |
| `GetCachePolicy` | `GET /2020-05-31/cache-policy/{Id}` | - | `Id` | - | `GetCachePolicyResult` | `AccessDenied`, `NoSuchCachePolicy` | Gets a cache policy, including the following metadata: The policy's identifier. The date and time when the policy was last modified. To get a cache policy, you must provide the policy's identifier. If the cache polic ... |
| `GetCachePolicyConfig` | `GET /2020-05-31/cache-policy/{Id}/config` | - | `Id` | - | `GetCachePolicyConfigResult` | `AccessDenied`, `NoSuchCachePolicy` | Gets a cache policy configuration. To get a cache policy configuration, you must provide the policy's identifier. If the cache policy is attached to a distribution's cache behavior, you can get the policy's identifie ... |
| `GetCloudFrontOriginAccessIdentity` | `GET /2020-05-31/origin-access-identity/cloudfront/{Id}` | - | `Id` | - | `GetCloudFrontOriginAccessIdentityResult` | `AccessDenied`, `NoSuchCloudFrontOriginAccessIdentity` | Get the information about an origin access identity. |
| `GetCloudFrontOriginAccessIdentityConfig` | `GET /2020-05-31/origin-access-identity/cloudfront/{Id}/config` | - | `Id` | - | `GetCloudFrontOriginAccessIdentityConfigResult` | `AccessDenied`, `NoSuchCloudFrontOriginAccessIdentity` | Get the configuration information about an origin access identity. |
| `GetConnectionFunction` | `GET /2020-05-31/connection-function/{Identifier}` | - | `Identifier` | - | `GetConnectionFunctionResult` | `AccessDenied`, `EntityNotFound`, `UnsupportedOperation` | Gets a connection function. |
| `GetConnectionGroup` | `GET /2020-05-31/connection-group/{Identifier}` | - | `Identifier` | - | `GetConnectionGroupResult` | `AccessDenied`, `EntityNotFound` | Gets information about a connection group. |
| `GetConnectionGroupByRoutingEndpoint` | `GET /2020-05-31/connection-group` | - | `RoutingEndpoint` | - | `GetConnectionGroupByRoutingEndpointResult` | `AccessDenied`, `EntityNotFound` | Gets information about a connection group by using the endpoint that you specify. |
| `GetContinuousDeploymentPolicy` | `GET /2020-05-31/continuous-deployment-policy/{Id}` | - | `Id` | - | `GetContinuousDeploymentPolicyResult` | `AccessDenied`, `NoSuchContinuousDeploymentPolicy` | Gets a continuous deployment policy, including metadata (the policy's identifier and the date and time when the policy was last modified). |
| `GetContinuousDeploymentPolicyConfig` | `GET /2020-05-31/continuous-deployment-policy/{Id}/config` | - | `Id` | - | `GetContinuousDeploymentPolicyConfigResult` | `AccessDenied`, `NoSuchContinuousDeploymentPolicy` | Gets configuration information about a continuous deployment policy. |
| `GetDistribution` | `GET /2020-05-31/distribution/{Id}` | - | `Id` | - | `GetDistributionResult` | `AccessDenied`, `NoSuchDistribution` | Get the information about a distribution. |
| `GetDistributionConfig` | `GET /2020-05-31/distribution/{Id}/config` | - | `Id` | - | `GetDistributionConfigResult` | `AccessDenied`, `NoSuchDistribution` | Get the configuration information about a distribution. |
| `GetDistributionTenant` | `GET /2020-05-31/distribution-tenant/{Identifier}` | - | `Identifier` | - | `GetDistributionTenantResult` | `AccessDenied`, `EntityNotFound` | Gets information about a distribution tenant. |
| `GetDistributionTenantByDomain` | `GET /2020-05-31/distribution-tenant` | - | `Domain` | - | `GetDistributionTenantByDomainResult` | `AccessDenied`, `EntityNotFound` | Gets information about a distribution tenant by the associated domain. |
| `GetFieldLevelEncryption` | `GET /2020-05-31/field-level-encryption/{Id}` | - | `Id` | - | `GetFieldLevelEncryptionResult` | `AccessDenied`, `NoSuchFieldLevelEncryptionConfig` | Get the field-level encryption configuration information. |
| `GetFieldLevelEncryptionConfig` | `GET /2020-05-31/field-level-encryption/{Id}/config` | - | `Id` | - | `GetFieldLevelEncryptionConfigResult` | `AccessDenied`, `NoSuchFieldLevelEncryptionConfig` | Get the field-level encryption configuration information. |
| `GetFieldLevelEncryptionProfile` | `GET /2020-05-31/field-level-encryption-profile/{Id}` | - | `Id` | - | `GetFieldLevelEncryptionProfileResult` | `AccessDenied`, `NoSuchFieldLevelEncryptionProfile` | Get the field-level encryption profile information. |
| `GetFieldLevelEncryptionProfileConfig` | `GET /2020-05-31/field-level-encryption-profile/{Id}/config` | - | `Id` | - | `GetFieldLevelEncryptionProfileConfigResult` | `AccessDenied`, `NoSuchFieldLevelEncryptionProfile` | Get the field-level encryption profile configuration information. |
| `GetFunction` | `GET /2020-05-31/function/{Name}` | - | `Name` | - | `GetFunctionResult` | `NoSuchFunctionExists`, `UnsupportedOperation` | Gets the code of a CloudFront function. To get configuration information and metadata about a function, use DescribeFunction . To get a function's code, you must provide the function's name and stage. To get these va ... |
| `GetInvalidation` | `GET /2020-05-31/distribution/{DistributionId}/invalidation/{Id}` | - | `DistributionId`, `Id` | - | `GetInvalidationResult` | `AccessDenied`, `NoSuchDistribution`, `NoSuchInvalidation` | Get the information about an invalidation. |
| `GetInvalidationForDistributionTenant` | `GET /2020-05-31/distribution-tenant/{DistributionTenantId}/invalidation/{Id}` | - | `DistributionTenantId`, `Id` | - | `GetInvalidationForDistributionTenantResult` | `AccessDenied`, `EntityNotFound`, `NoSuchInvalidation` | Gets information about a specific invalidation for a distribution tenant. |
| `GetKeyGroup` | `GET /2020-05-31/key-group/{Id}` | - | `Id` | - | `GetKeyGroupResult` | `NoSuchResource` | Gets a key group, including the date and time when the key group was last modified. To get a key group, you must provide the key group's identifier. If the key group is referenced in a distribution's cache behavior, ... |
| `GetKeyGroupConfig` | `GET /2020-05-31/key-group/{Id}/config` | - | `Id` | - | `GetKeyGroupConfigResult` | `NoSuchResource` | Gets a key group configuration. To get a key group configuration, you must provide the key group's identifier. If the key group is referenced in a distribution's cache behavior, you can get the key group's identifier ... |
| `GetManagedCertificateDetails` | `GET /2020-05-31/managed-certificate/{Identifier}` | - | `Identifier` | - | `GetManagedCertificateDetailsResult` | `AccessDenied`, `EntityNotFound` | Gets details about the CloudFront managed ACM certificate. |
| `GetMonitoringSubscription` | `GET /2020-05-31/distributions/{DistributionId}/monitoring-subscription` | - | `DistributionId` | - | `GetMonitoringSubscriptionResult` | `AccessDenied`, `NoSuchDistribution`, `NoSuchMonitoringSubscription`, `UnsupportedOperation` | Gets information about whether additional CloudWatch metrics are enabled for the specified CloudFront distribution. |
| `GetOriginAccessControl` | `GET /2020-05-31/origin-access-control/{Id}` | - | `Id` | - | `GetOriginAccessControlResult` | `AccessDenied`, `NoSuchOriginAccessControl` | Gets a CloudFront origin access control, including its unique identifier. |
| `GetOriginAccessControlConfig` | `GET /2020-05-31/origin-access-control/{Id}/config` | - | `Id` | - | `GetOriginAccessControlConfigResult` | `AccessDenied`, `NoSuchOriginAccessControl` | Gets a CloudFront origin access control configuration. |
| `GetOriginRequestPolicy` | `GET /2020-05-31/origin-request-policy/{Id}` | - | `Id` | - | `GetOriginRequestPolicyResult` | `AccessDenied`, `NoSuchOriginRequestPolicy` | Gets an origin request policy, including the following metadata: The policy's identifier. The date and time when the policy was last modified. To get an origin request policy, you must provide the policy's identifier ... |
| `GetOriginRequestPolicyConfig` | `GET /2020-05-31/origin-request-policy/{Id}/config` | - | `Id` | - | `GetOriginRequestPolicyConfigResult` | `AccessDenied`, `NoSuchOriginRequestPolicy` | Gets an origin request policy configuration. To get an origin request policy configuration, you must provide the policy's identifier. If the origin request policy is attached to a distribution's cache behavior, you c ... |
| `GetPublicKey` | `GET /2020-05-31/public-key/{Id}` | - | `Id` | - | `GetPublicKeyResult` | `AccessDenied`, `NoSuchPublicKey` | Gets a public key. |
| `GetPublicKeyConfig` | `GET /2020-05-31/public-key/{Id}/config` | - | `Id` | - | `GetPublicKeyConfigResult` | `AccessDenied`, `NoSuchPublicKey` | Gets a public key configuration. |
| `GetRealtimeLogConfig` | `POST /2020-05-31/get-realtime-log-config` | - | - | - | `GetRealtimeLogConfigResult` | `AccessDenied`, `InvalidArgument`, `NoSuchRealtimeLogConfig` | Gets a real-time log configuration. To get a real-time log configuration, you can provide the configuration's name or its Amazon Resource Name (ARN). You must provide at least one. If you provide both, CloudFront use ... |
| `GetResourcePolicy` | `POST /2020-05-31/get-resource-policy` | - | `ResourceArn` | - | `GetResourcePolicyResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument`, `UnsupportedOperation` | Retrieves the resource policy for the specified CloudFront resource that you own and have shared. |
| `GetResponseHeadersPolicy` | `GET /2020-05-31/response-headers-policy/{Id}` | - | `Id` | - | `GetResponseHeadersPolicyResult` | `AccessDenied`, `NoSuchResponseHeadersPolicy` | Gets a response headers policy, including metadata (the policy's identifier and the date and time when the policy was last modified). To get a response headers policy, you must provide the policy's identifier. If the ... |
| `GetResponseHeadersPolicyConfig` | `GET /2020-05-31/response-headers-policy/{Id}/config` | - | `Id` | - | `GetResponseHeadersPolicyConfigResult` | `AccessDenied`, `NoSuchResponseHeadersPolicy` | Gets a response headers policy configuration. To get a response headers policy configuration, you must provide the policy's identifier. If the response headers policy is attached to a distribution's cache behavior, y ... |
| `GetStreamingDistribution` | `GET /2020-05-31/streaming-distribution/{Id}` | - | `Id` | - | `GetStreamingDistributionResult` | `AccessDenied`, `NoSuchStreamingDistribution` | Gets information about a specified RTMP distribution, including the distribution configuration. |
| `GetStreamingDistributionConfig` | `GET /2020-05-31/streaming-distribution/{Id}/config` | - | `Id` | - | `GetStreamingDistributionConfigResult` | `AccessDenied`, `NoSuchStreamingDistribution` | Get the configuration information about a streaming distribution. |
| `GetTrustStore` | `GET /2020-05-31/trust-store/{Identifier}` | - | `Identifier` | - | `GetTrustStoreResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument` | Gets a trust store. |
| `GetVpcOrigin` | `GET /2020-05-31/vpc-origin/{Id}` | - | `Id` | - | `GetVpcOriginResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument`, `UnsupportedOperation` | Get the details of an Amazon CloudFront VPC origin. |
| `ListAnycastIpLists` | `GET /2020-05-31/anycast-ip-list` | - | - | - | `ListAnycastIpListsResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument`, `UnsupportedOperation` | Lists your Anycast static IP lists. |
| `ListCachePolicies` | `GET /2020-05-31/cache-policy` | - | - | - | `ListCachePoliciesResult` | `AccessDenied`, `InvalidArgument`, `NoSuchCachePolicy` | Gets a list of cache policies. You can optionally apply a filter to return only the managed policies created by Amazon Web Services, or only the custom policies created in your Amazon Web Services account. You can op ... |
| `ListCloudFrontOriginAccessIdentities` | `GET /2020-05-31/origin-access-identity/cloudfront` | `paginated` | - | - | `ListCloudFrontOriginAccessIdentitiesResult` | `InvalidArgument` | Lists origin access identities. |
| `ListConflictingAliases` | `GET /2020-05-31/conflicting-alias` | - | `DistributionId`, `Alias` | - | `ListConflictingAliasesResult` | `InvalidArgument`, `NoSuchDistribution` | The ListConflictingAliases API operation only supports standard distributions. To list domain conflicts for both standard distributions and distribution tenants, we recommend that you use the ListDomainConflicts API ... |
| `ListConnectionFunctions` | `POST /2020-05-31/connection-functions` | `paginated` | - | - | `ListConnectionFunctionsResult` | `AccessDenied`, `InvalidArgument`, `UnsupportedOperation` | Lists connection functions. |
| `ListConnectionGroups` | `POST /2020-05-31/connection-groups` | `paginated` | - | - | `ListConnectionGroupsResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument` | Lists the connection groups in your Amazon Web Services account. |
| `ListContinuousDeploymentPolicies` | `GET /2020-05-31/continuous-deployment-policy` | - | - | - | `ListContinuousDeploymentPoliciesResult` | `AccessDenied`, `InvalidArgument`, `NoSuchContinuousDeploymentPolicy` | Gets a list of the continuous deployment policies in your Amazon Web Services account. You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list excee ... |
| `ListDistributions` | `GET /2020-05-31/distribution` | `paginated` | - | - | `ListDistributionsResult` | `InvalidArgument` | List CloudFront distributions. |
| `ListDistributionsByAnycastIpListId` | `GET /2020-05-31/distributionsByAnycastIpListId/{AnycastIpListId}` | - | `AnycastIpListId` | - | `ListDistributionsByAnycastIpListIdResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument`, `UnsupportedOperation` | Lists the distributions in your account that are associated with the specified AnycastIpListId . |
| `ListDistributionsByCachePolicyId` | `GET /2020-05-31/distributionsByCachePolicyId/{CachePolicyId}` | - | `CachePolicyId` | - | `ListDistributionsByCachePolicyIdResult` | `AccessDenied`, `InvalidArgument`, `NoSuchCachePolicy` | Gets a list of distribution IDs for distributions that have a cache behavior that's associated with the specified cache policy. You can optionally specify the maximum number of items to receive in the response. If th ... |
| `ListDistributionsByConnectionFunction` | `GET /2020-05-31/distributionsByConnectionFunction` | `paginated` | `ConnectionFunctionIdentifier` | - | `ListDistributionsByConnectionFunctionResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument` | Lists distributions by connection function. |
| `ListDistributionsByConnectionMode` | `GET /2020-05-31/distributionsByConnectionMode/{ConnectionMode}` | `paginated` | `ConnectionMode` | - | `ListDistributionsByConnectionModeResult` | `AccessDenied`, `InvalidArgument` | Lists the distributions by the connection mode that you specify. |
| `ListDistributionsByKeyGroup` | `GET /2020-05-31/distributionsByKeyGroupId/{KeyGroupId}` | - | `KeyGroupId` | - | `ListDistributionsByKeyGroupResult` | `InvalidArgument`, `NoSuchResource` | Gets a list of distribution IDs for distributions that have a cache behavior that references the specified key group. You can optionally specify the maximum number of items to receive in the response. If the total nu ... |
| `ListDistributionsByOriginRequestPolicyId` | `GET /2020-05-31/distributionsByOriginRequestPolicyId/{OriginRequestPolicyId}` | - | `OriginRequestPolicyId` | - | `ListDistributionsByOriginRequestPolicyIdResult` | `AccessDenied`, `InvalidArgument`, `NoSuchOriginRequestPolicy` | Gets a list of distribution IDs for distributions that have a cache behavior that's associated with the specified origin request policy. You can optionally specify the maximum number of items to receive in the respon ... |
| `ListDistributionsByOwnedResource` | `GET /2020-05-31/distributionsByOwnedResource/{ResourceArn}` | - | `ResourceArn` | - | `ListDistributionsByOwnedResourceResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument`, `UnsupportedOperation` | Lists the CloudFront distributions that are associated with the specified resource that you own. |
| `ListDistributionsByRealtimeLogConfig` | `POST /2020-05-31/distributionsByRealtimeLogConfig` | - | - | - | `ListDistributionsByRealtimeLogConfigResult` | `InvalidArgument` | Gets a list of distributions that have a cache behavior that's associated with the specified real-time log configuration. You can specify the real-time log configuration by its name or its Amazon Resource Name (ARN). ... |
| `ListDistributionsByResponseHeadersPolicyId` | `GET /2020-05-31/distributionsByResponseHeadersPolicyId/{ResponseHeadersPolicyId}` | - | `ResponseHeadersPolicyId` | - | `ListDistributionsByResponseHeadersPolicyIdResult` | `AccessDenied`, `InvalidArgument`, `NoSuchResponseHeadersPolicy` | Gets a list of distribution IDs for distributions that have a cache behavior that's associated with the specified response headers policy. You can optionally specify the maximum number of items to receive in the resp ... |
| `ListDistributionsByTrustStore` | `GET /2020-05-31/distributionsByTrustStore` | `paginated` | `TrustStoreIdentifier` | - | `ListDistributionsByTrustStoreResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument` | Lists distributions by trust store. |
| `ListDistributionsByVpcOriginId` | `GET /2020-05-31/distributionsByVpcOriginId/{VpcOriginId}` | - | `VpcOriginId` | - | `ListDistributionsByVpcOriginIdResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument`, `UnsupportedOperation` | List CloudFront distributions by their VPC origin ID. |
| `ListDistributionsByWebACLId` | `GET /2020-05-31/distributionsByWebACLId/{WebACLId}` | - | `WebACLId` | - | `ListDistributionsByWebACLIdResult` | `InvalidArgument`, `InvalidWebACLId` | List the distributions that are associated with a specified WAF web ACL. |
| `ListDistributionTenants` | `POST /2020-05-31/distribution-tenants` | `paginated` | - | - | `ListDistributionTenantsResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument` | Lists the distribution tenants in your Amazon Web Services account. |
| `ListDistributionTenantsByCustomization` | `POST /2020-05-31/distribution-tenants-by-customization` | `paginated` | - | - | `ListDistributionTenantsByCustomizationResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument` | Lists distribution tenants by the customization that you specify. You must specify either the CertificateArn parameter or WebACLArn parameter, but not both in the same request. |
| `ListDomainConflicts` | `POST /2020-05-31/domain-conflicts` | `paginated` | `Domain`, `DomainControlValidationResource` | - | `ListDomainConflictsResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument` | We recommend that you use the ListDomainConflicts API operation to check for domain conflicts, as it supports both standard distributions and distribution tenants. ListConflictingAliases performs similar checks but o ... |
| `ListFieldLevelEncryptionConfigs` | `GET /2020-05-31/field-level-encryption` | - | - | - | `ListFieldLevelEncryptionConfigsResult` | `InvalidArgument` | List all field-level encryption configurations that have been created in CloudFront for this account. |
| `ListFieldLevelEncryptionProfiles` | `GET /2020-05-31/field-level-encryption-profile` | - | - | - | `ListFieldLevelEncryptionProfilesResult` | `InvalidArgument` | Request a list of field-level encryption profiles that have been created in CloudFront for this account. |
| `ListFunctions` | `GET /2020-05-31/function` | - | - | - | `ListFunctionsResult` | `InvalidArgument`, `UnsupportedOperation` | Gets a list of all CloudFront functions in your Amazon Web Services account. You can optionally apply a filter to return only the functions that are in the specified stage, either DEVELOPMENT or LIVE . You can option ... |
| `ListInvalidations` | `GET /2020-05-31/distribution/{DistributionId}/invalidation` | `paginated` | `DistributionId` | - | `ListInvalidationsResult` | `AccessDenied`, `InvalidArgument`, `NoSuchDistribution` | Lists invalidation batches. |
| `ListInvalidationsForDistributionTenant` | `GET /2020-05-31/distribution-tenant/{Id}/invalidation` | `paginated` | `Id` | - | `ListInvalidationsForDistributionTenantResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument` | Lists the invalidations for a distribution tenant. |
| `ListKeyGroups` | `GET /2020-05-31/key-group` | - | - | - | `ListKeyGroupsResult` | `InvalidArgument` | Gets a list of key groups. You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the default maximum, the ... |
| `ListKeyValueStores` | `GET /2020-05-31/key-value-store` | `paginated` | - | - | `ListKeyValueStoresResult` | `AccessDenied`, `InvalidArgument`, `UnsupportedOperation` | Specifies the key value stores to list. |
| `ListOriginAccessControls` | `GET /2020-05-31/origin-access-control` | `paginated` | - | - | `ListOriginAccessControlsResult` | `InvalidArgument` | Gets the list of CloudFront origin access controls (OACs) in this Amazon Web Services account. You can optionally specify the maximum number of items to receive in the response. If the total number of items in the li ... |
| `ListOriginRequestPolicies` | `GET /2020-05-31/origin-request-policy` | - | - | - | `ListOriginRequestPoliciesResult` | `AccessDenied`, `InvalidArgument`, `NoSuchOriginRequestPolicy` | Gets a list of origin request policies. You can optionally apply a filter to return only the managed policies created by Amazon Web Services, or only the custom policies created in your Amazon Web Services account. Y ... |
| `ListPublicKeys` | `GET /2020-05-31/public-key` | `paginated` | - | - | `ListPublicKeysResult` | `InvalidArgument` | List all public keys that have been added to CloudFront for this account. |
| `ListRealtimeLogConfigs` | `GET /2020-05-31/realtime-log-config` | - | - | - | `ListRealtimeLogConfigsResult` | `AccessDenied`, `InvalidArgument`, `NoSuchRealtimeLogConfig` | Gets a list of real-time log configurations. You can optionally specify the maximum number of items to receive in the response. If the total number of items in the list exceeds the maximum that you specify, or the de ... |
| `ListResponseHeadersPolicies` | `GET /2020-05-31/response-headers-policy` | - | - | - | `ListResponseHeadersPoliciesResult` | `AccessDenied`, `InvalidArgument`, `NoSuchResponseHeadersPolicy` | Gets a list of response headers policies. You can optionally apply a filter to get only the managed policies created by Amazon Web Services, or only the custom policies created in your Amazon Web Services account. Yo ... |
| `ListStreamingDistributions` | `GET /2020-05-31/streaming-distribution` | `paginated` | - | - | `ListStreamingDistributionsResult` | `InvalidArgument` | List streaming distributions. |
| `ListTagsForResource` | `GET /2020-05-31/tagging` | - | `Resource` | - | `ListTagsForResourceResult` | `AccessDenied`, `InvalidArgument`, `InvalidTagging`, `NoSuchResource` | List tags for a CloudFront resource. For more information, see Tagging a distribution in the Amazon CloudFront Developer Guide . |
| `ListTrustStores` | `POST /2020-05-31/trust-stores` | `paginated` | - | - | `ListTrustStoresResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument` | Lists trust stores. |
| `ListVpcOrigins` | `GET /2020-05-31/vpc-origin` | - | - | - | `ListVpcOriginsResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument`, `UnsupportedOperation` | List the CloudFront VPC origins in your account. |
| `PublishConnectionFunction` | `POST /2020-05-31/connection-function/{Id}/publish` | - | `Id`, `IfMatch` | - | `PublishConnectionFunctionResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument`, `InvalidIfMatchVersion`, `PreconditionFailed`, `UnsupportedOperation` | Publishes a connection function. |
| `PublishFunction` | `POST /2020-05-31/function/{Name}/publish` | - | `Name`, `IfMatch` | - | `PublishFunctionResult` | `InvalidArgument`, `InvalidIfMatchVersion`, `NoSuchFunctionExists`, `PreconditionFailed`, `UnsupportedOperation` | Publishes a CloudFront function by copying the function code from the DEVELOPMENT stage to LIVE . This automatically updates all cache behaviors that are using this function to use the newly published copy in the LIV ... |
| `PutResourcePolicy` | `POST /2020-05-31/put-resource-policy` | - | `ResourceArn`, `PolicyDocument` | - | `PutResourcePolicyResult` | `AccessDenied`, `EntityNotFound`, `IllegalUpdate`, `InvalidArgument`, `PreconditionFailed`, `UnsupportedOperation` | Creates a resource control policy for a given CloudFront resource. |
| `TagResource` | `POST /2020-05-31/tagging?Operation=Tag` | - | `Resource`, `Tags` | - | `Unit` | `AccessDenied`, `InvalidArgument`, `InvalidTagging`, `NoSuchResource` | Add tags to a CloudFront resource. For more information, see Tagging a distribution in the Amazon CloudFront Developer Guide . |
| `TestConnectionFunction` | `POST /2020-05-31/connection-function/{Id}/test` | - | `Id`, `IfMatch`, `ConnectionObject` | - | `TestConnectionFunctionResult` | `EntityNotFound`, `InvalidArgument`, `InvalidIfMatchVersion`, `PreconditionFailed`, `TestFunctionFailed`, `UnsupportedOperation` | Tests a connection function. |
| `TestFunction` | `POST /2020-05-31/function/{Name}/test` | - | `Name`, `IfMatch`, `EventObject` | - | `TestFunctionResult` | `InvalidArgument`, `InvalidIfMatchVersion`, `NoSuchFunctionExists`, `TestFunctionFailed`, `UnsupportedOperation` | Tests a CloudFront function. To test a function, you provide an event object that represents an HTTP request or response that your CloudFront distribution could receive in production. CloudFront runs the function, pa ... |
| `UntagResource` | `POST /2020-05-31/tagging?Operation=Untag` | - | `Resource`, `TagKeys` | - | `Unit` | `AccessDenied`, `InvalidArgument`, `InvalidTagging`, `NoSuchResource` | Remove tags from a CloudFront resource. For more information, see Tagging a distribution in the Amazon CloudFront Developer Guide . |
| `UpdateAnycastIpList` | `PUT /2020-05-31/anycast-ip-list/{Id}` | - | `Id`, `IfMatch` | - | `UpdateAnycastIpListResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument`, `InvalidIfMatchVersion`, `PreconditionFailed`, `UnsupportedOperation` | Updates an Anycast static IP list. |
| `UpdateCachePolicy` | `PUT /2020-05-31/cache-policy/{Id}` | - | `CachePolicyConfig`, `Id` | - | `UpdateCachePolicyResult` | `AccessDenied`, `CachePolicyAlreadyExists`, `IllegalUpdate`, `InconsistentQuantities`, `InvalidArgument`, `InvalidIfMatchVersion`, `NoSuchCachePolicy`, `PreconditionFailed`, `TooManyCookiesInCachePolicy`, `TooManyHeadersInCachePolicy`, `TooManyQueryStringsInCachePolicy` | Updates a cache policy configuration. When you update a cache policy configuration, all the fields are updated with the values provided in the request. You cannot update some fields independent of others. To update a ... |
| `UpdateCloudFrontOriginAccessIdentity` | `PUT /2020-05-31/origin-access-identity/cloudfront/{Id}/config` | - | `CloudFrontOriginAccessIdentityConfig`, `Id` | - | `UpdateCloudFrontOriginAccessIdentityResult` | `AccessDenied`, `IllegalUpdate`, `InconsistentQuantities`, `InvalidArgument`, `InvalidIfMatchVersion`, `MissingBody`, `NoSuchCloudFrontOriginAccessIdentity`, `PreconditionFailed` | Update an origin access identity. |
| `UpdateConnectionFunction` | `PUT /2020-05-31/connection-function/{Id}` | - | `Id`, `IfMatch`, `ConnectionFunctionConfig`, `ConnectionFunctionCode` | - | `UpdateConnectionFunctionResult` | `AccessDenied`, `EntityNotFound`, `EntitySizeLimitExceeded`, `InvalidArgument`, `InvalidIfMatchVersion`, `PreconditionFailed`, `UnsupportedOperation` | Updates a connection function. |
| `UpdateConnectionGroup` | `PUT /2020-05-31/connection-group/{Id}` | - | `Id`, `IfMatch` | - | `UpdateConnectionGroupResult` | `AccessDenied`, `EntityAlreadyExists`, `EntityLimitExceeded`, `EntityNotFound`, `InvalidArgument`, `InvalidIfMatchVersion`, `PreconditionFailed`, `ResourceInUse` | Updates a connection group. |
| `UpdateContinuousDeploymentPolicy` | `PUT /2020-05-31/continuous-deployment-policy/{Id}` | - | `ContinuousDeploymentPolicyConfig`, `Id` | - | `UpdateContinuousDeploymentPolicyResult` | `AccessDenied`, `InconsistentQuantities`, `InvalidArgument`, `InvalidIfMatchVersion`, `NoSuchContinuousDeploymentPolicy`, `PreconditionFailed`, `StagingDistributionInUse` | Updates a continuous deployment policy. You can update a continuous deployment policy to enable or disable it, to change the percentage of traffic that it sends to the staging distribution, or to change the staging d ... |
| `UpdateDistribution` | `PUT /2020-05-31/distribution/{Id}/config` | - | `DistributionConfig`, `Id` | - | `UpdateDistributionResult` | `AccessDenied`, `CNAMEAlreadyExists`, `ContinuousDeploymentPolicyInUse`, `EntityNotFound`, `IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior`, `IllegalOriginAccessConfiguration`, `IllegalUpdate`, `InconsistentQuantities`, `InvalidArgument`, `InvalidDefaultRootObject`, `InvalidDomainNameForOriginAccessControl`, `InvalidErrorCode`, `InvalidForwardCookies`, `InvalidFunctionAssociation`, `InvalidGeoRestrictionParameter`, `InvalidHeadersForS3Origin`, `InvalidIfMatchVersion`, `InvalidLambdaFunctionAssociation`, `InvalidLocationCode`, `InvalidMinimumProtocolVersion`, `InvalidOriginAccessControl`, `InvalidOriginAccessIdentity`, `InvalidOriginKeepaliveTimeout`, `InvalidOriginReadTimeout`, `InvalidQueryStringParameters`, `InvalidRelativePath`, `InvalidRequiredProtocol`, `InvalidResponseCode`, `InvalidTTLOrder`, `InvalidViewerCertificate`, `InvalidWebACLId`, `MissingBody`, `NoSuchCachePolicy`, `NoSuchContinuousDeploymentPolicy`, `NoSuchDistribution`, `NoSuchFieldLevelEncryptionConfig`, `NoSuchOrigin`, `NoSuchOriginRequestPolicy`, `NoSuchRealtimeLogConfig`, `NoSuchResponseHeadersPolicy`, `PreconditionFailed`, `RealtimeLogConfigOwnerMismatch`, `StagingDistributionInUse`, `TooManyCacheBehaviors`, `TooManyCertificates`, `TooManyCookieNamesInWhiteList`, `TooManyDistributionCNAMEs`, `TooManyDistributionsAssociatedToCachePolicy`, `TooManyDistributionsAssociatedToFieldLevelEncryptionConfig`, `TooManyDistributionsAssociatedToKeyGroup`, `TooManyDistributionsAssociatedToOriginAccessControl`, `TooManyDistributionsAssociatedToOriginRequestPolicy`, `TooManyDistributionsAssociatedToResponseHeadersPolicy`, `TooManyDistributionsWithFunctionAssociations`, `TooManyDistributionsWithLambdaAssociations`, `TooManyDistributionsWithSingleFunctionARN`, `TooManyFunctionAssociations`, `TooManyHeadersInForwardedValues`, `TooManyKeyGroupsAssociatedToDistribution`, `TooManyLambdaFunctionAssociations`, `TooManyOriginCustomHeaders`, `TooManyOriginGroupsPerDistribution`, `TooManyOrigins`, `TooManyQueryStringParameters`, `TooManyTrustedSigners`, `TrustedKeyGroupDoesNotExist`, `TrustedSignerDoesNotExist` | Updates the configuration for a CloudFront distribution. The update process includes getting the current distribution configuration, updating it to make your changes, and then submitting an UpdateDistribution request ... |
| `UpdateDistributionTenant` | `PUT /2020-05-31/distribution-tenant/{Id}` | - | `Id`, `IfMatch` | - | `UpdateDistributionTenantResult` | `AccessDenied`, `CNAMEAlreadyExists`, `EntityAlreadyExists`, `EntityLimitExceeded`, `EntityNotFound`, `InvalidArgument`, `InvalidAssociation`, `InvalidIfMatchVersion`, `PreconditionFailed` | Updates a distribution tenant. |
| `UpdateDistributionWithStagingConfig` | `PUT /2020-05-31/distribution/{Id}/promote-staging-config` | - | `Id` | - | `UpdateDistributionWithStagingConfigResult` | `AccessDenied`, `CNAMEAlreadyExists`, `EntityLimitExceeded`, `EntityNotFound`, `IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior`, `IllegalUpdate`, `InconsistentQuantities`, `InvalidArgument`, `InvalidDefaultRootObject`, `InvalidErrorCode`, `InvalidForwardCookies`, `InvalidFunctionAssociation`, `InvalidGeoRestrictionParameter`, `InvalidHeadersForS3Origin`, `InvalidIfMatchVersion`, `InvalidLambdaFunctionAssociation`, `InvalidLocationCode`, `InvalidMinimumProtocolVersion`, `InvalidOriginAccessControl`, `InvalidOriginAccessIdentity`, `InvalidOriginKeepaliveTimeout`, `InvalidOriginReadTimeout`, `InvalidQueryStringParameters`, `InvalidRelativePath`, `InvalidRequiredProtocol`, `InvalidResponseCode`, `InvalidTTLOrder`, `InvalidViewerCertificate`, `InvalidWebACLId`, `MissingBody`, `NoSuchCachePolicy`, `NoSuchDistribution`, `NoSuchFieldLevelEncryptionConfig`, `NoSuchOrigin`, `NoSuchOriginRequestPolicy`, `NoSuchRealtimeLogConfig`, `NoSuchResponseHeadersPolicy`, `PreconditionFailed`, `RealtimeLogConfigOwnerMismatch`, `TooManyCacheBehaviors`, `TooManyCertificates`, `TooManyCookieNamesInWhiteList`, `TooManyDistributionCNAMEs`, `TooManyDistributionsAssociatedToCachePolicy`, `TooManyDistributionsAssociatedToFieldLevelEncryptionConfig`, `TooManyDistributionsAssociatedToKeyGroup`, `TooManyDistributionsAssociatedToOriginAccessControl`, `TooManyDistributionsAssociatedToOriginRequestPolicy`, `TooManyDistributionsAssociatedToResponseHeadersPolicy`, `TooManyDistributionsWithFunctionAssociations`, `TooManyDistributionsWithLambdaAssociations`, `TooManyDistributionsWithSingleFunctionARN`, `TooManyFunctionAssociations`, `TooManyHeadersInForwardedValues`, `TooManyKeyGroupsAssociatedToDistribution`, `TooManyLambdaFunctionAssociations`, `TooManyOriginCustomHeaders`, `TooManyOriginGroupsPerDistribution`, `TooManyOrigins`, `TooManyQueryStringParameters`, `TooManyTrustedSigners`, `TrustedKeyGroupDoesNotExist`, `TrustedSignerDoesNotExist` | Copies the staging distribution's configuration to its corresponding primary distribution. The primary distribution retains its Aliases (also known as alternate domain names or CNAMEs) and ContinuousDeploymentPolicyI ... |
| `UpdateDomainAssociation` | `POST /2020-05-31/domain-association` | - | `Domain`, `TargetResource` | - | `UpdateDomainAssociationResult` | `AccessDenied`, `EntityNotFound`, `IllegalUpdate`, `InvalidArgument`, `InvalidIfMatchVersion`, `PreconditionFailed` | We recommend that you use the UpdateDomainAssociation API operation to move a domain association, as it supports both standard distributions and distribution tenants. AssociateAlias performs similar checks but only s ... |
| `UpdateFieldLevelEncryptionConfig` | `PUT /2020-05-31/field-level-encryption/{Id}/config` | - | `FieldLevelEncryptionConfig`, `Id` | - | `UpdateFieldLevelEncryptionConfigResult` | `AccessDenied`, `IllegalUpdate`, `InconsistentQuantities`, `InvalidArgument`, `InvalidIfMatchVersion`, `NoSuchFieldLevelEncryptionConfig`, `NoSuchFieldLevelEncryptionProfile`, `PreconditionFailed`, `QueryArgProfileEmpty`, `TooManyFieldLevelEncryptionContentTypeProfiles`, `TooManyFieldLevelEncryptionQueryArgProfiles` | Update a field-level encryption configuration. |
| `UpdateFieldLevelEncryptionProfile` | `PUT /2020-05-31/field-level-encryption-profile/{Id}/config` | - | `FieldLevelEncryptionProfileConfig`, `Id` | - | `UpdateFieldLevelEncryptionProfileResult` | `AccessDenied`, `FieldLevelEncryptionProfileAlreadyExists`, `FieldLevelEncryptionProfileSizeExceeded`, `IllegalUpdate`, `InconsistentQuantities`, `InvalidArgument`, `InvalidIfMatchVersion`, `NoSuchFieldLevelEncryptionProfile`, `NoSuchPublicKey`, `PreconditionFailed`, `TooManyFieldLevelEncryptionEncryptionEntities`, `TooManyFieldLevelEncryptionFieldPatterns` | Update a field-level encryption profile. |
| `UpdateFunction` | `PUT /2020-05-31/function/{Name}` | - | `Name`, `IfMatch`, `FunctionConfig`, `FunctionCode` | - | `UpdateFunctionResult` | `FunctionSizeLimitExceeded`, `InvalidArgument`, `InvalidIfMatchVersion`, `NoSuchFunctionExists`, `PreconditionFailed`, `UnsupportedOperation` | Updates a CloudFront function. You can update a function's code or the comment that describes the function. You cannot update a function's name. To update a function, you provide the function's name and version ( ETa ... |
| `UpdateKeyGroup` | `PUT /2020-05-31/key-group/{Id}` | - | `KeyGroupConfig`, `Id` | - | `UpdateKeyGroupResult` | `InvalidArgument`, `InvalidIfMatchVersion`, `KeyGroupAlreadyExists`, `NoSuchResource`, `PreconditionFailed`, `TooManyPublicKeysInKeyGroup` | Updates a key group. When you update a key group, all the fields are updated with the values provided in the request. You cannot update some fields independent of others. To update a key group: Get the current key gr ... |
| `UpdateKeyValueStore` | `PUT /2020-05-31/key-value-store/{Name}` | `idempotent` | `Name`, `Comment`, `IfMatch` | - | `UpdateKeyValueStoreResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument`, `InvalidIfMatchVersion`, `PreconditionFailed`, `UnsupportedOperation` | Specifies the key value store to update. |
| `UpdateOriginAccessControl` | `PUT /2020-05-31/origin-access-control/{Id}/config` | - | `OriginAccessControlConfig`, `Id` | - | `UpdateOriginAccessControlResult` | `AccessDenied`, `IllegalUpdate`, `InvalidArgument`, `InvalidIfMatchVersion`, `NoSuchOriginAccessControl`, `OriginAccessControlAlreadyExists`, `PreconditionFailed` | Updates a CloudFront origin access control. |
| `UpdateOriginRequestPolicy` | `PUT /2020-05-31/origin-request-policy/{Id}` | - | `OriginRequestPolicyConfig`, `Id` | - | `UpdateOriginRequestPolicyResult` | `AccessDenied`, `IllegalUpdate`, `InconsistentQuantities`, `InvalidArgument`, `InvalidIfMatchVersion`, `NoSuchOriginRequestPolicy`, `OriginRequestPolicyAlreadyExists`, `PreconditionFailed`, `TooManyCookiesInOriginRequestPolicy`, `TooManyHeadersInOriginRequestPolicy`, `TooManyQueryStringsInOriginRequestPolicy` | Updates an origin request policy configuration. When you update an origin request policy configuration, all the fields are updated with the values provided in the request. You cannot update some fields independent of ... |
| `UpdatePublicKey` | `PUT /2020-05-31/public-key/{Id}/config` | - | `PublicKeyConfig`, `Id` | - | `UpdatePublicKeyResult` | `AccessDenied`, `CannotChangeImmutablePublicKeyFields`, `IllegalUpdate`, `InvalidArgument`, `InvalidIfMatchVersion`, `NoSuchPublicKey`, `PreconditionFailed` | Update public key information. Note that the only value you can change is the comment. |
| `UpdateRealtimeLogConfig` | `PUT /2020-05-31/realtime-log-config` | - | - | - | `UpdateRealtimeLogConfigResult` | `AccessDenied`, `InvalidArgument`, `NoSuchRealtimeLogConfig` | Updates a real-time log configuration. When you update a real-time log configuration, all the parameters are updated with the values provided in the request. You cannot update some parameters independent of others. T ... |
| `UpdateResponseHeadersPolicy` | `PUT /2020-05-31/response-headers-policy/{Id}` | - | `ResponseHeadersPolicyConfig`, `Id` | - | `UpdateResponseHeadersPolicyResult` | `AccessDenied`, `IllegalUpdate`, `InconsistentQuantities`, `InvalidArgument`, `InvalidIfMatchVersion`, `NoSuchResponseHeadersPolicy`, `PreconditionFailed`, `ResponseHeadersPolicyAlreadyExists`, `TooLongCSPInResponseHeadersPolicy`, `TooManyCustomHeadersInResponseHeadersPolicy`, `TooManyRemoveHeadersInResponseHeadersPolicy` | Updates a response headers policy. When you update a response headers policy, the entire policy is replaced. You cannot update some policy fields independent of others. To update a response headers policy configurati ... |
| `UpdateStreamingDistribution` | `PUT /2020-05-31/streaming-distribution/{Id}/config` | - | `StreamingDistributionConfig`, `Id` | - | `UpdateStreamingDistributionResult` | `AccessDenied`, `CNAMEAlreadyExists`, `IllegalUpdate`, `InconsistentQuantities`, `InvalidArgument`, `InvalidIfMatchVersion`, `InvalidOriginAccessControl`, `InvalidOriginAccessIdentity`, `MissingBody`, `NoSuchStreamingDistribution`, `PreconditionFailed`, `TooManyStreamingDistributionCNAMEs`, `TooManyTrustedSigners`, `TrustedSignerDoesNotExist` | Update a streaming distribution. |
| `UpdateTrustStore` | `PUT /2020-05-31/trust-store/{Id}` | - | `Id`, `CaCertificatesBundleSource`, `IfMatch` | - | `UpdateTrustStoreResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument`, `InvalidIfMatchVersion`, `PreconditionFailed` | Updates a trust store. |
| `UpdateVpcOrigin` | `PUT /2020-05-31/vpc-origin/{Id}` | - | `VpcOriginEndpointConfig`, `Id`, `IfMatch` | - | `UpdateVpcOriginResult` | `AccessDenied`, `CannotUpdateEntityWhileInUse`, `EntityAlreadyExists`, `EntityLimitExceeded`, `EntityNotFound`, `IllegalUpdate`, `InconsistentQuantities`, `InvalidArgument`, `InvalidIfMatchVersion`, `PreconditionFailed`, `UnsupportedOperation` | Update an Amazon CloudFront VPC origin in your account. |
| `VerifyDnsConfiguration` | `POST /2020-05-31/verify-dns-configuration` | - | `Identifier` | - | `VerifyDnsConfigurationResult` | `AccessDenied`, `EntityNotFound`, `InvalidArgument` | Verify the DNS configuration for your domain names. This API operation checks whether your domain name points to the correct routing endpoint of the connection group, such as d111111abcdef8.cloudfront.net. You can us ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `AssociateAlias` | - | `Alias -> Alias` | - | - |
| `AssociateDistributionTenantWebACL` | `IfMatch -> If-Match` | - | - | - |
| `AssociateDistributionWebACL` | `IfMatch -> If-Match` | - | - | - |
| `CopyDistribution` | `Staging -> Staging`, `IfMatch -> If-Match` | - | - | - |
| `CreateCachePolicy` | - | - | - | `CachePolicyConfig` |
| `CreateCloudFrontOriginAccessIdentity` | - | - | - | `CloudFrontOriginAccessIdentityConfig` |
| `CreateContinuousDeploymentPolicy` | - | - | - | `ContinuousDeploymentPolicyConfig` |
| `CreateDistribution` | - | - | - | `DistributionConfig` |
| `CreateDistributionWithTags` | - | - | - | `DistributionConfigWithTags` |
| `CreateFieldLevelEncryptionConfig` | - | - | - | `FieldLevelEncryptionConfig` |
| `CreateFieldLevelEncryptionProfile` | - | - | - | `FieldLevelEncryptionProfileConfig` |
| `CreateInvalidation` | - | - | - | `InvalidationBatch` |
| `CreateInvalidationForDistributionTenant` | - | - | - | `InvalidationBatch` |
| `CreateKeyGroup` | - | - | - | `KeyGroupConfig` |
| `CreateMonitoringSubscription` | - | - | - | `MonitoringSubscription` |
| `CreateOriginAccessControl` | - | - | - | `OriginAccessControlConfig` |
| `CreateOriginRequestPolicy` | - | - | - | `OriginRequestPolicyConfig` |
| `CreatePublicKey` | - | - | - | `PublicKeyConfig` |
| `CreateResponseHeadersPolicy` | - | - | - | `ResponseHeadersPolicyConfig` |
| `CreateStreamingDistribution` | - | - | - | `StreamingDistributionConfig` |
| `CreateStreamingDistributionWithTags` | - | - | - | `StreamingDistributionConfigWithTags` |
| `DeleteAnycastIpList` | `IfMatch -> If-Match` | - | - | - |
| `DeleteCachePolicy` | `IfMatch -> If-Match` | - | - | - |
| `DeleteCloudFrontOriginAccessIdentity` | `IfMatch -> If-Match` | - | - | - |
| `DeleteConnectionFunction` | `IfMatch -> If-Match` | - | - | - |
| `DeleteConnectionGroup` | `IfMatch -> If-Match` | - | - | - |
| `DeleteContinuousDeploymentPolicy` | `IfMatch -> If-Match` | - | - | - |
| `DeleteDistribution` | `IfMatch -> If-Match` | - | - | - |
| `DeleteDistributionTenant` | `IfMatch -> If-Match` | - | - | - |
| `DeleteFieldLevelEncryptionConfig` | `IfMatch -> If-Match` | - | - | - |
| `DeleteFieldLevelEncryptionProfile` | `IfMatch -> If-Match` | - | - | - |
| `DeleteFunction` | `IfMatch -> If-Match` | - | - | - |
| `DeleteKeyGroup` | `IfMatch -> If-Match` | - | - | - |
| `DeleteKeyValueStore` | `IfMatch -> If-Match` | - | - | - |
| `DeleteOriginAccessControl` | `IfMatch -> If-Match` | - | - | - |
| `DeleteOriginRequestPolicy` | `IfMatch -> If-Match` | - | - | - |
| `DeletePublicKey` | `IfMatch -> If-Match` | - | - | - |
| `DeleteResponseHeadersPolicy` | `IfMatch -> If-Match` | - | - | - |
| `DeleteStreamingDistribution` | `IfMatch -> If-Match` | - | - | - |
| `DeleteTrustStore` | `IfMatch -> If-Match` | - | - | - |
| `DeleteVpcOrigin` | `IfMatch -> If-Match` | - | - | - |
| `DescribeConnectionFunction` | - | `Stage -> Stage` | - | - |
| `DescribeFunction` | - | `Stage -> Stage` | - | - |
| `DisassociateDistributionTenantWebACL` | `IfMatch -> If-Match` | - | - | - |
| `DisassociateDistributionWebACL` | `IfMatch -> If-Match` | - | - | - |
| `GetConnectionFunction` | - | `Stage -> Stage` | - | - |
| `GetConnectionGroupByRoutingEndpoint` | - | `RoutingEndpoint -> RoutingEndpoint` | - | - |
| `GetDistributionTenantByDomain` | - | `Domain -> domain` | - | - |
| `GetFunction` | - | `Stage -> Stage` | - | - |
| `ListAnycastIpLists` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListCachePolicies` | - | `Type -> Type`, `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListCloudFrontOriginAccessIdentities` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListConflictingAliases` | - | `DistributionId -> DistributionId`, `Alias -> Alias`, `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListContinuousDeploymentPolicies` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListDistributions` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListDistributionsByAnycastIpListId` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListDistributionsByCachePolicyId` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListDistributionsByConnectionFunction` | - | `Marker -> Marker`, `MaxItems -> MaxItems`, `ConnectionFunctionIdentifier -> ConnectionFunctionIdentifier` | - | - |
| `ListDistributionsByConnectionMode` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListDistributionsByKeyGroup` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListDistributionsByOriginRequestPolicyId` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListDistributionsByOwnedResource` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListDistributionsByResponseHeadersPolicyId` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListDistributionsByTrustStore` | - | `TrustStoreIdentifier -> TrustStoreIdentifier`, `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListDistributionsByVpcOriginId` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListDistributionsByWebACLId` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListFieldLevelEncryptionConfigs` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListFieldLevelEncryptionProfiles` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListFunctions` | - | `Marker -> Marker`, `MaxItems -> MaxItems`, `Stage -> Stage` | - | - |
| `ListInvalidations` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListInvalidationsForDistributionTenant` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListKeyGroups` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListKeyValueStores` | - | `Marker -> Marker`, `MaxItems -> MaxItems`, `Status -> Status` | - | - |
| `ListOriginAccessControls` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListOriginRequestPolicies` | - | `Type -> Type`, `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListPublicKeys` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListRealtimeLogConfigs` | - | `MaxItems -> MaxItems`, `Marker -> Marker` | - | - |
| `ListResponseHeadersPolicies` | - | `Type -> Type`, `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListStreamingDistributions` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `ListTagsForResource` | - | `Resource -> Resource` | - | - |
| `ListVpcOrigins` | - | `Marker -> Marker`, `MaxItems -> MaxItems` | - | - |
| `PublishConnectionFunction` | `IfMatch -> If-Match` | - | - | - |
| `PublishFunction` | `IfMatch -> If-Match` | - | - | - |
| `TagResource` | - | `Resource -> Resource` | - | `Tags` |
| `TestConnectionFunction` | `IfMatch -> If-Match` | - | - | - |
| `TestFunction` | `IfMatch -> If-Match` | - | - | - |
| `UntagResource` | - | `Resource -> Resource` | - | `TagKeys` |
| `UpdateAnycastIpList` | `IfMatch -> If-Match` | - | - | - |
| `UpdateCachePolicy` | `IfMatch -> If-Match` | - | - | `CachePolicyConfig` |
| `UpdateCloudFrontOriginAccessIdentity` | `IfMatch -> If-Match` | - | - | `CloudFrontOriginAccessIdentityConfig` |
| `UpdateConnectionFunction` | `IfMatch -> If-Match` | - | - | - |
| `UpdateConnectionGroup` | `IfMatch -> If-Match` | - | - | - |
| `UpdateContinuousDeploymentPolicy` | `IfMatch -> If-Match` | - | - | `ContinuousDeploymentPolicyConfig` |
| `UpdateDistribution` | `IfMatch -> If-Match` | - | - | `DistributionConfig` |
| `UpdateDistributionTenant` | `IfMatch -> If-Match` | - | - | - |
| `UpdateDistributionWithStagingConfig` | `IfMatch -> If-Match` | `StagingDistributionId -> StagingDistributionId` | - | - |
| `UpdateDomainAssociation` | `IfMatch -> If-Match` | - | - | - |
| `UpdateFieldLevelEncryptionConfig` | `IfMatch -> If-Match` | - | - | `FieldLevelEncryptionConfig` |
| `UpdateFieldLevelEncryptionProfile` | `IfMatch -> If-Match` | - | - | `FieldLevelEncryptionProfileConfig` |
| `UpdateFunction` | `IfMatch -> If-Match` | - | - | - |
| `UpdateKeyGroup` | `IfMatch -> If-Match` | - | - | `KeyGroupConfig` |
| `UpdateKeyValueStore` | `IfMatch -> If-Match` | - | - | - |
| `UpdateOriginAccessControl` | `IfMatch -> If-Match` | - | - | `OriginAccessControlConfig` |
| `UpdateOriginRequestPolicy` | `IfMatch -> If-Match` | - | - | `OriginRequestPolicyConfig` |
| `UpdatePublicKey` | `IfMatch -> If-Match` | - | - | `PublicKeyConfig` |
| `UpdateResponseHeadersPolicy` | `IfMatch -> If-Match` | - | - | `ResponseHeadersPolicyConfig` |
| `UpdateStreamingDistribution` | `IfMatch -> If-Match` | - | - | `StreamingDistributionConfig` |
| `UpdateTrustStore` | `IfMatch -> If-Match` | - | - | `CaCertificatesBundleSource` |
| `UpdateVpcOrigin` | `IfMatch -> If-Match` | - | - | `VpcOriginEndpointConfig` |

**Conditional-write/read coverage:** the following operations model RFC 7232 conditional headers and therefore must enforce 412 PreconditionFailed (and may emit 409 ConditionalRequestConflict on races) even though those error codes are typically not in the modelled `errors:` list: `AssociateDistributionTenantWebACL`, `AssociateDistributionWebACL`, `CopyDistribution`, `DeleteAnycastIpList`, `DeleteCachePolicy`, `DeleteCloudFrontOriginAccessIdentity`, `DeleteConnectionFunction`, `DeleteConnectionGroup`, `DeleteContinuousDeploymentPolicy`, `DeleteDistribution`, `DeleteDistributionTenant`, `DeleteFieldLevelEncryptionConfig`, `DeleteFieldLevelEncryptionProfile`, `DeleteFunction`, `DeleteKeyGroup`, `DeleteKeyValueStore`, `DeleteOriginAccessControl`, `DeleteOriginRequestPolicy`, `DeletePublicKey`, `DeleteResponseHeadersPolicy`, `DeleteStreamingDistribution`, `DeleteTrustStore`, `DeleteVpcOrigin`, `DisassociateDistributionTenantWebACL`, `DisassociateDistributionWebACL`, `PublishConnectionFunction`, `PublishFunction`, `TestConnectionFunction`, `TestFunction`, `UpdateAnycastIpList`, `UpdateCachePolicy`, `UpdateCloudFrontOriginAccessIdentity`, `UpdateConnectionFunction`, `UpdateConnectionGroup`, `UpdateContinuousDeploymentPolicy`, `UpdateDistribution`, `UpdateDistributionTenant`, `UpdateDistributionWithStagingConfig`, `UpdateDomainAssociation`, `UpdateFieldLevelEncryptionConfig`, `UpdateFieldLevelEncryptionProfile`, `UpdateFunction`, `UpdateKeyGroup`, `UpdateKeyValueStore`, `UpdateOriginAccessControl`, `UpdateOriginRequestPolicy`, `UpdatePublicKey`, `UpdateResponseHeadersPolicy`, `UpdateStreamingDistribution`, `UpdateTrustStore`, `UpdateVpcOrigin`.

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDenied` | `structure` | Message | Access denied. |
| `BatchTooLarge` | `structure` | Message | Invalidation batch specified is too large. |
| `CNAMEAlreadyExists` | `structure` | Message | The CNAME specified is already defined for CloudFront. |
| `CachePolicyAlreadyExists` | `structure` | Message | A cache policy with this name already exists. You must provide a unique name. To modify an existing cache policy, use UpdateCachePolicy . |
| `CachePolicyInUse` | `structure` | Message | Cannot delete the cache policy because it is attached to one or more cache behaviors. |
| `CannotChangeImmutablePublicKeyFields` | `structure` | Message | You can't change the value of a public key. |
| `CannotDeleteEntityWhileInUse` | `structure` | Message | The entity cannot be deleted while it is in use. |
| `CannotUpdateEntityWhileInUse` | `structure` | Message | The entity cannot be updated while it is in use. |
| `CloudFrontOriginAccessIdentityAlreadyExists` | `structure` | Message | If the CallerReference is a value you already sent in a previous request to create an identity but the content of the CloudFrontOriginAccessIdentityConfig i ... |
| `CloudFrontOriginAccessIdentityInUse` | `structure` | Message | The Origin Access Identity specified is already in use. |
| `ContinuousDeploymentPolicyAlreadyExists` | `structure` | Message | A continuous deployment policy with this configuration already exists. |
| `ContinuousDeploymentPolicyInUse` | `structure` | Message | You cannot delete a continuous deployment policy that is associated with a primary distribution. |
| `DistributionAlreadyExists` | `structure` | Message | The caller reference you attempted to create the distribution with is associated with another distribution. |
| `DistributionNotDisabled` | `structure` | Message | The specified CloudFront distribution is not disabled. You must disable the distribution before you can delete it. |
| `EntityAlreadyExists` | `structure` | Message | The entity already exists. You must provide a unique entity. |
| `EntityLimitExceeded` | `structure` | Message | The entity limit has been exceeded. |
| `EntityNotFound` | `structure` | Message | The entity was not found. |
| `EntitySizeLimitExceeded` | `structure` | Message | The entity size limit was exceeded. |
| `FieldLevelEncryptionConfigAlreadyExists` | `structure` | Message | The specified configuration for field-level encryption already exists. |
| `FieldLevelEncryptionConfigInUse` | `structure` | Message | The specified configuration for field-level encryption is in use. |
| `FieldLevelEncryptionProfileAlreadyExists` | `structure` | Message | The specified profile for field-level encryption already exists. |
| `FieldLevelEncryptionProfileInUse` | `structure` | Message | The specified profile for field-level encryption is in use. |
| `FieldLevelEncryptionProfileSizeExceeded` | `structure` | Message | The maximum size of a profile for field-level encryption was exceeded. |
| `FunctionAlreadyExists` | `structure` | Message | A function with the same name already exists in this Amazon Web Services account. To create a function, you must provide a unique name. To update an existin ... |
| `FunctionInUse` | `structure` | Message | Cannot delete the function because it's attached to one or more cache behaviors. |
| `FunctionSizeLimitExceeded` | `structure` | Message | The function is too large. For more information, see Quotas (formerly known as limits) in the Amazon CloudFront Developer Guide . |
| `IllegalDelete` | `structure` | Message | Deletion is not allowed for this entity. |
| `IllegalFieldLevelEncryptionConfigAssociationWithCacheBehavior` | `structure` | Message | The specified configuration for field-level encryption can't be associated with the specified cache behavior. |
| `IllegalOriginAccessConfiguration` | `structure` | Message | An origin cannot contain both an origin access control (OAC) and an origin access identity (OAI). |
| `IllegalUpdate` | `structure` | Message | The update contains modifications that are not allowed. |
| `InconsistentQuantities` | `structure` | Message | The value of Quantity and the size of Items don't match. |
| `InvalidArgument` | `structure` | Message | An argument is invalid. |
| `InvalidAssociation` | `structure` | Message | The specified CloudFront resource can't be associated. |
| `InvalidDefaultRootObject` | `structure` | Message | The default root object file name is too big or contains an invalid character. |
| `InvalidDomainNameForOriginAccessControl` | `structure` | Message | An origin access control is associated with an origin whose domain name is not supported. |
| `InvalidErrorCode` | `structure` | Message | An invalid error code was specified. |
| `InvalidForwardCookies` | `structure` | Message | Your request contains forward cookies option which doesn't match with the expectation for the whitelisted list of cookie names. Either list of cookie names ... |
| `InvalidFunctionAssociation` | `structure` | Message | A CloudFront function association is invalid. |
| `InvalidGeoRestrictionParameter` | `structure` | Message | The specified geo restriction parameter is not valid. |
| `InvalidHeadersForS3Origin` | `structure` | Message | The headers specified are not valid for an Amazon S3 origin. |
| `InvalidIfMatchVersion` | `structure` | Message | The If-Match version is missing or not valid. |
| `InvalidLambdaFunctionAssociation` | `structure` | Message | The specified Lambda@Edge function association is invalid. |
| `InvalidLocationCode` | `structure` | Message | The location code specified is not valid. |
| `InvalidMinimumProtocolVersion` | `structure` | Message | The minimum protocol version specified is not valid. |
| `InvalidOrigin` | `structure` | Message | The Amazon S3 origin server specified does not refer to a valid Amazon S3 bucket. |
| `InvalidOriginAccessControl` | `structure` | Message | The origin access control is not valid. |
| `InvalidOriginAccessIdentity` | `structure` | Message | The origin access identity is not valid or doesn't exist. |
| `InvalidOriginKeepaliveTimeout` | `structure` | Message | The keep alive timeout specified for the origin is not valid. |
| `InvalidOriginReadTimeout` | `structure` | Message | The read timeout specified for the origin is not valid. |
| `InvalidProtocolSettings` | `structure` | Message | You cannot specify SSLv3 as the minimum protocol version if you only want to support only clients that support Server Name Indication (SNI). |
| `InvalidQueryStringParameters` | `structure` | Message | The query string parameters specified are not valid. |
| `InvalidRelativePath` | `structure` | Message | The relative path is too big, is not URL-encoded, or does not begin with a slash (/). |
| `InvalidRequiredProtocol` | `structure` | Message | This operation requires the HTTPS protocol. Ensure that you specify the HTTPS protocol in your request, or omit the RequiredProtocols element from your dist ... |
| `InvalidResponseCode` | `structure` | Message | A response code is not valid. |
| `InvalidTTLOrder` | `structure` | Message | The TTL order specified is not valid. |
| `InvalidTagging` | `structure` | Message | The tagging specified is not valid. |
| `InvalidViewerCertificate` | `structure` | Message | A viewer certificate specified is not valid. |
| `InvalidWebACLId` | `structure` | Message | A web ACL ID specified is not valid. To specify a web ACL created using the latest version of WAF, use the ACL ARN, for example arn:aws:wafv2:us-east-1:1234 ... |
| `KeyGroupAlreadyExists` | `structure` | Message | A key group with this name already exists. You must provide a unique name. To modify an existing key group, use UpdateKeyGroup . |
| `MissingBody` | `structure` | Message | This operation requires a body. Ensure that the body is present and the Content-Type header is set. |
| `MonitoringSubscriptionAlreadyExists` | `structure` | Message | A monitoring subscription already exists for the specified distribution. |
| `NoSuchCachePolicy` | `structure` | Message | The cache policy does not exist. |
| `NoSuchCloudFrontOriginAccessIdentity` | `structure` | Message | The specified origin access identity does not exist. |
| `NoSuchContinuousDeploymentPolicy` | `structure` | Message | The continuous deployment policy doesn't exist. |
| `NoSuchDistribution` | `structure` | Message | The specified distribution does not exist. |
| `NoSuchFieldLevelEncryptionConfig` | `structure` | Message | The specified configuration for field-level encryption doesn't exist. |
| `NoSuchFieldLevelEncryptionProfile` | `structure` | Message | The specified profile for field-level encryption doesn't exist. |
| `NoSuchFunctionExists` | `structure` | Message | The function does not exist. |
| `NoSuchInvalidation` | `structure` | Message | The specified invalidation does not exist. |
| `NoSuchMonitoringSubscription` | `structure` | Message | A monitoring subscription does not exist for the specified distribution. |
| `NoSuchOrigin` | `structure` | Message | No origin exists with the specified Origin Id . |
| `NoSuchOriginAccessControl` | `structure` | Message | The origin access control does not exist. |
| `NoSuchOriginRequestPolicy` | `structure` | Message | The origin request policy does not exist. |
| `NoSuchPublicKey` | `structure` | Message | The specified public key doesn't exist. |
| `NoSuchRealtimeLogConfig` | `structure` | Message | The real-time log configuration does not exist. |
| `NoSuchResource` | `structure` | Message | A resource that was specified is not valid. |
| `NoSuchResponseHeadersPolicy` | `structure` | Message | The response headers policy does not exist. |
| `NoSuchStreamingDistribution` | `structure` | Message | The specified streaming distribution does not exist. |
| `OriginAccessControlAlreadyExists` | `structure` | Message | An origin access control with the specified parameters already exists. |
| `OriginAccessControlInUse` | `structure` | Message | Cannot delete the origin access control because it's in use by one or more distributions. |
| `OriginRequestPolicyAlreadyExists` | `structure` | Message | An origin request policy with this name already exists. You must provide a unique name. To modify an existing origin request policy, use UpdateOriginRequest ... |
| `OriginRequestPolicyInUse` | `structure` | Message | Cannot delete the origin request policy because it is attached to one or more cache behaviors. |
| `PreconditionFailed` | `structure` | Message | The precondition in one or more of the request fields evaluated to false . |
| `PublicKeyAlreadyExists` | `structure` | Message | The specified public key already exists. |
| `PublicKeyInUse` | `structure` | Message | The specified public key is in use. |
| `QueryArgProfileEmpty` | `structure` | Message | No profile specified for the field-level encryption query argument. |
| `RealtimeLogConfigAlreadyExists` | `structure` | Message | A real-time log configuration with this name already exists. You must provide a unique name. To modify an existing real-time log configuration, use UpdateRe ... |
| `RealtimeLogConfigInUse` | `structure` | Message | Cannot delete the real-time log configuration because it is attached to one or more cache behaviors. |
| `RealtimeLogConfigOwnerMismatch` | `structure` | Message | The specified real-time log configuration belongs to a different Amazon Web Services account. |
| `ResourceInUse` | `structure` | Message | Cannot delete this resource because it is in use. |
| `ResourceNotDisabled` | `structure` | Message | The specified CloudFront resource hasn't been disabled yet. |
| `ResponseHeadersPolicyAlreadyExists` | `structure` | Message | A response headers policy with this name already exists. You must provide a unique name. To modify an existing response headers policy, use UpdateResponseHe ... |
| `ResponseHeadersPolicyInUse` | `structure` | Message | Cannot delete the response headers policy because it is attached to one or more cache behaviors in a CloudFront distribution. |
| `StagingDistributionInUse` | `structure` | Message | A continuous deployment policy for this staging distribution already exists. |
| `StreamingDistributionAlreadyExists` | `structure` | Message | The caller reference you attempted to create the streaming distribution with is associated with another distribution |
| `StreamingDistributionNotDisabled` | `structure` | Message | The specified CloudFront distribution is not disabled. You must disable the distribution before you can delete it. |
| `TestFunctionFailed` | `structure` | Message | The CloudFront function failed. |
| `TooLongCSPInResponseHeadersPolicy` | `structure` | Message | The length of the Content-Security-Policy header value in the response headers policy exceeds the maximum. For more information, see Quotas (formerly known ... |
| `TooManyCacheBehaviors` | `structure` | Message | You cannot create more cache behaviors for the distribution. |
| `TooManyCachePolicies` | `structure` | Message | You have reached the maximum number of cache policies for this Amazon Web Services account. For more information, see Quotas (formerly known as limits) in t ... |
| `TooManyCertificates` | `structure` | Message | You cannot create anymore custom SSL/TLS certificates. |
| `TooManyCloudFrontOriginAccessIdentities` | `structure` | Message | Processing your request would cause you to exceed the maximum number of origin access identities allowed. |
| `TooManyContinuousDeploymentPolicies` | `structure` | Message | You have reached the maximum number of continuous deployment policies for this Amazon Web Services account. |
| `TooManyCookieNamesInWhiteList` | `structure` | Message | Your request contains more cookie names in the whitelist than are allowed per cache behavior. |
| `TooManyCookiesInCachePolicy` | `structure` | Message | The number of cookies in the cache policy exceeds the maximum. For more information, see Quotas (formerly known as limits) in the Amazon CloudFront Develope ... |
| `TooManyCookiesInOriginRequestPolicy` | `structure` | Message | The number of cookies in the origin request policy exceeds the maximum. For more information, see Quotas (formerly known as limits) in the Amazon CloudFront ... |
| `TooManyCustomHeadersInResponseHeadersPolicy` | `structure` | Message | The number of custom headers in the response headers policy exceeds the maximum. For more information, see Quotas (formerly known as limits) in the Amazon C ... |
| `TooManyDistributionCNAMEs` | `structure` | Message | Your request contains more CNAMEs than are allowed per distribution. |
| `TooManyDistributions` | `structure` | Message | Processing your request would cause you to exceed the maximum number of distributions allowed. |
| `TooManyDistributionsAssociatedToCachePolicy` | `structure` | Message | The maximum number of distributions have been associated with the specified cache policy. For more information, see Quotas (formerly known as limits) in the ... |
| `TooManyDistributionsAssociatedToFieldLevelEncryptionConfig` | `structure` | Message | The maximum number of distributions have been associated with the specified configuration for field-level encryption. |
| `TooManyDistributionsAssociatedToKeyGroup` | `structure` | Message | The number of distributions that reference this key group is more than the maximum allowed. For more information, see Quotas (formerly known as limits) in t ... |
| `TooManyDistributionsAssociatedToOriginAccessControl` | `structure` | Message | The maximum number of distributions have been associated with the specified origin access control. For more information, see Quotas (formerly known as limit ... |
| `TooManyDistributionsAssociatedToOriginRequestPolicy` | `structure` | Message | The maximum number of distributions have been associated with the specified origin request policy. For more information, see Quotas (formerly known as limit ... |
| `TooManyDistributionsAssociatedToResponseHeadersPolicy` | `structure` | Message | The maximum number of distributions have been associated with the specified response headers policy. For more information, see Quotas (formerly known as lim ... |
| `TooManyDistributionsWithFunctionAssociations` | `structure` | Message | You have reached the maximum number of distributions that are associated with a CloudFront function. For more information, see Quotas (formerly known as lim ... |
| `TooManyDistributionsWithLambdaAssociations` | `structure` | Message | Processing your request would cause the maximum number of distributions with Lambda@Edge function associations per owner to be exceeded. |
| `TooManyDistributionsWithSingleFunctionARN` | `structure` | Message | The maximum number of distributions have been associated with the specified Lambda@Edge function. |
| `TooManyFieldLevelEncryptionConfigs` | `structure` | Message | The maximum number of configurations for field-level encryption have been created. |
| `TooManyFieldLevelEncryptionContentTypeProfiles` | `structure` | Message | The maximum number of content type profiles for field-level encryption have been created. |
| `TooManyFieldLevelEncryptionEncryptionEntities` | `structure` | Message | The maximum number of encryption entities for field-level encryption have been created. |
| `TooManyFieldLevelEncryptionFieldPatterns` | `structure` | Message | The maximum number of field patterns for field-level encryption have been created. |
| `TooManyFieldLevelEncryptionProfiles` | `structure` | Message | The maximum number of profiles for field-level encryption have been created. |
| `TooManyFieldLevelEncryptionQueryArgProfiles` | `structure` | Message | The maximum number of query arg profiles for field-level encryption have been created. |
| `TooManyFunctionAssociations` | `structure` | Message | You have reached the maximum number of CloudFront function associations for this distribution. For more information, see Quotas (formerly known as limits) i ... |
| `TooManyFunctions` | `structure` | Message | You have reached the maximum number of CloudFront functions for this Amazon Web Services account. For more information, see Quotas (formerly known as limits ... |
| `TooManyHeadersInCachePolicy` | `structure` | Message | The number of headers in the cache policy exceeds the maximum. For more information, see Quotas (formerly known as limits) in the Amazon CloudFront Develope ... |
| `TooManyHeadersInForwardedValues` | `structure` | Message | Your request contains too many headers in forwarded values. |
| `TooManyHeadersInOriginRequestPolicy` | `structure` | Message | The number of headers in the origin request policy exceeds the maximum. For more information, see Quotas (formerly known as limits) in the Amazon CloudFront ... |
| `TooManyInvalidationsInProgress` | `structure` | Message | You have exceeded the maximum number of allowable InProgress invalidation batch requests, or invalidation objects. |
| `TooManyKeyGroups` | `structure` | Message | You have reached the maximum number of key groups for this Amazon Web Services account. For more information, see Quotas (formerly known as limits) in the A ... |
| `TooManyKeyGroupsAssociatedToDistribution` | `structure` | Message | The number of key groups referenced by this distribution is more than the maximum allowed. For more information, see Quotas (formerly known as limits) in th ... |
| `TooManyLambdaFunctionAssociations` | `structure` | Message | Your request contains more Lambda@Edge function associations than are allowed per distribution. |
| `TooManyOriginAccessControls` | `structure` | Message | The number of origin access controls in your Amazon Web Services account exceeds the maximum allowed. For more information, see Quotas (formerly known as li ... |
| `TooManyOriginCustomHeaders` | `structure` | Message | Your request contains too many origin custom headers. |
| `TooManyOriginGroupsPerDistribution` | `structure` | Message | Processing your request would cause you to exceed the maximum number of origin groups allowed. |
| `TooManyOriginRequestPolicies` | `structure` | Message | You have reached the maximum number of origin request policies for this Amazon Web Services account. For more information, see Quotas (formerly known as lim ... |
| `TooManyOrigins` | `structure` | Message | You cannot create more origins for the distribution. |
| `TooManyPublicKeys` | `structure` | Message | The maximum number of public keys for field-level encryption have been created. To create a new public key, delete one of the existing keys. |
| `TooManyPublicKeysInKeyGroup` | `structure` | Message | The number of public keys in this key group is more than the maximum allowed. For more information, see Quotas (formerly known as limits) in the Amazon Clou ... |
| `TooManyQueryStringParameters` | `structure` | Message | Your request contains too many query string parameters. |
| `TooManyQueryStringsInCachePolicy` | `structure` | Message | The number of query strings in the cache policy exceeds the maximum. For more information, see Quotas (formerly known as limits) in the Amazon CloudFront De ... |
| `TooManyQueryStringsInOriginRequestPolicy` | `structure` | Message | The number of query strings in the origin request policy exceeds the maximum. For more information, see Quotas (formerly known as limits) in the Amazon Clou ... |
| `TooManyRealtimeLogConfigs` | `structure` | Message | You have reached the maximum number of real-time log configurations for this Amazon Web Services account. For more information, see Quotas (formerly known a ... |
| `TooManyRemoveHeadersInResponseHeadersPolicy` | `structure` | Message | The number of headers in RemoveHeadersConfig in the response headers policy exceeds the maximum. For more information, see Quotas (formerly known as limits) ... |
| `TooManyResponseHeadersPolicies` | `structure` | Message | You have reached the maximum number of response headers policies for this Amazon Web Services account. For more information, see Quotas (formerly known as l ... |
| `TooManyStreamingDistributionCNAMEs` | `structure` | Message | Your request contains more CNAMEs than are allowed per distribution. |
| `TooManyStreamingDistributions` | `structure` | Message | Processing your request would cause you to exceed the maximum number of streaming distributions allowed. |
| `TooManyTrustedSigners` | `structure` | Message | Your request contains more trusted signers than are allowed per distribution. |
| `TrustedKeyGroupDoesNotExist` | `structure` | Message | The specified key group does not exist. |
| `TrustedSignerDoesNotExist` | `structure` | Message | One or more of your trusted signers don't exist. |
| `UnsupportedOperation` | `structure` | Message | This operation is not supported in this Amazon Web Services Region. |
## Winterbaume LTM Notes

Sources: .agents/docs/LTM/smithy-codegen-and-wire-serialization.md, .agents/docs/TODO.md.

Mode: reference summary.

- `.agents/docs/LTM/smithy-codegen-and-wire-serialization.md`: records CloudFront as the remaining restXml URL-routing dispatch shape to check before declaring request-deserialiser adoption complete.
- Service implication: CloudFront's `TagResource` / `UntagResource` route through query-string dispatch (`?Operation=Tag&Resource=...`), so those `params.get` reads are routing metadata rather than operation input-shape extraction.
- Service implication: body XML parsing can only move fully onto generated request deserialisers once dispatch keeps enough request context (`&MockRequest`, labels, query, and body) available to handlers instead of pre-extracting IDs and `body_str`.

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
