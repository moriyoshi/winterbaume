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
- Common required input members in this group: `ComponentName`, `ObservationId`, `PatternName`, `PatternSetName`, `ProblemId`, `ResourceGroupName`, `Tier`, `WorkloadId`

### List

- Operations: `ListApplications`, `ListComponents`, `ListConfigurationHistory`, `ListLogPatternSets`, `ListLogPatterns`, `ListProblems`, `ListTagsForResource`, `ListWorkloads`
- Traits: `paginated` (7)
- Common required input members in this group: `ComponentName`, `ResourceARN`, `ResourceGroupName`

### Update

- Operations: `UpdateApplication`, `UpdateComponent`, `UpdateComponentConfiguration`, `UpdateLogPattern`, `UpdateProblem`, `UpdateWorkload`
- Common required input members in this group: `ComponentName`, `PatternName`, `PatternSetName`, `ProblemId`, `ResourceGroupName`, `WorkloadConfiguration`

### Create

- Operations: `CreateApplication`, `CreateComponent`, `CreateLogPattern`
- Common required input members in this group: `ComponentName`, `Pattern`, `PatternName`, `PatternSetName`, `Rank`, `ResourceGroupName`, `ResourceList`

### Delete

- Operations: `DeleteApplication`, `DeleteComponent`, `DeleteLogPattern`
- Common required input members in this group: `ComponentName`, `PatternName`, `PatternSetName`, `ResourceGroupName`

### Add

- Operations: `AddWorkload`
- Common required input members in this group: `ComponentName`, `ResourceGroupName`, `WorkloadConfiguration`

### Remove

- Operations: `RemoveWorkload`
- Common required input members in this group: `ComponentName`, `ResourceGroupName`, `WorkloadId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddWorkload` | - | - | `ComponentName`, `ResourceGroupName`, `WorkloadConfiguration` | - | `AddWorkloadResponse` | `InternalServerException`, `ResourceInUseException`, `ResourceNotFoundException`, `ValidationException` | Adds a workload to a component. Each component can have at most five workloads. |
| `CreateApplication` | - | - | - | - | `CreateApplicationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceInUseException`, `ResourceNotFoundException`, `TagsAlreadyExistException`, `ValidationException` | Adds an application that is created from a resource group. |
| `CreateComponent` | - | - | `ComponentName`, `ResourceGroupName`, `ResourceList` | - | `CreateComponentResponse` | `InternalServerException`, `ResourceInUseException`, `ResourceNotFoundException`, `ValidationException` | Creates a custom component by grouping similar standalone instances to monitor. |
| `CreateLogPattern` | - | - | `Pattern`, `PatternName`, `PatternSetName`, `Rank`, `ResourceGroupName` | - | `CreateLogPatternResponse` | `InternalServerException`, `ResourceInUseException`, `ResourceNotFoundException`, `ValidationException` | Adds an log pattern to a `LogPatternSet`. |
| `DeleteApplication` | - | - | `ResourceGroupName` | - | `DeleteApplicationResponse` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes the specified application from monitoring. Does not delete the application. |
| `DeleteComponent` | - | - | `ComponentName`, `ResourceGroupName` | - | `DeleteComponentResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Ungroups a custom component. When you ungroup custom components, all applicable monitors that are set up for the component are removed and the instances revert to their standalone status. |
| `DeleteLogPattern` | - | - | `PatternName`, `PatternSetName`, `ResourceGroupName` | - | `DeleteLogPatternResponse` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes the specified log pattern from a `LogPatternSet`. |
| `DescribeApplication` | - | - | `ResourceGroupName` | - | `DescribeApplicationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes the application. |
| `DescribeComponent` | - | - | `ComponentName`, `ResourceGroupName` | - | `DescribeComponentResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes a component and lists the resources that are grouped together in a component. |
| `DescribeComponentConfiguration` | - | - | `ComponentName`, `ResourceGroupName` | - | `DescribeComponentConfigurationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes the monitoring configuration of the component. |
| `DescribeComponentConfigurationRecommendation` | - | - | `ComponentName`, `ResourceGroupName`, `Tier` | - | `DescribeComponentConfigurationRecommendationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes the recommended monitoring configuration of the component. |
| `DescribeLogPattern` | - | - | `PatternName`, `PatternSetName`, `ResourceGroupName` | - | `DescribeLogPatternResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describe a specific log pattern from a `LogPatternSet`. |
| `DescribeObservation` | - | - | `ObservationId` | - | `DescribeObservationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes an anomaly or error with the application. |
| `DescribeProblem` | - | - | `ProblemId` | - | `DescribeProblemResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes an application problem. |
| `DescribeProblemObservations` | - | - | `ProblemId` | - | `DescribeProblemObservationsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes the anomalies or errors associated with the problem. |
| `DescribeWorkload` | - | - | `ComponentName`, `ResourceGroupName`, `WorkloadId` | - | `DescribeWorkloadResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes a workload and its configuration. |
| `ListApplications` | - | `paginated` | - | - | `ListApplicationsResponse` | `InternalServerException`, `ValidationException` | Lists the IDs of the applications that you are monitoring. |
| `ListComponents` | - | `paginated` | `ResourceGroupName` | - | `ListComponentsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the auto-grouped, standalone, and custom components of the application. |
| `ListConfigurationHistory` | - | `paginated` | - | - | `ListConfigurationHistoryResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the INFO, WARN, and ERROR events for periodic configuration updates performed by Application Insights. Examples of events represented are: INFO: creating a new alarm or updating an alarm threshold. |
| `ListLogPatternSets` | - | `paginated` | `ResourceGroupName` | - | `ListLogPatternSetsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the log pattern sets in the specific application. |
| `ListLogPatterns` | - | `paginated` | `ResourceGroupName` | - | `ListLogPatternsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the log patterns in the specific log `LogPatternSet`. |
| `ListProblems` | - | `paginated` | - | - | `ListProblemsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the problems with your application. |
| `ListTagsForResource` | - | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException`, `ValidationException` | Retrieve a list of the tags (keys and values) that are associated with a specified application. A tag is a label that you optionally define and associate with an application. |
| `ListWorkloads` | - | `paginated` | `ComponentName`, `ResourceGroupName` | - | `ListWorkloadsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the workloads that are configured on a given component. |
| `RemoveWorkload` | - | - | `ComponentName`, `ResourceGroupName`, `WorkloadId` | - | `RemoveWorkloadResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Remove workload from a component. |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `TooManyTagsException`, `ValidationException` | Add one or more tags (keys and values) to a specified application. A tag is a label that you optionally define and associate with an application. |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException`, `ValidationException` | Remove one or more tags (keys and values) from a specified application. |
| `UpdateApplication` | - | - | `ResourceGroupName` | - | `UpdateApplicationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates the application. |
| `UpdateComponent` | - | - | `ComponentName`, `ResourceGroupName` | - | `UpdateComponentResponse` | `InternalServerException`, `ResourceInUseException`, `ResourceNotFoundException`, `ValidationException` | Updates the custom component name and/or the list of resources that make up the component. |
| `UpdateComponentConfiguration` | - | - | `ComponentName`, `ResourceGroupName` | - | `UpdateComponentConfigurationResponse` | `InternalServerException`, `ResourceInUseException`, `ResourceNotFoundException`, `ValidationException` | Updates the monitoring configurations for the component. The configuration input parameter is an escaped JSON of the configuration and should match the schema of what is returned by `DescribeComponentConfigurationRecommendation`. |
| `UpdateLogPattern` | - | - | `PatternName`, `PatternSetName`, `ResourceGroupName` | - | `UpdateLogPatternResponse` | `InternalServerException`, `ResourceInUseException`, `ResourceNotFoundException`, `ValidationException` | Adds a log pattern to a `LogPatternSet`. |
| `UpdateProblem` | - | - | `ProblemId` | - | `UpdateProblemResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates the visibility of the problem or specifies the problem as `RESOLVED`. |
| `UpdateWorkload` | - | - | `ComponentName`, `ResourceGroupName`, `WorkloadConfiguration` | - | `UpdateWorkloadResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Adds a workload to a component. Each component can have at most five workloads. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ValidationException` | `structure` | `Message` | The parameter is not valid. |
| `ResourceNotFoundException` | `structure` | `Message` | The resource does not exist in the customer account. |
| `InternalServerException` | `structure` | `Message` | The server encountered an internal error and is unable to complete the request. |
| `ResourceInUseException` | `structure` | `Message` | The resource is already created or in use. |
| `BadRequestException` | `structure` | `Message` | The request is not understood by the server. |
| `AddWorkloadRequest` | `structure` | `ComponentName`, `ResourceGroupName`, `WorkloadConfiguration` | - |
| `AddWorkloadResponse` | `structure` | `WorkloadConfiguration`, `WorkloadId` | - |
| `CreateApplicationRequest` | `structure` | `AttachMissingPermission`, `AutoConfigEnabled`, `AutoCreate`, `CWEMonitorEnabled`, `GroupingType`, `OpsCenterEnabled`, `OpsItemSNSTopicArn`, `ResourceGroupName`, `SNSNotificationArn`, `Tags` | - |
| `CreateApplicationResponse` | `structure` | `ApplicationInfo` | - |
| `AccessDeniedException` | `structure` | `Message` | User does not have permissions to perform this action. |
| `TagsAlreadyExistException` | `structure` | `Message` | Tags are already registered for the specified application ARN. |
| `CreateComponentRequest` | `structure` | `ComponentName`, `ResourceGroupName`, `ResourceList` | - |
| `CreateComponentResponse` | `structure` | - | - |
| `CreateLogPatternRequest` | `structure` | `Pattern`, `PatternName`, `PatternSetName`, `Rank`, `ResourceGroupName` | - |
| `CreateLogPatternResponse` | `structure` | `LogPattern`, `ResourceGroupName` | - |
| `DeleteApplicationRequest` | `structure` | `ResourceGroupName` | - |
| `DeleteApplicationResponse` | `structure` | - | - |
| `DeleteComponentRequest` | `structure` | `ComponentName`, `ResourceGroupName` | - |
| `DeleteComponentResponse` | `structure` | - | - |
| `DeleteLogPatternRequest` | `structure` | `PatternName`, `PatternSetName`, `ResourceGroupName` | - |
| `DeleteLogPatternResponse` | `structure` | - | - |
| `DescribeApplicationRequest` | `structure` | `AccountId`, `ResourceGroupName` | - |
| `DescribeApplicationResponse` | `structure` | `ApplicationInfo` | - |
| `DescribeComponentRequest` | `structure` | `AccountId`, `ComponentName`, `ResourceGroupName` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
