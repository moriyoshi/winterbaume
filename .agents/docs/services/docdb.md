# Amazon DocumentDB with MongoDB compatibility

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon DocumentDB is a fast, reliable, and fully managed database service. Amazon DocumentDB makes it easy to set up, operate, and scale MongoDB-compatible databases in the cloud. With Amazon DocumentDB, you can run the same application code and use the same drivers and tools that you use with MongoDB.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon DocumentDB with MongoDB compatibility where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon DocumentDB with MongoDB compatibility by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for Amazon DocumentDB with MongoDB compatibility resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon DocumentDB with MongoDB compatibility workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Create`, `Delete`, `Modify`, `Remove` operation families, including `DescribeCertificates`, `DescribeDBClusterParameterGroups`, `DescribeDBClusterParameters`, `DescribeDBClusterSnapshotAttributes`, `CreateDBCluster`, `CreateDBClusterParameterGroup`.

## Service Identity and Protocol

- AWS model slug: `docdb`
- AWS SDK for Rust slug: `rds`
- Model version: `2014-10-31`
- Model file: `vendor/api-models-aws/models/docdb/service/2014-10-31/docdb-2014-10-31.json`
- SDK ID: `DocDB`
- Endpoint prefix: `rds`
- ARN namespace: `rds`
- CloudFormation name: `DocDB`
- CloudTrail event source: `docdb.amazonaws.com`
- Protocols: `awsQuery`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (16), `Create` (7), `Delete` (7), `Modify` (7), `Remove` (3), `Add` (2), `Copy` (2), `Failover` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddSourceIdentifierToSubscription`, `AddTagsToResource`, `CreateDBCluster`, `CreateDBClusterParameterGroup`, `CreateDBClusterSnapshot`, `CreateDBInstance`, `CreateDBSubnetGroup`, `CreateEventSubscription`, `CreateGlobalCluster`, `DeleteDBCluster`, `DeleteDBClusterParameterGroup`, `DeleteDBClusterSnapshot`, `DeleteDBInstance`, `DeleteDBSubnetGroup`, `DeleteEventSubscription`, `DeleteGlobalCluster`, `ModifyDBCluster`, `ModifyDBClusterParameterGroup`, `ModifyDBClusterSnapshotAttribute`, `ModifyDBInstance`, ... (+10).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeCertificates`, `DescribeDBClusterParameterGroups`, `DescribeDBClusterParameters`, `DescribeDBClusterSnapshotAttributes`, `DescribeDBClusterSnapshots`, `DescribeDBClusters`, `DescribeDBEngineVersions`, `DescribeDBInstances`, `DescribeDBSubnetGroups`, `DescribeEngineDefaultClusterParameters`, `DescribeEventCategories`, `DescribeEventSubscriptions`, `DescribeEvents`, `DescribeGlobalClusters`, `DescribeOrderableDBInstanceOptions`, `DescribePendingMaintenanceActions`, `ListTagsForResource`.
- Pagination is modelled for 13 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartDBCluster`, `StopDBCluster`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 50 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `SNS`, `EC2/VPC`, `ECS`, `RDS`, `Redshift`, `Secrets Manager`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/documentdb/latest/developerguide/db-clusters-understanding.html
- https://docs.aws.amazon.com/documentdb/latest/developerguide/replication.html
- https://docs.aws.amazon.com/documentdb/latest/developerguide/how-it-works.html

Research outcomes:
- Amazon DocumentDB separates compute from storage. A cluster consists of one or more instances plus a shared cluster storage volume.
- The cluster volume stores six copies of data across three Availability Zones and presents one logical volume to all instances.
- A cluster always has one primary instance and can have up to 15 replica instances.
- Primary instances handle reads and writes. Replica instances are read-only, dedicated to read scaling, and act as failover targets.
- Replicas are eventually consistent and typically lag the primary by less than 100 ms, though lag can increase with high write rates or heavy replica queries.
- DocumentDB distributes instances across Availability Zones in the subnet group to balance availability.
- On primary failure, a replica can be promoted to primary. The old primary returns as a replica in its original Availability Zone after recovery.
- If no replica exists, the primary is recreated, which is slower than promoting a replica.
- Cluster endpoints connect to the current primary. Reader endpoints load balance read-only connections across replicas, and instance endpoints connect to a specific instance.
- In replica set mode with the cluster endpoint, clients can use read concern, write concern, and read preference. The replica set name is fixed as `rs0`.
- Storage auto-repair, survivable cache warming, and asynchronous crash recovery are service behaviours, and memory pressure can throttle or queue operations.

Parity implications:
- Model clusters, instances, shared cluster volume, primary/replica roles, endpoints, subnet groups, failover priority, snapshots/backups, parameter groups, and CloudWatch-facing state separately.
- Failover should update instance roles while preserving instance Availability Zones and endpoint semantics.
- Reader endpoint behaviour should distinguish connection balancing from per-query balancing and handle primary-only clusters.

## Operation Groups

### Describe

- Operations: `DescribeCertificates`, `DescribeDBClusterParameterGroups`, `DescribeDBClusterParameters`, `DescribeDBClusterSnapshotAttributes`, `DescribeDBClusterSnapshots`, `DescribeDBClusters`, `DescribeDBEngineVersions`, `DescribeDBInstances`, `DescribeDBSubnetGroups`, `DescribeEngineDefaultClusterParameters`, `DescribeEventCategories`, `DescribeEventSubscriptions`, `DescribeEvents`, `DescribeGlobalClusters`, `DescribeOrderableDBInstanceOptions`, `DescribePendingMaintenanceActions`
- Traits: `paginated` (13)
- Common required input members in this group: `DBClusterParameterGroupName`, `DBClusterSnapshotIdentifier`, `DBParameterGroupFamily`, `Engine`

### Create

- Operations: `CreateDBCluster`, `CreateDBClusterParameterGroup`, `CreateDBClusterSnapshot`, `CreateDBInstance`, `CreateDBSubnetGroup`, `CreateEventSubscription`, `CreateGlobalCluster`
- Common required input members in this group: `DBClusterIdentifier`, `DBClusterParameterGroupName`, `DBClusterSnapshotIdentifier`, `DBInstanceClass`, `DBInstanceIdentifier`, `DBParameterGroupFamily`, `DBSubnetGroupDescription`, `DBSubnetGroupName`, `Description`, `Engine`, `GlobalClusterIdentifier`, `SnsTopicArn`, `SubnetIds`, `SubscriptionName`

### Delete

- Operations: `DeleteDBCluster`, `DeleteDBClusterParameterGroup`, `DeleteDBClusterSnapshot`, `DeleteDBInstance`, `DeleteDBSubnetGroup`, `DeleteEventSubscription`, `DeleteGlobalCluster`
- Common required input members in this group: `DBClusterIdentifier`, `DBClusterParameterGroupName`, `DBClusterSnapshotIdentifier`, `DBInstanceIdentifier`, `DBSubnetGroupName`, `GlobalClusterIdentifier`, `SubscriptionName`

### Modify

- Operations: `ModifyDBCluster`, `ModifyDBClusterParameterGroup`, `ModifyDBClusterSnapshotAttribute`, `ModifyDBInstance`, `ModifyDBSubnetGroup`, `ModifyEventSubscription`, `ModifyGlobalCluster`
- Common required input members in this group: `AttributeName`, `DBClusterIdentifier`, `DBClusterParameterGroupName`, `DBClusterSnapshotIdentifier`, `DBInstanceIdentifier`, `DBSubnetGroupName`, `GlobalClusterIdentifier`, `Parameters`, `SubnetIds`, `SubscriptionName`

### Remove

- Operations: `RemoveFromGlobalCluster`, `RemoveSourceIdentifierFromSubscription`, `RemoveTagsFromResource`
- Common required input members in this group: `DbClusterIdentifier`, `GlobalClusterIdentifier`, `ResourceName`, `SourceIdentifier`, `SubscriptionName`, `TagKeys`

### Add

- Operations: `AddSourceIdentifierToSubscription`, `AddTagsToResource`
- Common required input members in this group: `ResourceName`, `SourceIdentifier`, `SubscriptionName`, `Tags`

### Copy

- Operations: `CopyDBClusterParameterGroup`, `CopyDBClusterSnapshot`
- Common required input members in this group: `SourceDBClusterParameterGroupIdentifier`, `SourceDBClusterSnapshotIdentifier`, `TargetDBClusterParameterGroupDescription`, `TargetDBClusterParameterGroupIdentifier`, `TargetDBClusterSnapshotIdentifier`

### Failover

- Operations: `FailoverDBCluster`, `FailoverGlobalCluster`
- Common required input members in this group: `GlobalClusterIdentifier`, `TargetDbClusterIdentifier`

### Restore

- Operations: `RestoreDBClusterFromSnapshot`, `RestoreDBClusterToPointInTime`
- Common required input members in this group: `DBClusterIdentifier`, `Engine`, `SnapshotIdentifier`, `SourceDBClusterIdentifier`

### Apply

- Operations: `ApplyPendingMaintenanceAction`
- Common required input members in this group: `ApplyAction`, `OptInType`, `ResourceIdentifier`

### List

- Operations: `ListTagsForResource`
- Common required input members in this group: `ResourceName`

### Reboot

- Operations: `RebootDBInstance`
- Common required input members in this group: `DBInstanceIdentifier`

### Reset

- Operations: `ResetDBClusterParameterGroup`
- Common required input members in this group: `DBClusterParameterGroupName`

### Start

- Operations: `StartDBCluster`
- Common required input members in this group: `DBClusterIdentifier`

### Stop

- Operations: `StopDBCluster`
- Common required input members in this group: `DBClusterIdentifier`

### Switchover

- Operations: `SwitchoverGlobalCluster`
- Common required input members in this group: `GlobalClusterIdentifier`, `TargetDbClusterIdentifier`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddSourceIdentifierToSubscription` | - | - | `SourceIdentifier`, `SubscriptionName` | - | `AddSourceIdentifierToSubscriptionResult` | `SourceNotFoundFault`, `SubscriptionNotFoundFault` | Adds a source identifier to an existing event notification subscription. |
| `AddTagsToResource` | - | - | `ResourceName`, `Tags` | - | `Unit` | `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `DBSnapshotNotFoundFault` | Adds metadata tags to an Amazon DocumentDB resource. You can use these tags with cost allocation reporting to track costs that are associated with Amazon DocumentDB resources or in a `Condition` statement in an Identity and Access Management (IAM) policy for... |
| `ApplyPendingMaintenanceAction` | - | - | `ApplyAction`, `OptInType`, `ResourceIdentifier` | - | `ApplyPendingMaintenanceActionResult` | `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `ResourceNotFoundFault` | Applies a pending maintenance action to a resource (for example, to an Amazon DocumentDB instance). |
| `CopyDBClusterParameterGroup` | - | - | `SourceDBClusterParameterGroupIdentifier`, `TargetDBClusterParameterGroupDescription`, `TargetDBClusterParameterGroupIdentifier` | - | `CopyDBClusterParameterGroupResult` | `DBParameterGroupAlreadyExistsFault`, `DBParameterGroupNotFoundFault`, `DBParameterGroupQuotaExceededFault` | Copies the specified cluster parameter group. |
| `CopyDBClusterSnapshot` | - | - | `SourceDBClusterSnapshotIdentifier`, `TargetDBClusterSnapshotIdentifier` | - | `CopyDBClusterSnapshotResult` | `DBClusterSnapshotAlreadyExistsFault`, `DBClusterSnapshotNotFoundFault`, `InvalidDBClusterSnapshotStateFault`, `InvalidDBClusterStateFault`, `KMSKeyNotAccessibleFault`, `SnapshotQuotaExceededFault` | Copies a snapshot of a cluster. To copy a cluster snapshot from a shared manual cluster snapshot, `SourceDBClusterSnapshotIdentifier` must be the Amazon Resource Name (ARN) of the shared cluster snapshot. |
| `CreateDBCluster` | - | - | `DBClusterIdentifier`, `Engine` | - | `CreateDBClusterResult` | `DBClusterAlreadyExistsFault`, `DBClusterNotFoundFault`, `DBClusterParameterGroupNotFoundFault`, `DBClusterQuotaExceededFault`, `DBInstanceNotFoundFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotFoundFault`, `GlobalClusterNotFoundFault`, ... (+10) | Creates a new Amazon DocumentDB cluster. |
| `CreateDBClusterParameterGroup` | - | - | `DBClusterParameterGroupName`, `DBParameterGroupFamily`, `Description` | - | `CreateDBClusterParameterGroupResult` | `DBParameterGroupAlreadyExistsFault`, `DBParameterGroupQuotaExceededFault` | Creates a new cluster parameter group. Parameters in a cluster parameter group apply to all of the instances in a cluster. |
| `CreateDBClusterSnapshot` | - | - | `DBClusterIdentifier`, `DBClusterSnapshotIdentifier` | - | `CreateDBClusterSnapshotResult` | `DBClusterNotFoundFault`, `DBClusterSnapshotAlreadyExistsFault`, `InvalidDBClusterSnapshotStateFault`, `InvalidDBClusterStateFault`, `SnapshotQuotaExceededFault` | Creates a snapshot of a cluster. |
| `CreateDBInstance` | - | - | `DBClusterIdentifier`, `DBInstanceClass`, `DBInstanceIdentifier`, `Engine` | - | `CreateDBInstanceResult` | `AuthorizationNotFoundFault`, `DBClusterNotFoundFault`, `DBInstanceAlreadyExistsFault`, `DBParameterGroupNotFoundFault`, `DBSecurityGroupNotFoundFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotFoundFault`, `InstanceQuotaExceededFault`, ... (+7) | Creates a new instance. |
| `CreateDBSubnetGroup` | - | - | `DBSubnetGroupDescription`, `DBSubnetGroupName`, `SubnetIds` | - | `CreateDBSubnetGroupResult` | `DBSubnetGroupAlreadyExistsFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupQuotaExceededFault`, `DBSubnetQuotaExceededFault`, `InvalidSubnet` | Creates a new subnet group. subnet groups must contain at least one subnet in at least two Availability Zones in the Amazon Web Services Region. |
| `CreateEventSubscription` | - | - | `SnsTopicArn`, `SubscriptionName` | - | `CreateEventSubscriptionResult` | `EventSubscriptionQuotaExceededFault`, `SNSInvalidTopicFault`, `SNSNoAuthorizationFault`, `SNSTopicArnNotFoundFault`, `SourceNotFoundFault`, `SubscriptionAlreadyExistFault`, `SubscriptionCategoryNotFoundFault` | Creates an Amazon DocumentDB event notification subscription. This action requires a topic Amazon Resource Name (ARN) created by using the Amazon DocumentDB console, the Amazon SNS console, or the Amazon SNS API. |
| `CreateGlobalCluster` | - | - | `GlobalClusterIdentifier` | - | `CreateGlobalClusterResult` | `DBClusterNotFoundFault`, `GlobalClusterAlreadyExistsFault`, `GlobalClusterQuotaExceededFault`, `InvalidDBClusterStateFault` | Creates an Amazon DocumentDB global cluster that can span multiple multiple Amazon Web Services Regions. The global cluster contains one primary cluster with read-write capability, and up-to 10 read-only secondary clusters. |
| `DeleteDBCluster` | - | - | `DBClusterIdentifier` | - | `DeleteDBClusterResult` | `DBClusterNotFoundFault`, `DBClusterSnapshotAlreadyExistsFault`, `InvalidDBClusterSnapshotStateFault`, `InvalidDBClusterStateFault`, `SnapshotQuotaExceededFault` | Deletes a previously provisioned cluster. When you delete a cluster, all automated backups for that cluster are deleted and can't be recovered. |
| `DeleteDBClusterParameterGroup` | - | - | `DBClusterParameterGroupName` | - | `Unit` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Deletes a specified cluster parameter group. The cluster parameter group to be deleted can't be associated with any clusters. |
| `DeleteDBClusterSnapshot` | - | - | `DBClusterSnapshotIdentifier` | - | `DeleteDBClusterSnapshotResult` | `DBClusterSnapshotNotFoundFault`, `InvalidDBClusterSnapshotStateFault` | Deletes a cluster snapshot. If the snapshot is being copied, the copy operation is terminated. |
| `DeleteDBInstance` | - | - | `DBInstanceIdentifier` | - | `DeleteDBInstanceResult` | `DBInstanceNotFoundFault`, `DBSnapshotAlreadyExistsFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `SnapshotQuotaExceededFault` | Deletes a previously provisioned instance. |
| `DeleteDBSubnetGroup` | - | - | `DBSubnetGroupName` | - | `Unit` | `DBSubnetGroupNotFoundFault`, `InvalidDBSubnetGroupStateFault`, `InvalidDBSubnetStateFault` | Deletes a subnet group. The specified database subnet group must not be associated with any DB instances. |
| `DeleteEventSubscription` | - | - | `SubscriptionName` | - | `DeleteEventSubscriptionResult` | `InvalidEventSubscriptionStateFault`, `SubscriptionNotFoundFault` | Deletes an Amazon DocumentDB event notification subscription. |
| `DeleteGlobalCluster` | - | - | `GlobalClusterIdentifier` | - | `DeleteGlobalClusterResult` | `GlobalClusterNotFoundFault`, `InvalidGlobalClusterStateFault` | Deletes a global cluster. The primary and secondary clusters must already be detached or deleted before attempting to delete a global cluster. |
| `DescribeCertificates` | - | `paginated` | - | - | `CertificateMessage` | `CertificateNotFoundFault` | Returns a list of certificate authority (CA) certificates provided by Amazon DocumentDB for this Amazon Web Services account. |
| `DescribeDBClusterParameterGroups` | - | `paginated` | - | - | `DBClusterParameterGroupsMessage` | `DBParameterGroupNotFoundFault` | Returns a list of `DBClusterParameterGroup` descriptions. If a `DBClusterParameterGroupName` parameter is specified, the list contains only the description of the specified cluster parameter group. |
| `DescribeDBClusterParameters` | - | `paginated` | `DBClusterParameterGroupName` | - | `DBClusterParameterGroupDetails` | `DBParameterGroupNotFoundFault` | Returns the detailed parameter list for a particular cluster parameter group. |
| `DescribeDBClusterSnapshotAttributes` | - | - | `DBClusterSnapshotIdentifier` | - | `DescribeDBClusterSnapshotAttributesResult` | `DBClusterSnapshotNotFoundFault` | Returns a list of cluster snapshot attribute names and values for a manual DB cluster snapshot. When you share snapshots with other Amazon Web Services accounts, `DescribeDBClusterSnapshotAttributes` returns the `restore` attribute and a list of IDs for the... |
| `DescribeDBClusterSnapshots` | - | `paginated` | - | - | `DBClusterSnapshotMessage` | `DBClusterSnapshotNotFoundFault` | Returns information about cluster snapshots. This API operation supports pagination. |
| `DescribeDBClusters` | - | `paginated` | - | - | `DBClusterMessage` | `DBClusterNotFoundFault` | Returns information about provisioned Amazon DocumentDB clusters. This API operation supports pagination. |
| `DescribeDBEngineVersions` | - | `paginated` | - | - | `DBEngineVersionMessage` | - | Returns a list of the available engines. |
| `DescribeDBInstances` | - | `paginated` | - | - | `DBInstanceMessage` | `DBInstanceNotFoundFault` | Returns information about provisioned Amazon DocumentDB instances. This API supports pagination. |
| `DescribeDBSubnetGroups` | - | `paginated` | - | - | `DBSubnetGroupMessage` | `DBSubnetGroupNotFoundFault` | Returns a list of `DBSubnetGroup` descriptions. If a `DBSubnetGroupName` is specified, the list will contain only the descriptions of the specified `DBSubnetGroup`. |
| `DescribeEngineDefaultClusterParameters` | - | - | `DBParameterGroupFamily` | - | `DescribeEngineDefaultClusterParametersResult` | - | Returns the default engine and system parameter information for the cluster database engine. |
| `DescribeEventCategories` | - | - | - | - | `EventCategoriesMessage` | - | Displays a list of categories for all event source types, or, if specified, for a specified source type. |
| `DescribeEventSubscriptions` | - | `paginated` | - | - | `EventSubscriptionsMessage` | `SubscriptionNotFoundFault` | Lists all the subscription descriptions for a customer account. The description for a subscription includes `SubscriptionName`, `SNSTopicARN`, `CustomerID`, `SourceType`, `SourceID`, `CreationTime`, and `Status`. |
| `DescribeEvents` | - | `paginated` | - | - | `EventsMessage` | - | Returns events related to instances, security groups, snapshots, and DB parameter groups for the past 14 days. You can obtain events specific to a particular DB instance, security group, snapshot, or parameter group by providing the name as a parameter. |
| `DescribeGlobalClusters` | - | `paginated` | - | - | `GlobalClustersMessage` | `GlobalClusterNotFoundFault` | Returns information about Amazon DocumentDB global clusters. This API supports pagination. |
| `DescribeOrderableDBInstanceOptions` | - | `paginated` | `Engine` | - | `OrderableDBInstanceOptionsMessage` | - | Returns a list of orderable instance options for the specified engine. |
| `DescribePendingMaintenanceActions` | - | `paginated` | - | - | `PendingMaintenanceActionsMessage` | `ResourceNotFoundFault` | Returns a list of resources (for example, instances) that have at least one pending maintenance action. |
| `FailoverDBCluster` | - | - | - | - | `FailoverDBClusterResult` | `DBClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault` | Forces a failover for a cluster. A failover for a cluster promotes one of the Amazon DocumentDB replicas (read-only instances) in the cluster to be the primary instance (the cluster writer). |
| `FailoverGlobalCluster` | - | - | `GlobalClusterIdentifier`, `TargetDbClusterIdentifier` | - | `FailoverGlobalClusterResult` | `DBClusterNotFoundFault`, `GlobalClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidGlobalClusterStateFault` | Promotes the specified secondary DB cluster to be the primary DB cluster in the global cluster when failing over a global cluster occurs. Use this operation to respond to an unplanned event, such as a regional disaster in the primary region. |
| `ListTagsForResource` | - | - | `ResourceName` | - | `TagListMessage` | `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `DBSnapshotNotFoundFault` | Lists all tags on an Amazon DocumentDB resource. |
| `ModifyDBCluster` | - | - | `DBClusterIdentifier` | - | `ModifyDBClusterResult` | `DBClusterAlreadyExistsFault`, `DBClusterNotFoundFault`, `DBClusterParameterGroupNotFoundFault`, `DBSubnetGroupNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `InvalidDBSecurityGroupStateFault`, `InvalidDBSubnetGroupStateFault`, ... (+4) | Modifies a setting for an Amazon DocumentDB cluster. You can change one or more database configuration parameters by specifying these parameters and the new values in the request. |
| `ModifyDBClusterParameterGroup` | - | - | `DBClusterParameterGroupName`, `Parameters` | - | `DBClusterParameterGroupNameMessage` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Modifies the parameters of a cluster parameter group. To modify more than one parameter, submit a list of the following: `ParameterName`, `ParameterValue`, and `ApplyMethod`. |
| `ModifyDBClusterSnapshotAttribute` | - | - | `AttributeName`, `DBClusterSnapshotIdentifier` | - | `ModifyDBClusterSnapshotAttributeResult` | `DBClusterSnapshotNotFoundFault`, `InvalidDBClusterSnapshotStateFault`, `SharedSnapshotQuotaExceededFault` | Adds an attribute and values to, or removes an attribute and values from, a manual cluster snapshot. To share a manual cluster snapshot with other Amazon Web Services accounts, specify `restore` as the `AttributeName`, and use the `ValuesToAdd` parameter to... |
| `ModifyDBInstance` | - | - | `DBInstanceIdentifier` | - | `ModifyDBInstanceResult` | `AuthorizationNotFoundFault`, `CertificateNotFoundFault`, `DBInstanceAlreadyExistsFault`, `DBInstanceNotFoundFault`, `DBParameterGroupNotFoundFault`, `DBSecurityGroupNotFoundFault`, `DBUpgradeDependencyFailureFault`, `InsufficientDBInstanceCapacityFault`, ... (+5) | Modifies settings for an instance. You can change one or more database configuration parameters by specifying these parameters and the new values in the request. |
| `ModifyDBSubnetGroup` | - | - | `DBSubnetGroupName`, `SubnetIds` | - | `ModifyDBSubnetGroupResult` | `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotFoundFault`, `DBSubnetQuotaExceededFault`, `InvalidSubnet`, `SubnetAlreadyInUse` | Modifies an existing subnet group. subnet groups must contain at least one subnet in at least two Availability Zones in the Amazon Web Services Region. |
| `ModifyEventSubscription` | - | - | `SubscriptionName` | - | `ModifyEventSubscriptionResult` | `EventSubscriptionQuotaExceededFault`, `SNSInvalidTopicFault`, `SNSNoAuthorizationFault`, `SNSTopicArnNotFoundFault`, `SubscriptionCategoryNotFoundFault`, `SubscriptionNotFoundFault` | Modifies an existing Amazon DocumentDB event notification subscription. |
| `ModifyGlobalCluster` | - | - | `GlobalClusterIdentifier` | - | `ModifyGlobalClusterResult` | `GlobalClusterNotFoundFault`, `InvalidGlobalClusterStateFault` | Modify a setting for an Amazon DocumentDB global cluster. You can change one or more configuration parameters (for example: deletion protection), or the global cluster identifier by specifying these parameters and the new values in the request. |
| `RebootDBInstance` | - | - | `DBInstanceIdentifier` | - | `RebootDBInstanceResult` | `DBInstanceNotFoundFault`, `InvalidDBInstanceStateFault` | You might need to reboot your instance, usually for maintenance reasons. For example, if you make certain changes, or if you change the cluster parameter group that is associated with the instance, you must reboot the instance for the changes to take effect. |
| `RemoveFromGlobalCluster` | - | - | `DbClusterIdentifier`, `GlobalClusterIdentifier` | - | `RemoveFromGlobalClusterResult` | `DBClusterNotFoundFault`, `GlobalClusterNotFoundFault`, `InvalidGlobalClusterStateFault` | Detaches an Amazon DocumentDB secondary cluster from a global cluster. The cluster becomes a standalone cluster with read-write capability instead of being read-only and receiving data from a primary in a different region. |
| `RemoveSourceIdentifierFromSubscription` | - | - | `SourceIdentifier`, `SubscriptionName` | - | `RemoveSourceIdentifierFromSubscriptionResult` | `SourceNotFoundFault`, `SubscriptionNotFoundFault` | Removes a source identifier from an existing Amazon DocumentDB event notification subscription. |
| `RemoveTagsFromResource` | - | - | `ResourceName`, `TagKeys` | - | `Unit` | `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `DBSnapshotNotFoundFault` | Removes metadata tags from an Amazon DocumentDB resource. |
| `ResetDBClusterParameterGroup` | - | - | `DBClusterParameterGroupName` | - | `DBClusterParameterGroupNameMessage` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Modifies the parameters of a cluster parameter group to the default value. To reset specific parameters, submit a list of the following: `ParameterName` and `ApplyMethod`. |
| `RestoreDBClusterFromSnapshot` | - | - | `DBClusterIdentifier`, `Engine`, `SnapshotIdentifier` | - | `RestoreDBClusterFromSnapshotResult` | `DBClusterAlreadyExistsFault`, `DBClusterQuotaExceededFault`, `DBClusterSnapshotNotFoundFault`, `DBSnapshotNotFoundFault`, `DBSubnetGroupNotFoundFault`, `InsufficientDBClusterCapacityFault`, `InsufficientStorageClusterCapacityFault`, `InvalidDBClusterSnapshotStateFault`, ... (+7) | Creates a new cluster from a snapshot or cluster snapshot. If a snapshot is specified, the target cluster is created from the source DB snapshot with a default configuration and default security group. |
| `RestoreDBClusterToPointInTime` | - | - | `DBClusterIdentifier`, `SourceDBClusterIdentifier` | - | `RestoreDBClusterToPointInTimeResult` | `DBClusterAlreadyExistsFault`, `DBClusterNotFoundFault`, `DBClusterQuotaExceededFault`, `DBClusterSnapshotNotFoundFault`, `DBSubnetGroupNotFoundFault`, `InsufficientDBClusterCapacityFault`, `InsufficientStorageClusterCapacityFault`, `InvalidDBClusterSnapshotStateFault`, ... (+8) | Restores a cluster to an arbitrary point in time. Users can restore to any point in time before `LatestRestorableTime` for up to `BackupRetentionPeriod` days. |
| `StartDBCluster` | - | - | `DBClusterIdentifier` | - | `StartDBClusterResult` | `DBClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault` | Restarts the stopped cluster that is specified by `DBClusterIdentifier`. For more information, see Stopping and Starting an Amazon DocumentDB Cluster. |
| `StopDBCluster` | - | - | `DBClusterIdentifier` | - | `StopDBClusterResult` | `DBClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault` | Stops the running cluster that is specified by `DBClusterIdentifier`. The cluster must be in the available state. |
| `SwitchoverGlobalCluster` | - | - | `GlobalClusterIdentifier`, `TargetDbClusterIdentifier` | - | `SwitchoverGlobalClusterResult` | `DBClusterNotFoundFault`, `GlobalClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidGlobalClusterStateFault` | Switches over the specified secondary Amazon DocumentDB cluster to be the new primary Amazon DocumentDB cluster in the global database cluster. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `DBClusterNotFoundFault` | `structure` | `message` | `DBClusterIdentifier` doesn't refer to an existing cluster. |
| `InvalidDBClusterStateFault` | `structure` | `message` | The cluster isn't in a valid state. |
| `InvalidDBInstanceStateFault` | `structure` | `message` | The specified instance isn't in the available state. |
| `DBInstanceNotFoundFault` | `structure` | `message` | `DBInstanceIdentifier` doesn't refer to an existing instance. |
| `DBParameterGroupNotFoundFault` | `structure` | `message` | `DBParameterGroupName` doesn't refer to an existing parameter group. |
| `DBSubnetGroupNotFoundFault` | `structure` | `message` | `DBSubnetGroupName` doesn't refer to an existing subnet group. |
| `DBClusterSnapshotNotFoundFault` | `structure` | `message` | `DBClusterSnapshotIdentifier` doesn't refer to an existing cluster snapshot. |
| `InvalidDBClusterSnapshotStateFault` | `structure` | `message` | The provided value isn't a valid cluster snapshot state. |
| `GlobalClusterNotFoundFault` | `structure` | `message` | The `GlobalClusterIdentifier` doesn't refer to an existing global cluster. |
| `InvalidSubnet` | `structure` | `message` | The requested subnet is not valid, or multiple subnets were requested that are not all in a common virtual private cloud (VPC). |
| `InvalidGlobalClusterStateFault` | `structure` | `message` | The requested operation can't be performed while the cluster is in this state. |
| `InvalidVPCNetworkStateFault` | `structure` | `message` | The subnet group doesn't cover all Availability Zones after it is created because of changes that were made. |
| `StorageQuotaExceededFault` | `structure` | `message` | The request would cause you to exceed the allowed amount of storage available across all instances. |
| `SubscriptionNotFoundFault` | `structure` | `message` | The subscription name does not exist. |
| `KMSKeyNotAccessibleFault` | `structure` | `message` | An error occurred when accessing an KMS key. |
| `DBSnapshotNotFoundFault` | `structure` | `message` | `DBSnapshotIdentifier` doesn't refer to an existing snapshot. |
| `SnapshotQuotaExceededFault` | `structure` | `message` | The request would cause you to exceed the allowed number of snapshots. |
| `DBClusterAlreadyExistsFault` | `structure` | `message` | You already have a cluster with the given identifier. |
| `DBSubnetGroupDoesNotCoverEnoughAZs` | `structure` | `message` | Subnets in the subnet group should cover at least two Availability Zones unless there is only one Availability Zone. |
| `NetworkTypeNotSupported` | `structure` | `message` | The network type is not supported by either `DBSubnetGroup` or the DB engine version. |
| `SourceNotFoundFault` | `structure` | `message` | The requested source could not be found. |
| `DBClusterSnapshotAlreadyExistsFault` | `structure` | `message` | You already have a cluster snapshot with the given identifier. |
| `DBClusterQuotaExceededFault` | `structure` | `message` | The cluster can't be created because you have reached the maximum allowed quota of clusters. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
