# Amazon Neptune Graph

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Neptune Analytics is a new analytics database engine for Amazon Neptune that helps customers get to insights faster by quickly processing large amounts of graph data, invoking popular graph analytic algorithms in low-latency queries, and getting analytics results in seconds.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Neptune Graph resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Neptune Graph workflows in the local mock. Key resources include `GraphResource`, `PrivateGraphEndpointResource`, `SnapshotResource`, `TaskResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Cancel`, `Delete` operation families, including `GetExportTask`, `GetGraph`, `GetGraphSnapshot`, `GetGraphSummary`, `ListExportTasks`, `ListGraphSnapshots`.

## Service Identity and Protocol

- AWS model slug: `neptune-graph`
- AWS SDK for Rust slug: `neptunegraph`
- Model version: `2023-11-29`
- Model file: `vendor/api-models-aws/models/neptune-graph/service/2023-11-29/neptune-graph-2023-11-29.json`
- SDK ID: `Neptune Graph`
- Endpoint prefix: `-`
- ARN namespace: `neptune-graph`
- CloudFormation name: `-`
- CloudTrail event source: `neptune-graph.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `ApiType`, `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (7), `List` (7), `Create` (4), `Cancel` (3), `Delete` (3), `Start` (3), `Execute` (1), `Reset` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelExportTask`, `CancelImportTask`, `CancelQuery`, `CreateGraph`, `CreateGraphSnapshot`, `CreateGraphUsingImportTask`, `CreatePrivateGraphEndpoint`, `DeleteGraph`, `DeleteGraphSnapshot`, `DeletePrivateGraphEndpoint`, `RestoreGraphFromSnapshot`, `StartExportTask`, `StartGraph`, `StartImportTask`, `StopGraph`, `TagResource`, `UntagResource`, `UpdateGraph`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetExportTask`, `GetGraph`, `GetGraphSnapshot`, `GetGraphSummary`, `GetImportTask`, `GetPrivateGraphEndpoint`, `GetQuery`, `ListExportTasks`, `ListGraphSnapshots`, `ListGraphs`, `ListImportTasks`, `ListPrivateGraphEndpoints`, `ListQueries`, `ListTagsForResource`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 3 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelExportTask`, `CancelImportTask`, `CancelQuery`, `CreateGraphUsingImportTask`, `GetExportTask`, `GetImportTask`, `ListExportTasks`, `ListImportTasks`, `StartExportTask`, `StartGraph`, `StartImportTask`, `StopGraph`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 34 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `GraphResource` | - | - | `CreateGraph`, `DeleteGraph`, `GetGraph`, `ListGraphs`, `ResetGraph`, `RestoreGraphFromSnapshot`, `StartGraph`, `StopGraph`, `UpdateGraph` | - |
| `PrivateGraphEndpointResource` | - | - | `CreatePrivateGraphEndpoint`, `DeletePrivateGraphEndpoint`, `GetPrivateGraphEndpoint`, `ListPrivateGraphEndpoints` | - |
| `SnapshotResource` | - | - | `CreateGraphSnapshot`, `DeleteGraphSnapshot`, `GetGraphSnapshot`, `ListGraphSnapshots` | - |
| `TaskResource` | - | - | `CancelExportTask`, `CancelImportTask`, `CreateGraphUsingImportTask`, `GetExportTask`, `GetImportTask`, `ListExportTasks`, `ListImportTasks`, `StartExportTask`, `StartImportTask` | - |
## Operation Groups

### Get

- Operations: `GetGraphSummary`, `GetQuery`
- Traits: `readonly` (2)
- Common required input members in this group: `graphIdentifier`

### List

- Operations: `ListQueries`, `ListTagsForResource`
- Traits: `readonly` (2)
- Common required input members in this group: -

### Cancel

- Operations: `CancelQuery`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Execute

- Operations: `ExecuteQuery`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelQuery` | `DELETE /queries/{queryId}` | `idempotent` | `graphIdentifier`, `queryId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels a specified query. |
| `ExecuteQuery` | `POST /queries` | - | `graphIdentifier`, `queryString`, `language` | - | `ExecuteQueryOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `UnprocessableException`, `ValidationException` | Execute an openCypher query. When invoking this operation in a Neptune Analytics cluster, the IAM user or role making the request must have a policy attached that allows one of the following IAM actions in that clust ... |
| `GetGraphSummary` | `GET /summary` | `readonly` | `graphIdentifier` | - | `GetGraphSummaryOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a graph summary for a property graph. |
| `GetQuery` | `GET /queries/{queryId}` | `readonly` | `graphIdentifier`, `queryId` | - | `GetQueryOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the status of a specified query. When invoking this operation in a Neptune Analytics cluster, the IAM user or role making the request must have the neptune-graph:GetQueryStatus IAM action attached. |
| `ListQueries` | `GET /queries` | `readonly` | `graphIdentifier`, `maxResults` | - | `ListQueriesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists active openCypher queries. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists tags associated with a specified resource. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds tags to the specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the specified tags from the specified resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `CancelQuery` | `graphIdentifier -> graphIdentifier` | - | - | - |
| `ExecuteQuery` | `graphIdentifier -> graphIdentifier` | - | - | - |
| `GetGraphSummary` | `graphIdentifier -> graphIdentifier` | `mode -> mode` | - | - |
| `GetQuery` | `graphIdentifier -> graphIdentifier` | - | - | - |
| `ListQueries` | `graphIdentifier -> graphIdentifier` | `maxResults -> maxResults`, `state -> state` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | Raised in case of an authentication or authorization failure. |
| `ConflictException` | `structure` | message, reason | Raised when a conflict is encountered. |
| `InternalServerException` | `structure` | message | A failure occurred on the server. |
| `ResourceNotFoundException` | `structure` | message | A specified resource could not be located. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, serviceCode, quotaCode | A service quota was exceeded. |
| `ThrottlingException` | `structure` | message | The exception was interrupted by throttling. |
| `UnprocessableException` | `structure` | message, reason | Request cannot be processed due to known reasons. Eg. partition full. |
| `ValidationException` | `structure` | message, reason | A resource could not be validated. |
| `CancelQueryInput` | `structure` | graphIdentifier, queryId | - |
| `ExecuteQueryInput` | `structure` | graphIdentifier, queryString, language, parameters, planCache, explainMode, queryTimeoutMilliseconds | - |
| `ExecuteQueryOutput` | `structure` | payload | - |
| `GetGraphSummaryInput` | `structure` | graphIdentifier, mode | - |
| `GetGraphSummaryOutput` | `structure` | version, lastStatisticsComputationTime, graphSummary | - |
| `GetQueryInput` | `structure` | graphIdentifier, queryId | - |
| `GetQueryOutput` | `structure` | id, queryString, waited, elapsed, state | - |
| `ListQueriesInput` | `structure` | graphIdentifier, maxResults, state | - |
| `ListQueriesOutput` | `structure` | queries | - |
| `ListTagsForResourceInput` | `structure` | resourceArn | - |
| `ListTagsForResourceOutput` | `structure` | tags | - |
| `TagResourceInput` | `structure` | resourceArn, tags | - |
| `TagResourceOutput` | `structure` | **empty (no members)** | - |
| `UntagResourceInput` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceOutput` | `structure` | **empty (no members)** | - |
| `BlankNodeHandling` | `enum` | CONVERT_TO_IRI | - |
| `ConflictExceptionReason` | `enum` | CONCURRENT_MODIFICATION | - |
| `ExplainMode` | `enum` | STATIC, DETAILS | - |
| `ExportFormat` | `enum` | PARQUET, CSV | - |
| `ExportTaskStatus` | `enum` | INITIALIZING, EXPORTING, SUCCEEDED, FAILED, CANCELLING, CANCELLED, DELETED | - |
| `Format` | `enum` | CSV, OPEN_CYPHER, PARQUET, NTRIPLES | - |
| `GraphStatus` | `enum` | CREATING, AVAILABLE, DELETING, RESETTING, UPDATING, SNAPSHOTTING, FAILED, IMPORTING, STARTING, STOPPING, STOPPED | - |
| `GraphSummaryMode` | `enum` | BASIC, DETAILED | - |
| `ImportTaskStatus` | `enum` | INITIALIZING, EXPORTING, ANALYZING_DATA, IMPORTING, REPROVISIONING, ROLLING_BACK, SUCCEEDED, FAILED, CANCELLING, CANCELLED, DELETED | - |
| `MultiValueHandlingType` | `enum` | TO_LIST, PICK_FIRST | - |
| `ParquetType` | `enum` | COLUMNAR | - |
| `PlanCacheType` | `enum` | ENABLED, DISABLED, AUTO | - |
| `PrivateGraphEndpointStatus` | `enum` | CREATING, AVAILABLE, DELETING, FAILED | - |
| `QueryLanguage` | `enum` | OPEN_CYPHER | - |
| `QueryState` | `enum` | RUNNING, WAITING, CANCELLING | - |
| `QueryStateInput` | `enum` | ALL, RUNNING, WAITING, CANCELLING | - |
| `SnapshotStatus` | `enum` | CREATING, AVAILABLE, DELETING, FAILED | - |
| `UnprocessableExceptionReason` | `enum` | QUERY_TIMEOUT, INTERNAL_LIMIT_EXCEEDED, MEMORY_LIMIT_EXCEEDED, STORAGE_LIMIT_EXCEEDED, PARTITION_FULL | - |
| `ValidationExceptionReason` | `enum` | CONSTRAINT_VIOLATION, ILLEGAL_ARGUMENT, MALFORMED_QUERY, QUERY_CANCELLED, QUERY_TOO_LARGE, UNSUPPORTED_OPERATION, BAD_REQUEST | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
