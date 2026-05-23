# Runtime for Amazon Bedrock Data Automation

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Bedrock Data Automation Runtime

## Possible Usage Scenarios
- From the AWS documentation and model: invoke Bedrock Data Automation projects and inspect asynchronous invocation results.
- From the operation surface: model document/media processing jobs, result retrieval, job state, and failure reporting for data automation workloads.

## Service Identity and Protocol

- AWS model slug: `bedrock-data-automation-runtime`
- AWS SDK for Rust slug: `bedrock`
- Model version: `2024-06-13`
- Model file: `vendor/api-models-aws/models/bedrock-data-automation-runtime/service/2024-06-13/bedrock-data-automation-runtime-2024-06-13.json`
- SDK ID: `Bedrock Data Automation Runtime`
- Endpoint prefix: `bedrock-data-automation-runtime`
- ARN namespace: `bedrock`
- CloudFormation name: `-`
- CloudTrail event source: `bedrock.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Invoke` (2), `Get` (1), `List` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetDataAutomationStatus`, `ListTagsForResource`.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 6 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AutomationJobResource` | `invocationArn` | create: `InvokeDataAutomationAsync`; read: `GetDataAutomationStatus` | - | - |
## Operation Groups

### Invoke

- Operations: `InvokeDataAutomation`
- Common required input members in this group: -

### List

- Operations: `ListTagsForResource`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `InvokeDataAutomation` | `-` | - | `inputConfiguration`, `dataAutomationProfileArn` | - | `InvokeDataAutomationResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Sync API: Invoke data automation. |
| `ListTagsForResource` | `-` | - | `resourceARN` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List tags for an Amazon Bedrock Data Automation resource |
| `TagResource` | `-` | - | `resourceARN`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Tag an Amazon Bedrock Data Automation resource |
| `UntagResource` | `-` | - | `resourceARN`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Untag an Amazon Bedrock Data Automation resource |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | This exception will be thrown when customer does not have access to API. |
| `InternalServerException` | `structure` | message | This exception is for any internal un-expected service errors. |
| `ResourceNotFoundException` | `structure` | message | This exception will be thrown when resource provided from customer not found. |
| `ServiceQuotaExceededException` | `structure` | message | This exception will be thrown when service quota is exceeded. |
| `ServiceUnavailableException` | `structure` | message | This exception will be thrown when service is temporarily unavailable. |
| `ThrottlingException` | `structure` | message | This exception will be thrown when customer reached API TPS limit. |
| `ValidationException` | `structure` | message | This exception will be thrown when customer provided invalid parameters. |
| `InvokeDataAutomationRequest` | `structure` | inputConfiguration, dataAutomationConfiguration, blueprints, dataAutomationProfileArn, encryptionConfiguration, outputConfiguration | Invoke Data Automation Request |
| `InvokeDataAutomationResponse` | `structure` | outputConfiguration, semanticModality, outputSegments | Invoke Data Automation Response |
| `ListTagsForResourceRequest` | `structure` | resourceARN | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | resourceARN, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceARN, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `AutomationJobStatus` | `enum` | CREATED, IN_PROGRESS, SUCCESS, SERVICE_ERROR, CLIENT_ERROR | List of status supported by automation jobs |
| `BlueprintStage` | `enum` | DEVELOPMENT, LIVE | Blueprint stage enum. |
| `CustomOutputStatus` | `enum` | MATCH, NO_MATCH | Custom output status enum |
| `DataAutomationStage` | `enum` | LIVE, DEVELOPMENT | Data automation stage. |
| `SemanticModality` | `enum` | DOCUMENT, IMAGE, AUDIO, VIDEO | Semantic modality enum |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
