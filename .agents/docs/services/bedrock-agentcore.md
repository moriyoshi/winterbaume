# Amazon Bedrock AgentCore

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Welcome to the Amazon Bedrock AgentCore Data Plane API reference. Data Plane actions process and handle data or workloads within Amazon Web Services services.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Bedrock AgentCore resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: invoke AgentCore runtime resources and interact with agent execution endpoints.
- From the operation surface: model low-latency runtime invocation, session continuity, payload exchange, and error handling for deployed AgentCore agents.

## Service Identity and Protocol

- AWS model slug: `bedrock-agentcore`
- AWS SDK for Rust slug: `bedrockagentcore`
- Model version: `2024-02-28`
- Model file: `vendor/api-models-aws/models/bedrock-agentcore/service/2024-02-28/bedrock-agentcore-2024-02-28.json`
- SDK ID: `Bedrock AgentCore`
- Endpoint prefix: `bedrock-agentcore`
- ARN namespace: `bedrock-agentcore`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (10), `List` (7), `Batch` (3), `Start` (3), `Stop` (3), `Delete` (2), `Invoke` (2), `Complete` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchCreateMemoryRecords`, `BatchDeleteMemoryRecords`, `BatchUpdateMemoryRecords`, `CreateEvent`, `DeleteEvent`, `DeleteMemoryRecord`, `StartBrowserSession`, `StartCodeInterpreterSession`, `StartMemoryExtractionJob`, `StopBrowserSession`, `StopCodeInterpreterSession`, `StopRuntimeSession`, `UpdateBrowserStream`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAgentCard`, `GetBrowserSession`, `GetCodeInterpreterSession`, `GetEvent`, `GetMemoryRecord`, `GetResourceApiKey`, `GetResourceOauth2Token`, `GetWorkloadAccessToken`, `GetWorkloadAccessTokenForJWT`, `GetWorkloadAccessTokenForUserId`, `ListActors`, `ListBrowserSessions`, `ListCodeInterpreterSessions`, `ListEvents`, `ListMemoryExtractionJobs`, `ListMemoryRecords`, `ListSessions`.
- Pagination is modelled for 6 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 12 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ListMemoryExtractionJobs`, `StartBrowserSession`, `StartCodeInterpreterSession`, `StartMemoryExtractionJob`, `StopBrowserSession`, `StopCodeInterpreterSession`, `StopRuntimeSession`.
- 36 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `Secrets Manager`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AgenticResource` | - | - | `GetAgentCard`, `InvokeAgentRuntime`, `StopRuntimeSession` | - |
| `BrowserProfileResource` | `profileIdentifier` | - | `SaveBrowserSessionProfile` | - |
| `BrowserSessionResource` | `browserIdentifier` | read: `GetBrowserSession` | `ListBrowserSessions`, `StartBrowserSession`, `StopBrowserSession`, `UpdateBrowserStream` | - |
| `CodeInterpreterSessionResource` | `codeInterpreterIdentifier` | read: `GetCodeInterpreterSession` | `ListCodeInterpreterSessions`, `StartCodeInterpreterSession`, `StopCodeInterpreterSession` | - |
| `EvaluationResource` | - | - | `Evaluate` | - |
| `MemoryResource` | - | - | `BatchCreateMemoryRecords`, `BatchDeleteMemoryRecords`, `BatchUpdateMemoryRecords`, `CreateEvent`, `DeleteEvent`, `DeleteMemoryRecord`, `GetEvent`, `GetMemoryRecord`, `ListActors`, `ListEvents`, ... (+5) | - |
## Operation Groups

### Get

- Operations: `GetResourceApiKey`, `GetResourceOauth2Token`, `GetWorkloadAccessToken`, `GetWorkloadAccessTokenForJWT`, `GetWorkloadAccessTokenForUserId`
- Traits: `readonly` (2)
- Common required input members in this group: `workloadIdentityToken`, `resourceCredentialProviderName`, `workloadName`

### Invoke

- Operations: `InvokeCodeInterpreter`, `InvokeHarness`
- Common required input members in this group: -

### Complete

- Operations: `CompleteResourceTokenAuth`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CompleteResourceTokenAuth` | `POST /identities/CompleteResourceTokenAuth` | - | `userIdentifier`, `sessionUri` | - | `CompleteResourceTokenAuthResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Confirms the user authentication session for obtaining OAuth2.0 tokens for a resource. |
| `GetResourceApiKey` | `POST /identities/api-key` | `readonly` | `workloadIdentityToken`, `resourceCredentialProviderName` | - | `GetResourceApiKeyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Retrieves the API key associated with an API key credential provider. |
| `GetResourceOauth2Token` | `POST /identities/oauth2/token` | `readonly` | `workloadIdentityToken`, `resourceCredentialProviderName`, `scopes`, `oauth2Flow` | - | `GetResourceOauth2TokenResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Returns the OAuth 2.0 token of the provided resource. |
| `GetWorkloadAccessToken` | `POST /identities/GetWorkloadAccessToken` | - | `workloadName` | - | `GetWorkloadAccessTokenResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Obtains a workload access token for agentic workloads not acting on behalf of a user. |
| `GetWorkloadAccessTokenForJWT` | `POST /identities/GetWorkloadAccessTokenForJWT` | - | `workloadName`, `userToken` | - | `GetWorkloadAccessTokenForJWTResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Obtains a workload access token for agentic workloads acting on behalf of a user, using a JWT token. |
| `GetWorkloadAccessTokenForUserId` | `POST /identities/GetWorkloadAccessTokenForUserId` | - | `workloadName`, `userId` | - | `GetWorkloadAccessTokenForUserIdResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Obtains a workload access token for agentic workloads acting on behalf of a user, using the user's ID. |
| `InvokeCodeInterpreter` | `POST /code-interpreters/{codeInterpreterIdentifier}/tools/invoke` | - | `codeInterpreterIdentifier`, `name` | - | `InvokeCodeInterpreterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Executes code within an active code interpreter session in Amazon Bedrock AgentCore. This operation processes the provided code, runs it in a secure environment, and returns the execution results including output, er ... |
| `InvokeHarness` | `POST /harnesses/invoke` | - | `harnessArn`, `runtimeSessionId`, `messages` | - | `InvokeHarnessResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Operation to invoke a Harness. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `InvokeCodeInterpreter` | `sessionId -> x-amzn-code-interpreter-session-id`, `traceId -> X-Amzn-Trace-Id`, `traceParent -> traceparent` | - | - | - |
| `InvokeHarness` | `runtimeSessionId -> X-Amzn-Bedrock-AgentCore-Runtime-Session-Id` | `harnessArn -> harnessArn` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | The exception that occurs when you do not have sufficient permissions to perform an action. Verify that your IAM policy includes the necessary permissions f ... |
| `ConflictException` | `structure` | message | The exception that occurs when the request conflicts with the current state of the resource. This can happen when trying to modify a resource that is curren ... |
| `DuplicateIdException` | `structure` | message | An exception thrown when attempting to create a resource with an identifier that already exists. |
| `InternalServerException` | `structure` | message | The exception that occurs when the service encounters an unexpected internal error. This is a temporary condition that will resolve itself with retries. We ... |
| `InvalidInputException` | `structure` | message | The input fails to satisfy the constraints specified by AgentCore. Check your input values and try again. |
| `ResourceNotFoundException` | `structure` | message | The exception that occurs when the specified resource does not exist. This can happen when using an invalid identifier or when trying to access a resource t ... |
| `RetryableConflictException` | `structure` | message | The exception that occurs when there is a retryable conflict performing an operation. This is a temporary condition that may resolve itself with retries. We ... |
| `RuntimeClientError` | `structure` | message | The exception that occurs when there is an error in the runtime client. This can happen due to network issues, invalid configuration, or other client-side p ... |
| `ServiceException` | `structure` | message | The service encountered an internal error. Try your request again later. |
| `ServiceQuotaExceededException` | `structure` | message | The exception that occurs when the request would cause a service quota to be exceeded. Review your service quotas and either reduce your request rate or req ... |
| `ThrottledException` | `structure` | message | The request was denied due to request throttling. Reduce the frequency of requests and try again. |
| `ThrottlingException` | `structure` | message | The exception that occurs when the request was denied due to request throttling. This happens when you exceed the allowed request rate for an operation. Red ... |
| `UnauthorizedException` | `structure` | message | This exception is thrown when the JWT bearer token is invalid or not found for OAuth bearer token based access |
| `ValidationException` | `structure` | message, reason, fieldList | The exception that occurs when the input fails to satisfy the constraints specified by the service. Check the error message for details about which input pa ... |
| `CompleteResourceTokenAuthRequest` | `structure` | userIdentifier, sessionUri | - |
| `CompleteResourceTokenAuthResponse` | `structure` | **empty (no members)** | - |
| `GetResourceApiKeyRequest` | `structure` | workloadIdentityToken, resourceCredentialProviderName | - |
| `GetResourceApiKeyResponse` | `structure` | apiKey | - |
| `GetResourceOauth2TokenRequest` | `structure` | workloadIdentityToken, resourceCredentialProviderName, scopes, oauth2Flow, sessionUri, resourceOauth2ReturnUrl, forceAuthentication, customParameters, customState, resources, audiences | - |
| `GetResourceOauth2TokenResponse` | `structure` | authorizationUrl, accessToken, sessionUri, sessionStatus | - |
| `GetWorkloadAccessTokenRequest` | `structure` | workloadName | - |
| `GetWorkloadAccessTokenResponse` | `structure` | workloadAccessToken | - |
| `GetWorkloadAccessTokenForJWTRequest` | `structure` | workloadName, userToken | - |
| `GetWorkloadAccessTokenForJWTResponse` | `structure` | workloadAccessToken | - |
| `GetWorkloadAccessTokenForUserIdRequest` | `structure` | workloadName, userId | - |
| `GetWorkloadAccessTokenForUserIdResponse` | `structure` | workloadAccessToken | - |
| `InvokeCodeInterpreterRequest` | `structure` | codeInterpreterIdentifier, sessionId, traceId, traceParent, name, arguments | - |
| `InvokeCodeInterpreterResponse` | `structure` | sessionId, stream | - |
| `InvokeHarnessRequest` | `structure` | harnessArn, runtimeSessionId, messages, model, systemPrompt, tools, skills, allowedTools, maxIterations, maxTokens, timeoutSeconds, actorId | - |
| `InvokeHarnessResponse` | `structure` | stream | - |
| `ABTestExecutionStatus` | `enum` | PAUSED, RUNNING, STOPPED, NOT_STARTED | - |
| `ABTestStatus` | `enum` | CREATING, ACTIVE, CREATE_FAILED, UPDATING, UPDATE_FAILED, DELETING, DELETE_FAILED, FAILED | - |
| `AutomationStreamStatus` | `enum` | ENABLED, DISABLED | - |
| `BatchEvaluationStatus` | `enum` | PENDING, IN_PROGRESS, COMPLETED, COMPLETED_WITH_ERRORS, FAILED, STOPPING, STOPPED, DELETING | - |
| `BrowserActionStatus` | `enum` | SUCCESS, FAILED | The status of a browser action execution. |
| `BrowserEnterprisePolicyType` | `enum` | MANAGED, RECOMMENDED | The type of browser enterprise policy. Available values are MANAGED and RECOMMENDED . |
| `BrowserSessionStatus` | `enum` | READY, TERMINATED | - |
| `CloudWatchLogsFilterOperator` | `enum` | EQUALS, NOT_EQUALS, GREATER_THAN, LESS_THAN, GREATER_THAN_OR_EQUAL, LESS_THAN_OR_EQUAL, CONTAINS, NOT_CONTAINS | - |
| `CodeInterpreterSessionStatus` | `enum` | READY, TERMINATED | - |
| `CommandExecutionStatus` | `enum` | COMPLETED, TIMED_OUT | - |
| `ContentBlockType` | `enum` | TEXT, IMAGE, EMBEDDED_RESOURCE, RESOURCE_LINK | - |
| `DescriptorType` | `enum` | MCP, A2A, CUSTOM, AGENT_SKILLS | The type of descriptor associated with a registry record. |
| `EventFilterCondition` | `enum` | HAS_EVENTS | The condition to use for filtering sessions by events. |
| `ExtractionJobStatus` | `enum` | FAILED | - |
| `HarnessConversationRole` | `enum` | USER, ASSISTANT | - |
| `HarnessStopReason` | `enum` | END_TURN, TOOL_USE, TOOL_RESULT, MAX_TOKENS, STOP_SEQUENCE, CONTENT_FILTERED, MALFORMED_MODEL_OUTPUT, MALFORMED_TOOL_USE, INTERRUPTED, PARTIAL_TURN, MODEL_CONTEXT_WINDOW_EXCEEDED, MAX_ITERATIONS_EXCEEDED, ... (+2) | - |
| `HarnessToolType` | `enum` | REMOTE_MCP, AGENTCORE_BROWSER, AGENTCORE_GATEWAY, INLINE_FUNCTION, AGENTCORE_CODE_INTERPRETER | - |
| `HarnessToolUseStatus` | `enum` | SUCCESS, ERROR | - |
| `HarnessToolUseType` | `enum` | TOOL_USE, SERVER_TOOL_USE, MCP_TOOL_USE | - |
| `LanguageRuntime` | `enum` | NODEJS, DENO, PYTHON | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
