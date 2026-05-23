# Agents for Amazon Bedrock Runtime

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Contains APIs related to model invocation and querying of knowledge bases.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Agents for Amazon Bedrock Runtime resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: invoke Bedrock agents, retrieve and generate responses, manage sessions, and interact with knowledge-base retrieval APIs.
- From the operation surface: model conversational agent runtime flows, trace/session context, action invocation, streaming responses, and retrieval-augmented generation.

## Service Identity and Protocol

- AWS model slug: `bedrock-agent-runtime`
- AWS SDK for Rust slug: `bedrock`
- Model version: `2023-07-26`
- Model file: `vendor/api-models-aws/models/bedrock-agent-runtime/service/2023-07-26/bedrock-agent-runtime-2023-07-26.json`
- SDK ID: `Bedrock Agent Runtime`
- Endpoint prefix: `bedrock-agent-runtime`
- ARN namespace: `bedrock`
- CloudFormation name: `-`
- CloudTrail event source: `bedrock.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (6), `Get` (5), `Invoke` (3), `Retrieve` (3), `Create` (2), `Delete` (2), `End` (1), `Generate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateInvocation`, `CreateSession`, `DeleteAgentMemory`, `DeleteSession`, `PutInvocationStep`, `StartFlowExecution`, `StopFlowExecution`, `TagResource`, `UntagResource`, `UpdateSession`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GenerateQuery`, `GetAgentMemory`, `GetExecutionFlowSnapshot`, `GetFlowExecution`, `GetInvocationStep`, `GetSession`, `ListFlowExecutionEvents`, `ListFlowExecutions`, `ListInvocationSteps`, `ListInvocations`, `ListSessions`, `ListTagsForResource`, `Retrieve`.
- Pagination is modelled for 8 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 8 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetExecutionFlowSnapshot`, `GetFlowExecution`, `ListFlowExecutionEvents`, `ListFlowExecutions`, `StartFlowExecution`, `StopFlowExecution`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 31 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `SNS`, `Lambda`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `FlowExecutionResource` | - | - | `GetExecutionFlowSnapshot`, `GetFlowExecution`, `ListFlowExecutionEvents`, `ListFlowExecutions`, `StartFlowExecution`, `StopFlowExecution` | - |
| `FlowResource` | - | - | `InvokeFlow` | - |
| `GenerateQueryResource` | - | - | `GenerateQuery` | - |
| `InferenceResource` | - | - | `InvokeAgent` | - |
| `InlineAgentResource` | - | - | `InvokeInlineAgent` | - |
| `InvocationResource` | `invocationIdentifier`, `sessionIdentifier` | create: `CreateInvocation`; list: `ListInvocations` | - | - |
| `InvocationStepResource` | `invocationIdentifier`, `invocationStepId`, `sessionIdentifier` | create: `PutInvocationStep`; read: `GetInvocationStep`; list: `ListInvocationSteps` | - | - |
| `MemoryResource` | - | - | `DeleteAgentMemory`, `GetAgentMemory` | - |
| `OptimizePromptResource` | - | - | `OptimizePrompt` | - |
| `RerankResource` | - | - | `Rerank` | - |
| `RetrieveAndGenerateResource` | - | - | `RetrieveAndGenerate` | - |
| `RetrieveAndGenerateStreamResource` | - | - | `RetrieveAndGenerateStream` | - |
| `RetrieveResource` | - | - | `Retrieve` | - |
| `SessionResource` | `sessionIdentifier` | create: `CreateSession`; read: `GetSession`; update: `UpdateSession`; delete: `DeleteSession`; list: `ListSessions` | `EndSession` | - |
| `TaggingResource` | - | - | `ListTagsForResource`, `TagResource`, `UntagResource` | - |
## Operation Groups

### List

- Operations: `ListFlowExecutionEvents`, `ListFlowExecutions`, `ListInvocationSteps`, `ListInvocations`, `ListSessions`, `ListTagsForResource`
- Traits: `paginated` (5), `readonly` (6)
- Common required input members in this group: `eventType`, `executionIdentifier`, `flowAliasIdentifier`, `flowIdentifier`, `resourceArn`, `sessionIdentifier`

### Get

- Operations: `GetAgentMemory`, `GetExecutionFlowSnapshot`, `GetFlowExecution`, `GetInvocationStep`, `GetSession`
- Traits: `paginated` (1), `readonly` (5)
- Common required input members in this group: `agentAliasId`, `agentId`, `executionIdentifier`, `flowAliasIdentifier`, `flowIdentifier`, `invocationIdentifier`, `invocationStepId`, `memoryId`, `memoryType`, `sessionIdentifier`

### Invoke

- Operations: `InvokeAgent`, `InvokeFlow`, `InvokeInlineAgent`
- Common required input members in this group: `agentAliasId`, `agentId`, `flowAliasIdentifier`, `flowIdentifier`, `foundationModel`, `inputs`, `instruction`, `sessionId`

### Retrieve

- Operations: `Retrieve`, `RetrieveAndGenerate`, `RetrieveAndGenerateStream`
- Traits: `paginated` (1), `readonly` (1)
- Common required input members in this group: `input`, `knowledgeBaseId`, `retrievalQuery`

### Create

- Operations: `CreateInvocation`, `CreateSession`
- Traits: `idempotent` (2)
- Common required input members in this group: `sessionIdentifier`

### Delete

- Operations: `DeleteAgentMemory`, `DeleteSession`
- Traits: `idempotent` (2)
- Common required input members in this group: `agentAliasId`, `agentId`, `sessionIdentifier`

### End

- Operations: `EndSession`
- Traits: `idempotent` (1)
- Common required input members in this group: `sessionIdentifier`

### Generate

- Operations: `GenerateQuery`
- Traits: `readonly` (1)
- Common required input members in this group: `queryGenerationInput`, `transformationConfiguration`

### Optimize

- Operations: `OptimizePrompt`
- Common required input members in this group: `input`, `targetModelId`

### Put

- Operations: `PutInvocationStep`
- Traits: `idempotent` (1)
- Common required input members in this group: `invocationIdentifier`, `invocationStepTime`, `payload`, `sessionIdentifier`

### Rerank

- Operations: `Rerank`
- Traits: `paginated` (1)
- Common required input members in this group: `queries`, `rerankingConfiguration`, `sources`

### Start

- Operations: `StartFlowExecution`
- Common required input members in this group: `flowAliasIdentifier`, `flowIdentifier`, `inputs`

### Stop

- Operations: `StopFlowExecution`
- Common required input members in this group: `executionIdentifier`, `flowAliasIdentifier`, `flowIdentifier`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateSession`
- Traits: `idempotent` (1)
- Common required input members in this group: `sessionIdentifier`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateInvocation` | `PUT /sessions/{sessionIdentifier}/invocations/` | `idempotent` | `sessionIdentifier` | - | `CreateInvocationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new invocation within a session. An invocation groups the related invocation steps that store the content from a conversation. |
| `CreateSession` | `PUT /sessions/` | `idempotent` | - | - | `CreateSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a session to temporarily store conversations for generative AI (GenAI) applications built with open-source frameworks such as LangGraph and LlamaIndex. Sessions enable you to save the state of conversations at checkpoints, with the added security and... |
| `DeleteAgentMemory` | `DELETE /agents/{agentId}/agentAliases/{agentAliasId}/memories` | `idempotent` | `agentAliasId`, `agentId` | - | `DeleteAgentMemoryResponse` | `AccessDeniedException`, `BadGatewayException`, `ConflictException`, `DependencyFailedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, ... (+1) | Deletes memory from the specified memory identifier. |
| `DeleteSession` | `DELETE /sessions/{sessionIdentifier}/` | `idempotent` | `sessionIdentifier` | - | `DeleteSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a session that you ended. You can't delete a session with an `ACTIVE` status. |
| `EndSession` | `PATCH /sessions/{sessionIdentifier}` | `idempotent` | `sessionIdentifier` | - | `EndSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Ends the session. After you end a session, you can still access its content but you can’t add to it. |
| `GenerateQuery` | `POST /generateQuery` | `readonly` | `queryGenerationInput`, `transformationConfiguration` | - | `GenerateQueryResponse` | `AccessDeniedException`, `BadGatewayException`, `ConflictException`, `DependencyFailedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, ... (+1) | Generates an SQL query from a natural language query. For more information, see Generate a query for structured data in the Amazon Bedrock User Guide. |
| `GetAgentMemory` | `GET /agents/{agentId}/agentAliases/{agentAliasId}/memories` | `readonly`, `paginated` | `agentAliasId`, `agentId`, `memoryId`, `memoryType` | - | `GetAgentMemoryResponse` | `AccessDeniedException`, `BadGatewayException`, `ConflictException`, `DependencyFailedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, ... (+1) | Gets the sessions stored in the memory of the agent. |
| `GetExecutionFlowSnapshot` | `GET /flows/{flowIdentifier}/aliases/{flowAliasIdentifier}/executions/{executionIdentifier}/flowsnapshot` | `readonly` | `executionIdentifier`, `flowAliasIdentifier`, `flowIdentifier` | - | `GetExecutionFlowSnapshotResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the flow definition snapshot used for a flow execution. The snapshot represents the flow metadata and definition as it existed at the time the execution was started. |
| `GetFlowExecution` | `GET /flows/{flowIdentifier}/aliases/{flowAliasIdentifier}/executions/{executionIdentifier}` | `readonly` | `executionIdentifier`, `flowAliasIdentifier`, `flowIdentifier` | - | `GetFlowExecutionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves details about a specific flow execution, including its status, start and end times, and any errors that occurred during execution. |
| `GetInvocationStep` | `POST /sessions/{sessionIdentifier}/invocationSteps/{invocationStepId}` | `readonly` | `invocationIdentifier`, `invocationStepId`, `sessionIdentifier` | - | `GetInvocationStepResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the details of a specific invocation step within an invocation in a session. For more information about sessions, see Store and retrieve conversation history and context with Amazon Bedrock sessions. |
| `GetSession` | `GET /sessions/{sessionIdentifier}/` | `readonly` | `sessionIdentifier` | - | `GetSessionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves details about a specific session. For more information about sessions, see Store and retrieve conversation history and context with Amazon Bedrock sessions. |
| `InvokeAgent` | `POST /agents/{agentId}/agentAliases/{agentAliasId}/sessions/{sessionId}/text` | - | `agentAliasId`, `agentId`, `sessionId` | - | `InvokeAgentResponse` | `AccessDeniedException`, `BadGatewayException`, `ConflictException`, `DependencyFailedException`, `InternalServerException`, `ModelNotReadyException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, ... (+2) | Sends a prompt for the agent to process and respond to. Note the following fields for the request: To continue the same conversation with an agent, use the same `sessionId` value in the request. |
| `InvokeFlow` | `POST /flows/{flowIdentifier}/aliases/{flowAliasIdentifier}` | - | `flowAliasIdentifier`, `flowIdentifier`, `inputs` | - | `InvokeFlowResponse` | `AccessDeniedException`, `BadGatewayException`, `ConflictException`, `DependencyFailedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, ... (+1) | Invokes an alias of a flow to run the inputs that you specify and return the output of each node as a stream. If there's an error, the error is returned. |
| `InvokeInlineAgent` | `POST /agents/{sessionId}` | - | `foundationModel`, `instruction`, `sessionId` | - | `InvokeInlineAgentResponse` | `AccessDeniedException`, `BadGatewayException`, `ConflictException`, `DependencyFailedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, ... (+1) | Invokes an inline Amazon Bedrock agent using the configurations you provide with the request. Specify the following fields for security purposes. |
| `ListFlowExecutionEvents` | `GET /flows/{flowIdentifier}/aliases/{flowAliasIdentifier}/executions/{executionIdentifier}/events` | `readonly`, `paginated` | `eventType`, `executionIdentifier`, `flowAliasIdentifier`, `flowIdentifier` | - | `ListFlowExecutionEventsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists events that occurred during a flow execution. Events provide detailed information about the execution progress, including node inputs and outputs, flow inputs and outputs, condition results, and failure events. |
| `ListFlowExecutions` | `GET /flows/{flowIdentifier}/executions` | `readonly`, `paginated` | `flowIdentifier` | - | `ListFlowExecutionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all executions of a flow. Results can be paginated and include summary information about each execution, such as status, start and end times, and the execution's Amazon Resource Name (ARN). |
| `ListInvocationSteps` | `POST /sessions/{sessionIdentifier}/invocationSteps/` | `readonly`, `paginated` | `sessionIdentifier` | - | `ListInvocationStepsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all invocation steps associated with a session and optionally, an invocation within the session. For more information about sessions, see Store and retrieve conversation history and context with Amazon Bedrock sessions. |
| `ListInvocations` | `POST /sessions/{sessionIdentifier}/invocations/` | `readonly`, `paginated` | `sessionIdentifier` | - | `ListInvocationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all invocations associated with a specific session. For more information about sessions, see Store and retrieve conversation history and context with Amazon Bedrock sessions. |
| `ListSessions` | `POST /sessions/` | `readonly`, `paginated` | - | - | `ListSessionsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all sessions in your Amazon Web Services account. For more information about sessions, see Store and retrieve conversation history and context with Amazon Bedrock sessions. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List all the tags for the resource you specify. |
| `OptimizePrompt` | `POST /optimize-prompt` | - | `input`, `targetModelId` | - | `OptimizePromptResponse` | `AccessDeniedException`, `BadGatewayException`, `DependencyFailedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Optimizes a prompt for the task that you specify. For more information, see Optimize a prompt in the Amazon Bedrock User Guide. |
| `PutInvocationStep` | `PUT /sessions/{sessionIdentifier}/invocationSteps/` | `idempotent` | `invocationIdentifier`, `invocationStepTime`, `payload`, `sessionIdentifier` | - | `PutInvocationStepResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Add an invocation step to an invocation in a session. An invocation step stores fine-grained state checkpoints, including text and images, for each interaction. |
| `Rerank` | `POST /rerank` | `paginated` | `queries`, `rerankingConfiguration`, `sources` | - | `RerankResponse` | `AccessDeniedException`, `BadGatewayException`, `ConflictException`, `DependencyFailedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, ... (+1) | Reranks the relevance of sources based on queries. For more information, see Improve the relevance of query responses with a reranker model. |
| `Retrieve` | `POST /knowledgebases/{knowledgeBaseId}/retrieve` | `readonly`, `paginated` | `knowledgeBaseId`, `retrievalQuery` | - | `RetrieveResponse` | `AccessDeniedException`, `BadGatewayException`, `ConflictException`, `DependencyFailedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, ... (+1) | Queries a knowledge base and retrieves information from it. |
| `RetrieveAndGenerate` | `POST /retrieveAndGenerate` | - | `input` | - | `RetrieveAndGenerateResponse` | `AccessDeniedException`, `BadGatewayException`, `ConflictException`, `DependencyFailedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, ... (+1) | Queries a knowledge base and generates responses based on the retrieved results and using the specified foundation model or inference profile. The response only cites sources that are relevant to the query. |
| `RetrieveAndGenerateStream` | `POST /retrieveAndGenerateStream` | - | `input` | - | `RetrieveAndGenerateStreamResponse` | `AccessDeniedException`, `BadGatewayException`, `ConflictException`, `DependencyFailedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, ... (+1) | Queries a knowledge base and generates responses based on the retrieved results, with output in streaming format. The CLI doesn't support streaming operations in Amazon Bedrock, including `InvokeModelWithResponseStream`. |
| `StartFlowExecution` | `POST /flows/{flowIdentifier}/aliases/{flowAliasIdentifier}/executions` | - | `flowAliasIdentifier`, `flowIdentifier`, `inputs` | - | `StartFlowExecutionResponse` | `AccessDeniedException`, `BadGatewayException`, `ConflictException`, `DependencyFailedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, ... (+1) | Starts an execution of an Amazon Bedrock flow. Unlike flows that run until completion or time out after five minutes, flow executions let you run flows asynchronously for longer durations. |
| `StopFlowExecution` | `POST /flows/{flowIdentifier}/aliases/{flowAliasIdentifier}/executions/{executionIdentifier}/stop` | - | `executionIdentifier`, `flowAliasIdentifier`, `flowIdentifier` | - | `StopFlowExecutionResponse` | `AccessDeniedException`, `BadGatewayException`, `ConflictException`, `DependencyFailedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops an Amazon Bedrock flow's execution. This operation prevents further processing of the flow and changes the execution status to `Aborted`. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associate tags with a resource. For more information, see Tagging resources in the Amazon Bedrock User Guide. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Remove tags from a resource. |
| `UpdateSession` | `PUT /sessions/{sessionIdentifier}/` | `idempotent` | `sessionIdentifier` | - | `UpdateSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the metadata or encryption settings of a session. For more information about sessions, see Store and retrieve conversation history and context with Amazon Bedrock sessions. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | The request is denied because of missing access permissions. |
| `InternalServerException` | `structure` | `message`, `reason` | An internal server error occurred. |
| `ThrottlingException` | `structure` | `message` | The number of requests exceeds the limit. |
| `ValidationException` | `structure` | `message` | Input validation failed. |
| `ResourceNotFoundException` | `structure` | `message` | The specified resource Amazon Resource Name (ARN) was not found. |
| `ConflictException` | `structure` | `message` | There was a conflict performing an operation. |
| `ServiceQuotaExceededException` | `structure` | `message` | The number of requests exceeds the service quota. |
| `BadGatewayException` | `structure` | `message`, `resourceName` | There was an issue with a dependency due to a server issue. |
| `DependencyFailedException` | `structure` | `message`, `resourceName` | There was an issue with a dependency. |
| `CreateInvocationRequest` | `structure` | `description`, `invocationId`, `sessionIdentifier` | - |
| `CreateInvocationResponse` | `structure` | `createdAt`, `invocationId`, `sessionId` | - |
| `CreateSessionRequest` | `structure` | `encryptionKeyArn`, `sessionMetadata`, `tags` | - |
| `CreateSessionResponse` | `structure` | `createdAt`, `sessionArn`, `sessionId`, `sessionStatus` | - |
| `DeleteAgentMemoryRequest` | `structure` | `agentAliasId`, `agentId`, `memoryId`, `sessionId` | - |
| `DeleteAgentMemoryResponse` | `structure` | - | - |
| `DeleteSessionRequest` | `structure` | `sessionIdentifier` | - |
| `DeleteSessionResponse` | `structure` | - | - |
| `EndSessionRequest` | `structure` | `sessionIdentifier` | - |
| `EndSessionResponse` | `structure` | `sessionArn`, `sessionId`, `sessionStatus` | - |
| `GenerateQueryRequest` | `structure` | `queryGenerationInput`, `transformationConfiguration` | - |
| `GenerateQueryResponse` | `structure` | `queries` | - |
| `GetAgentMemoryRequest` | `structure` | `agentAliasId`, `agentId`, `maxItems`, `memoryId`, `memoryType`, `nextToken` | - |
| `GetAgentMemoryResponse` | `structure` | `memoryContents`, `nextToken` | - |
| `GetExecutionFlowSnapshotRequest` | `structure` | `executionIdentifier`, `flowAliasIdentifier`, `flowIdentifier` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
