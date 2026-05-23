# Amazon CloudWatch Application Insights

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon CloudWatch Application Insights Amazon CloudWatch Application Insights is a service that helps you detect common problems with your applications. It enables you to pinpoint the source of issues in your applications (built with technologies such as Microsoft IIS, .NET, and Microsoft SQL Server), by providing key insights into detected problems. After you onboard your application, CloudWatch Application Insights identifies, recommends, and sets up metrics and logs. It continuously analyzes and correlates your metrics and logs for unusual behavior to surface actionable problems with your application. For example, if your application is slow and unresponsive and leading to HTTP 500 errors in your Application Load Balancer (ALB), Application Insights informs you that a memory pressure problem with your SQL Server database is occurring.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-applicationinsights/tests/scenario_test.rs`: set up application monitoring with an application, components, log patterns, workload, describing/listing, and teardown.
- From the AWS documentation and model: model application-centric observability setup, component detection, workload configuration, log pattern management, and problem or insight surfaces derived from monitored resources.

## Service Identity and Protocol

- AWS model slug: `application-insights`
- AWS SDK for Rust slug: `applicationinsights`
- Model version: `2018-11-25`
- Model file: `vendor/api-models-aws/models/application-insights/service/2018-11-25/application-insights-2018-11-25.json`
- SDK ID: `Application Insights`
- Endpoint prefix: `applicationinsights`
- ARN namespace: `applicationinsights`
- CloudFormation name: `ApplicationInsights`
- CloudTrail event source: `applicationinsights.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (9), `List` (8), `Update` (6), `Create` (3), `Delete` (3), `Add` (1), `Remove` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddWorkload`, `CreateApplication`, `CreateComponent`, `CreateLogPattern`, `DeleteApplication`, `DeleteComponent`, `DeleteLogPattern`, `RemoveWorkload`, `TagResource`, `UntagResource`, `UpdateApplication`, `UpdateComponent`, `UpdateComponentConfiguration`, `UpdateLogPattern`, `UpdateProblem`, `UpdateWorkload`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeApplication`, `DescribeComponent`, `DescribeComponentConfiguration`, `DescribeComponentConfigurationRecommendation`, `DescribeLogPattern`, `DescribeObservation`, `DescribeProblem`, `DescribeProblemObservations`, `DescribeWorkload`, `ListApplications`, `ListComponents`, `ListConfigurationHistory`, `ListLogPatternSets`, `ListLogPatterns`, `ListProblems`, `ListTagsForResource`, `ListWorkloads`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 33 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `CloudWatch`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/appinsights-what-is.html
- https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch-application-insights.html

Research outcomes:
- CloudWatch Application Insights monitors applications and detects common problems by correlating metrics, logs, alarms, and events.
- Applications contain monitored components such as EC2 instances, databases, and technology stacks.
- Application Insights can create automated dashboards and troubleshooting views for detected problems.
- Problems group related observations and anomalies to reduce mean time to resolution.
- Supported components and technology stacks drive what telemetry and recommendations are available.
- Application configuration is separate from the underlying CloudWatch metrics, logs, and alarms it observes.

Parity implications:
- Model applications, components, log patterns, monitoring configuration, detected problems, observations, and generated alarms separately.
- Problem generation should be derived from telemetry/anomaly state rather than directly from application creation.
- Component detection and monitoring should be technology-stack aware.

## Operation Groups

### Describe

- Operations: `DescribeApplication`, `DescribeComponent`, `DescribeComponentConfiguration`, `DescribeComponentConfigurationRecommendation`, `DescribeLogPattern`, `DescribeObservation`, `DescribeProblem`, `DescribeProblemObservations`, `DescribeWorkload`
- Common required input members in this group: `ResourceGroupName`, `ComponentName`, `ProblemId`

### List

- Operations: `ListApplications`, `ListComponents`, `ListConfigurationHistory`, `ListLogPatterns`, `ListLogPatternSets`, `ListProblems`, `ListTagsForResource`, `ListWorkloads`
- Traits: `paginated` (7)
- Common required input members in this group: `ResourceGroupName`

### Update

- Operations: `UpdateApplication`, `UpdateComponent`, `UpdateComponentConfiguration`, `UpdateLogPattern`, `UpdateProblem`, `UpdateWorkload`
- Common required input members in this group: `ResourceGroupName`, `ComponentName`

### Create

- Operations: `CreateApplication`, `CreateComponent`, `CreateLogPattern`
- Common required input members in this group: `ResourceGroupName`

### Delete

- Operations: `DeleteApplication`, `DeleteComponent`, `DeleteLogPattern`
- Common required input members in this group: `ResourceGroupName`

### Add

- Operations: `AddWorkload`
- Common required input members in this group: -

### Remove

- Operations: `RemoveWorkload`
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
| `AddWorkload` | `-` | - | `ResourceGroupName`, `ComponentName`, `WorkloadConfiguration` | - | `AddWorkloadResponse` | `InternalServerException`, `ResourceInUseException`, `ResourceNotFoundException`, `ValidationException` | Adds a workload to a component. Each component can have at most five workloads. |
| `CreateApplication` | `-` | - | - | - | `CreateApplicationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceInUseException`, `ResourceNotFoundException`, `TagsAlreadyExistException`, `ValidationException` | Adds an application that is created from a resource group. |
| `CreateComponent` | `-` | - | `ResourceGroupName`, `ComponentName`, `ResourceList` | - | `CreateComponentResponse` | `InternalServerException`, `ResourceInUseException`, `ResourceNotFoundException`, `ValidationException` | Creates a custom component by grouping similar standalone instances to monitor. |
| `CreateLogPattern` | `-` | - | `ResourceGroupName`, `PatternSetName`, `PatternName`, `Pattern`, `Rank` | - | `CreateLogPatternResponse` | `InternalServerException`, `ResourceInUseException`, `ResourceNotFoundException`, `ValidationException` | Adds an log pattern to a LogPatternSet . |
| `DeleteApplication` | `-` | - | `ResourceGroupName` | - | `DeleteApplicationResponse` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes the specified application from monitoring. Does not delete the application. |
| `DeleteComponent` | `-` | - | `ResourceGroupName`, `ComponentName` | - | `DeleteComponentResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Ungroups a custom component. When you ungroup custom components, all applicable monitors that are set up for the component are removed and the instances revert to their standalone status. |
| `DeleteLogPattern` | `-` | - | `ResourceGroupName`, `PatternSetName`, `PatternName` | - | `DeleteLogPatternResponse` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes the specified log pattern from a LogPatternSet . |
| `DescribeApplication` | `-` | - | `ResourceGroupName` | - | `DescribeApplicationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes the application. |
| `DescribeComponent` | `-` | - | `ResourceGroupName`, `ComponentName` | - | `DescribeComponentResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes a component and lists the resources that are grouped together in a component. |
| `DescribeComponentConfiguration` | `-` | - | `ResourceGroupName`, `ComponentName` | - | `DescribeComponentConfigurationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes the monitoring configuration of the component. |
| `DescribeComponentConfigurationRecommendation` | `-` | - | `ResourceGroupName`, `ComponentName`, `Tier` | - | `DescribeComponentConfigurationRecommendationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes the recommended monitoring configuration of the component. |
| `DescribeLogPattern` | `-` | - | `ResourceGroupName`, `PatternSetName`, `PatternName` | - | `DescribeLogPatternResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describe a specific log pattern from a LogPatternSet . |
| `DescribeObservation` | `-` | - | `ObservationId` | - | `DescribeObservationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes an anomaly or error with the application. |
| `DescribeProblem` | `-` | - | `ProblemId` | - | `DescribeProblemResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes an application problem. |
| `DescribeProblemObservations` | `-` | - | `ProblemId` | - | `DescribeProblemObservationsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes the anomalies or errors associated with the problem. |
| `DescribeWorkload` | `-` | - | `ResourceGroupName`, `ComponentName`, `WorkloadId` | - | `DescribeWorkloadResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes a workload and its configuration. |
| `ListApplications` | `-` | `paginated` | - | - | `ListApplicationsResponse` | `InternalServerException`, `ValidationException` | Lists the IDs of the applications that you are monitoring. |
| `ListComponents` | `-` | `paginated` | `ResourceGroupName` | - | `ListComponentsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the auto-grouped, standalone, and custom components of the application. |
| `ListConfigurationHistory` | `-` | `paginated` | - | - | `ListConfigurationHistoryResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the INFO, WARN, and ERROR events for periodic configuration updates performed by Application Insights. Examples of events represented are: INFO: creating a new alarm or updating an alarm threshold. WARN: alarm ... |
| `ListLogPatterns` | `-` | `paginated` | `ResourceGroupName` | - | `ListLogPatternsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the log patterns in the specific log LogPatternSet . |
| `ListLogPatternSets` | `-` | `paginated` | `ResourceGroupName` | - | `ListLogPatternSetsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the log pattern sets in the specific application. |
| `ListProblems` | `-` | `paginated` | - | - | `ListProblemsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the problems with your application. |
| `ListTagsForResource` | `-` | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException`, `ValidationException` | Retrieve a list of the tags (keys and values) that are associated with a specified application. A tag is a label that you optionally define and associate with an application. Each tag consists of a required tag key a ... |
| `ListWorkloads` | `-` | `paginated` | `ResourceGroupName`, `ComponentName` | - | `ListWorkloadsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the workloads that are configured on a given component. |
| `RemoveWorkload` | `-` | - | `ResourceGroupName`, `ComponentName`, `WorkloadId` | - | `RemoveWorkloadResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Remove workload from a component. |
| `TagResource` | `-` | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `TooManyTagsException`, `ValidationException` | Add one or more tags (keys and values) to a specified application. A tag is a label that you optionally define and associate with an application. Tags can help you categorize and manage application in different ways, ... |
| `UntagResource` | `-` | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException`, `ValidationException` | Remove one or more tags (keys and values) from a specified application. |
| `UpdateApplication` | `-` | - | `ResourceGroupName` | - | `UpdateApplicationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates the application. |
| `UpdateComponent` | `-` | - | `ResourceGroupName`, `ComponentName` | - | `UpdateComponentResponse` | `InternalServerException`, `ResourceInUseException`, `ResourceNotFoundException`, `ValidationException` | Updates the custom component name and/or the list of resources that make up the component. |
| `UpdateComponentConfiguration` | `-` | - | `ResourceGroupName`, `ComponentName` | - | `UpdateComponentConfigurationResponse` | `InternalServerException`, `ResourceInUseException`, `ResourceNotFoundException`, `ValidationException` | Updates the monitoring configurations for the component. The configuration input parameter is an escaped JSON of the configuration and should match the schema of what is returned by DescribeComponentConfigurationReco ... |
| `UpdateLogPattern` | `-` | - | `ResourceGroupName`, `PatternSetName`, `PatternName` | - | `UpdateLogPatternResponse` | `InternalServerException`, `ResourceInUseException`, `ResourceNotFoundException`, `ValidationException` | Adds a log pattern to a LogPatternSet . |
| `UpdateProblem` | `-` | - | `ProblemId` | - | `UpdateProblemResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates the visibility of the problem or specifies the problem as RESOLVED . |
| `UpdateWorkload` | `-` | - | `ResourceGroupName`, `ComponentName`, `WorkloadConfiguration` | - | `UpdateWorkloadResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds a workload to a component. Each component can have at most five workloads. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | User does not have permissions to perform this action. |
| `BadRequestException` | `structure` | Message | The request is not understood by the server. |
| `InternalServerException` | `structure` | Message | The server encountered an internal error and is unable to complete the request. |
| `ResourceInUseException` | `structure` | Message | The resource is already created or in use. |
| `ResourceNotFoundException` | `structure` | Message | The resource does not exist in the customer account. |
| `TagsAlreadyExistException` | `structure` | Message | Tags are already registered for the specified application ARN. |
| `TooManyTagsException` | `structure` | Message, ResourceName | The number of the provided tags is beyond the limit, or the number of total tags you are trying to attach to the specified resource exceeds the limit. |
| `ValidationException` | `structure` | Message | The parameter is not valid. |
| `AddWorkloadRequest` | `structure` | ResourceGroupName, ComponentName, WorkloadConfiguration | - |
| `AddWorkloadResponse` | `structure` | WorkloadId, WorkloadConfiguration | - |
| `CreateApplicationRequest` | `structure` | ResourceGroupName, OpsCenterEnabled, CWEMonitorEnabled, OpsItemSNSTopicArn, SNSNotificationArn, Tags, AutoConfigEnabled, AutoCreate, GroupingType, AttachMissingPermission | - |
| `CreateApplicationResponse` | `structure` | ApplicationInfo | - |
| `CreateComponentRequest` | `structure` | ResourceGroupName, ComponentName, ResourceList | - |
| `CreateComponentResponse` | `structure` | **empty (no members)** | - |
| `CreateLogPatternRequest` | `structure` | ResourceGroupName, PatternSetName, PatternName, Pattern, Rank | - |
| `CreateLogPatternResponse` | `structure` | LogPattern, ResourceGroupName | - |
| `DeleteApplicationRequest` | `structure` | ResourceGroupName | - |
| `DeleteApplicationResponse` | `structure` | **empty (no members)** | - |
| `DeleteComponentRequest` | `structure` | ResourceGroupName, ComponentName | - |
| `DeleteComponentResponse` | `structure` | **empty (no members)** | - |
| `DeleteLogPatternRequest` | `structure` | ResourceGroupName, PatternSetName, PatternName | - |
| `DeleteLogPatternResponse` | `structure` | **empty (no members)** | - |
| `DescribeApplicationRequest` | `structure` | ResourceGroupName, AccountId | - |
| `DescribeApplicationResponse` | `structure` | ApplicationInfo | - |
| `DescribeComponentRequest` | `structure` | ResourceGroupName, ComponentName, AccountId | - |
| `DescribeComponentResponse` | `structure` | ApplicationComponent, ResourceList | - |
| `DescribeComponentConfigurationRequest` | `structure` | ResourceGroupName, ComponentName, AccountId | - |
| `DescribeComponentConfigurationResponse` | `structure` | Monitor, Tier, ComponentConfiguration | - |
| `DescribeComponentConfigurationRecommendationRequest` | `structure` | ResourceGroupName, ComponentName, Tier, WorkloadName, RecommendationType | - |
| `DescribeComponentConfigurationRecommendationResponse` | `structure` | ComponentConfiguration | - |
| `DescribeLogPatternRequest` | `structure` | ResourceGroupName, PatternSetName, PatternName, AccountId | - |
| `DescribeLogPatternResponse` | `structure` | ResourceGroupName, AccountId, LogPattern | - |
| `DescribeObservationRequest` | `structure` | ObservationId, AccountId | - |
| `DescribeObservationResponse` | `structure` | Observation | - |
| `DescribeProblemRequest` | `structure` | ProblemId, AccountId | - |
| `DescribeProblemResponse` | `structure` | Problem, SNSNotificationArn | - |
| `DescribeProblemObservationsRequest` | `structure` | ProblemId, AccountId | - |
| `DescribeProblemObservationsResponse` | `structure` | RelatedObservations | - |
| `DescribeWorkloadRequest` | `structure` | ResourceGroupName, ComponentName, WorkloadId, AccountId | - |
| `DescribeWorkloadResponse` | `structure` | WorkloadId, WorkloadRemarks, WorkloadConfiguration | - |
| `CloudWatchEventSource` | `enum` | EC2, CODE_DEPLOY, HEALTH, RDS | - |
| `ConfigurationEventResourceType` | `enum` | CLOUDWATCH_ALARM, CLOUDWATCH_LOG, CLOUDFORMATION, SSM_ASSOCIATION | - |
| `ConfigurationEventStatus` | `enum` | INFO, WARN, ERROR | - |
| `DiscoveryType` | `enum` | RESOURCE_GROUP_BASED, ACCOUNT_BASED | - |
| `FeedbackKey` | `enum` | INSIGHTS_FEEDBACK | - |
| `FeedbackValue` | `enum` | NOT_SPECIFIED, USEFUL, NOT_USEFUL | - |
| `GroupingType` | `enum` | ACCOUNT_BASED | - |
| `LogFilter` | `enum` | ERROR, WARN, INFO | - |
| `OsType` | `enum` | WINDOWS, LINUX | - |
| `RecommendationType` | `enum` | INFRA_ONLY, WORKLOAD_ONLY, ALL | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
