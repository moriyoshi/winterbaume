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
- Common required input members in this group: `AnalysisReportId`, `EndTime`, `Group`, `GroupIdentifier`, `Identifier`, `MetricQueries`, `ServiceType`, `StartTime`

### List

- Operations: `ListAvailableResourceDimensions`, `ListAvailableResourceMetrics`, `ListPerformanceAnalysisReports`, `ListTagsForResource`
- Traits: `paginated` (3)
- Common required input members in this group: `Identifier`, `MetricTypes`, `Metrics`, `ResourceARN`, `ServiceType`

### Create

- Operations: `CreatePerformanceAnalysisReport`
- Common required input members in this group: `EndTime`, `Identifier`, `ServiceType`, `StartTime`

### Delete

- Operations: `DeletePerformanceAnalysisReport`
- Common required input members in this group: `AnalysisReportId`, `Identifier`, `ServiceType`

### Describe

- Operations: `DescribeDimensionKeys`
- Traits: `paginated` (1)
- Common required input members in this group: `EndTime`, `GroupBy`, `Identifier`, `Metric`, `ServiceType`, `StartTime`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `ServiceType`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `ServiceType`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreatePerformanceAnalysisReport` | - | - | `EndTime`, `Identifier`, `ServiceType`, `StartTime` | - | `CreatePerformanceAnalysisReportResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Creates a new performance analysis report for a specific time period for the DB instance. |
| `DeletePerformanceAnalysisReport` | - | - | `AnalysisReportId`, `Identifier`, `ServiceType` | - | `DeletePerformanceAnalysisReportResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Deletes a performance analysis report. |
| `DescribeDimensionKeys` | - | `paginated` | `EndTime`, `GroupBy`, `Identifier`, `Metric`, `ServiceType`, `StartTime` | - | `DescribeDimensionKeysResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | For a specific time period, retrieve the top `N` dimension keys for a metric. Each response element returns a maximum of 500 bytes. |
| `GetDimensionKeyDetails` | - | - | `Group`, `GroupIdentifier`, `Identifier`, `ServiceType` | - | `GetDimensionKeyDetailsResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Get the attributes of the specified dimension group for a DB instance or data source. For example, if you specify a SQL ID, `GetDimensionKeyDetails` retrieves the full text of the dimension `db.sql.statement` associated with this ID. |
| `GetPerformanceAnalysisReport` | - | - | `AnalysisReportId`, `Identifier`, `ServiceType` | - | `GetPerformanceAnalysisReportResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Retrieves the report including the report ID, status, time details, and the insights with recommendations. The report status can be `RUNNING`, `SUCCEEDED`, or `FAILED`. |
| `GetResourceMetadata` | - | - | `Identifier`, `ServiceType` | - | `GetResourceMetadataResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Retrieve the metadata for different features. For example, the metadata might indicate that a feature is turned on or off on a specific DB instance. |
| `GetResourceMetrics` | - | `paginated` | `EndTime`, `Identifier`, `MetricQueries`, `ServiceType`, `StartTime` | - | `GetResourceMetricsResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Retrieve Performance Insights metrics for a set of data sources over a time period. You can provide specific dimension groups and dimensions, and provide filtering criteria for each group. |
| `ListAvailableResourceDimensions` | - | `paginated` | `Identifier`, `Metrics`, `ServiceType` | - | `ListAvailableResourceDimensionsResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Retrieve the dimensions that can be queried for each specified metric type on a specified DB instance. |
| `ListAvailableResourceMetrics` | - | `paginated` | `Identifier`, `MetricTypes`, `ServiceType` | - | `ListAvailableResourceMetricsResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Retrieve metrics of the specified types that can be queried for a specified DB instance. |
| `ListPerformanceAnalysisReports` | - | `paginated` | `Identifier`, `ServiceType` | - | `ListPerformanceAnalysisReportsResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Lists all the analysis reports created for the DB instance. The reports are sorted based on the start time of each report. |
| `ListTagsForResource` | - | - | `ResourceARN`, `ServiceType` | - | `ListTagsForResourceResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Retrieves all the metadata tags associated with Amazon RDS Performance Insights resource. |
| `TagResource` | - | - | `ResourceARN`, `ServiceType`, `Tags` | - | `TagResourceResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Adds metadata tags to the Amazon RDS Performance Insights resource. |
| `UntagResource` | - | - | `ResourceARN`, `ServiceType`, `TagKeys` | - | `UntagResourceResponse` | `InternalServiceError`, `InvalidArgumentException`, `NotAuthorizedException` | Deletes the metadata tags from the Amazon RDS Performance Insights resource. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServiceError` | `structure` | `Message` | The request failed due to an unknown error. |
| `InvalidArgumentException` | `structure` | `Message` | One of the arguments provided is invalid for this request. |
| `NotAuthorizedException` | `structure` | `Message` | The user is not authorized to perform this request. |
| `CreatePerformanceAnalysisReportRequest` | `structure` | `EndTime`, `Identifier`, `ServiceType`, `StartTime`, `Tags` | - |
| `CreatePerformanceAnalysisReportResponse` | `structure` | `AnalysisReportId` | - |
| `DeletePerformanceAnalysisReportRequest` | `structure` | `AnalysisReportId`, `Identifier`, `ServiceType` | - |
| `DeletePerformanceAnalysisReportResponse` | `structure` | - | - |
| `DescribeDimensionKeysRequest` | `structure` | `AdditionalMetrics`, `EndTime`, `Filter`, `GroupBy`, `Identifier`, `MaxResults`, `Metric`, `NextToken`, `PartitionBy`, `PeriodInSeconds`, `ServiceType`, `StartTime` | - |
| `DescribeDimensionKeysResponse` | `structure` | `AlignedEndTime`, `AlignedStartTime`, `Keys`, `NextToken`, `PartitionKeys` | - |
| `GetDimensionKeyDetailsRequest` | `structure` | `Group`, `GroupIdentifier`, `Identifier`, `RequestedDimensions`, `ServiceType` | - |
| `GetDimensionKeyDetailsResponse` | `structure` | `Dimensions` | - |
| `GetPerformanceAnalysisReportRequest` | `structure` | `AcceptLanguage`, `AnalysisReportId`, `Identifier`, `ServiceType`, `TextFormat` | - |
| `GetPerformanceAnalysisReportResponse` | `structure` | `AnalysisReport` | - |
| `GetResourceMetadataRequest` | `structure` | `Identifier`, `ServiceType` | - |
| `GetResourceMetadataResponse` | `structure` | `Features`, `Identifier` | - |
| `GetResourceMetricsRequest` | `structure` | `EndTime`, `Identifier`, `MaxResults`, `MetricQueries`, `NextToken`, `PeriodAlignment`, `PeriodInSeconds`, `ServiceType`, `StartTime` | - |
| `GetResourceMetricsResponse` | `structure` | `AlignedEndTime`, `AlignedStartTime`, `Identifier`, `MetricList`, `NextToken` | - |
| `ListAvailableResourceDimensionsRequest` | `structure` | `AuthorizedActions`, `Identifier`, `MaxResults`, `Metrics`, `NextToken`, `ServiceType` | - |
| `ListAvailableResourceDimensionsResponse` | `structure` | `MetricDimensions`, `NextToken` | - |
| `ListAvailableResourceMetricsRequest` | `structure` | `Identifier`, `MaxResults`, `MetricTypes`, `NextToken`, `ServiceType` | - |
| `ListAvailableResourceMetricsResponse` | `structure` | `Metrics`, `NextToken` | - |
| `ListPerformanceAnalysisReportsRequest` | `structure` | `Identifier`, `ListTags`, `MaxResults`, `NextToken`, `ServiceType` | - |
| `ListPerformanceAnalysisReportsResponse` | `structure` | `AnalysisReports`, `NextToken` | - |
| `ListTagsForResourceRequest` | `structure` | `ResourceARN`, `ServiceType` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
