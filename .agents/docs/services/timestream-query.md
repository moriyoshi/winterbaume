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
- Common required input members in this group: `ScheduledQueryArn`

### List

- Operations: `ListScheduledQueries`, `ListTagsForResource`
- Traits: `paginated` (2)
- Common required input members in this group: `ResourceARN`

### Update

- Operations: `UpdateAccountSettings`, `UpdateScheduledQuery`
- Traits: `idempotent` (1)
- Common required input members in this group: `ScheduledQueryArn`, `State`

### Cancel

- Operations: `CancelQuery`
- Traits: `idempotent` (1)
- Common required input members in this group: `QueryId`

### Create

- Operations: `CreateScheduledQuery`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `ErrorReportConfiguration`, `Name`, `NotificationConfiguration`, `QueryString`, `ScheduleConfiguration`, `ScheduledQueryExecutionRoleArn`

### Delete

- Operations: `DeleteScheduledQuery`
- Traits: `idempotent` (1)
- Common required input members in this group: `ScheduledQueryArn`

### Execute

- Operations: `ExecuteScheduledQuery`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `InvocationTime`, `ScheduledQueryArn`

### Prepare

- Operations: `PrepareQuery`
- Traits: `idempotent` (1)
- Common required input members in this group: `QueryString`

### Query

- Operations: `Query`
- Traits: `idempotency-token` (1), `idempotent` (1), `paginated` (1)
- Common required input members in this group: `QueryString`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelQuery` | - | `idempotent` | `QueryId` | - | `CancelQueryResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ThrottlingException`, `ValidationException` | Cancels a query that has been issued. Cancellation is provided only if the query has not completed running before the cancellation request was issued. |
| `CreateScheduledQuery` | - | `idempotent`, `idempotency-token` | `ErrorReportConfiguration`, `Name`, `NotificationConfiguration`, `QueryString`, `ScheduleConfiguration`, `ScheduledQueryExecutionRoleArn` | `ClientToken` | `CreateScheduledQueryResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidEndpointException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create a scheduled query that will be run on your behalf at the configured schedule. Timestream assumes the execution role provided as part of the `ScheduledQueryExecutionRoleArn` parameter to run the query. |
| `DeleteScheduledQuery` | - | `idempotent` | `ScheduledQueryArn` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a given scheduled query. This is an irreversible operation. |
| `DescribeAccountSettings` | - | - | - | - | `DescribeAccountSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ThrottlingException` | Describes the settings for your account that include the query pricing model and the configured maximum TCUs the service can use for your query workload. You're charged only for the duration of compute units used for your workloads. |
| `DescribeEndpoints` | - | - | - | - | `DescribeEndpointsResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | DescribeEndpoints returns a list of available endpoints to make Timestream API calls against. This API is available through both Write and Query. |
| `DescribeScheduledQuery` | - | - | `ScheduledQueryArn` | - | `DescribeScheduledQueryResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provides detailed information about a scheduled query. |
| `ExecuteScheduledQuery` | - | `idempotent`, `idempotency-token` | `InvocationTime`, `ScheduledQueryArn` | `ClientToken` | `Unit` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | You can use this API to run a scheduled query manually. If you enabled `QueryInsights`, this API also returns insights and metrics related to the query that you executed as part of an Amazon SNS notification. |
| `ListScheduledQueries` | - | `paginated` | - | - | `ListScheduledQueriesResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ThrottlingException`, `ValidationException` | Gets a list of all scheduled queries in the caller's Amazon account and Region. `ListScheduledQueries` is eventually consistent. |
| `ListTagsForResource` | - | `paginated` | `ResourceARN` | - | `ListTagsForResourceResponse` | `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List all tags on a Timestream query resource. |
| `PrepareQuery` | - | `idempotent` | `QueryString` | - | `PrepareQueryResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ThrottlingException`, `ValidationException` | A synchronous operation that allows you to submit a query with parameters to be stored by Timestream for later running. Timestream only supports using this operation with `ValidateOnly` set to `true`. |
| `Query` | - | `idempotent`, `paginated`, `idempotency-token` | `QueryString` | `ClientToken` | `QueryResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidEndpointException`, `QueryExecutionException`, `ThrottlingException`, `ValidationException` | `Query` is a synchronous operation that enables you to run a query against your Amazon Timestream data. If you enabled `QueryInsights`, this API also returns insights and metrics related to the query that you executed. |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `InvalidEndpointException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associate a set of tags with a Timestream resource. You can then activate these user-defined tags so that they appear on the Billing and Cost Management console for cost allocation tracking. |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the association of tags from a Timestream query resource. |
| `UpdateAccountSettings` | - | `idempotent` | - | - | `UpdateAccountSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ThrottlingException`, `ValidationException` | Transitions your account to use TCUs for query pricing and modifies the maximum query compute units that you've configured. If you reduce the value of `MaxQueryTCU` to a desired configuration, the new value can take up to 24 hours to be effective. |
| `UpdateScheduledQuery` | - | - | `ScheduledQueryArn`, `State` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `InvalidEndpointException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a scheduled query. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ThrottlingException` | `structure` | `Message` | The request was throttled due to excessive requests. |
| `InvalidEndpointException` | `structure` | `Message` | The requested endpoint is invalid. |
| `ValidationException` | `structure` | `Message` | Invalid or malformed request. |
| `InternalServerException` | `structure` | `Message` | An internal server error occurred while processing the request. |
| `AccessDeniedException` | `structure` | `Message` | You do not have the necessary permissions to access the account settings. |
| `ResourceNotFoundException` | `structure` | `Message`, `ScheduledQueryArn` | The requested resource could not be found. |
| `ConflictException` | `structure` | `Message` | Unable to poll results for a cancelled query. |
| `ServiceQuotaExceededException` | `structure` | `Message` | You have exceeded the service quota. |
| `CancelQueryRequest` | `structure` | `QueryId` | - |
| `CancelQueryResponse` | `structure` | `CancellationMessage` | - |
| `CreateScheduledQueryRequest` | `structure` | `ClientToken`, `ErrorReportConfiguration`, `KmsKeyId`, `Name`, `NotificationConfiguration`, `QueryString`, `ScheduleConfiguration`, `ScheduledQueryExecutionRoleArn`, `Tags`, `TargetConfiguration` | - |
| `CreateScheduledQueryResponse` | `structure` | `Arn` | - |
| `DeleteScheduledQueryRequest` | `structure` | `ScheduledQueryArn` | - |
| `DescribeAccountSettingsRequest` | `structure` | - | - |
| `DescribeAccountSettingsResponse` | `structure` | `MaxQueryTCU`, `QueryCompute`, `QueryPricingModel` | - |
| `DescribeEndpointsRequest` | `structure` | - | - |
| `DescribeEndpointsResponse` | `structure` | `Endpoints` | - |
| `DescribeScheduledQueryRequest` | `structure` | `ScheduledQueryArn` | - |
| `DescribeScheduledQueryResponse` | `structure` | `ScheduledQuery` | - |
| `ExecuteScheduledQueryRequest` | `structure` | `ClientToken`, `InvocationTime`, `QueryInsights`, `ScheduledQueryArn` | - |
| `ListScheduledQueriesRequest` | `structure` | `MaxResults`, `NextToken` | - |
| `ListScheduledQueriesResponse` | `structure` | `NextToken`, `ScheduledQueries` | - |
| `ListTagsForResourceRequest` | `structure` | `MaxResults`, `NextToken`, `ResourceARN` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
