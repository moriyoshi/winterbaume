# Amazon WorkMail Message Flow

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The WorkMail Message Flow API provides access to email messages as they are being sent and received by a WorkMail organization.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon WorkMail Message Flow workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model setup and mutation workflows that create or update service resources across the `Get`, `Put` operation families, including `GetRawMessageContent`, `PutRawMessageContent`.

## Service Identity and Protocol

- AWS model slug: `workmailmessageflow`
- AWS SDK for Rust slug: `workmailmessageflow`
- Model version: `2019-05-01`
- Model file: `vendor/api-models-aws/models/workmailmessageflow/service/2019-05-01/workmailmessageflow-2019-05-01.json`
- SDK ID: `WorkMailMessageFlow`
- Endpoint prefix: `workmailmessageflow`
- ARN namespace: `workmailmessageflow`
- CloudFormation name: `WorkMailMessageFlow`
- CloudTrail event source: `workmailmessageflow.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (1), `Put` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `PutRawMessageContent`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetRawMessageContent`.
- 2 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `Lambda`.

## Operation Groups

### Get

- Operations: `GetRawMessageContent`
- Common required input members in this group: `messageId`

### Put

- Operations: `PutRawMessageContent`
- Common required input members in this group: `content`, `messageId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetRawMessageContent` | `GET /messages/{messageId}` | - | `messageId` | - | `GetRawMessageContentResponse` | `ResourceNotFoundException` | Retrieves the raw content of an in-transit email message, in MIME format. |
| `PutRawMessageContent` | `POST /messages/{messageId}` | - | `content`, `messageId` | - | `PutRawMessageContentResponse` | `InvalidContentLocation`, `MessageFrozen`, `MessageRejected`, `ResourceNotFoundException` | Updates the raw content of an in-transit email message, in MIME format. This example describes how to update in-transit email message. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ResourceNotFoundException` | `structure` | `message` | The requested email message is not found. |
| `GetRawMessageContentRequest` | `structure` | `messageId` | - |
| `GetRawMessageContentResponse` | `structure` | `messageContent` | - |
| `PutRawMessageContentRequest` | `structure` | `content`, `messageId` | - |
| `PutRawMessageContentResponse` | `structure` | - | - |
| `InvalidContentLocation` | `structure` | `message` | WorkMail could not access the updated email content. |
| `MessageFrozen` | `structure` | `message` | The requested email is not eligible for update. |
| `MessageRejected` | `structure` | `message` | The requested email could not be updated due to an error in the MIME content. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
