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

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | The request is denied because you do not have sufficient permissions to perform the requested action. For troubleshooting this error, see AccessDeniedExcept ... |
| `ConflictException` | `structure` | message | Error occurred because of a conflict while performing an operation. |
| `InternalServerException` | `structure` | message | An internal server error occurred. For troubleshooting this error, see InternalFailure in the Amazon Bedrock User Guide |
| `ModelErrorException` | `structure` | message, originalStatusCode, resourceName | The request failed due to an error while processing the model. |
| `ModelNotReadyException` | `structure` | message | The model specified in the request is not ready to serve inference requests. The AWS SDK will automatically retry the operation up to 5 times. For informati ... |
| `ModelStreamErrorException` | `structure` | message, originalStatusCode, originalMessage | An error occurred while streaming the response. Retry your request. |
| `ModelTimeoutException` | `structure` | message | The request took too long to process. Processing time exceeded the model timeout length. |
| `ResourceNotFoundException` | `structure` | message | The specified resource ARN was not found. For troubleshooting this error, see ResourceNotFound in the Amazon Bedrock User Guide |
| `ServiceQuotaExceededException` | `structure` | message | Your request exceeds the service quota for your account. You can view your quotas at Viewing service quotas . You can resubmit your request later. |
| `ServiceUnavailableException` | `structure` | message | The service isn't currently available. For troubleshooting this error, see ServiceUnavailable in the Amazon Bedrock User Guide |
| `ThrottlingException` | `structure` | message | Your request was denied due to exceeding the account quotas for Amazon Bedrock . For troubleshooting this error, see ThrottlingException in the Amazon Bedro ... |
| `ValidationException` | `structure` | message | The input fails to satisfy the constraints specified by Amazon Bedrock . For troubleshooting this error, see ValidationError in the Amazon Bedrock User Guide |
| `AsyncInvokeStatus` | `enum` | IN_PROGRESS, COMPLETED, FAILED | - |
| `AudioFormat` | `enum` | MP3, OPUS, WAV, AAC, FLAC, MP4, OGG, MKV, MKA, X_AAC, M4A, MPEG, ... (+3) | - |
| `CachePointType` | `enum` | DEFAULT | - |
| `CacheTTL` | `enum` | FIVE_MINUTES, ONE_HOUR | Time-to-live duration for ephemeral cache entries |
| `ConversationRole` | `enum` | USER, ASSISTANT | - |
| `DocumentFormat` | `enum` | PDF, CSV, DOC, DOCX, XLS, XLSX, HTML, TXT, MD | - |
| `GuardrailAction` | `enum` | NONE, GUARDRAIL_INTERVENED | - |
| `GuardrailAutomatedReasoningLogicWarningType` | `enum` | ALWAYS_FALSE, ALWAYS_TRUE | - |
| `GuardrailContentFilterConfidence` | `enum` | NONE, LOW, MEDIUM, HIGH | - |
| `GuardrailContentFilterStrength` | `enum` | NONE, LOW, MEDIUM, HIGH | - |
| `GuardrailContentFilterType` | `enum` | INSULTS, HATE, SEXUAL, VIOLENCE, MISCONDUCT, PROMPT_ATTACK | - |
| `GuardrailContentPolicyAction` | `enum` | BLOCKED, NONE | - |
| `GuardrailContentQualifier` | `enum` | GROUNDING_SOURCE, QUERY, GUARD_CONTENT | - |
| `GuardrailContentSource` | `enum` | INPUT, OUTPUT | - |
| `GuardrailContextualGroundingFilterType` | `enum` | GROUNDING, RELEVANCE | - |
| `GuardrailContextualGroundingPolicyAction` | `enum` | BLOCKED, NONE | - |
| `GuardrailConverseContentQualifier` | `enum` | GROUNDING_SOURCE, QUERY, GUARD_CONTENT | - |
| `GuardrailConverseImageFormat` | `enum` | PNG, JPEG | - |
| `GuardrailImageFormat` | `enum` | PNG, JPEG | - |
| `GuardrailManagedWordType` | `enum` | PROFANITY | - |
| `GuardrailOrigin` | `enum` | REQUEST, ACCOUNT_ENFORCED, ORGANIZATION_ENFORCED | - |
| `GuardrailOutputScope` | `enum` | INTERVENTIONS, FULL | - |
| `GuardrailOwnership` | `enum` | SELF, CROSS_ACCOUNT | - |
| `GuardrailPiiEntityType` | `enum` | ADDRESS, AGE, AWS_ACCESS_KEY, AWS_SECRET_KEY, CA_HEALTH_NUMBER, CA_SOCIAL_INSURANCE_NUMBER, CREDIT_DEBIT_CARD_CVV, CREDIT_DEBIT_CARD_EXPIRY, CREDIT_DEBIT_CARD_NUMBER, DRIVER_ID, EMAIL, INTERNATIONAL_BANK_ACCOUNT_NUMBER, ... (+19) | - |
| `GuardrailSensitiveInformationPolicyAction` | `enum` | ANONYMIZED, BLOCKED, NONE | - |
| `GuardrailStreamProcessingMode` | `enum` | SYNC, ASYNC | - |
| `GuardrailTopicPolicyAction` | `enum` | BLOCKED, NONE | - |
| `GuardrailTopicType` | `enum` | DENY | - |
| `GuardrailTrace` | `enum` | ENABLED, DISABLED, ENABLED_FULL | - |
| `GuardrailWordPolicyAction` | `enum` | BLOCKED, NONE | - |
| `ImageFormat` | `enum` | PNG, JPEG, GIF, WEBP | - |
| `OutputFormatType` | `enum` | JSON_SCHEMA | The type of structured output format. Available options are: json_schema. |
| `PerformanceConfigLatency` | `enum` | STANDARD, OPTIMIZED | - |
| `ServiceTierType` | `enum` | PRIORITY, DEFAULT, FLEX, RESERVED | - |
| `SortAsyncInvocationBy` | `enum` | SUBMISSION_TIME | - |
| `SortOrder` | `enum` | ASCENDING, DESCENDING | - |
| `StopReason` | `enum` | END_TURN, TOOL_USE, MAX_TOKENS, STOP_SEQUENCE, GUARDRAIL_INTERVENED, CONTENT_FILTERED, MALFORMED_MODEL_OUTPUT, MALFORMED_TOOL_USE, MODEL_CONTEXT_WINDOW_EXCEEDED | - |
| `ToolResultStatus` | `enum` | SUCCESS, ERROR | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
