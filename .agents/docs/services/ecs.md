# Amazon EC2 Container Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Elastic Container Service Amazon Elastic Container Service (Amazon ECS) is a highly scalable, fast, container management service. It makes it easy to run, stop, and manage Docker containers. You can host your cluster on a serverless infrastructure that's managed by Amazon ECS by launching your services or tasks on Fargate. For more control, you can host your tasks on a cluster of Amazon Elastic Compute Cloud (Amazon EC2) or External (on-premises) instances that you manage. Amazon ECS makes it easy to launch and stop container-based applications with simple API calls.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-ecs/tests/scenario_test.rs`: deploy a Fargate-style service through cluster, task definition, service creation, update, describe, and deletion.
- Backported from `scenario_test.rs`: run and inspect standalone task execution lifecycle.
- Backported from `scenario_test.rs`: configure capacity providers and cluster settings.
- Scenario insight from EC2: include mutable binding failover for Amazon EC2 Container Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon EC2 Container Service by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for Amazon EC2 Container Service resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent container orchestration workflows across clusters, task definitions, services, tasks, capacity providers, deployment state, networking, tagging, and cleanup semantics.

## Service Identity and Protocol

- AWS model slug: `ecs`
- AWS SDK for Rust slug: `ecs`
- Model version: `2014-11-13`
- Model file: `vendor/api-models-aws/models/ecs/service/2014-11-13/ecs-2014-11-13.json`
- SDK ID: `ECS`
- Endpoint prefix: `ecs`
- ARN namespace: `ecs`
- CloudFormation name: `ECS`
- CloudTrail event source: `ecs.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (11), `Describe` (10), `Update` (10), `Delete` (8), `Create` (5), `Put` (4), `Submit` (3), `Deregister` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateCapacityProvider`, `CreateCluster`, `CreateExpressGatewayService`, `CreateService`, `CreateTaskSet`, `DeleteAccountSetting`, `DeleteAttributes`, `DeleteCapacityProvider`, `DeleteCluster`, `DeleteExpressGatewayService`, `DeleteService`, `DeleteTaskDefinitions`, `DeleteTaskSet`, `DeregisterContainerInstance`, `DeregisterTaskDefinition`, `PutAccountSetting`, `PutAccountSettingDefault`, `PutAttributes`, `PutClusterCapacityProviders`, `RegisterContainerInstance`, ... (+19).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeCapacityProviders`, `DescribeClusters`, `DescribeContainerInstances`, `DescribeExpressGatewayService`, `DescribeServiceDeployments`, `DescribeServiceRevisions`, `DescribeServices`, `DescribeTaskDefinition`, `DescribeTaskSets`, `DescribeTasks`, `GetTaskProtection`, `ListAccountSettings`, `ListAttributes`, `ListClusters`, `ListContainerInstances`, `ListServiceDeployments`, `ListServices`, `ListServicesByNamespace`, `ListTagsForResource`, `ListTaskDefinitionFamilies`, ... (+2).
- Pagination is modelled for 9 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 6 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateTaskSet`, `DeleteTaskDefinitions`, `DeleteTaskSet`, `DeregisterTaskDefinition`, `DescribeTaskDefinition`, `DescribeTaskSets`, `DescribeTasks`, `GetTaskProtection`, `ListTaskDefinitionFamilies`, `ListTaskDefinitions`, `ListTasks`, `RegisterTaskDefinition`, `RunTask`, `StartTask`, `StopServiceDeployment`, `StopTask`, `SubmitTaskStateChange`, `UpdateServicePrimaryTaskSet`, `UpdateTaskProtection`, `UpdateTaskSet`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 64 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `CloudWatch Logs`, `Lambda`, `EC2/VPC`, `ECR`, `ECS`, `Secrets Manager`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `CapacityProviderResource` | `CapacityProviderName` | put: `CreateCapacityProvider`; update: `UpdateCapacityProvider`; delete: `DeleteCapacityProvider` | `DescribeCapacityProviders` | Represents a collection of parameters that are used to scale an ECS cluster when attached |
| `ClusterResource` | `ClusterName` | update: `UpdateCluster`; delete: `DeleteCluster` | `CreateCluster`, `DeregisterContainerInstance`, `DescribeClusters`, `ExecuteCommand`, `ListAttributes`, `ListClusters`, `ListContainerInstances`, `SubmitAttachmentStateChanges`, `SubmitContainerStateChange`, `SubmitTaskStateChange`, ... (+2) | Represents a logical grouping of container instances that you can place tasks on. |
| `ContainerInstanceResource` | `ClusterName`, `ContainerInstanceId` | - | `DeleteAttributes`, `DescribeContainerInstances`, `ListTasks`, `PutAttributes`, `RegisterContainerInstance`, `UpdateContainerAgent`, `UpdateContainerInstancesState` | Represents an Amazon EC2 instance that is running the Amazon ECS container agent and has been registered into a cluster. |
| `ServiceDeploymentResource` | `ClusterName`, `ServiceDeploymentId`, `ServiceName` | - | `DescribeServiceDeployments` | Represents a deployment that moves a service from one service revision to another |
| `ServiceResource` | `ClusterName`, `ServiceName` | - | `CreateExpressGatewayService`, `CreateService`, `DeleteExpressGatewayService`, `DeleteService`, `DescribeExpressGatewayService`, `DescribeServices`, `ListServiceDeployments`, `ListServices`, `StopServiceDeployment`, `UpdateExpressGatewayService`, ... (+2) | Represents a collection of instances of a task definition that are running simultaneously in an ECS cluster |
| `ServiceRevisionResource` | `ClusterName`, `ServiceName`, `ServiceRevisionId` | - | `DescribeServiceRevisions` | Represents a unique configuration that makes up a service at a point in time |
| `TaskDefinitionResource` | `TaskDefinitionFamilyName`, `TaskDefinitionRevisionNumber` | - | `DeleteTaskDefinitions`, `ListTaskDefinitions`, `RegisterTaskDefinition` | Represents a collection of settings that are used to run Docker containers in Amazon ECS |
| `TaskResource` | `ClusterName`, `TaskId` | - | `DescribeTasks`, `GetTaskProtection`, `RunTask`, `StartTask`, `StopTask`, `UpdateTaskProtection` | Represents an instantiation of a task definition on a container instance that is within your cluster |
| `TaskSetResource` | `ClusterName`, `ServiceName`, `TaskSetId` | update: `UpdateTaskSet`; delete: `DeleteTaskSet` | `CreateTaskSet`, `DescribeTaskSets` | Represents a primitive that is used to manage application deployments in a controlled manner within a single ECS Service |
## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_definitions.html
- https://docs.aws.amazon.com/AmazonECS/latest/developerguide/clusters.html
- https://docs.aws.amazon.com/AmazonECS/latest/developerguide/update-service-parameters.html

Research outcomes:
- A task definition is the blueprint for a task, including image, CPU, memory, network mode, IAM role, volumes, logs, command, and container failure behaviour.
- A task is an instantiation of a task definition in a cluster. A service maintains the desired number of running tasks and replaces failed or stopped tasks.
- Clusters are regional and have lifecycle states including ACTIVE, PROVISIONING, DEPROVISIONING, FAILED, and INACTIVE.
- Capacity provider strategy controls launch placement when no launch type is supplied. A strategy can contain only one capacity-provider family.
- ECS service placement attempts to balance tasks across Availability Zones by choosing zones with fewer running tasks for the service.
- Rolling deployments use minimum healthy percent and maximum percent to decide how many tasks can be stopped or started during deployment.
- The deployment circuit breaker can fail a deployment that cannot reach steady state and optionally roll back to the last successful deployment.
- Task replacement for unhealthy tasks depends on desiredCount and maximumPercent. ECS may start a replacement first, or stop an unhealthy task first when capacity is constrained.

Parity implications:
- Represent clusters, container instances, capacity providers, task definitions, tasks, services, deployments, and service revisions as separate state.
- Implement scheduler-visible desired count, placement constraints, Availability Zone balancing, health replacement, deployment batch limits, and circuit-breaker rollback.
- Service updates should validate immutable fields, capacity-provider transitions, deployment controller type, and load-balancer health semantics.

## Current Network Resource Stub Semantics

ECS currently stores `awsvpc`-style placement inputs as task and service metadata rather than provisioning EC2 networking.

- Task and service request paths preserve network configuration values such as subnets, security groups, and public-IP assignment when the implemented handler stores the surrounding task or service.
- ECS resources do not allocate ENIs, attach them to container instances, or track subnet capacity.
- Load balancer, service discovery, and network configuration references are service-local and are not reconciled with ELB, Cloud Map, or EC2 state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### List

- Operations: `ListAccountSettings`, `ListServicesByNamespace`, `ListTagsForResource`, `ListTaskDefinitionFamilies`
- Traits: `readonly` (4), `paginated` (3)
- Common required input members in this group: -

### Put

- Operations: `PutAccountSetting`, `PutAccountSettingDefault`
- Common required input members in this group: `name`, `value`

### Delete

- Operations: `DeleteAccountSetting`
- Common required input members in this group: -

### Deregister

- Operations: `DeregisterTaskDefinition`
- Common required input members in this group: -

### Describe

- Operations: `DescribeTaskDefinition`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Discover

- Operations: `DiscoverPollEndpoint`
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
| `DeleteAccountSetting` | `-` | - | `name` | - | `DeleteAccountSettingResponse` | `AccessDeniedException`, `ClientException`, `InvalidParameterException`, `ServerException` | Disables an account setting for a specified user, role, or the root user for an account. |
| `DeregisterTaskDefinition` | `-` | - | `taskDefinition` | - | `DeregisterTaskDefinitionResponse` | `AccessDeniedException`, `ClientException`, `InvalidParameterException`, `ServerException` | Deregisters the specified task definition by family and revision. Upon deregistration, the task definition is marked as INACTIVE . Existing tasks and services that reference an INACTIVE task definition continue to ru ... |
| `DescribeTaskDefinition` | `-` | `readonly` | `taskDefinition` | - | `DescribeTaskDefinitionResponse` | `AccessDeniedException`, `ClientException`, `InvalidParameterException`, `ServerException` | Describes a task definition. You can specify a family and revision to find information about a specific task definition, or you can simply specify the family to find the latest ACTIVE revision in that family. You can ... |
| `DiscoverPollEndpoint` | `-` | - | - | - | `DiscoverPollEndpointResponse` | `AccessDeniedException`, `ClientException`, `InvalidParameterException`, `ServerException` | This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent. Returns an endpoint for the Amazon ECS agent to poll for updates. |
| `ListAccountSettings` | `-` | `readonly`, `paginated` | - | - | `ListAccountSettingsResponse` | `AccessDeniedException`, `ClientException`, `InvalidParameterException`, `ServerException` | Lists the account settings for a specified principal. |
| `ListServicesByNamespace` | `-` | `readonly`, `paginated` | `namespace` | - | `ListServicesByNamespaceResponse` | `AccessDeniedException`, `ClientException`, `InvalidParameterException`, `NamespaceNotFoundException`, `ServerException` | This operation lists all of the services that are associated with a Cloud Map namespace. This list might include services in different clusters. In contrast, ListServices can only list services in one cluster at a ti ... |
| `ListTagsForResource` | `-` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException` | List the tags for an Amazon ECS resource. |
| `ListTaskDefinitionFamilies` | `-` | `readonly`, `paginated` | - | - | `ListTaskDefinitionFamiliesResponse` | `AccessDeniedException`, `ClientException`, `InvalidParameterException`, `ServerException` | Returns a list of task definition families that are registered to your account. This list includes task definition families that no longer have any ACTIVE task definition revisions. You can filter out task definition ... |
| `PutAccountSetting` | `-` | - | `name`, `value` | - | `PutAccountSettingResponse` | `AccessDeniedException`, `ClientException`, `InvalidParameterException`, `ServerException` | Modifies an account setting. Account settings are set on a per-Region basis. If you change the root user account setting, the default settings are reset for users and roles that do not have specified individual accou ... |
| `PutAccountSettingDefault` | `-` | - | `name`, `value` | - | `PutAccountSettingDefaultResponse` | `AccessDeniedException`, `ClientException`, `InvalidParameterException`, `ServerException` | Modifies an account setting for all users on an account for whom no individual account setting has been specified. Account settings are set on a per-Region basis. |
| `TagResource` | `-` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `LimitExceededException`, `ResourceNotFoundException`, `ServerException` | Associates the specified tags to a resource with the specified resourceArn . If existing tags on a resource aren't specified in the request parameters, they aren't changed. When a resource is deleted, the tags that a ... |
| `UntagResource` | `-` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServerException` | Deletes specified tags from a resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have authorization to perform the requested action. |
| `AttributeLimitExceededException` | `structure` | message | You can apply up to 10 custom attributes for each resource. You can view the attributes of a resource with ListAttributes . You can remove existing attribut ... |
| `BlockedException` | `structure` | message | Your Amazon Web Services account was blocked. For more information, contact Amazon Web Services Support . |
| `ClientException` | `structure` | message | These errors are usually caused by a client action. This client action might be using an action or resource on behalf of a user that doesn't have permission ... |
| `ClusterContainsCapacityProviderException` | `structure` | message | The cluster contains one or more capacity providers that prevent the requested operation. This exception occurs when you try to delete a cluster that still ... |
| `ClusterContainsContainerInstancesException` | `structure` | message | You can't delete a cluster that has registered container instances. First, deregister the container instances before you can delete the cluster. For more in ... |
| `ClusterContainsServicesException` | `structure` | message | You can't delete a cluster that contains services. First, update the service to reduce its desired task count to 0, and then delete the service. For more in ... |
| `ClusterContainsTasksException` | `structure` | message | You can't delete a cluster that has active tasks. |
| `ClusterNotFoundException` | `structure` | message | The specified cluster wasn't found. You can view your available clusters with ListClusters . Amazon ECS clusters are Region specific. |
| `ConflictException` | `structure` | resourceIds, message | The request could not be processed because of conflict in the current state of the resource. |
| `DaemonNotActiveException` | `structure` | message | The specified daemon isn't active. You can't update a daemon that's inactive. If you have previously deleted a daemon, you can re-create it with CreateDaemon . |
| `DaemonNotFoundException` | `structure` | message | The specified daemon wasn't found. You can view your available daemons with ListDaemons . Amazon ECS daemons are cluster specific and Region specific. |
| `InvalidParameterException` | `structure` | message | The specified parameter isn't valid. Review the available parameters for the API request. For more information about service event errors, see Amazon ECS se ... |
| `LimitExceededException` | `structure` | message | The limit for the resource was exceeded. |
| `MissingVersionException` | `structure` | message | Amazon ECS can't determine the current version of the Amazon ECS container agent on the container instance and doesn't have enough information to proceed wi ... |
| `NamespaceNotFoundException` | `structure` | message | The specified namespace wasn't found. |
| `NoUpdateAvailableException` | `structure` | message | There's no update available for this Amazon ECS container agent. This might be because the agent is already running the latest version or because it's so ol ... |
| `PlatformTaskDefinitionIncompatibilityException` | `structure` | message | The specified platform version doesn't satisfy the required capabilities of the task definition. |
| `PlatformUnknownException` | `structure` | message | The specified platform version doesn't exist. |
| `ResourceInUseException` | `structure` | message | The specified resource is in-use and can't be removed. |
| `ResourceNotFoundException` | `structure` | message | The specified resource wasn't found. |
| `ServerException` | `structure` | message | These errors are usually caused by a server issue. |
| `ServiceDeploymentNotFoundException` | `structure` | message | The service deploy ARN that you specified in the StopServiceDeployment doesn't exist. You can use ListServiceDeployments to retrieve the service deployment ... |
| `ServiceNotActiveException` | `structure` | message | The specified service isn't active. You can't update a service that's inactive. If you have previously deleted a service, you can re-create it with CreateSe ... |
| `ServiceNotFoundException` | `structure` | message | The specified service wasn't found. You can view your available services with ListServices . Amazon ECS services are cluster specific and Region specific. |
| `TargetNotConnectedException` | `structure` | message | The execute command cannot run. This error can be caused by any of the following configuration issues: Incorrect IAM permissions The SSM agent is not instal ... |
| `TargetNotFoundException` | `structure` | message | The specified target wasn't found. You can view your available container instances with ListContainerInstances . Amazon ECS container instances are cluster- ... |
| `TaskSetNotFoundException` | `structure` | message | The specified task set wasn't found. You can view your available task sets with DescribeTaskSets . Task sets are specific to each cluster, service and Region. |
| `UnsupportedFeatureException` | `structure` | message | The specified task isn't supported in this Region. |
| `UpdateInProgressException` | `structure` | message | There's already a current Amazon ECS container agent update in progress on the container instance that's specified. If the container agent becomes disconnec ... |
| `DeleteAccountSettingRequest` | `structure` | name, principalArn | - |
| `DeleteAccountSettingResponse` | `structure` | setting | - |
| `DeregisterTaskDefinitionRequest` | `structure` | taskDefinition | - |
| `DeregisterTaskDefinitionResponse` | `structure` | taskDefinition | - |
| `DescribeTaskDefinitionRequest` | `structure` | taskDefinition, include | - |
| `DescribeTaskDefinitionResponse` | `structure` | taskDefinition, tags | - |
| `DiscoverPollEndpointRequest` | `structure` | containerInstance, cluster | - |
| `DiscoverPollEndpointResponse` | `structure` | endpoint, telemetryEndpoint, serviceConnectEndpoint | - |
| `ListAccountSettingsRequest` | `structure` | name, value, principalArn, effectiveSettings, nextToken, maxResults | - |
| `ListAccountSettingsResponse` | `structure` | settings, nextToken | - |
| `AcceleratorManufacturer` | `enum` | AMAZON_WEB_SERVICES, AMD, NVIDIA, XILINX, HABANA | - |
| `AcceleratorName` | `enum` | A100, INFERENTIA, K520, K80, M60, RADEON_PRO_V520, T4, VU9P, V100, A10G, H100, T4G | - |
| `AcceleratorType` | `enum` | GPU, FPGA, INFERENCE | - |
| `AccessType` | `enum` | PUBLIC, PRIVATE | - |
| `AgentUpdateStatus` | `enum` | PENDING, STAGING, STAGED, UPDATING, UPDATED, FAILED | - |
| `ApplicationProtocol` | `enum` | HTTP, HTTP2, GRPC | - |
| `AssignPublicIp` | `enum` | ENABLED, DISABLED | - |
| `AutoRepairActionsStatus` | `enum` | ENABLED, DISABLED | - |
| `AvailabilityZoneRebalancing` | `enum` | ENABLED, DISABLED | - |
| `BareMetal` | `enum` | INCLUDED, REQUIRED, EXCLUDED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
