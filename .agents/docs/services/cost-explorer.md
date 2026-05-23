# AWS Cost Explorer Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

You can use the Cost Explorer API to programmatically query your cost and usage data. You can query for aggregated data such as total monthly costs or total daily usage. You can also query for granular data. This might include the number of daily write operations for Amazon DynamoDB database tables in your production environment. Service Endpoint The Cost Explorer API provides the following endpoint: `https://ce.us-east-1.amazonaws.com` For information about the costs that are associated with the Cost Explorer API, see Amazon Web Services Cost Management Pricing.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Cost Explorer Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- From the AWS documentation and model: represent documented AWS Cost Explorer Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Update`, `Create`, `Delete` operation families, including `GetAnomalies`, `GetAnomalyMonitors`, `GetAnomalySubscriptions`, `GetApproximateUsageRecords`, `ListCommitmentPurchaseAnalyses`, `ListCostAllocationTagBackfillHistory`.

## Service Identity and Protocol

- AWS model slug: `cost-explorer`
- AWS SDK for Rust slug: `costexplorer`
- Model version: `2017-10-25`
- Model file: `vendor/api-models-aws/models/cost-explorer/service/2017-10-25/cost-explorer-2017-10-25.json`
- SDK ID: `Cost Explorer`
- Endpoint prefix: `ce`
- ARN namespace: `ce`
- CloudFormation name: `CostExplorer`
- CloudTrail event source: `costexplorer.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (23), `List` (7), `Update` (4), `Create` (3), `Delete` (3), `Start` (3), `Describe` (1), `Provide` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateAnomalyMonitor`, `CreateAnomalySubscription`, `CreateCostCategoryDefinition`, `DeleteAnomalyMonitor`, `DeleteAnomalySubscription`, `DeleteCostCategoryDefinition`, `StartCommitmentPurchaseAnalysis`, `StartCostAllocationTagBackfill`, `StartSavingsPlansPurchaseRecommendationGeneration`, `TagResource`, `UntagResource`, `UpdateAnomalyMonitor`, `UpdateAnomalySubscription`, `UpdateCostAllocationTagsStatus`, `UpdateCostCategoryDefinition`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeCostCategoryDefinition`, `GetAnomalies`, `GetAnomalyMonitors`, `GetAnomalySubscriptions`, `GetApproximateUsageRecords`, `GetCommitmentPurchaseAnalysis`, `GetCostAndUsage`, `GetCostAndUsageComparisons`, `GetCostAndUsageWithResources`, `GetCostCategories`, `GetCostComparisonDrivers`, `GetCostForecast`, `GetDimensionValues`, `GetReservationCoverage`, `GetReservationPurchaseRecommendation`, `GetReservationUtilization`, `GetRightsizingRecommendation`, `GetSavingsPlanPurchaseRecommendationDetails`, `GetSavingsPlansCoverage`, `GetSavingsPlansPurchaseRecommendation`, ... (+11).
- Pagination is modelled for 15 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetCommitmentPurchaseAnalysis`, `StartCommitmentPurchaseAnalysis`, `StartCostAllocationTagBackfill`, `StartSavingsPlansPurchaseRecommendationGeneration`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 47 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `CloudWatch`, `SNS`, `EC2/VPC`, `RDS`, `Redshift`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/cost-management/latest/userguide/ce-what-is.html
- https://docs.aws.amazon.com/cost-management/latest/userguide/ce-api-best-practices.html
- https://docs.aws.amazon.com/cost-management/latest/userguide/ce-advanced-cost-analysis.html

Research outcomes:
- Cost Explorer analyses AWS costs and usage, with historical data, forecasts, grouping, filtering, and recommendations.
- The console can show up to 13 months of history and forecast future costs up to 18 months.
- Cost Explorer API queries use time periods, granularity, metrics, dimensions, tags, cost categories, filters, and group-by clauses.
- API best practices recommend filtering queries, caching results, and using access roles carefully because API calls and granular data can incur cost and quota impacts.
- Advanced analysis can enable multi-year and hourly/daily granularity depending on settings and availability.
- Cost Explorer recommendations include RI and Savings Plans analyses in related cost management workflows.

Parity implications:
- Model query requests, dimensions, metrics, grouping, filters, pagination, forecast data, recommendation data, and granular-data settings separately.
- Query results should be billing-data backed and time-period aware rather than resource-state instantaneous.
- Forecasts should be separate from historical actuals and should not mutate historical cost data.

## Operation Groups

### Get

- Operations: `GetAnomalies`, `GetAnomalyMonitors`, `GetAnomalySubscriptions`, `GetApproximateUsageRecords`, `GetCommitmentPurchaseAnalysis`, `GetCostAndUsage`, `GetCostAndUsageComparisons`, `GetCostAndUsageWithResources`, `GetCostCategories`, `GetCostComparisonDrivers`, `GetCostForecast`, `GetDimensionValues`, `GetReservationCoverage`, `GetReservationPurchaseRecommendation`, `GetReservationUtilization`, `GetRightsizingRecommendation`, `GetSavingsPlanPurchaseRecommendationDetails`, `GetSavingsPlansCoverage`, `GetSavingsPlansPurchaseRecommendation`, `GetSavingsPlansUtilization`, `GetSavingsPlansUtilizationDetails`, `GetTags`, `GetUsageForecast`
- Traits: `paginated` (9)
- Common required input members in this group: `AnalysisId`, `ApproximationDimension`, `BaselineTimePeriod`, `ComparisonTimePeriod`, `DateInterval`, `Dimension`, `Filter`, `Granularity`, `LookbackPeriodInDays`, `Metric`, `MetricForComparison`, `Metrics`, `PaymentOption`, `RecommendationDetailId`, `SavingsPlansType`, `Service`, `TermInYears`, `TimePeriod`

### List

- Operations: `ListCommitmentPurchaseAnalyses`, `ListCostAllocationTagBackfillHistory`, `ListCostAllocationTags`, `ListCostCategoryDefinitions`, `ListCostCategoryResourceAssociations`, `ListSavingsPlansPurchaseRecommendationGeneration`, `ListTagsForResource`
- Traits: `paginated` (6)
- Common required input members in this group: `ResourceArn`

### Update

- Operations: `UpdateAnomalyMonitor`, `UpdateAnomalySubscription`, `UpdateCostAllocationTagsStatus`, `UpdateCostCategoryDefinition`
- Common required input members in this group: `CostAllocationTagsStatus`, `CostCategoryArn`, `MonitorArn`, `RuleVersion`, `Rules`, `SubscriptionArn`

### Create

- Operations: `CreateAnomalyMonitor`, `CreateAnomalySubscription`, `CreateCostCategoryDefinition`
- Common required input members in this group: `AnomalyMonitor`, `AnomalySubscription`, `Name`, `RuleVersion`, `Rules`

### Delete

- Operations: `DeleteAnomalyMonitor`, `DeleteAnomalySubscription`, `DeleteCostCategoryDefinition`
- Common required input members in this group: `CostCategoryArn`, `MonitorArn`, `SubscriptionArn`

### Start

- Operations: `StartCommitmentPurchaseAnalysis`, `StartCostAllocationTagBackfill`, `StartSavingsPlansPurchaseRecommendationGeneration`
- Common required input members in this group: `BackfillFrom`, `CommitmentPurchaseAnalysisConfiguration`

### Describe

- Operations: `DescribeCostCategoryDefinition`
- Common required input members in this group: `CostCategoryArn`

### Provide

- Operations: `ProvideAnomalyFeedback`
- Common required input members in this group: `AnomalyId`, `Feedback`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `ResourceTags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `ResourceTagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateAnomalyMonitor` | - | - | `AnomalyMonitor` | - | `CreateAnomalyMonitorResponse` | `LimitExceededException` | Creates a new cost anomaly detection monitor with the requested type and monitor specification. |
| `CreateAnomalySubscription` | - | - | `AnomalySubscription` | - | `CreateAnomalySubscriptionResponse` | `LimitExceededException`, `UnknownMonitorException` | Adds an alert subscription to a cost anomaly detection monitor. You can use each subscription to define subscribers with email or SNS notifications. |
| `CreateCostCategoryDefinition` | - | - | `Name`, `RuleVersion`, `Rules` | - | `CreateCostCategoryDefinitionResponse` | `LimitExceededException`, `ServiceQuotaExceededException` | Creates a new cost category with the requested name and rules. |
| `DeleteAnomalyMonitor` | - | - | `MonitorArn` | - | `DeleteAnomalyMonitorResponse` | `LimitExceededException`, `UnknownMonitorException` | Deletes a cost anomaly monitor. |
| `DeleteAnomalySubscription` | - | - | `SubscriptionArn` | - | `DeleteAnomalySubscriptionResponse` | `LimitExceededException`, `UnknownSubscriptionException` | Deletes a cost anomaly subscription. |
| `DeleteCostCategoryDefinition` | - | - | `CostCategoryArn` | - | `DeleteCostCategoryDefinitionResponse` | `LimitExceededException`, `ResourceNotFoundException` | Deletes a cost category. Expenses from this month going forward will no longer be categorized with this cost category. |
| `DescribeCostCategoryDefinition` | - | - | `CostCategoryArn` | - | `DescribeCostCategoryDefinitionResponse` | `LimitExceededException`, `ResourceNotFoundException` | Returns the name, Amazon Resource Name (ARN), rules, definition, and effective dates of a cost category that's defined in the account. You have the option to use `EffectiveOn` to return a cost category that's active on a specific date. |
| `GetAnomalies` | - | `paginated` | `DateInterval` | - | `GetAnomaliesResponse` | `InvalidNextTokenException`, `LimitExceededException` | Retrieves all of the cost anomalies detected on your account during the time period that's specified by the `DateInterval` object. Anomalies are available for up to 90 days. |
| `GetAnomalyMonitors` | - | `paginated` | - | - | `GetAnomalyMonitorsResponse` | `InvalidNextTokenException`, `LimitExceededException`, `UnknownMonitorException` | Retrieves the cost anomaly monitor definitions for your account. You can filter using a list of cost anomaly monitor Amazon Resource Names (ARNs). |
| `GetAnomalySubscriptions` | - | `paginated` | - | - | `GetAnomalySubscriptionsResponse` | `InvalidNextTokenException`, `LimitExceededException`, `UnknownSubscriptionException` | Retrieves the cost anomaly subscription objects for your account. You can filter using a list of cost anomaly monitor Amazon Resource Names (ARNs). |
| `GetApproximateUsageRecords` | - | - | `ApproximationDimension`, `Granularity` | - | `GetApproximateUsageRecordsResponse` | `DataUnavailableException`, `LimitExceededException` | Retrieves estimated usage records for hourly granularity or resource-level data at daily granularity. |
| `GetCommitmentPurchaseAnalysis` | - | - | `AnalysisId` | - | `GetCommitmentPurchaseAnalysisResponse` | `AnalysisNotFoundException`, `DataUnavailableException`, `LimitExceededException` | Retrieves a commitment purchase analysis result based on the `AnalysisId`. |
| `GetCostAndUsage` | - | - | `Granularity`, `Metrics`, `TimePeriod` | - | `GetCostAndUsageResponse` | `BillExpirationException`, `BillingViewHealthStatusException`, `DataUnavailableException`, `InvalidNextTokenException`, `LimitExceededException`, `RequestChangedException`, `ResourceNotFoundException` | Retrieves cost and usage metrics for your account. You can specify which cost and usage-related metric that you want the request to return. |
| `GetCostAndUsageComparisons` | - | `paginated` | `BaselineTimePeriod`, `ComparisonTimePeriod`, `MetricForComparison` | - | `GetCostAndUsageComparisonsResponse` | `BillingViewHealthStatusException`, `DataUnavailableException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException` | Retrieves cost and usage comparisons for your account between two periods within the last 13 months. If you have enabled multi-year data at monthly granularity, you can go back up to 38 months. |
| `GetCostAndUsageWithResources` | - | - | `Filter`, `Granularity`, `TimePeriod` | - | `GetCostAndUsageWithResourcesResponse` | `BillExpirationException`, `BillingViewHealthStatusException`, `DataUnavailableException`, `InvalidNextTokenException`, `LimitExceededException`, `RequestChangedException`, `ResourceNotFoundException` | Retrieves cost and usage metrics with resources for your account. You can specify which cost and usage-related metric, such as `BlendedCosts` or `UsageQuantity`, that you want the request to return. |
| `GetCostCategories` | - | - | `TimePeriod` | - | `GetCostCategoriesResponse` | `BillExpirationException`, `BillingViewHealthStatusException`, `DataUnavailableException`, `InvalidNextTokenException`, `LimitExceededException`, `RequestChangedException`, `ResourceNotFoundException` | Retrieves an array of cost category names and values incurred cost. If some cost category names and values are not associated with any cost, they will not be returned by this API. |
| `GetCostComparisonDrivers` | - | `paginated` | `BaselineTimePeriod`, `ComparisonTimePeriod`, `MetricForComparison` | - | `GetCostComparisonDriversResponse` | `BillingViewHealthStatusException`, `DataUnavailableException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceNotFoundException` | Retrieves key factors driving cost changes between two time periods within the last 13 months, such as usage changes, discount changes, and commitment-based savings. If you have enabled multi-year data at monthly granularity, you can go back up to 38 months. |
| `GetCostForecast` | - | - | `Granularity`, `Metric`, `TimePeriod` | - | `GetCostForecastResponse` | `BillingViewHealthStatusException`, `DataUnavailableException`, `LimitExceededException`, `ResourceNotFoundException` | Retrieves a forecast for how much Amazon Web Services predicts that you will spend over the forecast time period that you select, based on your past costs. |
| `GetDimensionValues` | - | - | `Dimension`, `TimePeriod` | - | `GetDimensionValuesResponse` | `BillExpirationException`, `BillingViewHealthStatusException`, `DataUnavailableException`, `InvalidNextTokenException`, `LimitExceededException`, `RequestChangedException`, `ResourceNotFoundException` | Retrieves all available filter values for a specified filter over a period of time. You can search the dimension values for an arbitrary string. |
| `GetReservationCoverage` | - | - | `TimePeriod` | - | `GetReservationCoverageResponse` | `DataUnavailableException`, `InvalidNextTokenException`, `LimitExceededException` | Retrieves the reservation coverage for your account, which you can use to see how much of your Amazon Elastic Compute Cloud, Amazon ElastiCache, Amazon Relational Database Service, or Amazon Redshift usage is covered by a reservation. An organization's... |
| `GetReservationPurchaseRecommendation` | - | `paginated` | `Service` | - | `GetReservationPurchaseRecommendationResponse` | `DataUnavailableException`, `InvalidNextTokenException`, `LimitExceededException` | Gets recommendations for reservation purchases. These recommendations might help you to reduce your costs. |
| `GetReservationUtilization` | - | - | `TimePeriod` | - | `GetReservationUtilizationResponse` | `DataUnavailableException`, `InvalidNextTokenException`, `LimitExceededException` | Retrieves the reservation utilization for your account. Management account in an organization have access to member accounts. |
| `GetRightsizingRecommendation` | - | `paginated` | `Service` | - | `GetRightsizingRecommendationResponse` | `InvalidNextTokenException`, `LimitExceededException` | Creates recommendations that help you save cost by identifying idle and underutilized Amazon EC2 instances. Recommendations are generated to either downsize or terminate instances, along with providing savings detail and metrics. |
| `GetSavingsPlanPurchaseRecommendationDetails` | - | - | `RecommendationDetailId` | - | `GetSavingsPlanPurchaseRecommendationDetailsResponse` | `DataUnavailableException`, `LimitExceededException` | Retrieves the details for a Savings Plan recommendation. These details include the hourly data-points that construct the cost, coverage, and utilization charts. |
| `GetSavingsPlansCoverage` | - | `paginated` | `TimePeriod` | - | `GetSavingsPlansCoverageResponse` | `DataUnavailableException`, `InvalidNextTokenException`, `LimitExceededException` | Retrieves the Savings Plans covered for your account. This enables you to see how much of your cost is covered by a Savings Plan. |
| `GetSavingsPlansPurchaseRecommendation` | - | - | `LookbackPeriodInDays`, `PaymentOption`, `SavingsPlansType`, `TermInYears` | - | `GetSavingsPlansPurchaseRecommendationResponse` | `InvalidNextTokenException`, `LimitExceededException` | Retrieves the Savings Plans recommendations for your account. First use `StartSavingsPlansPurchaseRecommendationGeneration` to generate a new set of recommendations, and then use `GetSavingsPlansPurchaseRecommendation` to retrieve them. |
| `GetSavingsPlansUtilization` | - | - | `TimePeriod` | - | `GetSavingsPlansUtilizationResponse` | `DataUnavailableException`, `LimitExceededException` | Retrieves the Savings Plans utilization for your account across date ranges with daily or monthly granularity. Management account in an organization have access to member accounts. |
| `GetSavingsPlansUtilizationDetails` | - | `paginated` | `TimePeriod` | - | `GetSavingsPlansUtilizationDetailsResponse` | `DataUnavailableException`, `InvalidNextTokenException`, `LimitExceededException` | Retrieves attribute data along with aggregate utilization and savings data for a given time period. This doesn't support granular or grouped data (daily/monthly) in response. |
| `GetTags` | - | - | `TimePeriod` | - | `GetTagsResponse` | `BillExpirationException`, `BillingViewHealthStatusException`, `DataUnavailableException`, `InvalidNextTokenException`, `LimitExceededException`, `RequestChangedException`, `ResourceNotFoundException` | Queries for available tag keys and tag values for a specified period. You can search the tag values for an arbitrary string. |
| `GetUsageForecast` | - | - | `Granularity`, `Metric`, `TimePeriod` | - | `GetUsageForecastResponse` | `BillingViewHealthStatusException`, `DataUnavailableException`, `LimitExceededException`, `ResourceNotFoundException`, `UnresolvableUsageUnitException` | Retrieves a forecast for how much Amazon Web Services predicts that you will use over the forecast time period that you select, based on your past usage. |
| `ListCommitmentPurchaseAnalyses` | - | `paginated` | - | - | `ListCommitmentPurchaseAnalysesResponse` | `DataUnavailableException`, `InvalidNextTokenException`, `LimitExceededException` | Lists the commitment purchase analyses for your account. |
| `ListCostAllocationTagBackfillHistory` | - | `paginated` | - | - | `ListCostAllocationTagBackfillHistoryResponse` | `InvalidNextTokenException`, `LimitExceededException` | Retrieves a list of your historical cost allocation tag backfill requests. |
| `ListCostAllocationTags` | - | `paginated` | - | - | `ListCostAllocationTagsResponse` | `InvalidNextTokenException`, `LimitExceededException` | Get a list of cost allocation tags. All inputs in the API are optional and serve as filters. |
| `ListCostCategoryDefinitions` | - | `paginated` | - | - | `ListCostCategoryDefinitionsResponse` | `LimitExceededException` | Returns the name, Amazon Resource Name (ARN), `NumberOfRules` and effective dates of all cost categories defined in the account. You have the option to use `EffectiveOn` and `SupportedResourceTypes` to return a list of cost categories that were active on a... |
| `ListCostCategoryResourceAssociations` | - | `paginated` | - | - | `ListCostCategoryResourceAssociationsResponse` | `LimitExceededException`, `ResourceNotFoundException` | Returns resource associations of all cost categories defined in the account. You have the option to use `CostCategoryArn` to get the association for a specific cost category. |
| `ListSavingsPlansPurchaseRecommendationGeneration` | - | `paginated` | - | - | `ListSavingsPlansPurchaseRecommendationGenerationResponse` | `DataUnavailableException`, `InvalidNextTokenException`, `LimitExceededException` | Retrieves a list of your historical recommendation generations within the past 30 days. |
| `ListTagsForResource` | - | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `LimitExceededException`, `ResourceNotFoundException` | Returns a list of resource tags associated with the resource specified by the Amazon Resource Name (ARN). |
| `ProvideAnomalyFeedback` | - | - | `AnomalyId`, `Feedback` | - | `ProvideAnomalyFeedbackResponse` | `LimitExceededException` | Modifies the feedback property of a given cost anomaly. |
| `StartCommitmentPurchaseAnalysis` | - | - | `CommitmentPurchaseAnalysisConfiguration` | - | `StartCommitmentPurchaseAnalysisResponse` | `DataUnavailableException`, `GenerationExistsException`, `LimitExceededException`, `ServiceQuotaExceededException` | Specifies the parameters of a planned commitment purchase and starts the generation of the analysis. This enables you to estimate the cost, coverage, and utilization impact of your planned commitment purchases. |
| `StartCostAllocationTagBackfill` | - | - | `BackfillFrom` | - | `StartCostAllocationTagBackfillResponse` | `BackfillLimitExceededException`, `LimitExceededException` | Request a cost allocation tag backfill. This will backfill the activation status (either `active` or `inactive`) for all tag keys from `para:BackfillFrom` up to the time this request is made. |
| `StartSavingsPlansPurchaseRecommendationGeneration` | - | - | - | - | `StartSavingsPlansPurchaseRecommendationGenerationResponse` | `DataUnavailableException`, `GenerationExistsException`, `LimitExceededException`, `ServiceQuotaExceededException` | Requests a Savings Plans recommendation generation. This enables you to calculate a fresh set of Savings Plans recommendations that takes your latest usage data and current Savings Plans inventory into account. |
| `TagResource` | - | - | `ResourceArn`, `ResourceTags` | - | `TagResourceResponse` | `LimitExceededException`, `ResourceNotFoundException`, `TooManyTagsException` | An API operation for adding one or more tags (key-value pairs) to a resource. You can use the `TagResource` operation with a resource that already has tags. |
| `UntagResource` | - | - | `ResourceArn`, `ResourceTagKeys` | - | `UntagResourceResponse` | `LimitExceededException`, `ResourceNotFoundException` | Removes one or more tags from a resource. Specify only tag keys in your request. |
| `UpdateAnomalyMonitor` | - | - | `MonitorArn` | - | `UpdateAnomalyMonitorResponse` | `LimitExceededException`, `UnknownMonitorException` | Updates an existing cost anomaly monitor. The changes made are applied going forward, and doesn't change anomalies detected in the past. |
| `UpdateAnomalySubscription` | - | - | `SubscriptionArn` | - | `UpdateAnomalySubscriptionResponse` | `LimitExceededException`, `UnknownMonitorException`, `UnknownSubscriptionException` | Updates an existing cost anomaly subscription. Specify the fields that you want to update. |
| `UpdateCostAllocationTagsStatus` | - | - | `CostAllocationTagsStatus` | - | `UpdateCostAllocationTagsStatusResponse` | `LimitExceededException` | Updates status for cost allocation tags in bulk, with maximum batch size of 20. If the tag status that's updated is the same as the existing tag status, the request doesn't fail. |
| `UpdateCostCategoryDefinition` | - | - | `CostCategoryArn`, `RuleVersion`, `Rules` | - | `UpdateCostCategoryDefinitionResponse` | `LimitExceededException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | Updates an existing cost category. Changes made to the cost category rules will be used to categorize the current month’s expenses and future expenses. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `LimitExceededException` | `structure` | `Message` | You made too many calls in a short period of time. |
| `DataUnavailableException` | `structure` | `Message` | The requested data is unavailable. |
| `InvalidNextTokenException` | `structure` | `Message` | The pagination token is invalid. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceName` | The specified ARN in the request doesn't exist. |
| `BillingViewHealthStatusException` | `structure` | `Message` | The billing view status must be `HEALTHY` to perform this action. |
| `UnknownMonitorException` | `structure` | `Message` | The cost anomaly monitor does not exist for the account. |
| `BillExpirationException` | `structure` | `Message` | The requested report expired. |
| `RequestChangedException` | `structure` | `Message` | Your request parameters changed between pages. |
| `ServiceQuotaExceededException` | `structure` | `Message` | You've reached the limit on the number of resources you can create, or exceeded the size of an individual resource. |
| `UnknownSubscriptionException` | `structure` | `Message` | The cost anomaly subscription does not exist for the account. |
| `GenerationExistsException` | `structure` | `Message` | A request to generate a recommendation or analysis is already in progress. |
| `CreateAnomalyMonitorRequest` | `structure` | `AnomalyMonitor`, `ResourceTags` | - |
| `CreateAnomalyMonitorResponse` | `structure` | `MonitorArn` | - |
| `CreateAnomalySubscriptionRequest` | `structure` | `AnomalySubscription`, `ResourceTags` | - |
| `CreateAnomalySubscriptionResponse` | `structure` | `SubscriptionArn` | - |
| `CreateCostCategoryDefinitionRequest` | `structure` | `DefaultValue`, `EffectiveStart`, `Name`, `ResourceTags`, `RuleVersion`, `Rules`, `SplitChargeRules` | - |
| `CreateCostCategoryDefinitionResponse` | `structure` | `CostCategoryArn`, `EffectiveStart` | - |
| `DeleteAnomalyMonitorRequest` | `structure` | `MonitorArn` | - |
| `DeleteAnomalyMonitorResponse` | `structure` | - | - |
| `DeleteAnomalySubscriptionRequest` | `structure` | `SubscriptionArn` | - |
| `DeleteAnomalySubscriptionResponse` | `structure` | - | - |
| `DeleteCostCategoryDefinitionRequest` | `structure` | `CostCategoryArn` | - |
| `DeleteCostCategoryDefinitionResponse` | `structure` | `CostCategoryArn`, `EffectiveEnd` | - |
| `DescribeCostCategoryDefinitionRequest` | `structure` | `CostCategoryArn`, `EffectiveOn` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
