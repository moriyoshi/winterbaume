# AWS Performance Insights

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon RDS Performance Insights Amazon RDS Performance Insights enables you to monitor and explore different dimensions of database load based on data captured from a running DB instance. The guide provides detailed information about Performance Insights data types, parameters and errors. When Performance Insights is enabled, the Amazon RDS Performance Insights API provides visibility into the performance of your DB instance. Amazon CloudWatch provides the authoritative source for Amazon Web Services service-vended monitoring metrics. Performance Insights offers a domain-specific view of DB load.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-pi/tests/scenario_test.rs`: create, describe, list, and delete a performance analysis report.
- From the AWS documentation and model: support database performance investigation workflows using dimension keys, metric queries, analysis reports, and resource metadata for Performance Insights-enabled data stores.

## Service Identity and Protocol

- AWS model slug: `pi`
- AWS SDK for Rust slug: `pi`
- Model version: `2018-02-27`
- Model file: `vendor/api-models-aws/models/pi/service/2018-02-27/pi-2018-02-27.json`
- SDK ID: `PI`
- Endpoint prefix: `pi`
- ARN namespace: `pi`
- CloudFormation name: `PI`
- CloudTrail event source: `pi.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (4), `List` (4), `Create` (1), `Delete` (1), `Describe` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreatePerformanceAnalysisReport`, `DeletePerformanceAnalysisReport`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeDimensionKeys`, `GetDimensionKeyDetails`, `GetPerformanceAnalysisReport`, `GetResourceMetadata`, `GetResourceMetrics`, `ListAvailableResourceDimensions`, `ListAvailableResourceMetrics`, `ListPerformanceAnalysisReports`, `ListTagsForResource`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreatePerformanceAnalysisReport`, `DeletePerformanceAnalysisReport`, `GetPerformanceAnalysisReport`, `ListPerformanceAnalysisReports`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 13 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `CloudWatch`, `EC2/VPC`, `RDS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetDimensionKeyDetails`, `GetPerformanceAnalysisReport`, `GetResourceMetadata`, `GetResourceMetrics`
- Traits: `paginated` (1)
- Common required input members in this group: `ServiceType`, `Identifier`

### List

- Operations: `ListAvailableResourceDimensions`, `ListAvailableResourceMetrics`, `ListPerformanceAnalysisReports`, `ListTagsForResource`
- Traits: `paginated` (3)
- Common required input members in this group: `ServiceType`, `Identifier`

### Create

- Operations: `CreatePerformanceAnalysisReport`
- Common required input members in this group: -

### Delete

- Operations: `DeletePerformanceAnalysisReport`
- Common required input members in this group: -

### Describe

- Operations: `DescribeDimensionKeys`
- Traits: `paginated` (1)
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
| `CreatePerformanceAnalysisReport` | `-` | - | `ServiceType`, `Identifier`, `StartTime`, `EndTime` | - | `CreatePerformanceAnalysisReportResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Creates a new performance analysis report for a specific time period for the DB instance. |
| `DeletePerformanceAnalysisReport` | `-` | - | `ServiceType`, `Identifier`, `AnalysisReportId` | - | `DeletePerformanceAnalysisReportResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Deletes a performance analysis report. |
| `DescribeDimensionKeys` | `-` | `paginated` | `ServiceType`, `Identifier`, `StartTime`, `EndTime`, `Metric`, `GroupBy` | - | `DescribeDimensionKeysResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | For a specific time period, retrieve the top N dimension keys for a metric. Each response element returns a maximum of 500 bytes. For larger elements, such as SQL statements, only the first 500 bytes are returned. |
| `GetDimensionKeyDetails` | `-` | - | `ServiceType`, `Identifier`, `Group`, `GroupIdentifier` | - | `GetDimensionKeyDetailsResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Get the attributes of the specified dimension group for a DB instance or data source. For example, if you specify a SQL ID, GetDimensionKeyDetails retrieves the full text of the dimension db.sql.statement associated ... |
| `GetPerformanceAnalysisReport` | `-` | - | `ServiceType`, `Identifier`, `AnalysisReportId` | - | `GetPerformanceAnalysisReportResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Retrieves the report including the report ID, status, time details, and the insights with recommendations. The report status can be RUNNING , SUCCEEDED , or FAILED . The insights include the description and recommend ... |
| `GetResourceMetadata` | `-` | - | `ServiceType`, `Identifier` | - | `GetResourceMetadataResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Retrieve the metadata for different features. For example, the metadata might indicate that a feature is turned on or off on a specific DB instance. |
| `GetResourceMetrics` | `-` | `paginated` | `ServiceType`, `Identifier`, `MetricQueries`, `StartTime`, `EndTime` | - | `GetResourceMetricsResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Retrieve Performance Insights metrics for a set of data sources over a time period. You can provide specific dimension groups and dimensions, and provide filtering criteria for each group. You must specify an aggrega ... |
| `ListAvailableResourceDimensions` | `-` | `paginated` | `ServiceType`, `Identifier`, `Metrics` | - | `ListAvailableResourceDimensionsResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Retrieve the dimensions that can be queried for each specified metric type on a specified DB instance. |
| `ListAvailableResourceMetrics` | `-` | `paginated` | `ServiceType`, `Identifier`, `MetricTypes` | - | `ListAvailableResourceMetricsResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Retrieve metrics of the specified types that can be queried for a specified DB instance. |
| `ListPerformanceAnalysisReports` | `-` | `paginated` | `ServiceType`, `Identifier` | - | `ListPerformanceAnalysisReportsResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Lists all the analysis reports created for the DB instance. The reports are sorted based on the start time of each report. |
| `ListTagsForResource` | `-` | - | `ServiceType`, `ResourceARN` | - | `ListTagsForResourceResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Retrieves all the metadata tags associated with Amazon RDS Performance Insights resource. |
| `TagResource` | `-` | - | `ServiceType`, `ResourceARN`, `Tags` | - | `TagResourceResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Adds metadata tags to the Amazon RDS Performance Insights resource. |
| `UntagResource` | `-` | - | `ServiceType`, `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Deletes the metadata tags from the Amazon RDS Performance Insights resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServiceError` | `structure` | Message | The request failed due to an unknown error. |
| `InvalidArgumentException` | `structure` | Message | One of the arguments provided is invalid for this request. |
| `NotAuthorizedException` | `structure` | Message | The user is not authorized to perform this request. |
| `CreatePerformanceAnalysisReportRequest` | `structure` | ServiceType, Identifier, StartTime, EndTime, Tags | - |
| `CreatePerformanceAnalysisReportResponse` | `structure` | AnalysisReportId | - |
| `DeletePerformanceAnalysisReportRequest` | `structure` | ServiceType, Identifier, AnalysisReportId | - |
| `DeletePerformanceAnalysisReportResponse` | `structure` | **empty (no members)** | - |
| `DescribeDimensionKeysRequest` | `structure` | ServiceType, Identifier, StartTime, EndTime, Metric, PeriodInSeconds, GroupBy, AdditionalMetrics, PartitionBy, Filter, MaxResults, NextToken | - |
| `DescribeDimensionKeysResponse` | `structure` | AlignedStartTime, AlignedEndTime, PartitionKeys, Keys, NextToken | - |
| `GetDimensionKeyDetailsRequest` | `structure` | ServiceType, Identifier, Group, GroupIdentifier, RequestedDimensions | - |
| `GetDimensionKeyDetailsResponse` | `structure` | Dimensions | - |
| `GetPerformanceAnalysisReportRequest` | `structure` | ServiceType, Identifier, AnalysisReportId, TextFormat, AcceptLanguage | - |
| `GetPerformanceAnalysisReportResponse` | `structure` | AnalysisReport | - |
| `GetResourceMetadataRequest` | `structure` | ServiceType, Identifier | - |
| `GetResourceMetadataResponse` | `structure` | Identifier, Features | - |
| `GetResourceMetricsRequest` | `structure` | ServiceType, Identifier, MetricQueries, StartTime, EndTime, PeriodInSeconds, MaxResults, NextToken, PeriodAlignment | - |
| `GetResourceMetricsResponse` | `structure` | AlignedStartTime, AlignedEndTime, Identifier, MetricList, NextToken | - |
| `ListAvailableResourceDimensionsRequest` | `structure` | ServiceType, Identifier, Metrics, MaxResults, NextToken, AuthorizedActions | - |
| `ListAvailableResourceDimensionsResponse` | `structure` | MetricDimensions, NextToken | - |
| `ListAvailableResourceMetricsRequest` | `structure` | ServiceType, Identifier, MetricTypes, NextToken, MaxResults | - |
| `ListAvailableResourceMetricsResponse` | `structure` | Metrics, NextToken | - |
| `ListPerformanceAnalysisReportsRequest` | `structure` | ServiceType, Identifier, NextToken, MaxResults, ListTags | - |
| `ListPerformanceAnalysisReportsResponse` | `structure` | AnalysisReports, NextToken | - |
| `ListTagsForResourceRequest` | `structure` | ServiceType, ResourceARN | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `TagResourceRequest` | `structure` | ServiceType, ResourceARN, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ServiceType, ResourceARN, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `AcceptLanguage` | `enum` | EN_US | - |
| `AnalysisStatus` | `enum` | RUNNING, SUCCEEDED, FAILED | - |
| `ContextType` | `enum` | CAUSAL, CONTEXTUAL | - |
| `DetailStatus` | `enum` | AVAILABLE, PROCESSING, UNAVAILABLE | - |
| `FeatureStatus` | `enum` | ENABLED, DISABLED, UNSUPPORTED, ENABLED_PENDING_REBOOT, DISABLED_PENDING_REBOOT, UNKNOWN | - |
| `FineGrainedAction` | `enum` | DESCRIBE_DIMENSION_KEYS, GET_DIMENSION_KEY_DETAILS, GET_RESOURCE_METRICS | - |
| `PeriodAlignment` | `enum` | END_TIME, START_TIME | - |
| `ServiceType` | `enum` | RDS, DOCDB | - |
| `Severity` | `enum` | LOW, MEDIUM, HIGH | - |
| `TextFormat` | `enum` | PLAIN_TEXT, MARKDOWN | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
