# AWS Billing and Cost Management Data Exports

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

You can use the Data Exports API to create customized exports from multiple Amazon Web Services cost management and billing datasets, such as cost and usage data and cost optimization recommendations. The Data Exports API provides the following endpoint: https://bcm-data-exports.us-east-1.api.aws

## Possible Usage Scenarios
- From the AWS documentation and model: define billing data exports, tables, and delivery settings for cost and usage analysis.
- From the operation surface: support export lifecycle management, S3 destination metadata, table discovery, refresh/update flows, and tag-based cost-data inventory.

## Service Identity and Protocol

- AWS model slug: `bcm-data-exports`
- AWS SDK for Rust slug: `bcmdataexports`
- Model version: `2023-11-26`
- Model file: `vendor/api-models-aws/models/bcm-data-exports/service/2023-11-26/bcm-data-exports-2023-11-26.json`
- SDK ID: `BCM Data Exports`
- Endpoint prefix: `bcm-data-exports`
- ARN namespace: `bcm-data-exports`
- CloudFormation name: `BCMDataExports`
- CloudTrail event source: `bcm-data-exports.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Get` (3), `Create` (1), `Delete` (1), `Tag` (1), `Untag` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateExport`, `DeleteExport`, `TagResource`, `UntagResource`, `UpdateExport`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetExecution`, `GetExport`, `GetTable`, `ListExecutions`, `ListExports`, `ListTables`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateExport`, `DeleteExport`, `GetExecution`, `GetExport`, `ListExecutions`, `ListExports`, `UpdateExport`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 12 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `DataExport` | `ExportArn` | create: `CreateExport`; read: `GetExport`; update: `UpdateExport`; delete: `DeleteExport`; list: `ListExports` | - | - |
## Operation Groups

### List

- Operations: `ListExecutions`, `ListTables`, `ListTagsForResource`
- Traits: `readonly` (3), `paginated` (2)
- Common required input members in this group: -

### Get

- Operations: `GetExecution`, `GetTable`
- Traits: `readonly` (2)
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
| `GetExecution` | `-` | `readonly` | `ExportArn`, `ExecutionId` | - | `GetExecutionResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Exports data based on the source data update. |
| `GetTable` | `-` | `readonly` | `TableName` | - | `GetTableResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns the metadata for the specified table and table properties. This includes the list of columns in the table schema, their data types, and column descriptions. |
| `ListExecutions` | `-` | `readonly`, `paginated` | `ExportArn` | - | `ListExecutionsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the historical executions for the export. |
| `ListTables` | `-` | `readonly`, `paginated` | - | - | `ListTablesResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all available tables in data exports. |
| `ListTagsForResource` | `-` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List tags associated with an existing data export. |
| `TagResource` | `-` | - | `ResourceArn`, `ResourceTags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds tags for an existing data export definition. |
| `UntagResource` | `-` | - | `ResourceArn`, `ResourceTagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes tags associated with an existing data export definition. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You don't have sufficient access to perform this action. |
| `InternalServerException` | `structure` | Message | An error on the server occurred during the processing of your request. Try again later. |
| `ResourceNotFoundException` | `structure` | Message, ResourceId, ResourceType | The specified Amazon Resource Name (ARN) in the request doesn't exist. |
| `ServiceQuotaExceededException` | `structure` | Message, ResourceId, ResourceType, QuotaCode, ServiceCode | You've reached the limit on the number of resources you can create, or exceeded the size of an individual resource. |
| `ThrottlingException` | `structure` | Message, QuotaCode, ServiceCode | The request was denied due to request throttling. |
| `ValidationException` | `structure` | Message, Reason, Fields | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `GetExecutionRequest` | `structure` | ExportArn, ExecutionId | - |
| `GetExecutionResponse` | `structure` | ExecutionId, Export, ExecutionStatus | - |
| `GetTableRequest` | `structure` | TableName, TableProperties | - |
| `GetTableResponse` | `structure` | TableName, Description, TableProperties, Schema | - |
| `ListExecutionsRequest` | `structure` | ExportArn, MaxResults, NextToken | - |
| `ListExecutionsResponse` | `structure` | Executions, NextToken | - |
| `ListTablesRequest` | `structure` | NextToken, MaxResults | - |
| `ListTablesResponse` | `structure` | Tables, NextToken | - |
| `ListTagsForResourceRequest` | `structure` | ResourceArn, MaxResults, NextToken | - |
| `ListTagsForResourceResponse` | `structure` | ResourceTags, NextToken | - |
| `TagResourceRequest` | `structure` | ResourceArn, ResourceTags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceArn, ResourceTagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `CompressionOption` | `enum` | GZIP, PARQUET | - |
| `ExecutionStatusCode` | `enum` | INITIATION_IN_PROCESS, QUERY_QUEUED, QUERY_IN_PROCESS, QUERY_FAILURE, DELIVERY_IN_PROCESS, DELIVERY_SUCCESS, DELIVERY_FAILURE | - |
| `ExecutionStatusReason` | `enum` | INSUFFICIENT_PERMISSION, BILL_OWNER_CHANGED, INTERNAL_FAILURE | - |
| `ExportStatusCode` | `enum` | HEALTHY, UNHEALTHY | - |
| `FormatOption` | `enum` | TEXT_OR_CSV, PARQUET | - |
| `FrequencyOption` | `enum` | SYNCHRONOUS | - |
| `OverwriteOption` | `enum` | CREATE_NEW_REPORT, OVERWRITE_REPORT | - |
| `S3OutputType` | `enum` | CUSTOM | - |
| `ValidationExceptionReason` | `enum` | UNKNOWN_OPERATION, CANNOT_PARSE, FIELD_VALIDATION_FAILED, OTHER | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
