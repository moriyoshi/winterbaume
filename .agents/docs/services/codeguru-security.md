# Amazon CodeGuru Security

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

On November 20, 2025, AWS will discontinue support for Amazon CodeGuru Security. After November 20, 2025, you will no longer be able to access the /codeguru/security console, service resources, or documentation. For more information, see https://docs.aws.amazon.com/codeguru/latest/security-ug/end-of-support.html. This section provides documentation for the Amazon CodeGuru Security API operations. CodeGuru Security is a service that uses program analysis and machine learning to detect security policy violations and vulnerabilities, and recommends ways to address these security risks.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for Amazon CodeGuru Security by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent documented Amazon CodeGuru Security workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model setup and mutation workflows that create or update service resources across the `Get`, `List`, `Create`, `Batch`, `Tag` operation families, including `GetAccountConfiguration`, `GetFindings`, `GetMetricsSummary`, `GetScan`, `ListFindingsMetrics`, `ListScans`.

## Service Identity and Protocol

- AWS model slug: `codeguru-security`
- AWS SDK for Rust slug: `codegurusecurity`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/codeguru-security/service/2018-05-10/codeguru-security-2018-05-10.json`
- SDK ID: `CodeGuru Security`
- Endpoint prefix: `-`
- ARN namespace: `codeguru-security`
- CloudFormation name: `-`
- CloudTrail event source: `codeguru-security.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (4), `List` (3), `Create` (2), `Batch` (1), `Tag` (1), `Untag` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchGetFindings`, `CreateScan`, `CreateUploadUrl`, `TagResource`, `UntagResource`, `UpdateAccountConfiguration`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `BatchGetFindings`, `GetAccountConfiguration`, `GetFindings`, `GetMetricsSummary`, `GetScan`, `ListFindingsMetrics`, `ListScans`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 2 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateScan`, `GetScan`, `ListScans`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 13 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/cli/latest/reference/codeguru-security/create-upload-url.html
- https://docs.aws.amazon.com/cli/latest/reference/codeguru-security/create-scan.html
- https://docs.aws.amazon.com/cli/latest/reference/codeguru-security/batch-get-findings.html

Research outcomes:
- CodeGuru Security uses program analysis and machine learning to detect security policy violations and vulnerabilities and recommends ways to address the risks.
- `CreateUploadUrl` generates a pre-signed S3 URL, request headers, and a code artifact identifier for an uploaded code resource.
- Uploaded code resources are later referenced by `codeArtifactId`, which is returned by `CreateUploadUrl`.
- `CreateScan` creates a scan using code uploaded to S3 and supports a client token for idempotency.
- `CreateScan` takes a tagged-union resource identifier. For uploaded code, only `codeArtifactId` is set.
- `scanName` is the unique name used to track revisions across multiple scans of the same resource and is required for uploaded resources.
- Scan type can be `Standard` or `Express`. Express scans use limited resources and a limited detector set for near-real-time analysis; Standard scans use the full detector set and standard resource limits.
- Analysis type can be `Security` or `All`. `Security` generates security findings only; `All` includes security and quality findings.
- `CreateScan` returns a scan name, run id, and resource id. `BatchGetFindings` retrieves findings by run or finding identifiers.

Parity implications:
- Model upload URLs, required upload headers, code artifact ids, scans, run ids, scan names, scan type, analysis type, findings, and idempotency tokens separately.
- CreateScan should require a previously issued upload artifact when using `codeArtifactId` and should not conflate upload reservation with scan execution.
- Scan execution and finding retrieval should be asynchronous and keyed by scan/run identity.

## Operation Groups

### Get

- Operations: `GetAccountConfiguration`, `GetFindings`, `GetMetricsSummary`, `GetScan`
- Traits: `paginated` (1), `readonly` (4)
- Common required input members in this group: `date`, `scanName`

### List

- Operations: `ListFindingsMetrics`, `ListScans`, `ListTagsForResource`
- Traits: `paginated` (2), `readonly` (3)
- Common required input members in this group: `endDate`, `resourceArn`, `startDate`

### Create

- Operations: `CreateScan`, `CreateUploadUrl`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `resourceId`, `scanName`

### Batch

- Operations: `BatchGetFindings`
- Traits: `readonly` (1)
- Common required input members in this group: `findingIdentifiers`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

### Update

- Operations: `UpdateAccountConfiguration`
- Common required input members in this group: `encryptionConfig`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchGetFindings` | `POST /batchGetFindings` | `readonly` | `findingIdentifiers` | - | `BatchGetFindingsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of requested findings from standard scans. |
| `CreateScan` | `POST /scans` | `idempotency-token` | `resourceId`, `scanName` | `clientToken` | `CreateScanResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use to create a scan using code uploaded to an Amazon S3 bucket. |
| `CreateUploadUrl` | `POST /uploadUrl` | - | `scanName` | - | `CreateUploadUrlResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Generates a pre-signed URL, request headers used to upload a code resource, and code artifact identifier for the uploaded resource. You can upload your code resource to the URL with the request headers using any HTTP client. |
| `GetAccountConfiguration` | `GET /accountConfiguration/get` | `readonly` | - | - | `GetAccountConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Use to get the encryption configuration for an account. |
| `GetFindings` | `GET /findings/{scanName}` | `readonly`, `paginated` | `scanName` | - | `GetFindingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of all findings generated by a particular scan. |
| `GetMetricsSummary` | `GET /metrics/summary` | `readonly` | `date` | - | `GetMetricsSummaryResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a summary of metrics for an account from a specified date, including number of open findings, the categories with most findings, the scans with most open findings, and scans with most open critical findings. |
| `GetScan` | `GET /scans/{scanName}` | `readonly` | `scanName` | - | `GetScanResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns details about a scan, including whether or not a scan has completed. |
| `ListFindingsMetrics` | `GET /metrics/findings` | `readonly`, `paginated` | `endDate`, `startDate` | - | `ListFindingsMetricsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns metrics about all findings in an account within a specified time range. |
| `ListScans` | `GET /scans` | `readonly`, `paginated` | - | - | `ListScansResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of all scans in an account. Does not return `EXPRESS` scans. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of all tags associated with a scan. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use to add one or more tags to an existing scan. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use to remove one or more tags from an existing scan. |
| `UpdateAccountConfiguration` | `PUT /updateAccountConfiguration` | - | `encryptionConfig` | - | `UpdateAccountConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use to update the encryption configuration for an account. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `errorCode`, `message`, `resourceId`, `resourceType` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `error`, `message` | The server encountered an internal error and is unable to complete the request. |
| `ThrottlingException` | `structure` | `errorCode`, `message`, `quotaCode`, `serviceCode` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `errorCode`, `fieldList`, `message`, `reason` | The input fails to satisfy the specified constraints. |
| `ResourceNotFoundException` | `structure` | `errorCode`, `message`, `resourceId`, `resourceType` | The resource specified in the request was not found. |
| `ConflictException` | `structure` | `errorCode`, `message`, `resourceId`, `resourceType` | The requested operation would cause a conflict with the current state of a service resource associated with the request. |
| `BatchGetFindingsRequest` | `structure` | `findingIdentifiers` | - |
| `BatchGetFindingsResponse` | `structure` | `failedFindings`, `findings` | - |
| `CreateScanRequest` | `structure` | `analysisType`, `clientToken`, `resourceId`, `scanName`, `scanType`, `tags` | - |
| `CreateScanResponse` | `structure` | `resourceId`, `runId`, `scanName`, `scanNameArn`, `scanState` | - |
| `CreateUploadUrlRequest` | `structure` | `scanName` | - |
| `CreateUploadUrlResponse` | `structure` | `codeArtifactId`, `requestHeaders`, `s3Url` | - |
| `GetAccountConfigurationRequest` | `structure` | - | - |
| `GetAccountConfigurationResponse` | `structure` | `encryptionConfig` | - |
| `GetFindingsRequest` | `structure` | `maxResults`, `nextToken`, `scanName`, `status` | - |
| `GetFindingsResponse` | `structure` | `findings`, `nextToken` | - |
| `GetMetricsSummaryRequest` | `structure` | `date` | - |
| `GetMetricsSummaryResponse` | `structure` | `metricsSummary` | - |
| `GetScanRequest` | `structure` | `runId`, `scanName` | - |
| `GetScanResponse` | `structure` | `analysisType`, `createdAt`, `errorMessage`, `numberOfRevisions`, `runId`, `scanName`, `scanNameArn`, `scanState`, `updatedAt` | - |
| `ListFindingsMetricsRequest` | `structure` | `endDate`, `maxResults`, `nextToken`, `startDate` | - |
| `ListFindingsMetricsResponse` | `structure` | `findingsMetrics`, `nextToken` | - |
| `ListScansRequest` | `structure` | `maxResults`, `nextToken` | - |
| `ListScansResponse` | `structure` | `nextToken`, `summaries` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
