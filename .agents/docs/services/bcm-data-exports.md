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

- Operations: `ListExecutions`, `ListExports`, `ListTables`, `ListTagsForResource`
- Traits: `paginated` (3), `readonly` (4)
- Common required input members in this group: `ExportArn`, `ResourceArn`

### Get

- Operations: `GetExecution`, `GetExport`, `GetTable`
- Traits: `readonly` (3)
- Common required input members in this group: `ExecutionId`, `ExportArn`, `TableName`

### Create

- Operations: `CreateExport`
- Common required input members in this group: `Export`

### Delete

- Operations: `DeleteExport`
- Traits: `idempotent` (1)
- Common required input members in this group: `ExportArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `ResourceTags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `ResourceTagKeys`

### Update

- Operations: `UpdateExport`
- Common required input members in this group: `Export`, `ExportArn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateExport` | - | - | `Export` | - | `CreateExportResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a data export and specifies the data query, the delivery preference, and any optional resource tags. A `DataQuery` consists of both a `QueryStatement` and `TableConfigurations`. |
| `DeleteExport` | - | `idempotent` | `ExportArn` | - | `DeleteExportResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an existing data export. |
| `GetExecution` | - | `readonly` | `ExecutionId`, `ExportArn` | - | `GetExecutionResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Exports data based on the source data update. |
| `GetExport` | - | `readonly` | `ExportArn` | - | `GetExportResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Views the definition of an existing data export. |
| `GetTable` | - | `readonly` | `TableName` | - | `GetTableResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns the metadata for the specified table and table properties. This includes the list of columns in the table schema, their data types, and column descriptions. |
| `ListExecutions` | - | `readonly`, `paginated` | `ExportArn` | - | `ListExecutionsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the historical executions for the export. |
| `ListExports` | - | `readonly`, `paginated` | - | - | `ListExportsResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all data export definitions. |
| `ListTables` | - | `readonly`, `paginated` | - | - | `ListTablesResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all available tables in data exports. |
| `ListTagsForResource` | - | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List tags associated with an existing data export. |
| `TagResource` | - | - | `ResourceArn`, `ResourceTags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds tags for an existing data export definition. |
| `UntagResource` | - | - | `ResourceArn`, `ResourceTagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes tags associated with an existing data export definition. |
| `UpdateExport` | - | - | `Export`, `ExportArn` | - | `UpdateExportResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing data export by overwriting all export parameters. All export parameters must be provided in the UpdateExport request. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `Message` | An error on the server occurred during the processing of your request. |
| `ThrottlingException` | `structure` | `Message`, `QuotaCode`, `ServiceCode` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `Fields`, `Message`, `Reason` | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId`, `ResourceType` | The specified Amazon Resource Name (ARN) in the request doesn't exist. |
| `AccessDeniedException` | `structure` | `Message` | You don't have sufficient access to perform this action. |
| `CreateExportRequest` | `structure` | `Export`, `ResourceTags` | - |
| `CreateExportResponse` | `structure` | `ExportArn` | - |
| `ServiceQuotaExceededException` | `structure` | `Message`, `QuotaCode`, `ResourceId`, `ResourceType`, `ServiceCode` | You've reached the limit on the number of resources you can create, or exceeded the size of an individual resource. |
| `DeleteExportRequest` | `structure` | `ExportArn` | - |
| `DeleteExportResponse` | `structure` | `ExportArn` | - |
| `GetExecutionRequest` | `structure` | `ExecutionId`, `ExportArn` | - |
| `GetExecutionResponse` | `structure` | `ExecutionId`, `ExecutionStatus`, `Export` | - |
| `GetExportRequest` | `structure` | `ExportArn` | - |
| `GetExportResponse` | `structure` | `Export`, `ExportStatus` | - |
| `GetTableRequest` | `structure` | `TableName`, `TableProperties` | - |
| `GetTableResponse` | `structure` | `Description`, `Schema`, `TableName`, `TableProperties` | - |
| `ListExecutionsRequest` | `structure` | `ExportArn`, `MaxResults`, `NextToken` | - |
| `ListExecutionsResponse` | `structure` | `Executions`, `NextToken` | - |
| `ListExportsRequest` | `structure` | `MaxResults`, `NextToken` | - |
| `ListExportsResponse` | `structure` | `Exports`, `NextToken` | - |
| `ListTablesRequest` | `structure` | `MaxResults`, `NextToken` | - |
| `ListTablesResponse` | `structure` | `NextToken`, `Tables` | - |
| `ListTagsForResourceRequest` | `structure` | `MaxResults`, `NextToken`, `ResourceArn` | - |
| `ListTagsForResourceResponse` | `structure` | `NextToken`, `ResourceTags` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
