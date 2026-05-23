# Amazon Timestream Query

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Timestream Query

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for Amazon Timestream Query by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent documented Amazon Timestream Query workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `List`, `Update`, `Cancel`, `Create` operation families, including `DescribeAccountSettings`, `DescribeEndpoints`, `DescribeScheduledQuery`, `ListScheduledQueries`, `ListTagsForResource`, `UpdateAccountSettings`.

## Service Identity and Protocol

- AWS model slug: `timestream-query`
- AWS SDK for Rust slug: `timestreamquery`
- Model version: `2018-11-01`
- Model file: `vendor/api-models-aws/models/timestream-query/service/2018-11-01/timestream-query-2018-11-01.json`
- SDK ID: `Timestream Query`
- Endpoint prefix: `query.timestream`
- ARN namespace: `timestream`
- CloudFormation name: `TimestreamQuery`
- CloudTrail event source: `timestreamquery.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (3), `List` (2), `Update` (2), `Cancel` (1), `Create` (1), `Delete` (1), `Execute` (1), `Prepare` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelQuery`, `CreateScheduledQuery`, `DeleteScheduledQuery`, `TagResource`, `UntagResource`, `UpdateAccountSettings`, `UpdateScheduledQuery`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccountSettings`, `DescribeEndpoints`, `DescribeScheduledQuery`, `ListScheduledQueries`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 7 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelQuery`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 15 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SNS`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Describe

- Operations: `DescribeAccountSettings`, `DescribeEndpoints`, `DescribeScheduledQuery`
- Common required input members in this group: -

### List

- Operations: `ListScheduledQueries`, `ListTagsForResource`
- Traits: `paginated` (2)
- Common required input members in this group: -

### Update

- Operations: `UpdateAccountSettings`, `UpdateScheduledQuery`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Cancel

- Operations: `CancelQuery`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Create

- Operations: `CreateScheduledQuery`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Delete

- Operations: `DeleteScheduledQuery`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Execute

- Operations: `ExecuteScheduledQuery`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Prepare

- Operations: `PrepareQuery`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Query

- Operations: `Query`
- Traits: `idempotent` (1), `paginated` (1), `idempotency-token` (1)
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
| `CancelQuery` | `-` | `idempotent` | `QueryId` | - | `CancelQueryResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ThrottlingException`, `ValidationException` | Cancels a query that has been issued. Cancellation is provided only if the query has not completed running before the cancellation request was issued. Because cancellation is an idempotent operation, subsequent cance ... |
| `CreateScheduledQuery` | `-` | `idempotent`, `idempotency-token` | `Name`, `QueryString`, `ScheduleConfiguration`, `NotificationConfiguration`, `ScheduledQueryExecutionRoleArn`, `ErrorReportConfiguration` | `ClientToken` | `CreateScheduledQueryResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidEndpointException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a scheduled query that will be run on your behalf at the configured schedule. Timestream assumes the execution role provided as part of the ScheduledQueryExecutionRoleArn parameter to run the query. You can us ... |
| `DeleteScheduledQuery` | `-` | `idempotent` | `ScheduledQueryArn` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a given scheduled query. This is an irreversible operation. |
| `DescribeAccountSettings` | `-` | - | - | - | `DescribeAccountSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ThrottlingException` | Describes the settings for your account that include the query pricing model and the configured maximum TCUs the service can use for your query workload. You're charged only for the duration of compute units used for ... |
| `DescribeEndpoints` | `-` | - | - | - | `DescribeEndpointsResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | DescribeEndpoints returns a list of available endpoints to make Timestream API calls against. This API is available through both Write and Query. Because the Timestream SDKs are designed to transparently work with th ... |
| `DescribeScheduledQuery` | `-` | - | `ScheduledQueryArn` | - | `DescribeScheduledQueryResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provides detailed information about a scheduled query. |
| `ExecuteScheduledQuery` | `-` | `idempotent`, `idempotency-token` | `ScheduledQueryArn`, `InvocationTime` | `ClientToken` | `Unit` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | You can use this API to run a scheduled query manually. If you enabled QueryInsights , this API also returns insights and metrics related to the query that you executed as part of an Amazon SNS notification. QueryIns ... |
| `ListScheduledQueries` | `-` | `paginated` | - | - | `ListScheduledQueriesResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ThrottlingException`, `ValidationException` | Gets a list of all scheduled queries in the caller's Amazon account and Region. ListScheduledQueries is eventually consistent. |
| `ListTagsForResource` | `-` | `paginated` | `ResourceARN` | - | `ListTagsForResourceResponse` | `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List all tags on a Timestream query resource. |
| `PrepareQuery` | `-` | `idempotent` | `QueryString` | - | `PrepareQueryResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ThrottlingException`, `ValidationException` | A synchronous operation that allows you to submit a query with parameters to be stored by Timestream for later running. Timestream only supports using this operation with ValidateOnly set to true . |
| `Query` | `-` | `idempotent`, `paginated`, `idempotency-token` | `QueryString` | `ClientToken` | `QueryResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidEndpointException`, `QueryExecutionException`, `ThrottlingException`, `ValidationException` | Query is a synchronous operation that enables you to run a query against your Amazon Timestream data. If you enabled QueryInsights , this API also returns insights and metrics related to the query that you executed. ... |
| `TagResource` | `-` | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `InvalidEndpointException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associate a set of tags with a Timestream resource. You can then activate these user-defined tags so that they appear on the Billing and Cost Management console for cost allocation tracking. |
| `UntagResource` | `-` | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the association of tags from a Timestream query resource. |
| `UpdateAccountSettings` | `-` | `idempotent` | - | - | `UpdateAccountSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ThrottlingException`, `ValidationException` | Transitions your account to use TCUs for query pricing and modifies the maximum query compute units that you've configured. If you reduce the value of MaxQueryTCU to a desired configuration, the new value can take up ... |
| `UpdateScheduledQuery` | `-` | - | `ScheduledQueryArn`, `State` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a scheduled query. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You do not have the necessary permissions to access the account settings. |
| `ConflictException` | `structure` | Message | Unable to poll results for a cancelled query. |
| `InternalServerException` | `structure` | Message | An internal server error occurred while processing the request. |
| `InvalidEndpointException` | `structure` | Message | The requested endpoint is invalid. |
| `QueryExecutionException` | `structure` | Message | Timestream was unable to run the query successfully. |
| `ResourceNotFoundException` | `structure` | Message, ScheduledQueryArn | The requested resource could not be found. |
| `ServiceQuotaExceededException` | `structure` | Message | You have exceeded the service quota. |
| `ThrottlingException` | `structure` | Message | The request was throttled due to excessive requests. |
| `ValidationException` | `structure` | Message | Invalid or malformed request. |
| `CancelQueryRequest` | `structure` | QueryId | - |
| `CancelQueryResponse` | `structure` | CancellationMessage | - |
| `CreateScheduledQueryRequest` | `structure` | Name, QueryString, ScheduleConfiguration, NotificationConfiguration, TargetConfiguration, ClientToken, ScheduledQueryExecutionRoleArn, Tags, KmsKeyId, ErrorReportConfiguration | - |
| `CreateScheduledQueryResponse` | `structure` | Arn | - |
| `DeleteScheduledQueryRequest` | `structure` | ScheduledQueryArn | - |
| `DescribeAccountSettingsRequest` | `structure` | **empty (no members)** | - |
| `DescribeAccountSettingsResponse` | `structure` | MaxQueryTCU, QueryPricingModel, QueryCompute | - |
| `DescribeEndpointsRequest` | `structure` | **empty (no members)** | - |
| `DescribeEndpointsResponse` | `structure` | Endpoints | - |
| `DescribeScheduledQueryRequest` | `structure` | ScheduledQueryArn | - |
| `DescribeScheduledQueryResponse` | `structure` | ScheduledQuery | - |
| `ExecuteScheduledQueryRequest` | `structure` | ScheduledQueryArn, InvocationTime, ClientToken, QueryInsights | - |
| `ListScheduledQueriesRequest` | `structure` | MaxResults, NextToken | - |
| `ListScheduledQueriesResponse` | `structure` | ScheduledQueries, NextToken | - |
| `ListTagsForResourceRequest` | `structure` | ResourceARN, MaxResults, NextToken | - |
| `ListTagsForResourceResponse` | `structure` | Tags, NextToken | - |
| `PrepareQueryRequest` | `structure` | QueryString, ValidateOnly | - |
| `PrepareQueryResponse` | `structure` | QueryString, Columns, Parameters | - |
| `QueryRequest` | `structure` | QueryString, ClientToken, NextToken, MaxRows, QueryInsights | - |
| `QueryResponse` | `structure` | QueryId, NextToken, Rows, ColumnInfo, QueryStatus, QueryInsightsResponse | - |
| `TagResourceRequest` | `structure` | ResourceARN, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceARN, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `UpdateAccountSettingsRequest` | `structure` | MaxQueryTCU, QueryPricingModel, QueryCompute | - |
| `UpdateAccountSettingsResponse` | `structure` | MaxQueryTCU, QueryPricingModel, QueryCompute | - |
| `UpdateScheduledQueryRequest` | `structure` | ScheduledQueryArn, State | - |
| `ComputeMode` | `enum` | ON_DEMAND, PROVISIONED | - |
| `DimensionValueType` | `enum` | VARCHAR | - |
| `LastUpdateStatus` | `enum` | PENDING, FAILED, SUCCEEDED | - |
| `MeasureValueType` | `enum` | BIGINT, BOOLEAN, DOUBLE, VARCHAR, MULTI | - |
| `QueryInsightsMode` | `enum` | ENABLED_WITH_RATE_CONTROL, DISABLED | - |
| `QueryPricingModel` | `enum` | BYTES_SCANNED, COMPUTE_UNITS | - |
| `S3EncryptionOption` | `enum` | SSE_S3, SSE_KMS | - |
| `ScalarMeasureValueType` | `enum` | BIGINT, BOOLEAN, DOUBLE, VARCHAR, TIMESTAMP | - |
| `ScalarType` | `enum` | VARCHAR, BOOLEAN, BIGINT, DOUBLE, TIMESTAMP, DATE, TIME, INTERVAL_DAY_TO_SECOND, INTERVAL_YEAR_TO_MONTH, UNKNOWN, INTEGER | - |
| `ScheduledQueryInsightsMode` | `enum` | ENABLED_WITH_RATE_CONTROL, DISABLED | - |
| `ScheduledQueryRunStatus` | `enum` | AUTO_TRIGGER_SUCCESS, AUTO_TRIGGER_FAILURE, MANUAL_TRIGGER_SUCCESS, MANUAL_TRIGGER_FAILURE | - |
| `ScheduledQueryState` | `enum` | ENABLED, DISABLED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
