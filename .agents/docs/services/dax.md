# Amazon DynamoDB Accelerator (DAX)

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

DAX is a managed caching service engineered for Amazon DynamoDB. DAX dramatically speeds up database reads by caching frequently-accessed data from DynamoDB, so applications can access that data with sub-millisecond latency. You can create a DAX cluster easily, using the Amazon Web Services Management Console. With a few simple modifications to your code, your application can begin taking advantage of the DAX cluster and realize significant improvements in read performance.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for Amazon DynamoDB Accelerator (DAX) by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for Amazon DynamoDB Accelerator (DAX) resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon DynamoDB Accelerator (DAX) workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Create`, `Delete`, `Update`, `Decrease` operation families, including `DescribeClusters`, `DescribeDefaultParameters`, `DescribeEvents`, `DescribeParameterGroups`, `CreateCluster`, `CreateParameterGroup`.

## Service Identity and Protocol

- AWS model slug: `dax`
- AWS SDK for Rust slug: `dax`
- Model version: `2017-04-19`
- Model file: `vendor/api-models-aws/models/dax/service/2017-04-19/dax-2017-04-19.json`
- SDK ID: `DAX`
- Endpoint prefix: `dax`
- ARN namespace: `dax`
- CloudFormation name: `DAX`
- CloudTrail event source: `dax.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (6), `Create` (3), `Delete` (3), `Update` (3), `Decrease` (1), `Increase` (1), `List` (1), `Reboot` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateCluster`, `CreateParameterGroup`, `CreateSubnetGroup`, `DeleteCluster`, `DeleteParameterGroup`, `DeleteSubnetGroup`, `TagResource`, `UntagResource`, `UpdateCluster`, `UpdateParameterGroup`, `UpdateSubnetGroup`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeClusters`, `DescribeDefaultParameters`, `DescribeEvents`, `DescribeParameterGroups`, `DescribeParameters`, `DescribeSubnetGroups`, `ListTags`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 21 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SNS`, `EC2/VPC`, `ECS`, `Redshift`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DAX.html
- https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DAX.concepts.html
- https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/DAX.consistency.html

Research outcomes:
- DynamoDB Accelerator (DAX) is an in-memory cache for DynamoDB that provides microsecond latency for read-heavy applications.
- DAX clusters contain nodes and expose endpoints used by DAX clients.
- DAX has an item cache and query cache with TTL-based expiry.
- DAX uses write-through behaviour for supported writes to keep the item cache consistent with writes through DAX.
- Query cache entries operate independently from item cache entries and can be stale until TTL expiry or invalidation.
- Strongly consistent reads and transactional reads are passed through to DynamoDB and are not served from DAX cache.
- DAX can cache negative lookups, so missing-item results can be cached for a period.
- Clusters use subnet groups, parameter groups, security groups, encryption settings, and VPC networking.

Parity implications:
- Model clusters, nodes, endpoints, subnet groups, parameter groups, item cache, query cache, TTLs, encryption, and security groups separately.
- Read/write request behaviour should distinguish cacheable eventually consistent reads from pass-through strong/transactional operations.
- Cache invalidation and stale query-cache semantics matter for behavioural parity.

## Current Network Resource Stub Semantics

DAX currently has partial placeholders for subnet groups and security groups.

- Subnet group shapes and views include subnet IDs and an optional VPC ID, but `CreateSubnetGroup`, `DescribeSubnetGroups`, `UpdateSubnetGroup`, and `DeleteSubnetGroup` return `501 NotImplemented`.
- `CreateCluster` returns local cluster data using the requested subnet group name when present, but the mocked response falls back to default networking such as `default` subnet group and a fixed `sg-00000001` style security group.
- Cluster state is not tied to EC2 subnet membership or security group existence.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Describe

- Operations: `DescribeClusters`, `DescribeDefaultParameters`, `DescribeEvents`, `DescribeParameterGroups`, `DescribeParameters`, `DescribeSubnetGroups`
- Common required input members in this group: -

### Create

- Operations: `CreateCluster`, `CreateParameterGroup`, `CreateSubnetGroup`
- Common required input members in this group: -

### Delete

- Operations: `DeleteCluster`, `DeleteParameterGroup`, `DeleteSubnetGroup`
- Common required input members in this group: -

### Update

- Operations: `UpdateCluster`, `UpdateParameterGroup`, `UpdateSubnetGroup`
- Common required input members in this group: -

### Decrease

- Operations: `DecreaseReplicationFactor`
- Common required input members in this group: -

### Increase

- Operations: `IncreaseReplicationFactor`
- Common required input members in this group: -

### List

- Operations: `ListTags`
- Common required input members in this group: -

### Reboot

- Operations: `RebootNode`
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
| `CreateCluster` | `-` | - | `ClusterName`, `NodeType`, `ReplicationFactor`, `IamRoleArn` | - | `CreateClusterResponse` | `ClusterAlreadyExistsFault`, `ClusterQuotaForCustomerExceededFault`, `InsufficientClusterCapacityFault`, `InvalidClusterStateFault`, `InvalidParameterCombinationException`, `InvalidParameterGroupStateFault`, `InvalidParameterValueException`, `InvalidVPCNetworkStateFault`, `NodeQuotaForClusterExceededFault`, `NodeQuotaForCustomerExceededFault`, `ParameterGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault`, `ServiceQuotaExceededException`, `SubnetGroupNotFoundFault`, `TagQuotaPerResourceExceeded` | Creates a DAX cluster. All nodes in the cluster run the same DAX caching software. |
| `CreateParameterGroup` | `-` | - | `ParameterGroupName` | - | `CreateParameterGroupResponse` | `InvalidParameterCombinationException`, `InvalidParameterGroupStateFault`, `InvalidParameterValueException`, `ParameterGroupAlreadyExistsFault`, `ParameterGroupQuotaExceededFault`, `ServiceLinkedRoleNotFoundFault` | Creates a new parameter group. A parameter group is a collection of parameters that you apply to all of the nodes in a DAX cluster. |
| `CreateSubnetGroup` | `-` | - | `SubnetGroupName`, `SubnetIds` | - | `CreateSubnetGroupResponse` | `InvalidSubnet`, `ServiceLinkedRoleNotFoundFault`, `SubnetGroupAlreadyExistsFault`, `SubnetGroupQuotaExceededFault`, `SubnetNotAllowedFault`, `SubnetQuotaExceededFault` | Creates a new subnet group. |
| `DecreaseReplicationFactor` | `-` | - | `ClusterName`, `NewReplicationFactor` | - | `DecreaseReplicationFactorResponse` | `ClusterNotFoundFault`, `InvalidClusterStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `NodeNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Removes one or more nodes from a DAX cluster. You cannot use DecreaseReplicationFactor to remove the last node in a DAX cluster. If you need to do this, use DeleteCluster instead. |
| `DeleteCluster` | `-` | - | `ClusterName` | - | `DeleteClusterResponse` | `ClusterNotFoundFault`, `InvalidClusterStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ServiceLinkedRoleNotFoundFault` | Deletes a previously provisioned DAX cluster. DeleteCluster deletes all associated nodes, node endpoints and the DAX cluster itself. When you receive a successful response from this action, DAX immediately begins del ... |
| `DeleteParameterGroup` | `-` | - | `ParameterGroupName` | - | `DeleteParameterGroupResponse` | `InvalidParameterCombinationException`, `InvalidParameterGroupStateFault`, `InvalidParameterValueException`, `ParameterGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Deletes the specified parameter group. You cannot delete a parameter group if it is associated with any DAX clusters. |
| `DeleteSubnetGroup` | `-` | - | `SubnetGroupName` | - | `DeleteSubnetGroupResponse` | `ServiceLinkedRoleNotFoundFault`, `SubnetGroupInUseFault`, `SubnetGroupNotFoundFault` | Deletes a subnet group. You cannot delete a subnet group if it is associated with any DAX clusters. |
| `DescribeClusters` | `-` | - | - | - | `DescribeClustersResponse` | `ClusterNotFoundFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ServiceLinkedRoleNotFoundFault` | Returns information about all provisioned DAX clusters if no cluster identifier is specified, or about a specific DAX cluster if a cluster identifier is supplied. If the cluster is in the CREATING state, only cluster ... |
| `DescribeDefaultParameters` | `-` | - | - | - | `DescribeDefaultParametersResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ServiceLinkedRoleNotFoundFault` | Returns the default system parameter information for the DAX caching software. |
| `DescribeEvents` | `-` | - | - | - | `DescribeEventsResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ServiceLinkedRoleNotFoundFault` | Returns events related to DAX clusters and parameter groups. You can obtain events specific to a particular DAX cluster or parameter group by providing the name as a parameter. By default, only the events occurring w ... |
| `DescribeParameterGroups` | `-` | - | - | - | `DescribeParameterGroupsResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ParameterGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Returns a list of parameter group descriptions. If a parameter group name is specified, the list will contain only the descriptions for that group. |
| `DescribeParameters` | `-` | - | `ParameterGroupName` | - | `DescribeParametersResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ParameterGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Returns the detailed parameter list for a particular parameter group. |
| `DescribeSubnetGroups` | `-` | - | - | - | `DescribeSubnetGroupsResponse` | `ServiceLinkedRoleNotFoundFault`, `SubnetGroupNotFoundFault` | Returns a list of subnet group descriptions. If a subnet group name is specified, the list will contain only the description of that group. |
| `IncreaseReplicationFactor` | `-` | - | `ClusterName`, `NewReplicationFactor` | - | `IncreaseReplicationFactorResponse` | `ClusterNotFoundFault`, `InsufficientClusterCapacityFault`, `InvalidClusterStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidVPCNetworkStateFault`, `NodeQuotaForClusterExceededFault`, `NodeQuotaForCustomerExceededFault`, `ServiceLinkedRoleNotFoundFault` | Adds one or more nodes to a DAX cluster. |
| `ListTags` | `-` | - | `ResourceName` | - | `ListTagsResponse` | `ClusterNotFoundFault`, `InvalidARNFault`, `InvalidClusterStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ServiceLinkedRoleNotFoundFault` | List all of the tags for a DAX cluster. You can call ListTags up to 10 times per second, per account. |
| `RebootNode` | `-` | - | `ClusterName`, `NodeId` | - | `RebootNodeResponse` | `ClusterNotFoundFault`, `InvalidClusterStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `NodeNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Reboots a single node of a DAX cluster. The reboot action takes place as soon as possible. During the reboot, the node status is set to REBOOTING. RebootNode restarts the DAX engine process and does not remove the co ... |
| `TagResource` | `-` | - | `ResourceName`, `Tags` | - | `TagResourceResponse` | `ClusterNotFoundFault`, `InvalidARNFault`, `InvalidClusterStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ServiceLinkedRoleNotFoundFault`, `TagQuotaPerResourceExceeded` | Associates a set of tags with a DAX resource. You can call TagResource up to 5 times per second, per account. |
| `UntagResource` | `-` | - | `ResourceName`, `TagKeys` | - | `UntagResourceResponse` | `ClusterNotFoundFault`, `InvalidARNFault`, `InvalidClusterStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ServiceLinkedRoleNotFoundFault`, `TagNotFoundFault` | Removes the association of tags from a DAX resource. You can call UntagResource up to 5 times per second, per account. |
| `UpdateCluster` | `-` | - | `ClusterName` | - | `UpdateClusterResponse` | `ClusterNotFoundFault`, `InvalidClusterStateFault`, `InvalidParameterCombinationException`, `InvalidParameterGroupStateFault`, `InvalidParameterValueException`, `ParameterGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Modifies the settings for a DAX cluster. You can use this action to change one or more cluster configuration parameters by specifying the parameters and the new values. |
| `UpdateParameterGroup` | `-` | - | `ParameterGroupName`, `ParameterNameValues` | - | `UpdateParameterGroupResponse` | `InvalidParameterCombinationException`, `InvalidParameterGroupStateFault`, `InvalidParameterValueException`, `ParameterGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Modifies the parameters of a parameter group. You can modify up to 20 parameters in a single request by submitting a list parameter name and value pairs. |
| `UpdateSubnetGroup` | `-` | - | `SubnetGroupName` | - | `UpdateSubnetGroupResponse` | `InvalidSubnet`, `ServiceLinkedRoleNotFoundFault`, `SubnetGroupNotFoundFault`, `SubnetInUse`, `SubnetNotAllowedFault`, `SubnetQuotaExceededFault` | Modifies an existing subnet group. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ClusterAlreadyExistsFault` | `structure` | message | You already have a DAX cluster with the given identifier. |
| `ClusterNotFoundFault` | `structure` | message | The requested cluster ID does not refer to an existing DAX cluster. |
| `ClusterQuotaForCustomerExceededFault` | `structure` | message | You have attempted to exceed the maximum number of DAX clusters for your Amazon Web Services account. |
| `InsufficientClusterCapacityFault` | `structure` | message | There are not enough system resources to create the cluster you requested (or to resize an already-existing cluster). |
| `InvalidARNFault` | `structure` | message | The Amazon Resource Name (ARN) supplied in the request is not valid. |
| `InvalidClusterStateFault` | `structure` | message | The requested DAX cluster is not in the available state. |
| `InvalidParameterCombinationException` | `structure` | message | Two or more incompatible parameters were specified. |
| `InvalidParameterGroupStateFault` | `structure` | message | One or more parameters in a parameter group are in an invalid state. |
| `InvalidParameterValueException` | `structure` | message | The value for a parameter is invalid. |
| `InvalidSubnet` | `structure` | message | An invalid subnet identifier was specified. |
| `InvalidVPCNetworkStateFault` | `structure` | message | The VPC network is in an invalid state. |
| `NodeNotFoundFault` | `structure` | message | None of the nodes in the cluster have the given node ID. |
| `NodeQuotaForClusterExceededFault` | `structure` | message | You have attempted to exceed the maximum number of nodes for a DAX cluster. |
| `NodeQuotaForCustomerExceededFault` | `structure` | message | You have attempted to exceed the maximum number of nodes for your Amazon Web Services account. |
| `ParameterGroupAlreadyExistsFault` | `structure` | message | The specified parameter group already exists. |
| `ParameterGroupNotFoundFault` | `structure` | message | The specified parameter group does not exist. |
| `ParameterGroupQuotaExceededFault` | `structure` | message | You have attempted to exceed the maximum number of parameter groups. |
| `ServiceLinkedRoleNotFoundFault` | `structure` | message | The specified service linked role (SLR) was not found. |
| `ServiceQuotaExceededException` | `structure` | **empty (no members)** | You have reached the maximum number of x509 certificates that can be created for encrypted clusters in a 30 day period. Contact Amazon Web Services customer ... |
| `SubnetGroupAlreadyExistsFault` | `structure` | message | The specified subnet group already exists. |
| `SubnetGroupInUseFault` | `structure` | message | The specified subnet group is currently in use. |
| `SubnetGroupNotFoundFault` | `structure` | message | The requested subnet group name does not refer to an existing subnet group. |
| `SubnetGroupQuotaExceededFault` | `structure` | message | The request cannot be processed because it would exceed the allowed number of subnets in a subnet group. |
| `SubnetInUse` | `structure` | message | The requested subnet is being used by another subnet group. |
| `SubnetNotAllowedFault` | `structure` | message | The specified subnet can't be used for the requested network type. This error occurs when either there aren't enough subnets of the required network type to ... |
| `SubnetQuotaExceededFault` | `structure` | message | The request cannot be processed because it would exceed the allowed number of subnets in a subnet group. |
| `TagNotFoundFault` | `structure` | message | The tag does not exist. |
| `TagQuotaPerResourceExceeded` | `structure` | message | You have exceeded the maximum number of tags for this DAX cluster. |
| `CreateClusterRequest` | `structure` | ClusterName, NodeType, Description, ReplicationFactor, AvailabilityZones, SubnetGroupName, SecurityGroupIds, PreferredMaintenanceWindow, NotificationTopicArn, IamRoleArn, ParameterGroupName, Tags, ... (+3) | - |
| `CreateClusterResponse` | `structure` | Cluster | - |
| `CreateParameterGroupRequest` | `structure` | ParameterGroupName, Description | - |
| `CreateParameterGroupResponse` | `structure` | ParameterGroup | - |
| `CreateSubnetGroupRequest` | `structure` | SubnetGroupName, Description, SubnetIds | - |
| `CreateSubnetGroupResponse` | `structure` | SubnetGroup | - |
| `DecreaseReplicationFactorRequest` | `structure` | ClusterName, NewReplicationFactor, AvailabilityZones, NodeIdsToRemove | - |
| `DecreaseReplicationFactorResponse` | `structure` | Cluster | - |
| `DeleteClusterRequest` | `structure` | ClusterName | - |
| `DeleteClusterResponse` | `structure` | Cluster | - |
| `DeleteParameterGroupRequest` | `structure` | ParameterGroupName | - |
| `DeleteParameterGroupResponse` | `structure` | DeletionMessage | - |
| `ChangeType` | `enum` | IMMEDIATE, REQUIRES_REBOOT | - |
| `ClusterEndpointEncryptionType` | `enum` | NONE, TLS | - |
| `IsModifiable` | `enum` | TRUE, FALSE, CONDITIONAL | - |
| `NetworkType` | `enum` | IPV4, IPV6, DUAL_STACK | - |
| `ParameterType` | `enum` | DEFAULT, NODE_TYPE_SPECIFIC | - |
| `SSEStatus` | `enum` | ENABLING, ENABLED, DISABLING, DISABLED | - |
| `SourceType` | `enum` | CLUSTER, PARAMETER_GROUP, SUBNET_GROUP | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
