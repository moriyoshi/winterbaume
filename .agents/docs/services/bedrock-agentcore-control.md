# Amazon Bedrock AgentCore Control

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Welcome to the Amazon Bedrock AgentCore Control plane API reference. Control plane actions configure, create, modify, and monitor Amazon Web Services resources.

## Possible Usage Scenarios
- Scenario insight from EC2: cover association replacement for Amazon Bedrock AgentCore Control by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for Amazon Bedrock AgentCore Control resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: configure AgentCore runtimes, memory, gateways, credential providers, identity resources, and policy engines.
- From the operation surface: model agent runtime deployment, gateway tool exposure, memory/session persistence, policy generation, workload identity, and tag-based control-plane inventory.

## Service Identity and Protocol

- AWS model slug: `bedrock-agentcore-control`
- AWS SDK for Rust slug: `bedrockagentcorecontrol`
- Model version: `2023-06-05`
- Model file: `vendor/api-models-aws/models/bedrock-agentcore-control/service/2023-06-05/bedrock-agentcore-control-2023-06-05.json`
- SDK ID: `Bedrock AgentCore Control`
- Endpoint prefix: `bedrock-agentcore-control`
- ARN namespace: `bedrock-agentcore`
- CloudFormation name: `-`
- CloudTrail event source: `bedrock-agentcore.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (19), `Get` (18), `Delete` (16), `Create` (15), `Update` (12), `Put` (1), `Set` (1), `Start` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateAgentRuntime`, `CreateAgentRuntimeEndpoint`, `CreateApiKeyCredentialProvider`, `CreateBrowser`, `CreateBrowserProfile`, `CreateCodeInterpreter`, `CreateEvaluator`, `CreateGateway`, `CreateGatewayTarget`, `CreateMemory`, `CreateOauth2CredentialProvider`, `CreateOnlineEvaluationConfig`, `CreatePolicy`, `CreatePolicyEngine`, `CreateWorkloadIdentity`, `DeleteAgentRuntime`, `DeleteAgentRuntimeEndpoint`, `DeleteApiKeyCredentialProvider`, `DeleteBrowser`, `DeleteBrowserProfile`, ... (+28).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAgentRuntime`, `GetAgentRuntimeEndpoint`, `GetApiKeyCredentialProvider`, `GetBrowser`, `GetBrowserProfile`, `GetCodeInterpreter`, `GetEvaluator`, `GetGateway`, `GetGatewayTarget`, `GetMemory`, `GetOauth2CredentialProvider`, `GetOnlineEvaluationConfig`, `GetPolicy`, `GetPolicyEngine`, `GetPolicyGeneration`, `GetResourcePolicy`, `GetTokenVault`, `GetWorkloadIdentity`, `ListAgentRuntimeEndpoints`, `ListAgentRuntimeVersions`, ... (+17).
- Pagination is modelled for 18 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 46 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartPolicyGeneration`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 86 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `Lambda`, `Secrets Manager`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AgentEndpointResource` | `agentRuntimeId`, `endpointName` | create: `CreateAgentRuntimeEndpoint`; read: `GetAgentRuntimeEndpoint`; update: `UpdateAgentRuntimeEndpoint`; delete: `DeleteAgentRuntimeEndpoint`; list: `ListAgentRuntimeEndpoints` | - | - |
| `AgentResource` | `agentRuntimeId` | create: `CreateAgentRuntime`; read: `GetAgentRuntime`; update: `UpdateAgentRuntime`; delete: `DeleteAgentRuntime`; list: `ListAgentRuntimes` | `ListAgentRuntimeVersions` | - |
| `ApiKeyCredentialProvider` | `name` | put: `CreateApiKeyCredentialProvider`; read: `GetApiKeyCredentialProvider`; update: `UpdateApiKeyCredentialProvider`; delete: `DeleteApiKeyCredentialProvider` | `ListApiKeyCredentialProviders` | - |
| `BrowserProfileResource` | `profileId` | create: `CreateBrowserProfile`; read: `GetBrowserProfile`; delete: `DeleteBrowserProfile`; list: `ListBrowserProfiles` | - | - |
| `BrowserResource` | `browserId` | create: `CreateBrowser`; read: `GetBrowser`; delete: `DeleteBrowser`; list: `ListBrowsers` | - | - |
| `CodeInterpreterResource` | `codeInterpreterId` | create: `CreateCodeInterpreter`; read: `GetCodeInterpreter`; delete: `DeleteCodeInterpreter`; list: `ListCodeInterpreters` | - | - |
| `Evaluator` | `evaluatorId` | create: `CreateEvaluator`; read: `GetEvaluator`; update: `UpdateEvaluator`; delete: `DeleteEvaluator`; list: `ListEvaluators` | - | - |
| `GatewayResource` | - | - | `CreateGateway`, `DeleteGateway`, `GetGateway`, `ListGateways`, `UpdateGateway` | - |
| `GatewayTargetResource` | - | - | `CreateGatewayTarget`, `DeleteGatewayTarget`, `GetGatewayTarget`, `ListGatewayTargets`, `SynchronizeGatewayTargets`, `UpdateGatewayTarget` | - |
| `MemoryResource` | `memoryId` | create: `CreateMemory`; read: `GetMemory`; update: `UpdateMemory`; delete: `DeleteMemory`; list: `ListMemories` | - | - |
| `Oauth2CredentialProvider` | `name` | put: `CreateOauth2CredentialProvider`; read: `GetOauth2CredentialProvider`; update: `UpdateOauth2CredentialProvider`; delete: `DeleteOauth2CredentialProvider` | `ListOauth2CredentialProviders` | - |
| `OnlineEvaluationConfig` | `onlineEvaluationConfigId` | create: `CreateOnlineEvaluationConfig`; read: `GetOnlineEvaluationConfig`; update: `UpdateOnlineEvaluationConfig`; delete: `DeleteOnlineEvaluationConfig`; list: `ListOnlineEvaluationConfigs` | - | - |
| `PolicyEngineResource` | `policyEngineId` | create: `CreatePolicyEngine`; read: `GetPolicyEngine`; update: `UpdatePolicyEngine`; delete: `DeletePolicyEngine`; list: `ListPolicyEngines` | - | - |
| `PolicyGenerationResource` | `policyGenerationId` | create: `StartPolicyGeneration`; read: `GetPolicyGeneration`; list: `ListPolicyGenerations` | `ListPolicyGenerationAssets` | - |
| `PolicyResource` | `policyId` | create: `CreatePolicy`; read: `GetPolicy`; update: `UpdatePolicy`; delete: `DeletePolicy`; list: `ListPolicies` | - | - |
| `WorkloadIdentity` | `name` | put: `CreateWorkloadIdentity`; read: `GetWorkloadIdentity`; update: `UpdateWorkloadIdentity`; delete: `DeleteWorkloadIdentity` | `ListWorkloadIdentities` | - |
## Operation Groups

### Get

- Operations: `GetResourcePolicy`, `GetTokenVault`
- Traits: `readonly` (2)
- Common required input members in this group: -

### Delete

- Operations: `DeleteResourcePolicy`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Put

- Operations: `PutResourcePolicy`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Set

- Operations: `SetTokenVaultCMK`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DeleteResourcePolicy` | `DELETE /resourcepolicy/{resourceArn}` | `idempotent` | `resourceArn` | - | `DeleteResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the resource-based policy for a specified resource. This feature is currently available only for AgentCore Runtime and Gateway. |
| `GetResourcePolicy` | `GET /resourcepolicy/{resourceArn}` | `readonly` | `resourceArn` | - | `GetResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the resource-based policy for a specified resource. This feature is currently available only for AgentCore Runtime and Gateway. |
| `GetTokenVault` | `POST /identities/get-token-vault` | `readonly` | - | - | `GetTokenVaultResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Retrieves information about a token vault. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags associated with the specified resource. This feature is currently available only for AgentCore Runtime, Browser, Browser Profile, Code Interpreter tool, and Gateway. |
| `PutResourcePolicy` | `PUT /resourcepolicy/{resourceArn}` | `idempotent` | `resourceArn`, `policy` | - | `PutResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates or updates a resource-based policy for a resource with the specified resourceArn. This feature is currently available only for AgentCore Runtime and Gateway. |
| `SetTokenVaultCMK` | `POST /identities/set-token-vault-cmk` | - | `kmsConfiguration` | - | `SetTokenVaultCMKResponse` | `AccessDeniedException`, `ConcurrentModificationException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Sets the customer master key (CMK) for a token vault. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates the specified tags to a resource with the specified resourceArn. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags assoc ... |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the specified tags from the specified resource. This feature is currently available only for AgentCore Runtime, Browser, Browser Profile, Code Interpreter tool, and Gateway. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | This exception is thrown when a request is denied per access permissions |
| `ConcurrentModificationException` | `structure` | message | Exception thrown when a resource is modified concurrently by multiple requests. |
| `ConflictException` | `structure` | message | This exception is thrown when there is a conflict performing an operation |
| `DecryptionFailure` | `structure` | message | Exception thrown when decryption of a secret fails. |
| `EncryptionFailure` | `structure` | message | Exception thrown when encryption of a secret fails. |
| `InternalServerException` | `structure` | message | This exception is thrown if there was an unexpected error during processing of request |
| `ResourceLimitExceededException` | `structure` | message | Exception thrown when a resource limit is exceeded. |
| `ResourceNotFoundException` | `structure` | message | This exception is thrown when a resource referenced by the operation does not exist |
| `ServiceException` | `structure` | message | An internal error occurred. |
| `ServiceQuotaExceededException` | `structure` | message | This exception is thrown when a request is made beyond the service quota |
| `ThrottledException` | `structure` | message | API rate limit has been exceeded. |
| `ThrottlingException` | `structure` | message | This exception is thrown when the number of requests exceeds the limit |
| `UnauthorizedException` | `structure` | message | This exception is thrown when the JWT bearer token is invalid or not found for OAuth bearer token based access |
| `ValidationException` | `structure` | message, reason, fieldList | The input fails to satisfy the constraints specified by the service. |
| `DeleteResourcePolicyRequest` | `structure` | resourceArn | - |
| `DeleteResourcePolicyResponse` | `structure` | **empty (no members)** | - |
| `GetResourcePolicyRequest` | `structure` | resourceArn | - |
| `GetResourcePolicyResponse` | `structure` | policy | - |
| `GetTokenVaultRequest` | `structure` | tokenVaultId | - |
| `GetTokenVaultResponse` | `structure` | tokenVaultId, kmsConfiguration, lastModifiedDate | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `PutResourcePolicyRequest` | `structure` | resourceArn, policy | - |
| `PutResourcePolicyResponse` | `structure` | policy | - |
| `SetTokenVaultCMKRequest` | `structure` | tokenVaultId, kmsConfiguration | - |
| `SetTokenVaultCMKResponse` | `structure` | tokenVaultId, kmsConfiguration, lastModifiedDate | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `ActorTokenContentType` | `enum` | NONE, M2M, AWS_IAM_ID_TOKEN_JWT | - |
| `AgentManagedRuntimeType` | `enum` | PYTHON_3_10, PYTHON_3_11, PYTHON_3_12, PYTHON_3_13, PYTHON_3_14, NODE_22 | - |
| `AgentRuntimeEndpointStatus` | `enum` | CREATING, CREATE_FAILED, UPDATING, UPDATE_FAILED, READY, DELETING | - |
| `AgentRuntimeStatus` | `enum` | CREATING, CREATE_FAILED, UPDATING, UPDATE_FAILED, READY, DELETING | - |
| `ApiKeyCredentialLocation` | `enum` | HEADER, QUERY_PARAMETER | - |
| `AuthorizerType` | `enum` | CUSTOM_JWT, AWS_IAM, NONE, AUTHENTICATE_ONLY | - |
| `BrowserEnterprisePolicyType` | `enum` | MANAGED, RECOMMENDED | - |
| `BrowserNetworkMode` | `enum` | PUBLIC, VPC | - |
| `BrowserProfileStatus` | `enum` | READY, DELETING, DELETED, SAVING | The status of a browser profile. |
| `BrowserStatus` | `enum` | CREATING, CREATE_FAILED, READY, DELETING, DELETE_FAILED, DELETED | - |
| `ClaimMatchOperatorType` | `enum` | EQUALS, CONTAINS, CONTAINS_ANY | - |
| `ClientAuthenticationMethodType` | `enum` | CLIENT_SECRET_BASIC, CLIENT_SECRET_POST, AWS_IAM_ID_TOKEN_JWT | - |
| `CodeInterpreterNetworkMode` | `enum` | PUBLIC, SANDBOX, VPC | - |
| `CodeInterpreterStatus` | `enum` | CREATING, CREATE_FAILED, READY, DELETING, DELETE_FAILED, DELETED | - |
| `ConfigurationBundleStatus` | `enum` | ACTIVE, CREATING, CREATE_FAILED, UPDATING, UPDATE_FAILED, DELETING, DELETE_FAILED | - |
| `ContentLevel` | `enum` | METADATA_ONLY, FULL_CONTENT | - |
| `ContentType` | `enum` | MEMORY_RECORDS | - |
| `CredentialProviderType` | `enum` | GATEWAY_IAM_ROLE, OAUTH, API_KEY, CALLER_IAM_CREDENTIALS, JWT_PASSTHROUGH | - |
| `CredentialProviderVendorType` | `enum` | GoogleOauth2, GithubOauth2, SlackOauth2, SalesforceOauth2, MicrosoftOauth2, CustomOauth2, AtlassianOauth2, LinkedinOauth2, XOauth2, OktaOauth2, OneLoginOauth2, PingOneOauth2, ... (+13) | - |
| `DescriptorType` | `enum` | MCP, A2A, CUSTOM, AGENT_SKILLS | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
