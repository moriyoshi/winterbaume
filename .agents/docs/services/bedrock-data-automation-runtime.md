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

- Operations: `InvokeDataAutomation`, `InvokeDataAutomationAsync`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `dataAutomationProfileArn`, `inputConfiguration`, `outputConfiguration`

### Get

- Operations: `GetDataAutomationStatus`
- Traits: `readonly` (1)
- Common required input members in this group: `invocationArn`

### List

- Operations: `ListTagsForResource`
- Common required input members in this group: `resourceARN`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceARN`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceARN`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetDataAutomationStatus` | - | `readonly` | `invocationArn` | - | `GetDataAutomationStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | API used to get data automation status. |
| `InvokeDataAutomation` | - | - | `dataAutomationProfileArn`, `inputConfiguration` | - | `InvokeDataAutomationResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Sync API: Invoke data automation. |
| `InvokeDataAutomationAsync` | - | `idempotent`, `idempotency-token` | `dataAutomationProfileArn`, `inputConfiguration`, `outputConfiguration` | `clientToken` | `InvokeDataAutomationAsyncResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Async API: Invoke data automation. |
| `ListTagsForResource` | - | - | `resourceARN` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List tags for an Amazon Bedrock Data Automation resource |
| `TagResource` | - | - | `resourceARN`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Tag an Amazon Bedrock Data Automation resource |
| `UntagResource` | - | - | `resourceARN`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Untag an Amazon Bedrock Data Automation resource |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | This exception will be thrown when customer does not have access to API. |
| `InternalServerException` | `structure` | `message` | This exception is for any internal un-expected service errors. |
| `ThrottlingException` | `structure` | `message` | This exception will be thrown when customer reached API TPS limit. |
| `ValidationException` | `structure` | `message` | This exception will be thrown when customer provided invalid parameters. |
| `ResourceNotFoundException` | `structure` | `message` | This exception will be thrown when resource provided from customer not found. |
| `ServiceQuotaExceededException` | `structure` | `message` | This exception will be thrown when service quota is exceeded. |
| `GetDataAutomationStatusRequest` | `structure` | `invocationArn` | Structure for request of GetDataAutomationStatus API. |
| `GetDataAutomationStatusResponse` | `structure` | `errorMessage`, `errorType`, `jobCompletionTime`, `jobDurationInSeconds`, `jobSubmissionTime`, `outputConfiguration`, `status` | Response of GetDataAutomationStatus API. |
| `InvokeDataAutomationRequest` | `structure` | `blueprints`, `dataAutomationConfiguration`, `dataAutomationProfileArn`, `encryptionConfiguration`, `inputConfiguration`, `outputConfiguration` | Invoke Data Automation Request |
| `InvokeDataAutomationResponse` | `structure` | `outputConfiguration`, `outputSegments`, `semanticModality` | Invoke Data Automation Response |
| `ServiceUnavailableException` | `structure` | `message` | This exception will be thrown when service is temporarily unavailable. |
| `InvokeDataAutomationAsyncRequest` | `structure` | `blueprints`, `clientToken`, `dataAutomationConfiguration`, `dataAutomationProfileArn`, `encryptionConfiguration`, `inputConfiguration`, `notificationConfiguration`, `outputConfiguration`, `tags` | Invoke Data Automation Async Request |
| `InvokeDataAutomationAsyncResponse` | `structure` | `invocationArn` | Invoke Data Automation Async Response |
| `ListTagsForResourceRequest` | `structure` | `resourceARN` | - |
| `ListTagsForResourceResponse` | `structure` | `tags` | - |
| `TagResourceRequest` | `structure` | `resourceARN`, `tags` | - |
| `TagResourceResponse` | `structure` | - | - |
| `UntagResourceRequest` | `structure` | `resourceARN`, `tagKeys` | - |
| `UntagResourceResponse` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
