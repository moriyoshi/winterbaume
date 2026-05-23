# Auto Scaling

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon EC2 Auto Scaling The DescribeAutoScalingGroups API operation might be throttled when retrieving details for an Auto Scaling group that contains many instances. By default, this operation returns details for all instances in the group. To help prevent throttling, you can set the `IncludeInstances` parameter to `false` to exclude instance details from the response. Amazon EC2 Auto Scaling is designed to automatically launch and terminate EC2 instances based on user-defined scaling policies, scheduled actions, and health checks. For more information, see the Amazon EC2 Auto Scaling User Guide and the Amazon EC2 Auto Scaling API Reference.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Auto Scaling where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Auto Scaling by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: manage Auto Scaling groups, launch configurations, lifecycle hooks, policies, scheduled actions, warm pools, and instance refreshes.
- From the operation surface: support EC2 fleet elasticity, rolling replacement, health-based instance lifecycle, scaling activity history, and tag-based capacity organisation.

## Service Identity and Protocol

- AWS model slug: `auto-scaling`
- AWS SDK for Rust slug: `autoscaling`
- Model version: `2011-01-01`
- Model file: `vendor/api-models-aws/models/auto-scaling/service/2011-01-01/auto-scaling-2011-01-01.json`
- SDK ID: `Auto Scaling`
- Endpoint prefix: `autoscaling`
- ARN namespace: `autoscaling`
- CloudFormation name: `AutoScaling`
- CloudTrail event source: `autoscaling.amazonaws.com`
- Protocols: `awsQuery`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (21), `Delete` (8), `Put` (5), `Attach` (4), `Detach` (4), `Create` (3), `Set` (3), `Batch` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AttachInstances`, `AttachLoadBalancerTargetGroups`, `AttachLoadBalancers`, `AttachTrafficSources`, `BatchDeleteScheduledAction`, `BatchPutScheduledUpdateGroupAction`, `CancelInstanceRefresh`, `CreateAutoScalingGroup`, `CreateLaunchConfiguration`, `CreateOrUpdateTags`, `DeleteAutoScalingGroup`, `DeleteLaunchConfiguration`, `DeleteLifecycleHook`, `DeleteNotificationConfiguration`, `DeletePolicy`, `DeleteScheduledAction`, `DeleteTags`, `DeleteWarmPool`, `DetachInstances`, `DetachLoadBalancerTargetGroups`, ... (+15).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccountLimits`, `DescribeAdjustmentTypes`, `DescribeAutoScalingGroups`, `DescribeAutoScalingInstances`, `DescribeAutoScalingNotificationTypes`, `DescribeInstanceRefreshes`, `DescribeLaunchConfigurations`, `DescribeLifecycleHookTypes`, `DescribeLifecycleHooks`, `DescribeLoadBalancerTargetGroups`, `DescribeLoadBalancers`, `DescribeMetricCollectionTypes`, `DescribeNotificationConfigurations`, `DescribePolicies`, `DescribeScalingActivities`, `DescribeScalingProcessTypes`, `DescribeScheduledActions`, `DescribeTags`, `DescribeTerminationPolicyTypes`, `DescribeTrafficSources`, ... (+2).
- Pagination is modelled for 13 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelInstanceRefresh`, `StartInstanceRefresh`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 66 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `SNS`, `Lambda`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/autoscaling/ec2/userguide/auto-scaling-groups.html
- https://docs.aws.amazon.com/autoscaling/ec2/userguide/lifecycle-hooks.html
- https://docs.aws.amazon.com/autoscaling/ec2/userguide/ec2-auto-scaling-scaling-cooldowns.html

Research outcomes:
- An Auto Scaling group maintains desired capacity by launching instances and replacing unhealthy instances after health checks.
- Desired capacity is bounded by min and max capacity. Scaling policies adjust desired capacity and the group launches or terminates instances as needed.
- When multiple Availability Zones are configured, EC2 Auto Scaling distributes desired capacity across them and maintains balance during scaling actions.
- Spot instances can be terminated by EC2 capacity conditions; the group attempts to launch replacements to maintain desired capacity.
- Lifecycle hooks can run during instance launch and termination, including replacements, instance refresh, capacity rebalancing, and warm pool transitions.
- Lifecycle hooks do not apply to attach/detach operations, standby transitions, or force-delete group deletion.
- Lifecycle hook wait state defaults to one hour, with a global maximum of 48 hours or 100 times the heartbeat timeout, whichever is smaller.
- Unhealthy instance replacement and scheduled actions do not wait for scaling cooldowns. Target tracking and step scaling use instance warmup rather than simple cooldown waiting.

Parity implications:
- Model groups, launch templates/configurations, desired/min/max capacity, instances, health checks, lifecycle states, lifecycle hooks, warm pools, scaling policies, scheduled actions, and activities.
- Scaling behaviour should include AZ balancing, unhealthy replacement, lifecycle wait states, hook completion results, cooldowns, warmup, and Spot interruption replacement.
- Delete, detach, standby, manual scale, and force-delete paths need distinct lifecycle-hook and cooldown semantics.

## Current Network Resource Stub Semantics

Auto Scaling stores network placement inputs as local scalar or list fields.

- Launch configurations keep `security_groups` exactly as supplied by the request.
- Auto Scaling groups keep `vpc_zone_identifier` as an optional raw string; update operations replace that string when supplied.
- The service does not split the VPC zone identifier into subnets for validation, availability zone inference, or attachment tracking.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Describe

- Operations: `DescribeAccountLimits`, `DescribeAdjustmentTypes`, `DescribeAutoScalingGroups`, `DescribeAutoScalingInstances`, `DescribeAutoScalingNotificationTypes`, `DescribeInstanceRefreshes`, `DescribeLaunchConfigurations`, `DescribeLifecycleHookTypes`, `DescribeLifecycleHooks`, `DescribeLoadBalancerTargetGroups`, `DescribeLoadBalancers`, `DescribeMetricCollectionTypes`, `DescribeNotificationConfigurations`, `DescribePolicies`, `DescribeScalingActivities`, `DescribeScalingProcessTypes`, `DescribeScheduledActions`, `DescribeTags`, `DescribeTerminationPolicyTypes`, `DescribeTrafficSources`, `DescribeWarmPool`
- Traits: `paginated` (13)
- Common required input members in this group: `AutoScalingGroupName`

### Delete

- Operations: `DeleteAutoScalingGroup`, `DeleteLaunchConfiguration`, `DeleteLifecycleHook`, `DeleteNotificationConfiguration`, `DeletePolicy`, `DeleteScheduledAction`, `DeleteTags`, `DeleteWarmPool`
- Common required input members in this group: `AutoScalingGroupName`, `LaunchConfigurationName`, `LifecycleHookName`, `PolicyName`, `ScheduledActionName`, `Tags`, `TopicARN`

### Put

- Operations: `PutLifecycleHook`, `PutNotificationConfiguration`, `PutScalingPolicy`, `PutScheduledUpdateGroupAction`, `PutWarmPool`
- Common required input members in this group: `AutoScalingGroupName`, `LifecycleHookName`, `NotificationTypes`, `PolicyName`, `ScheduledActionName`, `TopicARN`

### Attach

- Operations: `AttachInstances`, `AttachLoadBalancerTargetGroups`, `AttachLoadBalancers`, `AttachTrafficSources`
- Common required input members in this group: `AutoScalingGroupName`, `LoadBalancerNames`, `TargetGroupARNs`, `TrafficSources`

### Detach

- Operations: `DetachInstances`, `DetachLoadBalancerTargetGroups`, `DetachLoadBalancers`, `DetachTrafficSources`
- Common required input members in this group: `AutoScalingGroupName`, `LoadBalancerNames`, `ShouldDecrementDesiredCapacity`, `TargetGroupARNs`, `TrafficSources`

### Create

- Operations: `CreateAutoScalingGroup`, `CreateLaunchConfiguration`, `CreateOrUpdateTags`
- Common required input members in this group: `AutoScalingGroupName`, `LaunchConfigurationName`, `MaxSize`, `MinSize`, `Tags`

### Set

- Operations: `SetDesiredCapacity`, `SetInstanceHealth`, `SetInstanceProtection`
- Common required input members in this group: `AutoScalingGroupName`, `DesiredCapacity`, `HealthStatus`, `InstanceId`, `InstanceIds`, `ProtectedFromScaleIn`

### Batch

- Operations: `BatchDeleteScheduledAction`, `BatchPutScheduledUpdateGroupAction`
- Common required input members in this group: `AutoScalingGroupName`, `ScheduledActionNames`, `ScheduledUpdateGroupActions`

### Cancel

- Operations: `CancelInstanceRefresh`
- Common required input members in this group: `AutoScalingGroupName`

### Complete

- Operations: `CompleteLifecycleAction`
- Common required input members in this group: `AutoScalingGroupName`, `LifecycleActionResult`, `LifecycleHookName`

### Disable

- Operations: `DisableMetricsCollection`
- Common required input members in this group: `AutoScalingGroupName`

### Enable

- Operations: `EnableMetricsCollection`
- Common required input members in this group: `AutoScalingGroupName`, `Granularity`

### Enter

- Operations: `EnterStandby`
- Common required input members in this group: `AutoScalingGroupName`, `ShouldDecrementDesiredCapacity`

### Execute

- Operations: `ExecutePolicy`
- Common required input members in this group: `PolicyName`

### Exit

- Operations: `ExitStandby`
- Common required input members in this group: `AutoScalingGroupName`

### Get

- Operations: `GetPredictiveScalingForecast`
- Common required input members in this group: `AutoScalingGroupName`, `EndTime`, `PolicyName`, `StartTime`

### Launch

- Operations: `LaunchInstances`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `AutoScalingGroupName`, `ClientToken`, `RequestedCapacity`

### Record

- Operations: `RecordLifecycleActionHeartbeat`
- Common required input members in this group: `AutoScalingGroupName`, `LifecycleHookName`

### Resume

- Operations: `ResumeProcesses`
- Common required input members in this group: `AutoScalingGroupName`

### Rollback

- Operations: `RollbackInstanceRefresh`
- Common required input members in this group: `AutoScalingGroupName`

### Start

- Operations: `StartInstanceRefresh`
- Common required input members in this group: `AutoScalingGroupName`

### Suspend

- Operations: `SuspendProcesses`
- Common required input members in this group: `AutoScalingGroupName`

### Terminate

- Operations: `TerminateInstanceInAutoScalingGroup`
- Common required input members in this group: `InstanceId`, `ShouldDecrementDesiredCapacity`

### Update

- Operations: `UpdateAutoScalingGroup`
- Common required input members in this group: `AutoScalingGroupName`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AttachInstances` | - | - | `AutoScalingGroupName` | - | `Unit` | `ResourceContentionFault`, `ServiceLinkedRoleFailure` | Attaches one or more EC2 instances to the specified Auto Scaling group. When you attach instances, Amazon EC2 Auto Scaling increases the desired capacity of the group by the number of instances being attached. |
| `AttachLoadBalancerTargetGroups` | - | - | `AutoScalingGroupName`, `TargetGroupARNs` | - | `AttachLoadBalancerTargetGroupsResultType` | `InstanceRefreshInProgressFault`, `ResourceContentionFault`, `ServiceLinkedRoleFailure` | This API operation is superseded by AttachTrafficSources, which can attach multiple traffic sources types. We recommend using `AttachTrafficSources` to simplify how you manage traffic sources. |
| `AttachLoadBalancers` | - | - | `AutoScalingGroupName`, `LoadBalancerNames` | - | `AttachLoadBalancersResultType` | `InstanceRefreshInProgressFault`, `ResourceContentionFault`, `ServiceLinkedRoleFailure` | This API operation is superseded by AttachTrafficSources, which can attach multiple traffic sources types. We recommend using `AttachTrafficSources` to simplify how you manage traffic sources. |
| `AttachTrafficSources` | - | - | `AutoScalingGroupName`, `TrafficSources` | - | `AttachTrafficSourcesResultType` | `InstanceRefreshInProgressFault`, `ResourceContentionFault`, `ServiceLinkedRoleFailure` | Attaches one or more traffic sources to the specified Auto Scaling group. You can use any of the following as traffic sources for an Auto Scaling group: Application Load Balancer Classic Load Balancer Gateway Load Balancer Network Load Balancer VPC Lattice... |
| `BatchDeleteScheduledAction` | - | - | `AutoScalingGroupName`, `ScheduledActionNames` | - | `BatchDeleteScheduledActionAnswer` | `ResourceContentionFault` | Deletes one or more scheduled actions for the specified Auto Scaling group. |
| `BatchPutScheduledUpdateGroupAction` | - | - | `AutoScalingGroupName`, `ScheduledUpdateGroupActions` | - | `BatchPutScheduledUpdateGroupActionAnswer` | `AlreadyExistsFault`, `LimitExceededFault`, `ResourceContentionFault` | Creates or updates one or more scheduled scaling actions for an Auto Scaling group. |
| `CancelInstanceRefresh` | - | - | `AutoScalingGroupName` | - | `CancelInstanceRefreshAnswer` | `ActiveInstanceRefreshNotFoundFault`, `LimitExceededFault`, `ResourceContentionFault` | Cancels an instance refresh or rollback that is in progress. If an instance refresh or rollback is not in progress, an `ActiveInstanceRefreshNotFound` error occurs. |
| `CompleteLifecycleAction` | - | - | `AutoScalingGroupName`, `LifecycleActionResult`, `LifecycleHookName` | - | `CompleteLifecycleActionAnswer` | `ResourceContentionFault` | Completes the lifecycle action for the specified token or instance with the specified result. This step is a part of the procedure for adding a lifecycle hook to an Auto Scaling group: (Optional) Create a launch template or launch configuration with a user... |
| `CreateAutoScalingGroup` | - | - | `AutoScalingGroupName`, `MaxSize`, `MinSize` | - | `Unit` | `AlreadyExistsFault`, `LimitExceededFault`, `ResourceContentionFault`, `ServiceLinkedRoleFailure` | We strongly recommend using a launch template when calling this operation to ensure full functionality for Amazon EC2 Auto Scaling and Amazon EC2. Creates an Auto Scaling group with the specified name and attributes. |
| `CreateLaunchConfiguration` | - | - | `LaunchConfigurationName` | - | `Unit` | `AlreadyExistsFault`, `LimitExceededFault`, `ResourceContentionFault` | Creates a launch configuration. If you exceed your maximum limit of launch configurations, the call fails. |
| `CreateOrUpdateTags` | - | - | `Tags` | - | `Unit` | `AlreadyExistsFault`, `LimitExceededFault`, `ResourceContentionFault`, `ResourceInUseFault` | Creates or updates tags for the specified Auto Scaling group. When you specify a tag with a key that already exists, the operation overwrites the previous tag definition, and you do not get an error message. |
| `DeleteAutoScalingGroup` | - | - | `AutoScalingGroupName` | - | `Unit` | `ResourceContentionFault`, `ResourceInUseFault`, `ScalingActivityInProgressFault` | Deletes the specified Auto Scaling group. If the group has instances or scaling activities in progress, you must specify the option to force the deletion in order for it to succeed. |
| `DeleteLaunchConfiguration` | - | - | `LaunchConfigurationName` | - | `Unit` | `ResourceContentionFault`, `ResourceInUseFault` | Deletes the specified launch configuration. The launch configuration must not be attached to an Auto Scaling group. |
| `DeleteLifecycleHook` | - | - | `AutoScalingGroupName`, `LifecycleHookName` | - | `DeleteLifecycleHookAnswer` | `ResourceContentionFault` | Deletes the specified lifecycle hook. If there are any outstanding lifecycle actions, they are completed first (`ABANDON` for launching instances, `CONTINUE` for terminating instances). |
| `DeleteNotificationConfiguration` | - | - | `AutoScalingGroupName`, `TopicARN` | - | `Unit` | `ResourceContentionFault` | Deletes the specified notification. |
| `DeletePolicy` | - | - | `PolicyName` | - | `Unit` | `ResourceContentionFault`, `ServiceLinkedRoleFailure` | Deletes the specified scaling policy. Deleting either a step scaling policy or a simple scaling policy deletes the underlying alarm action, but does not delete the alarm, even if it no longer has an associated action. |
| `DeleteScheduledAction` | - | - | `AutoScalingGroupName`, `ScheduledActionName` | - | `Unit` | `ResourceContentionFault` | Deletes the specified scheduled action. |
| `DeleteTags` | - | - | `Tags` | - | `Unit` | `ResourceContentionFault`, `ResourceInUseFault` | Deletes the specified tags. |
| `DeleteWarmPool` | - | - | `AutoScalingGroupName` | - | `DeleteWarmPoolAnswer` | `LimitExceededFault`, `ResourceContentionFault`, `ResourceInUseFault`, `ScalingActivityInProgressFault` | Deletes the warm pool for the specified Auto Scaling group. For more information, see Warm pools for Amazon EC2 Auto Scaling in the Amazon EC2 Auto Scaling User Guide . |
| `DescribeAccountLimits` | - | - | - | - | `DescribeAccountLimitsAnswer` | `ResourceContentionFault` | Describes the current Amazon EC2 Auto Scaling resource quotas for your account. When you establish an Amazon Web Services account, the account has initial quotas on the maximum number of Auto Scaling groups and launch configurations that you can create in a... |
| `DescribeAdjustmentTypes` | - | - | - | - | `DescribeAdjustmentTypesAnswer` | `ResourceContentionFault` | Describes the available adjustment types for step scaling and simple scaling policies. The following adjustment types are supported: `ChangeInCapacity` `ExactCapacity` `PercentChangeInCapacity` |
| `DescribeAutoScalingGroups` | - | `paginated` | - | - | `AutoScalingGroupsType` | `InvalidNextToken`, `ResourceContentionFault` | Gets information about the Auto Scaling groups in the account and Region. If you specify Auto Scaling group names, the output includes information for only the specified Auto Scaling groups. |
| `DescribeAutoScalingInstances` | - | `paginated` | - | - | `AutoScalingInstancesType` | `InvalidNextToken`, `ResourceContentionFault` | Gets information about the Auto Scaling instances in the account and Region. |
| `DescribeAutoScalingNotificationTypes` | - | - | - | - | `DescribeAutoScalingNotificationTypesAnswer` | `ResourceContentionFault` | Describes the notification types that are supported by Amazon EC2 Auto Scaling. |
| `DescribeInstanceRefreshes` | - | `paginated` | `AutoScalingGroupName` | - | `DescribeInstanceRefreshesAnswer` | `InvalidNextToken`, `ResourceContentionFault` | Gets information about the instance refreshes for the specified Auto Scaling group from the previous six weeks. This operation is part of the instance refresh feature in Amazon EC2 Auto Scaling, which helps you update instances in your Auto Scaling group... |
| `DescribeLaunchConfigurations` | - | `paginated` | - | - | `LaunchConfigurationsType` | `InvalidNextToken`, `ResourceContentionFault` | Gets information about the launch configurations in the account and Region. |
| `DescribeLifecycleHookTypes` | - | - | - | - | `DescribeLifecycleHookTypesAnswer` | `ResourceContentionFault` | Describes the available types of lifecycle hooks. The following hook types are supported: `autoscaling:EC2_INSTANCE_LAUNCHING` `autoscaling:EC2_INSTANCE_TERMINATING` |
| `DescribeLifecycleHooks` | - | - | `AutoScalingGroupName` | - | `DescribeLifecycleHooksAnswer` | `ResourceContentionFault` | Gets information about the lifecycle hooks for the specified Auto Scaling group. |
| `DescribeLoadBalancerTargetGroups` | - | `paginated` | `AutoScalingGroupName` | - | `DescribeLoadBalancerTargetGroupsResponse` | `InvalidNextToken`, `ResourceContentionFault` | This API operation is superseded by DescribeTrafficSources, which can describe multiple traffic sources types. We recommend using `DetachTrafficSources` to simplify how you manage traffic sources. |
| `DescribeLoadBalancers` | - | `paginated` | `AutoScalingGroupName` | - | `DescribeLoadBalancersResponse` | `InvalidNextToken`, `ResourceContentionFault` | This API operation is superseded by DescribeTrafficSources, which can describe multiple traffic sources types. We recommend using `DescribeTrafficSources` to simplify how you manage traffic sources. |
| `DescribeMetricCollectionTypes` | - | - | - | - | `DescribeMetricCollectionTypesAnswer` | `ResourceContentionFault` | Describes the available CloudWatch metrics for Amazon EC2 Auto Scaling. |
| `DescribeNotificationConfigurations` | - | `paginated` | - | - | `DescribeNotificationConfigurationsAnswer` | `InvalidNextToken`, `ResourceContentionFault` | Gets information about the Amazon SNS notifications that are configured for one or more Auto Scaling groups. |
| `DescribePolicies` | - | `paginated` | - | - | `PoliciesType` | `InvalidNextToken`, `ResourceContentionFault`, `ServiceLinkedRoleFailure` | Gets information about the scaling policies in the account and Region. |
| `DescribeScalingActivities` | - | `paginated` | - | - | `ActivitiesType` | `InvalidNextToken`, `ResourceContentionFault` | Gets information about the scaling activities in the account and Region. When scaling events occur, you see a record of the scaling activity in the scaling activities. |
| `DescribeScalingProcessTypes` | - | - | - | - | `ProcessesType` | `ResourceContentionFault` | Describes the scaling process types for use with the ResumeProcesses and SuspendProcesses APIs. |
| `DescribeScheduledActions` | - | `paginated` | - | - | `ScheduledActionsType` | `InvalidNextToken`, `ResourceContentionFault` | Gets information about the scheduled actions that haven't run or that have not reached their end time. To describe the scaling activities for scheduled actions that have already run, call the DescribeScalingActivities API. |
| `DescribeTags` | - | `paginated` | - | - | `TagsType` | `InvalidNextToken`, `ResourceContentionFault` | Describes the specified tags. You can use filters to limit the results. |
| `DescribeTerminationPolicyTypes` | - | - | - | - | `DescribeTerminationPolicyTypesAnswer` | `ResourceContentionFault` | Describes the termination policies supported by Amazon EC2 Auto Scaling. For more information, see Configure termination policies for Amazon EC2 Auto Scaling in the Amazon EC2 Auto Scaling User Guide . |
| `DescribeTrafficSources` | - | `paginated` | `AutoScalingGroupName` | - | `DescribeTrafficSourcesResponse` | `InvalidNextToken`, `ResourceContentionFault` | Gets information about the traffic sources for the specified Auto Scaling group. You can optionally provide a traffic source type. |
| `DescribeWarmPool` | - | `paginated` | `AutoScalingGroupName` | - | `DescribeWarmPoolAnswer` | `InvalidNextToken`, `LimitExceededFault`, `ResourceContentionFault` | Gets information about a warm pool and its instances. For more information, see Warm pools for Amazon EC2 Auto Scaling in the Amazon EC2 Auto Scaling User Guide . |
| `DetachInstances` | - | - | `AutoScalingGroupName`, `ShouldDecrementDesiredCapacity` | - | `DetachInstancesAnswer` | `ResourceContentionFault` | Removes one or more instances from the specified Auto Scaling group. After the instances are detached, you can manage them independent of the Auto Scaling group. |
| `DetachLoadBalancerTargetGroups` | - | - | `AutoScalingGroupName`, `TargetGroupARNs` | - | `DetachLoadBalancerTargetGroupsResultType` | `ResourceContentionFault` | This API operation is superseded by DetachTrafficSources, which can detach multiple traffic sources types. We recommend using `DetachTrafficSources` to simplify how you manage traffic sources. |
| `DetachLoadBalancers` | - | - | `AutoScalingGroupName`, `LoadBalancerNames` | - | `DetachLoadBalancersResultType` | `ResourceContentionFault` | This API operation is superseded by DetachTrafficSources, which can detach multiple traffic sources types. We recommend using `DetachTrafficSources` to simplify how you manage traffic sources. |
| `DetachTrafficSources` | - | - | `AutoScalingGroupName`, `TrafficSources` | - | `DetachTrafficSourcesResultType` | `ResourceContentionFault` | Detaches one or more traffic sources from the specified Auto Scaling group. When you detach a traffic source, it enters the `Removing` state while deregistering the instances in the group. |
| `DisableMetricsCollection` | - | - | `AutoScalingGroupName` | - | `Unit` | `ResourceContentionFault` | Disables group metrics collection for the specified Auto Scaling group. |
| `EnableMetricsCollection` | - | - | `AutoScalingGroupName`, `Granularity` | - | `Unit` | `ResourceContentionFault` | Enables group metrics collection for the specified Auto Scaling group. You can use these metrics to track changes in an Auto Scaling group and to set alarms on threshold values. |
| `EnterStandby` | - | - | `AutoScalingGroupName`, `ShouldDecrementDesiredCapacity` | - | `EnterStandbyAnswer` | `ResourceContentionFault` | Moves the specified instances into the standby state. If you choose to decrement the desired capacity of the Auto Scaling group, the instances can enter standby as long as the desired capacity of the Auto Scaling group after the instances are placed into... |
| `ExecutePolicy` | - | - | `PolicyName` | - | `Unit` | `ResourceContentionFault`, `ScalingActivityInProgressFault` | Executes the specified policy. This can be useful for testing the design of your scaling policy. |
| `ExitStandby` | - | - | `AutoScalingGroupName` | - | `ExitStandbyAnswer` | `ResourceContentionFault` | Moves the specified instances out of the standby state. After you put the instances back in service, the desired capacity is incremented. |
| `GetPredictiveScalingForecast` | - | - | `AutoScalingGroupName`, `EndTime`, `PolicyName`, `StartTime` | - | `GetPredictiveScalingForecastAnswer` | `ResourceContentionFault` | Retrieves the forecast data for a predictive scaling policy. Load forecasts are predictions of the hourly load values using historical load data from CloudWatch and an analysis of historical trends. |
| `LaunchInstances` | - | `idempotency-token` | `AutoScalingGroupName`, `ClientToken`, `RequestedCapacity` | `ClientToken` | `LaunchInstancesResult` | `IdempotentParameterMismatchError`, `ResourceContentionFault` | Launches a specified number of instances in an Auto Scaling group. Returns instance IDs and other details if launch is successful or error details if launch is unsuccessful. |
| `PutLifecycleHook` | - | - | `AutoScalingGroupName`, `LifecycleHookName` | - | `PutLifecycleHookAnswer` | `LimitExceededFault`, `ResourceContentionFault` | Creates or updates a lifecycle hook for the specified Auto Scaling group. Lifecycle hooks let you create solutions that are aware of events in the Auto Scaling instance lifecycle, and then perform a custom action on instances when the corresponding lifecycle... |
| `PutNotificationConfiguration` | - | - | `AutoScalingGroupName`, `NotificationTypes`, `TopicARN` | - | `Unit` | `LimitExceededFault`, `ResourceContentionFault`, `ServiceLinkedRoleFailure` | Configures an Auto Scaling group to send notifications when specified events take place. Subscribers to the specified topic can have messages delivered to an endpoint such as a web server or an email address. |
| `PutScalingPolicy` | - | - | `AutoScalingGroupName`, `PolicyName` | - | `PolicyARNType` | `LimitExceededFault`, `ResourceContentionFault`, `ServiceLinkedRoleFailure` | Creates or updates a scaling policy for an Auto Scaling group. Scaling policies are used to scale an Auto Scaling group based on configurable metrics. |
| `PutScheduledUpdateGroupAction` | - | - | `AutoScalingGroupName`, `ScheduledActionName` | - | `Unit` | `AlreadyExistsFault`, `LimitExceededFault`, `ResourceContentionFault` | Creates or updates a scheduled scaling action for an Auto Scaling group. For more information, see Scheduled scaling in the Amazon EC2 Auto Scaling User Guide . |
| `PutWarmPool` | - | - | `AutoScalingGroupName` | - | `PutWarmPoolAnswer` | `InstanceRefreshInProgressFault`, `LimitExceededFault`, `ResourceContentionFault` | Creates or updates a warm pool for the specified Auto Scaling group. A warm pool is a pool of pre-initialized EC2 instances that sits alongside the Auto Scaling group. |
| `RecordLifecycleActionHeartbeat` | - | - | `AutoScalingGroupName`, `LifecycleHookName` | - | `RecordLifecycleActionHeartbeatAnswer` | `ResourceContentionFault` | Records a heartbeat for the lifecycle action associated with the specified token or instance. This extends the timeout by the length of time defined using the PutLifecycleHook API call. |
| `ResumeProcesses` | - | - | `AutoScalingGroupName` | - | `Unit` | `ResourceContentionFault`, `ResourceInUseFault` | Resumes the specified suspended auto scaling processes, or all suspended process, for the specified Auto Scaling group. For more information, see Suspend and resume Amazon EC2 Auto Scaling processes in the Amazon EC2 Auto Scaling User Guide . |
| `RollbackInstanceRefresh` | - | - | `AutoScalingGroupName` | - | `RollbackInstanceRefreshAnswer` | `ActiveInstanceRefreshNotFoundFault`, `IrreversibleInstanceRefreshFault`, `LimitExceededFault`, `ResourceContentionFault` | Cancels an instance refresh that is in progress and rolls back any changes that it made. Amazon EC2 Auto Scaling replaces any instances that were replaced during the instance refresh. |
| `SetDesiredCapacity` | - | - | `AutoScalingGroupName`, `DesiredCapacity` | - | `Unit` | `ResourceContentionFault`, `ScalingActivityInProgressFault` | Sets the size of the specified Auto Scaling group. If a scale-in activity occurs as a result of a new `DesiredCapacity` value that is lower than the current size of the group, the Auto Scaling group uses its termination policy to determine which instances to... |
| `SetInstanceHealth` | - | - | `HealthStatus`, `InstanceId` | - | `Unit` | `ResourceContentionFault` | Sets the health status of the specified instance. For more information, see Set up a custom health check for your Auto Scaling group in the Amazon EC2 Auto Scaling User Guide . |
| `SetInstanceProtection` | - | - | `AutoScalingGroupName`, `InstanceIds`, `ProtectedFromScaleIn` | - | `SetInstanceProtectionAnswer` | `LimitExceededFault`, `ResourceContentionFault` | Updates the instance protection settings of the specified instances. This operation cannot be called on instances in a warm pool. |
| `StartInstanceRefresh` | - | - | `AutoScalingGroupName` | - | `StartInstanceRefreshAnswer` | `InstanceRefreshInProgressFault`, `LimitExceededFault`, `ResourceContentionFault` | Starts an instance refresh. This operation is part of the instance refresh feature in Amazon EC2 Auto Scaling, which helps you update instances in your Auto Scaling group. |
| `SuspendProcesses` | - | - | `AutoScalingGroupName` | - | `Unit` | `ResourceContentionFault`, `ResourceInUseFault` | Suspends the specified auto scaling processes, or all processes, for the specified Auto Scaling group. If you suspend either the `Launch` or `Terminate` process types, it can prevent other process types from functioning properly. |
| `TerminateInstanceInAutoScalingGroup` | - | - | `InstanceId`, `ShouldDecrementDesiredCapacity` | - | `ActivityType` | `ResourceContentionFault`, `ScalingActivityInProgressFault` | Terminates the specified instance and optionally adjusts the desired group size. This operation cannot be called on instances in a warm pool. |
| `UpdateAutoScalingGroup` | - | - | `AutoScalingGroupName` | - | `Unit` | `ResourceContentionFault`, `ScalingActivityInProgressFault`, `ServiceLinkedRoleFailure` | We strongly recommend that all Auto Scaling groups use launch templates to ensure full functionality for Amazon EC2 Auto Scaling and Amazon EC2. Updates the configuration for the specified Auto Scaling group. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ResourceContentionFault` | `structure` | `message` | You already have a pending update to an Amazon EC2 Auto Scaling resource (for example, an Auto Scaling group, instance, or load balancer). |
| `LimitExceededFault` | `structure` | `message` | You have already reached a limit for your Amazon EC2 Auto Scaling resources (for example, Auto Scaling groups, launch configurations, or lifecycle hooks). |
| `InvalidNextToken` | `structure` | `message` | The `NextToken` value is not valid. |
| `ServiceLinkedRoleFailure` | `structure` | `message` | The service-linked role is not yet ready for use. |
| `ResourceInUseFault` | `structure` | `message` | The operation can't be performed because the resource is in use. |
| `ScalingActivityInProgressFault` | `structure` | `message` | The operation can't be performed because there are scaling activities in progress. |
| `InstanceRefreshInProgressFault` | `structure` | `message` | The request failed because an active instance refresh already exists for the specified Auto Scaling group. |
| `AlreadyExistsFault` | `structure` | `message` | You already have an Auto Scaling group or launch configuration with this name. |
| `ActiveInstanceRefreshNotFoundFault` | `structure` | `message` | The request failed because an active instance refresh or rollback for the specified Auto Scaling group was not found. |
| `ScalingProcessQuery` | `structure` | `AutoScalingGroupName`, `ScalingProcesses` | - |
| `AttachInstancesQuery` | `structure` | `AutoScalingGroupName`, `InstanceIds` | - |
| `AttachLoadBalancerTargetGroupsType` | `structure` | `AutoScalingGroupName`, `TargetGroupARNs` | - |
| `AttachLoadBalancerTargetGroupsResultType` | `structure` | - | - |
| `AttachLoadBalancersType` | `structure` | `AutoScalingGroupName`, `LoadBalancerNames` | - |
| `AttachLoadBalancersResultType` | `structure` | - | - |
| `AttachTrafficSourcesType` | `structure` | `AutoScalingGroupName`, `SkipZonalShiftValidation`, `TrafficSources` | - |
| `AttachTrafficSourcesResultType` | `structure` | - | - |
| `BatchDeleteScheduledActionType` | `structure` | `AutoScalingGroupName`, `ScheduledActionNames` | - |
| `BatchDeleteScheduledActionAnswer` | `structure` | `FailedScheduledActions` | - |
| `BatchPutScheduledUpdateGroupActionType` | `structure` | `AutoScalingGroupName`, `ScheduledUpdateGroupActions` | - |
| `BatchPutScheduledUpdateGroupActionAnswer` | `structure` | `FailedScheduledUpdateGroupActions` | - |
| `CancelInstanceRefreshType` | `structure` | `AutoScalingGroupName`, `WaitForTransitioningInstances` | - |
| `CancelInstanceRefreshAnswer` | `structure` | `InstanceRefreshId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
