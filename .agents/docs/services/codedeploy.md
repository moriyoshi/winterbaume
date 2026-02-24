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

- Operations: `ListApplicationRevisions`, `ListApplications`, `ListDeploymentConfigs`, `ListDeploymentGroups`, `ListDeploymentInstances`, `ListDeploymentTargets`, `ListDeployments`, `ListGitHubAccountTokenNames`, `ListOnPremisesInstances`, `ListTagsForResource`
- Traits: `paginated` (6)
- Common required input members in this group: `ResourceArn`, `applicationName`, `deploymentId`

### Get

- Operations: `GetApplication`, `GetApplicationRevision`, `GetDeployment`, `GetDeploymentConfig`, `GetDeploymentGroup`, `GetDeploymentInstance`, `GetDeploymentTarget`, `GetOnPremisesInstance`
- Common required input members in this group: `applicationName`, `deploymentConfigName`, `deploymentGroupName`, `deploymentId`, `instanceId`, `instanceName`, `revision`, `targetId`

### Batch

- Operations: `BatchGetApplicationRevisions`, `BatchGetApplications`, `BatchGetDeploymentGroups`, `BatchGetDeploymentInstances`, `BatchGetDeploymentTargets`, `BatchGetDeployments`, `BatchGetOnPremisesInstances`
- Common required input members in this group: `applicationName`, `applicationNames`, `deploymentGroupNames`, `deploymentId`, `deploymentIds`, `instanceIds`, `instanceNames`, `revisions`, `targetIds`

### Delete

- Operations: `DeleteApplication`, `DeleteDeploymentConfig`, `DeleteDeploymentGroup`, `DeleteGitHubAccountToken`, `DeleteResourcesByExternalId`
- Common required input members in this group: `applicationName`, `deploymentConfigName`, `deploymentGroupName`

### Create

- Operations: `CreateApplication`, `CreateDeployment`, `CreateDeploymentConfig`, `CreateDeploymentGroup`
- Common required input members in this group: `applicationName`, `deploymentConfigName`, `deploymentGroupName`, `serviceRoleArn`

### Register

- Operations: `RegisterApplicationRevision`, `RegisterOnPremisesInstance`
- Common required input members in this group: `applicationName`, `instanceName`, `revision`

### Update

- Operations: `UpdateApplication`, `UpdateDeploymentGroup`
- Common required input members in this group: `applicationName`, `currentDeploymentGroupName`

### Add

- Operations: `AddTagsToOnPremisesInstances`
- Common required input members in this group: `instanceNames`, `tags`

### Continue

- Operations: `ContinueDeployment`

### Deregister

- Operations: `DeregisterOnPremisesInstance`
- Common required input members in this group: `instanceName`

### Put

- Operations: `PutLifecycleEventHookExecutionStatus`

### Remove

- Operations: `RemoveTagsFromOnPremisesInstances`
- Common required input members in this group: `instanceNames`, `tags`

### Skip

- Operations: `SkipWaitTimeForInstanceTermination`

### Stop

- Operations: `StopDeployment`
- Common required input members in this group: `deploymentId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddTagsToOnPremisesInstances` | - | - | `instanceNames`, `tags` | - | `Unit` | `InstanceLimitExceededException`, `InstanceNameRequiredException`, `InstanceNotRegisteredException`, `InvalidInstanceNameException`, `InvalidTagException`, `TagLimitExceededException`, `TagRequiredException` | Adds tags to on-premises instances. |
| `BatchGetApplicationRevisions` | - | - | `applicationName`, `revisions` | - | `BatchGetApplicationRevisionsOutput` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `BatchLimitExceededException`, `InvalidApplicationNameException`, `InvalidRevisionException`, `RevisionRequiredException` | Gets information about one or more application revisions. The maximum number of application revisions that can be returned is 25. |
| `BatchGetApplications` | - | - | `applicationNames` | - | `BatchGetApplicationsOutput` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `BatchLimitExceededException`, `InvalidApplicationNameException` | Gets information about one or more applications. The maximum number of applications that can be returned is 100. |
| `BatchGetDeploymentGroups` | - | - | `applicationName`, `deploymentGroupNames` | - | `BatchGetDeploymentGroupsOutput` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `BatchLimitExceededException`, `DeploymentConfigDoesNotExistException`, `DeploymentGroupNameRequiredException`, `InvalidApplicationNameException`, `InvalidDeploymentGroupNameException` | Gets information about one or more deployment groups. |
| `BatchGetDeploymentInstances` | - | - | `deploymentId`, `instanceIds` | - | `BatchGetDeploymentInstancesOutput` | `BatchLimitExceededException`, `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `InstanceIdRequiredException`, `InvalidComputePlatformException`, `InvalidDeploymentIdException`, `InvalidInstanceNameException` | This method works, but is deprecated. Use `BatchGetDeploymentTargets` instead. |
| `BatchGetDeploymentTargets` | - | - | `deploymentId`, `targetIds` | - | `BatchGetDeploymentTargetsOutput` | `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `DeploymentNotStartedException`, `DeploymentTargetDoesNotExistException`, `DeploymentTargetIdRequiredException`, `DeploymentTargetListSizeExceededException`, `InstanceDoesNotExistException`, `InvalidDeploymentIdException`, ... (+1) | Returns an array of one or more targets associated with a deployment. This method works with all compute types and should be used instead of the deprecated `BatchGetDeploymentInstances`. |
| `BatchGetDeployments` | - | - | `deploymentIds` | - | `BatchGetDeploymentsOutput` | `BatchLimitExceededException`, `DeploymentIdRequiredException`, `InvalidDeploymentIdException` | Gets information about one or more deployments. The maximum number of deployments that can be returned is 25. |
| `BatchGetOnPremisesInstances` | - | - | `instanceNames` | - | `BatchGetOnPremisesInstancesOutput` | `BatchLimitExceededException`, `InstanceNameRequiredException`, `InvalidInstanceNameException` | Gets information about one or more on-premises instances. The maximum number of on-premises instances that can be returned is 25. |
| `ContinueDeployment` | - | - | - | - | `Unit` | `DeploymentAlreadyCompletedException`, `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `DeploymentIsNotInReadyStateException`, `InvalidDeploymentIdException`, `InvalidDeploymentStatusException`, `InvalidDeploymentWaitTypeException`, `UnsupportedActionForDeploymentTypeException` | For a blue/green deployment, starts the process of rerouting traffic from instances in the original environment to instances in the replacement environment without waiting for a specified wait time to elapse. (Traffic rerouting, which is achieved by... |
| `CreateApplication` | - | - | `applicationName` | - | `CreateApplicationOutput` | `ApplicationAlreadyExistsException`, `ApplicationLimitExceededException`, `ApplicationNameRequiredException`, `InvalidApplicationNameException`, `InvalidComputePlatformException`, `InvalidTagsToAddException` | Creates an application. |
| `CreateDeployment` | - | - | `applicationName` | - | `CreateDeploymentOutput` | `AlarmsLimitExceededException`, `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `DeploymentConfigDoesNotExistException`, `DeploymentGroupDoesNotExistException`, `DeploymentGroupNameRequiredException`, `DeploymentLimitExceededException`, `DescriptionTooLongException`, ... (+18) | Deploys an application revision through the specified deployment group. |
| `CreateDeploymentConfig` | - | - | `deploymentConfigName` | - | `CreateDeploymentConfigOutput` | `DeploymentConfigAlreadyExistsException`, `DeploymentConfigLimitExceededException`, `DeploymentConfigNameRequiredException`, `InvalidComputePlatformException`, `InvalidDeploymentConfigNameException`, `InvalidMinimumHealthyHostValueException`, `InvalidTrafficRoutingConfigurationException`, `InvalidZonalDeploymentConfigurationException` | Creates a deployment configuration. |
| `CreateDeploymentGroup` | - | - | `applicationName`, `deploymentGroupName`, `serviceRoleArn` | - | `CreateDeploymentGroupOutput` | `AlarmsLimitExceededException`, `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `DeploymentConfigDoesNotExistException`, `DeploymentGroupAlreadyExistsException`, `DeploymentGroupLimitExceededException`, `DeploymentGroupNameRequiredException`, `ECSServiceMappingLimitExceededException`, ... (+25) | Creates a deployment group to which application revisions are deployed. |
| `DeleteApplication` | - | - | `applicationName` | - | `Unit` | `ApplicationNameRequiredException`, `InvalidApplicationNameException`, `InvalidRoleException` | Deletes an application. |
| `DeleteDeploymentConfig` | - | - | `deploymentConfigName` | - | `Unit` | `DeploymentConfigInUseException`, `DeploymentConfigNameRequiredException`, `InvalidDeploymentConfigNameException`, `InvalidOperationException` | Deletes a deployment configuration. A deployment configuration cannot be deleted if it is currently in use. |
| `DeleteDeploymentGroup` | - | - | `applicationName`, `deploymentGroupName` | - | `DeleteDeploymentGroupOutput` | `ApplicationNameRequiredException`, `DeploymentGroupNameRequiredException`, `InvalidApplicationNameException`, `InvalidDeploymentGroupNameException`, `InvalidRoleException` | Deletes a deployment group. |
| `DeleteGitHubAccountToken` | - | - | - | - | `DeleteGitHubAccountTokenOutput` | `GitHubAccountTokenDoesNotExistException`, `GitHubAccountTokenNameRequiredException`, `InvalidGitHubAccountTokenNameException`, `OperationNotSupportedException`, `ResourceValidationException` | Deletes a GitHub account connection. |
| `DeleteResourcesByExternalId` | - | - | - | - | `DeleteResourcesByExternalIdOutput` | - | Deletes resources linked to an external ID. This action only applies if you have configured blue/green deployments through CloudFormation. |
| `DeregisterOnPremisesInstance` | - | - | `instanceName` | - | `Unit` | `InstanceNameRequiredException`, `InvalidInstanceNameException` | Deregisters an on-premises instance. |
| `GetApplication` | - | - | `applicationName` | - | `GetApplicationOutput` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `InvalidApplicationNameException` | Gets information about an application. |
| `GetApplicationRevision` | - | - | `applicationName`, `revision` | - | `GetApplicationRevisionOutput` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `InvalidApplicationNameException`, `InvalidRevisionException`, `RevisionDoesNotExistException`, `RevisionRequiredException` | Gets information about an application revision. |
| `GetDeployment` | - | - | `deploymentId` | - | `GetDeploymentOutput` | `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `InvalidDeploymentIdException` | Gets information about a deployment. The `content` property of the `appSpecContent` object in the returned revision is always null. |
| `GetDeploymentConfig` | - | - | `deploymentConfigName` | - | `GetDeploymentConfigOutput` | `DeploymentConfigDoesNotExistException`, `DeploymentConfigNameRequiredException`, `InvalidComputePlatformException`, `InvalidDeploymentConfigNameException` | Gets information about a deployment configuration. |
| `GetDeploymentGroup` | - | - | `applicationName`, `deploymentGroupName` | - | `GetDeploymentGroupOutput` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `DeploymentConfigDoesNotExistException`, `DeploymentGroupDoesNotExistException`, `DeploymentGroupNameRequiredException`, `InvalidApplicationNameException`, `InvalidDeploymentGroupNameException` | Gets information about a deployment group. |
| `GetDeploymentInstance` | - | - | `deploymentId`, `instanceId` | - | `GetDeploymentInstanceOutput` | `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `InstanceDoesNotExistException`, `InstanceIdRequiredException`, `InvalidComputePlatformException`, `InvalidDeploymentIdException`, `InvalidInstanceNameException` | Gets information about an instance as part of a deployment. |
| `GetDeploymentTarget` | - | - | `deploymentId`, `targetId` | - | `GetDeploymentTargetOutput` | `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `DeploymentNotStartedException`, `DeploymentTargetDoesNotExistException`, `DeploymentTargetIdRequiredException`, `InvalidDeploymentIdException`, `InvalidDeploymentTargetIdException`, `InvalidInstanceNameException` | Returns information about a deployment target. |
| `GetOnPremisesInstance` | - | - | `instanceName` | - | `GetOnPremisesInstanceOutput` | `InstanceNameRequiredException`, `InstanceNotRegisteredException`, `InvalidInstanceNameException` | Gets information about an on-premises instance. |
| `ListApplicationRevisions` | - | `paginated` | `applicationName` | - | `ListApplicationRevisionsOutput` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `BucketNameFilterRequiredException`, `InvalidApplicationNameException`, `InvalidBucketNameFilterException`, `InvalidDeployedStateFilterException`, `InvalidKeyPrefixFilterException`, `InvalidNextTokenException`, ... (+2) | Lists information about revisions for an application. |
| `ListApplications` | - | `paginated` | - | - | `ListApplicationsOutput` | `InvalidNextTokenException` | Lists the applications registered with the user or Amazon Web Services account. |
| `ListDeploymentConfigs` | - | `paginated` | - | - | `ListDeploymentConfigsOutput` | `InvalidNextTokenException` | Lists the deployment configurations with the user or Amazon Web Services account. |
| `ListDeploymentGroups` | - | `paginated` | `applicationName` | - | `ListDeploymentGroupsOutput` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `InvalidApplicationNameException`, `InvalidNextTokenException` | Lists the deployment groups for an application registered with the Amazon Web Services user or Amazon Web Services account. |
| `ListDeploymentInstances` | - | `paginated` | `deploymentId` | - | `ListDeploymentInstancesOutput` | `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `DeploymentNotStartedException`, `InvalidComputePlatformException`, `InvalidDeploymentIdException`, `InvalidDeploymentInstanceTypeException`, `InvalidInstanceStatusException`, `InvalidInstanceTypeException`, ... (+2) | The newer `BatchGetDeploymentTargets` should be used instead because it works with all compute types. `ListDeploymentInstances` throws an exception if it is used with a compute platform other than EC2/On-premises or Lambda. |
| `ListDeploymentTargets` | - | - | `deploymentId` | - | `ListDeploymentTargetsOutput` | `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `DeploymentNotStartedException`, `InvalidDeploymentIdException`, `InvalidDeploymentInstanceTypeException`, `InvalidInstanceStatusException`, `InvalidInstanceTypeException`, `InvalidNextTokenException`, ... (+1) | Returns an array of target IDs that are associated a deployment. |
| `ListDeployments` | - | `paginated` | - | - | `ListDeploymentsOutput` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `DeploymentGroupDoesNotExistException`, `DeploymentGroupNameRequiredException`, `InvalidApplicationNameException`, `InvalidDeploymentGroupNameException`, `InvalidDeploymentStatusException`, `InvalidExternalIdException`, ... (+3) | Lists the deployments in a deployment group for an application registered with the user or Amazon Web Services account. |
| `ListGitHubAccountTokenNames` | - | - | - | - | `ListGitHubAccountTokenNamesOutput` | `InvalidNextTokenException`, `OperationNotSupportedException`, `ResourceValidationException` | Lists the names of stored connections to GitHub accounts. |
| `ListOnPremisesInstances` | - | - | - | - | `ListOnPremisesInstancesOutput` | `InvalidNextTokenException`, `InvalidRegistrationStatusException`, `InvalidTagFilterException` | Gets a list of names for one or more on-premises instances. Unless otherwise specified, both registered and deregistered on-premises instance names are listed. |
| `ListTagsForResource` | - | - | `ResourceArn` | - | `ListTagsForResourceOutput` | `ArnNotSupportedException`, `InvalidArnException`, `ResourceArnRequiredException` | Returns a list of tags for the resource identified by a specified Amazon Resource Name (ARN). Tags are used to organize and categorize your CodeDeploy resources. |
| `PutLifecycleEventHookExecutionStatus` | - | - | - | - | `PutLifecycleEventHookExecutionStatusOutput` | `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `InvalidDeploymentIdException`, `InvalidLifecycleEventHookExecutionIdException`, `InvalidLifecycleEventHookExecutionStatusException`, `LifecycleEventAlreadyCompletedException`, `UnsupportedActionForDeploymentTypeException` | Sets the result of a Lambda validation function. The function validates lifecycle hooks during a deployment that uses the Lambda or Amazon ECS compute platform. |
| `RegisterApplicationRevision` | - | - | `applicationName`, `revision` | - | `Unit` | `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `DescriptionTooLongException`, `InvalidApplicationNameException`, `InvalidRevisionException`, `RevisionRequiredException` | Registers with CodeDeploy a revision for the specified application. |
| `RegisterOnPremisesInstance` | - | - | `instanceName` | - | `Unit` | `IamArnRequiredException`, `IamSessionArnAlreadyRegisteredException`, `IamUserArnAlreadyRegisteredException`, `IamUserArnRequiredException`, `InstanceNameAlreadyRegisteredException`, `InstanceNameRequiredException`, `InvalidIamSessionArnException`, `InvalidIamUserArnException`, ... (+2) | Registers an on-premises instance. Only one IAM ARN (an IAM session ARN or IAM user ARN) is supported in the request. |
| `RemoveTagsFromOnPremisesInstances` | - | - | `instanceNames`, `tags` | - | `Unit` | `InstanceLimitExceededException`, `InstanceNameRequiredException`, `InstanceNotRegisteredException`, `InvalidInstanceNameException`, `InvalidTagException`, `TagLimitExceededException`, `TagRequiredException` | Removes one or more tags from one or more on-premises instances. |
| `SkipWaitTimeForInstanceTermination` | - | - | - | - | `Unit` | `DeploymentAlreadyCompletedException`, `DeploymentDoesNotExistException`, `DeploymentIdRequiredException`, `DeploymentNotStartedException`, `InvalidDeploymentIdException`, `UnsupportedActionForDeploymentTypeException` | In a blue/green deployment, overrides any specified wait time and starts terminating instances immediately after the traffic routing is complete. |
| `StopDeployment` | - | - | `deploymentId` | - | `StopDeploymentOutput` | `DeploymentAlreadyCompletedException`, `DeploymentDoesNotExistException`, `DeploymentGroupDoesNotExistException`, `DeploymentIdRequiredException`, `InvalidDeploymentIdException`, `UnsupportedActionForDeploymentTypeException` | Attempts to stop an ongoing deployment. |
| `TagResource` | - | - | `ResourceArn`, `Tags` | - | `TagResourceOutput` | `ApplicationDoesNotExistException`, `ArnNotSupportedException`, `DeploymentConfigDoesNotExistException`, `DeploymentGroupDoesNotExistException`, `InvalidArnException`, `InvalidTagsToAddException`, `ResourceArnRequiredException`, `TagRequiredException` | Associates the list of tags in the input `Tags` parameter with the resource identified by the `ResourceArn` input parameter. |
| `UntagResource` | - | - | `ResourceArn`, `TagKeys` | - | `UntagResourceOutput` | `ApplicationDoesNotExistException`, `ArnNotSupportedException`, `DeploymentConfigDoesNotExistException`, `DeploymentGroupDoesNotExistException`, `InvalidArnException`, `InvalidTagsToAddException`, `ResourceArnRequiredException`, `TagRequiredException` | Disassociates a resource from a list of tags. The resource is identified by the `ResourceArn` input parameter. |
| `UpdateApplication` | - | - | - | - | `Unit` | `ApplicationAlreadyExistsException`, `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `InvalidApplicationNameException` | Changes the name of an application. |
| `UpdateDeploymentGroup` | - | - | `applicationName`, `currentDeploymentGroupName` | - | `UpdateDeploymentGroupOutput` | `AlarmsLimitExceededException`, `ApplicationDoesNotExistException`, `ApplicationNameRequiredException`, `DeploymentConfigDoesNotExistException`, `DeploymentGroupAlreadyExistsException`, `DeploymentGroupDoesNotExistException`, `DeploymentGroupNameRequiredException`, `ECSServiceMappingLimitExceededException`, ... (+23) | Changes information about a deployment group. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ApplicationNameRequiredException` | `structure` | `message` | The minimum number of required application names was not specified. |
| `InvalidApplicationNameException` | `structure` | `message` | The application name was specified in an invalid format. |
| `ApplicationDoesNotExistException` | `structure` | `message` | The application does not exist with the user or Amazon Web Services account. |
| `DeploymentIdRequiredException` | `structure` | `message` | At least one deployment ID must be specified. |
| `InvalidDeploymentIdException` | `structure` | `message` | At least one of the deployment IDs was specified in an invalid format. |
| `DeploymentDoesNotExistException` | `structure` | `message` | The deployment with the user or Amazon Web Services account does not exist. |
| `InvalidInstanceNameException` | `structure` | `message` | The on-premises instance name was specified in an invalid format. |
| `InvalidNextTokenException` | `structure` | `message` | The next token was specified in an invalid format. |
| `DeploymentConfigDoesNotExistException` | `structure` | `message` | The deployment configuration does not exist with the user or Amazon Web Services account. |
| `DeploymentGroupNameRequiredException` | `structure` | `message` | The deployment group name was not specified. |
| `InvalidDeploymentGroupNameException` | `structure` | `message` | The deployment group name was specified in an invalid format. |
| `DeploymentGroupDoesNotExistException` | `structure` | `message` | The named deployment group with the user or Amazon Web Services account does not exist. |
| `InstanceNameRequiredException` | `structure` | `message` | An on-premises instance name was not specified. |
| `BatchLimitExceededException` | `structure` | `message` | The maximum number of names or IDs allowed for this request (100) was exceeded. |
| `InvalidComputePlatformException` | `structure` | `message` | The computePlatform is invalid. |
| `InvalidDeploymentConfigNameException` | `structure` | `message` | The deployment configuration name was specified in an invalid format. |
| `DeploymentNotStartedException` | `structure` | `message` | The specified deployment has not started. |
| `InvalidRoleException` | `structure` | `message` | The service role ARN was specified in an invalid format. |
| `InvalidTagException` | `structure` | `message` | The tag was specified in an invalid format. |
| `TagRequiredException` | `structure` | `message` | A tag was not specified. |
| `InvalidRevisionException` | `structure` | `message` | The revision was specified in an invalid format. |
| `RevisionRequiredException` | `structure` | `message` | The revision ID was not specified. |
| `UnsupportedActionForDeploymentTypeException` | `structure` | `message` | A call was submitted that is not supported for the specified deployment type. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
