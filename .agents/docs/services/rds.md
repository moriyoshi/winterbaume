# Amazon Relational Database Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Relational Database Service Amazon Relational Database Service (Amazon RDS) is a web service that makes it easier to set up, operate, and scale a relational database in the cloud. It provides cost-efficient, resizeable capacity for an industry-standard relational database and manages common database administration tasks, freeing up developers to focus on what makes their applications and businesses unique. Amazon RDS gives you access to the capabilities of a MySQL, MariaDB, PostgreSQL, Microsoft SQL Server, Oracle, Db2, or Amazon Aurora database server. These capabilities mean that the code, applications, and tools you already use today with your existing databases work with Amazon RDS without modification. Amazon RDS automatically backs up your database and maintains the database software that powers your DB instance.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Relational Database Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon Relational Database Service by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon Relational Database Service by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon Relational Database Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Modify`, `Delete`, `Create`, `Restore` operation families, including `DescribeAccountAttributes`, `DescribeBlueGreenDeployments`, `DescribeCertificates`, `DescribeDBClusterAutomatedBackups`, `ModifyActivityStream`, `ModifyCertificates`.

## Service Identity and Protocol

- AWS model slug: `rds`
- AWS SDK for Rust slug: `rds`
- Model version: `2014-10-31`
- Model file: `vendor/api-models-aws/models/rds/service/2014-10-31/rds-2014-10-31.json`
- SDK ID: `RDS`
- Endpoint prefix: `rds`
- ARN namespace: `rds`
- CloudFormation name: `RDS`
- CloudTrail event source: `rds.amazonaws.com`
- Protocols: `awsQuery`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (46), `Modify` (23), `Delete` (21), `Create` (20), `Restore` (6), `Copy` (5), `Remove` (5), `Start` (5).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddRoleToDBCluster`, `AddRoleToDBInstance`, `AddSourceIdentifierToSubscription`, `AddTagsToResource`, `CancelExportTask`, `CreateBlueGreenDeployment`, `CreateCustomDBEngineVersion`, `CreateDBCluster`, `CreateDBClusterEndpoint`, `CreateDBClusterParameterGroup`, `CreateDBClusterSnapshot`, `CreateDBInstance`, `CreateDBInstanceReadReplica`, `CreateDBParameterGroup`, `CreateDBProxy`, `CreateDBProxyEndpoint`, `CreateDBSecurityGroup`, `CreateDBShardGroup`, `CreateDBSnapshot`, `CreateDBSubnetGroup`, ... (+74).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccountAttributes`, `DescribeBlueGreenDeployments`, `DescribeCertificates`, `DescribeDBClusterAutomatedBackups`, `DescribeDBClusterBacktracks`, `DescribeDBClusterEndpoints`, `DescribeDBClusterParameterGroups`, `DescribeDBClusterParameters`, `DescribeDBClusterSnapshotAttributes`, `DescribeDBClusterSnapshots`, `DescribeDBClusters`, `DescribeDBEngineVersions`, `DescribeDBInstanceAutomatedBackups`, `DescribeDBInstances`, `DescribeDBLogFiles`, `DescribeDBMajorEngineVersions`, `DescribeDBParameterGroups`, `DescribeDBParameters`, `DescribeDBProxies`, `DescribeDBProxyEndpoints`, ... (+27).
- Pagination is modelled for 41 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelExportTask`, `DescribeExportTasks`, `StartActivityStream`, `StartDBCluster`, `StartDBInstance`, `StartDBInstanceAutomatedBackupsReplication`, `StartExportTask`, `StopActivityStream`, `StopDBCluster`, `StopDBInstance`, `StopDBInstanceAutomatedBackupsReplication`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 151 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `SNS`, `Glue`, `EC2/VPC`, `ECS`, `RDS`, `Redshift`, `Secrets Manager`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Overview.DBInstance.Modifying.html
- https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_UpgradeDBInstance.Maintenance.html
- https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_WorkingWithAutomatedBackups.html
- https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_DeleteInstance.html

Research outcomes:
- Many DB instance modifications can be applied immediately or deferred to the next maintenance window. Some changes require a manual reboot to take effect.
- ApplyImmediately can cause downtime depending on the modification. Deferred changes are visible as pending modifications until applied.
- Every DB instance has a weekly maintenance window. If none is specified, RDS assigns a random 30-minute window from a Region-specific block.
- The backup window and maintenance window cannot overlap.
- Multi-AZ operating system maintenance updates the standby, promotes it, then updates the old primary. Engine upgrades for many Multi-AZ instances can make both primary and standby unavailable.
- Backup storage consists of automated backups and manual snapshots per Region. Manual snapshots are independent of automated backups and survive DB instance deletion.
- Deletion protection must be disabled before deleting a DB instance.
- Deleting an instance can create a final snapshot, retain automated backups for the retention period, and automatically promote same-Region read replicas to standalone instances.

Parity implications:
- Model DB instances, pending modifications, maintenance windows, backup windows, snapshots, automated backups, read replicas, and deletion state separately.
- Modify APIs should distinguish immediate effects, deferred pending values, and reboot-required changes.
- Delete workflows need deletion-protection checks, final snapshot constraints, retained-backup choices, read-replica promotion, and long-running deleting status.

## Control-Plane / Data-Plane Coherence

- **Paired with `rdsdata` ( RDS Data API ).** RDS Data ( `winterbaume-rdsdata` ) executes SQL against Aurora Serverless v1/v2 and Aurora MySQL/PostgreSQL clusters that this control plane creates and manages the lifecycle of. `ExecuteStatement` and `BeginTransaction` carry a `resourceArn` that must point to an existing cluster with the Data API enabled; in real AWS the call fails with `BadRequestException` ( "Cluster not enabled for Data API" ) if it is not.
- **Current Winterbaume status: deliberately separate.** `winterbaume-rdsdata` is a "bring-your-own-result" mock — callers `enqueue_result` ahead of time and `ExecuteStatement` dequeues. It does not validate `resourceArn` against this crate's clusters and never executes real SQL. This is acceptable for unit testing where the test author seeds expected results, but it is not parity behaviour.
- **What needs to change ( low priority ):** if a future workflow needs the data plane to validate cluster existence and "Data API enabled" state, `winterbaume-rdsdata` should observe this crate's `db_clusters` and reject unknown ARNs. Real SQL execution is not in scope for the mock.

## Current Network Resource Stub Semantics

RDS currently stores database subnet groups and security group references inside RDS state.

- DB subnet group operations store subnet IDs and return RDS-local subnet group data.
- DB instances and clusters store DB subnet group names, classic DB security group names, and VPC security group IDs as request metadata.
- Security group ingress operations record RDS-local authorisations and do not mutate EC2 security groups.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Describe

- Operations: `DescribeAccountAttributes`, `DescribeBlueGreenDeployments`, `DescribeCertificates`, `DescribeDBClusterAutomatedBackups`, `DescribeDBClusterBacktracks`, `DescribeDBClusterEndpoints`, `DescribeDBClusterParameterGroups`, `DescribeDBClusterParameters`, `DescribeDBClusters`, `DescribeDBClusterSnapshotAttributes`, `DescribeDBClusterSnapshots`, `DescribeDBEngineVersions`, `DescribeDBInstanceAutomatedBackups`, `DescribeDBInstances`, `DescribeDBLogFiles`, `DescribeDBMajorEngineVersions`, `DescribeDBParameterGroups`, `DescribeDBParameters`, `DescribeDBProxies`, `DescribeDBProxyEndpoints`, `DescribeDBProxyTargetGroups`, `DescribeDBProxyTargets`, `DescribeDBRecommendations`, `DescribeDBSecurityGroups`, `DescribeDBShardGroups`, `DescribeDBSnapshotAttributes`, `DescribeDBSnapshots`, `DescribeDBSnapshotTenantDatabases`, `DescribeDBSubnetGroups`, `DescribeEngineDefaultClusterParameters`, `DescribeEngineDefaultParameters`, `DescribeEventCategories`, `DescribeEvents`, `DescribeEventSubscriptions`, `DescribeExportTasks`, `DescribeGlobalClusters`, `DescribeIntegrations`, `DescribeOptionGroupOptions`, `DescribeOptionGroups`, `DescribeOrderableDBInstanceOptions`, `DescribePendingMaintenanceActions`, `DescribeReservedDBInstances`, `DescribeReservedDBInstancesOfferings`, `DescribeServerlessV2PlatformVersions`, `DescribeSourceRegions`, `DescribeTenantDatabases`, `DescribeValidDBInstanceModifications`
- Traits: `paginated` (41)
- Common required input members in this group: `DBInstanceIdentifier`, `DBProxyName`, `DBParameterGroupFamily`

### Modify

- Operations: `ModifyActivityStream`, `ModifyCertificates`, `ModifyCurrentDBClusterCapacity`, `ModifyCustomDBEngineVersion`, `ModifyDBCluster`, `ModifyDBClusterEndpoint`, `ModifyDBClusterParameterGroup`, `ModifyDBClusterSnapshotAttribute`, `ModifyDBInstance`, `ModifyDBParameterGroup`, `ModifyDBProxy`, `ModifyDBProxyEndpoint`, `ModifyDBProxyTargetGroup`, `ModifyDBRecommendation`, `ModifyDBShardGroup`, `ModifyDBSnapshot`, `ModifyDBSnapshotAttribute`, `ModifyDBSubnetGroup`, `ModifyEventSubscription`, `ModifyGlobalCluster`, `ModifyIntegration`, `ModifyOptionGroup`, `ModifyTenantDatabase`
- Common required input members in this group: `DBClusterIdentifier`, `Parameters`, `AttributeName`, `DBInstanceIdentifier`, `DBProxyName`, `DBSnapshotIdentifier`

### Delete

- Operations: `DeleteBlueGreenDeployment`, `DeleteCustomDBEngineVersion`, `DeleteDBCluster`, `DeleteDBClusterAutomatedBackup`, `DeleteDBClusterEndpoint`, `DeleteDBClusterParameterGroup`, `DeleteDBClusterSnapshot`, `DeleteDBInstance`, `DeleteDBInstanceAutomatedBackup`, `DeleteDBParameterGroup`, `DeleteDBProxy`, `DeleteDBProxyEndpoint`, `DeleteDBSecurityGroup`, `DeleteDBShardGroup`, `DeleteDBSnapshot`, `DeleteDBSubnetGroup`, `DeleteEventSubscription`, `DeleteGlobalCluster`, `DeleteIntegration`, `DeleteOptionGroup`, `DeleteTenantDatabase`
- Common required input members in this group: `DBInstanceIdentifier`

### Create

- Operations: `CreateBlueGreenDeployment`, `CreateCustomDBEngineVersion`, `CreateDBCluster`, `CreateDBClusterEndpoint`, `CreateDBClusterParameterGroup`, `CreateDBClusterSnapshot`, `CreateDBInstance`, `CreateDBInstanceReadReplica`, `CreateDBParameterGroup`, `CreateDBProxy`, `CreateDBProxyEndpoint`, `CreateDBSecurityGroup`, `CreateDBShardGroup`, `CreateDBSnapshot`, `CreateDBSubnetGroup`, `CreateEventSubscription`, `CreateGlobalCluster`, `CreateIntegration`, `CreateOptionGroup`, `CreateTenantDatabase`
- Common required input members in this group: `Engine`, `DBClusterIdentifier`, `DBParameterGroupFamily`, `Description`, `DBInstanceIdentifier`, `DBProxyName`, `VpcSubnetIds`

### Restore

- Operations: `RestoreDBClusterFromS3`, `RestoreDBClusterFromSnapshot`, `RestoreDBClusterToPointInTime`, `RestoreDBInstanceFromDBSnapshot`, `RestoreDBInstanceFromS3`, `RestoreDBInstanceToPointInTime`
- Common required input members in this group: `DBClusterIdentifier`, `Engine`, `SourceEngine`, `SourceEngineVersion`, `S3BucketName`, `S3IngestionRoleArn`, `DBInstanceIdentifier`

### Copy

- Operations: `CopyDBClusterParameterGroup`, `CopyDBClusterSnapshot`, `CopyDBParameterGroup`, `CopyDBSnapshot`, `CopyOptionGroup`
- Common required input members in this group: -

### Remove

- Operations: `RemoveFromGlobalCluster`, `RemoveRoleFromDBCluster`, `RemoveRoleFromDBInstance`, `RemoveSourceIdentifierFromSubscription`, `RemoveTagsFromResource`
- Common required input members in this group: `RoleArn`

### Start

- Operations: `StartActivityStream`, `StartDBCluster`, `StartDBInstance`, `StartDBInstanceAutomatedBackupsReplication`, `StartExportTask`
- Common required input members in this group: `KmsKeyId`

### Add

- Operations: `AddRoleToDBCluster`, `AddRoleToDBInstance`, `AddSourceIdentifierToSubscription`, `AddTagsToResource`
- Common required input members in this group: `RoleArn`

### Stop

- Operations: `StopActivityStream`, `StopDBCluster`, `StopDBInstance`, `StopDBInstanceAutomatedBackupsReplication`
- Common required input members in this group: -

### Reboot

- Operations: `RebootDBCluster`, `RebootDBInstance`, `RebootDBShardGroup`
- Common required input members in this group: -

### Switchover

- Operations: `SwitchoverBlueGreenDeployment`, `SwitchoverGlobalCluster`, `SwitchoverReadReplica`
- Common required input members in this group: -

### Failover

- Operations: `FailoverDBCluster`, `FailoverGlobalCluster`
- Common required input members in this group: -

### Promote

- Operations: `PromoteReadReplica`, `PromoteReadReplicaDBCluster`
- Common required input members in this group: -

### Reset

- Operations: `ResetDBClusterParameterGroup`, `ResetDBParameterGroup`
- Common required input members in this group: -

### Apply

- Operations: `ApplyPendingMaintenanceAction`
- Common required input members in this group: -

### Authorize

- Operations: `AuthorizeDBSecurityGroupIngress`
- Common required input members in this group: -

### Backtrack

- Operations: `BacktrackDBCluster`
- Common required input members in this group: -

### Cancel

- Operations: `CancelExportTask`
- Common required input members in this group: -

### Deregister

- Operations: `DeregisterDBProxyTargets`
- Common required input members in this group: -

### Disable

- Operations: `DisableHttpEndpoint`
- Common required input members in this group: -

### Download

- Operations: `DownloadDBLogFilePortion`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Enable

- Operations: `EnableHttpEndpoint`
- Common required input members in this group: -

### List

- Operations: `ListTagsForResource`
- Common required input members in this group: -

### Purchase

- Operations: `PurchaseReservedDBInstancesOffering`
- Common required input members in this group: -

### Register

- Operations: `RegisterDBProxyTargets`
- Common required input members in this group: -

### Revoke

- Operations: `RevokeDBSecurityGroupIngress`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddRoleToDBCluster` | `-` | - | `DBClusterIdentifier`, `RoleArn` | - | `Unit` | `DBClusterNotFoundFault`, `DBClusterRoleAlreadyExistsFault`, `DBClusterRoleQuotaExceededFault`, `InvalidDBClusterStateFault` | Associates an Identity and Access Management (IAM) role with a DB cluster. |
| `AddRoleToDBInstance` | `-` | - | `DBInstanceIdentifier`, `RoleArn`, `FeatureName` | - | `Unit` | `DBInstanceNotFoundFault`, `DBInstanceRoleAlreadyExistsFault`, `DBInstanceRoleQuotaExceededFault`, `InvalidDBInstanceStateFault` | Associates an Amazon Web Services Identity and Access Management (IAM) role with a DB instance. To add a role to a DB instance, the status of the DB instance must be available . This command doesn't apply to RDS Custom. |
| `AddSourceIdentifierToSubscription` | `-` | - | `SubscriptionName`, `SourceIdentifier` | - | `AddSourceIdentifierToSubscriptionResult` | `SourceNotFoundFault`, `SubscriptionNotFoundFault` | Adds a source identifier to an existing RDS event notification subscription. |
| `AddTagsToResource` | `-` | - | `ResourceName`, `Tags` | - | `Unit` | `BlueGreenDeploymentNotFoundFault`, `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `DBProxyEndpointNotFoundFault`, `DBProxyNotFoundFault`, `DBProxyTargetGroupNotFoundFault`, `DBShardGroupNotFoundFault`, `DBSnapshotNotFoundFault`, `DBSnapshotTenantDatabaseNotFoundFault`, `IntegrationNotFoundFault`, `InvalidDBClusterEndpointStateFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `TenantDatabaseNotFoundFault` | Adds metadata tags to an Amazon RDS resource. These tags can also be used with cost allocation reporting to track cost associated with Amazon RDS resources, or used in a Condition statement in an IAM policy for Amazo ... |
| `ApplyPendingMaintenanceAction` | `-` | - | `ResourceIdentifier`, `ApplyAction`, `OptInType` | - | `ApplyPendingMaintenanceActionResult` | `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `ResourceNotFoundFault` | Applies a pending maintenance action to a resource (for example, to a DB instance). |
| `AuthorizeDBSecurityGroupIngress` | `-` | - | `DBSecurityGroupName` | - | `AuthorizeDBSecurityGroupIngressResult` | `AuthorizationAlreadyExistsFault`, `AuthorizationQuotaExceededFault`, `DBSecurityGroupNotFoundFault`, `InvalidDBSecurityGroupStateFault` | Enables ingress to a DBSecurityGroup using one of two forms of authorization. First, EC2 or VPC security groups can be added to the DBSecurityGroup if the application using the database is running on EC2 or VPC insta ... |
| `BacktrackDBCluster` | `-` | - | `DBClusterIdentifier`, `BacktrackTo` | - | `DBClusterBacktrack` | `DBClusterNotFoundFault`, `InvalidDBClusterStateFault` | Backtracks a DB cluster to a specific time, without creating a new DB cluster. For more information on backtracking, see Backtracking an Aurora DB Cluster in the Amazon Aurora User Guide . This action applies only to ... |
| `CancelExportTask` | `-` | - | `ExportTaskIdentifier` | - | `ExportTask` | `ExportTaskNotFoundFault`, `InvalidExportTaskStateFault` | Cancels an export task in progress that is exporting a snapshot or cluster to Amazon S3. Any data that has already been written to the S3 bucket isn't removed. |
| `CopyDBClusterParameterGroup` | `-` | - | `SourceDBClusterParameterGroupIdentifier`, `TargetDBClusterParameterGroupIdentifier`, `TargetDBClusterParameterGroupDescription` | - | `CopyDBClusterParameterGroupResult` | `DBParameterGroupAlreadyExistsFault`, `DBParameterGroupNotFoundFault`, `DBParameterGroupQuotaExceededFault` | Copies the specified DB cluster parameter group. You can't copy a default DB cluster parameter group. Instead, create a new custom DB cluster parameter group, which copies the default parameters and values for the sp ... |
| `CopyDBClusterSnapshot` | `-` | - | `SourceDBClusterSnapshotIdentifier`, `TargetDBClusterSnapshotIdentifier` | - | `CopyDBClusterSnapshotResult` | `DBClusterSnapshotAlreadyExistsFault`, `DBClusterSnapshotNotFoundFault`, `InvalidDBClusterSnapshotStateFault`, `InvalidDBClusterStateFault`, `KMSKeyNotAccessibleFault`, `SnapshotQuotaExceededFault` | Copies a snapshot of a DB cluster. To copy a DB cluster snapshot from a shared manual DB cluster snapshot, SourceDBClusterSnapshotIdentifier must be the Amazon Resource Name (ARN) of the shared DB cluster snapshot. Y ... |
| `CopyDBParameterGroup` | `-` | - | `SourceDBParameterGroupIdentifier`, `TargetDBParameterGroupIdentifier`, `TargetDBParameterGroupDescription` | - | `CopyDBParameterGroupResult` | `DBParameterGroupAlreadyExistsFault`, `DBParameterGroupNotFoundFault`, `DBParameterGroupQuotaExceededFault` | Copies the specified DB parameter group. You can't copy a default DB parameter group. Instead, create a new custom DB parameter group, which copies the default parameters and values for the specified DB parameter gro ... |
| `CopyDBSnapshot` | `-` | - | `SourceDBSnapshotIdentifier`, `TargetDBSnapshotIdentifier` | - | `CopyDBSnapshotResult` | `CustomAvailabilityZoneNotFoundFault`, `DBSnapshotAlreadyExistsFault`, `DBSnapshotNotFoundFault`, `InvalidDBSnapshotStateFault`, `KMSKeyNotAccessibleFault`, `SnapshotQuotaExceededFault` | Copies the specified DB snapshot. The source DB snapshot must be in the available state. You can copy a snapshot from one Amazon Web Services Region to another. In that case, the Amazon Web Services Region where you ... |
| `CopyOptionGroup` | `-` | - | `SourceOptionGroupIdentifier`, `TargetOptionGroupIdentifier`, `TargetOptionGroupDescription` | - | `CopyOptionGroupResult` | `OptionGroupAlreadyExistsFault`, `OptionGroupNotFoundFault`, `OptionGroupQuotaExceededFault` | Copies the specified option group. |
| `CreateBlueGreenDeployment` | `-` | - | `BlueGreenDeploymentName`, `Source` | - | `CreateBlueGreenDeploymentResponse` | `BlueGreenDeploymentAlreadyExistsFault`, `DBClusterNotFoundFault`, `DBClusterParameterGroupNotFoundFault`, `DBClusterQuotaExceededFault`, `DBInstanceNotFoundFault`, `DBParameterGroupNotFoundFault`, `InstanceQuotaExceededFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `SourceClusterNotSupportedFault`, `SourceDatabaseNotSupportedFault`, `StorageQuotaExceededFault` | Creates a blue/green deployment. A blue/green deployment creates a staging environment that copies the production environment. In a blue/green deployment, the blue environment is the current production environment. T ... |
| `CreateCustomDBEngineVersion` | `-` | - | `Engine`, `EngineVersion` | - | `DBEngineVersion` | `CreateCustomDBEngineVersionFault`, `CustomDBEngineVersionAlreadyExistsFault`, `CustomDBEngineVersionNotFoundFault`, `CustomDBEngineVersionQuotaExceededFault`, `Ec2ImagePropertiesNotSupportedFault`, `InvalidCustomDBEngineVersionStateFault`, `KMSKeyNotAccessibleFault` | Creates a custom DB engine version (CEV). |
| `CreateDBCluster` | `-` | - | `DBClusterIdentifier`, `Engine` | - | `CreateDBClusterResult` | `DBClusterAlreadyExistsFault`, `DBClusterNotFoundFault`, `DBClusterParameterGroupNotFoundFault`, `DBClusterQuotaExceededFault`, `DBInstanceNotFoundFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotFoundFault`, `DomainNotFoundFault`, `GlobalClusterNotFoundFault`, `InsufficientDBInstanceCapacityFault`, `InsufficientStorageClusterCapacityFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `InvalidDBSubnetGroupFault`, `InvalidDBSubnetGroupStateFault`, `InvalidGlobalClusterStateFault`, `InvalidSubnet`, `InvalidVPCNetworkStateFault`, `KMSKeyNotAccessibleFault`, `NetworkTypeNotSupported`, `OptionGroupNotFoundFault`, `StorageQuotaExceededFault`, `StorageTypeNotSupportedFault`, `VpcEncryptionControlViolationException` | Creates a new Amazon Aurora DB cluster or Multi-AZ DB cluster. If you create an Aurora DB cluster, the request creates an empty cluster. You must explicitly create the writer instance for your DB cluster using the Cr ... |
| `CreateDBClusterEndpoint` | `-` | - | `DBClusterIdentifier`, `DBClusterEndpointIdentifier`, `EndpointType` | - | `DBClusterEndpoint` | `DBClusterEndpointAlreadyExistsFault`, `DBClusterEndpointQuotaExceededFault`, `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault` | Creates a new custom endpoint and associates it with an Amazon Aurora DB cluster. This action applies only to Aurora DB clusters. |
| `CreateDBClusterParameterGroup` | `-` | - | `DBClusterParameterGroupName`, `DBParameterGroupFamily`, `Description` | - | `CreateDBClusterParameterGroupResult` | `DBParameterGroupAlreadyExistsFault`, `DBParameterGroupQuotaExceededFault` | Creates a new DB cluster parameter group. Parameters in a DB cluster parameter group apply to all of the instances in a DB cluster. A DB cluster parameter group is initially created with the default parameters for th ... |
| `CreateDBClusterSnapshot` | `-` | - | `DBClusterSnapshotIdentifier`, `DBClusterIdentifier` | - | `CreateDBClusterSnapshotResult` | `DBClusterNotFoundFault`, `DBClusterSnapshotAlreadyExistsFault`, `InvalidDBClusterSnapshotStateFault`, `InvalidDBClusterStateFault`, `SnapshotQuotaExceededFault` | Creates a snapshot of a DB cluster. For more information on Amazon Aurora, see What is Amazon Aurora? in the Amazon Aurora User Guide . For more information on Multi-AZ DB clusters, see Multi-AZ DB cluster deployment ... |
| `CreateDBInstance` | `-` | - | `DBInstanceIdentifier`, `DBInstanceClass`, `Engine` | - | `CreateDBInstanceResult` | `AuthorizationNotFoundFault`, `BackupPolicyNotFoundFault`, `CertificateNotFoundFault`, `DBClusterNotFoundFault`, `DBInstanceAlreadyExistsFault`, `DBParameterGroupNotFoundFault`, `DBSecurityGroupNotFoundFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotFoundFault`, `DomainNotFoundFault`, `InstanceQuotaExceededFault`, `InsufficientDBInstanceCapacityFault`, `InvalidDBClusterStateFault`, `InvalidSubnet`, `InvalidVPCNetworkStateFault`, `KMSKeyNotAccessibleFault`, `NetworkTypeNotSupported`, `OptionGroupNotFoundFault`, `ProvisionedIopsNotAvailableInAZFault`, `StorageQuotaExceededFault`, `StorageTypeNotSupportedFault`, `TenantDatabaseQuotaExceededFault`, `VpcEncryptionControlViolationException` | Creates a new DB instance. The new DB instance can be an RDS DB instance, or it can be a DB instance in an Aurora DB cluster. For an Aurora DB cluster, you can call this operation multiple times to add more than one ... |
| `CreateDBInstanceReadReplica` | `-` | - | `DBInstanceIdentifier` | - | `CreateDBInstanceReadReplicaResult` | `CertificateNotFoundFault`, `DBClusterNotFoundFault`, `DBInstanceAlreadyExistsFault`, `DBInstanceNotFoundFault`, `DBParameterGroupNotFoundFault`, `DBSecurityGroupNotFoundFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotAllowedFault`, `DBSubnetGroupNotFoundFault`, `DomainNotFoundFault`, `InstanceQuotaExceededFault`, `InsufficientDBInstanceCapacityFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `InvalidDBSubnetGroupFault`, `InvalidSubnet`, `InvalidVPCNetworkStateFault`, `KMSKeyNotAccessibleFault`, `NetworkTypeNotSupported`, `OptionGroupNotFoundFault`, `ProvisionedIopsNotAvailableInAZFault`, `StorageQuotaExceededFault`, `StorageTypeNotSupportedFault`, `TenantDatabaseQuotaExceededFault`, `VpcEncryptionControlViolationException` | Creates a new DB instance that acts as a read replica for an existing source DB instance or Multi-AZ DB cluster. You can create a read replica for a DB instance running Db2, MariaDB, MySQL, Oracle, PostgreSQL, or SQL ... |
| `CreateDBParameterGroup` | `-` | - | `DBParameterGroupName`, `DBParameterGroupFamily`, `Description` | - | `CreateDBParameterGroupResult` | `DBParameterGroupAlreadyExistsFault`, `DBParameterGroupQuotaExceededFault` | Creates a new DB parameter group. A DB parameter group is initially created with the default parameters for the database engine used by the DB instance. To provide custom values for any of the parameters, you must mo ... |
| `CreateDBProxy` | `-` | - | `DBProxyName`, `EngineFamily`, `RoleArn`, `VpcSubnetIds` | - | `CreateDBProxyResponse` | `DBProxyAlreadyExistsFault`, `DBProxyQuotaExceededFault`, `InvalidSubnet` | Creates a new DB proxy. |
| `CreateDBProxyEndpoint` | `-` | - | `DBProxyName`, `DBProxyEndpointName`, `VpcSubnetIds` | - | `CreateDBProxyEndpointResponse` | `DBProxyEndpointAlreadyExistsFault`, `DBProxyEndpointQuotaExceededFault`, `DBProxyNotFoundFault`, `InvalidDBProxyStateFault`, `InvalidSubnet` | Creates a DBProxyEndpoint . Only applies to proxies that are associated with Aurora DB clusters. You can use DB proxy endpoints to specify read/write or read-only access to the DB cluster. You can also use DB proxy e ... |
| `CreateDBSecurityGroup` | `-` | - | `DBSecurityGroupName`, `DBSecurityGroupDescription` | - | `CreateDBSecurityGroupResult` | `DBSecurityGroupAlreadyExistsFault`, `DBSecurityGroupNotSupportedFault`, `DBSecurityGroupQuotaExceededFault` | Creates a new DB security group. DB security groups control access to a DB instance. A DB security group controls access to EC2-Classic DB instances that are not in a VPC. EC2-Classic was retired on August 15, 2022. ... |
| `CreateDBShardGroup` | `-` | - | `DBShardGroupIdentifier`, `DBClusterIdentifier`, `MaxACU` | - | `DBShardGroup` | `DBClusterNotFoundFault`, `DBShardGroupAlreadyExistsFault`, `InvalidDBClusterStateFault`, `InvalidVPCNetworkStateFault`, `MaxDBShardGroupLimitReached`, `NetworkTypeNotSupported`, `UnsupportedDBEngineVersionFault` | Creates a new DB shard group for Aurora Limitless Database. You must enable Aurora Limitless Database to create a DB shard group. Valid for: Aurora DB clusters only |
| `CreateDBSnapshot` | `-` | - | `DBSnapshotIdentifier`, `DBInstanceIdentifier` | - | `CreateDBSnapshotResult` | `DBInstanceNotFoundFault`, `DBSnapshotAlreadyExistsFault`, `InvalidDBInstanceStateFault`, `SnapshotQuotaExceededFault` | Creates a snapshot of a DB instance. The source DB instance must be in the available or storage-optimization state. |
| `CreateDBSubnetGroup` | `-` | - | `DBSubnetGroupName`, `DBSubnetGroupDescription`, `SubnetIds` | - | `CreateDBSubnetGroupResult` | `DBSubnetGroupAlreadyExistsFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupQuotaExceededFault`, `DBSubnetQuotaExceededFault`, `InvalidSubnet` | Creates a new DB subnet group. DB subnet groups must contain at least one subnet in at least two AZs in the Amazon Web Services Region. |
| `CreateEventSubscription` | `-` | - | `SubscriptionName`, `SnsTopicArn` | - | `CreateEventSubscriptionResult` | `EventSubscriptionQuotaExceededFault`, `SNSInvalidTopicFault`, `SNSNoAuthorizationFault`, `SNSTopicArnNotFoundFault`, `SourceNotFoundFault`, `SubscriptionAlreadyExistFault`, `SubscriptionCategoryNotFoundFault` | Creates an RDS event notification subscription. This operation requires a topic Amazon Resource Name (ARN) created by either the RDS console, the SNS console, or the SNS API. To obtain an ARN with SNS, you must creat ... |
| `CreateGlobalCluster` | `-` | - | `GlobalClusterIdentifier` | - | `CreateGlobalClusterResult` | `DBClusterNotFoundFault`, `GlobalClusterAlreadyExistsFault`, `GlobalClusterQuotaExceededFault`, `InvalidDBClusterStateFault`, `InvalidDBShardGroupStateFault`, `ResourceNotFoundFault` | Creates an Aurora global database spread across multiple Amazon Web Services Regions. The global database contains a single primary cluster with read-write capability, and a read-only secondary cluster that receives ... |
| `CreateIntegration` | `-` | - | `SourceArn`, `TargetArn`, `IntegrationName` | - | `Integration` | `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `IntegrationAlreadyExistsFault`, `IntegrationConflictOperationFault`, `IntegrationQuotaExceededFault`, `KMSKeyNotAccessibleFault` | Creates a zero-ETL integration with Amazon Redshift. |
| `CreateOptionGroup` | `-` | - | `OptionGroupName`, `EngineName`, `MajorEngineVersion`, `OptionGroupDescription` | - | `CreateOptionGroupResult` | `OptionGroupAlreadyExistsFault`, `OptionGroupQuotaExceededFault` | Creates a new option group. You can create up to 20 option groups. This command doesn't apply to RDS Custom. |
| `CreateTenantDatabase` | `-` | - | `DBInstanceIdentifier`, `TenantDBName`, `MasterUsername` | - | `CreateTenantDatabaseResult` | `DBInstanceNotFoundFault`, `InvalidDBInstanceStateFault`, `KMSKeyNotAccessibleFault`, `TenantDatabaseAlreadyExistsFault`, `TenantDatabaseQuotaExceededFault` | Creates a tenant database in a DB instance that uses the multi-tenant configuration. Only RDS for Oracle container database (CDB) instances are supported. |
| `DeleteBlueGreenDeployment` | `-` | - | `BlueGreenDeploymentIdentifier` | - | `DeleteBlueGreenDeploymentResponse` | `BlueGreenDeploymentNotFoundFault`, `InvalidBlueGreenDeploymentStateFault` | Deletes a blue/green deployment. For more information, see Using Amazon RDS Blue/Green Deployments for database updates in the Amazon RDS User Guide and Using Amazon RDS Blue/Green Deployments for database updates in ... |
| `DeleteCustomDBEngineVersion` | `-` | - | `Engine`, `EngineVersion` | - | `DBEngineVersion` | `CustomDBEngineVersionNotFoundFault`, `InvalidCustomDBEngineVersionStateFault` | Deletes a custom engine version. To run this command, make sure you meet the following prerequisites: The CEV must not be the default for RDS Custom. If it is, change the default before running this command. The CEV ... |
| `DeleteDBCluster` | `-` | - | `DBClusterIdentifier` | - | `DeleteDBClusterResult` | `DBClusterAutomatedBackupQuotaExceededFault`, `DBClusterNotFoundFault`, `DBClusterSnapshotAlreadyExistsFault`, `InvalidDBClusterSnapshotStateFault`, `InvalidDBClusterStateFault`, `InvalidGlobalClusterStateFault`, `KMSKeyNotAccessibleFault`, `SnapshotQuotaExceededFault` | The DeleteDBCluster action deletes a previously provisioned DB cluster. When you delete a DB cluster, all automated backups for that DB cluster are deleted and can't be recovered. Manual DB cluster snapshots of the s ... |
| `DeleteDBClusterAutomatedBackup` | `-` | - | `DbClusterResourceId` | - | `DeleteDBClusterAutomatedBackupResult` | `DBClusterAutomatedBackupNotFoundFault`, `InvalidDBClusterAutomatedBackupStateFault` | Deletes automated backups using the DbClusterResourceId value of the source DB cluster or the Amazon Resource Name (ARN) of the automated backups. |
| `DeleteDBClusterEndpoint` | `-` | - | `DBClusterEndpointIdentifier` | - | `DBClusterEndpoint` | `DBClusterEndpointNotFoundFault`, `InvalidDBClusterEndpointStateFault`, `InvalidDBClusterStateFault` | Deletes a custom endpoint and removes it from an Amazon Aurora DB cluster. This action only applies to Aurora DB clusters. |
| `DeleteDBClusterParameterGroup` | `-` | - | `DBClusterParameterGroupName` | - | `Unit` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Deletes a specified DB cluster parameter group. The DB cluster parameter group to be deleted can't be associated with any DB clusters. For more information on Amazon Aurora, see What is Amazon Aurora? in the Amazon A ... |
| `DeleteDBClusterSnapshot` | `-` | - | `DBClusterSnapshotIdentifier` | - | `DeleteDBClusterSnapshotResult` | `DBClusterSnapshotNotFoundFault`, `InvalidDBClusterSnapshotStateFault` | Deletes a DB cluster snapshot. If the snapshot is being copied, the copy operation is terminated. The DB cluster snapshot must be in the available state to be deleted. For more information on Amazon Aurora, see What ... |
| `DeleteDBInstance` | `-` | - | `DBInstanceIdentifier` | - | `DeleteDBInstanceResult` | `DBInstanceAutomatedBackupQuotaExceededFault`, `DBInstanceNotFoundFault`, `DBSnapshotAlreadyExistsFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `KMSKeyNotAccessibleFault`, `SnapshotQuotaExceededFault` | Deletes a previously provisioned DB instance. When you delete a DB instance, all automated backups for that instance are deleted and can't be recovered. However, manual DB snapshots of the DB instance aren't deleted. ... |
| `DeleteDBInstanceAutomatedBackup` | `-` | - | - | - | `DeleteDBInstanceAutomatedBackupResult` | `DBInstanceAutomatedBackupNotFoundFault`, `InvalidDBInstanceAutomatedBackupStateFault` | Deletes automated backups using the DbiResourceId value of the source DB instance or the Amazon Resource Name (ARN) of the automated backups. |
| `DeleteDBParameterGroup` | `-` | - | `DBParameterGroupName` | - | `Unit` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Deletes a specified DB parameter group. The DB parameter group to be deleted can't be associated with any DB instances. |
| `DeleteDBProxy` | `-` | - | `DBProxyName` | - | `DeleteDBProxyResponse` | `DBProxyNotFoundFault`, `InvalidDBProxyStateFault` | Deletes an existing DB proxy. |
| `DeleteDBProxyEndpoint` | `-` | - | `DBProxyEndpointName` | - | `DeleteDBProxyEndpointResponse` | `DBProxyEndpointNotFoundFault`, `InvalidDBProxyEndpointStateFault` | Deletes a DBProxyEndpoint . Doing so removes the ability to access the DB proxy using the endpoint that you defined. The endpoint that you delete might have provided capabilities such as read/write or read-only opera ... |
| `DeleteDBSecurityGroup` | `-` | - | `DBSecurityGroupName` | - | `Unit` | `DBSecurityGroupNotFoundFault`, `InvalidDBSecurityGroupStateFault` | Deletes a DB security group. The specified DB security group must not be associated with any DB instances. EC2-Classic was retired on August 15, 2022. If you haven't migrated from EC2-Classic to a VPC, we recommend t ... |
| `DeleteDBShardGroup` | `-` | - | `DBShardGroupIdentifier` | - | `DBShardGroup` | `DBShardGroupNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBShardGroupStateFault` | Deletes an Aurora Limitless Database DB shard group. |
| `DeleteDBSnapshot` | `-` | - | `DBSnapshotIdentifier` | - | `DeleteDBSnapshotResult` | `DBSnapshotNotFoundFault`, `InvalidDBSnapshotStateFault` | Deletes a DB snapshot. If the snapshot is being copied, the copy operation is terminated. The DB snapshot must be in the available state to be deleted. |
| `DeleteDBSubnetGroup` | `-` | - | `DBSubnetGroupName` | - | `Unit` | `DBSubnetGroupNotFoundFault`, `InvalidDBSubnetGroupStateFault`, `InvalidDBSubnetStateFault` | Deletes a DB subnet group. The specified database subnet group must not be associated with any DB instances. |
| `DeleteEventSubscription` | `-` | - | `SubscriptionName` | - | `DeleteEventSubscriptionResult` | `InvalidEventSubscriptionStateFault`, `SubscriptionNotFoundFault` | Deletes an RDS event notification subscription. |
| `DeleteGlobalCluster` | `-` | - | `GlobalClusterIdentifier` | - | `DeleteGlobalClusterResult` | `GlobalClusterNotFoundFault`, `InvalidGlobalClusterStateFault` | Deletes a global database cluster. The primary and secondary clusters must already be detached or destroyed first. This action only applies to Aurora DB clusters. |
| `DeleteIntegration` | `-` | - | `IntegrationIdentifier` | - | `Integration` | `IntegrationConflictOperationFault`, `IntegrationNotFoundFault`, `InvalidIntegrationStateFault` | Deletes a zero-ETL integration with Amazon Redshift. |
| `DeleteOptionGroup` | `-` | - | `OptionGroupName` | - | `Unit` | `InvalidOptionGroupStateFault`, `OptionGroupNotFoundFault` | Deletes an existing option group. |
| `DeleteTenantDatabase` | `-` | - | `DBInstanceIdentifier`, `TenantDBName` | - | `DeleteTenantDatabaseResult` | `DBInstanceNotFoundFault`, `DBSnapshotAlreadyExistsFault`, `InvalidDBInstanceStateFault`, `TenantDatabaseNotFoundFault` | Deletes a tenant database from your DB instance. This command only applies to RDS for Oracle container database (CDB) instances. You can't delete a tenant database when it is the only tenant in the DB instance. |
| `DeregisterDBProxyTargets` | `-` | - | `DBProxyName` | - | `DeregisterDBProxyTargetsResponse` | `DBProxyNotFoundFault`, `DBProxyTargetGroupNotFoundFault`, `DBProxyTargetNotFoundFault`, `InvalidDBProxyStateFault` | Remove the association between one or more DBProxyTarget data structures and a DBProxyTargetGroup . |
| `DescribeAccountAttributes` | `-` | - | - | - | `AccountAttributesMessage` | - | Lists all of the attributes for a customer account. The attributes include Amazon RDS quotas for the account, such as the number of DB instances allowed. The description for a quota includes the quota name, current u ... |
| `DescribeBlueGreenDeployments` | `-` | `paginated` | - | - | `DescribeBlueGreenDeploymentsResponse` | `BlueGreenDeploymentNotFoundFault` | Describes one or more blue/green deployments. For more information, see Using Amazon RDS Blue/Green Deployments for database updates in the Amazon RDS User Guide and Using Amazon RDS Blue/Green Deployments for databa ... |
| `DescribeCertificates` | `-` | `paginated` | - | - | `CertificateMessage` | `CertificateNotFoundFault` | Lists the set of certificate authority (CA) certificates provided by Amazon RDS for this Amazon Web Services account. For more information, see Using SSL/TLS to encrypt a connection to a DB instance in the Amazon RDS ... |
| `DescribeDBClusterAutomatedBackups` | `-` | `paginated` | - | - | `DBClusterAutomatedBackupMessage` | `DBClusterAutomatedBackupNotFoundFault` | Displays backups for both current and deleted DB clusters. For example, use this operation to find details about automated backups for previously deleted clusters. Current clusters are returned for both the DescribeD ... |
| `DescribeDBClusterBacktracks` | `-` | `paginated` | `DBClusterIdentifier` | - | `DBClusterBacktrackMessage` | `DBClusterBacktrackNotFoundFault`, `DBClusterNotFoundFault` | Returns information about backtracks for a DB cluster. For more information on Amazon Aurora, see What is Amazon Aurora? in the Amazon Aurora User Guide . This action only applies to Aurora MySQL DB clusters. |
| `DescribeDBClusterEndpoints` | `-` | `paginated` | - | - | `DBClusterEndpointMessage` | `DBClusterNotFoundFault` | Returns information about endpoints for an Amazon Aurora DB cluster. This action only applies to Aurora DB clusters. |
| `DescribeDBClusterParameterGroups` | `-` | `paginated` | - | - | `DBClusterParameterGroupsMessage` | `DBParameterGroupNotFoundFault` | Returns a list of DBClusterParameterGroup descriptions. If a DBClusterParameterGroupName parameter is specified, the list will contain only the description of the specified DB cluster parameter group. For more inform ... |
| `DescribeDBClusterParameters` | `-` | `paginated` | `DBClusterParameterGroupName` | - | `DBClusterParameterGroupDetails` | `DBParameterGroupNotFoundFault` | Returns the detailed parameter list for a particular DB cluster parameter group. For more information on Amazon Aurora, see What is Amazon Aurora? in the Amazon Aurora User Guide . For more information on Multi-AZ DB ... |
| `DescribeDBClusters` | `-` | `paginated` | - | - | `DBClusterMessage` | `DBClusterNotFoundFault` | Describes existing Amazon Aurora DB clusters and Multi-AZ DB clusters. This API supports pagination. For more information on Amazon Aurora DB clusters, see What is Amazon Aurora? in the Amazon Aurora User Guide . For ... |
| `DescribeDBClusterSnapshotAttributes` | `-` | - | `DBClusterSnapshotIdentifier` | - | `DescribeDBClusterSnapshotAttributesResult` | `DBClusterSnapshotNotFoundFault` | Returns a list of DB cluster snapshot attribute names and values for a manual DB cluster snapshot. When sharing snapshots with other Amazon Web Services accounts, DescribeDBClusterSnapshotAttributes returns the resto ... |
| `DescribeDBClusterSnapshots` | `-` | `paginated` | - | - | `DBClusterSnapshotMessage` | `DBClusterSnapshotNotFoundFault` | Returns information about DB cluster snapshots. This API action supports pagination. For more information on Amazon Aurora DB clusters, see What is Amazon Aurora? in the Amazon Aurora User Guide . For more informatio ... |
| `DescribeDBEngineVersions` | `-` | `paginated` | - | - | `DBEngineVersionMessage` | - | Describes the properties of specific versions of DB engines. |
| `DescribeDBInstanceAutomatedBackups` | `-` | `paginated` | - | - | `DBInstanceAutomatedBackupMessage` | `DBInstanceAutomatedBackupNotFoundFault` | Displays backups for both current and deleted instances. For example, use this operation to find details about automated backups for previously deleted instances. Current instances with retention periods greater than ... |
| `DescribeDBInstances` | `-` | `paginated` | - | - | `DBInstanceMessage` | `DBInstanceNotFoundFault` | Describes provisioned RDS instances. This API supports pagination. This operation can also return information for Amazon Neptune DB instances and Amazon DocumentDB instances. |
| `DescribeDBLogFiles` | `-` | `paginated` | `DBInstanceIdentifier` | - | `DescribeDBLogFilesResponse` | `DBInstanceNotFoundFault`, `DBInstanceNotReadyFault` | Returns a list of DB log files for the DB instance. This command doesn't apply to RDS Custom. |
| `DescribeDBMajorEngineVersions` | `-` | `paginated` | - | - | `DescribeDBMajorEngineVersionsResponse` | - | Describes the properties of specific major versions of DB engines. |
| `DescribeDBParameterGroups` | `-` | `paginated` | - | - | `DBParameterGroupsMessage` | `DBParameterGroupNotFoundFault` | Returns a list of DBParameterGroup descriptions. If a DBParameterGroupName is specified, the list will contain only the description of the specified DB parameter group. |
| `DescribeDBParameters` | `-` | `paginated` | `DBParameterGroupName` | - | `DBParameterGroupDetails` | `DBParameterGroupNotFoundFault` | Returns the detailed parameter list for a particular DB parameter group. |
| `DescribeDBProxies` | `-` | `paginated` | - | - | `DescribeDBProxiesResponse` | `DBProxyNotFoundFault` | Returns information about DB proxies. |
| `DescribeDBProxyEndpoints` | `-` | `paginated` | - | - | `DescribeDBProxyEndpointsResponse` | `DBProxyEndpointNotFoundFault`, `DBProxyNotFoundFault` | Returns information about DB proxy endpoints. |
| `DescribeDBProxyTargetGroups` | `-` | `paginated` | `DBProxyName` | - | `DescribeDBProxyTargetGroupsResponse` | `DBProxyNotFoundFault`, `DBProxyTargetGroupNotFoundFault`, `InvalidDBProxyStateFault` | Returns information about DB proxy target groups, represented by DBProxyTargetGroup data structures. |
| `DescribeDBProxyTargets` | `-` | `paginated` | `DBProxyName` | - | `DescribeDBProxyTargetsResponse` | `DBProxyNotFoundFault`, `DBProxyTargetGroupNotFoundFault`, `DBProxyTargetNotFoundFault`, `InvalidDBProxyStateFault` | Returns information about DBProxyTarget objects. This API supports pagination. |
| `DescribeDBRecommendations` | `-` | `paginated` | - | - | `DBRecommendationsMessage` | - | Describes the recommendations to resolve the issues for your DB instances, DB clusters, and DB parameter groups. |
| `DescribeDBSecurityGroups` | `-` | `paginated` | - | - | `DBSecurityGroupMessage` | `DBSecurityGroupNotFoundFault` | Returns a list of DBSecurityGroup descriptions. If a DBSecurityGroupName is specified, the list will contain only the descriptions of the specified DB security group. EC2-Classic was retired on August 15, 2022. If yo ... |
| `DescribeDBShardGroups` | `-` | - | - | - | `DescribeDBShardGroupsResponse` | `DBClusterNotFoundFault`, `DBShardGroupNotFoundFault` | Describes existing Aurora Limitless Database DB shard groups. |
| `DescribeDBSnapshotAttributes` | `-` | - | `DBSnapshotIdentifier` | - | `DescribeDBSnapshotAttributesResult` | `DBSnapshotNotFoundFault` | Returns a list of DB snapshot attribute names and values for a manual DB snapshot. When sharing snapshots with other Amazon Web Services accounts, DescribeDBSnapshotAttributes returns the restore attribute and a list ... |
| `DescribeDBSnapshots` | `-` | `paginated` | - | - | `DBSnapshotMessage` | `DBSnapshotNotFoundFault` | Returns information about DB snapshots. This API action supports pagination. |
| `DescribeDBSnapshotTenantDatabases` | `-` | `paginated` | - | - | `DBSnapshotTenantDatabasesMessage` | `DBSnapshotNotFoundFault` | Describes the tenant databases that exist in a DB snapshot. This command only applies to RDS for Oracle DB instances in the multi-tenant configuration. You can use this command to inspect the tenant databases within ... |
| `DescribeDBSubnetGroups` | `-` | `paginated` | - | - | `DBSubnetGroupMessage` | `DBSubnetGroupNotFoundFault` | Returns a list of DBSubnetGroup descriptions. If a DBSubnetGroupName is specified, the list will contain only the descriptions of the specified DBSubnetGroup. For an overview of CIDR ranges, go to the Wikipedia Tutor ... |
| `DescribeEngineDefaultClusterParameters` | `-` | `paginated` | `DBParameterGroupFamily` | - | `DescribeEngineDefaultClusterParametersResult` | - | Returns the default engine and system parameter information for the cluster database engine. For more information on Amazon Aurora, see What is Amazon Aurora? in the Amazon Aurora User Guide . |
| `DescribeEngineDefaultParameters` | `-` | `paginated` | `DBParameterGroupFamily` | - | `DescribeEngineDefaultParametersResult` | - | Returns the default engine and system parameter information for the specified database engine. |
| `DescribeEventCategories` | `-` | - | - | - | `EventCategoriesMessage` | - | Displays a list of categories for all event source types, or, if specified, for a specified source type. You can also see this list in the "Amazon RDS event categories and event messages" section of the Amazon RDS Us ... |
| `DescribeEvents` | `-` | `paginated` | - | - | `EventsMessage` | - | Returns events related to DB instances, DB clusters, DB parameter groups, DB security groups, DB snapshots, DB cluster snapshots, and RDS Proxies for the past 14 days. Events specific to a particular DB instance, DB ... |
| `DescribeEventSubscriptions` | `-` | `paginated` | - | - | `EventSubscriptionsMessage` | `SubscriptionNotFoundFault` | Lists all the subscription descriptions for a customer account. The description for a subscription includes SubscriptionName , SNSTopicARN , CustomerID , SourceType , SourceID , CreationTime , and Status . If you spe ... |
| `DescribeExportTasks` | `-` | `paginated` | - | - | `ExportTasksMessage` | `ExportTaskNotFoundFault` | Returns information about a snapshot or cluster export to Amazon S3. This API operation supports pagination. |
| `DescribeGlobalClusters` | `-` | `paginated` | - | - | `GlobalClustersMessage` | `GlobalClusterNotFoundFault` | Returns information about Aurora global database clusters. This API supports pagination. For more information on Amazon Aurora, see What is Amazon Aurora? in the Amazon Aurora User Guide . This action only applies to ... |
| `DescribeIntegrations` | `-` | `paginated` | - | - | `DescribeIntegrationsResponse` | `IntegrationNotFoundFault` | Describe one or more zero-ETL integrations with Amazon Redshift. |
| `DescribeOptionGroupOptions` | `-` | `paginated` | `EngineName` | - | `OptionGroupOptionsMessage` | - | Describes all available options for the specified engine. |
| `DescribeOptionGroups` | `-` | `paginated` | - | - | `OptionGroups` | `OptionGroupNotFoundFault` | Describes the available option groups. |
| `DescribeOrderableDBInstanceOptions` | `-` | `paginated` | `Engine` | - | `OrderableDBInstanceOptionsMessage` | - | Describes the orderable DB instance options for a specified DB engine. |
| `DescribePendingMaintenanceActions` | `-` | `paginated` | - | - | `PendingMaintenanceActionsMessage` | `ResourceNotFoundFault` | Returns a list of resources (for example, DB instances) that have at least one pending maintenance action. This API follows an eventual consistency model. This means that the result of the DescribePendingMaintenanceA ... |
| `DescribeReservedDBInstances` | `-` | `paginated` | - | - | `ReservedDBInstanceMessage` | `ReservedDBInstanceNotFoundFault` | Returns information about reserved DB instances for this account, or about a specified reserved DB instance. |
| `DescribeReservedDBInstancesOfferings` | `-` | `paginated` | - | - | `ReservedDBInstancesOfferingMessage` | `ReservedDBInstancesOfferingNotFoundFault` | Lists available reserved DB instance offerings. |
| `DescribeServerlessV2PlatformVersions` | `-` | `paginated` | - | - | `ServerlessV2PlatformVersionsMessage` | - | Describes the properties of specific platform versions for Aurora Serverless v2. |
| `DescribeSourceRegions` | `-` | `paginated` | - | - | `SourceRegionMessage` | - | Returns a list of the source Amazon Web Services Regions where the current Amazon Web Services Region can create a read replica, copy a DB snapshot from, or replicate automated backups from. Use this operation to det ... |
| `DescribeTenantDatabases` | `-` | `paginated` | - | - | `TenantDatabasesMessage` | `DBInstanceNotFoundFault` | Describes the tenant databases in a DB instance that uses the multi-tenant configuration. Only RDS for Oracle CDB instances are supported. |
| `DescribeValidDBInstanceModifications` | `-` | - | `DBInstanceIdentifier` | - | `DescribeValidDBInstanceModificationsResult` | `DBInstanceNotFoundFault`, `InvalidDBInstanceStateFault` | You can call DescribeValidDBInstanceModifications to learn what modifications you can make to your DB instance. You can use this information when you call ModifyDBInstance . This command doesn't apply to RDS Custom. |
| `DisableHttpEndpoint` | `-` | - | `ResourceArn` | - | `DisableHttpEndpointResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Disables the HTTP endpoint for the specified DB cluster. Disabling this endpoint disables RDS Data API. For more information, see Using RDS Data API in the Amazon Aurora User Guide . This operation applies only to Au ... |
| `DownloadDBLogFilePortion` | `-` | `paginated` | `DBInstanceIdentifier`, `LogFileName` | - | `DownloadDBLogFilePortionDetails` | `DBInstanceNotFoundFault`, `DBInstanceNotReadyFault`, `DBLogFileNotFoundFault` | Downloads all or a portion of the specified log file, up to 1 MB in size. This command doesn't apply to RDS Custom. This operation uses resources on database instances. Because of this, we recommend publishing databa ... |
| `EnableHttpEndpoint` | `-` | - | `ResourceArn` | - | `EnableHttpEndpointResponse` | `InvalidResourceStateFault`, `ResourceNotFoundFault` | Enables the HTTP endpoint for the DB cluster. By default, the HTTP endpoint isn't enabled. When enabled, this endpoint provides a connectionless web service API (RDS Data API) for running SQL queries on the Aurora DB ... |
| `FailoverDBCluster` | `-` | - | `DBClusterIdentifier` | - | `FailoverDBClusterResult` | `DBClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault` | Forces a failover for a DB cluster. For an Aurora DB cluster, failover for a DB cluster promotes one of the Aurora Replicas (read-only instances) in the DB cluster to be the primary DB instance (the cluster writer). ... |
| `FailoverGlobalCluster` | `-` | - | `GlobalClusterIdentifier`, `TargetDbClusterIdentifier` | - | `FailoverGlobalClusterResult` | `DBClusterNotFoundFault`, `GlobalClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidGlobalClusterStateFault` | Promotes the specified secondary DB cluster to be the primary DB cluster in the global database cluster to fail over or switch over a global database. Switchover operations were previously called "managed planned fai ... |
| `ListTagsForResource` | `-` | - | `ResourceName` | - | `TagListMessage` | `BlueGreenDeploymentNotFoundFault`, `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `DBProxyEndpointNotFoundFault`, `DBProxyNotFoundFault`, `DBProxyTargetGroupNotFoundFault`, `DBShardGroupNotFoundFault`, `DBSnapshotNotFoundFault`, `DBSnapshotTenantDatabaseNotFoundFault`, `IntegrationNotFoundFault`, `TenantDatabaseNotFoundFault` | Lists all tags on an Amazon RDS resource. For an overview on tagging an Amazon RDS resource, see Tagging Amazon RDS Resources in the Amazon RDS User Guide or Tagging Amazon Aurora and Amazon RDS Resources in the Amaz ... |
| `ModifyActivityStream` | `-` | - | - | - | `ModifyActivityStreamResponse` | `DBInstanceNotFoundFault`, `InvalidDBInstanceStateFault`, `ResourceNotFoundFault` | Changes the audit policy state of a database activity stream to either locked (default) or unlocked. A locked policy is read-only, whereas an unlocked policy is read/write. If your activity stream is started and lock ... |
| `ModifyCertificates` | `-` | - | - | - | `ModifyCertificatesResult` | `CertificateNotFoundFault` | Override the system-default Secure Sockets Layer/Transport Layer Security (SSL/TLS) certificate for Amazon RDS for new DB instances, or remove the override. By using this operation, you can specify an RDS-approved SS ... |
| `ModifyCurrentDBClusterCapacity` | `-` | - | `DBClusterIdentifier` | - | `DBClusterCapacityInfo` | `DBClusterNotFoundFault`, `InvalidDBClusterCapacityFault`, `InvalidDBClusterStateFault` | Set the capacity of an Aurora Serverless v1 DB cluster to a specific value. Aurora Serverless v1 scales seamlessly based on the workload on the DB cluster. In some cases, the capacity might not scale fast enough to m ... |
| `ModifyCustomDBEngineVersion` | `-` | - | `Engine`, `EngineVersion` | - | `DBEngineVersion` | `CustomDBEngineVersionNotFoundFault`, `InvalidCustomDBEngineVersionStateFault` | Modifies the status of a custom engine version (CEV). You can find CEVs to modify by calling DescribeDBEngineVersions . The MediaImport service that imports files from Amazon S3 to create CEVs isn't integrated with A ... |
| `ModifyDBCluster` | `-` | - | `DBClusterIdentifier` | - | `ModifyDBClusterResult` | `DBClusterAlreadyExistsFault`, `DBClusterNotFoundFault`, `DBClusterParameterGroupNotFoundFault`, `DBInstanceAlreadyExistsFault`, `DBParameterGroupNotFoundFault`, `DBSubnetGroupNotFoundFault`, `DomainNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `InvalidDBSecurityGroupStateFault`, `InvalidDBSubnetGroupStateFault`, `InvalidGlobalClusterStateFault`, `InvalidSubnet`, `InvalidVPCNetworkStateFault`, `KMSKeyNotAccessibleFault`, `NetworkTypeNotSupported`, `OptionGroupNotFoundFault`, `StorageQuotaExceededFault`, `StorageTypeNotAvailableFault`, `StorageTypeNotSupportedFault`, `VpcEncryptionControlViolationException` | Modifies the settings of an Amazon Aurora DB cluster or a Multi-AZ DB cluster. You can change one or more settings by specifying these parameters and the new values in the request. For more information on Amazon Auro ... |
| `ModifyDBClusterEndpoint` | `-` | - | `DBClusterEndpointIdentifier` | - | `DBClusterEndpoint` | `DBClusterEndpointNotFoundFault`, `DBInstanceNotFoundFault`, `InvalidDBClusterEndpointStateFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault` | Modifies the properties of an endpoint in an Amazon Aurora DB cluster. This operation only applies to Aurora DB clusters. |
| `ModifyDBClusterParameterGroup` | `-` | - | `DBClusterParameterGroupName`, `Parameters` | - | `DBClusterParameterGroupNameMessage` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Modifies the parameters of a DB cluster parameter group. To modify more than one parameter, submit a list of the following: ParameterName , ParameterValue , and ApplyMethod . A maximum of 20 parameters can be modifie ... |
| `ModifyDBClusterSnapshotAttribute` | `-` | - | `DBClusterSnapshotIdentifier`, `AttributeName` | - | `ModifyDBClusterSnapshotAttributeResult` | `DBClusterSnapshotNotFoundFault`, `InvalidDBClusterSnapshotStateFault`, `SharedSnapshotQuotaExceededFault` | Adds an attribute and values to, or removes an attribute and values from, a manual DB cluster snapshot. To share a manual DB cluster snapshot with other Amazon Web Services accounts, specify restore as the AttributeN ... |
| `ModifyDBInstance` | `-` | - | `DBInstanceIdentifier` | - | `ModifyDBInstanceResult` | `AuthorizationNotFoundFault`, `BackupPolicyNotFoundFault`, `CertificateNotFoundFault`, `DBInstanceAlreadyExistsFault`, `DBInstanceNotFoundFault`, `DBParameterGroupNotFoundFault`, `DBSecurityGroupNotFoundFault`, `DBUpgradeDependencyFailureFault`, `DomainNotFoundFault`, `InsufficientDBInstanceCapacityFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `InvalidDBSecurityGroupStateFault`, `InvalidVPCNetworkStateFault`, `KMSKeyNotAccessibleFault`, `NetworkTypeNotSupported`, `OptionGroupNotFoundFault`, `ProvisionedIopsNotAvailableInAZFault`, `StorageQuotaExceededFault`, `StorageTypeNotSupportedFault`, `TenantDatabaseQuotaExceededFault`, `VpcEncryptionControlViolationException` | Modifies settings for a DB instance. You can change one or more database configuration parameters by specifying these parameters and the new values in the request. To learn what modifications you can make to your DB ... |
| `ModifyDBParameterGroup` | `-` | - | `DBParameterGroupName`, `Parameters` | - | `DBParameterGroupNameMessage` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Modifies the parameters of a DB parameter group. To modify more than one parameter, submit a list of the following: ParameterName , ParameterValue , and ApplyMethod . A maximum of 20 parameters can be modified in a s ... |
| `ModifyDBProxy` | `-` | - | `DBProxyName` | - | `ModifyDBProxyResponse` | `DBProxyAlreadyExistsFault`, `DBProxyNotFoundFault`, `InvalidDBProxyStateFault` | Changes the settings for an existing DB proxy. |
| `ModifyDBProxyEndpoint` | `-` | - | `DBProxyEndpointName` | - | `ModifyDBProxyEndpointResponse` | `DBProxyEndpointAlreadyExistsFault`, `DBProxyEndpointNotFoundFault`, `InvalidDBProxyEndpointStateFault`, `InvalidDBProxyStateFault` | Changes the settings for an existing DB proxy endpoint. |
| `ModifyDBProxyTargetGroup` | `-` | - | `TargetGroupName`, `DBProxyName` | - | `ModifyDBProxyTargetGroupResponse` | `DBProxyNotFoundFault`, `DBProxyTargetGroupNotFoundFault`, `InvalidDBProxyStateFault` | Modifies the properties of a DBProxyTargetGroup . |
| `ModifyDBRecommendation` | `-` | - | `RecommendationId` | - | `DBRecommendationMessage` | - | Updates the recommendation status and recommended action status for the specified recommendation. |
| `ModifyDBShardGroup` | `-` | - | `DBShardGroupIdentifier` | - | `DBShardGroup` | `DBShardGroupAlreadyExistsFault`, `DBShardGroupNotFoundFault`, `InvalidDBClusterStateFault` | Modifies the settings of an Aurora Limitless Database DB shard group. You can change one or more settings by specifying these parameters and the new values in the request. |
| `ModifyDBSnapshot` | `-` | - | `DBSnapshotIdentifier` | - | `ModifyDBSnapshotResult` | `DBSnapshotNotFoundFault`, `InvalidDBSnapshotStateFault`, `KMSKeyNotAccessibleFault` | Updates a manual DB snapshot with a new engine version. The snapshot can be encrypted or unencrypted, but not shared or public. Amazon RDS supports upgrading DB snapshots for MariaDB, MySQL, PostgreSQL, and Oracle. T ... |
| `ModifyDBSnapshotAttribute` | `-` | - | `DBSnapshotIdentifier`, `AttributeName` | - | `ModifyDBSnapshotAttributeResult` | `DBSnapshotNotFoundFault`, `InvalidDBSnapshotStateFault`, `SharedSnapshotQuotaExceededFault` | Adds an attribute and values to, or removes an attribute and values from, a manual DB snapshot. To share a manual DB snapshot with other Amazon Web Services accounts, specify restore as the AttributeName and use the ... |
| `ModifyDBSubnetGroup` | `-` | - | `DBSubnetGroupName`, `SubnetIds` | - | `ModifyDBSubnetGroupResult` | `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotFoundFault`, `DBSubnetQuotaExceededFault`, `InvalidDBSubnetGroupStateFault`, `InvalidSubnet`, `SubnetAlreadyInUse` | Modifies an existing DB subnet group. DB subnet groups must contain at least one subnet in at least two AZs in the Amazon Web Services Region. |
| `ModifyEventSubscription` | `-` | - | `SubscriptionName` | - | `ModifyEventSubscriptionResult` | `EventSubscriptionQuotaExceededFault`, `SNSInvalidTopicFault`, `SNSNoAuthorizationFault`, `SNSTopicArnNotFoundFault`, `SubscriptionCategoryNotFoundFault`, `SubscriptionNotFoundFault` | Modifies an existing RDS event notification subscription. You can't modify the source identifiers using this call. To change source identifiers for a subscription, use the AddSourceIdentifierToSubscription and Remove ... |
| `ModifyGlobalCluster` | `-` | - | `GlobalClusterIdentifier` | - | `ModifyGlobalClusterResult` | `GlobalClusterAlreadyExistsFault`, `GlobalClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `InvalidGlobalClusterStateFault` | Modifies a setting for an Amazon Aurora global database cluster. You can change one or more database configuration parameters by specifying these parameters and the new values in the request. For more information on ... |
| `ModifyIntegration` | `-` | - | `IntegrationIdentifier` | - | `Integration` | `IntegrationConflictOperationFault`, `IntegrationNotFoundFault`, `InvalidIntegrationStateFault` | Modifies a zero-ETL integration with Amazon Redshift. |
| `ModifyOptionGroup` | `-` | - | `OptionGroupName` | - | `ModifyOptionGroupResult` | `InvalidOptionGroupStateFault`, `OptionGroupNotFoundFault` | Modifies an existing option group. |
| `ModifyTenantDatabase` | `-` | - | `DBInstanceIdentifier`, `TenantDBName` | - | `ModifyTenantDatabaseResult` | `DBInstanceNotFoundFault`, `InvalidDBInstanceStateFault`, `KMSKeyNotAccessibleFault`, `TenantDatabaseAlreadyExistsFault`, `TenantDatabaseNotFoundFault` | Modifies an existing tenant database in a DB instance. You can change the tenant database name or the master user password. This operation is supported only for RDS for Oracle CDB instances using the multi-tenant con ... |
| `PromoteReadReplica` | `-` | - | `DBInstanceIdentifier` | - | `PromoteReadReplicaResult` | `DBInstanceNotFoundFault`, `InvalidDBInstanceStateFault` | Promotes a read replica DB instance to a standalone DB instance. Backup duration is a function of the amount of changes to the database since the previous backup. If you plan to promote a read replica to a standalone ... |
| `PromoteReadReplicaDBCluster` | `-` | - | `DBClusterIdentifier` | - | `PromoteReadReplicaDBClusterResult` | `DBClusterNotFoundFault`, `InvalidDBClusterStateFault` | Promotes a read replica DB cluster to a standalone DB cluster. |
| `PurchaseReservedDBInstancesOffering` | `-` | - | `ReservedDBInstancesOfferingId` | - | `PurchaseReservedDBInstancesOfferingResult` | `ReservedDBInstanceAlreadyExistsFault`, `ReservedDBInstanceQuotaExceededFault`, `ReservedDBInstancesOfferingNotFoundFault` | Purchases a reserved DB instance offering. |
| `RebootDBCluster` | `-` | - | `DBClusterIdentifier` | - | `RebootDBClusterResult` | `DBClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault` | You might need to reboot your DB cluster, usually for maintenance reasons. For example, if you make certain modifications, or if you change the DB cluster parameter group associated with the DB cluster, reboot the DB ... |
| `RebootDBInstance` | `-` | - | `DBInstanceIdentifier` | - | `RebootDBInstanceResult` | `DBInstanceNotFoundFault`, `InvalidDBInstanceStateFault`, `KMSKeyNotAccessibleFault` | You might need to reboot your DB instance, usually for maintenance reasons. For example, if you make certain modifications, or if you change the DB parameter group associated with the DB instance, you must reboot the ... |
| `RebootDBShardGroup` | `-` | - | `DBShardGroupIdentifier` | - | `DBShardGroup` | `DBShardGroupNotFoundFault`, `InvalidDBShardGroupStateFault` | You might need to reboot your DB shard group, usually for maintenance reasons. For example, if you make certain modifications, reboot the DB shard group for the changes to take effect. This operation applies only to ... |
| `RegisterDBProxyTargets` | `-` | - | `DBProxyName` | - | `RegisterDBProxyTargetsResponse` | `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `DBProxyNotFoundFault`, `DBProxyTargetAlreadyRegisteredFault`, `DBProxyTargetGroupNotFoundFault`, `InsufficientAvailableIPsInSubnetFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `InvalidDBProxyStateFault` | Associate one or more DBProxyTarget data structures with a DBProxyTargetGroup . |
| `RemoveFromGlobalCluster` | `-` | - | `GlobalClusterIdentifier`, `DbClusterIdentifier` | - | `RemoveFromGlobalClusterResult` | `DBClusterNotFoundFault`, `GlobalClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidGlobalClusterStateFault` | Detaches an Aurora secondary cluster from an Aurora global database cluster. The cluster becomes a standalone cluster with read-write capability instead of being read-only and receiving data from a primary cluster in ... |
| `RemoveRoleFromDBCluster` | `-` | - | `DBClusterIdentifier`, `RoleArn` | - | `Unit` | `DBClusterNotFoundFault`, `DBClusterRoleNotFoundFault`, `InvalidDBClusterStateFault` | Removes the asssociation of an Amazon Web Services Identity and Access Management (IAM) role from a DB cluster. For more information on Amazon Aurora DB clusters, see What is Amazon Aurora? in the Amazon Aurora User ... |
| `RemoveRoleFromDBInstance` | `-` | - | `DBInstanceIdentifier`, `RoleArn`, `FeatureName` | - | `Unit` | `DBInstanceNotFoundFault`, `DBInstanceRoleNotFoundFault`, `InvalidDBInstanceStateFault` | Disassociates an Amazon Web Services Identity and Access Management (IAM) role from a DB instance. |
| `RemoveSourceIdentifierFromSubscription` | `-` | - | `SubscriptionName`, `SourceIdentifier` | - | `RemoveSourceIdentifierFromSubscriptionResult` | `SourceNotFoundFault`, `SubscriptionNotFoundFault` | Removes a source identifier from an existing RDS event notification subscription. |
| `RemoveTagsFromResource` | `-` | - | `ResourceName`, `TagKeys` | - | `Unit` | `BlueGreenDeploymentNotFoundFault`, `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `DBProxyEndpointNotFoundFault`, `DBProxyNotFoundFault`, `DBProxyTargetGroupNotFoundFault`, `DBShardGroupNotFoundFault`, `DBSnapshotNotFoundFault`, `DBSnapshotTenantDatabaseNotFoundFault`, `IntegrationNotFoundFault`, `InvalidDBClusterEndpointStateFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `TenantDatabaseNotFoundFault` | Removes metadata tags from an Amazon RDS resource. For an overview on tagging an Amazon RDS resource, see Tagging Amazon RDS Resources in the Amazon RDS User Guide or Tagging Amazon Aurora and Amazon RDS Resources in ... |
| `ResetDBClusterParameterGroup` | `-` | - | `DBClusterParameterGroupName` | - | `DBClusterParameterGroupNameMessage` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Modifies the parameters of a DB cluster parameter group to the default value. To reset specific parameters submit a list of the following: ParameterName and ApplyMethod . To reset the entire DB cluster parameter grou ... |
| `ResetDBParameterGroup` | `-` | - | `DBParameterGroupName` | - | `DBParameterGroupNameMessage` | `DBParameterGroupNotFoundFault`, `InvalidDBParameterGroupStateFault` | Modifies the parameters of a DB parameter group to the engine/system default value. To reset specific parameters, provide a list of the following: ParameterName and ApplyMethod . To reset the entire DB parameter grou ... |
| `RestoreDBClusterFromS3` | `-` | - | `DBClusterIdentifier`, `Engine`, `MasterUsername`, `SourceEngine`, `SourceEngineVersion`, `S3BucketName`, `S3IngestionRoleArn` | - | `RestoreDBClusterFromS3Result` | `DBClusterAlreadyExistsFault`, `DBClusterNotFoundFault`, `DBClusterParameterGroupNotFoundFault`, `DBClusterQuotaExceededFault`, `DBSubnetGroupNotFoundFault`, `DomainNotFoundFault`, `InsufficientStorageClusterCapacityFault`, `InvalidDBClusterStateFault`, `InvalidDBSubnetGroupStateFault`, `InvalidS3BucketFault`, `InvalidSubnet`, `InvalidVPCNetworkStateFault`, `KMSKeyNotAccessibleFault`, `NetworkTypeNotSupported`, `StorageQuotaExceededFault`, `StorageTypeNotSupportedFault` | Creates an Amazon Aurora DB cluster from MySQL data stored in an Amazon S3 bucket. Amazon RDS must be authorized to access the Amazon S3 bucket and the data must be created using the Percona XtraBackup utility as des ... |
| `RestoreDBClusterFromSnapshot` | `-` | - | `DBClusterIdentifier`, `SnapshotIdentifier`, `Engine` | - | `RestoreDBClusterFromSnapshotResult` | `DBClusterAlreadyExistsFault`, `DBClusterParameterGroupNotFoundFault`, `DBClusterQuotaExceededFault`, `DBClusterSnapshotNotFoundFault`, `DBSnapshotNotFoundFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotFoundFault`, `DomainNotFoundFault`, `InsufficientDBClusterCapacityFault`, `InsufficientDBInstanceCapacityFault`, `InsufficientStorageClusterCapacityFault`, `InvalidDBClusterSnapshotStateFault`, `InvalidDBInstanceStateFault`, `InvalidDBSnapshotStateFault`, `InvalidRestoreFault`, `InvalidSubnet`, `InvalidVPCNetworkStateFault`, `KMSKeyNotAccessibleFault`, `NetworkTypeNotSupported`, `OptionGroupNotFoundFault`, `StorageQuotaExceededFault`, `StorageTypeNotSupportedFault`, `VpcEncryptionControlViolationException` | Creates a new DB cluster from a DB snapshot or DB cluster snapshot. The target DB cluster is created from the source snapshot with a default configuration. If you don't specify a security group, the new DB cluster is ... |
| `RestoreDBClusterToPointInTime` | `-` | - | `DBClusterIdentifier` | - | `RestoreDBClusterToPointInTimeResult` | `DBClusterAlreadyExistsFault`, `DBClusterAutomatedBackupNotFoundFault`, `DBClusterNotFoundFault`, `DBClusterParameterGroupNotFoundFault`, `DBClusterQuotaExceededFault`, `DBClusterSnapshotNotFoundFault`, `DBSubnetGroupNotFoundFault`, `DomainNotFoundFault`, `InsufficientDBClusterCapacityFault`, `InsufficientDBInstanceCapacityFault`, `InsufficientStorageClusterCapacityFault`, `InvalidDBClusterSnapshotStateFault`, `InvalidDBClusterStateFault`, `InvalidDBSnapshotStateFault`, `InvalidRestoreFault`, `InvalidSubnet`, `InvalidVPCNetworkStateFault`, `KMSKeyNotAccessibleFault`, `NetworkTypeNotSupported`, `OptionGroupNotFoundFault`, `StorageQuotaExceededFault`, `StorageTypeNotSupportedFault`, `VpcEncryptionControlViolationException` | Restores a DB cluster to an arbitrary point in time. Users can restore to any point in time before LatestRestorableTime for up to BackupRetentionPeriod days. The target DB cluster is created from the source DB cluste ... |
| `RestoreDBInstanceFromDBSnapshot` | `-` | - | `DBInstanceIdentifier` | - | `RestoreDBInstanceFromDBSnapshotResult` | `AuthorizationNotFoundFault`, `BackupPolicyNotFoundFault`, `CertificateNotFoundFault`, `DBClusterSnapshotNotFoundFault`, `DBInstanceAlreadyExistsFault`, `DBParameterGroupNotFoundFault`, `DBSecurityGroupNotFoundFault`, `DBSnapshotNotFoundFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotFoundFault`, `DomainNotFoundFault`, `InstanceQuotaExceededFault`, `InsufficientDBInstanceCapacityFault`, `InvalidDBSnapshotStateFault`, `InvalidRestoreFault`, `InvalidSubnet`, `InvalidVPCNetworkStateFault`, `KMSKeyNotAccessibleFault`, `NetworkTypeNotSupported`, `OptionGroupNotFoundFault`, `ProvisionedIopsNotAvailableInAZFault`, `StorageQuotaExceededFault`, `StorageTypeNotSupportedFault`, `TenantDatabaseQuotaExceededFault`, `VpcEncryptionControlViolationException` | Creates a new DB instance from a DB snapshot. The target database is created from the source database restore point with most of the source's original configuration, including the default security group and DB parame ... |
| `RestoreDBInstanceFromS3` | `-` | - | `DBInstanceIdentifier`, `DBInstanceClass`, `Engine`, `SourceEngine`, `SourceEngineVersion`, `S3BucketName`, `S3IngestionRoleArn` | - | `RestoreDBInstanceFromS3Result` | `AuthorizationNotFoundFault`, `BackupPolicyNotFoundFault`, `CertificateNotFoundFault`, `DBInstanceAlreadyExistsFault`, `DBParameterGroupNotFoundFault`, `DBSecurityGroupNotFoundFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotFoundFault`, `InstanceQuotaExceededFault`, `InsufficientDBInstanceCapacityFault`, `InvalidS3BucketFault`, `InvalidSubnet`, `InvalidVPCNetworkStateFault`, `KMSKeyNotAccessibleFault`, `NetworkTypeNotSupported`, `OptionGroupNotFoundFault`, `ProvisionedIopsNotAvailableInAZFault`, `StorageQuotaExceededFault`, `StorageTypeNotSupportedFault`, `VpcEncryptionControlViolationException` | Amazon Relational Database Service (Amazon RDS) supports importing MySQL databases by using backup files. You can create a backup of your on-premises database, store it on Amazon Simple Storage Service (Amazon S3), a ... |
| `RestoreDBInstanceToPointInTime` | `-` | - | `TargetDBInstanceIdentifier` | - | `RestoreDBInstanceToPointInTimeResult` | `AuthorizationNotFoundFault`, `BackupPolicyNotFoundFault`, `CertificateNotFoundFault`, `DBInstanceAlreadyExistsFault`, `DBInstanceAutomatedBackupNotFoundFault`, `DBInstanceNotFoundFault`, `DBParameterGroupNotFoundFault`, `DBSecurityGroupNotFoundFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotFoundFault`, `DomainNotFoundFault`, `InstanceQuotaExceededFault`, `InsufficientDBInstanceCapacityFault`, `InvalidDBInstanceStateFault`, `InvalidRestoreFault`, `InvalidSubnet`, `InvalidVPCNetworkStateFault`, `KMSKeyNotAccessibleFault`, `NetworkTypeNotSupported`, `OptionGroupNotFoundFault`, `PointInTimeRestoreNotEnabledFault`, `ProvisionedIopsNotAvailableInAZFault`, `StorageQuotaExceededFault`, `StorageTypeNotSupportedFault`, `TenantDatabaseQuotaExceededFault`, `VpcEncryptionControlViolationException` | Restores a DB instance to an arbitrary point in time. You can restore to any point in time before the time identified by the LatestRestorableTime property. You can restore to a point up to the number of days specifie ... |
| `RevokeDBSecurityGroupIngress` | `-` | - | `DBSecurityGroupName` | - | `RevokeDBSecurityGroupIngressResult` | `AuthorizationNotFoundFault`, `DBSecurityGroupNotFoundFault`, `InvalidDBSecurityGroupStateFault` | Revokes ingress from a DBSecurityGroup for previously authorized IP ranges or EC2 or VPC security groups. Required parameters for this API are one of CIDRIP, EC2SecurityGroupId for VPC, or (EC2SecurityGroupOwnerId an ... |
| `StartActivityStream` | `-` | - | `ResourceArn`, `Mode`, `KmsKeyId` | - | `StartActivityStreamResponse` | `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `KMSKeyNotAccessibleFault`, `ResourceNotFoundFault` | Starts a database activity stream to monitor activity on the database. For more information, see Monitoring Amazon Aurora with Database Activity Streams in the Amazon Aurora User Guide or Monitoring Amazon RDS with D ... |
| `StartDBCluster` | `-` | - | `DBClusterIdentifier` | - | `StartDBClusterResult` | `DBClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `InvalidDBShardGroupStateFault`, `KMSKeyNotAccessibleFault`, `VpcEncryptionControlViolationException` | Starts an Amazon Aurora DB cluster that was stopped using the Amazon Web Services console, the stop-db-cluster CLI command, or the StopDBCluster operation. For more information, see Stopping and Starting an Aurora Cl ... |
| `StartDBInstance` | `-` | - | `DBInstanceIdentifier` | - | `StartDBInstanceResult` | `AuthorizationNotFoundFault`, `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `DBSubnetGroupDoesNotCoverEnoughAZs`, `DBSubnetGroupNotFoundFault`, `InsufficientDBInstanceCapacityFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `InvalidSubnet`, `InvalidVPCNetworkStateFault`, `KMSKeyNotAccessibleFault`, `VpcEncryptionControlViolationException` | Starts an Amazon RDS DB instance that was stopped using the Amazon Web Services console, the stop-db-instance CLI command, or the StopDBInstance operation. For more information, see Starting an Amazon RDS DB instance ... |
| `StartDBInstanceAutomatedBackupsReplication` | `-` | - | `SourceDBInstanceArn` | - | `StartDBInstanceAutomatedBackupsReplicationResult` | `DBInstanceAutomatedBackupQuotaExceededFault`, `DBInstanceNotFoundFault`, `InvalidDBInstanceAutomatedBackupStateFault`, `InvalidDBInstanceStateFault`, `KMSKeyNotAccessibleFault`, `StorageTypeNotSupportedFault` | Enables replication of automated backups to a different Amazon Web Services Region. This command doesn't apply to RDS Custom. For more information, see Replicating Automated Backups to Another Amazon Web Services Reg ... |
| `StartExportTask` | `-` | - | `ExportTaskIdentifier`, `SourceArn`, `S3BucketName`, `IamRoleArn`, `KmsKeyId` | - | `ExportTask` | `DBClusterNotFoundFault`, `DBClusterSnapshotNotFoundFault`, `DBSnapshotNotFoundFault`, `ExportTaskAlreadyExistsFault`, `IamRoleMissingPermissionsFault`, `IamRoleNotFoundFault`, `InvalidExportOnlyFault`, `InvalidExportSourceStateFault`, `InvalidS3BucketFault`, `KMSKeyNotAccessibleFault` | Starts an export of DB snapshot or DB cluster data to Amazon S3. The provided IAM role must have access to the S3 bucket. You can't export snapshot data from RDS Custom DB instances. For more information, see Support ... |
| `StopActivityStream` | `-` | - | `ResourceArn` | - | `StopActivityStreamResponse` | `DBClusterNotFoundFault`, `DBInstanceNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `ResourceNotFoundFault` | Stops a database activity stream that was started using the Amazon Web Services console, the start-activity-stream CLI command, or the StartActivityStream operation. For more information, see Monitoring Amazon Aurora ... |
| `StopDBCluster` | `-` | - | `DBClusterIdentifier` | - | `StopDBClusterResult` | `DBClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `InvalidDBShardGroupStateFault` | Stops an Amazon Aurora DB cluster. When you stop a DB cluster, Aurora retains the DB cluster's metadata, including its endpoints and DB parameter groups. Aurora also retains the transaction logs so you can do a point ... |
| `StopDBInstance` | `-` | - | `DBInstanceIdentifier` | - | `StopDBInstanceResult` | `DBInstanceNotFoundFault`, `DBSnapshotAlreadyExistsFault`, `InvalidDBClusterStateFault`, `InvalidDBInstanceStateFault`, `SnapshotQuotaExceededFault` | Stops an Amazon RDS DB instance temporarily. When you stop a DB instance, Amazon RDS retains the DB instance's metadata, including its endpoint, DB parameter group, and option group membership. Amazon RDS also retain ... |
| `StopDBInstanceAutomatedBackupsReplication` | `-` | - | `SourceDBInstanceArn` | - | `StopDBInstanceAutomatedBackupsReplicationResult` | `DBInstanceNotFoundFault`, `InvalidDBInstanceStateFault` | Stops automated backup replication for a DB instance. This command doesn't apply to RDS Custom, Aurora MySQL, and Aurora PostgreSQL. For more information, see Replicating Automated Backups to Another Amazon Web Servi ... |
| `SwitchoverBlueGreenDeployment` | `-` | - | `BlueGreenDeploymentIdentifier` | - | `SwitchoverBlueGreenDeploymentResponse` | `BlueGreenDeploymentNotFoundFault`, `InvalidBlueGreenDeploymentStateFault` | Switches over a blue/green deployment. Before you switch over, production traffic is routed to the databases in the blue environment. After you switch over, production traffic is routed to the databases in the green ... |
| `SwitchoverGlobalCluster` | `-` | - | `GlobalClusterIdentifier`, `TargetDbClusterIdentifier` | - | `SwitchoverGlobalClusterResult` | `DBClusterNotFoundFault`, `GlobalClusterNotFoundFault`, `InvalidDBClusterStateFault`, `InvalidGlobalClusterStateFault` | Switches over the specified secondary DB cluster to be the new primary DB cluster in the global database cluster. Switchover operations were previously called "managed planned failovers." Aurora promotes the specifie ... |
| `SwitchoverReadReplica` | `-` | - | `DBInstanceIdentifier` | - | `SwitchoverReadReplicaResult` | `DBInstanceNotFoundFault`, `InvalidDBInstanceStateFault` | Switches over an Oracle standby database in an Oracle Data Guard environment, making it the new primary database. Issue this command in the Region that hosts the current standby database. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AuthorizationAlreadyExistsFault` | `structure` | message | The specified CIDR IP range or Amazon EC2 security group is already authorized for the specified DB security group. |
| `AuthorizationNotFoundFault` | `structure` | message | The specified CIDR IP range or Amazon EC2 security group might not be authorized for the specified DB security group. Or, RDS might not be authorized to per ... |
| `AuthorizationQuotaExceededFault` | `structure` | message | The DB security group authorization quota has been reached. |
| `BackupPolicyNotFoundFault` | `structure` | message | - |
| `BlueGreenDeploymentAlreadyExistsFault` | `structure` | message | A blue/green deployment with the specified name already exists. |
| `BlueGreenDeploymentNotFoundFault` | `structure` | message | BlueGreenDeploymentIdentifier doesn't refer to an existing blue/green deployment. |
| `CertificateNotFoundFault` | `structure` | message | CertificateIdentifier doesn't refer to an existing certificate. |
| `CreateCustomDBEngineVersionFault` | `structure` | message | An error occurred while trying to create the CEV. |
| `CustomAvailabilityZoneNotFoundFault` | `structure` | message | CustomAvailabilityZoneId doesn't refer to an existing custom Availability Zone identifier. |
| `CustomDBEngineVersionAlreadyExistsFault` | `structure` | message | A CEV with the specified name already exists. |
| `CustomDBEngineVersionNotFoundFault` | `structure` | message | The specified CEV was not found. |
| `CustomDBEngineVersionQuotaExceededFault` | `structure` | message | You have exceeded your CEV quota. |
| `DBClusterAlreadyExistsFault` | `structure` | message | The user already has a DB cluster with the given identifier. |
| `DBClusterAutomatedBackupNotFoundFault` | `structure` | message | No automated backup for this DB cluster was found. |
| `DBClusterAutomatedBackupQuotaExceededFault` | `structure` | message | The quota for retained automated backups was exceeded. This prevents you from retaining any additional automated backups. The retained automated backups quo ... |
| `DBClusterBacktrackNotFoundFault` | `structure` | message | BacktrackIdentifier doesn't refer to an existing backtrack. |
| `DBClusterEndpointAlreadyExistsFault` | `structure` | message | The specified custom endpoint can't be created because it already exists. |
| `DBClusterEndpointNotFoundFault` | `structure` | message | The specified custom endpoint doesn't exist. |
| `DBClusterEndpointQuotaExceededFault` | `structure` | message | The cluster already has the maximum number of custom endpoints. |
| `DBClusterNotFoundFault` | `structure` | message | DBClusterIdentifier doesn't refer to an existing DB cluster. |
| `DBClusterParameterGroupNotFoundFault` | `structure` | message | DBClusterParameterGroupName doesn't refer to an existing DB cluster parameter group. |
| `DBClusterQuotaExceededFault` | `structure` | message | The user attempted to create a new DB cluster and the user has already reached the maximum allowed DB cluster quota. |
| `DBClusterRoleAlreadyExistsFault` | `structure` | message | The specified IAM role Amazon Resource Name (ARN) is already associated with the specified DB cluster. |
| `DBClusterRoleNotFoundFault` | `structure` | message | The specified IAM role Amazon Resource Name (ARN) isn't associated with the specified DB cluster. |
| `DBClusterRoleQuotaExceededFault` | `structure` | message | You have exceeded the maximum number of IAM roles that can be associated with the specified DB cluster. |
| `DBClusterSnapshotAlreadyExistsFault` | `structure` | message | The user already has a DB cluster snapshot with the given identifier. |
| `DBClusterSnapshotNotFoundFault` | `structure` | message | DBClusterSnapshotIdentifier doesn't refer to an existing DB cluster snapshot. |
| `DBInstanceAlreadyExistsFault` | `structure` | message | The user already has a DB instance with the given identifier. |
| `DBInstanceAutomatedBackupNotFoundFault` | `structure` | message | No automated backup for this DB instance was found. |
| `DBInstanceAutomatedBackupQuotaExceededFault` | `structure` | message | The quota for retained automated backups was exceeded. This prevents you from retaining any additional automated backups. The retained automated backups quo ... |
| `DBInstanceNotFoundFault` | `structure` | message | DBInstanceIdentifier doesn't refer to an existing DB instance. |
| `DBInstanceNotReadyFault` | `structure` | message | An attempt to download or examine log files didn't succeed because an Aurora Serverless v2 instance was paused. |
| `DBInstanceRoleAlreadyExistsFault` | `structure` | message | The specified RoleArn or FeatureName value is already associated with the DB instance. |
| `DBInstanceRoleNotFoundFault` | `structure` | message | The specified RoleArn value doesn't match the specified feature for the DB instance. |
| `DBInstanceRoleQuotaExceededFault` | `structure` | message | You can't associate any more Amazon Web Services Identity and Access Management (IAM) roles with the DB instance because the quota has been reached. |
| `DBLogFileNotFoundFault` | `structure` | message | LogFileName doesn't refer to an existing DB log file. |
| `DBParameterGroupAlreadyExistsFault` | `structure` | message | A DB parameter group with the same name exists. |
| `DBParameterGroupNotFoundFault` | `structure` | message | DBParameterGroupName doesn't refer to an existing DB parameter group. |
| `DBParameterGroupQuotaExceededFault` | `structure` | message | The request would result in the user exceeding the allowed number of DB parameter groups. |
| `DBProxyAlreadyExistsFault` | `structure` | message | The specified proxy name must be unique for all proxies owned by your Amazon Web Services account in the specified Amazon Web Services Region. |
| `DBProxyEndpointAlreadyExistsFault` | `structure` | message | The specified DB proxy endpoint name must be unique for all DB proxy endpoints owned by your Amazon Web Services account in the specified Amazon Web Service ... |
| `DBProxyEndpointNotFoundFault` | `structure` | message | The DB proxy endpoint doesn't exist. |
| `DBProxyEndpointQuotaExceededFault` | `structure` | message | The DB proxy already has the maximum number of endpoints. |
| `DBProxyNotFoundFault` | `structure` | message | The specified proxy name doesn't correspond to a proxy owned by your Amazon Web Services account in the specified Amazon Web Services Region. |
| `DBProxyQuotaExceededFault` | `structure` | message | Your Amazon Web Services account already has the maximum number of proxies in the specified Amazon Web Services Region. |
| `DBProxyTargetAlreadyRegisteredFault` | `structure` | message | The proxy is already associated with the specified RDS DB instance or Aurora DB cluster. |
| `DBProxyTargetGroupNotFoundFault` | `structure` | message | The specified target group isn't available for a proxy owned by your Amazon Web Services account in the specified Amazon Web Services Region. |
| `DBProxyTargetNotFoundFault` | `structure` | message | The specified RDS DB instance or Aurora DB cluster isn't available for a proxy owned by your Amazon Web Services account in the specified Amazon Web Service ... |
| `DBSecurityGroupAlreadyExistsFault` | `structure` | message | A DB security group with the name specified in DBSecurityGroupName already exists. |
| `DBSecurityGroupNotFoundFault` | `structure` | message | DBSecurityGroupName doesn't refer to an existing DB security group. |
| `DBSecurityGroupNotSupportedFault` | `structure` | message | A DB security group isn't allowed for this action. |
| `DBSecurityGroupQuotaExceededFault` | `structure` | message | The request would result in the user exceeding the allowed number of DB security groups. |
| `DBShardGroupAlreadyExistsFault` | `structure` | message | The specified DB shard group name must be unique in your Amazon Web Services account in the specified Amazon Web Services Region. |
| `DBShardGroupNotFoundFault` | `structure` | message | The specified DB shard group name wasn't found. |
| `DBSnapshotAlreadyExistsFault` | `structure` | message | DBSnapshotIdentifier is already used by an existing snapshot. |
| `DBSnapshotNotFoundFault` | `structure` | message | DBSnapshotIdentifier doesn't refer to an existing DB snapshot. |
| `DBSnapshotTenantDatabaseNotFoundFault` | `structure` | message | The specified snapshot tenant database wasn't found. |
| `DBSubnetGroupAlreadyExistsFault` | `structure` | message | DBSubnetGroupName is already used by an existing DB subnet group. |
| `DBSubnetGroupDoesNotCoverEnoughAZs` | `structure` | message | Subnets in the DB subnet group should cover at least two Availability Zones unless there is only one Availability Zone. |
| `DBSubnetGroupNotAllowedFault` | `structure` | message | The DBSubnetGroup shouldn't be specified while creating read replicas that lie in the same region as the source instance. |
| `DBSubnetGroupNotFoundFault` | `structure` | message | DBSubnetGroupName doesn't refer to an existing DB subnet group. |
| `DBSubnetGroupQuotaExceededFault` | `structure` | message | The request would result in the user exceeding the allowed number of DB subnet groups. |
| `DBSubnetQuotaExceededFault` | `structure` | message | The request would result in the user exceeding the allowed number of subnets in a DB subnet groups. |
| `DBUpgradeDependencyFailureFault` | `structure` | message | The DB upgrade failed because a resource the DB depends on can't be modified. |
| `DomainNotFoundFault` | `structure` | message | Domain doesn't refer to an existing Active Directory domain. |
| `Ec2ImagePropertiesNotSupportedFault` | `structure` | message | The AMI configuration prerequisite has not been met. |
| `EventSubscriptionQuotaExceededFault` | `structure` | message | You have reached the maximum number of event subscriptions. |
| `ExportTaskAlreadyExistsFault` | `structure` | message | You can't start an export task that's already running. |
| `ExportTaskNotFoundFault` | `structure` | message | The export task doesn't exist. |
| `GlobalClusterAlreadyExistsFault` | `structure` | message | The GlobalClusterIdentifier already exists. Specify a new global database identifier (unique name) to create a new global database cluster or to rename an e ... |
| `GlobalClusterNotFoundFault` | `structure` | message | The GlobalClusterIdentifier doesn't refer to an existing global database cluster. |
| `GlobalClusterQuotaExceededFault` | `structure` | message | The number of global database clusters for this account is already at the maximum allowed. |
| `IamRoleMissingPermissionsFault` | `structure` | message | The IAM role requires additional permissions to export to an Amazon S3 bucket. |
| `IamRoleNotFoundFault` | `structure` | message | The IAM role is missing for exporting to an Amazon S3 bucket. |
| `InstanceQuotaExceededFault` | `structure` | message | The request would result in the user exceeding the allowed number of DB instances. |
| `InsufficientAvailableIPsInSubnetFault` | `structure` | message | The requested operation can't be performed because there aren't enough available IP addresses in the proxy's subnets. Add more CIDR blocks to the VPC or rem ... |
| `InsufficientDBClusterCapacityFault` | `structure` | message | The DB cluster doesn't have enough capacity for the current operation. |
| `InsufficientDBInstanceCapacityFault` | `structure` | message | The specified DB instance class isn't available in the specified Availability Zone. |
| `InsufficientStorageClusterCapacityFault` | `structure` | message | There is insufficient storage available for the current action. You might be able to resolve this error by updating your subnet group to use different Avail ... |
| `IntegrationAlreadyExistsFault` | `structure` | message | The integration you are trying to create already exists. |
| `IntegrationConflictOperationFault` | `structure` | message | A conflicting conditional operation is currently in progress against this resource. Typically occurs when there are multiple requests being made to the same ... |
| `IntegrationNotFoundFault` | `structure` | message | The specified integration could not be found. |
| `IntegrationQuotaExceededFault` | `structure` | message | You can't crate any more zero-ETL integrations because the quota has been reached. |
| `InvalidBlueGreenDeploymentStateFault` | `structure` | message | The blue/green deployment can't be switched over or deleted because there is an invalid configuration in the green environment. |
| `InvalidCustomDBEngineVersionStateFault` | `structure` | message | You can't delete the CEV. |
| `InvalidDBClusterAutomatedBackupStateFault` | `structure` | message | The automated backup is in an invalid state. For example, this automated backup is associated with an active cluster. |
| `InvalidDBClusterCapacityFault` | `structure` | message | Capacity isn't a valid Aurora Serverless DB cluster capacity. Valid capacity values are 2 , 4 , 8 , 16 , 32 , 64 , 128 , and 256 . |
| `InvalidDBClusterEndpointStateFault` | `structure` | message | The requested operation can't be performed on the endpoint while the endpoint is in this state. |
| `InvalidDBClusterSnapshotStateFault` | `structure` | message | The supplied value isn't a valid DB cluster snapshot state. |
| `InvalidDBClusterStateFault` | `structure` | message | The requested operation can't be performed while the cluster is in this state. |
| `InvalidDBInstanceAutomatedBackupStateFault` | `structure` | message | The automated backup is in an invalid state. For example, this automated backup is associated with an active instance. |
| `InvalidDBInstanceStateFault` | `structure` | message | The DB instance isn't in a valid state. |
| `InvalidDBParameterGroupStateFault` | `structure` | message | The DB parameter group is in use or is in an invalid state. If you are attempting to delete the parameter group, you can't delete it when the parameter grou ... |
| `InvalidDBProxyEndpointStateFault` | `structure` | message | You can't perform this operation while the DB proxy endpoint is in a particular state. |
| `InvalidDBProxyStateFault` | `structure` | message | The requested operation can't be performed while the proxy is in this state. |
| `InvalidDBSecurityGroupStateFault` | `structure` | message | The state of the DB security group doesn't allow deletion. |
| `InvalidDBShardGroupStateFault` | `structure` | message | The DB shard group must be in the available state. |
| `InvalidDBSnapshotStateFault` | `structure` | message | The state of the DB snapshot doesn't allow deletion. |
| `InvalidDBSubnetGroupFault` | `structure` | message | The DBSubnetGroup doesn't belong to the same VPC as that of an existing cross-region read replica of the same source instance. |
| `InvalidDBSubnetGroupStateFault` | `structure` | message | The DB subnet group cannot be deleted because it's in use. |
| `InvalidDBSubnetStateFault` | `structure` | message | The DB subnet isn't in the available state. |
| `InvalidEventSubscriptionStateFault` | `structure` | message | This error can occur if someone else is modifying a subscription. You should retry the action. |
| `InvalidExportOnlyFault` | `structure` | message | The export is invalid for exporting to an Amazon S3 bucket. |
| `InvalidExportSourceStateFault` | `structure` | message | The state of the export snapshot is invalid for exporting to an Amazon S3 bucket. |
| `InvalidExportTaskStateFault` | `structure` | message | You can't cancel an export task that has completed. |
| `InvalidGlobalClusterStateFault` | `structure` | message | The global cluster is in an invalid state and can't perform the requested operation. |
| `InvalidIntegrationStateFault` | `structure` | message | The integration is in an invalid state and can't perform the requested operation. |
| `InvalidOptionGroupStateFault` | `structure` | message | The option group isn't in the available state. |
| `InvalidResourceStateFault` | `structure` | message | The operation can't be performed because another operation is in progress. |
| `InvalidRestoreFault` | `structure` | message | Cannot restore from VPC backup to non-VPC DB instance. |
| `InvalidS3BucketFault` | `structure` | message | The specified Amazon S3 bucket name can't be found or Amazon RDS isn't authorized to access the specified Amazon S3 bucket. Verify the SourceS3BucketName an ... |
| `InvalidSubnet` | `structure` | message | The requested subnet is invalid, or multiple subnets were requested that are not all in a common VPC. |
| `InvalidVPCNetworkStateFault` | `structure` | message | The DB subnet group doesn't cover all Availability Zones after it's created because of users' change. |
| `KMSKeyNotAccessibleFault` | `structure` | message | An error occurred accessing an Amazon Web Services KMS key. |
| `MaxDBShardGroupLimitReached` | `structure` | message | The maximum number of DB shard groups for your Amazon Web Services account in the specified Amazon Web Services Region has been reached. |
| `NetworkTypeNotSupported` | `structure` | message | The network type is invalid for the DB instance. Valid nework type values are IPV4 and DUAL . |
| `OptionGroupAlreadyExistsFault` | `structure` | message | The option group you are trying to create already exists. |
| `OptionGroupNotFoundFault` | `structure` | message | The specified option group could not be found. |
| `OptionGroupQuotaExceededFault` | `structure` | message | The quota of 20 option groups was exceeded for this Amazon Web Services account. |
| `PointInTimeRestoreNotEnabledFault` | `structure` | message | SourceDBInstanceIdentifier refers to a DB instance with BackupRetentionPeriod equal to 0. |
| `ProvisionedIopsNotAvailableInAZFault` | `structure` | message | Provisioned IOPS not available in the specified Availability Zone. |
| `ReservedDBInstanceAlreadyExistsFault` | `structure` | message | User already has a reservation with the given identifier. |
| `ReservedDBInstanceNotFoundFault` | `structure` | message | The specified reserved DB Instance not found. |
| `ReservedDBInstanceQuotaExceededFault` | `structure` | message | Request would exceed the user's DB Instance quota. |
| `ReservedDBInstancesOfferingNotFoundFault` | `structure` | message | Specified offering does not exist. |
| `ResourceNotFoundFault` | `structure` | message | The specified resource ID was not found. |
| `SNSInvalidTopicFault` | `structure` | message | SNS has responded that there is a problem with the SNS topic specified. |
| `SNSNoAuthorizationFault` | `structure` | message | You do not have permission to publish to the SNS topic ARN. |
| `SNSTopicArnNotFoundFault` | `structure` | message | The SNS topic ARN does not exist. |
| `SharedSnapshotQuotaExceededFault` | `structure` | message | You have exceeded the maximum number of accounts that you can share a manual DB snapshot with. |
| `SnapshotQuotaExceededFault` | `structure` | message | The request would result in the user exceeding the allowed number of DB snapshots. |
| `SourceClusterNotSupportedFault` | `structure` | message | The source DB cluster isn't supported for a blue/green deployment. |
| `SourceDatabaseNotSupportedFault` | `structure` | message | The source DB instance isn't supported for a blue/green deployment. |
| `SourceNotFoundFault` | `structure` | message | The requested source could not be found. |
| `StorageQuotaExceededFault` | `structure` | message | The request would result in the user exceeding the allowed amount of storage available across all DB instances. |
| `StorageTypeNotAvailableFault` | `structure` | message | The aurora-iopt1 storage type isn't available, because you modified the DB cluster to use this storage type less than one month ago. |
| `StorageTypeNotSupportedFault` | `structure` | message | The specified StorageType can't be associated with the DB instance. |
| `SubnetAlreadyInUse` | `structure` | message | The DB subnet is already in use in the Availability Zone. |
| `SubscriptionAlreadyExistFault` | `structure` | message | The supplied subscription name already exists. |
| `SubscriptionCategoryNotFoundFault` | `structure` | message | The supplied category does not exist. |
| `SubscriptionNotFoundFault` | `structure` | message | The subscription name does not exist. |
| `TenantDatabaseAlreadyExistsFault` | `structure` | message | You attempted to either create a tenant database that already exists or modify a tenant database to use the name of an existing tenant database. |
| `TenantDatabaseNotFoundFault` | `structure` | message | The specified tenant database wasn't found in the DB instance. |
| `TenantDatabaseQuotaExceededFault` | `structure` | message | You attempted to create more tenant databases than are permitted in your Amazon Web Services account. |
| `UnsupportedDBEngineVersionFault` | `structure` | message | The specified DB engine version isn't supported for Aurora Limitless Database. |
| `VpcEncryptionControlViolationException` | `structure` | message | The operation violates VPC encryption control settings. Make sure that your DB instance type supports the Nitro encryption-in-transit capability, or modify ... |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
