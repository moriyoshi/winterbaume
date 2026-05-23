# Amazon EMR

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon EMR is a web service that makes it easier to process large amounts of data efficiently. Amazon EMR uses Hadoop processing combined with several Amazon Web Services services to do tasks such as web indexing, data mining, log file analysis, machine learning, scientific simulation, and data warehouse management.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon EMR resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon EMR workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model setup and mutation workflows that create or update service resources across the `List`, `Describe`, `Get`, `Add`, `Create` operation families, including `ListBootstrapActions`, `ListClusters`, `ListInstanceFleets`, `ListInstanceGroups`, `DescribeCluster`, `DescribeJobFlows`.

## Service Identity and Protocol

- AWS model slug: `emr`
- AWS SDK for Rust slug: `emr`
- Model version: `2009-03-31`
- Model file: `vendor/api-models-aws/models/emr/service/2009-03-31/emr-2009-03-31.json`
- SDK ID: `EMR`
- Endpoint prefix: `elasticmapreduce`
- ARN namespace: `elasticmapreduce`
- CloudFormation name: `EMR`
- CloudTrail event source: `elasticmapreduce.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (12), `Describe` (8), `Get` (7), `Add` (4), `Create` (4), `Put` (4), `Remove` (4), `Set` (4).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddInstanceFleet`, `AddInstanceGroups`, `AddJobFlowSteps`, `AddTags`, `CancelSteps`, `CreatePersistentAppUI`, `CreateSecurityConfiguration`, `CreateStudio`, `CreateStudioSessionMapping`, `DeleteSecurityConfiguration`, `DeleteStudio`, `DeleteStudioSessionMapping`, `ModifyCluster`, `ModifyInstanceFleet`, `ModifyInstanceGroups`, `PutAutoScalingPolicy`, `PutAutoTerminationPolicy`, `PutBlockPublicAccessConfiguration`, `PutManagedScalingPolicy`, `RemoveAutoScalingPolicy`, ... (+12).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeCluster`, `DescribeJobFlows`, `DescribeNotebookExecution`, `DescribePersistentAppUI`, `DescribeReleaseLabel`, `DescribeSecurityConfiguration`, `DescribeStep`, `DescribeStudio`, `GetAutoTerminationPolicy`, `GetBlockPublicAccessConfiguration`, `GetClusterSessionCredentials`, `GetManagedScalingPolicy`, `GetOnClusterAppUIPresignedURL`, `GetPersistentAppUIPresignedURL`, `GetStudioSessionMapping`, `ListBootstrapActions`, `ListClusters`, `ListInstanceFleets`, `ListInstanceGroups`, `ListInstances`, ... (+7).
- Pagination is modelled for 12 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `AddJobFlowSteps`, `CancelSteps`, `DescribeJobFlows`, `DescribeNotebookExecution`, `ListNotebookExecutions`, `RunJobFlow`, `SetKeepJobFlowAliveWhenNoSteps`, `StartNotebookExecution`, `StopNotebookExecution`, `TerminateJobFlows`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 52 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `EC2/VPC`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-overview.html
- https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-instance-group-configuration.html
- https://docs.aws.amazon.com/emr/latest/ManagementGuide/emr-automatic-scaling.html

Research outcomes:
- An EMR cluster is a collection of EC2 nodes with node types: primary, core, and optional task nodes.
- The primary node manages the cluster, coordinates data and tasks, tracks task status, and monitors cluster health. Multi-node clusters have at least one core node.
- Work can be specified as ordered steps at cluster creation, submitted later to a long-running cluster, or run directly through installed application interfaces.
- Steps are sequential by default: pending steps remain `PENDING` until earlier steps finish, then each becomes `RUNNING` and finally `COMPLETED`.
- If a step fails, its state becomes `FAILED`. By default remaining steps become `CANCELLED`, but a step can also ignore failure or terminate the cluster.
- Cluster lifecycle states include `STARTING`, `BOOTSTRAPPING`, `RUNNING`, `WAITING`, `TERMINATING`, `TERMINATED`, and `TERMINATED_WITH_ERRORS`.
- Lifecycle failures terminate the cluster and delete cluster-local data unless termination protection is enabled.
- Instance fleets let each node type use multiple EC2 instance types and target On-Demand and Spot capacities. Primary fleets choose a single instance type and purchase option.
- Uniform instance groups are simpler. A cluster can have one primary group, one core group, and up to 48 task groups, with manual or automatic scaling for core and task groups.
- EMR automatic scaling rules use CloudWatch metrics, min/max constraints, scaling adjustments, evaluation periods, and cooldowns. Scale-in can wait for task completion depending on scale-down behaviour.

Parity implications:
- Model clusters, node types, instance groups/fleets, bootstrap actions, applications, steps, step failure policy, auto-termination, termination protection, scaling policies, and cluster state separately.
- Step processing should be ordered, stateful, and failure-policy dependent rather than immediately completed.
- Scaling behaviour should update instance group or fleet capacity according to CloudWatch-driven rules and cooldowns.

## Current Network Resource Stub Semantics

EMR currently records VPC-related settings as EMR metadata.

- Security configurations can carry `block_public_security_group_rules`, but the value only affects the stored EMR security configuration document.
- Studio records include VPC ID, subnet IDs, workspace security group ID, and engine security group ID fields when created.
- Cluster placement, Studio lifecycle, and security configuration rules are not checked against EC2 subnet or security group state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### List

- Operations: `ListBootstrapActions`, `ListClusters`, `ListInstanceFleets`, `ListInstanceGroups`, `ListInstances`, `ListNotebookExecutions`, `ListReleaseLabels`, `ListSecurityConfigurations`, `ListSteps`, `ListStudioSessionMappings`, `ListStudios`, `ListSupportedInstanceTypes`
- Traits: `paginated` (12)
- Common required input members in this group: `ClusterId`, `ReleaseLabel`

### Describe

- Operations: `DescribeCluster`, `DescribeJobFlows`, `DescribeNotebookExecution`, `DescribePersistentAppUI`, `DescribeReleaseLabel`, `DescribeSecurityConfiguration`, `DescribeStep`, `DescribeStudio`
- Common required input members in this group: `ClusterId`, `Name`, `NotebookExecutionId`, `PersistentAppUIId`, `StepId`, `StudioId`

### Get

- Operations: `GetAutoTerminationPolicy`, `GetBlockPublicAccessConfiguration`, `GetClusterSessionCredentials`, `GetManagedScalingPolicy`, `GetOnClusterAppUIPresignedURL`, `GetPersistentAppUIPresignedURL`, `GetStudioSessionMapping`
- Common required input members in this group: `ClusterId`, `IdentityType`, `PersistentAppUIId`, `StudioId`

### Add

- Operations: `AddInstanceFleet`, `AddInstanceGroups`, `AddJobFlowSteps`, `AddTags`
- Common required input members in this group: `ClusterId`, `InstanceFleet`, `InstanceGroups`, `JobFlowId`, `ResourceId`, `Steps`, `Tags`

### Create

- Operations: `CreatePersistentAppUI`, `CreateSecurityConfiguration`, `CreateStudio`, `CreateStudioSessionMapping`
- Common required input members in this group: `AuthMode`, `DefaultS3Location`, `EngineSecurityGroupId`, `IdentityType`, `Name`, `SecurityConfiguration`, `ServiceRole`, `SessionPolicyArn`, `StudioId`, `SubnetIds`, `TargetResourceArn`, `VpcId`, `WorkspaceSecurityGroupId`

### Put

- Operations: `PutAutoScalingPolicy`, `PutAutoTerminationPolicy`, `PutBlockPublicAccessConfiguration`, `PutManagedScalingPolicy`
- Common required input members in this group: `AutoScalingPolicy`, `BlockPublicAccessConfiguration`, `ClusterId`, `InstanceGroupId`, `ManagedScalingPolicy`

### Remove

- Operations: `RemoveAutoScalingPolicy`, `RemoveAutoTerminationPolicy`, `RemoveManagedScalingPolicy`, `RemoveTags`
- Common required input members in this group: `ClusterId`, `InstanceGroupId`, `ResourceId`, `TagKeys`

### Set

- Operations: `SetKeepJobFlowAliveWhenNoSteps`, `SetTerminationProtection`, `SetUnhealthyNodeReplacement`, `SetVisibleToAllUsers`
- Common required input members in this group: `JobFlowIds`, `KeepJobFlowAliveWhenNoSteps`, `TerminationProtected`, `UnhealthyNodeReplacement`, `VisibleToAllUsers`

### Delete

- Operations: `DeleteSecurityConfiguration`, `DeleteStudio`, `DeleteStudioSessionMapping`
- Common required input members in this group: `IdentityType`, `Name`, `StudioId`

### Modify

- Operations: `ModifyCluster`, `ModifyInstanceFleet`, `ModifyInstanceGroups`
- Common required input members in this group: `ClusterId`, `InstanceFleet`

### Update

- Operations: `UpdateStudio`, `UpdateStudioSessionMapping`
- Common required input members in this group: `IdentityType`, `SessionPolicyArn`, `StudioId`

### Cancel

- Operations: `CancelSteps`
- Common required input members in this group: `ClusterId`, `StepIds`

### Run

- Operations: `RunJobFlow`
- Common required input members in this group: `Instances`, `Name`

### Start

- Operations: `StartNotebookExecution`
- Common required input members in this group: `ExecutionEngine`, `ServiceRole`

### Stop

- Operations: `StopNotebookExecution`
- Common required input members in this group: `NotebookExecutionId`

### Terminate

- Operations: `TerminateJobFlows`
- Common required input members in this group: `JobFlowIds`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddInstanceFleet` | - | - | `ClusterId`, `InstanceFleet` | - | `AddInstanceFleetOutput` | `InternalServerException`, `InvalidRequestException` | Adds an instance fleet to a running cluster. The instance fleet configuration is available only in Amazon EMR releases 4.8.0 and later, excluding 5.0.x. |
| `AddInstanceGroups` | - | - | `InstanceGroups`, `JobFlowId` | - | `AddInstanceGroupsOutput` | `InternalServerError` | Adds one or more instance groups to a running cluster. |
| `AddJobFlowSteps` | - | - | `JobFlowId`, `Steps` | - | `AddJobFlowStepsOutput` | `InternalServerError` | AddJobFlowSteps adds new steps to a running cluster. A maximum of 256 steps are allowed in each job flow. |
| `AddTags` | - | - | `ResourceId`, `Tags` | - | `AddTagsOutput` | `InternalServerException`, `InvalidRequestException` | Adds tags to an Amazon EMR resource, such as a cluster or an Amazon EMR Studio. Tags make it easier to associate resources in various ways, such as grouping clusters to track your Amazon EMR resource allocation costs. |
| `CancelSteps` | - | - | `ClusterId`, `StepIds` | - | `CancelStepsOutput` | `InternalServerError`, `InvalidRequestException` | Cancels a pending step or steps in a running cluster. Available only in Amazon EMR versions 4.8.0 and later, excluding version 5.0.0. |
| `CreatePersistentAppUI` | - | - | `TargetResourceArn` | - | `CreatePersistentAppUIOutput` | `InternalServerException`, `InvalidRequestException` | Creates a persistent application user interface. |
| `CreateSecurityConfiguration` | - | - | `Name`, `SecurityConfiguration` | - | `CreateSecurityConfigurationOutput` | `InternalServerException`, `InvalidRequestException` | Creates a security configuration, which is stored in the service and can be specified when a cluster is created. |
| `CreateStudio` | - | - | `AuthMode`, `DefaultS3Location`, `EngineSecurityGroupId`, `Name`, `ServiceRole`, `SubnetIds`, `VpcId`, `WorkspaceSecurityGroupId` | - | `CreateStudioOutput` | `InternalServerException`, `InvalidRequestException` | Creates a new Amazon EMR Studio. |
| `CreateStudioSessionMapping` | - | - | `IdentityType`, `SessionPolicyArn`, `StudioId` | - | `Unit` | `InternalServerError`, `InvalidRequestException` | Maps a user or group to the Amazon EMR Studio specified by `StudioId`, and applies a session policy to refine Studio permissions for that user or group. Use `CreateStudioSessionMapping` to assign users to a Studio when you use IAM Identity Center... |
| `DeleteSecurityConfiguration` | - | - | `Name` | - | `DeleteSecurityConfigurationOutput` | `InternalServerException`, `InvalidRequestException` | Deletes a security configuration. |
| `DeleteStudio` | - | - | `StudioId` | - | `Unit` | `InternalServerException`, `InvalidRequestException` | Removes an Amazon EMR Studio from the Studio metadata store. |
| `DeleteStudioSessionMapping` | - | - | `IdentityType`, `StudioId` | - | `Unit` | `InternalServerError`, `InvalidRequestException` | Removes a user or group from an Amazon EMR Studio. |
| `DescribeCluster` | - | - | `ClusterId` | - | `DescribeClusterOutput` | `InternalServerException`, `InvalidRequestException` | Provides cluster-level details including status, hardware and software configuration, VPC settings, and so on. |
| `DescribeJobFlows` | - | - | - | - | `DescribeJobFlowsOutput` | `InternalServerError` | This API is no longer supported and will eventually be removed. We recommend you use ListClusters, DescribeCluster, ListSteps, ListInstanceGroups and ListBootstrapActions instead. |
| `DescribeNotebookExecution` | - | - | `NotebookExecutionId` | - | `DescribeNotebookExecutionOutput` | `InternalServerError`, `InvalidRequestException` | Provides details of a notebook execution. |
| `DescribePersistentAppUI` | - | - | `PersistentAppUIId` | - | `DescribePersistentAppUIOutput` | `InternalServerException`, `InvalidRequestException` | Describes a persistent application user interface. |
| `DescribeReleaseLabel` | - | - | - | - | `DescribeReleaseLabelOutput` | `InternalServerException`, `InvalidRequestException` | Provides Amazon EMR release label details, such as the releases available the Region where the API request is run, and the available applications for a specific Amazon EMR release label. Can also list Amazon EMR releases that support a specified version of... |
| `DescribeSecurityConfiguration` | - | - | `Name` | - | `DescribeSecurityConfigurationOutput` | `InternalServerException`, `InvalidRequestException` | Provides the details of a security configuration by returning the configuration JSON. |
| `DescribeStep` | - | - | `ClusterId`, `StepId` | - | `DescribeStepOutput` | `InternalServerException`, `InvalidRequestException` | Provides more detail about the cluster step. |
| `DescribeStudio` | - | - | `StudioId` | - | `DescribeStudioOutput` | `InternalServerException`, `InvalidRequestException` | Returns details for the specified Amazon EMR Studio including ID, Name, VPC, Studio access URL, and so on. |
| `GetAutoTerminationPolicy` | - | - | `ClusterId` | - | `GetAutoTerminationPolicyOutput` | - | Returns the auto-termination policy for an Amazon EMR cluster. |
| `GetBlockPublicAccessConfiguration` | - | - | - | - | `GetBlockPublicAccessConfigurationOutput` | `InternalServerException`, `InvalidRequestException` | Returns the Amazon EMR block public access configuration for your Amazon Web Services account in the current Region. For more information see Configure Block Public Access for Amazon EMR in the Amazon EMR Management Guide . |
| `GetClusterSessionCredentials` | - | - | `ClusterId` | - | `GetClusterSessionCredentialsOutput` | `InternalServerError`, `InvalidRequestException` | Provides temporary, HTTP basic credentials that are associated with a given runtime IAM role and used by a cluster with fine-grained access control activated. You can use these credentials to connect to cluster endpoints that support username and password... |
| `GetManagedScalingPolicy` | - | - | `ClusterId` | - | `GetManagedScalingPolicyOutput` | - | Fetches the attached managed scaling policy for an Amazon EMR cluster. |
| `GetOnClusterAppUIPresignedURL` | - | - | `ClusterId` | - | `GetOnClusterAppUIPresignedURLOutput` | `InternalServerError`, `InvalidRequestException` | The presigned URL properties for the cluster's application user interface. |
| `GetPersistentAppUIPresignedURL` | - | - | `PersistentAppUIId` | - | `GetPersistentAppUIPresignedURLOutput` | `InternalServerError`, `InvalidRequestException` | The presigned URL properties for the cluster's application user interface. |
| `GetStudioSessionMapping` | - | - | `IdentityType`, `StudioId` | - | `GetStudioSessionMappingOutput` | `InternalServerError`, `InvalidRequestException` | Fetches mapping details for the specified Amazon EMR Studio and identity (user or group). |
| `ListBootstrapActions` | - | `paginated` | `ClusterId` | - | `ListBootstrapActionsOutput` | `InternalServerException`, `InvalidRequestException` | Provides information about the bootstrap actions associated with a cluster. |
| `ListClusters` | - | `paginated` | - | - | `ListClustersOutput` | `InternalServerException`, `InvalidRequestException` | Provides the status of all clusters visible to this Amazon Web Services account. Allows you to filter the list of clusters based on certain criteria; for example, filtering by cluster creation date and time or by status. |
| `ListInstanceFleets` | - | `paginated` | `ClusterId` | - | `ListInstanceFleetsOutput` | `InternalServerException`, `InvalidRequestException` | Lists all available details about the instance fleets in a cluster. The instance fleet configuration is available only in Amazon EMR releases 4.8.0 and later, excluding 5.0.x versions. |
| `ListInstanceGroups` | - | `paginated` | `ClusterId` | - | `ListInstanceGroupsOutput` | `InternalServerException`, `InvalidRequestException` | Provides all available details about the instance groups in a cluster. |
| `ListInstances` | - | `paginated` | `ClusterId` | - | `ListInstancesOutput` | `InternalServerException`, `InvalidRequestException` | Provides information for all active Amazon EC2 instances and Amazon EC2 instances terminated in the last 30 days, up to a maximum of 2,000. Amazon EC2 instances in any of the following states are considered active: AWAITING_FULFILLMENT, PROVISIONING... |
| `ListNotebookExecutions` | - | `paginated` | - | - | `ListNotebookExecutionsOutput` | `InternalServerError`, `InvalidRequestException` | Provides summaries of all notebook executions. You can filter the list based on multiple criteria such as status, time range, and editor id. |
| `ListReleaseLabels` | - | `paginated` | - | - | `ListReleaseLabelsOutput` | `InternalServerException`, `InvalidRequestException` | Retrieves release labels of Amazon EMR services in the Region where the API is called. |
| `ListSecurityConfigurations` | - | `paginated` | - | - | `ListSecurityConfigurationsOutput` | `InternalServerException`, `InvalidRequestException` | Lists all the security configurations visible to this account, providing their creation dates and times, and their names. This call returns a maximum of 50 clusters per call, but returns a marker to track the paging of the cluster list across multiple... |
| `ListSteps` | - | `paginated` | `ClusterId` | - | `ListStepsOutput` | `InternalServerException`, `InvalidRequestException` | Provides a list of steps for the cluster in reverse order unless you specify `stepIds` with the request or filter by `StepStates`. You can specify a maximum of 10 `stepIDs`. |
| `ListStudioSessionMappings` | - | `paginated` | - | - | `ListStudioSessionMappingsOutput` | `InternalServerError`, `InvalidRequestException` | Returns a list of all user or group session mappings for the Amazon EMR Studio specified by `StudioId`. |
| `ListStudios` | - | `paginated` | - | - | `ListStudiosOutput` | `InternalServerException`, `InvalidRequestException` | Returns a list of all Amazon EMR Studios associated with the Amazon Web Services account. The list includes details such as ID, Studio Access URL, and creation time for each Studio. |
| `ListSupportedInstanceTypes` | - | `paginated` | `ReleaseLabel` | - | `ListSupportedInstanceTypesOutput` | `InternalServerException`, `InvalidRequestException` | A list of the instance types that Amazon EMR supports. You can filter the list by Amazon Web Services Region and Amazon EMR release. |
| `ModifyCluster` | - | - | `ClusterId` | - | `ModifyClusterOutput` | `InternalServerError`, `InvalidRequestException` | Modifies the number of steps that can be executed concurrently for the cluster specified using ClusterID. |
| `ModifyInstanceFleet` | - | - | `ClusterId`, `InstanceFleet` | - | `Unit` | `InternalServerException`, `InvalidRequestException` | Modifies the target On-Demand and target Spot capacities for the instance fleet with the specified InstanceFleetID within the cluster specified using ClusterID. The call either succeeds or fails atomically. |
| `ModifyInstanceGroups` | - | - | - | - | `Unit` | `InternalServerError` | ModifyInstanceGroups modifies the number of nodes and configuration settings of an instance group. The input parameters include the new target instance count for the group and the instance group ID. |
| `PutAutoScalingPolicy` | - | - | `AutoScalingPolicy`, `ClusterId`, `InstanceGroupId` | - | `PutAutoScalingPolicyOutput` | - | Creates or updates an automatic scaling policy for a core instance group or task instance group in an Amazon EMR cluster. The automatic scaling policy defines how an instance group dynamically adds and terminates Amazon EC2 instances in response to the value... |
| `PutAutoTerminationPolicy` | - | - | `ClusterId` | - | `PutAutoTerminationPolicyOutput` | - | Auto-termination is supported in Amazon EMR releases 5.30.0 and 6.1.0 and later. For more information, see Using an auto-termination policy. |
| `PutBlockPublicAccessConfiguration` | - | - | `BlockPublicAccessConfiguration` | - | `PutBlockPublicAccessConfigurationOutput` | `InternalServerException`, `InvalidRequestException` | Creates or updates an Amazon EMR block public access configuration for your Amazon Web Services account in the current Region. For more information see Configure Block Public Access for Amazon EMR in the Amazon EMR Management Guide . |
| `PutManagedScalingPolicy` | - | - | `ClusterId`, `ManagedScalingPolicy` | - | `PutManagedScalingPolicyOutput` | - | Creates or updates a managed scaling policy for an Amazon EMR cluster. The managed scaling policy defines the limits for resources, such as Amazon EC2 instances that can be added or terminated from a cluster. |
| `RemoveAutoScalingPolicy` | - | - | `ClusterId`, `InstanceGroupId` | - | `RemoveAutoScalingPolicyOutput` | - | Removes an automatic scaling policy from a specified instance group within an Amazon EMR cluster. |
| `RemoveAutoTerminationPolicy` | - | - | `ClusterId` | - | `RemoveAutoTerminationPolicyOutput` | - | Removes an auto-termination policy from an Amazon EMR cluster. |
| `RemoveManagedScalingPolicy` | - | - | `ClusterId` | - | `RemoveManagedScalingPolicyOutput` | - | Removes a managed scaling policy from a specified Amazon EMR cluster. |
| `RemoveTags` | - | - | `ResourceId`, `TagKeys` | - | `RemoveTagsOutput` | `InternalServerException`, `InvalidRequestException` | Removes tags from an Amazon EMR resource, such as a cluster or Amazon EMR Studio. Tags make it easier to associate resources in various ways, such as grouping clusters to track your Amazon EMR resource allocation costs. |
| `RunJobFlow` | - | - | `Instances`, `Name` | - | `RunJobFlowOutput` | `InternalServerError` | RunJobFlow creates and starts running a new cluster (job flow). The cluster runs the steps specified. |
| `SetKeepJobFlowAliveWhenNoSteps` | - | - | `JobFlowIds`, `KeepJobFlowAliveWhenNoSteps` | - | `Unit` | `InternalServerError` | You can use the `SetKeepJobFlowAliveWhenNoSteps` to configure a cluster (job flow) to terminate after the step execution, i.e., all your steps are executed. If you want a transient cluster that shuts down after the last of the current executing steps are... |
| `SetTerminationProtection` | - | - | `JobFlowIds`, `TerminationProtected` | - | `Unit` | `InternalServerError` | SetTerminationProtection locks a cluster (job flow) so the Amazon EC2 instances in the cluster cannot be terminated by user intervention, an API call, or in the event of a job-flow error. The cluster still terminates upon successful completion of the job flow. |
| `SetUnhealthyNodeReplacement` | - | - | `JobFlowIds`, `UnhealthyNodeReplacement` | - | `Unit` | `InternalServerError` | Specify whether to enable unhealthy node replacement, which lets Amazon EMR gracefully replace core nodes on a cluster if any nodes become unhealthy. For example, a node becomes unhealthy if disk usage is above 90%. |
| `SetVisibleToAllUsers` | - | - | `JobFlowIds`, `VisibleToAllUsers` | - | `Unit` | `InternalServerError` | The SetVisibleToAllUsers parameter is no longer supported. Your cluster may be visible to all users in your account. |
| `StartNotebookExecution` | - | - | `ExecutionEngine`, `ServiceRole` | - | `StartNotebookExecutionOutput` | `InternalServerException`, `InvalidRequestException` | Starts a notebook execution. |
| `StopNotebookExecution` | - | - | `NotebookExecutionId` | - | `Unit` | `InternalServerError`, `InvalidRequestException` | Stops a notebook execution. |
| `TerminateJobFlows` | - | - | `JobFlowIds` | - | `Unit` | `InternalServerError` | TerminateJobFlows shuts a list of clusters (job flows) down. When a job flow is shut down, any step not yet completed is canceled and the Amazon EC2 instances on which the cluster is running are stopped. |
| `UpdateStudio` | - | - | `StudioId` | - | `Unit` | `InternalServerException`, `InvalidRequestException` | Updates an Amazon EMR Studio configuration, including attributes such as name, description, and subnets. |
| `UpdateStudioSessionMapping` | - | - | `IdentityType`, `SessionPolicyArn`, `StudioId` | - | `Unit` | `InternalServerError`, `InvalidRequestException` | Updates the session policy attached to the user or group for the specified Amazon EMR Studio. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidRequestException` | `structure` | `ErrorCode`, `Message` | This exception occurs when there is something wrong with user input. |
| `InternalServerException` | `structure` | `Message` | This exception occurs when there is an internal failure in the Amazon EMR service. |
| `InternalServerError` | `structure` | - | Indicates that an error occurred while processing the request and that the request was not completed. |
| `AddInstanceFleetInput` | `structure` | `ClusterId`, `InstanceFleet` | - |
| `AddInstanceFleetOutput` | `structure` | `ClusterArn`, `ClusterId`, `InstanceFleetId` | - |
| `AddInstanceGroupsInput` | `structure` | `InstanceGroups`, `JobFlowId` | Input to an AddInstanceGroups call. |
| `AddInstanceGroupsOutput` | `structure` | `ClusterArn`, `InstanceGroupIds`, `JobFlowId` | Output from an AddInstanceGroups call. |
| `AddJobFlowStepsInput` | `structure` | `ExecutionRoleArn`, `JobFlowId`, `Steps` | The input argument to the AddJobFlowSteps operation. |
| `AddJobFlowStepsOutput` | `structure` | `StepIds` | The output for the AddJobFlowSteps operation. |
| `AddTagsInput` | `structure` | `ResourceId`, `Tags` | This input identifies an Amazon EMR resource and a list of tags to attach. |
| `AddTagsOutput` | `structure` | - | This output indicates the result of adding tags to a resource. |
| `CancelStepsInput` | `structure` | `ClusterId`, `StepCancellationOption`, `StepIds` | The input argument to the CancelSteps operation. |
| `CancelStepsOutput` | `structure` | `CancelStepsInfoList` | The output for the CancelSteps operation. |
| `CreatePersistentAppUIInput` | `structure` | `EMRContainersConfig`, `ProfilerType`, `Tags`, `TargetResourceArn`, `XReferer` | - |
| `CreatePersistentAppUIOutput` | `structure` | `PersistentAppUIId`, `RuntimeRoleEnabledCluster` | - |
| `CreateSecurityConfigurationInput` | `structure` | `Name`, `SecurityConfiguration` | - |
| `CreateSecurityConfigurationOutput` | `structure` | `CreationDateTime`, `Name` | - |
| `CreateStudioInput` | `structure` | `AuthMode`, `DefaultS3Location`, `Description`, `EncryptionKeyArn`, `EngineSecurityGroupId`, `IdcInstanceArn`, `IdcUserAssignment`, `IdpAuthUrl`, `IdpRelayStateParameterName`, `Name`, `ServiceRole`, `SubnetIds`, ... (+5) | - |
| `CreateStudioOutput` | `structure` | `StudioId`, `Url` | - |
| `CreateStudioSessionMappingInput` | `structure` | `IdentityId`, `IdentityName`, `IdentityType`, `SessionPolicyArn`, `StudioId` | - |
| `DeleteSecurityConfigurationInput` | `structure` | `Name` | - |
| `DeleteSecurityConfigurationOutput` | `structure` | - | - |
| `DeleteStudioInput` | `structure` | `StudioId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
