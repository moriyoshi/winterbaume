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

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | The request is denied because of missing access permissions. Check your permissions and retry your request. |
| `BadGatewayException` | `structure` | message, resourceName | There was an issue with a dependency due to a server issue. Retry your request. |
| `ConflictException` | `structure` | message | There was a conflict performing an operation. Resolve the conflict and retry your request. |
| `DependencyFailedException` | `structure` | message, resourceName | There was an issue with a dependency. Check the resource configurations and retry the request. |
| `InternalServerException` | `structure` | message, reason | An internal server error occurred. Retry your request. |
| `ModelNotReadyException` | `structure` | message | The model specified in the request is not ready to serve inference requests. The AWS SDK will automatically retry the operation up to 5 times. For informati ... |
| `ResourceNotFoundException` | `structure` | message | The specified resource Amazon Resource Name (ARN) was not found. Check the Amazon Resource Name (ARN) and try your request again. |
| `ServiceQuotaExceededException` | `structure` | message | The number of requests exceeds the service quota. Resubmit your request later. |
| `ThrottlingException` | `structure` | message | The number of requests exceeds the limit. Resubmit your request later. |
| `ValidationException` | `structure` | message | Input validation failed. Check your request parameters and retry the request. |
| `ActionGroupSignature` | `enum` | AMAZON_USERINPUT, AMAZON_CODEINTERPRETER, ANTHROPIC_COMPUTER, ANTHROPIC_BASH, ANTHROPIC_TEXTEDITOR | - |
| `ActionInvocationType` | `enum` | RESULT, USER_CONFIRMATION, USER_CONFIRMATION_AND_RESULT | - |
| `AgentCollaboration` | `enum` | SUPERVISOR, SUPERVISOR_ROUTER, DISABLED | - |
| `AttributeType` | `enum` | STRING, NUMBER, BOOLEAN, STRING_LIST | - |
| `ConfirmationState` | `enum` | CONFIRM, DENY | - |
| `ConversationRole` | `enum` | USER, ASSISTANT | - |
| `CreationMode` | `enum` | DEFAULT, OVERRIDDEN | - |
| `CustomControlMethod` | `enum` | RETURN_CONTROL | - |
| `ExecutionType` | `enum` | LAMBDA, RETURN_CONTROL | - |
| `ExternalSourceType` | `enum` | S3, BYTE_CONTENT | - |
| `FileSourceType` | `enum` | S3, BYTE_CONTENT | - |
| `FileUseCase` | `enum` | CODE_INTERPRETER, CHAT | - |
| `FlowCompletionReason` | `enum` | SUCCESS, INPUT_REQUIRED | - |
| `FlowControlNodeType` | `enum` | ITERATOR, LOOP | - |
| `FlowErrorCode` | `enum` | VALIDATION, INTERNAL_SERVER, NODE_EXECUTION_FAILED | - |
| `FlowExecutionErrorType` | `enum` | TIMED_OUT | - |
| `FlowExecutionEventType` | `enum` | NODE, FLOW | - |
| `FlowExecutionStatus` | `enum` | RUNNING, SUCCEEDED, FAILED, TIMED_OUT, ABORTED | - |
| `FlowNodeIODataType` | `enum` | STRING, NUMBER, BOOLEAN, OBJECT, ARRAY | - |
| `FlowNodeInputCategory` | `enum` | LOOP_CONDITION, RETURN_VALUE_TO_LOOP_START, EXIT_LOOP | - |
| `GeneratedQueryType` | `enum` | REDSHIFT_SQL | - |
| `GuadrailAction` | `enum` | INTERVENED, NONE | - |
| `GuardrailAction` | `enum` | INTERVENED, NONE | - |
| `GuardrailContentFilterConfidence` | `enum` | NONE, LOW, MEDIUM, HIGH | - |
| `GuardrailContentFilterType` | `enum` | INSULTS, HATE, SEXUAL, VIOLENCE, MISCONDUCT, PROMPT_ATTACK | - |
| `GuardrailContentPolicyAction` | `enum` | BLOCKED | - |
| `GuardrailManagedWordType` | `enum` | PROFANITY | - |
| `GuardrailPiiEntityType` | `enum` | ADDRESS, AGE, AWS_ACCESS_KEY, AWS_SECRET_KEY, CA_HEALTH_NUMBER, CA_SOCIAL_INSURANCE_NUMBER, CREDIT_DEBIT_CARD_CVV, CREDIT_DEBIT_CARD_EXPIRY, CREDIT_DEBIT_CARD_NUMBER, DRIVER_ID, EMAIL, INTERNATIONAL_BANK_ACCOUNT_NUMBER, ... (+19) | - |
| `GuardrailSensitiveInformationPolicyAction` | `enum` | BLOCKED, ANONYMIZED | - |
| `GuardrailTopicPolicyAction` | `enum` | BLOCKED | - |
| `GuardrailTopicType` | `enum` | DENY | - |
| `GuardrailWordPolicyAction` | `enum` | BLOCKED | - |
| `ImageFormat` | `enum` | PNG, JPEG, GIF, WEBP | - |
| `ImageInputFormat` | `enum` | PNG, JPEG, GIF, WEBP | - |
| `InputImageFormat` | `enum` | png, jpeg, gif, webp | - |
| `InputQueryType` | `enum` | TEXT | - |
| `InvocationType` | `enum` | ACTION_GROUP, KNOWLEDGE_BASE, FINISH, ACTION_GROUP_CODE_INTERPRETER, AGENT_COLLABORATOR | - |
| `KnowledgeBaseQueryType` | `enum` | TEXT, IMAGE | - |
| `MemoryType` | `enum` | SESSION_SUMMARY | - |
| `NodeErrorCode` | `enum` | VALIDATION, DEPENDENCY_FAILED, BAD_GATEWAY, INTERNAL_SERVER | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
