# Amazon Managed Grafana

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Managed Grafana is a fully managed and secure data visualization service that you can use to instantly query, correlate, and visualize operational metrics, logs, and traces from multiple sources. Amazon Managed Grafana makes it easy to deploy, operate, and scale Grafana, a widely deployed data visualization tool that is popular for its extensible data support. With Amazon Managed Grafana, you create logically isolated Grafana servers called workspaces . In a workspace, you can create Grafana dashboards and visualizations to analyze your metrics, logs, and traces without having to build, package, or deploy any hardware to run Grafana servers.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Managed Grafana where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon Managed Grafana by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon Managed Grafana workflows in the local mock. Key resources include `ApiKey`, `Authentication`, `Configuration`, `License`, `Permission`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Delete`, `Update`, `Describe` operation families, including `ListPermissions`, `ListTagsForResource`, `ListVersions`, `ListWorkspaceServiceAccountTokens`, `CreateWorkspace`, `CreateWorkspaceApiKey`.

## Service Identity and Protocol

- AWS model slug: `grafana`
- AWS SDK for Rust slug: `grafana`
- Model version: `2020-08-18`
- Model file: `vendor/api-models-aws/models/grafana/service/2020-08-18/grafana-2020-08-18.json`
- SDK ID: `grafana`
- Endpoint prefix: `-`
- ARN namespace: `grafana`
- CloudFormation name: `Grafana`
- CloudTrail event source: `grafana.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (6), `Create` (4), `Delete` (4), `Update` (4), `Describe` (3), `Associate` (1), `Disassociate` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateLicense`, `CreateWorkspace`, `CreateWorkspaceApiKey`, `CreateWorkspaceServiceAccount`, `CreateWorkspaceServiceAccountToken`, `DeleteWorkspace`, `DeleteWorkspaceApiKey`, `DeleteWorkspaceServiceAccount`, `DeleteWorkspaceServiceAccountToken`, `DisassociateLicense`, `TagResource`, `UntagResource`, `UpdatePermissions`, `UpdateWorkspace`, `UpdateWorkspaceAuthentication`, `UpdateWorkspaceConfiguration`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeWorkspace`, `DescribeWorkspaceAuthentication`, `DescribeWorkspaceConfiguration`, `ListPermissions`, `ListTagsForResource`, `ListVersions`, `ListWorkspaceServiceAccountTokens`, `ListWorkspaceServiceAccounts`, `ListWorkspaces`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 3 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 25 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ApiKey` | `workspaceId` | - | `CreateWorkspaceApiKey`, `DeleteWorkspaceApiKey` | - |
| `Authentication` | `workspaceId` | read: `DescribeWorkspaceAuthentication`; update: `UpdateWorkspaceAuthentication` | - | - |
| `Configuration` | `workspaceId` | - | `DescribeWorkspaceConfiguration`, `UpdateWorkspaceConfiguration` | - |
| `License` | `licenseType`, `workspaceId` | - | `AssociateLicense`, `DisassociateLicense` | - |
| `Permission` | `workspaceId` | read: `ListPermissions`; update: `UpdatePermissions` | - | - |
| `ServiceAccount` | `workspaceId` | - | `CreateWorkspaceServiceAccount`, `DeleteWorkspaceServiceAccount`, `ListWorkspaceServiceAccounts` | - |
| `ServiceAccountToken` | `serviceAccountId`, `workspaceId` | - | `CreateWorkspaceServiceAccountToken`, `DeleteWorkspaceServiceAccountToken`, `ListWorkspaceServiceAccountTokens` | - |
| `Workspace` | `workspaceId` | create: `CreateWorkspace`; read: `DescribeWorkspace`; update: `UpdateWorkspace`; delete: `DeleteWorkspace`; list: `ListWorkspaces` | - | - |
## Operation Groups

### List

- Operations: `ListPermissions`, `ListTagsForResource`, `ListVersions`, `ListWorkspaceServiceAccountTokens`, `ListWorkspaceServiceAccounts`, `ListWorkspaces`
- Traits: `paginated` (5), `readonly` (6)
- Common required input members in this group: `resourceArn`, `serviceAccountId`, `workspaceId`

### Create

- Operations: `CreateWorkspace`, `CreateWorkspaceApiKey`, `CreateWorkspaceServiceAccount`, `CreateWorkspaceServiceAccountToken`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `accountAccessType`, `authenticationProviders`, `grafanaRole`, `keyName`, `keyRole`, `name`, `permissionType`, `secondsToLive`, `serviceAccountId`, `workspaceId`

### Delete

- Operations: `DeleteWorkspace`, `DeleteWorkspaceApiKey`, `DeleteWorkspaceServiceAccount`, `DeleteWorkspaceServiceAccountToken`
- Traits: `idempotent` (1)
- Common required input members in this group: `keyName`, `serviceAccountId`, `tokenId`, `workspaceId`

### Update

- Operations: `UpdatePermissions`, `UpdateWorkspace`, `UpdateWorkspaceAuthentication`, `UpdateWorkspaceConfiguration`
- Common required input members in this group: `authenticationProviders`, `configuration`, `updateInstructionBatch`, `workspaceId`

### Describe

- Operations: `DescribeWorkspace`, `DescribeWorkspaceAuthentication`, `DescribeWorkspaceConfiguration`
- Traits: `readonly` (2)
- Common required input members in this group: `workspaceId`

### Associate

- Operations: `AssociateLicense`
- Common required input members in this group: `licenseType`, `workspaceId`

### Disassociate

- Operations: `DisassociateLicense`
- Common required input members in this group: `licenseType`, `workspaceId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateLicense` | `POST /workspaces/{workspaceId}/licenses/{licenseType}` | - | `licenseType`, `workspaceId` | - | `AssociateLicenseResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Assigns a Grafana Enterprise license to a workspace. To upgrade, you must use `ENTERPRISE` for the `licenseType`, and pass in a valid Grafana Labs token for the `grafanaToken`. |
| `CreateWorkspace` | `POST /workspaces` | `idempotent`, `idempotency-token` | `accountAccessType`, `authenticationProviders`, `permissionType` | `clientToken` | `CreateWorkspaceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a workspace . In a workspace, you can create Grafana dashboards and visualizations to analyze your metrics, logs, and traces. |
| `CreateWorkspaceApiKey` | `POST /workspaces/{workspaceId}/apikeys` | - | `keyName`, `keyRole`, `secondsToLive`, `workspaceId` | - | `CreateWorkspaceApiKeyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a Grafana API key for the workspace. This key can be used to authenticate requests sent to the workspace's HTTP API. |
| `CreateWorkspaceServiceAccount` | `POST /workspaces/{workspaceId}/serviceaccounts` | - | `grafanaRole`, `name`, `workspaceId` | - | `CreateWorkspaceServiceAccountResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a service account for the workspace. A service account can be used to call Grafana HTTP APIs, and run automated workloads. |
| `CreateWorkspaceServiceAccountToken` | `POST /workspaces/{workspaceId}/serviceaccounts/{serviceAccountId}/tokens` | - | `name`, `secondsToLive`, `serviceAccountId`, `workspaceId` | - | `CreateWorkspaceServiceAccountTokenResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a token that can be used to authenticate and authorize Grafana HTTP API operations for the given workspace service account. The service account acts as a user for the API operations, and defines the permissions that are used by the API. |
| `DeleteWorkspace` | `DELETE /workspaces/{workspaceId}` | `idempotent` | `workspaceId` | - | `DeleteWorkspaceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an Amazon Managed Grafana workspace. |
| `DeleteWorkspaceApiKey` | `DELETE /workspaces/{workspaceId}/apikeys/{keyName}` | - | `keyName`, `workspaceId` | - | `DeleteWorkspaceApiKeyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a Grafana API key for the workspace. In workspaces compatible with Grafana version 9 or above, use workspace service accounts instead of API keys. |
| `DeleteWorkspaceServiceAccount` | `DELETE /workspaces/{workspaceId}/serviceaccounts/{serviceAccountId}` | - | `serviceAccountId`, `workspaceId` | - | `DeleteWorkspaceServiceAccountResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a workspace service account from the workspace. This will delete any tokens created for the service account, as well. |
| `DeleteWorkspaceServiceAccountToken` | `DELETE /workspaces/{workspaceId}/serviceaccounts/{serviceAccountId}/tokens/{tokenId}` | - | `serviceAccountId`, `tokenId`, `workspaceId` | - | `DeleteWorkspaceServiceAccountTokenResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a token for the workspace service account. This will disable the key associated with the token. |
| `DescribeWorkspace` | `GET /workspaces/{workspaceId}` | `readonly` | `workspaceId` | - | `DescribeWorkspaceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Displays information about one Amazon Managed Grafana workspace. |
| `DescribeWorkspaceAuthentication` | `GET /workspaces/{workspaceId}/authentication` | `readonly` | `workspaceId` | - | `DescribeWorkspaceAuthenticationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Displays information about the authentication methods used in one Amazon Managed Grafana workspace. |
| `DescribeWorkspaceConfiguration` | `GET /workspaces/{workspaceId}/configuration` | - | `workspaceId` | - | `DescribeWorkspaceConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Gets the current configuration string for the given workspace. |
| `DisassociateLicense` | `DELETE /workspaces/{workspaceId}/licenses/{licenseType}` | - | `licenseType`, `workspaceId` | - | `DisassociateLicenseResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the Grafana Enterprise license from a workspace. |
| `ListPermissions` | `GET /workspaces/{workspaceId}/permissions` | `readonly`, `paginated` | `workspaceId` | - | `ListPermissionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the users and groups who have the Grafana `Admin` and `Editor` roles in this workspace. If you use this operation without specifying `userId` or `groupId`, the operation returns the roles of all users and groups. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The `ListTagsForResource` operation returns the tags that are associated with the Amazon Managed Service for Grafana resource specified by the `resourceArn`. Currently, the only resource that can be tagged is a workspace. |
| `ListVersions` | `GET /versions` | `readonly`, `paginated` | - | - | `ListVersionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists available versions of Grafana. These are available when calling `CreateWorkspace`. |
| `ListWorkspaceServiceAccountTokens` | `GET /workspaces/{workspaceId}/serviceaccounts/{serviceAccountId}/tokens` | `readonly`, `paginated` | `serviceAccountId`, `workspaceId` | - | `ListWorkspaceServiceAccountTokensResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of tokens for a workspace service account. This does not return the key for each token. |
| `ListWorkspaceServiceAccounts` | `GET /workspaces/{workspaceId}/serviceaccounts` | `readonly`, `paginated` | `workspaceId` | - | `ListWorkspaceServiceAccountsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of service accounts for a workspace. Service accounts are only available for workspaces that are compatible with Grafana version 9 and above. |
| `ListWorkspaces` | `GET /workspaces` | `readonly`, `paginated` | - | - | `ListWorkspacesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException` | Returns a list of Amazon Managed Grafana workspaces in the account, with some information about each workspace. For more complete information about one workspace, use DescribeWorkspace. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The `TagResource` operation associates tags with an Amazon Managed Grafana resource. Currently, the only resource that can be tagged is workspaces. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The `UntagResource` operation removes the association of the tag with the Amazon Managed Grafana resource. |
| `UpdatePermissions` | `PATCH /workspaces/{workspaceId}/permissions` | - | `updateInstructionBatch`, `workspaceId` | - | `UpdatePermissionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates which users in a workspace have the Grafana `Admin` or `Editor` roles. |
| `UpdateWorkspace` | `PUT /workspaces/{workspaceId}` | - | `workspaceId` | - | `UpdateWorkspaceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Modifies an existing Amazon Managed Grafana workspace. If you use this operation and omit any optional parameters, the existing values of those parameters are not changed. |
| `UpdateWorkspaceAuthentication` | `POST /workspaces/{workspaceId}/authentication` | - | `authenticationProviders`, `workspaceId` | - | `UpdateWorkspaceAuthenticationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use this operation to define the identity provider (IdP) that this workspace authenticates users from, using SAML. You can also map SAML assertion attributes to workspace user information and define which groups in the assertion attribute are to have the... |
| `UpdateWorkspaceConfiguration` | `PUT /workspaces/{workspaceId}/configuration` | - | `configuration`, `workspaceId` | - | `UpdateWorkspaceConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the configuration string for the given workspace |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient permissions to perform this action. |
| `InternalServerException` | `structure` | `message`, `retryAfterSeconds` | Unexpected error while processing the request. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | The request was denied because of request throttling. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The request references a resource that does not exist. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | The value of a parameter in the request caused an error. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | A resource was in an inconsistent state during an update or a deletion. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | The request would cause a service quota to be exceeded. |
| `AssociateLicenseRequest` | `structure` | `grafanaToken`, `licenseType`, `workspaceId` | - |
| `AssociateLicenseResponse` | `structure` | `workspace` | - |
| `CreateWorkspaceRequest` | `structure` | `accountAccessType`, `authenticationProviders`, `clientToken`, `configuration`, `grafanaVersion`, `kmsKeyId`, `networkAccessControl`, `organizationRoleName`, `permissionType`, `stackSetName`, `tags`, `vpcConfiguration`, ... (+6) | - |
| `CreateWorkspaceResponse` | `structure` | `workspace` | - |
| `CreateWorkspaceApiKeyRequest` | `structure` | `keyName`, `keyRole`, `secondsToLive`, `workspaceId` | - |
| `CreateWorkspaceApiKeyResponse` | `structure` | `key`, `keyName`, `workspaceId` | - |
| `CreateWorkspaceServiceAccountRequest` | `structure` | `grafanaRole`, `name`, `workspaceId` | - |
| `CreateWorkspaceServiceAccountResponse` | `structure` | `grafanaRole`, `id`, `name`, `workspaceId` | - |
| `CreateWorkspaceServiceAccountTokenRequest` | `structure` | `name`, `secondsToLive`, `serviceAccountId`, `workspaceId` | - |
| `CreateWorkspaceServiceAccountTokenResponse` | `structure` | `serviceAccountId`, `serviceAccountToken`, `workspaceId` | - |
| `DeleteWorkspaceRequest` | `structure` | `workspaceId` | - |
| `DeleteWorkspaceResponse` | `structure` | `workspace` | - |
| `DeleteWorkspaceApiKeyRequest` | `structure` | `keyName`, `workspaceId` | - |
| `DeleteWorkspaceApiKeyResponse` | `structure` | `keyName`, `workspaceId` | - |
| `DeleteWorkspaceServiceAccountRequest` | `structure` | `serviceAccountId`, `workspaceId` | - |
| `DeleteWorkspaceServiceAccountResponse` | `structure` | `serviceAccountId`, `workspaceId` | - |
| `DeleteWorkspaceServiceAccountTokenRequest` | `structure` | `serviceAccountId`, `tokenId`, `workspaceId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
