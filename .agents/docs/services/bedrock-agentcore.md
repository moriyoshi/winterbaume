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

- Operations: `GetAgentCard`, `GetBrowserSession`, `GetCodeInterpreterSession`, `GetEvent`, `GetMemoryRecord`, `GetResourceApiKey`, `GetResourceOauth2Token`, `GetWorkloadAccessToken`, `GetWorkloadAccessTokenForJWT`, `GetWorkloadAccessTokenForUserId`
- Traits: `idempotency-token` (1), `readonly` (6)
- Common required input members in this group: `actorId`, `agentRuntimeArn`, `browserIdentifier`, `codeInterpreterIdentifier`, `eventId`, `memoryId`, `memoryRecordId`, `oauth2Flow`, `resourceCredentialProviderName`, `scopes`, `sessionId`, `userId`, `userToken`, `workloadIdentityToken`, `workloadName`

### List

- Operations: `ListActors`, `ListBrowserSessions`, `ListCodeInterpreterSessions`, `ListEvents`, `ListMemoryExtractionJobs`, `ListMemoryRecords`, `ListSessions`
- Traits: `paginated` (5), `readonly` (6)
- Common required input members in this group: `actorId`, `browserIdentifier`, `codeInterpreterIdentifier`, `memoryId`, `namespace`, `sessionId`

### Batch

- Operations: `BatchCreateMemoryRecords`, `BatchDeleteMemoryRecords`, `BatchUpdateMemoryRecords`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `memoryId`, `records`

### Start

- Operations: `StartBrowserSession`, `StartCodeInterpreterSession`, `StartMemoryExtractionJob`
- Traits: `idempotency-token` (3), `idempotent` (3)
- Common required input members in this group: `browserIdentifier`, `codeInterpreterIdentifier`, `extractionJob`, `memoryId`

### Stop

- Operations: `StopBrowserSession`, `StopCodeInterpreterSession`, `StopRuntimeSession`
- Traits: `idempotency-token` (3), `idempotent` (3)
- Common required input members in this group: `agentRuntimeArn`, `browserIdentifier`, `codeInterpreterIdentifier`, `runtimeSessionId`, `sessionId`

### Delete

- Operations: `DeleteEvent`, `DeleteMemoryRecord`
- Common required input members in this group: `actorId`, `eventId`, `memoryId`, `memoryRecordId`, `sessionId`

### Invoke

- Operations: `InvokeAgentRuntime`, `InvokeCodeInterpreter`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `agentRuntimeArn`, `codeInterpreterIdentifier`, `name`, `payload`

### Complete

- Operations: `CompleteResourceTokenAuth`
- Common required input members in this group: `sessionUri`, `userIdentifier`

### Create

- Operations: `CreateEvent`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `actorId`, `eventTimestamp`, `memoryId`, `payload`

### Evaluate

- Operations: `Evaluate`
- Common required input members in this group: `evaluationInput`, `evaluatorId`

### Retrieve

- Operations: `RetrieveMemoryRecords`
- Traits: `paginated` (1)
- Common required input members in this group: `memoryId`, `namespace`, `searchCriteria`

### Save

- Operations: `SaveBrowserSessionProfile`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `browserIdentifier`, `profileIdentifier`, `sessionId`

### Update

- Operations: `UpdateBrowserStream`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `browserIdentifier`, `sessionId`, `streamUpdate`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchCreateMemoryRecords` | `POST /memories/{memoryId}/memoryRecords/batchCreate` | `idempotent`, `idempotency-token` | `memoryId`, `records` | `clientToken` | `BatchCreateMemoryRecordsOutput` | `AccessDeniedException`, `ResourceNotFoundException`, `ServiceException`, `ServiceQuotaExceededException`, `ThrottledException`, `ValidationException` | Creates multiple memory records in a single batch operation for the specified memory with custom content. |
| `BatchDeleteMemoryRecords` | `POST /memories/{memoryId}/memoryRecords/batchDelete` | - | `memoryId`, `records` | - | `BatchDeleteMemoryRecordsOutput` | `AccessDeniedException`, `ResourceNotFoundException`, `ServiceException`, `ServiceQuotaExceededException`, `ThrottledException`, `ValidationException` | Deletes multiple memory records in a single batch operation from the specified memory. |
| `BatchUpdateMemoryRecords` | `POST /memories/{memoryId}/memoryRecords/batchUpdate` | - | `memoryId`, `records` | - | `BatchUpdateMemoryRecordsOutput` | `AccessDeniedException`, `ResourceNotFoundException`, `ServiceException`, `ServiceQuotaExceededException`, `ThrottledException`, `ValidationException` | Updates multiple memory records with custom content in a single batch operation within the specified memory. |
| `CompleteResourceTokenAuth` | `POST /identities/CompleteResourceTokenAuth` | - | `sessionUri`, `userIdentifier` | - | `CompleteResourceTokenAuthResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Confirms the user authentication session for obtaining OAuth2.0 tokens for a resource. |
| `CreateEvent` | `POST /memories/{memoryId}/events` | `idempotent`, `idempotency-token` | `actorId`, `eventTimestamp`, `memoryId`, `payload` | `clientToken` | `CreateEventOutput` | `AccessDeniedException`, `InvalidInputException`, `ResourceNotFoundException`, `RetryableConflictException`, `ServiceException`, `ServiceQuotaExceededException`, `ThrottledException`, `ValidationException` | Creates an event in an AgentCore Memory resource. Events represent interactions or activities that occur within a session and are associated with specific actors. |
| `DeleteEvent` | `DELETE /memories/{memoryId}/actor/{actorId}/sessions/{sessionId}/events/{eventId}` | - | `actorId`, `eventId`, `memoryId`, `sessionId` | - | `DeleteEventOutput` | `AccessDeniedException`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceException`, `ServiceQuotaExceededException`, `ThrottledException`, `ValidationException` | Deletes an event from an AgentCore Memory resource. When you delete an event, it is permanently removed. |
| `DeleteMemoryRecord` | `DELETE /memories/{memoryId}/memoryRecords/{memoryRecordId}` | - | `memoryId`, `memoryRecordId` | - | `DeleteMemoryRecordOutput` | `AccessDeniedException`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceException`, `ServiceQuotaExceededException`, `ThrottledException`, `ValidationException` | Deletes a memory record from an AgentCore Memory resource. When you delete a memory record, it is permanently removed. |
| `Evaluate` | `POST /evaluations/evaluate/{evaluatorId}` | - | `evaluationInput`, `evaluatorId` | - | `EvaluateResponse` | `AccessDeniedException`, `ConflictException`, `DuplicateIdException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, ... (+1) | Performs on-demand evaluation of agent traces using a specified evaluator. This synchronous API accepts traces in OpenTelemetry format and returns immediate scoring results with detailed explanations. |
| `GetAgentCard` | `GET /runtimes/{agentRuntimeArn}/invocations/.well-known/agent-card.json` | `idempotency-token` | `agentRuntimeArn` | `runtimeSessionId` | `GetAgentCardResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `RuntimeClientError`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves the A2A agent card associated with an AgentCore Runtime agent. |
| `GetBrowserSession` | `GET /browsers/{browserIdentifier}/sessions/get` | `readonly` | `browserIdentifier`, `sessionId` | - | `GetBrowserSessionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a specific browser session in Amazon Bedrock AgentCore. This operation returns the session's configuration, current status, associated streams, and metadata. |
| `GetCodeInterpreterSession` | `GET /code-interpreters/{codeInterpreterIdentifier}/sessions/get` | `readonly` | `codeInterpreterIdentifier`, `sessionId` | - | `GetCodeInterpreterSessionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a specific code interpreter session in Amazon Bedrock AgentCore. This operation returns the session's configuration, current status, and metadata. |
| `GetEvent` | `GET /memories/{memoryId}/actor/{actorId}/sessions/{sessionId}/events/{eventId}` | `readonly` | `actorId`, `eventId`, `memoryId`, `sessionId` | - | `GetEventOutput` | `AccessDeniedException`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceException`, `ServiceQuotaExceededException`, `ThrottledException`, `ValidationException` | Retrieves information about a specific event in an AgentCore Memory resource. To use this operation, you must have the `bedrock-agentcore:GetEvent` permission. |
| `GetMemoryRecord` | `GET /memories/{memoryId}/memoryRecord/{memoryRecordId}` | `readonly` | `memoryId`, `memoryRecordId` | - | `GetMemoryRecordOutput` | `AccessDeniedException`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceException`, `ServiceQuotaExceededException`, `ThrottledException`, `ValidationException` | Retrieves a specific memory record from an AgentCore Memory resource. To use this operation, you must have the `bedrock-agentcore:GetMemoryRecord` permission. |
| `GetResourceApiKey` | `POST /identities/api-key` | `readonly` | `resourceCredentialProviderName`, `workloadIdentityToken` | - | `GetResourceApiKeyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Retrieves the API key associated with an API key credential provider. |
| `GetResourceOauth2Token` | `POST /identities/oauth2/token` | `readonly` | `oauth2Flow`, `resourceCredentialProviderName`, `scopes`, `workloadIdentityToken` | - | `GetResourceOauth2TokenResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Returns the OAuth 2.0 token of the provided resource. |
| `GetWorkloadAccessToken` | `POST /identities/GetWorkloadAccessToken` | - | `workloadName` | - | `GetWorkloadAccessTokenResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Obtains a workload access token for agentic workloads not acting on behalf of a user. |
| `GetWorkloadAccessTokenForJWT` | `POST /identities/GetWorkloadAccessTokenForJWT` | - | `userToken`, `workloadName` | - | `GetWorkloadAccessTokenForJWTResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Obtains a workload access token for agentic workloads acting on behalf of a user, using a JWT token. |
| `GetWorkloadAccessTokenForUserId` | `POST /identities/GetWorkloadAccessTokenForUserId` | - | `userId`, `workloadName` | - | `GetWorkloadAccessTokenForUserIdResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Obtains a workload access token for agentic workloads acting on behalf of a user, using the user's ID. |
| `InvokeAgentRuntime` | `POST /runtimes/{agentRuntimeArn}/invocations` | `idempotency-token` | `agentRuntimeArn`, `payload` | `runtimeSessionId` | `InvokeAgentRuntimeResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `RuntimeClientError`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Sends a request to an agent or tool hosted in an Amazon Bedrock AgentCore Runtime and receives responses in real-time. To invoke an agent you must specify the AgentCore Runtime ARN and provide a payload containing your request. |
| `InvokeCodeInterpreter` | `POST /code-interpreters/{codeInterpreterIdentifier}/tools/invoke` | - | `codeInterpreterIdentifier`, `name` | - | `InvokeCodeInterpreterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Executes code within an active code interpreter session in Amazon Bedrock AgentCore. This operation processes the provided code, runs it in a secure environment, and returns the execution results including output, errors, and generated visualizations. |
| `ListActors` | `POST /memories/{memoryId}/actors` | `readonly`, `paginated` | `memoryId` | - | `ListActorsOutput` | `AccessDeniedException`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceException`, `ServiceQuotaExceededException`, `ThrottledException`, `ValidationException` | Lists all actors in an AgentCore Memory resource. We recommend using pagination to ensure that the operation returns quickly and successfully. |
| `ListBrowserSessions` | `POST /browsers/{browserIdentifier}/sessions/list` | `readonly` | `browserIdentifier` | - | `ListBrowserSessionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of browser sessions in Amazon Bedrock AgentCore that match the specified criteria. This operation returns summary information about each session, including identifiers, status, and timestamps. |
| `ListCodeInterpreterSessions` | `POST /code-interpreters/{codeInterpreterIdentifier}/sessions/list` | `readonly` | `codeInterpreterIdentifier` | - | `ListCodeInterpreterSessionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of code interpreter sessions in Amazon Bedrock AgentCore that match the specified criteria. This operation returns summary information about each session, including identifiers, status, and timestamps. |
| `ListEvents` | `POST /memories/{memoryId}/actor/{actorId}/sessions/{sessionId}` | `readonly`, `paginated` | `actorId`, `memoryId`, `sessionId` | - | `ListEventsOutput` | `AccessDeniedException`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceException`, `ServiceQuotaExceededException`, `ThrottledException`, `ValidationException` | Lists events in an AgentCore Memory resource based on specified criteria. We recommend using pagination to ensure that the operation returns quickly and successfully. |
| `ListMemoryExtractionJobs` | `POST /memories/{memoryId}/extractionJobs` | `paginated` | `memoryId` | - | `ListMemoryExtractionJobsOutput` | `AccessDeniedException`, `ResourceNotFoundException`, `ServiceException`, `ServiceQuotaExceededException`, `ThrottledException`, `ValidationException` | Lists all long-term memory extraction jobs that are eligible to be started with optional filtering. To use this operation, you must have the `bedrock-agentcore:ListMemoryExtractionJobs` permission. |
| `ListMemoryRecords` | `POST /memories/{memoryId}/memoryRecords` | `readonly`, `paginated` | `memoryId`, `namespace` | - | `ListMemoryRecordsOutput` | `AccessDeniedException`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceException`, `ServiceQuotaExceededException`, `ThrottledException`, `ValidationException` | Lists memory records in an AgentCore Memory resource based on specified criteria. We recommend using pagination to ensure that the operation returns quickly and successfully. |
| `ListSessions` | `POST /memories/{memoryId}/actor/{actorId}/sessions` | `readonly`, `paginated` | `actorId`, `memoryId` | - | `ListSessionsOutput` | `AccessDeniedException`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceException`, `ServiceQuotaExceededException`, `ThrottledException`, `ValidationException` | Lists sessions in an AgentCore Memory resource based on specified criteria. We recommend using pagination to ensure that the operation returns quickly and successfully. |
| `RetrieveMemoryRecords` | `POST /memories/{memoryId}/retrieve` | `paginated` | `memoryId`, `namespace`, `searchCriteria` | - | `RetrieveMemoryRecordsOutput` | `AccessDeniedException`, `InvalidInputException`, `ResourceNotFoundException`, `ServiceException`, `ServiceQuotaExceededException`, `ThrottledException`, `ValidationException` | Searches for and retrieves memory records from an AgentCore Memory resource based on specified search criteria. We recommend using pagination to ensure that the operation returns quickly and successfully. |
| `SaveBrowserSessionProfile` | `PUT /browser-profiles/{profileIdentifier}/save` | `idempotent`, `idempotency-token` | `browserIdentifier`, `profileIdentifier`, `sessionId` | `clientToken` | `SaveBrowserSessionProfileResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Saves the current state of a browser session as a reusable profile in Amazon Bedrock AgentCore. A browser profile captures persistent browser data such as cookies and local storage from an active session, enabling you to reuse this data in future browser... |
| `StartBrowserSession` | `PUT /browsers/{browserIdentifier}/sessions/start` | `idempotent`, `idempotency-token` | `browserIdentifier` | `clientToken` | `StartBrowserSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates and initializes a browser session in Amazon Bedrock AgentCore. The session enables agents to navigate and interact with web content, extract information from websites, and perform web-based tasks as part of their response generation. |
| `StartCodeInterpreterSession` | `PUT /code-interpreters/{codeInterpreterIdentifier}/sessions/start` | `idempotent`, `idempotency-token` | `codeInterpreterIdentifier` | `clientToken` | `StartCodeInterpreterSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates and initializes a code interpreter session in Amazon Bedrock AgentCore. The session enables agents to execute code as part of their response generation, supporting programming languages such as Python for data analysis, visualization, and computation... |
| `StartMemoryExtractionJob` | `POST /memories/{memoryId}/extractionJobs/start` | `idempotent`, `idempotency-token` | `extractionJob`, `memoryId` | `clientToken` | `StartMemoryExtractionJobOutput` | `AccessDeniedException`, `ResourceNotFoundException`, `ServiceException`, `ServiceQuotaExceededException`, `ThrottledException`, `ValidationException` | Starts a memory extraction job that processes events that failed extraction previously in an AgentCore Memory resource and produces structured memory records. When earlier extraction attempts have left events unprocessed, this job will pick up and extract... |
| `StopBrowserSession` | `PUT /browsers/{browserIdentifier}/sessions/stop` | `idempotent`, `idempotency-token` | `browserIdentifier`, `sessionId` | `clientToken` | `StopBrowserSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Terminates an active browser session in Amazon Bedrock AgentCore. This operation stops the session, releases associated resources, and makes the session unavailable for further use. |
| `StopCodeInterpreterSession` | `PUT /code-interpreters/{codeInterpreterIdentifier}/sessions/stop` | `idempotent`, `idempotency-token` | `codeInterpreterIdentifier`, `sessionId` | `clientToken` | `StopCodeInterpreterSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Terminates an active code interpreter session in Amazon Bedrock AgentCore. This operation stops the session, releases associated resources, and makes the session unavailable for further use. |
| `StopRuntimeSession` | `POST /runtimes/{agentRuntimeArn}/stopruntimesession` | `idempotent`, `idempotency-token` | `agentRuntimeArn`, `runtimeSessionId` | `clientToken` | `StopRuntimeSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `RuntimeClientError`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, ... (+1) | Stops a session that is running in an running AgentCore Runtime agent. |
| `UpdateBrowserStream` | `PUT /browsers/{browserIdentifier}/sessions/streams/update` | `idempotent`, `idempotency-token` | `browserIdentifier`, `sessionId`, `streamUpdate` | `clientToken` | `UpdateBrowserStreamResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates a browser stream. To use this operation, you must have permissions to perform the bedrock:UpdateBrowserStream action. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | The exception that occurs when you do not have sufficient permissions to perform an action. |
| `ResourceNotFoundException` | `structure` | `message` | The exception that occurs when the specified resource does not exist. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | The exception that occurs when the input fails to satisfy the constraints specified by the service. |
| `ServiceQuotaExceededException` | `structure` | `message` | The exception that occurs when the request would cause a service quota to be exceeded. |
| `InternalServerException` | `structure` | `message` | The exception that occurs when the service encounters an unexpected internal error. |
| `ThrottlingException` | `structure` | `message` | The exception that occurs when the request was denied due to request throttling. |
| `ServiceException` | `structure` | `message` | The service encountered an internal error. |
| `ThrottledException` | `structure` | `message` | The request was denied due to request throttling. |
| `InvalidInputException` | `structure` | `message` | The input fails to satisfy the constraints specified by AgentCore. |
| `ConflictException` | `structure` | `message` | The exception that occurs when the request conflicts with the current state of the resource. |
| `UnauthorizedException` | `structure` | `message` | This exception is thrown when the JWT bearer token is invalid or not found for OAuth bearer token based access |
| `RuntimeClientError` | `structure` | `message` | The exception that occurs when there is an error in the runtime client. |
| `BatchCreateMemoryRecordsInput` | `structure` | `clientToken`, `memoryId`, `records` | - |
| `BatchCreateMemoryRecordsOutput` | `structure` | `failedRecords`, `successfulRecords` | - |
| `BatchDeleteMemoryRecordsInput` | `structure` | `memoryId`, `records` | - |
| `BatchDeleteMemoryRecordsOutput` | `structure` | `failedRecords`, `successfulRecords` | - |
| `BatchUpdateMemoryRecordsInput` | `structure` | `memoryId`, `records` | - |
| `BatchUpdateMemoryRecordsOutput` | `structure` | `failedRecords`, `successfulRecords` | - |
| `CompleteResourceTokenAuthRequest` | `structure` | `sessionUri`, `userIdentifier` | - |
| `CompleteResourceTokenAuthResponse` | `structure` | - | - |
| `CreateEventInput` | `structure` | `actorId`, `branch`, `clientToken`, `eventTimestamp`, `memoryId`, `metadata`, `payload`, `sessionId` | - |
| `CreateEventOutput` | `structure` | `event` | - |
| `RetryableConflictException` | `structure` | `message` | The exception that occurs when there is a retryable conflict performing an operation. |
| `DeleteEventInput` | `structure` | `actorId`, `eventId`, `memoryId`, `sessionId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
