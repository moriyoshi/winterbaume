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

### Get

- Operations: `GetIdentitySource`, `GetPolicy`, `GetPolicyStore`, `GetPolicyTemplate`, `GetSchema`
- Traits: `readonly` (5)
- Common required input members in this group: `identitySourceId`, `policyId`, `policyStoreId`, `policyTemplateId`

### List

- Operations: `ListIdentitySources`, `ListPolicies`, `ListPolicyStores`, `ListPolicyTemplates`, `ListTagsForResource`
- Traits: `paginated` (4), `readonly` (5)
- Common required input members in this group: `policyStoreId`, `resourceArn`

### Create

- Operations: `CreateIdentitySource`, `CreatePolicy`, `CreatePolicyStore`, `CreatePolicyTemplate`
- Traits: `idempotency-token` (4), `idempotent` (4)
- Common required input members in this group: `configuration`, `definition`, `policyStoreId`, `statement`, `validationSettings`

### Delete

- Operations: `DeleteIdentitySource`, `DeletePolicy`, `DeletePolicyStore`, `DeletePolicyTemplate`
- Traits: `idempotent` (4)
- Common required input members in this group: `identitySourceId`, `policyId`, `policyStoreId`, `policyTemplateId`

### Update

- Operations: `UpdateIdentitySource`, `UpdatePolicy`, `UpdatePolicyStore`, `UpdatePolicyTemplate`
- Traits: `idempotent` (4)
- Common required input members in this group: `identitySourceId`, `policyId`, `policyStoreId`, `policyTemplateId`, `statement`, `updateConfiguration`, `validationSettings`

### Batch

- Operations: `BatchGetPolicy`, `BatchIsAuthorized`, `BatchIsAuthorizedWithToken`
- Traits: `readonly` (3)
- Common required input members in this group: `policyStoreId`, `requests`

### Is

- Operations: `IsAuthorized`, `IsAuthorizedWithToken`
- Traits: `readonly` (2)
- Common required input members in this group: `policyStoreId`

### Put

- Operations: `PutSchema`
- Traits: `idempotent` (1)
- Common required input members in this group: `definition`, `policyStoreId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchGetPolicy` | - | `readonly` | `requests` | - | `BatchGetPolicyOutput` | - | Retrieves information about a group (batch) of policies. The `BatchGetPolicy` operation doesn't have its own IAM permission. |
| `BatchIsAuthorized` | - | `readonly` | `policyStoreId`, `requests` | - | `BatchIsAuthorizedOutput` | `ResourceNotFoundException` | Makes a series of decisions about multiple authorization requests for one principal or resource. Each request contains the equivalent content of an `IsAuthorized` request: principal, action, resource, and context. |
| `BatchIsAuthorizedWithToken` | - | `readonly` | `policyStoreId`, `requests` | - | `BatchIsAuthorizedWithTokenOutput` | `ResourceNotFoundException` | Makes a series of decisions about multiple authorization requests for one token. The principal in this request comes from an external identity source in the form of an identity or access token, formatted as a JSON web token (JWT). |
| `CreateIdentitySource` | - | `idempotent`, `idempotency-token` | `configuration`, `policyStoreId` | `clientToken` | `CreateIdentitySourceOutput` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | Adds an identity source to a policy store–an Amazon Cognito user pool or OpenID Connect (OIDC) identity provider (IdP). After you create an identity source, you can use the identities provided by the IdP as proxies for the principal in authorization queries... |
| `CreatePolicy` | - | `idempotent`, `idempotency-token` | `definition`, `policyStoreId` | `clientToken` | `CreatePolicyOutput` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | Creates a Cedar policy and saves it in the specified policy store. You can create either a static policy or a policy linked to a policy template. |
| `CreatePolicyStore` | - | `idempotent`, `idempotency-token` | `validationSettings` | `clientToken` | `CreatePolicyStoreOutput` | `ConflictException`, `ServiceQuotaExceededException` | Creates a policy store. A policy store is a container for policy resources. |
| `CreatePolicyTemplate` | - | `idempotent`, `idempotency-token` | `policyStoreId`, `statement` | `clientToken` | `CreatePolicyTemplateOutput` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | Creates a policy template. A template can use placeholders for the principal and resource. |
| `DeleteIdentitySource` | - | `idempotent` | `identitySourceId`, `policyStoreId` | - | `DeleteIdentitySourceOutput` | `ConflictException`, `ResourceNotFoundException` | Deletes an identity source that references an identity provider (IdP) such as Amazon Cognito. After you delete the identity source, you can no longer use tokens for identities from that identity source to represent principals in authorization queries made... |
| `DeletePolicy` | - | `idempotent` | `policyId`, `policyStoreId` | - | `DeletePolicyOutput` | `ConflictException`, `ResourceNotFoundException` | Deletes the specified policy from the policy store. This operation is idempotent; if you specify a policy that doesn't exist, the request response returns a successful `HTTP 200` status code. |
| `DeletePolicyStore` | - | `idempotent` | `policyStoreId` | - | `DeletePolicyStoreOutput` | `InvalidStateException` | Deletes the specified policy store. This operation is idempotent. |
| `DeletePolicyTemplate` | - | `idempotent` | `policyStoreId`, `policyTemplateId` | - | `DeletePolicyTemplateOutput` | `ConflictException`, `ResourceNotFoundException` | Deletes the specified policy template from the policy store. This operation also deletes any policies that were created from the specified policy template. |
| `GetIdentitySource` | - | `readonly` | `identitySourceId`, `policyStoreId` | - | `GetIdentitySourceOutput` | `ResourceNotFoundException` | Retrieves the details about the specified identity source. |
| `GetPolicy` | - | `readonly` | `policyId`, `policyStoreId` | - | `GetPolicyOutput` | `ResourceNotFoundException` | Retrieves information about the specified policy. |
| `GetPolicyStore` | - | `readonly` | `policyStoreId` | - | `GetPolicyStoreOutput` | `ResourceNotFoundException` | Retrieves details about a policy store. |
| `GetPolicyTemplate` | - | `readonly` | `policyStoreId`, `policyTemplateId` | - | `GetPolicyTemplateOutput` | `ResourceNotFoundException` | Retrieve the details for the specified policy template in the specified policy store. |
| `GetSchema` | - | `readonly` | `policyStoreId` | - | `GetSchemaOutput` | `ResourceNotFoundException` | Retrieve the details for the specified schema in the specified policy store. |
| `IsAuthorized` | - | `readonly` | `policyStoreId` | - | `IsAuthorizedOutput` | `ResourceNotFoundException` | Makes an authorization decision about a service request described in the parameters. The information in the parameters can also define additional context that Verified Permissions can include in the evaluation. |
| `IsAuthorizedWithToken` | - | `readonly` | `policyStoreId` | - | `IsAuthorizedWithTokenOutput` | `ResourceNotFoundException` | Makes an authorization decision about a service request described in the parameters. The principal in this request comes from an external identity source in the form of an identity token formatted as a JSON web token (JWT). |
| `ListIdentitySources` | - | `readonly`, `paginated` | `policyStoreId` | - | `ListIdentitySourcesOutput` | `ResourceNotFoundException` | Returns a paginated list of all of the identity sources defined in the specified policy store. |
| `ListPolicies` | - | `readonly`, `paginated` | `policyStoreId` | - | `ListPoliciesOutput` | `ResourceNotFoundException` | Returns a paginated list of all policies stored in the specified policy store. |
| `ListPolicyStores` | - | `readonly`, `paginated` | - | - | `ListPolicyStoresOutput` | - | Returns a paginated list of all policy stores in the calling Amazon Web Services account. |
| `ListPolicyTemplates` | - | `readonly`, `paginated` | `policyStoreId` | - | `ListPolicyTemplatesOutput` | `ResourceNotFoundException` | Returns a paginated list of all policy templates in the specified policy store. |
| `ListTagsForResource` | - | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Returns the tags associated with the specified Amazon Verified Permissions resource. In Verified Permissions, policy stores can be tagged. |
| `PutSchema` | - | `idempotent` | `definition`, `policyStoreId` | - | `PutSchemaOutput` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | Creates or updates the policy schema in the specified policy store. The schema is used to validate any Cedar policies and policy templates submitted to the policy store. |
| `TagResource` | - | - | `resourceArn`, `tags` | - | `TagResourceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `TooManyTagsException` | Assigns one or more tags (key-value pairs) to the specified Amazon Verified Permissions resource. Tags can help you organize and categorize your resources. |
| `UntagResource` | - | - | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Removes one or more tags from the specified Amazon Verified Permissions resource. In Verified Permissions, policy stores can be tagged. |
| `UpdateIdentitySource` | - | `idempotent` | `identitySourceId`, `policyStoreId`, `updateConfiguration` | - | `UpdateIdentitySourceOutput` | `ConflictException`, `ResourceNotFoundException` | Updates the specified identity source to use a new identity provider (IdP), or to change the mapping of identities from the IdP to a different principal entity type. Verified Permissions is eventually consistent . |
| `UpdatePolicy` | - | `idempotent` | `policyId`, `policyStoreId` | - | `UpdatePolicyOutput` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | Modifies a Cedar static policy in the specified policy store. You can change only certain elements of the UpdatePolicyDefinition parameter. |
| `UpdatePolicyStore` | - | `idempotent` | `policyStoreId`, `validationSettings` | - | `UpdatePolicyStoreOutput` | `ConflictException`, `ResourceNotFoundException` | Modifies the validation setting for a policy store. Verified Permissions is eventually consistent . |
| `UpdatePolicyTemplate` | - | `idempotent` | `policyStoreId`, `policyTemplateId`, `statement` | - | `UpdatePolicyTemplateOutput` | `ConflictException`, `ResourceNotFoundException` | Updates the specified policy template. You can update only the description and the some elements of the policyBody. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The request failed because it references a resource that doesn't exist. |
| `ConflictException` | `structure` | `message`, `resources` | The request failed because another request to modify a resource occurred at the same. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | The request failed because it would cause a service quota to be exceeded. |
| `AccessDeniedException` | `structure` | `message` | You don't have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message` | The request failed because of an internal error. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `serviceCode` | The request failed because it exceeded a throttling quota. |
| `BatchGetPolicyInput` | `structure` | `requests` | - |
| `BatchGetPolicyOutput` | `structure` | `errors`, `results` | - |
| `BatchIsAuthorizedInput` | `structure` | `entities`, `policyStoreId`, `requests` | - |
| `BatchIsAuthorizedOutput` | `structure` | `results` | - |
| `BatchIsAuthorizedWithTokenInput` | `structure` | `accessToken`, `entities`, `identityToken`, `policyStoreId`, `requests` | - |
| `BatchIsAuthorizedWithTokenOutput` | `structure` | `principal`, `results` | - |
| `CreateIdentitySourceInput` | `structure` | `clientToken`, `configuration`, `policyStoreId`, `principalEntityType` | - |
| `CreateIdentitySourceOutput` | `structure` | `createdDate`, `identitySourceId`, `lastUpdatedDate`, `policyStoreId` | - |
| `CreatePolicyInput` | `structure` | `clientToken`, `definition`, `policyStoreId` | - |
| `CreatePolicyOutput` | `structure` | `actions`, `createdDate`, `effect`, `lastUpdatedDate`, `policyId`, `policyStoreId`, `policyType`, `principal`, `resource` | - |
| `CreatePolicyStoreInput` | `structure` | `clientToken`, `deletionProtection`, `description`, `encryptionSettings`, `tags`, `validationSettings` | - |
| `CreatePolicyStoreOutput` | `structure` | `arn`, `createdDate`, `lastUpdatedDate`, `policyStoreId` | - |
| `CreatePolicyTemplateInput` | `structure` | `clientToken`, `description`, `policyStoreId`, `statement` | - |
| `CreatePolicyTemplateOutput` | `structure` | `createdDate`, `lastUpdatedDate`, `policyStoreId`, `policyTemplateId` | - |
| `DeleteIdentitySourceInput` | `structure` | `identitySourceId`, `policyStoreId` | - |
| `DeleteIdentitySourceOutput` | `structure` | - | - |
| `DeletePolicyInput` | `structure` | `policyId`, `policyStoreId` | - |
| `DeletePolicyOutput` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
