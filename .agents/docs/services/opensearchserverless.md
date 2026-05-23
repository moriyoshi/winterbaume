# OpenSearch Service Serverless

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Use the Amazon OpenSearch Serverless API to create, configure, and manage OpenSearch Serverless collections and security policies. OpenSearch Serverless is an on-demand, pre-provisioned serverless configuration for Amazon OpenSearch Service. OpenSearch Serverless removes the operational complexities of provisioning, configuring, and tuning your OpenSearch clusters. It enables you to easily search and analyze petabytes of data without having to worry about the underlying infrastructure and data management. To learn more about OpenSearch Serverless, see What is Amazon OpenSearch Serverless?

## Possible Usage Scenarios
- Backported from `crates/winterbaume-opensearchserverless/tests/scenario_test.rs`: provision a vector-search collection protected by encryption and access policies.
- Backported from `scenario_test.rs`: manage multiple collections of different types and apply the corresponding policy resources.
- Scenario insight from EC2: exercise account or service defaults for OpenSearch Service Serverless by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: support serverless search and vector workloads with collections, security/access/encryption/network policies, VPC endpoints, lifecycle state, and tag management.

## Service Identity and Protocol

- AWS model slug: `opensearchserverless`
- AWS SDK for Rust slug: `opensearchserverless`
- Model version: `2021-11-01`
- Model file: `vendor/api-models-aws/models/opensearchserverless/service/2021-11-01/opensearchserverless-2021-11-01.json`
- SDK ID: `OpenSearchServerless`
- Endpoint prefix: `-`
- ARN namespace: `aoss`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Update` (9), `Create` (8), `Delete` (8), `List` (8), `Get` (6), `Batch` (5), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchGetCollection`, `BatchGetCollectionGroup`, `BatchGetEffectiveLifecyclePolicy`, `BatchGetLifecyclePolicy`, `BatchGetVpcEndpoint`, `CreateAccessPolicy`, `CreateCollection`, `CreateCollectionGroup`, `CreateIndex`, `CreateLifecyclePolicy`, `CreateSecurityConfig`, `CreateSecurityPolicy`, `CreateVpcEndpoint`, `DeleteAccessPolicy`, `DeleteCollection`, `DeleteCollectionGroup`, `DeleteIndex`, `DeleteLifecyclePolicy`, `DeleteSecurityConfig`, `DeleteSecurityPolicy`, ... (+12).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `BatchGetCollection`, `BatchGetCollectionGroup`, `BatchGetEffectiveLifecyclePolicy`, `BatchGetLifecyclePolicy`, `BatchGetVpcEndpoint`, `GetAccessPolicy`, `GetAccountSettings`, `GetIndex`, `GetPoliciesStats`, `GetSecurityConfig`, `GetSecurityPolicy`, `ListAccessPolicies`, `ListCollectionGroups`, `ListCollections`, `ListLifecyclePolicies`, `ListSecurityConfigs`, `ListSecurityPolicies`, `ListTagsForResource`, `ListVpcEndpoints`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 24 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 46 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `EC2/VPC`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AccessPolicy` | `name`, `type` | put: `CreateAccessPolicy`; read: `GetAccessPolicy`; update: `UpdateAccessPolicy`; delete: `DeleteAccessPolicy`; list: `ListAccessPolicies` | - | - |
| `Collection` | `id` | create: `CreateCollection`; update: `UpdateCollection`; delete: `DeleteCollection`; list: `ListCollections` | - | - |
| `CollectionGroup` | `id` | create: `CreateCollectionGroup`; update: `UpdateCollectionGroup`; delete: `DeleteCollectionGroup`; list: `ListCollectionGroups` | - | - |
| `Index` | `id`, `indexName` | put: `CreateIndex`; read: `GetIndex`; update: `UpdateIndex`; delete: `DeleteIndex` | - | - |
| `LifecyclePolicy` | `name`, `type` | update: `UpdateLifecyclePolicy`; delete: `DeleteLifecyclePolicy`; list: `ListLifecyclePolicies` | - | - |
| `SecurityConfig` | `id` | create: `CreateSecurityConfig`; read: `GetSecurityConfig`; update: `UpdateSecurityConfig`; delete: `DeleteSecurityConfig`; list: `ListSecurityConfigs` | - | - |
| `SecurityPolicy` | `name`, `type` | read: `GetSecurityPolicy`; update: `UpdateSecurityPolicy`; delete: `DeleteSecurityPolicy`; list: `ListSecurityPolicies` | - | - |
| `VpcEndpoint` | `id` | create: `CreateVpcEndpoint`; delete: `DeleteVpcEndpoint`; list: `ListVpcEndpoints` | - | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/opensearch-service/latest/developerguide/serverless-security.html
- https://docs.aws.amazon.com/opensearch-service/latest/developerguide/serverless-lifecycle.html

Research outcomes:
- OpenSearch Serverless organises search workloads into collections rather than provisioned domains.
- Security is controlled through encryption policies, network policies, and data access policies.
- Encryption policies define which AWS KMS key protects matching collections.
- Network policies control public and VPC endpoint access to collection and Dashboards endpoints.
- Data access policies grant principals permissions for collections and indexes.
- IAM and SAML principals can authenticate for access according to configured policies.
- Data lifecycle policies define retention for time-series indexes, with policy precedence rules.

Parity implications:
- Model collections, encryption policies, network policies, access policies, VPC endpoints, lifecycle policies, indexes, and policy precedence separately.
- Collection creation should require matching encryption policy and should derive access from separate policy resources.
- Data access should evaluate both IAM authentication and serverless data access policies.

## Current Network Resource Stub Semantics

OpenSearch Serverless currently has one implemented VPC endpoint create path and several unimplemented endpoint operations.

- `CreateVpcEndpoint` requires a name, VPC ID, subnet IDs, and optional security group IDs, then stores them in `OpenSearchServerlessState.vpc_endpoints` with a generated `vpce-` ID.
- Duplicate endpoint names are rejected by the OpenSearch Serverless state map.
- `BatchGetVpcEndpoint`, `ListVpcEndpoints`, `UpdateVpcEndpoint`, and `DeleteVpcEndpoint` currently return not-implemented errors despite the stored endpoint map.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Batch

- Operations: `BatchGetCollection`, `BatchGetCollectionGroup`, `BatchGetEffectiveLifecyclePolicy`, `BatchGetLifecyclePolicy`, `BatchGetVpcEndpoint`
- Traits: `readonly` (5)
- Common required input members in this group: -

### Create

- Operations: `CreateLifecyclePolicy`, `CreateSecurityPolicy`
- Traits: `idempotent` (2), `idempotency-token` (2)
- Common required input members in this group: `type`, `name`, `policy`

### Get

- Operations: `GetAccountSettings`, `GetPoliciesStats`
- Traits: `readonly` (2)
- Common required input members in this group: -

### Update

- Operations: `UpdateAccountSettings`, `UpdateVpcEndpoint`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### List

- Operations: `ListTagsForResource`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchGetCollection` | `-` | `readonly` | - | - | `BatchGetCollectionResponse` | `InternalServerException`, `ValidationException` | Returns attributes for one or more collections, including the collection endpoint, the OpenSearch Dashboards endpoint, and FIPS-compliant endpoints. For more information, see Creating and managing Amazon OpenSearch S ... |
| `BatchGetCollectionGroup` | `-` | `readonly` | - | - | `BatchGetCollectionGroupResponse` | `InternalServerException`, `ValidationException` | Returns attributes for one or more collection groups, including capacity limits and the number of collections in each group. For more information, see Creating and managing Amazon OpenSearch Serverless collections . |
| `BatchGetEffectiveLifecyclePolicy` | `-` | `readonly` | `resourceIdentifiers` | - | `BatchGetEffectiveLifecyclePolicyResponse` | `InternalServerException`, `ValidationException` | Returns a list of successful and failed retrievals for the OpenSearch Serverless indexes. For more information, see Viewing data lifecycle policies . |
| `BatchGetLifecyclePolicy` | `-` | `readonly` | `identifiers` | - | `BatchGetLifecyclePolicyResponse` | `InternalServerException`, `ValidationException` | Returns one or more configured OpenSearch Serverless lifecycle policies. For more information, see Viewing data lifecycle policies . |
| `BatchGetVpcEndpoint` | `-` | `readonly` | `ids` | - | `BatchGetVpcEndpointResponse` | `InternalServerException`, `ValidationException` | Returns attributes for one or more VPC endpoints associated with the current account. For more information, see Access Amazon OpenSearch Serverless using an interface endpoint . |
| `CreateLifecyclePolicy` | `-` | `idempotent`, `idempotency-token` | `type`, `name`, `policy` | `clientToken` | `CreateLifecyclePolicyResponse` | `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a lifecyle policy to be applied to OpenSearch Serverless indexes. Lifecycle policies define the number of days or hours to retain the data on an OpenSearch Serverless index. For more information, see Creating ... |
| `CreateSecurityPolicy` | `-` | `idempotent`, `idempotency-token` | `type`, `name`, `policy` | `clientToken` | `CreateSecurityPolicyResponse` | `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a security policy to be used by one or more OpenSearch Serverless collections. Security policies provide access to a collection and its OpenSearch Dashboards endpoint from public networks or specific VPC endp ... |
| `GetAccountSettings` | `-` | `readonly` | - | - | `GetAccountSettingsResponse` | `InternalServerException`, `ValidationException` | Returns account-level settings related to OpenSearch Serverless. |
| `GetPoliciesStats` | `-` | `readonly` | - | - | `GetPoliciesStatsResponse` | `InternalServerException` | Returns statistical information about your OpenSearch Serverless access policies, security configurations, and security policies. |
| `ListTagsForResource` | `-` | - | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns the tags for an OpenSearch Serverless resource. For more information, see Tagging Amazon OpenSearch Serverless collections . |
| `TagResource` | `-` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Associates tags with an OpenSearch Serverless resource. For more information, see Tagging Amazon OpenSearch Serverless collections . |
| `UntagResource` | `-` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes a tag or set of tags from an OpenSearch Serverless resource. For more information, see Tagging Amazon OpenSearch Serverless collections . |
| `UpdateAccountSettings` | `-` | - | - | - | `UpdateAccountSettingsResponse` | `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | Update the OpenSearch Serverless settings for the current Amazon Web Services account. For more information, see Managing capacity limits for Amazon OpenSearch Serverless . |
| `UpdateVpcEndpoint` | `-` | `idempotent`, `idempotency-token` | `id` | `clientToken` | `UpdateVpcEndpointResponse` | `ConflictException`, `InternalServerException`, `ValidationException` | Updates an OpenSearch Serverless-managed interface endpoint. For more information, see Access Amazon OpenSearch Serverless using an interface endpoint . |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ConflictException` | `structure` | message | When creating a resource, thrown when a resource with the same name already exists or is being created. When deleting a resource, thrown when the resource i ... |
| `InternalServerException` | `structure` | message | Thrown when an error internal to the service occurs while processing a request. |
| `OcuLimitExceededException` | `structure` | message | Thrown when the collection you're attempting to create results in a number of search or indexing OCUs that exceeds the account limit. |
| `ResourceNotFoundException` | `structure` | message | Thrown when accessing or deleting a resource that does not exist. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, serviceCode, quotaCode | Thrown when you attempt to create more resources than the service allows based on service quotas. |
| `ValidationException` | `structure` | message | Thrown when the HTTP request contains invalid input or is missing required input. |
| `BatchGetCollectionRequest` | `structure` | ids, names | - |
| `BatchGetCollectionResponse` | `structure` | collectionDetails, collectionErrorDetails | - |
| `BatchGetCollectionGroupRequest` | `structure` | ids, names | - |
| `BatchGetCollectionGroupResponse` | `structure` | collectionGroupDetails, collectionGroupErrorDetails | - |
| `BatchGetEffectiveLifecyclePolicyRequest` | `structure` | resourceIdentifiers | - |
| `BatchGetEffectiveLifecyclePolicyResponse` | `structure` | effectiveLifecyclePolicyDetails, effectiveLifecyclePolicyErrorDetails | - |
| `BatchGetLifecyclePolicyRequest` | `structure` | identifiers | - |
| `BatchGetLifecyclePolicyResponse` | `structure` | lifecyclePolicyDetails, lifecyclePolicyErrorDetails | - |
| `BatchGetVpcEndpointRequest` | `structure` | ids | - |
| `BatchGetVpcEndpointResponse` | `structure` | vpcEndpointDetails, vpcEndpointErrorDetails | - |
| `CreateLifecyclePolicyRequest` | `structure` | type, name, description, policy, clientToken | - |
| `CreateLifecyclePolicyResponse` | `structure` | lifecyclePolicyDetail | - |
| `CreateSecurityPolicyRequest` | `structure` | type, name, description, policy, clientToken | - |
| `CreateSecurityPolicyResponse` | `structure` | securityPolicyDetail | - |
| `GetAccountSettingsRequest` | `structure` | **empty (no members)** | - |
| `GetAccountSettingsResponse` | `structure` | accountSettingsDetail | - |
| `GetPoliciesStatsRequest` | `structure` | **empty (no members)** | - |
| `GetPoliciesStatsResponse` | `structure` | AccessPolicyStats, SecurityPolicyStats, SecurityConfigStats, LifecyclePolicyStats, TotalPolicyCount | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `UpdateAccountSettingsRequest` | `structure` | capacityLimits | - |
| `UpdateAccountSettingsResponse` | `structure` | accountSettingsDetail | - |
| `UpdateVpcEndpointRequest` | `structure` | id, addSubnetIds, removeSubnetIds, addSecurityGroupIds, removeSecurityGroupIds, clientToken | - |
| `UpdateVpcEndpointResponse` | `structure` | UpdateVpcEndpointDetail | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
