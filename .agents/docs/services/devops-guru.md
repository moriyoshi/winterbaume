# Amazon DevOps Guru

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon DevOps Guru is a fully managed service that helps you identify anomalous behavior in business critical operational applications. You specify the Amazon Web Services resources that you want DevOps Guru to cover, then the Amazon CloudWatch metrics and Amazon Web Services CloudTrail events related to those resources are analyzed. When anomalous behavior is detected, DevOps Guru creates an insight that includes recommendations, related events, and related metrics that can help you improve your operational applications. For more information, see What is Amazon DevOps Guru. You can specify 1 or 2 Amazon Simple Notification Service topics so you are notified every time a new insight is created.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon DevOps Guru resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon DevOps Guru workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Describe`, `List`, `Update`, `Get`, `Search` operation families, including `DescribeAccountHealth`, `DescribeAccountOverview`, `DescribeAnomaly`, `DescribeEventSourcesConfig`, `ListAnomaliesForInsight`, `ListAnomalousLogGroups`.

## Service Identity and Protocol

- AWS model slug: `devops-guru`
- AWS SDK for Rust slug: `devopsguru`
- Model version: `2020-12-01`
- Model file: `vendor/api-models-aws/models/devops-guru/service/2020-12-01/devops-guru-2020-12-01.json`
- SDK ID: `DevOps Guru`
- Endpoint prefix: `devops-guru`
- ARN namespace: `devops-guru`
- CloudFormation name: `DevOpsGuru`
- CloudTrail event source: `devopsguru.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (11), `List` (8), `Update` (3), `Get` (2), `Search` (2), `Add` (1), `Delete` (1), `Put` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddNotificationChannel`, `DeleteInsight`, `PutFeedback`, `RemoveNotificationChannel`, `StartCostEstimation`, `UpdateEventSourcesConfig`, `UpdateResourceCollection`, `UpdateServiceIntegration`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccountHealth`, `DescribeAccountOverview`, `DescribeAnomaly`, `DescribeEventSourcesConfig`, `DescribeFeedback`, `DescribeInsight`, `DescribeOrganizationHealth`, `DescribeOrganizationOverview`, `DescribeOrganizationResourceCollectionHealth`, `DescribeResourceCollectionHealth`, `DescribeServiceIntegration`, `GetCostEstimation`, `GetResourceCollection`, `ListAnomaliesForInsight`, `ListAnomalousLogGroups`, `ListEvents`, `ListInsights`, `ListMonitoredResources`, `ListNotificationChannels`, `ListOrganizationInsights`, ... (+3).
- Pagination is modelled for 14 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartCostEstimation`.
- 31 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `EventBridge`, `SNS`, `Lambda`, `EC2/VPC`, `ECS`, `RDS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Describe

- Operations: `DescribeAccountHealth`, `DescribeAccountOverview`, `DescribeAnomaly`, `DescribeEventSourcesConfig`, `DescribeFeedback`, `DescribeInsight`, `DescribeOrganizationHealth`, `DescribeOrganizationOverview`, `DescribeOrganizationResourceCollectionHealth`, `DescribeResourceCollectionHealth`, `DescribeServiceIntegration`
- Traits: `paginated` (2)
- Common required input members in this group: `FromTime`, `Id`, `OrganizationResourceCollectionType`, `ResourceCollectionType`

### List

- Operations: `ListAnomaliesForInsight`, `ListAnomalousLogGroups`, `ListEvents`, `ListInsights`, `ListMonitoredResources`, `ListNotificationChannels`, `ListOrganizationInsights`, `ListRecommendations`
- Traits: `paginated` (8)
- Common required input members in this group: `Filters`, `InsightId`, `StatusFilter`

### Update

- Operations: `UpdateEventSourcesConfig`, `UpdateResourceCollection`, `UpdateServiceIntegration`
- Common required input members in this group: `Action`, `ResourceCollection`, `ServiceIntegration`

### Get

- Operations: `GetCostEstimation`, `GetResourceCollection`
- Traits: `paginated` (2)
- Common required input members in this group: `ResourceCollectionType`

### Search

- Operations: `SearchInsights`, `SearchOrganizationInsights`
- Traits: `paginated` (2)
- Common required input members in this group: `AccountIds`, `StartTimeRange`, `Type`

### Add

- Operations: `AddNotificationChannel`
- Common required input members in this group: `Config`

### Delete

- Operations: `DeleteInsight`
- Common required input members in this group: `Id`

### Put

- Operations: `PutFeedback`

### Remove

- Operations: `RemoveNotificationChannel`
- Common required input members in this group: `Id`

### Start

- Operations: `StartCostEstimation`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ResourceCollection`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddNotificationChannel` | `PUT /channels` | - | `Config` | - | `AddNotificationChannelResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds a notification channel to DevOps Guru. A notification channel is used to notify you about important DevOps Guru events, such as when an insight is generated. |
| `DeleteInsight` | `DELETE /insights/{Id}` | - | `Id` | - | `DeleteInsightResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the insight along with the associated anomalies, events and recommendations. |
| `DescribeAccountHealth` | `GET /accounts/health` | - | - | - | `DescribeAccountHealthResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns the number of open reactive insights, the number of open proactive insights, and the number of metrics analyzed in your Amazon Web Services account. Use these numbers to gauge the health of operations in your Amazon Web Services account. |
| `DescribeAccountOverview` | `POST /accounts/overview` | - | `FromTime` | - | `DescribeAccountOverviewResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | For the time range passed in, returns the number of open reactive insight that were created, the number of open proactive insights that were created, and the Mean Time to Recover (MTTR) for all closed reactive insights. |
| `DescribeAnomaly` | `GET /anomalies/{Id}` | - | `Id` | - | `DescribeAnomalyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns details about an anomaly that you specify using its ID. |
| `DescribeEventSourcesConfig` | `POST /event-sources` | - | - | - | `DescribeEventSourcesConfigResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns the integration status of services that are integrated with DevOps Guru as Consumer via EventBridge. The one service that can be integrated with DevOps Guru is Amazon CodeGuru Profiler, which can produce proactive recommendations which can be stored... |
| `DescribeFeedback` | `POST /feedback` | - | - | - | `DescribeFeedbackResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the most recent feedback submitted in the current Amazon Web Services account and Region. |
| `DescribeInsight` | `GET /insights/{Id}` | - | `Id` | - | `DescribeInsightResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns details about an insight that you specify using its ID. |
| `DescribeOrganizationHealth` | `POST /organization/health` | - | - | - | `DescribeOrganizationHealthResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns active insights, predictive insights, and resource hours analyzed in last hour. |
| `DescribeOrganizationOverview` | `POST /organization/overview` | - | `FromTime` | - | `DescribeOrganizationOverviewResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns an overview of your organization's history based on the specified time range. The overview includes the total reactive and proactive insights. |
| `DescribeOrganizationResourceCollectionHealth` | `POST /organization/health/resource-collection` | `paginated` | `OrganizationResourceCollectionType` | - | `DescribeOrganizationResourceCollectionHealthResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Provides an overview of your system's health. If additional member accounts are part of your organization, you can filter those accounts using the `AccountIds` field. |
| `DescribeResourceCollectionHealth` | `GET /accounts/health/resource-collection/{ResourceCollectionType}` | `paginated` | `ResourceCollectionType` | - | `DescribeResourceCollectionHealthResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns the number of open proactive insights, open reactive insights, and the Mean Time to Recover (MTTR) for all closed insights in resource collections in your account. You specify the type of Amazon Web Services resources collection. |
| `DescribeServiceIntegration` | `GET /service-integrations` | - | - | - | `DescribeServiceIntegrationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the integration status of services that are integrated with DevOps Guru. The one service that can be integrated with DevOps Guru is Amazon Web Services Systems Manager, which can be used to create an OpsItem for each generated insight. |
| `GetCostEstimation` | `GET /cost-estimation` | `paginated` | - | - | `GetCostEstimationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns an estimate of the monthly cost for DevOps Guru to analyze your Amazon Web Services resources. For more information, see Estimate your Amazon DevOps Guru costs and Amazon DevOps Guru pricing. |
| `GetResourceCollection` | `GET /resource-collections/{ResourceCollectionType}` | `paginated` | `ResourceCollectionType` | - | `GetResourceCollectionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns lists Amazon Web Services resources that are of the specified resource collection type. The two types of Amazon Web Services resource collections supported are Amazon Web Services CloudFormation stacks and Amazon Web Services resources that contain... |
| `ListAnomaliesForInsight` | `POST /anomalies/insight/{InsightId}` | `paginated` | `InsightId` | - | `ListAnomaliesForInsightResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of the anomalies that belong to an insight that you specify using its ID. |
| `ListAnomalousLogGroups` | `POST /list-log-anomalies` | `paginated` | `InsightId` | - | `ListAnomalousLogGroupsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the list of log groups that contain log anomalies. |
| `ListEvents` | `POST /events` | `paginated` | `Filters` | - | `ListEventsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of the events emitted by the resources that are evaluated by DevOps Guru. You can use filters to specify which events are returned. |
| `ListInsights` | `POST /insights` | `paginated` | `StatusFilter` | - | `ListInsightsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of insights in your Amazon Web Services account. You can specify which insights are returned by their start time and status (`ONGOING`, `CLOSED`, or `ANY`). |
| `ListMonitoredResources` | `POST /monitoredResources` | `paginated` | - | - | `ListMonitoredResourcesResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the list of all log groups that are being monitored and tagged by DevOps Guru. |
| `ListNotificationChannels` | `POST /channels` | `paginated` | - | - | `ListNotificationChannelsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of notification channels configured for DevOps Guru. Each notification channel is used to notify you when DevOps Guru generates an insight that contains information about how to improve your operations. |
| `ListOrganizationInsights` | `POST /organization/insights` | `paginated` | `StatusFilter` | - | `ListOrganizationInsightsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of insights associated with the account or OU Id. |
| `ListRecommendations` | `POST /recommendations` | `paginated` | `InsightId` | - | `ListRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of a specified insight's recommendations. Each recommendation includes a list of related metrics and a list of related events. |
| `PutFeedback` | `PUT /feedback` | - | - | - | `PutFeedbackResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Collects customer feedback about the specified insight. |
| `RemoveNotificationChannel` | `DELETE /channels/{Id}` | - | `Id` | - | `RemoveNotificationChannelResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a notification channel from DevOps Guru. A notification channel is used to notify you when DevOps Guru generates an insight that contains information about how to improve your operations. |
| `SearchInsights` | `POST /insights/search` | `paginated` | `StartTimeRange`, `Type` | - | `SearchInsightsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of insights in your Amazon Web Services account. You can specify which insights are returned by their start time, one or more statuses (`ONGOING` or `CLOSED`), one or more severities (`LOW`, `MEDIUM`, and `HIGH`), and type (`REACTIVE` or... |
| `SearchOrganizationInsights` | `POST /organization/insights/search` | `paginated` | `AccountIds`, `StartTimeRange`, `Type` | - | `SearchOrganizationInsightsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of insights in your organization. You can specify which insights are returned by their start time, one or more statuses (`ONGOING`, `CLOSED`, and `CLOSED`), one or more severities (`LOW`, `MEDIUM`, and `HIGH`), and type (`REACTIVE` or... |
| `StartCostEstimation` | `PUT /cost-estimation` | `idempotency-token` | `ResourceCollection` | `ClientToken` | `StartCostEstimationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts the creation of an estimate of the monthly cost to analyze your Amazon Web Services resources. |
| `UpdateEventSourcesConfig` | `PUT /event-sources` | - | - | - | `UpdateEventSourcesConfigResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Enables or disables integration with a service that can be integrated with DevOps Guru. The one service that can be integrated with DevOps Guru is Amazon CodeGuru Profiler, which can produce proactive recommendations which can be stored and viewed in DevOps... |
| `UpdateResourceCollection` | `PUT /resource-collections` | - | `Action`, `ResourceCollection` | - | `UpdateResourceCollectionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Updates the collection of resources that DevOps Guru analyzes. The two types of Amazon Web Services resource collections supported are Amazon Web Services CloudFormation stacks and Amazon Web Services resources that contain the same Amazon Web Services tag. |
| `UpdateServiceIntegration` | `PUT /service-integrations` | - | `ServiceIntegration` | - | `UpdateServiceIntegrationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Enables or disables integration with a service that can be integrated with DevOps Guru. The one service that can be integrated with DevOps Guru is Amazon Web Services Systems Manager, which can be used to create an OpsItem for each generated insight. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `Message`, `RetryAfterSeconds` | An internal failure in an Amazon service occurred. |
| `ThrottlingException` | `structure` | `Message`, `QuotaCode`, `RetryAfterSeconds`, `ServiceCode` | The request was denied due to a request throttling. |
| `ValidationException` | `structure` | `Fields`, `Message`, `Reason` | Contains information about data passed in to a field during a request that is not valid. |
| `AccessDeniedException` | `structure` | `Message` | You don't have permissions to perform the requested operation. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId`, `ResourceType` | A requested resource could not be found |
| `ConflictException` | `structure` | `Message`, `ResourceId`, `ResourceType` | An exception that is thrown when a conflict occurs. |
| `AddNotificationChannelRequest` | `structure` | `Config` | - |
| `AddNotificationChannelResponse` | `structure` | `Id` | - |
| `ServiceQuotaExceededException` | `structure` | `Message` | The request contains a value that exceeds a maximum quota. |
| `DeleteInsightRequest` | `structure` | `Id` | - |
| `DeleteInsightResponse` | `structure` | - | - |
| `DescribeAccountHealthRequest` | `structure` | - | - |
| `DescribeAccountHealthResponse` | `structure` | `AnalyzedResourceCount`, `MetricsAnalyzed`, `OpenProactiveInsights`, `OpenReactiveInsights`, `ResourceHours` | - |
| `DescribeAccountOverviewRequest` | `structure` | `FromTime`, `ToTime` | - |
| `DescribeAccountOverviewResponse` | `structure` | `MeanTimeToRecoverInMilliseconds`, `ProactiveInsights`, `ReactiveInsights` | - |
| `DescribeAnomalyRequest` | `structure` | `AccountId`, `Id` | - |
| `DescribeAnomalyResponse` | `structure` | `ProactiveAnomaly`, `ReactiveAnomaly` | - |
| `DescribeEventSourcesConfigRequest` | `structure` | - | - |
| `DescribeEventSourcesConfigResponse` | `structure` | `EventSources` | - |
| `DescribeFeedbackRequest` | `structure` | `InsightId` | - |
| `DescribeFeedbackResponse` | `structure` | `InsightFeedback` | - |
| `DescribeInsightRequest` | `structure` | `AccountId`, `Id` | - |
| `DescribeInsightResponse` | `structure` | `ProactiveInsight`, `ReactiveInsight` | - |
| `DescribeOrganizationHealthRequest` | `structure` | `AccountIds`, `OrganizationalUnitIds` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
