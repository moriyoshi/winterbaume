# Amazon SageMaker Runtime HTTP2

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Amazon SageMaker AI runtime HTTP/2 API.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon SageMaker Runtime HTTP2 workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model workflows exposed by the modelled operation families across the `Invoke` operation families, including `InvokeEndpointWithBidirectionalStream`.

## Service Identity and Protocol

- AWS model slug: `sagemaker-runtime-http2`
- AWS SDK for Rust slug: `sagemaker`
- Model version: `2025-10-01`
- Model file: `vendor/api-models-aws/models/sagemaker-runtime-http2/service/2025-10-01/sagemaker-runtime-http2-2025-10-01.json`
- SDK ID: `SageMaker Runtime HTTP2`
- Endpoint prefix: `runtime.sagemaker`
- ARN namespace: `sagemaker`
- CloudFormation name: `SageMakerRuntime`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Invoke` (1).
- 1 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Operation Groups

### Invoke

- Operations: `InvokeEndpointWithBidirectionalStream`
- Common required input members in this group: `Body`, `EndpointName`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `InvokeEndpointWithBidirectionalStream` | `POST /endpoints/{EndpointName}/invocations-bidirectional-stream` | - | `Body`, `EndpointName` | - | `InvokeEndpointWithBidirectionalStreamOutput` | `InputValidationError`, `InternalServerError`, `InternalStreamFailure`, `ModelError`, `ModelStreamError`, `ServiceUnavailableError` | Invokes a model endpoint with bidirectional streaming capabilities. This operation establishes a persistent connection that allows you to send multiple requests and receive streaming responses from the model in real-time. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `InvokeEndpointWithBidirectionalStream` | `TargetVariant -> X-Amzn-SageMaker-Target-Variant`, `ModelInvocationPath -> X-Amzn-SageMaker-Model-Invocation-Path`, `ModelQueryString -> X-Amzn-SageMaker-Model-Query-String` | - | - | `Body` |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvokeEndpointWithBidirectionalStreamInput` | `structure` | `Body`, `EndpointName`, `ModelInvocationPath`, `ModelQueryString`, `TargetVariant` | - |
| `InvokeEndpointWithBidirectionalStreamOutput` | `structure` | `Body`, `InvokedProductionVariant` | - |
| `InputValidationError` | `structure` | `ErrorCode`, `Message` | The input fails to satisfy the constraints specified by an AWS service. |
| `InternalServerError` | `structure` | `ErrorCode`, `Message` | The request processing has failed because of an unknown error, exception or failure. |
| `InternalStreamFailure` | `structure` | `Message` | Internal stream failure that occurs during streaming. |
| `ModelError` | `structure` | `ErrorCode`, `LogStreamArn`, `Message`, `OriginalMessage`, `OriginalStatusCode` | An error occurred while processing the model. |
| `ModelStreamError` | `structure` | `ErrorCode`, `Message` | Model stream error that occurs during streaming. |
| `ServiceUnavailableError` | `structure` | `ErrorCode`, `Message` | The request has failed due to a temporary failure of the server. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
