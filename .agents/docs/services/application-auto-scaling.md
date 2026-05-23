# Application Auto Scaling

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

With Application Auto Scaling, you can configure automatic scaling for the following resources: Amazon AppStream 2.0 fleets Amazon Aurora Replicas Amazon Comprehend document classification and entity recognizer endpoints Amazon DynamoDB tables and global secondary indexes throughput capacity Amazon ECS services Amazon ElastiCache replication groups (Redis OSS and Valkey) and Memcached clusters Amazon EMR clusters Amazon Keyspaces (for Apache Cassandra) tables Lambda function provisioned concurrency Amazon Managed Streaming for Apache Kafka broker storage Amazon Neptune clusters Amazon SageMaker endpoint variants Amazon SageMaker inference components Amazon SageMaker serverless endpoint provisioned concurrency Spot Fleets (Amazon EC2) Pool of WorkSpaces Custom resources provided by your own applications or services To learn more about Application Auto Scaling, see the Application Auto Scaling User Guide. API Summary The Application Auto Scaling service API includes three key sets of actions: Register and manage scalable targets - Register Amazon Web Services or custom resources as scalable targets (a resource that Application Auto Scaling can scale), set minimum and maximum capacity limits, and retrieve information on existing scalable targets. Configure and manage automatic scaling - Define scaling policies to dynamically scale your resources in response to CloudWatch alarms...

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Application Auto Scaling where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- From the AWS documentation and model: register scalable targets, attach scaling policies, configure scheduled actions, and describe scaling activities.
- From the operation surface: model autoscaling control loops for DynamoDB, ECS, Lambda provisioned concurrency, SageMaker variants, and other scalable AWS resources.

## Service Identity and Protocol

- AWS model slug: `application-auto-scaling`
- AWS SDK for Rust slug: `applicationautoscaling`
- Model version: `2016-02-06`
- Model file: `vendor/api-models-aws/models/application-auto-scaling/service/2016-02-06/application-auto-scaling-2016-02-06.json`
- SDK ID: `Application Auto Scaling`
- Endpoint prefix: `application-autoscaling`
- ARN namespace: `application-autoscaling`
- CloudFormation name: `ApplicationAutoScaling`
- CloudTrail event source: `applicationautoscaling.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (4), `Delete` (2), `Put` (2), `Deregister` (1), `Get` (1), `List` (1), `Register` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DeleteScalingPolicy`, `DeleteScheduledAction`, `DeregisterScalableTarget`, `PutScalingPolicy`, `PutScheduledAction`, `RegisterScalableTarget`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeScalableTargets`, `DescribeScalingActivities`, `DescribeScalingPolicies`, `DescribeScheduledActions`, `GetPredictiveScalingForecast`, `ListTagsForResource`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 14 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `Lambda`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/autoscaling/application/userguide/target-tracking-scaling-policy-overview.html
- https://docs.aws.amazon.com/autoscaling/application/userguide/application-auto-scaling-scheduled-scaling.html
- https://docs.aws.amazon.com/autoscaling/application/userguide/step-scaling-policy-overview.html

Research outcomes:
- Application Auto Scaling manages scalable targets for many AWS services, not only EC2 instances.
- Target tracking policies use CloudWatch metrics and target values. Application Auto Scaling creates and manages the CloudWatch alarms for those policies.
- Target tracking scales out when the metric is above target, and scales in conservatively when the metric falls sufficiently below target.
- Missing metric data moves alarms to INSUFFICIENT_DATA, and Application Auto Scaling cannot scale the target until new data points are found.
- Scale-out and scale-in cooldowns have different semantics. Scale-out can still allow a larger subsequent scale-out, while scale-in is blocked until cooldown expires unless a scale-out occurs.
- Multiple target tracking policies scale out if any policy is ready to scale out, but scale in only if all scale-in-enabled policies are ready to scale in.
- Scheduled actions can immediately change min and max capacity without waiting for policy cooldowns.
- Scheduled scaling and scaling policies can work together, with current capacity constrained by the scheduled min/max boundaries.

Parity implications:
- Model scalable targets, min/max capacity, suspended state, scaling policies, scheduled actions, alarms, cooldown timers, and scaling activities.
- Policy evaluation should distinguish target tracking, step scaling, scheduled actions, missing data, and multiple-policy arbitration.
- Do not require users to manage target-tracking CloudWatch alarms directly; they are service-managed artefacts.

## Operation Groups

### Describe

- Operations: `DescribeScalableTargets`, `DescribeScalingActivities`, `DescribeScalingPolicies`, `DescribeScheduledActions`
- Traits: `paginated` (4)
- Common required input members in this group: `ServiceNamespace`

### Delete

- Operations: `DeleteScalingPolicy`, `DeleteScheduledAction`
- Common required input members in this group: `ServiceNamespace`, `ResourceId`, `ScalableDimension`

### Put

- Operations: `PutScalingPolicy`, `PutScheduledAction`
- Common required input members in this group: `ServiceNamespace`, `ResourceId`, `ScalableDimension`

### Deregister

- Operations: `DeregisterScalableTarget`
- Common required input members in this group: -

### Get

- Operations: `GetPredictiveScalingForecast`
- Common required input members in this group: -

### List

- Operations: `ListTagsForResource`
- Common required input members in this group: -

### Register

- Operations: `RegisterScalableTarget`
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
| `DeleteScalingPolicy` | `-` | - | `PolicyName`, `ServiceNamespace`, `ResourceId`, `ScalableDimension` | - | `DeleteScalingPolicyResponse` | `ConcurrentUpdateException`, `InternalServiceException`, `ObjectNotFoundException`, `ValidationException` | Deletes the specified scaling policy for an Application Auto Scaling scalable target. Deleting a step scaling policy deletes the underlying alarm action, but does not delete the CloudWatch alarm associated with the s ... |
| `DeleteScheduledAction` | `-` | - | `ServiceNamespace`, `ScheduledActionName`, `ResourceId`, `ScalableDimension` | - | `DeleteScheduledActionResponse` | `ConcurrentUpdateException`, `InternalServiceException`, `ObjectNotFoundException`, `ValidationException` | Deletes the specified scheduled action for an Application Auto Scaling scalable target. For more information, see Delete a scheduled action in the Application Auto Scaling User Guide . |
| `DeregisterScalableTarget` | `-` | - | `ServiceNamespace`, `ResourceId`, `ScalableDimension` | - | `DeregisterScalableTargetResponse` | `ConcurrentUpdateException`, `InternalServiceException`, `ObjectNotFoundException`, `ValidationException` | Deregisters an Application Auto Scaling scalable target when you have finished using it. To see which resources have been registered, use DescribeScalableTargets . Deregistering a scalable target deletes the scaling ... |
| `DescribeScalableTargets` | `-` | `paginated` | `ServiceNamespace` | - | `DescribeScalableTargetsResponse` | `ConcurrentUpdateException`, `InternalServiceException`, `InvalidNextTokenException`, `ValidationException` | Gets information about the scalable targets in the specified namespace. You can filter the results using ResourceIds and ScalableDimension . |
| `DescribeScalingActivities` | `-` | `paginated` | `ServiceNamespace` | - | `DescribeScalingActivitiesResponse` | `ConcurrentUpdateException`, `InternalServiceException`, `InvalidNextTokenException`, `ValidationException` | Provides descriptive information about the scaling activities in the specified namespace from the previous six weeks. You can filter the results using ResourceId and ScalableDimension . For information about viewing ... |
| `DescribeScalingPolicies` | `-` | `paginated` | `ServiceNamespace` | - | `DescribeScalingPoliciesResponse` | `ConcurrentUpdateException`, `FailedResourceAccessException`, `InternalServiceException`, `InvalidNextTokenException`, `ValidationException` | Describes the Application Auto Scaling scaling policies for the specified service namespace. You can filter the results using ResourceId , ScalableDimension , and PolicyNames . For more information, see Target tracki ... |
| `DescribeScheduledActions` | `-` | `paginated` | `ServiceNamespace` | - | `DescribeScheduledActionsResponse` | `ConcurrentUpdateException`, `InternalServiceException`, `InvalidNextTokenException`, `ValidationException` | Describes the Application Auto Scaling scheduled actions for the specified service namespace. You can filter the results using the ResourceId , ScalableDimension , and ScheduledActionNames parameters. For more inform ... |
| `GetPredictiveScalingForecast` | `-` | - | `ServiceNamespace`, `ResourceId`, `ScalableDimension`, `PolicyName`, `StartTime`, `EndTime` | - | `GetPredictiveScalingForecastResponse` | `InternalServiceException`, `ValidationException` | Retrieves the forecast data for a predictive scaling policy. Load forecasts are predictions of the hourly load values using historical load data from CloudWatch and an analysis of historical trends. Capacity forecast ... |
| `ListTagsForResource` | `-` | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | Returns all the tags on the specified Application Auto Scaling scalable target. For general information about tags, including the format and syntax, see Tagging your Amazon Web Services resources in the Amazon Web Se ... |
| `PutScalingPolicy` | `-` | - | `PolicyName`, `ServiceNamespace`, `ResourceId`, `ScalableDimension` | - | `PutScalingPolicyResponse` | `ConcurrentUpdateException`, `FailedResourceAccessException`, `InternalServiceException`, `LimitExceededException`, `ObjectNotFoundException`, `ValidationException` | Creates or updates a scaling policy for an Application Auto Scaling scalable target. Each scalable target is identified by a service namespace, resource ID, and scalable dimension. A scaling policy applies to the sca ... |
| `PutScheduledAction` | `-` | - | `ServiceNamespace`, `ScheduledActionName`, `ResourceId`, `ScalableDimension` | - | `PutScheduledActionResponse` | `ConcurrentUpdateException`, `InternalServiceException`, `LimitExceededException`, `ObjectNotFoundException`, `ValidationException` | Creates or updates a scheduled action for an Application Auto Scaling scalable target. Each scalable target is identified by a service namespace, resource ID, and scalable dimension. A scheduled action applies to the ... |
| `RegisterScalableTarget` | `-` | - | `ServiceNamespace`, `ResourceId`, `ScalableDimension` | - | `RegisterScalableTargetResponse` | `ConcurrentUpdateException`, `InternalServiceException`, `LimitExceededException`, `ValidationException` | Registers or updates a scalable target, which is the resource that you want to scale. Scalable targets are uniquely identified by the combination of resource ID, scalable dimension, and namespace, which represents so ... |
| `TagResource` | `-` | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `TooManyTagsException`, `ValidationException` | Adds or edits tags on an Application Auto Scaling scalable target. Each tag consists of a tag key and a tag value, which are both case-sensitive strings. To add a tag, specify a new tag key and a tag value. To edit a ... |
| `UntagResource` | `-` | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException`, `ValidationException` | Deletes tags from an Application Auto Scaling scalable target. To delete a tag, specify the tag key and the Application Auto Scaling scalable target. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ConcurrentUpdateException` | `structure` | Message | Concurrent updates caused an exception, for example, if you request an update to an Application Auto Scaling resource that already has a pending update. |
| `FailedResourceAccessException` | `structure` | Message | Failed access to resources caused an exception. This exception is thrown when Application Auto Scaling is unable to retrieve the alarms associated with a sc ... |
| `InternalServiceException` | `structure` | Message | The service encountered an internal error. |
| `InvalidNextTokenException` | `structure` | Message | The next token supplied was invalid. |
| `LimitExceededException` | `structure` | Message | A per-account resource limit is exceeded. For more information, see Application Auto Scaling service quotas . |
| `ObjectNotFoundException` | `structure` | Message | The specified object could not be found. For any operation that depends on the existence of a scalable target, this exception is thrown if the scalable targ ... |
| `ResourceNotFoundException` | `structure` | Message, ResourceName | The specified resource doesn't exist. |
| `TooManyTagsException` | `structure` | Message, ResourceName | The request contains too many tags. Try the request again with fewer tags. |
| `ValidationException` | `structure` | Message | An exception was thrown for a validation issue. Review the available parameters for the API request. |
| `DeleteScalingPolicyRequest` | `structure` | PolicyName, ServiceNamespace, ResourceId, ScalableDimension | - |
| `DeleteScalingPolicyResponse` | `structure` | **empty (no members)** | - |
| `DeleteScheduledActionRequest` | `structure` | ServiceNamespace, ScheduledActionName, ResourceId, ScalableDimension | - |
| `DeleteScheduledActionResponse` | `structure` | **empty (no members)** | - |
| `DeregisterScalableTargetRequest` | `structure` | ServiceNamespace, ResourceId, ScalableDimension | - |
| `DeregisterScalableTargetResponse` | `structure` | **empty (no members)** | - |
| `DescribeScalableTargetsRequest` | `structure` | ServiceNamespace, ResourceIds, ScalableDimension, MaxResults, NextToken | - |
| `DescribeScalableTargetsResponse` | `structure` | ScalableTargets, NextToken | - |
| `DescribeScalingActivitiesRequest` | `structure` | ServiceNamespace, ResourceId, ScalableDimension, MaxResults, NextToken, IncludeNotScaledActivities | - |
| `DescribeScalingActivitiesResponse` | `structure` | ScalingActivities, NextToken | - |
| `DescribeScalingPoliciesRequest` | `structure` | PolicyNames, ServiceNamespace, ResourceId, ScalableDimension, MaxResults, NextToken | - |
| `DescribeScalingPoliciesResponse` | `structure` | ScalingPolicies, NextToken | - |
| `DescribeScheduledActionsRequest` | `structure` | ScheduledActionNames, ServiceNamespace, ResourceId, ScalableDimension, MaxResults, NextToken | - |
| `DescribeScheduledActionsResponse` | `structure` | ScheduledActions, NextToken | - |
| `GetPredictiveScalingForecastRequest` | `structure` | ServiceNamespace, ResourceId, ScalableDimension, PolicyName, StartTime, EndTime | - |
| `GetPredictiveScalingForecastResponse` | `structure` | LoadForecast, CapacityForecast, UpdateTime | - |
| `ListTagsForResourceRequest` | `structure` | ResourceARN | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `PutScalingPolicyRequest` | `structure` | PolicyName, ServiceNamespace, ResourceId, ScalableDimension, PolicyType, StepScalingPolicyConfiguration, TargetTrackingScalingPolicyConfiguration, PredictiveScalingPolicyConfiguration | - |
| `PutScalingPolicyResponse` | `structure` | PolicyARN, Alarms | - |
| `PutScheduledActionRequest` | `structure` | ServiceNamespace, Schedule, Timezone, ScheduledActionName, ResourceId, ScalableDimension, StartTime, EndTime, ScalableTargetAction | - |
| `PutScheduledActionResponse` | `structure` | **empty (no members)** | - |
| `RegisterScalableTargetRequest` | `structure` | ServiceNamespace, ResourceId, ScalableDimension, MinCapacity, MaxCapacity, RoleARN, SuspendedState, Tags | - |
| `RegisterScalableTargetResponse` | `structure` | ScalableTargetARN | - |
| `TagResourceRequest` | `structure` | ResourceARN, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceARN, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `AdjustmentType` | `enum` | ChangeInCapacity, PercentChangeInCapacity, ExactCapacity | - |
| `MetricAggregationType` | `enum` | Average, Minimum, Maximum | - |
| `MetricStatistic` | `enum` | Average, Minimum, Maximum, SampleCount, Sum | - |
| `MetricType` | `enum` | DynamoDBReadCapacityUtilization, DynamoDBWriteCapacityUtilization, ALBRequestCountPerTarget, RDSReaderAverageCPUUtilization, RDSReaderAverageDatabaseConnections, EC2SpotFleetRequestAverageCPUUtilization, EC2SpotFleetRequestAverageNetworkIn, EC2SpotFleetRequestAverageNetworkOut, SageMakerVariantInvocationsPerInstance, ECSServiceAverageCPUUtilization, ECSServiceAverageMemoryUtilization, AppStreamAverageCapacityUtilization, ... (+17) | - |
| `PolicyType` | `enum` | StepScaling, TargetTrackingScaling, PredictiveScaling | - |
| `PredictiveScalingMaxCapacityBreachBehavior` | `enum` | HonorMaxCapacity, IncreaseMaxCapacity | - |
| `PredictiveScalingMode` | `enum` | ForecastOnly, ForecastAndScale | - |
| `ScalableDimension` | `enum` | ECSServiceDesiredCount, EC2SpotFleetRequestTargetCapacity, EMRInstanceGroupInstanceCount, AppstreamFleetDesiredCapacity, DynamoDBTableReadCapacityUnits, DynamoDBTableWriteCapacityUnits, DynamoDBIndexReadCapacityUnits, DynamoDBIndexWriteCapacityUnits, RDSClusterReadReplicaCount, SageMakerVariantDesiredInstanceCount, CustomResourceScalableDimension, ComprehendDocClassifierEndpointInferenceUnits, ... (+12) | - |
| `ScalingActivityStatusCode` | `enum` | Pending, InProgress, Successful, Overridden, Unfulfilled, Failed | - |
| `ServiceNamespace` | `enum` | ECS, EMR, EC2, APPSTREAM, DYNAMODB, RDS, SAGEMAKER, CUSTOM_RESOURCE, COMPREHEND, LAMBDA, CASSANDRA, KAFKA, ... (+3) | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
