# AWS Auto Scaling Plans

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Auto Scaling Use AWS Auto Scaling to create scaling plans for your applications to automatically scale your scalable AWS resources. API Summary You can use the AWS Auto Scaling service API to accomplish the following tasks: Create and manage scaling plans Define target tracking scaling policies to dynamically scale your resources based on utilization Scale Amazon EC2 Auto Scaling groups using predictive scaling and dynamic scaling to scale your Amazon EC2 capacity faster Set minimum and maximum capacity limits Retrieve information on existing scaling plans Access current forecast data and historical forecast data for up to 56 days previous To learn more about AWS Auto Scaling, including information about granting IAM users required permissions for AWS Auto Scaling actions, see the AWS Auto Scaling User Guide.

## Possible Usage Scenarios
- From the AWS documentation and model: define scaling plans that coordinate scaling instructions across multiple AWS resources.
- From the operation surface: model plan creation, resource scaling configuration, forecast/load metrics, and plan deletion for capacity-management workflows.

## Service Identity and Protocol

- AWS model slug: `auto-scaling-plans`
- AWS SDK for Rust slug: `autoscalingplans`
- Model version: `2018-01-06`
- Model file: `vendor/api-models-aws/models/auto-scaling-plans/service/2018-01-06/auto-scaling-plans-2018-01-06.json`
- SDK ID: `Auto Scaling Plans`
- Endpoint prefix: `autoscaling-plans`
- ARN namespace: `autoscaling-plans`
- CloudFormation name: `AutoScalingPlans`
- CloudTrail event source: `autoscalingplans.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (2), `Create` (1), `Delete` (1), `Get` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateScalingPlan`, `DeleteScalingPlan`, `UpdateScalingPlan`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeScalingPlanResources`, `DescribeScalingPlans`, `GetScalingPlanResourceForecastData`.
- 6 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Describe

- Operations: `DescribeScalingPlanResources`, `DescribeScalingPlans`
- Common required input members in this group: -

### Create

- Operations: `CreateScalingPlan`
- Common required input members in this group: -

### Delete

- Operations: `DeleteScalingPlan`
- Common required input members in this group: -

### Get

- Operations: `GetScalingPlanResourceForecastData`
- Common required input members in this group: -

### Update

- Operations: `UpdateScalingPlan`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateScalingPlan` | `-` | - | `ScalingPlanName`, `ApplicationSource`, `ScalingInstructions` | - | `CreateScalingPlanResponse` | `ConcurrentUpdateException`, `InternalServiceException`, `LimitExceededException`, `ValidationException` | Creates a scaling plan. |
| `DeleteScalingPlan` | `-` | - | `ScalingPlanName`, `ScalingPlanVersion` | - | `DeleteScalingPlanResponse` | `ConcurrentUpdateException`, `InternalServiceException`, `ObjectNotFoundException`, `ValidationException` | Deletes the specified scaling plan. Deleting a scaling plan deletes the underlying ScalingInstruction for all of the scalable resources that are covered by the plan. If the plan has launched resources or has scaling ... |
| `DescribeScalingPlanResources` | `-` | - | `ScalingPlanName`, `ScalingPlanVersion` | - | `DescribeScalingPlanResourcesResponse` | `ConcurrentUpdateException`, `InternalServiceException`, `InvalidNextTokenException`, `ValidationException` | Describes the scalable resources in the specified scaling plan. |
| `DescribeScalingPlans` | `-` | - | - | - | `DescribeScalingPlansResponse` | `ConcurrentUpdateException`, `InternalServiceException`, `InvalidNextTokenException`, `ValidationException` | Describes one or more of your scaling plans. |
| `GetScalingPlanResourceForecastData` | `-` | - | `ScalingPlanName`, `ScalingPlanVersion`, `ServiceNamespace`, `ResourceId`, `ScalableDimension`, `ForecastDataType`, `StartTime`, `EndTime` | - | `GetScalingPlanResourceForecastDataResponse` | `InternalServiceException`, `ValidationException` | Retrieves the forecast data for a scalable resource. Capacity forecasts are represented as predicted values, or data points, that are calculated using historical data points from a specified CloudWatch load metric. D ... |
| `UpdateScalingPlan` | `-` | - | `ScalingPlanName`, `ScalingPlanVersion` | - | `UpdateScalingPlanResponse` | `ConcurrentUpdateException`, `InternalServiceException`, `ObjectNotFoundException`, `ValidationException` | Updates the specified scaling plan. You cannot update a scaling plan if it is in the process of being created, updated, or deleted. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ConcurrentUpdateException` | `structure` | Message | Concurrent updates caused an exception, for example, if you request an update to a scaling plan that already has a pending update. |
| `InternalServiceException` | `structure` | Message | The service encountered an internal error. |
| `InvalidNextTokenException` | `structure` | Message | The token provided is not valid. |
| `LimitExceededException` | `structure` | Message | Your account exceeded a limit. This exception is thrown when a per-account resource limit is exceeded. |
| `ObjectNotFoundException` | `structure` | Message | The specified object could not be found. |
| `ValidationException` | `structure` | Message | An exception was thrown for a validation issue. Review the parameters provided. |
| `CreateScalingPlanRequest` | `structure` | ScalingPlanName, ApplicationSource, ScalingInstructions | - |
| `CreateScalingPlanResponse` | `structure` | ScalingPlanVersion | - |
| `DeleteScalingPlanRequest` | `structure` | ScalingPlanName, ScalingPlanVersion | - |
| `DeleteScalingPlanResponse` | `structure` | **empty (no members)** | - |
| `DescribeScalingPlanResourcesRequest` | `structure` | ScalingPlanName, ScalingPlanVersion, MaxResults, NextToken | - |
| `DescribeScalingPlanResourcesResponse` | `structure` | ScalingPlanResources, NextToken | - |
| `DescribeScalingPlansRequest` | `structure` | ScalingPlanNames, ScalingPlanVersion, ApplicationSources, MaxResults, NextToken | - |
| `DescribeScalingPlansResponse` | `structure` | ScalingPlans, NextToken | - |
| `GetScalingPlanResourceForecastDataRequest` | `structure` | ScalingPlanName, ScalingPlanVersion, ServiceNamespace, ResourceId, ScalableDimension, ForecastDataType, StartTime, EndTime | - |
| `GetScalingPlanResourceForecastDataResponse` | `structure` | Datapoints | - |
| `UpdateScalingPlanRequest` | `structure` | ScalingPlanName, ScalingPlanVersion, ApplicationSource, ScalingInstructions | - |
| `UpdateScalingPlanResponse` | `structure` | **empty (no members)** | - |
| `ForecastDataType` | `enum` | CapacityForecast, LoadForecast, ScheduledActionMinCapacity, ScheduledActionMaxCapacity | - |
| `LoadMetricType` | `enum` | ASGTotalCPUUtilization, ASGTotalNetworkIn, ASGTotalNetworkOut, ALBTargetGroupRequestCount | - |
| `MetricStatistic` | `enum` | Average, Minimum, Maximum, SampleCount, Sum | - |
| `PolicyType` | `enum` | TargetTrackingScaling | - |
| `PredictiveScalingMaxCapacityBehavior` | `enum` | SetForecastCapacityToMaxCapacity, SetMaxCapacityToForecastCapacity, SetMaxCapacityAboveForecastCapacity | - |
| `PredictiveScalingMode` | `enum` | ForecastAndScale, ForecastOnly | - |
| `ScalableDimension` | `enum` | AutoScalingGroupDesiredCapacity, ECSServiceDesiredCount, EC2SpotFleetRequestTargetCapacity, RDSClusterReadReplicaCount, DynamoDBTableReadCapacityUnits, DynamoDBTableWriteCapacityUnits, DynamoDBIndexReadCapacityUnits, DynamoDBIndexWriteCapacityUnits | - |
| `ScalingMetricType` | `enum` | ASGAverageCPUUtilization, ASGAverageNetworkIn, ASGAverageNetworkOut, DynamoDBReadCapacityUtilization, DynamoDBWriteCapacityUtilization, ECSServiceAverageCPUUtilization, ECSServiceAverageMemoryUtilization, ALBRequestCountPerTarget, RDSReaderAverageCPUUtilization, RDSReaderAverageDatabaseConnections, EC2SpotFleetRequestAverageCPUUtilization, EC2SpotFleetRequestAverageNetworkIn, ... (+1) | - |
| `ScalingPlanStatusCode` | `enum` | Active, ActiveWithProblems, CreationInProgress, CreationFailed, DeletionInProgress, DeletionFailed, UpdateInProgress, UpdateFailed | - |
| `ScalingPolicyUpdateBehavior` | `enum` | KeepExternalPolicies, ReplaceExternalPolicies | - |
| `ScalingStatusCode` | `enum` | Inactive, PartiallyActive, Active | - |
| `ServiceNamespace` | `enum` | AUTOSCALING, ECS, EC2, RDS, DYNAMODB | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
