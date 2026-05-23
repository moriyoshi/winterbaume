# Amazon Verified Permissions

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Verified Permissions is a permissions management service from Amazon Web Services. You can use Verified Permissions to manage permissions for your application, and authorize user access based on those permissions. Using Verified Permissions, application developers can grant access based on information about the users, resources, and requested actions. You can also evaluate additional information like group membership, attributes of the resources, and session context, such as time of request and IP addresses. Verified Permissions manages these permissions by letting you create and store authorization policies for your applications, such as consumer-facing web sites and enterprise business systems.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Verified Permissions workflows in the local mock. Key resources include `IdentitySource`, `Policy`, `PolicyStore`, `PolicyTemplate`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Update` operation families, including `GetIdentitySource`, `GetPolicy`, `GetPolicyStore`, `GetPolicyTemplate`, `ListIdentitySources`, `ListPolicies`.

## Service Identity and Protocol

- AWS model slug: `verifiedpermissions`
- AWS SDK for Rust slug: `verifiedpermissions`
- Model version: `2021-12-01`
- Model file: `vendor/api-models-aws/models/verifiedpermissions/service/2021-12-01/verifiedpermissions-2021-12-01.json`
- SDK ID: `VerifiedPermissions`
- Endpoint prefix: `verifiedpermissions`
- ARN namespace: `verifiedpermissions`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (5), `List` (5), `Create` (4), `Delete` (4), `Update` (4), `Batch` (3), `Is` (2), `Put` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchGetPolicy`, `BatchIsAuthorized`, `BatchIsAuthorizedWithToken`, `CreateIdentitySource`, `CreatePolicy`, `CreatePolicyStore`, `CreatePolicyTemplate`, `DeleteIdentitySource`, `DeletePolicy`, `DeletePolicyStore`, `DeletePolicyTemplate`, `PutSchema`, `TagResource`, `UntagResource`, `UpdateIdentitySource`, `UpdatePolicy`, `UpdatePolicyStore`, `UpdatePolicyTemplate`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `BatchGetPolicy`, `BatchIsAuthorized`, `BatchIsAuthorizedWithToken`, `GetIdentitySource`, `GetPolicy`, `GetPolicyStore`, `GetPolicyTemplate`, `GetSchema`, `IsAuthorized`, `IsAuthorizedWithToken`, `ListIdentitySources`, `ListPolicies`, `ListPolicyStores`, `ListPolicyTemplates`, `ListTagsForResource`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 13 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 28 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `EC2/VPC`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `IdentitySource` | `identitySourceId`, `policyStoreId` | create: `CreateIdentitySource`; read: `GetIdentitySource`; update: `UpdateIdentitySource`; delete: `DeleteIdentitySource`; list: `ListIdentitySources` | - | - |
| `Policy` | `policyId`, `policyStoreId` | create: `CreatePolicy`; read: `GetPolicy`; update: `UpdatePolicy`; delete: `DeletePolicy`; list: `ListPolicies` | - | - |
| `PolicyStore` | `policyStoreId` | create: `CreatePolicyStore`; read: `GetPolicyStore`; update: `UpdatePolicyStore`; delete: `DeletePolicyStore`; list: `ListPolicyStores` | `BatchGetPolicy`, `BatchIsAuthorized`, `BatchIsAuthorizedWithToken`, `GetSchema`, `IsAuthorized`, `IsAuthorizedWithToken`, `PutSchema` | Represents a policy store that you can place schema, policies, and policy templates in to validate authorization requests |
| `PolicyTemplate` | `policyStoreId`, `policyTemplateId` | create: `CreatePolicyTemplate`; read: `GetPolicyTemplate`; update: `UpdatePolicyTemplate`; delete: `DeletePolicyTemplate`; list: `ListPolicyTemplates` | - | - |
## Operation Groups

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
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
| `ListTagsForResource` | `-` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the tags associated with the specified Amazon Verified Permissions resource. In Verified Permissions, policy stores can be tagged. |
| `TagResource` | `-` | - | `resourceArn`, `tags` | - | `TagResourceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `TooManyTagsException` | Assigns one or more tags (key-value pairs) to the specified Amazon Verified Permissions resource. Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting ... |
| `UntagResource` | `-` | - | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Removes one or more tags from the specified Amazon Verified Permissions resource. In Verified Permissions, policy stores can be tagged. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have sufficient access to perform this action. |
| `ConflictException` | `structure` | message, resources | The request failed because another request to modify a resource occurred at the same time. |
| `InternalServerException` | `structure` | message | The request failed because of an internal error. Try your request again later |
| `InvalidStateException` | `structure` | message | The policy store can't be deleted because deletion protection is enabled. To delete this policy store, disable deletion protection. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | The request failed because it references a resource that doesn't exist. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, serviceCode, quotaCode | The request failed because it would cause a service quota to be exceeded. |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode | The request failed because it exceeded a throttling quota. |
| `TooManyTagsException` | `structure` | message, resourceName | No more tags be added because the limit (50) has been reached. To add new tags, use UntagResource to remove existing tags. |
| `ValidationException` | `structure` | message, fieldList | The request failed because one or more input parameters don't satisfy their constraint requirements. The output is provided as a list of fields and a reason ... |
| `ListTagsForResourceInput` | `structure` | resourceArn | - |
| `ListTagsForResourceOutput` | `structure` | tags | - |
| `TagResourceInput` | `structure` | resourceArn, tags | - |
| `TagResourceOutput` | `structure` | **empty (no members)** | - |
| `UntagResourceInput` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceOutput` | `structure` | **empty (no members)** | - |
| `AliasState` | `enum` | ACTIVE, PENDING_DELETION | - |
| `BatchGetPolicyErrorCode` | `enum` | POLICY_STORE_NOT_FOUND, POLICY_NOT_FOUND, POLICY_STORE_ALIAS_NOT_FOUND | - |
| `CedarVersion` | `enum` | CEDAR_2, CEDAR_4 | - |
| `Decision` | `enum` | ALLOW, DENY | - |
| `DeletionProtection` | `enum` | ENABLED, DISABLED | - |
| `OpenIdIssuer` | `enum` | COGNITO | - |
| `PolicyEffect` | `enum` | PERMIT, FORBID | - |
| `PolicyType` | `enum` | STATIC, TEMPLATE_LINKED | - |
| `ResourceType` | `enum` | IDENTITY_SOURCE, POLICY_STORE, POLICY, POLICY_TEMPLATE, SCHEMA, POLICY_STORE_ALIAS | - |
| `ValidationMode` | `enum` | OFF, STRICT | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
