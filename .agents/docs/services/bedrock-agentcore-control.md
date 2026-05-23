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

### List

- Operations: `ListAgentRuntimeEndpoints`, `ListAgentRuntimeVersions`, `ListAgentRuntimes`, `ListApiKeyCredentialProviders`, `ListBrowserProfiles`, `ListBrowsers`, `ListCodeInterpreters`, `ListEvaluators`, `ListGatewayTargets`, `ListGateways`, `ListMemories`, `ListOauth2CredentialProviders`, `ListOnlineEvaluationConfigs`, `ListPolicies`, `ListPolicyEngines`, `ListPolicyGenerationAssets`, `ListPolicyGenerations`, `ListTagsForResource`, `ListWorkloadIdentities`
- Traits: `paginated` (18), `readonly` (19)
- Common required input members in this group: `agentRuntimeId`, `gatewayIdentifier`, `policyEngineId`, `policyGenerationId`, `resourceArn`

### Get

- Operations: `GetAgentRuntime`, `GetAgentRuntimeEndpoint`, `GetApiKeyCredentialProvider`, `GetBrowser`, `GetBrowserProfile`, `GetCodeInterpreter`, `GetEvaluator`, `GetGateway`, `GetGatewayTarget`, `GetMemory`, `GetOauth2CredentialProvider`, `GetOnlineEvaluationConfig`, `GetPolicy`, `GetPolicyEngine`, `GetPolicyGeneration`, `GetResourcePolicy`, `GetTokenVault`, `GetWorkloadIdentity`
- Traits: `readonly` (18)
- Common required input members in this group: `agentRuntimeId`, `browserId`, `codeInterpreterId`, `endpointName`, `evaluatorId`, `gatewayIdentifier`, `memoryId`, `name`, `onlineEvaluationConfigId`, `policyEngineId`, `policyGenerationId`, `policyId`, `profileId`, `resourceArn`, `targetId`

### Delete

- Operations: `DeleteAgentRuntime`, `DeleteAgentRuntimeEndpoint`, `DeleteApiKeyCredentialProvider`, `DeleteBrowser`, `DeleteBrowserProfile`, `DeleteCodeInterpreter`, `DeleteEvaluator`, `DeleteGateway`, `DeleteGatewayTarget`, `DeleteMemory`, `DeleteOauth2CredentialProvider`, `DeleteOnlineEvaluationConfig`, `DeletePolicy`, `DeletePolicyEngine`, `DeleteResourcePolicy`, `DeleteWorkloadIdentity`
- Traits: `idempotency-token` (6), `idempotent` (16)
- Common required input members in this group: `agentRuntimeId`, `browserId`, `codeInterpreterId`, `endpointName`, `evaluatorId`, `gatewayIdentifier`, `memoryId`, `name`, `onlineEvaluationConfigId`, `policyEngineId`, `policyId`, `profileId`, `resourceArn`, `targetId`

### Create

- Operations: `CreateAgentRuntime`, `CreateAgentRuntimeEndpoint`, `CreateApiKeyCredentialProvider`, `CreateBrowser`, `CreateBrowserProfile`, `CreateCodeInterpreter`, `CreateEvaluator`, `CreateGateway`, `CreateGatewayTarget`, `CreateMemory`, `CreateOauth2CredentialProvider`, `CreateOnlineEvaluationConfig`, `CreatePolicy`, `CreatePolicyEngine`, `CreateWorkloadIdentity`
- Traits: `idempotency-token` (12), `idempotent` (11)
- Common required input members in this group: `agentRuntimeArtifact`, `agentRuntimeId`, `agentRuntimeName`, `apiKey`, `authorizerType`, `credentialProviderVendor`, `dataSourceConfig`, `definition`, `enableOnCreate`, `evaluationExecutionRoleArn`, `evaluatorConfig`, `evaluatorName`, `evaluators`, `eventExpiryDuration`, `gatewayIdentifier`, `level`, `name`, `networkConfiguration`, `oauth2ProviderConfigInput`, `onlineEvaluationConfigName`, `policyEngineId`, `protocolType`, `roleArn`, `rule`, ... (+1)

### Update

- Operations: `UpdateAgentRuntime`, `UpdateAgentRuntimeEndpoint`, `UpdateApiKeyCredentialProvider`, `UpdateEvaluator`, `UpdateGateway`, `UpdateGatewayTarget`, `UpdateMemory`, `UpdateOauth2CredentialProvider`, `UpdateOnlineEvaluationConfig`, `UpdatePolicy`, `UpdatePolicyEngine`, `UpdateWorkloadIdentity`
- Traits: `idempotency-token` (5), `idempotent` (11)
- Common required input members in this group: `agentRuntimeArtifact`, `agentRuntimeId`, `apiKey`, `authorizerType`, `credentialProviderVendor`, `endpointName`, `evaluatorId`, `gatewayIdentifier`, `memoryId`, `name`, `networkConfiguration`, `oauth2ProviderConfigInput`, `onlineEvaluationConfigId`, `policyEngineId`, `policyId`, `protocolType`, `roleArn`, `targetConfiguration`, `targetId`

### Put

- Operations: `PutResourcePolicy`
- Traits: `idempotent` (1)
- Common required input members in this group: `policy`, `resourceArn`

### Set

- Operations: `SetTokenVaultCMK`
- Common required input members in this group: `kmsConfiguration`

### Start

- Operations: `StartPolicyGeneration`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `content`, `name`, `policyEngineId`, `resource`

### Synchronize

- Operations: `SynchronizeGatewayTargets`
- Traits: `idempotent` (1)
- Common required input members in this group: `gatewayIdentifier`, `targetIdList`

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
| `CreateAgentRuntime` | `PUT /runtimes/` | `idempotent`, `idempotency-token` | `agentRuntimeArtifact`, `agentRuntimeName`, `networkConfiguration`, `roleArn` | `clientToken` | `CreateAgentRuntimeResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Amazon Bedrock AgentCore Runtime. |
| `CreateAgentRuntimeEndpoint` | `PUT /runtimes/{agentRuntimeId}/runtime-endpoints/` | `idempotent`, `idempotency-token` | `agentRuntimeId`, `name` | `clientToken` | `CreateAgentRuntimeEndpointResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an AgentCore Runtime endpoint. |
| `CreateApiKeyCredentialProvider` | `POST /identities/CreateApiKeyCredentialProvider` | `idempotent` | `apiKey`, `name` | - | `CreateApiKeyCredentialProviderResponse` | `AccessDeniedException`, `ConflictException`, `DecryptionFailure`, `EncryptionFailure`, `InternalServerException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, ... (+3) | Creates a new API key credential provider. |
| `CreateBrowser` | `PUT /browsers` | `idempotent`, `idempotency-token` | `name`, `networkConfiguration` | `clientToken` | `CreateBrowserResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a custom browser. |
| `CreateBrowserProfile` | `PUT /browser-profiles` | `idempotent`, `idempotency-token` | `name` | `clientToken` | `CreateBrowserProfileResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a browser profile in Amazon Bedrock AgentCore. A browser profile stores persistent browser data such as cookies, local storage, session storage, and browsing history that can be saved from browser sessions and reused in subsequent sessions. |
| `CreateCodeInterpreter` | `PUT /code-interpreters` | `idempotent`, `idempotency-token` | `name`, `networkConfiguration` | `clientToken` | `CreateCodeInterpreterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a custom code interpreter. |
| `CreateEvaluator` | `POST /evaluators/create` | `idempotency-token` | `evaluatorConfig`, `evaluatorName`, `level` | `clientToken` | `CreateEvaluatorResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a custom evaluator for agent quality assessment. Custom evaluators use LLM-as-a-Judge configurations with user-defined prompts, rating scales, and model settings to evaluate agent performance at tool call, trace, or session levels. |
| `CreateGateway` | `POST /gateways/` | `idempotent`, `idempotency-token` | `authorizerType`, `name`, `protocolType`, `roleArn` | `clientToken` | `CreateGatewayResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a gateway for Amazon Bedrock Agent. A gateway serves as an integration point between your agent and external services. |
| `CreateGatewayTarget` | `POST /gateways/{gatewayIdentifier}/targets/` | `idempotent`, `idempotency-token` | `gatewayIdentifier`, `name`, `targetConfiguration` | `clientToken` | `CreateGatewayTargetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a target for a gateway. A target defines an endpoint that the gateway can connect to. |
| `CreateMemory` | `POST /memories/create` | `idempotent`, `idempotency-token` | `eventExpiryDuration`, `name` | `clientToken` | `CreateMemoryOutput` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceException`, `ServiceQuotaExceededException`, `ThrottledException`, `ValidationException` | Creates a new Amazon Bedrock AgentCore Memory resource. |
| `CreateOauth2CredentialProvider` | `POST /identities/CreateOauth2CredentialProvider` | `idempotent` | `credentialProviderVendor`, `name`, `oauth2ProviderConfigInput` | - | `CreateOauth2CredentialProviderResponse` | `AccessDeniedException`, `ConflictException`, `DecryptionFailure`, `EncryptionFailure`, `InternalServerException`, `ResourceLimitExceededException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, ... (+3) | Creates a new OAuth2 credential provider. |
| `CreateOnlineEvaluationConfig` | `POST /online-evaluation-configs/create` | `idempotency-token` | `dataSourceConfig`, `enableOnCreate`, `evaluationExecutionRoleArn`, `evaluators`, `onlineEvaluationConfigName`, `rule` | `clientToken` | `CreateOnlineEvaluationConfigResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an online evaluation configuration for continuous monitoring of agent performance. Online evaluation automatically samples live traffic from CloudWatch logs at specified rates and applies evaluators to assess agent quality in production. |
| `CreatePolicy` | `POST /policy-engines/{policyEngineId}/policies` | `idempotency-token` | `definition`, `name`, `policyEngineId` | `clientToken` | `CreatePolicyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a policy within the AgentCore Policy system. Policies provide real-time, deterministic control over agentic interactions with AgentCore Gateway. |
| `CreatePolicyEngine` | `POST /policy-engines` | `idempotency-token` | `name` | `clientToken` | `CreatePolicyEngineResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new policy engine within the AgentCore Policy system. A policy engine is a collection of policies that evaluates and authorizes agent tool calls. |
| `CreateWorkloadIdentity` | `POST /identities/CreateWorkloadIdentity` | `idempotent` | `name` | - | `CreateWorkloadIdentityResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Creates a new workload identity. |
| `DeleteAgentRuntime` | `DELETE /runtimes/{agentRuntimeId}/` | `idempotent`, `idempotency-token` | `agentRuntimeId` | `clientToken` | `DeleteAgentRuntimeResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes an Amazon Bedrock AgentCore Runtime. |
| `DeleteAgentRuntimeEndpoint` | `DELETE /runtimes/{agentRuntimeId}/runtime-endpoints/{endpointName}/` | `idempotent`, `idempotency-token` | `agentRuntimeId`, `endpointName` | `clientToken` | `DeleteAgentRuntimeEndpointResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes an AAgentCore Runtime endpoint. |
| `DeleteApiKeyCredentialProvider` | `POST /identities/DeleteApiKeyCredentialProvider` | `idempotent` | `name` | - | `DeleteApiKeyCredentialProviderResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Deletes an API key credential provider. |
| `DeleteBrowser` | `DELETE /browsers/{browserId}` | `idempotent`, `idempotency-token` | `browserId` | `clientToken` | `DeleteBrowserResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes a custom browser. |
| `DeleteBrowserProfile` | `DELETE /browser-profiles/{profileId}` | `idempotent`, `idempotency-token` | `profileId` | `clientToken` | `DeleteBrowserProfileResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a browser profile. |
| `DeleteCodeInterpreter` | `DELETE /code-interpreters/{codeInterpreterId}` | `idempotent`, `idempotency-token` | `codeInterpreterId` | `clientToken` | `DeleteCodeInterpreterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes a custom code interpreter. |
| `DeleteEvaluator` | `DELETE /evaluators/{evaluatorId}` | `idempotent` | `evaluatorId` | - | `DeleteEvaluatorResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a custom evaluator. Builtin evaluators cannot be deleted. |
| `DeleteGateway` | `DELETE /gateways/{gatewayIdentifier}/` | `idempotent` | `gatewayIdentifier` | - | `DeleteGatewayResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a gateway. |
| `DeleteGatewayTarget` | `DELETE /gateways/{gatewayIdentifier}/targets/{targetId}/` | `idempotent` | `gatewayIdentifier`, `targetId` | - | `DeleteGatewayTargetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a gateway target. |
| `DeleteMemory` | `DELETE /memories/{memoryId}/delete` | `idempotent`, `idempotency-token` | `memoryId` | `clientToken` | `DeleteMemoryOutput` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceException`, `ThrottledException`, `ValidationException` | Deletes an Amazon Bedrock AgentCore Memory resource. |
| `DeleteOauth2CredentialProvider` | `POST /identities/DeleteOauth2CredentialProvider` | `idempotent` | `name` | - | `DeleteOauth2CredentialProviderResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Deletes an OAuth2 credential provider. |
| `DeleteOnlineEvaluationConfig` | `DELETE /online-evaluation-configs/{onlineEvaluationConfigId}` | `idempotent` | `onlineEvaluationConfigId` | - | `DeleteOnlineEvaluationConfigResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an online evaluation configuration and stops any ongoing evaluation processes associated with it. |
| `DeletePolicy` | `DELETE /policy-engines/{policyEngineId}/policies/{policyId}` | `idempotent` | `policyEngineId`, `policyId` | - | `DeletePolicyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing policy from the AgentCore Policy system. Once deleted, the policy can no longer be used for agent behavior control and all references to it become invalid. |
| `DeletePolicyEngine` | `DELETE /policy-engines/{policyEngineId}` | `idempotent` | `policyEngineId` | - | `DeletePolicyEngineResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing policy engine from the AgentCore Policy system. The policy engine must not have any associated policies before deletion. |
| `DeleteResourcePolicy` | `DELETE /resourcepolicy/{resourceArn}` | `idempotent` | `resourceArn` | - | `DeleteResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the resource-based policy for a specified resource. This feature is currently available only for AgentCore Runtime and Gateway. |
| `DeleteWorkloadIdentity` | `POST /identities/DeleteWorkloadIdentity` | `idempotent` | `name` | - | `DeleteWorkloadIdentityResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Deletes a workload identity. |
| `GetAgentRuntime` | `GET /runtimes/{agentRuntimeId}/` | `readonly` | `agentRuntimeId` | - | `GetAgentRuntimeResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets an Amazon Bedrock AgentCore Runtime. |
| `GetAgentRuntimeEndpoint` | `GET /runtimes/{agentRuntimeId}/runtime-endpoints/{endpointName}/` | `readonly` | `agentRuntimeId`, `endpointName` | - | `GetAgentRuntimeEndpointResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about an Amazon Secure AgentEndpoint. |
| `GetApiKeyCredentialProvider` | `POST /identities/GetApiKeyCredentialProvider` | `readonly` | `name` | - | `GetApiKeyCredentialProviderResponse` | `AccessDeniedException`, `DecryptionFailure`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Retrieves information about an API key credential provider. |
| `GetBrowser` | `GET /browsers/{browserId}` | `readonly` | `browserId` | - | `GetBrowserResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException` | Gets information about a custom browser. |
| `GetBrowserProfile` | `GET /browser-profiles/{profileId}` | `readonly` | `profileId` | - | `GetBrowserProfileResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a browser profile. |
| `GetCodeInterpreter` | `GET /code-interpreters/{codeInterpreterId}` | `readonly` | `codeInterpreterId` | - | `GetCodeInterpreterResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException` | Gets information about a custom code interpreter. |
| `GetEvaluator` | `GET /evaluators/{evaluatorId}` | `readonly` | `evaluatorId` | - | `GetEvaluatorResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about an evaluator, including its configuration, status, and metadata. Works with both built-in and custom evaluators. |
| `GetGateway` | `GET /gateways/{gatewayIdentifier}/` | `readonly` | `gatewayIdentifier` | - | `GetGatewayResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a specific Gateway. |
| `GetGatewayTarget` | `GET /gateways/{gatewayIdentifier}/targets/{targetId}/` | `readonly` | `gatewayIdentifier`, `targetId` | - | `GetGatewayTargetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a specific gateway target. |
| `GetMemory` | `GET /memories/{memoryId}/details` | `readonly` | `memoryId` | - | `GetMemoryOutput` | `AccessDeniedException`, `ResourceNotFoundException`, `ServiceException`, `ThrottledException`, `ValidationException` | Retrieve an existing Amazon Bedrock AgentCore Memory resource. |
| `GetOauth2CredentialProvider` | `POST /identities/GetOauth2CredentialProvider` | `readonly` | `name` | - | `GetOauth2CredentialProviderResponse` | `AccessDeniedException`, `DecryptionFailure`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Retrieves information about an OAuth2 credential provider. |
| `GetOnlineEvaluationConfig` | `GET /online-evaluation-configs/{onlineEvaluationConfigId}` | `readonly` | `onlineEvaluationConfigId` | - | `GetOnlineEvaluationConfigResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about an online evaluation configuration, including its rules, data sources, evaluators, and execution status. |
| `GetPolicy` | `GET /policy-engines/{policyEngineId}/policies/{policyId}` | `readonly` | `policyEngineId`, `policyId` | - | `GetPolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a specific policy within the AgentCore Policy system. This operation returns the complete policy definition, metadata, and current status, allowing administrators to review and manage policy configurations. |
| `GetPolicyEngine` | `GET /policy-engines/{policyEngineId}` | `readonly` | `policyEngineId` | - | `GetPolicyEngineResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a specific policy engine within the AgentCore Policy system. This operation returns the complete policy engine configuration, metadata, and current status, allowing administrators to review and manage policy engine... |
| `GetPolicyGeneration` | `GET /policy-engines/{policyEngineId}/policy-generations/{policyGenerationId}` | `readonly` | `policyEngineId`, `policyGenerationId` | - | `GetPolicyGenerationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a policy generation request within the AgentCore Policy system. Policy generation converts natural language descriptions into Cedar policy statements using AI-powered translation, enabling non-technical users to create policies. |
| `GetResourcePolicy` | `GET /resourcepolicy/{resourceArn}` | `readonly` | `resourceArn` | - | `GetResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the resource-based policy for a specified resource. This feature is currently available only for AgentCore Runtime and Gateway. |
| `GetTokenVault` | `POST /identities/get-token-vault` | `readonly` | - | - | `GetTokenVaultResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Retrieves information about a token vault. |
| `GetWorkloadIdentity` | `POST /identities/GetWorkloadIdentity` | `readonly` | `name` | - | `GetWorkloadIdentityResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Retrieves information about a workload identity. |
| `ListAgentRuntimeEndpoints` | `POST /runtimes/{agentRuntimeId}/runtime-endpoints/` | `readonly`, `paginated` | `agentRuntimeId` | - | `ListAgentRuntimeEndpointsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all endpoints for a specific Amazon Secure Agent. |
| `ListAgentRuntimeVersions` | `POST /runtimes/{agentRuntimeId}/versions/` | `readonly`, `paginated` | `agentRuntimeId` | - | `ListAgentRuntimeVersionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all versions of a specific Amazon Secure Agent. |
| `ListAgentRuntimes` | `POST /runtimes/` | `readonly`, `paginated` | - | - | `ListAgentRuntimesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all Amazon Secure Agents in your account. |
| `ListApiKeyCredentialProviders` | `POST /identities/ListApiKeyCredentialProviders` | `readonly`, `paginated` | - | - | `ListApiKeyCredentialProvidersResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Lists all API key credential providers in your account. |
| `ListBrowserProfiles` | `POST /browser-profiles` | `readonly`, `paginated` | - | - | `ListBrowserProfilesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all browser profiles in your account. |
| `ListBrowsers` | `POST /browsers` | `readonly`, `paginated` | - | - | `ListBrowsersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all custom browsers in your account. |
| `ListCodeInterpreters` | `POST /code-interpreters` | `readonly`, `paginated` | - | - | `ListCodeInterpretersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all custom code interpreters in your account. |
| `ListEvaluators` | `POST /evaluators` | `readonly`, `paginated` | - | - | `ListEvaluatorsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all available evaluators, including both builtin evaluators provided by the service and custom evaluators created by the user. |
| `ListGatewayTargets` | `GET /gateways/{gatewayIdentifier}/targets/` | `readonly`, `paginated` | `gatewayIdentifier` | - | `ListGatewayTargetsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all targets for a specific gateway. |
| `ListGateways` | `GET /gateways/` | `readonly`, `paginated` | - | - | `ListGatewaysResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all gateways in the account. |
| `ListMemories` | `POST /memories/` | `readonly`, `paginated` | - | - | `ListMemoriesOutput` | `AccessDeniedException`, `ResourceNotFoundException`, `ServiceException`, `ThrottledException`, `ValidationException` | Lists the available Amazon Bedrock AgentCore Memory resources in the current Amazon Web Services Region. |
| `ListOauth2CredentialProviders` | `POST /identities/ListOauth2CredentialProviders` | `readonly`, `paginated` | - | - | `ListOauth2CredentialProvidersResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Lists all OAuth2 credential providers in your account. |
| `ListOnlineEvaluationConfigs` | `POST /online-evaluation-configs` | `readonly`, `paginated` | - | - | `ListOnlineEvaluationConfigsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all online evaluation configurations in the account, providing summary information about each configuration's status and settings. |
| `ListPolicies` | `GET /policy-engines/{policyEngineId}/policies` | `readonly`, `paginated` | `policyEngineId` | - | `ListPoliciesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of policies within the AgentCore Policy engine. This operation supports pagination and filtering to help administrators manage and discover policies across policy engines. |
| `ListPolicyEngines` | `GET /policy-engines` | `readonly`, `paginated` | - | - | `ListPolicyEnginesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of policy engines within the AgentCore Policy system. This operation supports pagination to help administrators discover and manage policy engines across their account. |
| `ListPolicyGenerationAssets` | `GET /policy-engines/{policyEngineId}/policy-generations/{policyGenerationId}/assets` | `readonly`, `paginated` | `policyEngineId`, `policyGenerationId` | - | `ListPolicyGenerationAssetsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of generated policy assets from a policy generation request within the AgentCore Policy system. This operation returns the actual Cedar policies and related artifacts produced by the AI-powered policy generation process, allowing users to... |
| `ListPolicyGenerations` | `GET /policy-engines/{policyEngineId}/policy-generations` | `readonly`, `paginated` | `policyEngineId` | - | `ListPolicyGenerationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of policy generation requests within the AgentCore Policy system. This operation supports pagination and filtering to help track and manage AI-powered policy generation operations. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags associated with the specified resource. This feature is currently available only for AgentCore Runtime, Browser, Browser Profile, Code Interpreter tool, and Gateway. |
| `ListWorkloadIdentities` | `POST /identities/ListWorkloadIdentities` | `readonly`, `paginated` | - | - | `ListWorkloadIdentitiesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Lists all workload identities in your account. |
| `PutResourcePolicy` | `PUT /resourcepolicy/{resourceArn}` | `idempotent` | `policy`, `resourceArn` | - | `PutResourcePolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates or updates a resource-based policy for a resource with the specified resourceArn. This feature is currently available only for AgentCore Runtime and Gateway. |
| `SetTokenVaultCMK` | `POST /identities/set-token-vault-cmk` | - | `kmsConfiguration` | - | `SetTokenVaultCMKResponse` | `AccessDeniedException`, `ConcurrentModificationException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Sets the customer master key (CMK) for a token vault. |
| `StartPolicyGeneration` | `POST /policy-engines/{policyEngineId}/policy-generations` | `idempotency-token` | `content`, `name`, `policyEngineId`, `resource` | `clientToken` | `StartPolicyGenerationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Initiates the AI-powered generation of Cedar policies from natural language descriptions within the AgentCore Policy system. This feature enables both technical and non-technical users to create policies by describing their authorization requirements in plain... |
| `SynchronizeGatewayTargets` | `PUT /gateways/{gatewayIdentifier}/synchronizeTargets` | `idempotent` | `gatewayIdentifier`, `targetIdList` | - | `SynchronizeGatewayTargetsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | The gateway targets. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates the specified tags to a resource with the specified resourceArn. If existing tags on a resource are not specified in the request parameters, they are not changed. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the specified tags from the specified resource. This feature is currently available only for AgentCore Runtime, Browser, Browser Profile, Code Interpreter tool, and Gateway. |
| `UpdateAgentRuntime` | `PUT /runtimes/{agentRuntimeId}/` | `idempotent`, `idempotency-token` | `agentRuntimeArtifact`, `agentRuntimeId`, `networkConfiguration`, `roleArn` | `clientToken` | `UpdateAgentRuntimeResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an existing Amazon Secure Agent. |
| `UpdateAgentRuntimeEndpoint` | `PUT /runtimes/{agentRuntimeId}/runtime-endpoints/{endpointName}/` | `idempotent`, `idempotency-token` | `agentRuntimeId`, `endpointName` | `clientToken` | `UpdateAgentRuntimeEndpointResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an existing Amazon Bedrock AgentCore Runtime endpoint. |
| `UpdateApiKeyCredentialProvider` | `POST /identities/UpdateApiKeyCredentialProvider` | `idempotent` | `apiKey`, `name` | - | `UpdateApiKeyCredentialProviderResponse` | `AccessDeniedException`, `ConflictException`, `DecryptionFailure`, `EncryptionFailure`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, ... (+2) | Updates an existing API key credential provider. |
| `UpdateEvaluator` | `PUT /evaluators/{evaluatorId}` | `idempotent`, `idempotency-token` | `evaluatorId` | `clientToken` | `UpdateEvaluatorResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates a custom evaluator's configuration, description, or evaluation level. Built-in evaluators cannot be updated. |
| `UpdateGateway` | `PUT /gateways/{gatewayIdentifier}/` | `idempotent` | `authorizerType`, `gatewayIdentifier`, `name`, `protocolType`, `roleArn` | - | `UpdateGatewayResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an existing gateway. |
| `UpdateGatewayTarget` | `PUT /gateways/{gatewayIdentifier}/targets/{targetId}/` | `idempotent` | `gatewayIdentifier`, `name`, `targetConfiguration`, `targetId` | - | `UpdateGatewayTargetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an existing gateway target. |
| `UpdateMemory` | `PUT /memories/{memoryId}/update` | `idempotent`, `idempotency-token` | `memoryId` | `clientToken` | `UpdateMemoryOutput` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceException`, `ServiceQuotaExceededException`, `ThrottledException`, `ValidationException` | Update an Amazon Bedrock AgentCore Memory resource memory. |
| `UpdateOauth2CredentialProvider` | `POST /identities/UpdateOauth2CredentialProvider` | - | `credentialProviderVendor`, `name`, `oauth2ProviderConfigInput` | - | `UpdateOauth2CredentialProviderResponse` | `AccessDeniedException`, `ConflictException`, `DecryptionFailure`, `EncryptionFailure`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, ... (+2) | Updates an existing OAuth2 credential provider. |
| `UpdateOnlineEvaluationConfig` | `PUT /online-evaluation-configs/{onlineEvaluationConfigId}` | `idempotent`, `idempotency-token` | `onlineEvaluationConfigId` | `clientToken` | `UpdateOnlineEvaluationConfigResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an online evaluation configuration's settings, including rules, data sources, evaluators, and execution status. Changes take effect immediately for ongoing evaluations. |
| `UpdatePolicy` | `PATCH /policy-engines/{policyEngineId}/policies/{policyId}` | `idempotent` | `policyEngineId`, `policyId` | - | `UpdatePolicyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing policy within the AgentCore Policy system. This operation allows modification of the policy description and definition while maintaining the policy's identity. |
| `UpdatePolicyEngine` | `PATCH /policy-engines/{policyEngineId}` | `idempotent` | `policyEngineId` | - | `UpdatePolicyEngineResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing policy engine within the AgentCore Policy system. This operation allows modification of the policy engine description while maintaining its identity. |
| `UpdateWorkloadIdentity` | `POST /identities/UpdateWorkloadIdentity` | `idempotent` | `name` | - | `UpdateWorkloadIdentityResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Updates an existing workload identity. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | This exception is thrown when a request is denied per access permissions |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | The input fails to satisfy the constraints specified by the service. |
| `InternalServerException` | `structure` | `message` | This exception is thrown if there was an unexpected error during processing of request |
| `ThrottlingException` | `structure` | `message` | This exception is thrown when the number of requests exceeds the limit |
| `ResourceNotFoundException` | `structure` | `message` | This exception is thrown when a resource referenced by the operation does not exist |
| `ConflictException` | `structure` | `message` | This exception is thrown when there is a conflict performing an operation |
| `ServiceQuotaExceededException` | `structure` | `message` | This exception is thrown when a request is made beyond the service quota |
| `UnauthorizedException` | `structure` | `message` | This exception is thrown when the JWT bearer token is invalid or not found for OAuth bearer token based access |
| `DecryptionFailure` | `structure` | `message` | Exception thrown when decryption of a secret fails. |
| `ServiceException` | `structure` | `message` | An internal error occurred. |
| `ThrottledException` | `structure` | `message` | API rate limit has been exceeded. |
| `EncryptionFailure` | `structure` | `message` | Exception thrown when encryption of a secret fails. |
| `ResourceLimitExceededException` | `structure` | `message` | Exception thrown when a resource limit is exceeded. |
| `CreateAgentRuntimeRequest` | `structure` | `agentRuntimeArtifact`, `agentRuntimeName`, `authorizerConfiguration`, `clientToken`, `description`, `environmentVariables`, `lifecycleConfiguration`, `networkConfiguration`, `protocolConfiguration`, `requestHeaderConfiguration`, `roleArn`, `tags` | - |
| `CreateAgentRuntimeResponse` | `structure` | `agentRuntimeArn`, `agentRuntimeId`, `agentRuntimeVersion`, `createdAt`, `status`, `workloadIdentityDetails` | - |
| `CreateAgentRuntimeEndpointRequest` | `structure` | `agentRuntimeId`, `agentRuntimeVersion`, `clientToken`, `description`, `name`, `tags` | - |
| `CreateAgentRuntimeEndpointResponse` | `structure` | `agentRuntimeArn`, `agentRuntimeEndpointArn`, `agentRuntimeId`, `createdAt`, `endpointName`, `status`, `targetVersion` | - |
| `CreateApiKeyCredentialProviderRequest` | `structure` | `apiKey`, `name`, `tags` | - |
| `CreateApiKeyCredentialProviderResponse` | `structure` | `apiKeySecretArn`, `credentialProviderArn`, `name` | - |
| `CreateBrowserRequest` | `structure` | `browserSigning`, `clientToken`, `description`, `executionRoleArn`, `name`, `networkConfiguration`, `recording`, `tags` | - |
| `CreateBrowserResponse` | `structure` | `browserArn`, `browserId`, `createdAt`, `status` | - |
| `CreateBrowserProfileRequest` | `structure` | `clientToken`, `description`, `name`, `tags` | - |
| `CreateBrowserProfileResponse` | `structure` | `createdAt`, `profileArn`, `profileId`, `status` | - |
| `CreateCodeInterpreterRequest` | `structure` | `clientToken`, `description`, `executionRoleArn`, `name`, `networkConfiguration`, `tags` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
