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

- Operations: `ListAccountSettings`, `ListAttributes`, `ListClusters`, `ListContainerInstances`, `ListServiceDeployments`, `ListServices`, `ListServicesByNamespace`, `ListTagsForResource`, `ListTaskDefinitionFamilies`, `ListTaskDefinitions`, `ListTasks`
- Traits: `paginated` (9), `readonly` (11)
- Common required input members in this group: `namespace`, `resourceArn`, `service`, `targetType`

### Describe

- Operations: `DescribeCapacityProviders`, `DescribeClusters`, `DescribeContainerInstances`, `DescribeExpressGatewayService`, `DescribeServiceDeployments`, `DescribeServiceRevisions`, `DescribeServices`, `DescribeTaskDefinition`, `DescribeTaskSets`, `DescribeTasks`
- Traits: `readonly` (10)
- Common required input members in this group: `cluster`, `containerInstances`, `service`, `serviceArn`, `serviceDeploymentArns`, `serviceRevisionArns`, `services`, `taskDefinition`, `tasks`

### Update

- Operations: `UpdateCapacityProvider`, `UpdateCluster`, `UpdateClusterSettings`, `UpdateContainerAgent`, `UpdateContainerInstancesState`, `UpdateExpressGatewayService`, `UpdateService`, `UpdateServicePrimaryTaskSet`, `UpdateTaskProtection`, `UpdateTaskSet`
- Common required input members in this group: `cluster`, `containerInstance`, `containerInstances`, `name`, `primaryTaskSet`, `protectionEnabled`, `scale`, `service`, `serviceArn`, `settings`, `status`, `taskSet`, `tasks`

### Delete

- Operations: `DeleteAccountSetting`, `DeleteAttributes`, `DeleteCapacityProvider`, `DeleteCluster`, `DeleteExpressGatewayService`, `DeleteService`, `DeleteTaskDefinitions`, `DeleteTaskSet`
- Traits: `idempotent` (4)
- Common required input members in this group: `attributes`, `capacityProvider`, `cluster`, `name`, `service`, `serviceArn`, `taskDefinitions`, `taskSet`

### Create

- Operations: `CreateCapacityProvider`, `CreateCluster`, `CreateExpressGatewayService`, `CreateService`, `CreateTaskSet`
- Traits: `idempotent` (1)
- Common required input members in this group: `cluster`, `executionRoleArn`, `infrastructureRoleArn`, `name`, `primaryContainer`, `service`, `serviceName`, `taskDefinition`

### Put

- Operations: `PutAccountSetting`, `PutAccountSettingDefault`, `PutAttributes`, `PutClusterCapacityProviders`
- Common required input members in this group: `attributes`, `capacityProviders`, `cluster`, `defaultCapacityProviderStrategy`, `name`, `value`

### Submit

- Operations: `SubmitAttachmentStateChanges`, `SubmitContainerStateChange`, `SubmitTaskStateChange`
- Common required input members in this group: `attachments`

### Deregister

- Operations: `DeregisterContainerInstance`, `DeregisterTaskDefinition`
- Common required input members in this group: `containerInstance`, `taskDefinition`

### Register

- Operations: `RegisterContainerInstance`, `RegisterTaskDefinition`
- Common required input members in this group: `containerDefinitions`, `family`

### Stop

- Operations: `StopServiceDeployment`, `StopTask`
- Common required input members in this group: `serviceDeploymentArn`, `task`

### Discover

- Operations: `DiscoverPollEndpoint`

### Execute

- Operations: `ExecuteCommand`
- Common required input members in this group: `command`, `interactive`, `task`

### Get

- Operations: `GetTaskProtection`
- Traits: `readonly` (1)
- Common required input members in this group: `cluster`

### Run

- Operations: `RunTask`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `taskDefinition`

### Start

- Operations: `StartTask`
- Common required input members in this group: `containerInstances`, `taskDefinition`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateCapacityProvider` | - | `idempotent` | `name` | - | `CreateCapacityProviderResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `LimitExceededException`, `ServerException`, `UnsupportedFeatureException`, `UpdateInProgressException` | Creates a capacity provider. Capacity providers are associated with a cluster and are used in capacity provider strategies to facilitate cluster auto scaling. |
| `CreateCluster` | - | - | - | - | `CreateClusterResponse` | `ClientException`, `InvalidParameterException`, `NamespaceNotFoundException`, `ServerException` | Creates a new Amazon ECS cluster. By default, your account receives a `default` cluster when you launch your first container instance. |
| `CreateExpressGatewayService` | - | - | `executionRoleArn`, `infrastructureRoleArn`, `primaryContainer` | - | `CreateExpressGatewayServiceResponse` | `AccessDeniedException`, `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `PlatformTaskDefinitionIncompatibilityException`, `PlatformUnknownException`, `ServerException`, `UnsupportedFeatureException` | Creates an Express service that simplifies deploying containerized web applications on Amazon ECS with managed Amazon Web Services infrastructure. This operation provisions and configures Application Load Balancers, target groups, security groups, and... |
| `CreateService` | - | - | `serviceName` | - | `CreateServiceResponse` | `AccessDeniedException`, `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `NamespaceNotFoundException`, `PlatformTaskDefinitionIncompatibilityException`, `PlatformUnknownException`, `ServerException`, ... (+1) | Runs and maintains your desired number of tasks from a specified task definition. If the number of tasks running in a service drops below the `desiredCount`, Amazon ECS runs another copy of the task in the specified cluster. |
| `CreateTaskSet` | - | - | `cluster`, `service`, `taskDefinition` | - | `CreateTaskSetResponse` | `AccessDeniedException`, `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `NamespaceNotFoundException`, `PlatformTaskDefinitionIncompatibilityException`, `PlatformUnknownException`, `ServerException`, ... (+3) | Create a task set in the specified cluster and service. This is used when a service uses the `EXTERNAL` deployment controller type. |
| `DeleteAccountSetting` | - | - | `name` | - | `DeleteAccountSettingResponse` | `ClientException`, `InvalidParameterException`, `ServerException` | Disables an account setting for a specified user, role, or the root user for an account. |
| `DeleteAttributes` | - | - | `attributes` | - | `DeleteAttributesResponse` | `ClusterNotFoundException`, `InvalidParameterException`, `TargetNotFoundException` | Deletes one or more custom attributes from an Amazon ECS resource. |
| `DeleteCapacityProvider` | - | `idempotent` | `capacityProvider` | - | `DeleteCapacityProviderResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException`, `UnsupportedFeatureException` | Deletes the specified capacity provider. The `FARGATE` and `FARGATE_SPOT` capacity providers are reserved and can't be deleted. |
| `DeleteCluster` | - | `idempotent` | `cluster` | - | `DeleteClusterResponse` | `ClientException`, `ClusterContainsCapacityProviderException`, `ClusterContainsContainerInstancesException`, `ClusterContainsServicesException`, `ClusterContainsTasksException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException`, ... (+1) | Deletes the specified cluster. The cluster transitions to the `INACTIVE` state. |
| `DeleteExpressGatewayService` | - | - | `serviceArn` | - | `DeleteExpressGatewayServiceResponse` | `AccessDeniedException`, `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException`, `ServiceNotActiveException`, `ServiceNotFoundException`, `UnsupportedFeatureException` | Deletes an Express service and removes all associated Amazon Web Services resources. This operation stops service tasks, removes the Application Load Balancer, target groups, security groups, auto-scaling policies, and other managed infrastructure components. |
| `DeleteService` | - | `idempotent` | `service` | - | `DeleteServiceResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException`, `ServiceNotFoundException` | Deletes a specified service within a cluster. You can delete a service if you have no running tasks in it and the desired task count is zero. |
| `DeleteTaskDefinitions` | - | - | `taskDefinitions` | - | `DeleteTaskDefinitionsResponse` | `AccessDeniedException`, `ClientException`, `InvalidParameterException`, `ServerException` | Deletes one or more task definitions. You must deregister a task definition revision before you delete it. |
| `DeleteTaskSet` | - | `idempotent` | `cluster`, `service`, `taskSet` | - | `DeleteTaskSetResponse` | `AccessDeniedException`, `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException`, `ServiceNotActiveException`, `ServiceNotFoundException`, `TaskSetNotFoundException`, ... (+1) | Deletes a specified task set within a service. This is used when a service uses the `EXTERNAL` deployment controller type. |
| `DeregisterContainerInstance` | - | - | `containerInstance` | - | `DeregisterContainerInstanceResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException` | Deregisters an Amazon ECS container instance from the specified cluster. This instance is no longer available to run tasks. |
| `DeregisterTaskDefinition` | - | - | `taskDefinition` | - | `DeregisterTaskDefinitionResponse` | `ClientException`, `InvalidParameterException`, `ServerException` | Deregisters the specified task definition by family and revision. Upon deregistration, the task definition is marked as `INACTIVE`. |
| `DescribeCapacityProviders` | - | `readonly` | - | - | `DescribeCapacityProvidersResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException`, `UnsupportedFeatureException` | Describes one or more of your capacity providers. |
| `DescribeClusters` | - | `readonly` | - | - | `DescribeClustersResponse` | `ClientException`, `InvalidParameterException`, `ServerException` | Describes one or more of your clusters. For CLI examples, see describe-clusters.rst on GitHub. |
| `DescribeContainerInstances` | - | `readonly` | `containerInstances` | - | `DescribeContainerInstancesResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException` | Describes one or more container instances. Returns metadata about each container instance requested. |
| `DescribeExpressGatewayService` | - | `readonly` | `serviceArn` | - | `DescribeExpressGatewayServiceResponse` | `AccessDeniedException`, `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServerException`, `UnsupportedFeatureException` | Retrieves detailed information about an Express service, including current status, configuration, managed infrastructure, and service revisions. Returns comprehensive service details, active service revisions, ingress paths with endpoints, and managed Amazon... |
| `DescribeServiceDeployments` | - | `readonly` | `serviceDeploymentArns` | - | `DescribeServiceDeploymentsResponse` | `AccessDeniedException`, `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException`, `ServiceNotFoundException`, `UnsupportedFeatureException` | Describes one or more of your service deployments. A service deployment happens when you release a software update for the service. |
| `DescribeServiceRevisions` | - | `readonly` | `serviceRevisionArns` | - | `DescribeServiceRevisionsResponse` | `AccessDeniedException`, `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException`, `ServiceNotFoundException`, `UnsupportedFeatureException` | Describes one or more service revisions. A service revision is a version of the service that includes the values for the Amazon ECS resources (for example, task definition) and the environment resources (for example, load balancers, subnets, and security... |
| `DescribeServices` | - | `readonly` | `services` | - | `DescribeServicesResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException` | Describes the specified services running in your cluster. |
| `DescribeTaskDefinition` | - | `readonly` | `taskDefinition` | - | `DescribeTaskDefinitionResponse` | `ClientException`, `InvalidParameterException`, `ServerException` | Describes a task definition. You can specify a `family` and `revision` to find information about a specific task definition, or you can simply specify the family to find the latest `ACTIVE` revision in that family. |
| `DescribeTaskSets` | - | `readonly` | `cluster`, `service` | - | `DescribeTaskSetsResponse` | `AccessDeniedException`, `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException`, `ServiceNotActiveException`, `ServiceNotFoundException`, `UnsupportedFeatureException` | Describes the task sets in the specified cluster and service. This is used when a service uses the `EXTERNAL` deployment controller type. |
| `DescribeTasks` | - | `readonly` | `tasks` | - | `DescribeTasksResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException` | Describes a specified task or tasks. Currently, stopped tasks appear in the returned results for at least one hour. |
| `DiscoverPollEndpoint` | - | - | - | - | `DiscoverPollEndpointResponse` | `ClientException`, `ServerException` | This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent. Returns an endpoint for the Amazon ECS agent to poll for updates. |
| `ExecuteCommand` | - | - | `command`, `interactive`, `task` | - | `ExecuteCommandResponse` | `AccessDeniedException`, `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException`, `TargetNotConnectedException` | Runs a command remotely on a container within a task. If you use a condition key in your IAM policy to refine the conditions for the policy statement, for example limit the actions to a specific cluster, you receive an `AccessDeniedException` when there is a... |
| `GetTaskProtection` | - | `readonly` | `cluster` | - | `GetTaskProtectionResponse` | `AccessDeniedException`, `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServerException`, `UnsupportedFeatureException` | Retrieves the protection status of tasks in an Amazon ECS service. |
| `ListAccountSettings` | - | `readonly`, `paginated` | - | - | `ListAccountSettingsResponse` | `ClientException`, `InvalidParameterException`, `ServerException` | Lists the account settings for a specified principal. |
| `ListAttributes` | - | `readonly`, `paginated` | `targetType` | - | `ListAttributesResponse` | `ClusterNotFoundException`, `InvalidParameterException` | Lists the attributes for Amazon ECS resources within a specified target type and cluster. When you specify a target type and cluster, `ListAttributes` returns a list of attribute objects, one for each attribute on each resource. |
| `ListClusters` | - | `readonly`, `paginated` | - | - | `ListClustersResponse` | `ClientException`, `InvalidParameterException`, `ServerException` | Returns a list of existing clusters. |
| `ListContainerInstances` | - | `readonly`, `paginated` | - | - | `ListContainerInstancesResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException` | Returns a list of container instances in a specified cluster. You can filter the results of a `ListContainerInstances` operation with cluster query language statements inside the `filter` parameter. |
| `ListServiceDeployments` | - | `readonly` | `service` | - | `ListServiceDeploymentsResponse` | `AccessDeniedException`, `ClientException`, `InvalidParameterException`, `ServerException`, `ServiceNotFoundException`, `UnsupportedFeatureException` | This operation lists all the service deployments that meet the specified filter criteria. A service deployment happens when you release a software update for the service. |
| `ListServices` | - | `readonly`, `paginated` | - | - | `ListServicesResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException` | Returns a list of services. You can filter the results by cluster, launch type, and scheduling strategy. |
| `ListServicesByNamespace` | - | `readonly`, `paginated` | `namespace` | - | `ListServicesByNamespaceResponse` | `ClientException`, `InvalidParameterException`, `NamespaceNotFoundException`, `ServerException` | This operation lists all of the services that are associated with a Cloud Map namespace. This list might include services in different clusters. |
| `ListTagsForResource` | - | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException` | List the tags for an Amazon ECS resource. |
| `ListTaskDefinitionFamilies` | - | `readonly`, `paginated` | - | - | `ListTaskDefinitionFamiliesResponse` | `ClientException`, `InvalidParameterException`, `ServerException` | Returns a list of task definition families that are registered to your account. This list includes task definition families that no longer have any `ACTIVE` task definition revisions. |
| `ListTaskDefinitions` | - | `readonly`, `paginated` | - | - | `ListTaskDefinitionsResponse` | `ClientException`, `InvalidParameterException`, `ServerException` | Returns a list of task definitions that are registered to your account. You can filter the results by family name with the `familyPrefix` parameter or by status with the `status` parameter. |
| `ListTasks` | - | `readonly`, `paginated` | - | - | `ListTasksResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException`, `ServiceNotFoundException` | Returns a list of tasks. You can filter the results by cluster, task definition family, container instance, launch type, what IAM principal started the task, or by the desired status of the task. |
| `PutAccountSetting` | - | - | `name`, `value` | - | `PutAccountSettingResponse` | `ClientException`, `InvalidParameterException`, `ServerException` | Modifies an account setting. Account settings are set on a per-Region basis. |
| `PutAccountSettingDefault` | - | - | `name`, `value` | - | `PutAccountSettingDefaultResponse` | `ClientException`, `InvalidParameterException`, `ServerException` | Modifies an account setting for all users on an account for whom no individual account setting has been specified. Account settings are set on a per-Region basis. |
| `PutAttributes` | - | - | `attributes` | - | `PutAttributesResponse` | `AttributeLimitExceededException`, `ClusterNotFoundException`, `InvalidParameterException`, `TargetNotFoundException` | Create or update an attribute on an Amazon ECS resource. If the attribute doesn't exist, it's created. |
| `PutClusterCapacityProviders` | - | - | `capacityProviders`, `cluster`, `defaultCapacityProviderStrategy` | - | `PutClusterCapacityProvidersResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ResourceInUseException`, `ServerException`, `UpdateInProgressException` | Modifies the available capacity providers and the default capacity provider strategy for a cluster. You must specify both the available capacity providers and a default capacity provider strategy for the cluster. |
| `RegisterContainerInstance` | - | - | - | - | `RegisterContainerInstanceResponse` | `ClientException`, `InvalidParameterException`, `ServerException` | This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent. Registers an EC2 instance into the specified cluster. |
| `RegisterTaskDefinition` | - | - | `containerDefinitions`, `family` | - | `RegisterTaskDefinitionResponse` | `ClientException`, `InvalidParameterException`, `ServerException` | Registers a new task definition from the supplied `family` and `containerDefinitions`. Optionally, you can add data volumes to your containers with the `volumes` parameter. |
| `RunTask` | - | `idempotency-token` | `taskDefinition` | `clientToken` | `RunTaskResponse` | `AccessDeniedException`, `BlockedException`, `ClientException`, `ClusterNotFoundException`, `ConflictException`, `InvalidParameterException`, `PlatformTaskDefinitionIncompatibilityException`, `PlatformUnknownException`, ... (+2) | Starts a new task using the specified task definition. On March 21, 2024, a change was made to resolve the task definition revision before authorization. |
| `StartTask` | - | - | `containerInstances`, `taskDefinition` | - | `StartTaskResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException`, `UnsupportedFeatureException` | Starts a new task from the specified task definition on the specified container instance or instances. On March 21, 2024, a change was made to resolve the task definition revision before authorization. |
| `StopServiceDeployment` | - | - | `serviceDeploymentArn` | - | `StopServiceDeploymentResponse` | `AccessDeniedException`, `ClientException`, `ConflictException`, `InvalidParameterException`, `ServerException`, `ServiceDeploymentNotFoundException`, `UnsupportedFeatureException` | Stops an ongoing service deployment. The following stop types are avaiable: ROLLBACK - This option rolls back the service deployment to the previous service revision. |
| `StopTask` | - | - | `task` | - | `StopTaskResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException` | Stops a running task. Any tags associated with the task will be deleted. |
| `SubmitAttachmentStateChanges` | - | - | `attachments` | - | `SubmitAttachmentStateChangesResponse` | `AccessDeniedException`, `ClientException`, `InvalidParameterException`, `ServerException` | This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent. Sent to acknowledge that an attachment changed states. |
| `SubmitContainerStateChange` | - | - | - | - | `SubmitContainerStateChangeResponse` | `AccessDeniedException`, `ClientException`, `ServerException` | This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent. Sent to acknowledge that a container changed states. |
| `SubmitTaskStateChange` | - | - | - | - | `SubmitTaskStateChangeResponse` | `AccessDeniedException`, `ClientException`, `InvalidParameterException`, `ServerException` | This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent. Sent to acknowledge that a task changed states. |
| `TagResource` | - | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServerException` | Associates the specified tags to a resource with the specified `resourceArn`. If existing tags on a resource aren't specified in the request parameters, they aren't changed. |
| `UntagResource` | - | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServerException` | Deletes specified tags from a resource. |
| `UpdateCapacityProvider` | - | - | `name` | - | `UpdateCapacityProviderResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException`, `UnsupportedFeatureException` | Modifies the parameters for a capacity provider. These changes only apply to new Amazon ECS Managed Instances, or EC2 instances, not existing ones. |
| `UpdateCluster` | - | - | `cluster` | - | `UpdateClusterResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `NamespaceNotFoundException`, `ServerException` | Updates the cluster. |
| `UpdateClusterSettings` | - | - | `cluster`, `settings` | - | `UpdateClusterSettingsResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException` | Modifies the settings to use for a cluster. |
| `UpdateContainerAgent` | - | - | `containerInstance` | - | `UpdateContainerAgentResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `MissingVersionException`, `NoUpdateAvailableException`, `ServerException`, `UpdateInProgressException` | Updates the Amazon ECS container agent on a specified container instance. Updating the Amazon ECS container agent doesn't interrupt running tasks or services on the container instance. |
| `UpdateContainerInstancesState` | - | - | `containerInstances`, `status` | - | `UpdateContainerInstancesStateResponse` | `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException` | Modifies the status of an Amazon ECS container instance. Once a container instance has reached an `ACTIVE` state, you can change the status of a container instance to `DRAINING` to manually remove an instance from a cluster, for example to perform system... |
| `UpdateExpressGatewayService` | - | - | `serviceArn` | - | `UpdateExpressGatewayServiceResponse` | `AccessDeniedException`, `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException`, `ServiceNotActiveException`, `ServiceNotFoundException`, `UnsupportedFeatureException` | Updates an existing Express service configuration. Modifies container settings, resource allocation, auto-scaling configuration, and other service parameters without recreating the service. |
| `UpdateService` | - | - | `service` | - | `UpdateServiceResponse` | `AccessDeniedException`, `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `NamespaceNotFoundException`, `PlatformTaskDefinitionIncompatibilityException`, `PlatformUnknownException`, `ServerException`, ... (+3) | Modifies the parameters of a service. On March 21, 2024, a change was made to resolve the task definition revision before authorization. |
| `UpdateServicePrimaryTaskSet` | - | - | `cluster`, `primaryTaskSet`, `service` | - | `UpdateServicePrimaryTaskSetResponse` | `AccessDeniedException`, `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException`, `ServiceNotActiveException`, `ServiceNotFoundException`, `TaskSetNotFoundException`, ... (+1) | Modifies which task set in a service is the primary task set. Any parameters that are updated on the primary task set in a service will transition to the service. |
| `UpdateTaskProtection` | - | - | `cluster`, `protectionEnabled`, `tasks` | - | `UpdateTaskProtectionResponse` | `AccessDeniedException`, `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServerException`, `UnsupportedFeatureException` | Updates the protection status of a task. You can set `protectionEnabled` to `true` to protect your task from termination during scale-in events from Service Autoscaling or deployments. |
| `UpdateTaskSet` | - | - | `cluster`, `scale`, `service`, `taskSet` | - | `UpdateTaskSetResponse` | `AccessDeniedException`, `ClientException`, `ClusterNotFoundException`, `InvalidParameterException`, `ServerException`, `ServiceNotActiveException`, `ServiceNotFoundException`, `TaskSetNotFoundException`, ... (+1) | Modifies a task set. This is used when a service uses the `EXTERNAL` deployment controller type. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidParameterException` | `structure` | `message` | The specified parameter isn't valid. |
| `ClientException` | `structure` | `message` | These errors are usually caused by a client action. |
| `ServerException` | `structure` | `message` | These errors are usually caused by a server issue. |
| `ClusterNotFoundException` | `structure` | `message` | The specified cluster wasn't found. |
| `UnsupportedFeatureException` | `structure` | `message` | The specified task isn't supported in this Region. |
| `AccessDeniedException` | `structure` | `message` | You don't have authorization to perform the requested action. |
| `ServiceNotFoundException` | `structure` | `message` | The specified service wasn't found. |
| `ServiceNotActiveException` | `structure` | `message` | The specified service isn't active. |
| `NamespaceNotFoundException` | `structure` | `message` | The specified namespace wasn't found. |
| `PlatformTaskDefinitionIncompatibilityException` | `structure` | `message` | The specified platform version doesn't satisfy the required capabilities of the task definition. |
| `PlatformUnknownException` | `structure` | `message` | The specified platform version doesn't exist. |
| `ResourceNotFoundException` | `structure` | `message` | The specified resource wasn't found. |
| `UpdateInProgressException` | `structure` | `message` | There's already a current Amazon ECS container agent update in progress on the container instance that's specified. |
| `TaskSetNotFoundException` | `structure` | `message` | The specified task set wasn't found. |
| `TargetNotFoundException` | `structure` | `message` | The specified target wasn't found. |
| `ConflictException` | `structure` | `message`, `resourceIds` | The request could not be processed because of conflict in the current state of the resource. |
| `CreateCapacityProviderRequest` | `structure` | `autoScalingGroupProvider`, `cluster`, `managedInstancesProvider`, `name`, `tags` | - |
| `CreateCapacityProviderResponse` | `structure` | `capacityProvider` | - |
| `LimitExceededException` | `structure` | `message` | The limit for the resource was exceeded. |
| `CreateClusterRequest` | `structure` | `capacityProviders`, `clusterName`, `configuration`, `defaultCapacityProviderStrategy`, `serviceConnectDefaults`, `settings`, `tags` | - |
| `CreateClusterResponse` | `structure` | `cluster` | - |
| `CreateExpressGatewayServiceRequest` | `structure` | `cluster`, `cpu`, `executionRoleArn`, `healthCheckPath`, `infrastructureRoleArn`, `memory`, `networkConfiguration`, `primaryContainer`, `scalingTarget`, `serviceName`, `tags`, `taskRoleArn` | - |
| `CreateExpressGatewayServiceResponse` | `structure` | `service` | - |
| `CreateServiceRequest` | `structure` | `availabilityZoneRebalancing`, `capacityProviderStrategy`, `clientToken`, `cluster`, `deploymentConfiguration`, `deploymentController`, `desiredCount`, `enableECSManagedTags`, `enableExecuteCommand`, `healthCheckGracePeriodSeconds`, `launchType`, `loadBalancers`, ... (+14) | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
