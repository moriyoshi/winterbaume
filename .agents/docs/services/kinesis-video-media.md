# Amazon Kinesis Video Streams Media

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

No high-level service documentation is embedded in the AWS API model.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Kinesis Video Streams Media workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Get` operation families, including `GetMedia`.

## Service Identity and Protocol

- AWS model slug: `kinesis-video-media`
- AWS SDK for Rust slug: `kinesisvideo`
- Model version: `2017-09-30`
- Model file: `vendor/api-models-aws/models/kinesis-video-media/service/2017-09-30/kinesis-video-media-2017-09-30.json`
- SDK ID: `Kinesis Video Media`
- Endpoint prefix: `kinesisvideo`
- ARN namespace: `kinesisvideo`
- CloudFormation name: `KinesisVideoMedia`
- CloudTrail event source: `kinesisvideomedia.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetMedia`.
- 1 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Operation Groups

### Get

- Operations: `GetMedia`
- Common required input members in this group: `StartSelector`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetMedia` | `POST /getMedia` | - | `StartSelector` | - | `GetMediaOutput` | `ClientLimitExceededException`, `ConnectionLimitExceededException`, `InvalidArgumentException`, `InvalidEndpointException`, `NotAuthorizedException`, `ResourceNotFoundException` | Use this API to retrieve media content from a Kinesis video stream. In the request, you identify the stream name or stream Amazon Resource Name (ARN), and the starting chunk. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `GetMediaInput` | `structure` | `StartSelector`, `StreamARN`, `StreamName` | - |
| `GetMediaOutput` | `structure` | `ContentType`, `Payload` | - |
| `ClientLimitExceededException` | `structure` | `Message` | Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client calls. |
| `ConnectionLimitExceededException` | `structure` | `Message` | Kinesis Video Streams has throttled the request because you have exceeded the limit of allowed client connections. |
| `InvalidArgumentException` | `structure` | `Message` | The value for this input parameter is invalid. |
| `InvalidEndpointException` | `structure` | `Message` | Status Code: 400, Caller used wrong endpoint to write data to a stream. |
| `NotAuthorizedException` | `structure` | `Message` | Status Code: 403, The caller is not authorized to perform an operation on the given stream, or the token has expired. |
| `ResourceNotFoundException` | `structure` | `Message` | Status Code: 404, The stream with the given name does not exist. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
