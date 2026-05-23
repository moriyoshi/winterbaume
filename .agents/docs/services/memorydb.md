# Amazon MemoryDB

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

MemoryDB is a fully managed, Redis OSS-compatible, in-memory database that delivers ultra-fast performance and Multi-AZ durability for modern applications built using microservices architectures. MemoryDB stores the entire database in-memory, enabling low latency and high throughput data access. It is compatible with Redis OSS, a popular open source data store, enabling you to leverage Redis OSS’ flexible and friendly data structures, APIs, and commands.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-memorydb/tests/scenario_test.rs`: create, describe/update, list, and delete a MemoryDB cluster.
- Backported from `scenario_test.rs`: create snapshots around cluster deletion, including final snapshot behaviour.
- Backported from `scenario_test.rs`: chain cluster creation with a subnet group.
- From the AWS documentation and model: model Redis-compatible cluster lifecycle, subnet groups, parameter groups, users/ACLs, snapshots, security groups, maintenance windows, and tag-based inventory.

## Service Identity and Protocol

- AWS model slug: `memorydb`
- AWS SDK for Rust slug: `memorydb`
- Model version: `2021-01-01`
- Model file: `vendor/api-models-aws/models/memorydb/service/2021-01-01/memorydb-2021-01-01.json`
- SDK ID: `MemoryDB`
- Endpoint prefix: `memory-db`
- ARN namespace: `memorydb`
- CloudFormation name: `MemoryDB`
- CloudTrail event source: `memorydb.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (15), `Create` (7), `Delete` (7), `Update` (6), `List` (3), `Batch` (1), `Copy` (1), `Failover` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchUpdateCluster`, `CreateACL`, `CreateCluster`, `CreateMultiRegionCluster`, `CreateParameterGroup`, `CreateSnapshot`, `CreateSubnetGroup`, `CreateUser`, `DeleteACL`, `DeleteCluster`, `DeleteMultiRegionCluster`, `DeleteParameterGroup`, `DeleteSnapshot`, `DeleteSubnetGroup`, `DeleteUser`, `TagResource`, `UntagResource`, `UpdateACL`, `UpdateCluster`, `UpdateMultiRegionCluster`, ... (+3).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeACLs`, `DescribeClusters`, `DescribeEngineVersions`, `DescribeEvents`, `DescribeMultiRegionClusters`, `DescribeMultiRegionParameterGroups`, `DescribeMultiRegionParameters`, `DescribeParameterGroups`, `DescribeParameters`, `DescribeReservedNodes`, `DescribeReservedNodesOfferings`, `DescribeServiceUpdates`, `DescribeSnapshots`, `DescribeSubnetGroups`, `DescribeUsers`, `ListAllowedMultiRegionClusterUpdates`, `ListAllowedNodeTypeUpdates`, `ListTags`.
- Pagination is modelled for 13 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 45 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `KMS`, `EC2/VPC`, `ECS`, `Redshift`.

## Current Network Resource Stub Semantics

MemoryDB currently models subnet groups and security groups inside MemoryDB state.

- `CreateSubnetGroup` stores supplied subnet IDs and mints a synthetic VPC ID from the generated resource identifier rather than deriving it from EC2.
- Clusters store the selected subnet group name and supplied security group IDs as raw strings.
- Subnet group deletion checks local MemoryDB cluster references, but not EC2 subnet or security group state.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Describe

- Operations: `DescribeACLs`, `DescribeClusters`, `DescribeEngineVersions`, `DescribeEvents`, `DescribeMultiRegionClusters`, `DescribeMultiRegionParameterGroups`, `DescribeMultiRegionParameters`, `DescribeParameterGroups`, `DescribeParameters`, `DescribeReservedNodes`, `DescribeReservedNodesOfferings`, `DescribeServiceUpdates`, `DescribeSnapshots`, `DescribeSubnetGroups`, `DescribeUsers`
- Traits: `paginated` (13)
- Common required input members in this group: -

### Create

- Operations: `CreateACL`, `CreateCluster`, `CreateMultiRegionCluster`, `CreateParameterGroup`, `CreateSnapshot`, `CreateSubnetGroup`, `CreateUser`
- Common required input members in this group: `ACLName`, `ClusterName`, `NodeType`

### Delete

- Operations: `DeleteACL`, `DeleteCluster`, `DeleteMultiRegionCluster`, `DeleteParameterGroup`, `DeleteSnapshot`, `DeleteSubnetGroup`, `DeleteUser`
- Common required input members in this group: -

### Update

- Operations: `UpdateACL`, `UpdateCluster`, `UpdateMultiRegionCluster`, `UpdateParameterGroup`, `UpdateSubnetGroup`, `UpdateUser`
- Common required input members in this group: -

### List

- Operations: `ListAllowedMultiRegionClusterUpdates`, `ListAllowedNodeTypeUpdates`, `ListTags`
- Common required input members in this group: -

### Batch

- Operations: `BatchUpdateCluster`
- Common required input members in this group: -

### Copy

- Operations: `CopySnapshot`
- Common required input members in this group: -

### Failover

- Operations: `FailoverShard`
- Common required input members in this group: -

### Purchase

- Operations: `PurchaseReservedNodesOffering`
- Common required input members in this group: -

### Reset

- Operations: `ResetParameterGroup`
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
| `BatchUpdateCluster` | `-` | - | `ClusterNames` | - | `BatchUpdateClusterResponse` | `InvalidParameterValueException`, `ServiceUpdateNotFoundFault` | Apply the service update to a list of clusters supplied. For more information on service updates and applying them, see Applying the service updates . |
| `CopySnapshot` | `-` | - | `SourceSnapshotName`, `TargetSnapshotName` | - | `CopySnapshotResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidSnapshotStateFault`, `ServiceLinkedRoleNotFoundFault`, `SnapshotAlreadyExistsFault`, `SnapshotNotFoundFault`, `SnapshotQuotaExceededFault`, `TagQuotaPerResourceExceeded` | Makes a copy of an existing snapshot. |
| `CreateACL` | `-` | - | `ACLName` | - | `CreateACLResponse` | `ACLAlreadyExistsFault`, `ACLQuotaExceededFault`, `DefaultUserRequired`, `DuplicateUserNameFault`, `InvalidParameterValueException`, `TagQuotaPerResourceExceeded`, `UserNotFoundFault` | Creates an Access Control List. For more information, see Authenticating users with Access Contol Lists (ACLs) . |
| `CreateCluster` | `-` | - | `ClusterName`, `NodeType`, `ACLName` | - | `CreateClusterResponse` | `ACLNotFoundFault`, `ClusterAlreadyExistsFault`, `ClusterQuotaForCustomerExceededFault`, `InsufficientClusterCapacityFault`, `InvalidACLStateFault`, `InvalidCredentialsException`, `InvalidMultiRegionClusterStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidVPCNetworkStateFault`, `MultiRegionClusterNotFoundFault`, `NodeQuotaForClusterExceededFault`, `NodeQuotaForCustomerExceededFault`, `ParameterGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault`, `ShardsPerClusterQuotaExceededFault`, `SubnetGroupNotFoundFault`, `TagQuotaPerResourceExceeded` | Creates a cluster. All nodes in the cluster run the same protocol-compliant engine software. |
| `CreateMultiRegionCluster` | `-` | - | `MultiRegionClusterNameSuffix`, `NodeType` | - | `CreateMultiRegionClusterResponse` | `ClusterQuotaForCustomerExceededFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `MultiRegionClusterAlreadyExistsFault`, `MultiRegionParameterGroupNotFoundFault`, `TagQuotaPerResourceExceeded` | Creates a new multi-Region cluster. |
| `CreateParameterGroup` | `-` | - | `ParameterGroupName`, `Family` | - | `CreateParameterGroupResponse` | `InvalidParameterCombinationException`, `InvalidParameterGroupStateFault`, `InvalidParameterValueException`, `ParameterGroupAlreadyExistsFault`, `ParameterGroupQuotaExceededFault`, `ServiceLinkedRoleNotFoundFault`, `TagQuotaPerResourceExceeded` | Creates a new MemoryDB parameter group. A parameter group is a collection of parameters and their values that are applied to all of the nodes in any cluster. For more information, see Configuring engine parameters us ... |
| `CreateSnapshot` | `-` | - | `ClusterName`, `SnapshotName` | - | `CreateSnapshotResponse` | `ClusterNotFoundFault`, `InvalidClusterStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ServiceLinkedRoleNotFoundFault`, `SnapshotAlreadyExistsFault`, `SnapshotQuotaExceededFault`, `TagQuotaPerResourceExceeded` | Creates a copy of an entire cluster at a specific moment in time. |
| `CreateSubnetGroup` | `-` | - | `SubnetGroupName`, `SubnetIds` | - | `CreateSubnetGroupResponse` | `InvalidSubnet`, `ServiceLinkedRoleNotFoundFault`, `SubnetGroupAlreadyExistsFault`, `SubnetGroupQuotaExceededFault`, `SubnetNotAllowedFault`, `SubnetQuotaExceededFault`, `TagQuotaPerResourceExceeded` | Creates a subnet group. A subnet group is a collection of subnets (typically private) that you can designate for your clusters running in an Amazon Virtual Private Cloud (VPC) environment. When you create a cluster i ... |
| `CreateUser` | `-` | - | `UserName`, `AuthenticationMode`, `AccessString` | - | `CreateUserResponse` | `DuplicateUserNameFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `TagQuotaPerResourceExceeded`, `UserAlreadyExistsFault`, `UserQuotaExceededFault` | Creates a MemoryDB user. For more information, see Authenticating users with Access Contol Lists (ACLs) . |
| `DeleteACL` | `-` | - | `ACLName` | - | `DeleteACLResponse` | `ACLNotFoundFault`, `InvalidACLStateFault`, `InvalidParameterValueException` | Deletes an Access Control List. The ACL must first be disassociated from the cluster before it can be deleted. For more information, see Authenticating users with Access Contol Lists (ACLs) . |
| `DeleteCluster` | `-` | - | `ClusterName` | - | `DeleteClusterResponse` | `ClusterNotFoundFault`, `InvalidClusterStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ServiceLinkedRoleNotFoundFault`, `SnapshotAlreadyExistsFault` | Deletes a cluster. It also deletes all associated nodes and node endpoints. CreateSnapshot permission is required to create a final snapshot. Without this permission, the API call will fail with an Access Denied exce ... |
| `DeleteMultiRegionCluster` | `-` | - | `MultiRegionClusterName` | - | `DeleteMultiRegionClusterResponse` | `InvalidMultiRegionClusterStateFault`, `InvalidParameterValueException`, `MultiRegionClusterNotFoundFault` | Deletes an existing multi-Region cluster. |
| `DeleteParameterGroup` | `-` | - | `ParameterGroupName` | - | `DeleteParameterGroupResponse` | `InvalidParameterCombinationException`, `InvalidParameterGroupStateFault`, `InvalidParameterValueException`, `ParameterGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Deletes the specified parameter group. You cannot delete a parameter group if it is associated with any clusters. You cannot delete the default parameter groups in your account. |
| `DeleteSnapshot` | `-` | - | `SnapshotName` | - | `DeleteSnapshotResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidSnapshotStateFault`, `ServiceLinkedRoleNotFoundFault`, `SnapshotNotFoundFault` | Deletes an existing snapshot. When you receive a successful response from this operation, MemoryDB immediately begins deleting the snapshot; you cannot cancel or revert this operation. |
| `DeleteSubnetGroup` | `-` | - | `SubnetGroupName` | - | `DeleteSubnetGroupResponse` | `ServiceLinkedRoleNotFoundFault`, `SubnetGroupInUseFault`, `SubnetGroupNotFoundFault` | Deletes a subnet group. You cannot delete a default subnet group or one that is associated with any clusters. |
| `DeleteUser` | `-` | - | `UserName` | - | `DeleteUserResponse` | `InvalidParameterValueException`, `InvalidUserStateFault`, `UserNotFoundFault` | Deletes a user. The user will be removed from all ACLs and in turn removed from all clusters. |
| `DescribeACLs` | `-` | `paginated` | - | - | `DescribeACLsResponse` | `ACLNotFoundFault`, `InvalidParameterCombinationException` | Returns a list of ACLs. |
| `DescribeClusters` | `-` | `paginated` | - | - | `DescribeClustersResponse` | `ClusterNotFoundFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ServiceLinkedRoleNotFoundFault` | Returns information about all provisioned clusters if no cluster identifier is specified, or about a specific cluster if a cluster name is supplied. |
| `DescribeEngineVersions` | `-` | `paginated` | - | - | `DescribeEngineVersionsResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ServiceLinkedRoleNotFoundFault` | Returns a list of the available Redis OSS engine versions. |
| `DescribeEvents` | `-` | `paginated` | - | - | `DescribeEventsResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ServiceLinkedRoleNotFoundFault` | Returns events related to clusters, security groups, and parameter groups. You can obtain events specific to a particular cluster, security group, or parameter group by providing the name as a parameter. By default, ... |
| `DescribeMultiRegionClusters` | `-` | `paginated` | - | - | `DescribeMultiRegionClustersResponse` | `ClusterNotFoundFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `MultiRegionClusterNotFoundFault` | Returns details about one or more multi-Region clusters. |
| `DescribeMultiRegionParameterGroups` | `-` | - | - | - | `DescribeMultiRegionParameterGroupsResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `MultiRegionParameterGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Returns a list of multi-region parameter groups. |
| `DescribeMultiRegionParameters` | `-` | - | `MultiRegionParameterGroupName` | - | `DescribeMultiRegionParametersResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `MultiRegionParameterGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Returns the detailed parameter list for a particular multi-region parameter group. |
| `DescribeParameterGroups` | `-` | `paginated` | - | - | `DescribeParameterGroupsResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ParameterGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Returns a list of parameter group descriptions. If a parameter group name is specified, the list contains only the descriptions for that group. |
| `DescribeParameters` | `-` | `paginated` | `ParameterGroupName` | - | `DescribeParametersResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ParameterGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Returns the detailed parameter list for a particular parameter group. |
| `DescribeReservedNodes` | `-` | `paginated` | - | - | `DescribeReservedNodesResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ReservedNodeNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Returns information about reserved nodes for this account, or about a specified reserved node. |
| `DescribeReservedNodesOfferings` | `-` | `paginated` | - | - | `DescribeReservedNodesOfferingsResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ReservedNodesOfferingNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Lists available reserved node offerings. |
| `DescribeServiceUpdates` | `-` | `paginated` | - | - | `DescribeServiceUpdatesResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException` | Returns details of the service updates. |
| `DescribeSnapshots` | `-` | `paginated` | - | - | `DescribeSnapshotsResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ServiceLinkedRoleNotFoundFault`, `SnapshotNotFoundFault` | Returns information about cluster snapshots. By default, DescribeSnapshots lists all of your snapshots; it can optionally describe a single snapshot, or just the snapshots associated with a particular cluster. |
| `DescribeSubnetGroups` | `-` | `paginated` | - | - | `DescribeSubnetGroupsResponse` | `ServiceLinkedRoleNotFoundFault`, `SubnetGroupNotFoundFault` | Returns a list of subnet group descriptions. If a subnet group name is specified, the list contains only the description of that group. |
| `DescribeUsers` | `-` | `paginated` | - | - | `DescribeUsersResponse` | `InvalidParameterCombinationException`, `UserNotFoundFault` | Returns a list of users. |
| `FailoverShard` | `-` | - | `ClusterName`, `ShardName` | - | `FailoverShardResponse` | `APICallRateForCustomerExceededFault`, `ClusterNotFoundFault`, `InvalidClusterStateFault`, `InvalidKMSKeyFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ShardNotFoundFault`, `TestFailoverNotAvailableFault` | Used to failover a shard. This API is designed for testing the behavior of your application in case of MemoryDB failover. It is not designed to be used as a production-level tool for initiating a failover to overcome ... |
| `ListAllowedMultiRegionClusterUpdates` | `-` | - | `MultiRegionClusterName` | - | `ListAllowedMultiRegionClusterUpdatesResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `MultiRegionClusterNotFoundFault` | Lists the allowed updates for a multi-Region cluster. |
| `ListAllowedNodeTypeUpdates` | `-` | - | `ClusterName` | - | `ListAllowedNodeTypeUpdatesResponse` | `ClusterNotFoundFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ServiceLinkedRoleNotFoundFault` | Lists all available node types that you can scale to from your cluster's current node type. When you use the UpdateCluster operation to scale your cluster, the value of the NodeType parameter must be one of the node ... |
| `ListTags` | `-` | - | `ResourceArn` | - | `ListTagsResponse` | `ACLNotFoundFault`, `ClusterNotFoundFault`, `InvalidARNFault`, `InvalidClusterStateFault`, `MultiRegionClusterNotFoundFault`, `MultiRegionParameterGroupNotFoundFault`, `ParameterGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault`, `SnapshotNotFoundFault`, `SubnetGroupNotFoundFault`, `UserNotFoundFault` | Lists all tags currently on a named resource. A tag is a key-value pair where the key and value are case-sensitive. You can use tags to categorize and track your MemoryDB resources. For more information, see Tagging ... |
| `PurchaseReservedNodesOffering` | `-` | - | `ReservedNodesOfferingId` | - | `PurchaseReservedNodesOfferingResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `ReservedNodeAlreadyExistsFault`, `ReservedNodeQuotaExceededFault`, `ReservedNodesOfferingNotFoundFault`, `ServiceLinkedRoleNotFoundFault`, `TagQuotaPerResourceExceeded` | Allows you to purchase a reserved node offering. Reserved nodes are not eligible for cancellation and are non-refundable. |
| `ResetParameterGroup` | `-` | - | `ParameterGroupName` | - | `ResetParameterGroupResponse` | `InvalidParameterCombinationException`, `InvalidParameterGroupStateFault`, `InvalidParameterValueException`, `ParameterGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Modifies the parameters of a parameter group to the engine or system default value. You can reset specific parameters by submitting a list of parameter names. To reset the entire parameter group, specify the AllParam ... |
| `TagResource` | `-` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `ACLNotFoundFault`, `ClusterNotFoundFault`, `InvalidARNFault`, `InvalidClusterStateFault`, `InvalidParameterValueException`, `MultiRegionClusterNotFoundFault`, `MultiRegionParameterGroupNotFoundFault`, `ParameterGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault`, `SnapshotNotFoundFault`, `SubnetGroupNotFoundFault`, `TagQuotaPerResourceExceeded`, `UserNotFoundFault` | Use this operation to add tags to a resource. A tag is a key-value pair where the key and value are case-sensitive. You can use tags to categorize and track all your MemoryDB resources. For more information, see Tagg ... |
| `UntagResource` | `-` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `ACLNotFoundFault`, `ClusterNotFoundFault`, `InvalidARNFault`, `InvalidClusterStateFault`, `InvalidParameterValueException`, `MultiRegionClusterNotFoundFault`, `MultiRegionParameterGroupNotFoundFault`, `ParameterGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault`, `SnapshotNotFoundFault`, `SubnetGroupNotFoundFault`, `TagNotFoundFault`, `UserNotFoundFault` | Use this operation to remove tags on a resource. A tag is a key-value pair where the key and value are case-sensitive. You can use tags to categorize and track all your MemoryDB resources. For more information, see T ... |
| `UpdateACL` | `-` | - | `ACLName` | - | `UpdateACLResponse` | `ACLNotFoundFault`, `DefaultUserRequired`, `DuplicateUserNameFault`, `InvalidACLStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `UserNotFoundFault` | Changes the list of users that belong to the Access Control List. |
| `UpdateCluster` | `-` | - | `ClusterName` | - | `UpdateClusterResponse` | `ACLNotFoundFault`, `ClusterNotFoundFault`, `ClusterQuotaForCustomerExceededFault`, `InvalidACLStateFault`, `InvalidClusterStateFault`, `InvalidKMSKeyFault`, `InvalidNodeStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidVPCNetworkStateFault`, `NodeQuotaForClusterExceededFault`, `NodeQuotaForCustomerExceededFault`, `NoOperationFault`, `ParameterGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault`, `ShardsPerClusterQuotaExceededFault` | Modifies the settings for a cluster. You can use this operation to change one or more cluster configuration settings by specifying the settings and the new values. |
| `UpdateMultiRegionCluster` | `-` | - | `MultiRegionClusterName` | - | `UpdateMultiRegionClusterResponse` | `InvalidMultiRegionClusterStateFault`, `InvalidParameterCombinationException`, `InvalidParameterValueException`, `MultiRegionClusterNotFoundFault`, `MultiRegionParameterGroupNotFoundFault` | Updates the configuration of an existing multi-Region cluster. |
| `UpdateParameterGroup` | `-` | - | `ParameterGroupName`, `ParameterNameValues` | - | `UpdateParameterGroupResponse` | `InvalidParameterCombinationException`, `InvalidParameterGroupStateFault`, `InvalidParameterValueException`, `ParameterGroupNotFoundFault`, `ServiceLinkedRoleNotFoundFault` | Updates the parameters of a parameter group. You can modify up to 20 parameters in a single request by submitting a list parameter name and value pairs. |
| `UpdateSubnetGroup` | `-` | - | `SubnetGroupName` | - | `UpdateSubnetGroupResponse` | `InvalidSubnet`, `ServiceLinkedRoleNotFoundFault`, `SubnetGroupNotFoundFault`, `SubnetInUse`, `SubnetNotAllowedFault`, `SubnetQuotaExceededFault` | Updates a subnet group. For more information, see Updating a subnet group |
| `UpdateUser` | `-` | - | `UserName` | - | `UpdateUserResponse` | `InvalidParameterCombinationException`, `InvalidParameterValueException`, `InvalidUserStateFault`, `UserNotFoundFault` | Changes user password(s) and/or access string. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ACLAlreadyExistsFault` | `structure` | message | An ACL with the specified name already exists. |
| `ACLNotFoundFault` | `structure` | message | The specified ACL does not exist. |
| `ACLQuotaExceededFault` | `structure` | message | The request cannot be processed because it would exceed the maximum number of ACLs allowed. |
| `APICallRateForCustomerExceededFault` | `structure` | message | The customer has exceeded the maximum number of API requests allowed per time period. |
| `ClusterAlreadyExistsFault` | `structure` | message | A cluster with the specified name already exists. |
| `ClusterNotFoundFault` | `structure` | message | The specified cluster does not exist. |
| `ClusterQuotaForCustomerExceededFault` | `structure` | message | The request cannot be processed because it would exceed the maximum number of clusters allowed for this customer. |
| `DefaultUserRequired` | `structure` | message | A default user is required and must be specified. |
| `DuplicateUserNameFault` | `structure` | message | A user with the specified name already exists. |
| `InsufficientClusterCapacityFault` | `structure` | message | The cluster does not have sufficient capacity to perform the requested operation. |
| `InvalidACLStateFault` | `structure` | message | The ACL is not in a valid state for the requested operation. |
| `InvalidARNFault` | `structure` | message | The specified Amazon Resource Name (ARN) is not valid. |
| `InvalidClusterStateFault` | `structure` | message | The cluster is not in a valid state for the requested operation. |
| `InvalidCredentialsException` | `structure` | message | The provided credentials are not valid. |
| `InvalidKMSKeyFault` | `structure` | message | The specified KMS key is not valid or accessible. |
| `InvalidMultiRegionClusterStateFault` | `structure` | message | The requested operation cannot be performed on the multi-Region cluster in its current state. |
| `InvalidNodeStateFault` | `structure` | message | The node is not in a valid state for the requested operation. |
| `InvalidParameterCombinationException` | `structure` | message | The specified parameter combination is not valid. |
| `InvalidParameterGroupStateFault` | `structure` | message | The parameter group is not in a valid state for the requested operation. |
| `InvalidParameterValueException` | `structure` | message | The specified parameter value is not valid. |
| `InvalidSnapshotStateFault` | `structure` | message | The snapshot is not in a valid state for the requested operation. |
| `InvalidSubnet` | `structure` | message | The specified subnet is not valid. |
| `InvalidUserStateFault` | `structure` | message | The user is not in a valid state for the requested operation. |
| `InvalidVPCNetworkStateFault` | `structure` | message | The VPC network is not in a valid state for the requested operation. |
| `MultiRegionClusterAlreadyExistsFault` | `structure` | message | A multi-Region cluster with the specified name already exists. |
| `MultiRegionClusterNotFoundFault` | `structure` | message | The specified multi-Region cluster does not exist. |
| `MultiRegionParameterGroupNotFoundFault` | `structure` | message | The specified multi-Region parameter group does not exist. |
| `NoOperationFault` | `structure` | message | The requested operation would result in no changes. |
| `NodeQuotaForClusterExceededFault` | `structure` | message | The request cannot be processed because it would exceed the maximum number of nodes allowed for this cluster. |
| `NodeQuotaForCustomerExceededFault` | `structure` | message | The request cannot be processed because it would exceed the maximum number of nodes allowed for this customer. |
| `ParameterGroupAlreadyExistsFault` | `structure` | message | A parameter group with the specified name already exists. |
| `ParameterGroupNotFoundFault` | `structure` | message | The specified parameter group does not exist. |
| `ParameterGroupQuotaExceededFault` | `structure` | message | The request cannot be processed because it would exceed the maximum number of parameter groups allowed. |
| `ReservedNodeAlreadyExistsFault` | `structure` | message | You already have a reservation with the given identifier. |
| `ReservedNodeNotFoundFault` | `structure` | message | The requested node does not exist. |
| `ReservedNodeQuotaExceededFault` | `structure` | message | The request cannot be processed because it would exceed the user's node quota. |
| `ReservedNodesOfferingNotFoundFault` | `structure` | message | The requested node offering does not exist. |
| `ServiceLinkedRoleNotFoundFault` | `structure` | message | The required service-linked role was not found. |
| `ServiceUpdateNotFoundFault` | `structure` | message | The specified service update does not exist. |
| `ShardNotFoundFault` | `structure` | message | The specified shard does not exist. |
| `ShardsPerClusterQuotaExceededFault` | `structure` | message | The request cannot be processed because it would exceed the maximum number of shards allowed per cluster. |
| `SnapshotAlreadyExistsFault` | `structure` | message | A snapshot with the specified name already exists. |
| `SnapshotNotFoundFault` | `structure` | message | The specified snapshot does not exist. |
| `SnapshotQuotaExceededFault` | `structure` | message | The request cannot be processed because it would exceed the maximum number of snapshots allowed. |
| `SubnetGroupAlreadyExistsFault` | `structure` | message | A subnet group with the specified name already exists. |
| `SubnetGroupInUseFault` | `structure` | message | The subnet group is currently in use and cannot be deleted. |
| `SubnetGroupNotFoundFault` | `structure` | message | The specified subnet group does not exist. |
| `SubnetGroupQuotaExceededFault` | `structure` | message | The request cannot be processed because it would exceed the maximum number of subnet groups allowed. |
| `SubnetInUse` | `structure` | message | The subnet is currently in use and cannot be deleted. |
| `SubnetNotAllowedFault` | `structure` | message | The specified subnet is not allowed for this operation. |
| `SubnetQuotaExceededFault` | `structure` | message | The request cannot be processed because it would exceed the maximum number of subnets allowed. |
| `TagNotFoundFault` | `structure` | message | The specified tag does not exist. |
| `TagQuotaPerResourceExceeded` | `structure` | message | The request cannot be processed because it would exceed the maximum number of tags allowed per resource. |
| `TestFailoverNotAvailableFault` | `structure` | message | Test failover is not available for this cluster configuration. |
| `UserAlreadyExistsFault` | `structure` | message | A user with the specified name already exists. |
| `UserNotFoundFault` | `structure` | message | The specified user does not exist. |
| `UserQuotaExceededFault` | `structure` | message | The request cannot be processed because it would exceed the maximum number of users allowed. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
