# Amazon SimpleDB v2

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon SimpleDB is a web service providing the core database functions of data indexing and querying in the cloud. By offloading the time and effort associated with building and operating a web-scale database, SimpleDB provides developers the freedom to focus on application development. A traditional, clustered relational database requires a sizable upfront capital outlay, is complex to design, and often requires extensive and repetitive database administration. Amazon SimpleDB is dramatically simpler, requiring no schema, automatically indexing your data and providing a simple API for storage and access. This approach eliminates the administrative burden of data modeling, index maintenance, and performance tuning.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon SimpleDB v2 resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon SimpleDB v2 workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model setup and mutation workflows that create or update service resources across the `Get`, `List`, `Start` operation families, including `GetExport`, `ListExports`, `StartDomainExport`.

## Service Identity and Protocol

- AWS model slug: `simpledbv2`
- AWS SDK for Rust slug: `simpledbv2`
- Model version: `2025-09-26`
- Model file: `vendor/api-models-aws/models/simpledbv2/service/2025-09-26/simpledbv2-2025-09-26.json`
- SDK ID: `SimpleDBv2`
- Endpoint prefix: `sdb`
- ARN namespace: `sdb`
- CloudFormation name: `SDB`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (1), `List` (1), `Start` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `StartDomainExport`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetExport`, `ListExports`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetExport`, `ListExports`, `StartDomainExport`.
- 3 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.

## Operation Groups

### Get

- Operations: `GetExport`
- Traits: `readonly` (1)
- Common required input members in this group: -

### List

- Operations: `ListExports`
- Traits: `readonly` (1), `paginated` (1)
- Common required input members in this group: -

### Start

- Operations: `StartDomainExport`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetExport` | `POST /v2/GetExport` | `readonly` | `exportArn` | - | `GetExportResponse` | `InvalidParameterValueException`, `NoSuchExportException` | Returns information for an existing domain export. |
| `ListExports` | `POST /v2/ListExports` | `readonly`, `paginated` | - | - | `ListExportsResponse` | `InvalidNextTokenException`, `InvalidParameterValueException`, `NoSuchDomainException` | Lists all exports that were created. The results are paginated and can be filtered by domain name. |
| `StartDomainExport` | `POST /v2/StartDomainExport` | `idempotent`, `idempotency-token` | `domainName`, `s3Bucket` | `clientToken` | `StartDomainExportResponse` | `ConflictException`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `NoSuchDomainException`, `NumberExportsLimitExceeded` | Initiates the export of a SimpleDB domain to an S3 bucket. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ConflictException` | `structure` | message | Indicates a conflict with one or more parameters of the request. |
| `InvalidNextTokenException` | `structure` | message | The specified next token is not valid. |
| `InvalidParameterCombinationException` | `structure` | message | Parameters that must not be used together were used together in the request. |
| `InvalidParameterValueException` | `structure` | message | The specified parameter value is not valid. |
| `NoSuchDomainException` | `structure` | message | The specified domain does not exist. |
| `NoSuchExportException` | `structure` | message | Export with specified ARN does not exist. |
| `NumberExportsLimitExceeded` | `structure` | message | Cannot start export as export quota limit was exceeded |
| `GetExportRequest` | `structure` | exportArn | - |
| `GetExportResponse` | `structure` | exportArn, clientToken, exportStatus, domainName, requestedAt, s3Bucket, s3KeyPrefix, s3SseAlgorithm, s3SseKmsKeyId, s3BucketOwner, failureCode, failureMessage, ... (+3) | - |
| `ListExportsRequest` | `structure` | domainName, maxResults, nextToken | - |
| `ListExportsResponse` | `structure` | exportSummaries, nextToken | - |
| `StartDomainExportRequest` | `structure` | clientToken, domainName, s3Bucket, s3KeyPrefix, s3SseAlgorithm, s3SseKmsKeyId, s3BucketOwner | - |
| `StartDomainExportResponse` | `structure` | clientToken, exportArn, requestedAt | - |
| `ExportStatus` | `enum` | PENDING, IN_PROGRESS, SUCCEEDED, FAILED | The current state of the export. Current possible values include : PENDING - export request received, IN_PROGRESS - export is being processed, SUCCEEDED - e ... |
| `S3SseAlgorithm` | `enum` | AES256, KMS | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
