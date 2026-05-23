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
- Common required input members in this group: `ScalingPlanName`, `ScalingPlanVersion`

### Create

- Operations: `CreateScalingPlan`
- Common required input members in this group: `ApplicationSource`, `ScalingInstructions`, `ScalingPlanName`

### Delete

- Operations: `DeleteScalingPlan`
- Common required input members in this group: `ScalingPlanName`, `ScalingPlanVersion`

### Get

- Operations: `GetScalingPlanResourceForecastData`
- Common required input members in this group: `EndTime`, `ForecastDataType`, `ResourceId`, `ScalableDimension`, `ScalingPlanName`, `ScalingPlanVersion`, `ServiceNamespace`, `StartTime`

### Update

- Operations: `UpdateScalingPlan`
- Common required input members in this group: `ScalingPlanName`, `ScalingPlanVersion`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateScalingPlan` | - | - | `ApplicationSource`, `ScalingInstructions`, `ScalingPlanName` | - | `CreateScalingPlanResponse` | `ConcurrentUpdateException`, `InternalServiceException`, `LimitExceededException`, `ValidationException` | Creates a scaling plan. |
| `DeleteScalingPlan` | - | - | `ScalingPlanName`, `ScalingPlanVersion` | - | `DeleteScalingPlanResponse` | `ConcurrentUpdateException`, `InternalServiceException`, `ObjectNotFoundException`, `ValidationException` | Deletes the specified scaling plan. Deleting a scaling plan deletes the underlying ScalingInstruction for all of the scalable resources that are covered by the plan. |
| `DescribeScalingPlanResources` | - | - | `ScalingPlanName`, `ScalingPlanVersion` | - | `DescribeScalingPlanResourcesResponse` | `ConcurrentUpdateException`, `InternalServiceException`, `InvalidNextTokenException`, `ValidationException` | Describes the scalable resources in the specified scaling plan. |
| `DescribeScalingPlans` | - | - | - | - | `DescribeScalingPlansResponse` | `ConcurrentUpdateException`, `InternalServiceException`, `InvalidNextTokenException`, `ValidationException` | Describes one or more of your scaling plans. |
| `GetScalingPlanResourceForecastData` | - | - | `EndTime`, `ForecastDataType`, `ResourceId`, `ScalableDimension`, `ScalingPlanName`, `ScalingPlanVersion`, `ServiceNamespace`, `StartTime` | - | `GetScalingPlanResourceForecastDataResponse` | `InternalServiceException`, `ValidationException` | Retrieves the forecast data for a scalable resource. Capacity forecasts are represented as predicted values, or data points, that are calculated using historical data points from a specified CloudWatch load metric. |
| `UpdateScalingPlan` | - | - | `ScalingPlanName`, `ScalingPlanVersion` | - | `UpdateScalingPlanResponse` | `ConcurrentUpdateException`, `InternalServiceException`, `ObjectNotFoundException`, `ValidationException` | Updates the specified scaling plan. You cannot update a scaling plan if it is in the process of being created, updated, or deleted. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServiceException` | `structure` | `Message` | The service encountered an internal error. |
| `ValidationException` | `structure` | `Message` | An exception was thrown for a validation issue. |
| `ConcurrentUpdateException` | `structure` | `Message` | Concurrent updates caused an exception, for example, if you request an update to a scaling plan that already has a pending update. |
| `ObjectNotFoundException` | `structure` | `Message` | The specified object could not be found. |
| `InvalidNextTokenException` | `structure` | `Message` | The token provided is not valid. |
| `CreateScalingPlanRequest` | `structure` | `ApplicationSource`, `ScalingInstructions`, `ScalingPlanName` | - |
| `CreateScalingPlanResponse` | `structure` | `ScalingPlanVersion` | - |
| `LimitExceededException` | `structure` | `Message` | Your account exceeded a limit. |
| `DeleteScalingPlanRequest` | `structure` | `ScalingPlanName`, `ScalingPlanVersion` | - |
| `DeleteScalingPlanResponse` | `structure` | - | - |
| `DescribeScalingPlanResourcesRequest` | `structure` | `MaxResults`, `NextToken`, `ScalingPlanName`, `ScalingPlanVersion` | - |
| `DescribeScalingPlanResourcesResponse` | `structure` | `NextToken`, `ScalingPlanResources` | - |
| `DescribeScalingPlansRequest` | `structure` | `ApplicationSources`, `MaxResults`, `NextToken`, `ScalingPlanNames`, `ScalingPlanVersion` | - |
| `DescribeScalingPlansResponse` | `structure` | `NextToken`, `ScalingPlans` | - |
| `GetScalingPlanResourceForecastDataRequest` | `structure` | `EndTime`, `ForecastDataType`, `ResourceId`, `ScalableDimension`, `ScalingPlanName`, `ScalingPlanVersion`, `ServiceNamespace`, `StartTime` | - |
| `GetScalingPlanResourceForecastDataResponse` | `structure` | `Datapoints` | - |
| `UpdateScalingPlanRequest` | `structure` | `ApplicationSource`, `ScalingInstructions`, `ScalingPlanName`, `ScalingPlanVersion` | - |
| `UpdateScalingPlanResponse` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
