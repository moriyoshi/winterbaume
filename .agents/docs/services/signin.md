# AWS Sign-In Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Sign-In manages authentication for AWS services. This service provides secure authentication flows for accessing AWS resources from the console and developer tools.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Sign-In Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model setup and mutation workflows that create or update service resources across the `Create` operation families, including `CreateOAuth2Token`.

## Service Identity and Protocol

- AWS model slug: `signin`
- AWS SDK for Rust slug: `signin`
- Model version: `2023-01-01`
- Model file: `vendor/api-models-aws/models/signin/service/2023-01-01/signin-2023-01-01.json`
- SDK ID: `Signin`
- Endpoint prefix: `signin`
- ARN namespace: `signin`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Create` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateOAuth2Token`.
- 1 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Create

- Operations: `CreateOAuth2Token`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateOAuth2Token` | `POST /v1/token` | - | `tokenInput` | - | `CreateOAuth2TokenResponse` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsError`, `ValidationException` | CreateOAuth2Token API Path: /v1/token Request Method: POST Content-Type: application/json or application/x-www-form-urlencoded This API implements OAuth 2.0 flows for AWS Sign-In CLI clients, supporting both: 1. Auth ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `CreateOAuth2Token` | - | - | - | `tokenInput` |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | error, message | Error thrown for access denied scenarios with flexible HTTP status mapping Runtime HTTP Status Code Mapping: - HTTP 401 (Unauthorized): TOKEN_EXPIRED, AUTHC ... |
| `InternalServerException` | `structure` | error, message | Error thrown when an internal server error occurs HTTP Status Code: 500 Internal Server Error Used for unexpected server-side errors that prevent request pr ... |
| `TooManyRequestsError` | `structure` | error, message | Error thrown when rate limit is exceeded HTTP Status Code: 429 Too Many Requests Possible OAuth2ErrorCode values: - INVALID_REQUEST: Rate limiting, too many ... |
| `ValidationException` | `structure` | error, message | Error thrown when request validation fails HTTP Status Code: 400 Bad Request Used for request validation errors such as malformed parameters, missing requir ... |
| `CreateOAuth2TokenRequest` | `structure` | tokenInput | Input structure for CreateOAuth2Token operation Contains flattened token operation inputs for both authorization code and refresh token flows. The operation ... |
| `CreateOAuth2TokenResponse` | `structure` | tokenOutput | Output structure for CreateOAuth2Token operation Contains flattened token operation outputs for both authorization code and refresh token flows. The respons ... |
| `OAuth2ErrorCode` | `enum` | TOKEN_EXPIRED, USER_CREDENTIALS_CHANGED, INSUFFICIENT_PERMISSIONS, AUTHCODE_EXPIRED, SERVER_ERROR, INVALID_REQUEST | OAuth 2.0 error codes returned by the server Standard OAuth 2.0 error codes used in error responses to indicate the specific type of error that occurred dur ... |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
