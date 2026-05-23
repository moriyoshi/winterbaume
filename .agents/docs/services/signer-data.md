# AWS Signer Data Plane

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Signer Data Plane service provides APIs for checking revocation status of signed artifacts.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Signer Data Plane workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Get` operation families, including `GetRevocationStatus`.

## Service Identity and Protocol

- AWS model slug: `signer-data`
- AWS SDK for Rust slug: `signer`
- Model version: `2017-08-25`
- Model file: `vendor/api-models-aws/models/signer-data/service/2017-08-25/signer-data-2017-08-25.json`
- SDK ID: `Signer Data`
- Endpoint prefix: `data-signer`
- ARN namespace: `signer`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetRevocationStatus`.
- 1 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Operation Groups

### Get

- Operations: `GetRevocationStatus`
- Traits: `readonly` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetRevocationStatus` | `GET /revocations` | `readonly` | `signatureTimestamp`, `platformId`, `profileVersionArn`, `jobArn`, `certificateHashes` | - | `GetRevocationStatusResponse` | `AccessDeniedException`, `InternalServiceErrorException`, `TooManyRequestsException`, `ValidationException` | Retrieves the revocation status for a signed artifact by checking if the signing profile, job, or certificate has been revoked. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetRevocationStatus` | - | `signatureTimestamp -> signatureTimestamp`, `platformId -> platformId`, `profileVersionArn -> profileVersionArn`, `jobArn -> jobArn`, `certificateHashes -> certificateHashes` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message, code | You do not have sufficient permissions to perform this action. |
| `InternalServiceErrorException` | `structure` | message, code | An internal service error occurred. |
| `TooManyRequestsException` | `structure` | message, code | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message, code | The request contains invalid parameters or is malformed. |
| `GetRevocationStatusRequest` | `structure` | signatureTimestamp, platformId, profileVersionArn, jobArn, certificateHashes | Request structure for checking revocation status. |
| `GetRevocationStatusResponse` | `structure` | revokedEntities | Response containing the list of revoked entities. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
