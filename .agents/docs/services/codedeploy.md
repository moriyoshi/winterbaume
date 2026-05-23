# AWS CodeDeploy

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

CodeDeploy is a deployment service that automates application deployments to Amazon EC2 instances, on-premises instances running in your own facility, serverless Lambda functions, or applications in an Amazon ECS service. You can deploy a nearly unlimited variety of application content, such as an updated Lambda function, updated applications in an Amazon ECS service, code, web and configuration files, executables, packages, scripts, multimedia files, and so on. CodeDeploy can deploy application content stored in Amazon S3 buckets, GitHub repositories, or Bitbucket repositories. You do not need to make changes to your existing code before you can use CodeDeploy. CodeDeploy makes it easier for you to rapidly release new features, helps you avoid downtime during application deployment, and handles the complexity of updating your applications, without many of the risks associated with error-prone manual deployments.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS CodeDeploy where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: add full state-machine walks for AWS CodeDeploy resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS CodeDeploy workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Batch`, `Delete`, `Create` operation families, including `ListApplicationRevisions`, `ListApplications`, `ListDeploymentConfigs`, `ListDeploymentGroups`, `GetApplication`, `GetApplicationRevision`.

## Service Identity and Protocol

- AWS model slug: `codedeploy`
- AWS SDK for Rust slug: `codedeploy`
- Model version: `2014-10-06`
- Model file: `vendor/api-models-aws/models/codedeploy/service/2014-10-06/codedeploy-2014-10-06.json`
- SDK ID: `CodeDeploy`
- Endpoint prefix: `codedeploy`
- ARN namespace: `codedeploy`
- CloudFormation name: `CodeDeploy`
- CloudTrail event source: `codedeploy.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (10), `Get` (8), `Batch` (7), `Delete` (5), `Create` (4), `Register` (2), `Update` (2), `Add` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddTagsToOnPremisesInstances`, `BatchGetApplicationRevisions`, `BatchGetApplications`, `BatchGetDeploymentGroups`, `BatchGetDeploymentInstances`, `BatchGetDeploymentTargets`, `BatchGetDeployments`, `BatchGetOnPremisesInstances`, `CreateApplication`, `CreateDeployment`, `CreateDeploymentConfig`, `CreateDeploymentGroup`, `DeleteApplication`, `DeleteDeploymentConfig`, `DeleteDeploymentGroup`, `DeleteGitHubAccountToken`, `DeleteResourcesByExternalId`, `DeregisterOnPremisesInstance`, `PutLifecycleEventHookExecutionStatus`, `RegisterApplicationRevision`, ... (+7).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetApplication`, `GetApplicationRevision`, `GetDeployment`, `GetDeploymentConfig`, `GetDeploymentGroup`, `GetDeploymentInstance`, `GetDeploymentTarget`, `GetOnPremisesInstance`, `ListApplicationRevisions`, `ListApplications`, `ListDeploymentConfigs`, `ListDeploymentGroups`, `ListDeploymentInstances`, `ListDeploymentTargets`, `ListDeployments`, `ListGitHubAccountTokenNames`, `ListOnPremisesInstances`, `ListTagsForResource`.
- Pagination is modelled for 6 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `PutLifecycleEventHookExecutionStatus`, `StopDeployment`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 46 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `Lambda`, `EC2/VPC`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/codedeploy/latest/userguide/welcome.html
- https://docs.aws.amazon.com/codedeploy/latest/userguide/deployment-groups.html
- https://docs.aws.amazon.com/codedeploy/latest/userguide/troubleshooting-ecs.html

Research outcomes:
- CodeDeploy supports EC2/On-Premises, Lambda, and ECS compute platforms.
- EC2/On-Premises deployments can be in-place or blue/green. Lambda and ECS deployments are blue/green only.
- In-place EC2 deployments stop the application, install the revision, start and validate it, and can use a load balancer to deregister and restore instances during deployment.
- EC2 blue/green deployments create or select replacement instances, install the revision, optionally wait for tests, reroute traffic, then terminate or keep original instances.
- Lambda blue/green deployments shift traffic from one function version to another using canary, linear, or all-at-once configurations.
- ECS deployments create a replacement task set, reroute production traffic to it, and terminate the original task set after success.
- Deployment groups define target resources, deployment configuration, traffic routing, alarms, rollback settings, and, for ECS, production and optional test listeners.
- CodeDeploy can stop and roll back deployments automatically or manually when errors occur.

Parity implications:
- Model applications, deployment groups, revisions, deployments, deployment configurations, lifecycle hooks, alarms, rollback settings, and compute-platform-specific targets.
- Deployment state machines must differ by compute platform and deployment type, including traffic-shift timing, validation hooks, waits, and original-resource cleanup.
- ECS and Lambda deployments require integration with target groups, task sets, aliases, alarms, and rollback decisions.

## Operation Groups

### List

- Operations: `ListApplicationRevisions`, `ListApplications`, `ListDeploymentConfigs`, `ListDeploymentGroups`, `ListDeploymentInstances`, `ListDeployments`, `ListDeploymentTargets`, `ListGitHubAccountTokenNames`, `ListOnPremisesInstances`, `ListTagsForResource`
- Traits: `paginated` (6)
- Common required input members in this group: `applicationName`, `deploymentId`

### Get

- Operations: `GetApplication`, `GetApplicationRevision`, `GetDeployment`, `GetDeploymentConfig`, `GetDeploymentGroup`, `GetDeploymentInstance`, `GetDeploymentTarget`, `GetOnPremisesInstance`
- Common required input members in this group: `applicationName`, `deploymentId`

### Batch

- Operations: `BatchGetApplicationRevisions`, `BatchGetApplications`, `BatchGetDeploymentGroups`, `BatchGetDeploymentInstances`, `BatchGetDeployments`, `BatchGetDeploymentTargets`, `BatchGetOnPremisesInstances`
- Common required input members in this group: `applicationName`, `deploymentId`

### Delete

- Operations: `DeleteApplication`, `DeleteDeploymentConfig`, `DeleteDeploymentGroup`, `DeleteGitHubAccountToken`, `DeleteResourcesByExternalId`
- Common required input members in this group: `applicationName`

### Create

- Operations: `CreateApplication`, `CreateDeployment`, `CreateDeploymentConfig`, `CreateDeploymentGroup`
- Common required input members in this group: `applicationName`

### Register

- Operations: `RegisterApplicationRevision`, `RegisterOnPremisesInstance`
- Common required input members in this group: -

### Update

- Operations: `UpdateApplication`, `UpdateDeploymentGroup`
- Common required input members in this group: -

### Add

- Operations: `AddTagsToOnPremisesInstances`
- Common required input members in this group: -

### Continue

- Operations: `ContinueDeployment`
- Common required input members in this group: -

### Deregister

- Operations: `DeregisterOnPremisesInstance`
- Common required input members in this group: -

### Put

- Operations: `PutLifecycleEventHookExecutionStatus`
- Common required input members in this group: -

### Remove

- Operations: `RemoveTagsFromOnPremisesInstances`
- Common required input members in this group: -

### Skip

- Operations: `SkipWaitTimeForInstanceTermination`
- Common required input members in this group: -

### Stop

- Operations: `StopDeployment`
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
| `AddTagsToOnPremisesInstances` | `-` | - | `tags`, `instanceNames` | - | `Unit` | `InstanceLimitExceededException`, `InstanceNameRequiredException`, `InstanceNotRegisteredException`, `InvalidInstanceNameException`, `InvalidTagException`, `TagLimitExceededException`, `TagRequiredException` | Adds tags to on-premises instances. |
| `BatchGetApplicationRevisions` | `-` | - | `applicationName`, `revisions` | - | `BatchGetApplicationRevisionsOutput` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `BatchLimitExceededException`, `InvalidApplicationNameException`, `InvalidRevisionException`, `RevisionRequiredException` | Gets information about one or more application revisions. The maximum number of application revisions that can be returned is 25. |
| `BatchGetApplications` | `-` | - | `applicationNames` | - | `BatchGetApplicationsOutput` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `BatchLimitExceededException`, `InvalidApplicationNameException` | Gets information about one or more applications. The maximum number of applications that can be returned is 100. |
| `BatchGetDeploymentGroups` | `-` | - | `applicationName`, `deploymentGroupNames` | - | `BatchGetDeploymentGroupsOutput` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `BatchLimitExceededException`, `DeploymentConfigDoesNotExistException`, `DeploymentGroupNameRequiredException`, `InvalidApplicationNameException`, `InvalidDeploymentGroupNameException` | Gets information about one or more deployment groups. |
| `BatchGetDeploymentInstances` | `-` | - | `deploymentId`, `instanceIds` | - | `BatchGetDeploymentInstancesOutput` | `BatchLimitExceededException`, `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `InstanceIdRequiredException`, `InvalidComputePlatformException`, `InvalidDeploymentIdException`, `InvalidInstanceNameException` | This method works, but is deprecated. Use BatchGetDeploymentTargets instead. Returns an array of one or more instances associated with a deployment. This method works with EC2/On-premises and Lambda compute platforms ... |
| `BatchGetDeployments` | `-` | - | `deploymentIds` | - | `BatchGetDeploymentsOutput` | `BatchLimitExceededException`, `DeploymentIdRequiredException`, `InvalidDeploymentIdException` | Gets information about one or more deployments. The maximum number of deployments that can be returned is 25. |
| `BatchGetDeploymentTargets` | `-` | - | `deploymentId`, `targetIds` | - | `BatchGetDeploymentTargetsOutput` | `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `DeploymentNotStartedException`, `DeploymentTargetDoesNotExistException`, `DeploymentTargetIdRequiredException`, `DeploymentTargetListSizeExceededException`, `InstanceDoesNotExistException`, `InvalidDeploymentIdException`, `InvalidDeploymentTargetIdException` | Returns an array of one or more targets associated with a deployment. This method works with all compute types and should be used instead of the deprecated BatchGetDeploymentInstances . The maximum number of targets ... |
| `BatchGetOnPremisesInstances` | `-` | - | `instanceNames` | - | `BatchGetOnPremisesInstancesOutput` | `BatchLimitExceededException`, `InstanceNameRequiredException`, `InvalidInstanceNameException` | Gets information about one or more on-premises instances. The maximum number of on-premises instances that can be returned is 25. |
| `ContinueDeployment` | `-` | - | - | - | `Unit` | `DeploymentAlreadyCompletedException`, `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `DeploymentIsNotInReadyStateException`, `InvalidDeploymentIdException`, `InvalidDeploymentStatusException`, `InvalidDeploymentWaitTypeException`, `UnsupportedActionForDeploymentTypeException` | For a blue/green deployment, starts the process of rerouting traffic from instances in the original environment to instances in the replacement environment without waiting for a specified wait time to elapse. (Traffi ... |
| `CreateApplication` | `-` | - | `applicationName` | - | `CreateApplicationOutput` | `ApplicationAlreadyExistsException`, `ApplicationLimitExceededException`, `ApplicationNameRequiredException`, `InvalidApplicationNameException`, `InvalidComputePlatformException`, `InvalidTagsToAddException` | Creates an application. |
| `CreateDeployment` | `-` | - | `applicationName` | - | `CreateDeploymentOutput` | `AlarmsLimitExceededException`, `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `DeploymentConfigDoesNotExistException`, `DeploymentGroupDoesNotExistException`, `DeploymentGroupNameRequiredException`, `DeploymentLimitExceededException`, `DescriptionTooLongException`, `InvalidAlarmConfigException`, `InvalidApplicationNameException`, `InvalidAutoRollbackConfigException`, `InvalidAutoScalingGroupException`, `InvalidDeploymentConfigNameException`, `InvalidDeploymentGroupNameException`, `InvalidFileExistsBehaviorException`, `InvalidGitHubAccountTokenException`, `InvalidIgnoreApplicationStopFailuresValueException`, `InvalidLoadBalancerInfoException`, `InvalidRevisionException`, `InvalidRoleException`, `InvalidTargetInstancesException`, `InvalidTrafficRoutingConfigurationException`, `InvalidUpdateOutdatedInstancesOnlyValueException`, `RevisionDoesNotExistException`, `RevisionRequiredException`, `ThrottlingException` | Deploys an application revision through the specified deployment group. |
| `CreateDeploymentConfig` | `-` | - | `deploymentConfigName` | - | `CreateDeploymentConfigOutput` | `DeploymentConfigAlreadyExistsException`, `DeploymentConfigLimitExceededException`, `DeploymentConfigNameRequiredException`, `InvalidComputePlatformException`, `InvalidDeploymentConfigNameException`, `InvalidMinimumHealthyHostValueException`, `InvalidTrafficRoutingConfigurationException`, `InvalidZonalDeploymentConfigurationException` | Creates a deployment configuration. |
| `CreateDeploymentGroup` | `-` | - | `applicationName`, `deploymentGroupName`, `serviceRoleArn` | - | `CreateDeploymentGroupOutput` | `AlarmsLimitExceededException`, `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `DeploymentConfigDoesNotExistException`, `DeploymentGroupAlreadyExistsException`, `DeploymentGroupLimitExceededException`, `DeploymentGroupNameRequiredException`, `ECSServiceMappingLimitExceededException`, `InvalidAlarmConfigException`, `InvalidApplicationNameException`, `InvalidAutoRollbackConfigException`, `InvalidAutoScalingGroupException`, `InvalidBlueGreenDeploymentConfigurationException`, `InvalidDeploymentConfigNameException`, `InvalidDeploymentGroupNameException`, `InvalidDeploymentStyleException`, `InvalidEC2TagCombinationException`, `InvalidEC2TagException`, `InvalidECSServiceException`, `InvalidInputException`, `InvalidLoadBalancerInfoException`, `InvalidOnPremisesTagCombinationException`, `InvalidRoleException`, `InvalidTagException`, `InvalidTagsToAddException`, `InvalidTargetGroupPairException`, `InvalidTrafficRoutingConfigurationException`, `InvalidTriggerConfigException`, `LifecycleHookLimitExceededException`, `RoleRequiredException`, `TagSetListLimitExceededException`, `ThrottlingException`, `TriggerTargetsLimitExceededException` | Creates a deployment group to which application revisions are deployed. |
| `DeleteApplication` | `-` | - | `applicationName` | - | `Unit` | `ApplicationNameRequiredException`, `InvalidApplicationNameException`, `InvalidRoleException` | Deletes an application. |
| `DeleteDeploymentConfig` | `-` | - | `deploymentConfigName` | - | `Unit` | `DeploymentConfigInUseException`, `DeploymentConfigNameRequiredException`, `InvalidDeploymentConfigNameException`, `InvalidOperationException` | Deletes a deployment configuration. A deployment configuration cannot be deleted if it is currently in use. Predefined configurations cannot be deleted. |
| `DeleteDeploymentGroup` | `-` | - | `applicationName`, `deploymentGroupName` | - | `DeleteDeploymentGroupOutput` | `ApplicationNameRequiredException`, `DeploymentGroupNameRequiredException`, `InvalidApplicationNameException`, `InvalidDeploymentGroupNameException`, `InvalidRoleException` | Deletes a deployment group. |
| `DeleteGitHubAccountToken` | `-` | - | - | - | `DeleteGitHubAccountTokenOutput` | `GitHubAccountTokenDoesNotExistException`, `GitHubAccountTokenNameRequiredException`, `InvalidGitHubAccountTokenNameException`, `OperationNotSupportedException`, `ResourceValidationException` | Deletes a GitHub account connection. |
| `DeleteResourcesByExternalId` | `-` | - | - | - | `DeleteResourcesByExternalIdOutput` | - | Deletes resources linked to an external ID. This action only applies if you have configured blue/green deployments through CloudFormation. It is not necessary to call this action directly. CloudFormation calls it on ... |
| `DeregisterOnPremisesInstance` | `-` | - | `instanceName` | - | `Unit` | `InstanceNameRequiredException`, `InvalidInstanceNameException` | Deregisters an on-premises instance. |
| `GetApplication` | `-` | - | `applicationName` | - | `GetApplicationOutput` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `InvalidApplicationNameException` | Gets information about an application. |
| `GetApplicationRevision` | `-` | - | `applicationName`, `revision` | - | `GetApplicationRevisionOutput` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `InvalidApplicationNameException`, `InvalidRevisionException`, `RevisionDoesNotExistException`, `RevisionRequiredException` | Gets information about an application revision. |
| `GetDeployment` | `-` | - | `deploymentId` | - | `GetDeploymentOutput` | `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `InvalidDeploymentIdException` | Gets information about a deployment. The content property of the appSpecContent object in the returned revision is always null. Use GetApplicationRevision and the sha256 property of the returned appSpecContent object ... |
| `GetDeploymentConfig` | `-` | - | `deploymentConfigName` | - | `GetDeploymentConfigOutput` | `DeploymentConfigDoesNotExistException`, `DeploymentConfigNameRequiredException`, `InvalidComputePlatformException`, `InvalidDeploymentConfigNameException` | Gets information about a deployment configuration. |
| `GetDeploymentGroup` | `-` | - | `applicationName`, `deploymentGroupName` | - | `GetDeploymentGroupOutput` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `DeploymentConfigDoesNotExistException`, `DeploymentGroupDoesNotExistException`, `DeploymentGroupNameRequiredException`, `InvalidApplicationNameException`, `InvalidDeploymentGroupNameException` | Gets information about a deployment group. |
| `GetDeploymentInstance` | `-` | - | `deploymentId`, `instanceId` | - | `GetDeploymentInstanceOutput` | `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `InstanceDoesNotExistException`, `InstanceIdRequiredException`, `InvalidComputePlatformException`, `InvalidDeploymentIdException`, `InvalidInstanceNameException` | Gets information about an instance as part of a deployment. |
| `GetDeploymentTarget` | `-` | - | `deploymentId`, `targetId` | - | `GetDeploymentTargetOutput` | `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `DeploymentNotStartedException`, `DeploymentTargetDoesNotExistException`, `DeploymentTargetIdRequiredException`, `InvalidDeploymentIdException`, `InvalidDeploymentTargetIdException`, `InvalidInstanceNameException` | Returns information about a deployment target. |
| `GetOnPremisesInstance` | `-` | - | `instanceName` | - | `GetOnPremisesInstanceOutput` | `InstanceNameRequiredException`, `InstanceNotRegisteredException`, `InvalidInstanceNameException` | Gets information about an on-premises instance. |
| `ListApplicationRevisions` | `-` | `paginated` | `applicationName` | - | `ListApplicationRevisionsOutput` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `BucketNameFilterRequiredException`, `InvalidApplicationNameException`, `InvalidBucketNameFilterException`, `InvalidDeployedStateFilterException`, `InvalidKeyPrefixFilterException`, `InvalidNextTokenException`, `InvalidSortByException`, `InvalidSortOrderException` | Lists information about revisions for an application. |
| `ListApplications` | `-` | `paginated` | - | - | `ListApplicationsOutput` | `InvalidNextTokenException` | Lists the applications registered with the user or Amazon Web Services account. |
| `ListDeploymentConfigs` | `-` | `paginated` | - | - | `ListDeploymentConfigsOutput` | `InvalidNextTokenException` | Lists the deployment configurations with the user or Amazon Web Services account. |
| `ListDeploymentGroups` | `-` | `paginated` | `applicationName` | - | `ListDeploymentGroupsOutput` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `InvalidApplicationNameException`, `InvalidNextTokenException` | Lists the deployment groups for an application registered with the Amazon Web Services user or Amazon Web Services account. |
| `ListDeploymentInstances` | `-` | `paginated` | `deploymentId` | - | `ListDeploymentInstancesOutput` | `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `DeploymentNotStartedException`, `InvalidComputePlatformException`, `InvalidDeploymentIdException`, `InvalidDeploymentInstanceTypeException`, `InvalidInstanceStatusException`, `InvalidInstanceTypeException`, `InvalidNextTokenException`, `InvalidTargetFilterNameException` | The newer BatchGetDeploymentTargets should be used instead because it works with all compute types. ListDeploymentInstances throws an exception if it is used with a compute platform other than EC2/On-premises or Lamb ... |
| `ListDeployments` | `-` | `paginated` | - | - | `ListDeploymentsOutput` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `DeploymentGroupDoesNotExistException`, `DeploymentGroupNameRequiredException`, `InvalidApplicationNameException`, `InvalidDeploymentGroupNameException`, `InvalidDeploymentStatusException`, `InvalidExternalIdException`, `InvalidInputException`, `InvalidNextTokenException`, `InvalidTimeRangeException` | Lists the deployments in a deployment group for an application registered with the user or Amazon Web Services account. |
| `ListDeploymentTargets` | `-` | - | `deploymentId` | - | `ListDeploymentTargetsOutput` | `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `DeploymentNotStartedException`, `InvalidDeploymentIdException`, `InvalidDeploymentInstanceTypeException`, `InvalidInstanceStatusException`, `InvalidInstanceTypeException`, `InvalidNextTokenException`, `InvalidTargetFilterNameException` | Returns an array of target IDs that are associated a deployment. |
| `ListGitHubAccountTokenNames` | `-` | - | - | - | `ListGitHubAccountTokenNamesOutput` | `InvalidNextTokenException`, `OperationNotSupportedException`, `ResourceValidationException` | Lists the names of stored connections to GitHub accounts. |
| `ListOnPremisesInstances` | `-` | - | - | - | `ListOnPremisesInstancesOutput` | `InvalidNextTokenException`, `InvalidRegistrationStatusException`, `InvalidTagFilterException` | Gets a list of names for one or more on-premises instances. Unless otherwise specified, both registered and deregistered on-premises instance names are listed. To list only registered or deregistered on-premises inst ... |
| `ListTagsForResource` | `-` | - | `ResourceArn` | - | `ListTagsForResourceOutput` | `ArnNotSupportedException`, `InvalidArnException`, `ResourceArnRequiredException` | Returns a list of tags for the resource identified by a specified Amazon Resource Name (ARN). Tags are used to organize and categorize your CodeDeploy resources. |
| `PutLifecycleEventHookExecutionStatus` | `-` | - | - | - | `PutLifecycleEventHookExecutionStatusOutput` | `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `InvalidDeploymentIdException`, `InvalidLifecycleEventHookExecutionIdException`, `InvalidLifecycleEventHookExecutionStatusException`, `LifecycleEventAlreadyCompletedException`, `UnsupportedActionForDeploymentTypeException` | Sets the result of a Lambda validation function. The function validates lifecycle hooks during a deployment that uses the Lambda or Amazon ECS compute platform. For Lambda deployments, the available lifecycle hooks a ... |
| `RegisterApplicationRevision` | `-` | - | `applicationName`, `revision` | - | `Unit` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `DescriptionTooLongException`, `InvalidApplicationNameException`, `InvalidRevisionException`, `RevisionRequiredException` | Registers with CodeDeploy a revision for the specified application. |
| `RegisterOnPremisesInstance` | `-` | - | `instanceName` | - | `Unit` | `IamArnRequiredException`, `IamSessionArnAlreadyRegisteredException`, `IamUserArnAlreadyRegisteredException`, `IamUserArnRequiredException`, `InstanceNameAlreadyRegisteredException`, `InstanceNameRequiredException`, `InvalidIamSessionArnException`, `InvalidIamUserArnException`, `InvalidInstanceNameException`, `MultipleIamArnsProvidedException` | Registers an on-premises instance. Only one IAM ARN (an IAM session ARN or IAM user ARN) is supported in the request. You cannot use both. |
| `RemoveTagsFromOnPremisesInstances` | `-` | - | `tags`, `instanceNames` | - | `Unit` | `InstanceLimitExceededException`, `InstanceNameRequiredException`, `InstanceNotRegisteredException`, `InvalidInstanceNameException`, `InvalidTagException`, `TagLimitExceededException`, `TagRequiredException` | Removes one or more tags from one or more on-premises instances. |
| `SkipWaitTimeForInstanceTermination` | `-` | - | - | - | `Unit` | `DeploymentAlreadyCompletedException`, `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `DeploymentNotStartedException`, `InvalidDeploymentIdException`, `UnsupportedActionForDeploymentTypeException` | In a blue/green deployment, overrides any specified wait time and starts terminating instances immediately after the traffic routing is complete. |
| `StopDeployment` | `-` | - | `deploymentId` | - | `StopDeploymentOutput` | `DeploymentAlreadyCompletedException`, `DeploymentDoesNotExistException`, `DeploymentGroupDoesNotExistException`, `DeploymentIdRequiredException`, `InvalidDeploymentIdException`, `UnsupportedActionForDeploymentTypeException` | Attempts to stop an ongoing deployment. |
| `TagResource` | `-` | - | `ResourceArn`, `Tags` | - | `TagResourceOutput` | `ApplicationDoesNotExistException`, `ArnNotSupportedException`, `DeploymentConfigDoesNotExistException`, `DeploymentGroupDoesNotExistException`, `InvalidArnException`, `InvalidTagsToAddException`, `ResourceArnRequiredException`, `TagRequiredException` | Associates the list of tags in the input Tags parameter with the resource identified by the ResourceArn input parameter. |
| `UntagResource` | `-` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceOutput` | `ApplicationDoesNotExistException`, `ArnNotSupportedException`, `DeploymentConfigDoesNotExistException`, `DeploymentGroupDoesNotExistException`, `InvalidArnException`, `InvalidTagsToAddException`, `ResourceArnRequiredException`, `TagRequiredException` | Disassociates a resource from a list of tags. The resource is identified by the ResourceArn input parameter. The tags are identified by the list of keys in the TagKeys input parameter. |
| `UpdateApplication` | `-` | - | - | - | `Unit` | `ApplicationAlreadyExistsException`, `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `InvalidApplicationNameException` | Changes the name of an application. |
| `UpdateDeploymentGroup` | `-` | - | `applicationName`, `currentDeploymentGroupName` | - | `UpdateDeploymentGroupOutput` | `AlarmsLimitExceededException`, `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `DeploymentConfigDoesNotExistException`, `DeploymentGroupAlreadyExistsException`, `DeploymentGroupDoesNotExistException`, `DeploymentGroupNameRequiredException`, `ECSServiceMappingLimitExceededException`, `InvalidAlarmConfigException`, `InvalidApplicationNameException`, `InvalidAutoRollbackConfigException`, `InvalidAutoScalingGroupException`, `InvalidBlueGreenDeploymentConfigurationException`, `InvalidDeploymentConfigNameException`, `InvalidDeploymentGroupNameException`, `InvalidDeploymentStyleException`, `InvalidEC2TagCombinationException`, `InvalidEC2TagException`, `InvalidECSServiceException`, `InvalidInputException`, `InvalidLoadBalancerInfoException`, `InvalidOnPremisesTagCombinationException`, `InvalidRoleException`, `InvalidTagException`, `InvalidTargetGroupPairException`, `InvalidTrafficRoutingConfigurationException`, `InvalidTriggerConfigException`, `LifecycleHookLimitExceededException`, `TagSetListLimitExceededException`, `ThrottlingException`, `TriggerTargetsLimitExceededException` | Changes information about a deployment group. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AlarmsLimitExceededException` | `structure` | message | The maximum number of alarms for a deployment group (10) was exceeded. |
| `ApplicationAlreadyExistsException` | `structure` | message | An application with the specified name with the user or Amazon Web Services account already exists. |
| `ApplicationDoesNotExistException` | `structure` | message | The application does not exist with the user or Amazon Web Services account. |
| `ApplicationLimitExceededException` | `structure` | message | More applications were attempted to be created than are allowed. |
| `ApplicationNameRequiredException` | `structure` | message | The minimum number of required application names was not specified. |
| `ArnNotSupportedException` | `structure` | message | The specified ARN is not supported. For example, it might be an ARN for a resource that is not expected. |
| `BatchLimitExceededException` | `structure` | message | The maximum number of names or IDs allowed for this request (100) was exceeded. |
| `BucketNameFilterRequiredException` | `structure` | message | A bucket name is required, but was not provided. |
| `DeploymentAlreadyCompletedException` | `structure` | message | The deployment is already complete. |
| `DeploymentConfigAlreadyExistsException` | `structure` | message | A deployment configuration with the specified name with the user or Amazon Web Services account already exists. |
| `DeploymentConfigDoesNotExistException` | `structure` | message | The deployment configuration does not exist with the user or Amazon Web Services account. |
| `DeploymentConfigInUseException` | `structure` | message | The deployment configuration is still in use. |
| `DeploymentConfigLimitExceededException` | `structure` | message | The deployment configurations limit was exceeded. |
| `DeploymentConfigNameRequiredException` | `structure` | message | The deployment configuration name was not specified. |
| `DeploymentDoesNotExistException` | `structure` | message | The deployment with the user or Amazon Web Services account does not exist. |
| `DeploymentGroupAlreadyExistsException` | `structure` | message | A deployment group with the specified name with the user or Amazon Web Services account already exists. |
| `DeploymentGroupDoesNotExistException` | `structure` | message | The named deployment group with the user or Amazon Web Services account does not exist. |
| `DeploymentGroupLimitExceededException` | `structure` | message | The deployment groups limit was exceeded. |
| `DeploymentGroupNameRequiredException` | `structure` | message | The deployment group name was not specified. |
| `DeploymentIdRequiredException` | `structure` | message | At least one deployment ID must be specified. |
| `DeploymentIsNotInReadyStateException` | `structure` | message | The deployment does not have a status of Ready and can't continue yet. |
| `DeploymentLimitExceededException` | `structure` | message | The number of allowed deployments was exceeded. |
| `DeploymentNotStartedException` | `structure` | message | The specified deployment has not started. |
| `DeploymentTargetDoesNotExistException` | `structure` | message | The provided target ID does not belong to the attempted deployment. |
| `DeploymentTargetIdRequiredException` | `structure` | message | A deployment target ID was not provided. |
| `DeploymentTargetListSizeExceededException` | `structure` | message | The maximum number of targets that can be associated with an Amazon ECS or Lambda deployment was exceeded. The target list of both types of deployments must ... |
| `DescriptionTooLongException` | `structure` | message | The description is too long. |
| `ECSServiceMappingLimitExceededException` | `structure` | message | The Amazon ECS service is associated with more than one deployment groups. An Amazon ECS service can be associated with only one deployment group. |
| `GitHubAccountTokenDoesNotExistException` | `structure` | message | No GitHub account connection exists with the named specified in the call. |
| `GitHubAccountTokenNameRequiredException` | `structure` | message | The call is missing a required GitHub account connection name. |
| `IamArnRequiredException` | `structure` | message | No IAM ARN was included in the request. You must use an IAM session ARN or user ARN in the request. |
| `IamSessionArnAlreadyRegisteredException` | `structure` | message | The request included an IAM session ARN that has already been used to register a different instance. |
| `IamUserArnAlreadyRegisteredException` | `structure` | message | The specified user ARN is already registered with an on-premises instance. |
| `IamUserArnRequiredException` | `structure` | message | An user ARN was not specified. |
| `InstanceDoesNotExistException` | `structure` | message | The specified instance does not exist in the deployment group. |
| `InstanceIdRequiredException` | `structure` | message | The instance ID was not specified. |
| `InstanceLimitExceededException` | `structure` | message | The maximum number of allowed on-premises instances in a single call was exceeded. |
| `InstanceNameAlreadyRegisteredException` | `structure` | message | The specified on-premises instance name is already registered. |
| `InstanceNameRequiredException` | `structure` | message | An on-premises instance name was not specified. |
| `InstanceNotRegisteredException` | `structure` | message | The specified on-premises instance is not registered. |
| `InvalidAlarmConfigException` | `structure` | message | The format of the alarm configuration is invalid. Possible causes include: The alarm list is null. The alarm object is null. The alarm name is empty or null ... |
| `InvalidApplicationNameException` | `structure` | message | The application name was specified in an invalid format. |
| `InvalidArnException` | `structure` | message | The specified ARN is not in a valid format. |
| `InvalidAutoRollbackConfigException` | `structure` | message | The automatic rollback configuration was specified in an invalid format. For example, automatic rollback is enabled, but an invalid triggering event type or ... |
| `InvalidAutoScalingGroupException` | `structure` | message | The Auto Scaling group was specified in an invalid format or does not exist. |
| `InvalidBlueGreenDeploymentConfigurationException` | `structure` | message | The configuration for the blue/green deployment group was provided in an invalid format. For information about deployment configuration format, see CreateDe ... |
| `InvalidBucketNameFilterException` | `structure` | message | The bucket name either doesn't exist or was specified in an invalid format. |
| `InvalidComputePlatformException` | `structure` | message | The computePlatform is invalid. The computePlatform should be Lambda , Server , or ECS . |
| `InvalidDeployedStateFilterException` | `structure` | message | The deployed state filter was specified in an invalid format. |
| `InvalidDeploymentConfigNameException` | `structure` | message | The deployment configuration name was specified in an invalid format. |
| `InvalidDeploymentGroupNameException` | `structure` | message | The deployment group name was specified in an invalid format. |
| `InvalidDeploymentIdException` | `structure` | message | At least one of the deployment IDs was specified in an invalid format. |
| `InvalidDeploymentInstanceTypeException` | `structure` | message | An instance type was specified for an in-place deployment. Instance types are supported for blue/green deployments only. |
| `InvalidDeploymentStatusException` | `structure` | message | The specified deployment status doesn't exist or cannot be determined. |
| `InvalidDeploymentStyleException` | `structure` | message | An invalid deployment style was specified. Valid deployment types include "IN_PLACE" and "BLUE_GREEN." Valid deployment options include "WITH_TRAFFIC_CONTRO ... |
| `InvalidDeploymentTargetIdException` | `structure` | message | The target ID provided was not valid. |
| `InvalidDeploymentWaitTypeException` | `structure` | message | The wait type is invalid. |
| `InvalidEC2TagCombinationException` | `structure` | message | A call was submitted that specified both Ec2TagFilters and Ec2TagSet, but only one of these data types can be used in a single call. |
| `InvalidEC2TagException` | `structure` | message | The tag was specified in an invalid format. |
| `InvalidECSServiceException` | `structure` | message | The Amazon ECS service identifier is not valid. |
| `InvalidExternalIdException` | `structure` | message | The external ID was specified in an invalid format. |
| `InvalidFileExistsBehaviorException` | `structure` | message | An invalid fileExistsBehavior option was specified to determine how CodeDeploy handles files or directories that already exist in a deployment target locati ... |
| `InvalidGitHubAccountTokenException` | `structure` | message | The GitHub token is not valid. |
| `InvalidGitHubAccountTokenNameException` | `structure` | message | The format of the specified GitHub account connection name is invalid. |
| `InvalidIamSessionArnException` | `structure` | message | The IAM session ARN was specified in an invalid format. |
| `InvalidIamUserArnException` | `structure` | message | The user ARN was specified in an invalid format. |
| `InvalidIgnoreApplicationStopFailuresValueException` | `structure` | message | The IgnoreApplicationStopFailures value is invalid. For Lambda deployments, false is expected. For EC2/On-premises deployments, true or false is expected. |
| `InvalidInputException` | `structure` | message | The input was specified in an invalid format. |
| `InvalidInstanceNameException` | `structure` | message | The on-premises instance name was specified in an invalid format. |
| `InvalidInstanceStatusException` | `structure` | message | The specified instance status does not exist. |
| `InvalidInstanceTypeException` | `structure` | message | An invalid instance type was specified for instances in a blue/green deployment. Valid values include "Blue" for an original environment and "Green" for a r ... |
| `InvalidKeyPrefixFilterException` | `structure` | message | The specified key prefix filter was specified in an invalid format. |
| `InvalidLifecycleEventHookExecutionIdException` | `structure` | message | A lifecycle event hook is invalid. Review the hooks section in your AppSpec file to ensure the lifecycle events and hooks functions are valid. |
| `InvalidLifecycleEventHookExecutionStatusException` | `structure` | message | The result of a Lambda validation function that verifies a lifecycle event is invalid. It should return Succeeded or Failed . |
| `InvalidLoadBalancerInfoException` | `structure` | message | An invalid load balancer name, or no load balancer name, was specified. |
| `InvalidMinimumHealthyHostValueException` | `structure` | message | The minimum healthy instance value was specified in an invalid format. |
| `InvalidNextTokenException` | `structure` | message | The next token was specified in an invalid format. |
| `InvalidOnPremisesTagCombinationException` | `structure` | message | A call was submitted that specified both OnPremisesTagFilters and OnPremisesTagSet, but only one of these data types can be used in a single call. |
| `InvalidOperationException` | `structure` | message | An invalid operation was detected. |
| `InvalidRegistrationStatusException` | `structure` | message | The registration status was specified in an invalid format. |
| `InvalidRevisionException` | `structure` | message | The revision was specified in an invalid format. |
| `InvalidRoleException` | `structure` | message | The service role ARN was specified in an invalid format. Or, if an Auto Scaling group was specified, the specified service role does not grant the appropria ... |
| `InvalidSortByException` | `structure` | message | The column name to sort by is either not present or was specified in an invalid format. |
| `InvalidSortOrderException` | `structure` | message | The sort order was specified in an invalid format. |
| `InvalidTagException` | `structure` | message | The tag was specified in an invalid format. |
| `InvalidTagFilterException` | `structure` | message | The tag filter was specified in an invalid format. |
| `InvalidTagsToAddException` | `structure` | message | The specified tags are not valid. |
| `InvalidTargetFilterNameException` | `structure` | message | The target filter name is invalid. |
| `InvalidTargetGroupPairException` | `structure` | message | A target group pair associated with this deployment is not valid. |
| `InvalidTargetInstancesException` | `structure` | message | The target instance configuration is invalid. Possible causes include: Configuration data for target instances was entered for an in-place deployment. The l ... |
| `InvalidTimeRangeException` | `structure` | message | The specified time range was specified in an invalid format. |
| `InvalidTrafficRoutingConfigurationException` | `structure` | message | The configuration that specifies how traffic is routed during a deployment is invalid. |
| `InvalidTriggerConfigException` | `structure` | message | The trigger was specified in an invalid format. |
| `InvalidUpdateOutdatedInstancesOnlyValueException` | `structure` | message | The UpdateOutdatedInstancesOnly value is invalid. For Lambda deployments, false is expected. For EC2/On-premises deployments, true or false is expected. |
| `InvalidZonalDeploymentConfigurationException` | `structure` | message | The ZonalConfig object is not valid. |
| `LifecycleEventAlreadyCompletedException` | `structure` | message | An attempt to return the status of an already completed lifecycle event occurred. |
| `LifecycleHookLimitExceededException` | `structure` | message | The limit for lifecycle hooks was exceeded. |
| `MultipleIamArnsProvidedException` | `structure` | message | Both an user ARN and an IAM session ARN were included in the request. Use only one ARN type. |
| `OperationNotSupportedException` | `structure` | message | The API used does not support the deployment. |
| `ResourceArnRequiredException` | `structure` | message | The ARN of a resource is required, but was not found. |
| `ResourceValidationException` | `structure` | message | The specified resource could not be validated. |
| `RevisionDoesNotExistException` | `structure` | message | The named revision does not exist with the user or Amazon Web Services account. |
| `RevisionRequiredException` | `structure` | message | The revision ID was not specified. |
| `RoleRequiredException` | `structure` | message | The role ID was not specified. |
| `TagLimitExceededException` | `structure` | message | The maximum allowed number of tags was exceeded. |
| `TagRequiredException` | `structure` | message | A tag was not specified. |
| `TagSetListLimitExceededException` | `structure` | message | The number of tag groups included in the tag set list exceeded the maximum allowed limit of 3. |
| `ThrottlingException` | `structure` | message | An API function was called too frequently. |
| `TriggerTargetsLimitExceededException` | `structure` | message | The maximum allowed number of triggers was exceeded. |
| `UnsupportedActionForDeploymentTypeException` | `structure` | message | A call was submitted that is not supported for the specified deployment type. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
