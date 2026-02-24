# Amazon Bedrock Runtime

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Describes the API operations for running inference using Amazon Bedrock models.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Bedrock Runtime resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: invoke foundation models, stream responses, apply guardrails, count tokens, and use asynchronous model invocation.
- From the operation surface: model generative AI inference, prompt/response streaming, guarded outputs, request trace metadata, and output retrieval from asynchronous jobs.

## Service Identity and Protocol

- AWS model slug: `bedrock-runtime`
- AWS SDK for Rust slug: `bedrockruntime`
- Model version: `2023-09-30`
- Model file: `vendor/api-models-aws/models/bedrock-runtime/service/2023-09-30/bedrock-runtime-2023-09-30.json`
- SDK ID: `Bedrock Runtime`
- Endpoint prefix: `bedrock-runtime`
- ARN namespace: `-`
- CloudFormation name: `-`
- CloudTrail event source: `bedrock.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Invoke` (3), `Converse` (2), `Apply` (1), `Count` (1), `Get` (1), `List` (1), `Start` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `StartAsyncInvoke`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `CountTokens`, `GetAsyncInvoke`, `ListAsyncInvokes`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartAsyncInvoke`.
- 10 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SNS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AsyncInvokeResource` | - | - | `GetAsyncInvoke`, `ListAsyncInvokes`, `StartAsyncInvoke` | - |
| `GuardrailResource` | - | - | `ApplyGuardrail` | - |
| `InferenceResource` | - | - | `Converse`, `ConverseStream`, `InvokeModel`, `InvokeModelWithBidirectionalStream`, `InvokeModelWithResponseStream` | - |
| `TokenizerResource` | - | - | `CountTokens` | - |
## Operation Groups

### Invoke

- Operations: `InvokeModel`, `InvokeModelWithBidirectionalStream`, `InvokeModelWithResponseStream`
- Common required input members in this group: `body`, `modelId`

### Converse

- Operations: `Converse`, `ConverseStream`
- Common required input members in this group: `modelId`

### Apply

- Operations: `ApplyGuardrail`
- Common required input members in this group: `content`, `guardrailIdentifier`, `guardrailVersion`, `source`

### Count

- Operations: `CountTokens`
- Traits: `readonly` (1)
- Common required input members in this group: `input`, `modelId`

### Get

- Operations: `GetAsyncInvoke`
- Traits: `readonly` (1)
- Common required input members in this group: `invocationArn`

### List

- Operations: `ListAsyncInvokes`
- Traits: `paginated` (1), `readonly` (1)

### Start

- Operations: `StartAsyncInvoke`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `modelId`, `modelInput`, `outputDataConfig`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ApplyGuardrail` | `POST /guardrail/{guardrailIdentifier}/version/{guardrailVersion}/apply` | - | `content`, `guardrailIdentifier`, `guardrailVersion`, `source` | - | `ApplyGuardrailResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | The action to apply a guardrail. For troubleshooting some of the common errors you might encounter when using the `ApplyGuardrail` API, see Troubleshooting Amazon Bedrock API Error Codes in the Amazon Bedrock User Guide |
| `Converse` | `POST /model/{modelId}/converse` | - | `modelId` | - | `ConverseResponse` | `AccessDeniedException`, `InternalServerException`, `ModelErrorException`, `ModelNotReadyException`, `ModelTimeoutException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, ... (+1) | Sends messages to the specified Amazon Bedrock model. `Converse` provides a consistent interface that works with all models that support messages. |
| `ConverseStream` | `POST /model/{modelId}/converse-stream` | - | `modelId` | - | `ConverseStreamResponse` | `AccessDeniedException`, `InternalServerException`, `ModelErrorException`, `ModelNotReadyException`, `ModelTimeoutException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, ... (+1) | Sends messages to the specified Amazon Bedrock model and returns the response in a stream. `ConverseStream` provides a consistent API that works with all Amazon Bedrock models that support messages. |
| `CountTokens` | `POST /model/{modelId}/count-tokens` | `readonly` | `input`, `modelId` | - | `CountTokensResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Returns the token count for a given inference request. This operation helps you estimate token usage before sending requests to foundation models by returning the token count that would be used if the same input were sent to the model in an inference request. |
| `GetAsyncInvoke` | `GET /async-invoke/{invocationArn}` | `readonly` | `invocationArn` | - | `GetAsyncInvokeResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieve information about an asynchronous invocation. |
| `InvokeModel` | `POST /model/{modelId}/invoke` | - | `modelId` | - | `InvokeModelResponse` | `AccessDeniedException`, `InternalServerException`, `ModelErrorException`, `ModelNotReadyException`, `ModelTimeoutException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, ... (+2) | Invokes the specified Amazon Bedrock model to run inference using the prompt and inference parameters provided in the request body. You use model inference to generate text, images, and embeddings. |
| `InvokeModelWithBidirectionalStream` | `POST /model/{modelId}/invoke-with-bidirectional-stream` | - | `body`, `modelId` | - | `InvokeModelWithBidirectionalStreamResponse` | `AccessDeniedException`, `InternalServerException`, `ModelErrorException`, `ModelNotReadyException`, `ModelStreamErrorException`, `ModelTimeoutException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, ... (+3) | Invoke the specified Amazon Bedrock model to run inference using the bidirectional stream. The response is returned in a stream that remains open for 8 minutes. |
| `InvokeModelWithResponseStream` | `POST /model/{modelId}/invoke-with-response-stream` | - | `modelId` | - | `InvokeModelWithResponseStreamResponse` | `AccessDeniedException`, `InternalServerException`, `ModelErrorException`, `ModelNotReadyException`, `ModelStreamErrorException`, `ModelTimeoutException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, ... (+3) | Invoke the specified Amazon Bedrock model to run inference using the prompt and inference parameters provided in the request body. The response is returned in a stream. |
| `ListAsyncInvokes` | `GET /async-invoke` | `readonly`, `paginated` | - | - | `ListAsyncInvokesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists asynchronous invocations. |
| `StartAsyncInvoke` | `POST /async-invoke` | `idempotent`, `idempotency-token` | `modelId`, `modelInput`, `outputDataConfig` | `clientRequestToken` | `StartAsyncInvokeResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Starts an asynchronous invocation. This operation requires permission for the `bedrock:InvokeModel` action. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | The request is denied because you do not have sufficient permissions to perform the requested action. |
| `InternalServerException` | `structure` | `message` | An internal server error occurred. |
| `ThrottlingException` | `structure` | `message` | Your request was denied due to exceeding the account quotas for Amazon Bedrock . |
| `ValidationException` | `structure` | `message` | The input fails to satisfy the constraints specified by Amazon Bedrock . |
| `ResourceNotFoundException` | `structure` | `message` | The specified resource ARN was not found. |
| `ServiceUnavailableException` | `structure` | `message` | The service isn't currently available. |
| `ServiceQuotaExceededException` | `structure` | `message` | Your request exceeds the service quota for your account. |
| `ModelErrorException` | `structure` | `message`, `originalStatusCode`, `resourceName` | The request failed due to an error while processing the model. |
| `ModelNotReadyException` | `structure` | `message` | The model specified in the request is not ready to serve inference requests. |
| `ModelTimeoutException` | `structure` | `message` | The request took too long to process. |
| `ModelStreamErrorException` | `structure` | `message`, `originalMessage`, `originalStatusCode` | An error occurred while streaming the response. |
| `ApplyGuardrailRequest` | `structure` | `content`, `guardrailIdentifier`, `guardrailVersion`, `outputScope`, `source` | - |
| `ApplyGuardrailResponse` | `structure` | `action`, `actionReason`, `assessments`, `guardrailCoverage`, `outputs`, `usage` | - |
| `ConverseRequest` | `structure` | `additionalModelRequestFields`, `additionalModelResponseFieldPaths`, `guardrailConfig`, `inferenceConfig`, `messages`, `modelId`, `outputConfig`, `performanceConfig`, `promptVariables`, `requestMetadata`, `serviceTier`, `system`, ... (+1) | - |
| `ConverseResponse` | `structure` | `additionalModelResponseFields`, `metrics`, `output`, `performanceConfig`, `serviceTier`, `stopReason`, `trace`, `usage` | - |
| `ConverseStreamRequest` | `structure` | `additionalModelRequestFields`, `additionalModelResponseFieldPaths`, `guardrailConfig`, `inferenceConfig`, `messages`, `modelId`, `outputConfig`, `performanceConfig`, `promptVariables`, `requestMetadata`, `serviceTier`, `system`, ... (+1) | - |
| `ConverseStreamResponse` | `structure` | `stream` | - |
| `CountTokensRequest` | `structure` | `input`, `modelId` | - |
| `CountTokensResponse` | `structure` | `inputTokens` | - |
| `GetAsyncInvokeRequest` | `structure` | `invocationArn` | - |
| `GetAsyncInvokeResponse` | `structure` | `clientRequestToken`, `endTime`, `failureMessage`, `invocationArn`, `lastModifiedTime`, `modelArn`, `outputDataConfig`, `status`, `submitTime` | - |
| `InvokeModelRequest` | `structure` | `accept`, `body`, `contentType`, `guardrailIdentifier`, `guardrailVersion`, `modelId`, `performanceConfigLatency`, `serviceTier`, `trace` | - |
| `InvokeModelResponse` | `structure` | `body`, `contentType`, `performanceConfigLatency`, `serviceTier` | - |
| `InvokeModelWithBidirectionalStreamRequest` | `structure` | `body`, `modelId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
