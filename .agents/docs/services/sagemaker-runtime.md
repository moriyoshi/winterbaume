# Amazon SageMaker Runtime

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Amazon SageMaker AI runtime API.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon SageMaker Runtime workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model workflows exposed by the modelled operation families across the `Invoke` operation families, including `InvokeEndpoint`, `InvokeEndpointAsync`, `InvokeEndpointWithResponseStream`.

## Service Identity and Protocol

- AWS model slug: `sagemaker-runtime`
- AWS SDK for Rust slug: `sagemakerruntime`
- Model version: `2017-05-13`
- Model file: `vendor/api-models-aws/models/sagemaker-runtime/service/2017-05-13/sagemaker-runtime-2017-05-13.json`
- SDK ID: `SageMaker Runtime`
- Endpoint prefix: `runtime.sagemaker`
- ARN namespace: `sagemaker`
- CloudFormation name: `SageMakerRuntime`
- CloudTrail event source: `sagemakerruntime.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Invoke` (3).
- 3 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Control-Plane / Data-Plane Coherence

- **Paired with `sagemaker` ( same SDK slug `sagemaker` ).** `InvokeEndpoint`, `InvokeEndpointAsync`, and `InvokeEndpointWithResponseStream` target endpoints that the SageMaker control plane ( `winterbaume-sagemaker` ) creates via `CreateEndpoint`. In real AWS, invoking a non-existent endpoint fails with `ValidationError` ( "Endpoint <name> of account <id> not found" ).
- **Current Winterbaume status: divergent ( explicitly so ).** This crate's `state.rs` carries the comment "*In production, endpoints are created via the SageMaker API ( not Runtime ). For mock purposes, we auto-create endpoints on first invocation.*" `InvokeEndpoint` always succeeds, even for endpoint names that were never created in `winterbaume-sagemaker`. Invocation records are stored locally and are not visible to the control plane.
- **What needs to change:** depend on `winterbaume-sagemaker` and resolve the endpoint name through its `endpoints` map; reject unknown endpoints with the real-AWS error. The endpoint-config / model / variant graph should also resolve through there so the invocation record can capture the targeted variant ( useful for shadow tests, A/B routing, and inference-component invocation ).

## Operation Groups

### Invoke

- Operations: `InvokeEndpoint`, `InvokeEndpointAsync`, `InvokeEndpointWithResponseStream`
- Common required input members in this group: `Body`, `EndpointName`, `InputLocation`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `InvokeEndpoint` | `POST /endpoints/{EndpointName}/invocations` | - | `Body`, `EndpointName` | - | `InvokeEndpointOutput` | `InternalDependencyException`, `InternalFailure`, `ModelError`, `ModelNotReadyException`, `ServiceUnavailable`, `ValidationError` | After you deploy a model into production using Amazon SageMaker AI hosting services, your client applications use this API to get inferences from the model hosted at the specified endpoint. For an overview of Amazon SageMaker AI, see How It Works. |
| `InvokeEndpointAsync` | `POST /endpoints/{EndpointName}/async-invocations` | - | `EndpointName`, `InputLocation` | - | `InvokeEndpointAsyncOutput` | `InternalFailure`, `ServiceUnavailable`, `ValidationError` | After you deploy a model into production using Amazon SageMaker AI hosting services, your client applications use this API to get inferences from the model hosted at the specified endpoint in an asynchronous manner. Inference requests sent to this API are... |
| `InvokeEndpointWithResponseStream` | `POST /endpoints/{EndpointName}/invocations-response-stream` | - | `Body`, `EndpointName` | - | `InvokeEndpointWithResponseStreamOutput` | `InternalFailure`, `InternalStreamFailure`, `ModelError`, `ModelStreamError`, `ServiceUnavailable`, `ValidationError` | Invokes a model at the specified endpoint to return the inference response as a stream. The inference stream provides the response payload incrementally as a series of parts. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalFailure` | `structure` | `Message` | An internal failure occurred. |
| `ServiceUnavailable` | `structure` | `Message` | The service is unavailable. |
| `ValidationError` | `structure` | `Message` | Inspect your request and try again. |
| `ModelError` | `structure` | `LogStreamArn`, `Message`, `OriginalMessage`, `OriginalStatusCode` | Model (owned by the customer in the container) returned 4xx or 5xx error code. |
| `InvokeEndpointInput` | `structure` | `Accept`, `Body`, `ContentType`, `CustomAttributes`, `EnableExplanations`, `EndpointName`, `InferenceComponentName`, `InferenceId`, `SessionId`, `TargetContainerHostname`, `TargetModel`, `TargetVariant` | - |
| `InvokeEndpointOutput` | `structure` | `Body`, `ClosedSessionId`, `ContentType`, `CustomAttributes`, `InvokedProductionVariant`, `NewSessionId` | - |
| `InternalDependencyException` | `structure` | `Message` | Your request caused an exception with an internal dependency. |
| `ModelNotReadyException` | `structure` | `Message` | Either a serverless endpoint variant's resources are still being provisioned, or a multi-model endpoint is still downloading or loading the target model. |
| `InvokeEndpointAsyncInput` | `structure` | `Accept`, `ContentType`, `CustomAttributes`, `EndpointName`, `Filename`, `InferenceId`, `InputLocation`, `InvocationTimeoutSeconds`, `RequestTTLSeconds`, `S3OutputPathExtension` | - |
| `InvokeEndpointAsyncOutput` | `structure` | `FailureLocation`, `InferenceId`, `OutputLocation` | - |
| `InvokeEndpointWithResponseStreamInput` | `structure` | `Accept`, `Body`, `ContentType`, `CustomAttributes`, `EndpointName`, `InferenceComponentName`, `InferenceId`, `SessionId`, `TargetContainerHostname`, `TargetVariant` | - |
| `InvokeEndpointWithResponseStreamOutput` | `structure` | `Body`, `ContentType`, `CustomAttributes`, `InvokedProductionVariant` | - |
| `InternalStreamFailure` | `structure` | `Message` | The stream processing failed because of an unknown error, exception or failure. |
| `ModelStreamError` | `structure` | `ErrorCode`, `Message` | An error occurred while streaming the response body. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
