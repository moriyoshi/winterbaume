# Amazon Redshift

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Redshift Overview This is an interface reference for Amazon Redshift. It contains documentation for one of the programming or command line interfaces you can use to manage Amazon Redshift clusters. Note that Amazon Redshift is asynchronous, which means that some interfaces may require techniques, such as polling or asynchronous callback handlers, to determine when a command has been applied. In this reference, the parameter descriptions indicate whether a change is applied immediately, on the next instance reboot, or during the next maintenance window. For a summary of the Amazon Redshift cluster management interfaces, go to Using the Amazon Redshift Management Interfaces.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Redshift where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon Redshift by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon Redshift by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon Redshift workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Delete`, `Modify`, `Create`, `Get` operation families, including `DescribeAccountAttributes`, `DescribeAuthenticationProfiles`, `DescribeClusterDbRevisions`, `DescribeClusterParameterGroups`, `DeleteAuthenticationProfile`, `DeleteCluster`.

## Service Identity and Protocol

- AWS model slug: `redshift`
- AWS SDK for Rust slug: `redshift`
- Model version: `2012-12-01`
- Model file: `vendor/api-models-aws/models/redshift/service/2012-12-01/redshift-2012-12-01.json`
- SDK ID: `Redshift`
- Endpoint prefix: `redshift`
- ARN namespace: `redshift`
- CloudFormation name: `Redshift`
- CloudTrail event source: `redshift.amazonaws.com`
- Protocols: `awsQuery`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (41), `Delete` (20), `Modify` (20), `Create` (18), `Get` (6), `Authorize` (4), `Revoke` (3), `Batch` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptReservedNodeExchange`, `AddPartner`, `AssociateDataShareConsumer`, `BatchDeleteClusterSnapshots`, `BatchModifyClusterSnapshots`, `CancelResize`, `CreateAuthenticationProfile`, `CreateCluster`, `CreateClusterParameterGroup`, `CreateClusterSecurityGroup`, `CreateClusterSnapshot`, `CreateClusterSubnetGroup`, `CreateCustomDomainAssociation`, `CreateEndpointAccess`, `CreateEventSubscription`, `CreateHsmClientCertificate`, `CreateHsmConfiguration`, `CreateIntegration`, `CreateRedshiftIdcApplication`, `CreateScheduledAction`, ... (+59).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccountAttributes`, `DescribeAuthenticationProfiles`, `DescribeClusterDbRevisions`, `DescribeClusterParameterGroups`, `DescribeClusterParameters`, `DescribeClusterSecurityGroups`, `DescribeClusterSnapshots`, `DescribeClusterSubnetGroups`, `DescribeClusterTracks`, `DescribeClusterVersions`, `DescribeClusters`, `DescribeCustomDomainAssociations`, `DescribeDataShares`, `DescribeDataSharesForConsumer`, `DescribeDataSharesForProducer`, `DescribeDefaultClusterParameters`, `DescribeEndpointAccess`, `DescribeEndpointAuthorization`, `DescribeEventCategories`, `DescribeEventSubscriptions`, ... (+28).
- Pagination is modelled for 37 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelResize`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 133 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `SNS`, `Glue`, `EC2/VPC`, `ECS`, `Redshift`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/redshift/latest/mgmt/working-with-clusters.html
- https://docs.aws.amazon.com/redshift/latest/mgmt/rs-mgmt-pause-resume-cluster.html
- https://docs.aws.amazon.com/redshift/latest/mgmt/working-with-snapshots.html

Research outcomes:
- A provisioned cluster has a leader node and one or more compute nodes. Single-node clusters share leader and compute functionality.
- Cluster status is a rich lifecycle value including creating, available, modifying, resizing, paused, rebooting, deleting, final-snapshot, hardware-failure, storage-full, and several incompatible states.
- Some elastic resize states keep the cluster available for reads and writes but block cluster operations such as snapshot creation.
- Pausing creates a snapshot, terminates queries, and moves the cluster through pausing to paused. Pause and resume cannot be cancelled or rolled back after initiation.
- Paused clusters cannot be modified, resumed before pause completes, or used for queries. Scheduled actions such as snapshots, resizing, and maintenance are skipped while paused.
- Resuming can update the cluster to a maintenance version and may change node IP addresses or network interfaces.
- Automated snapshots are enabled by default, are taken periodically, and are retained according to retention settings. RA3 automated snapshots cannot be disabled and support 1-35 days retention.
- Manual snapshots are retained indefinitely by default and survive cluster deletion until manually deleted or their retention period expires.

Parity implications:
- Model clusters, nodes, network state, snapshots, schedules, maintenance, pause/resume operations, resize operations, and status transitions.
- Long-running operations should expose intermediate statuses and block incompatible operations while allowing documented read/write access where applicable.
- Snapshot retention, automated/manual distinction, final-snapshot delete state, pause-created snapshots, and RA3-specific constraints need explicit handling.

## Control-Plane / Data-Plane Coherence

- **Paired with `redshiftdata` ( Redshift Data API ).** Redshift Data ( `winterbaume-redshiftdata` ) executes SQL against Redshift provisioned clusters and Redshift Serverless workgroups that this control plane ( and `winterbaume-redshiftserverless` ) create and manage. `ExecuteStatement` requires either a `clusterIdentifier` ( provisioned ) or a `workgroupName` ( serverless ); in real AWS the call fails with `ValidationException` if neither resolves to an existing target.
- **Current Winterbaume status: deliberately separate.** `winterbaume-redshiftdata` carries its own catalogue ( `databases`, `schemas`, `table_names`, `table_columns` ) and a result queue, with no dependency on this crate or on `winterbaume-redshiftserverless`. It does not validate that the `clusterIdentifier` exists. This crate's catalogue plus the SQL execution backend ( `winterbaume-query-engine` family ) is what the data plane uses, separately.
- **What needs to change ( low priority ):** if a future workflow needs the data plane to validate cluster/workgroup existence and authentication mode, `winterbaume-redshiftdata` should observe this crate's `clusters` and `winterbaume-redshiftserverless`'s `workgroups`. The catalogue itself stays in the data-plane crate ( Redshift's control plane does not expose a schema-catalog API ).

## Current Network Resource Stub Semantics

Redshift currently keeps subnet groups, VPC security groups, and endpoint access metadata in Redshift state.

- Cluster subnet groups store supplied subnet IDs and are referenced by clusters as local resource names.
- Clusters and endpoints store VPC security group IDs and endpoint subnet group information without EC2 validation.
- Endpoint and cluster lifecycle does not allocate ENIs or modify EC2 security groups.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Describe

- Operations: `DescribeAccountAttributes`, `DescribeAuthenticationProfiles`, `DescribeClusterDbRevisions`, `DescribeClusterParameterGroups`, `DescribeClusterParameters`, `DescribeClusterSecurityGroups`, `DescribeClusterSnapshots`, `DescribeClusterSubnetGroups`, `DescribeClusterTracks`, `DescribeClusterVersions`, `DescribeClusters`, `DescribeCustomDomainAssociations`, `DescribeDataShares`, `DescribeDataSharesForConsumer`, `DescribeDataSharesForProducer`, `DescribeDefaultClusterParameters`, `DescribeEndpointAccess`, `DescribeEndpointAuthorization`, `DescribeEventCategories`, `DescribeEventSubscriptions`, `DescribeEvents`, `DescribeHsmClientCertificates`, `DescribeHsmConfigurations`, `DescribeInboundIntegrations`, `DescribeIntegrations`, `DescribeLoggingStatus`, `DescribeNodeConfigurationOptions`, `DescribeOrderableClusterOptions`, `DescribePartners`, `DescribeRedshiftIdcApplications`, `DescribeReservedNodeExchangeStatus`, `DescribeReservedNodeOfferings`, `DescribeReservedNodes`, `DescribeResize`, `DescribeScheduledActions`, `DescribeSnapshotCopyGrants`, `DescribeSnapshotSchedules`, `DescribeStorage`, `DescribeTableRestoreStatus`, `DescribeTags`, ... (+1)
- Traits: `paginated` (34)
- Common required input members in this group: `AccountId`, `ActionType`, `ClusterIdentifier`, `ParameterGroupFamily`, `ParameterGroupName`

### Delete

- Operations: `DeleteAuthenticationProfile`, `DeleteCluster`, `DeleteClusterParameterGroup`, `DeleteClusterSecurityGroup`, `DeleteClusterSnapshot`, `DeleteClusterSubnetGroup`, `DeleteCustomDomainAssociation`, `DeleteEndpointAccess`, `DeleteEventSubscription`, `DeleteHsmClientCertificate`, `DeleteHsmConfiguration`, `DeleteIntegration`, `DeletePartner`, `DeleteRedshiftIdcApplication`, `DeleteResourcePolicy`, `DeleteScheduledAction`, `DeleteSnapshotCopyGrant`, `DeleteSnapshotSchedule`, `DeleteTags`, `DeleteUsageLimit`
- Common required input members in this group: `AccountId`, `AuthenticationProfileName`, `ClusterIdentifier`, `ClusterSecurityGroupName`, `ClusterSubnetGroupName`, `CustomDomainName`, `DatabaseName`, `EndpointName`, `HsmClientCertificateIdentifier`, `HsmConfigurationIdentifier`, `IntegrationArn`, `ParameterGroupName`, `PartnerName`, `RedshiftIdcApplicationArn`, `ResourceArn`, `ResourceName`, `ScheduleIdentifier`, `ScheduledActionName`, `SnapshotCopyGrantName`, `SnapshotIdentifier`, `SubscriptionName`, `TagKeys`, `UsageLimitId`

### Modify

- Operations: `ModifyAquaConfiguration`, `ModifyAuthenticationProfile`, `ModifyCluster`, `ModifyClusterDbRevision`, `ModifyClusterIamRoles`, `ModifyClusterMaintenance`, `ModifyClusterParameterGroup`, `ModifyClusterSnapshot`, `ModifyClusterSnapshotSchedule`, `ModifyClusterSubnetGroup`, `ModifyCustomDomainAssociation`, `ModifyEndpointAccess`, `ModifyEventSubscription`, `ModifyIntegration`, `ModifyLakehouseConfiguration`, `ModifyRedshiftIdcApplication`, `ModifyScheduledAction`, `ModifySnapshotCopyRetentionPeriod`, `ModifySnapshotSchedule`, `ModifyUsageLimit`
- Common required input members in this group: `AuthenticationProfileContent`, `AuthenticationProfileName`, `ClusterIdentifier`, `ClusterSubnetGroupName`, `CustomDomainCertificateArn`, `CustomDomainName`, `EndpointName`, `IntegrationArn`, `ParameterGroupName`, `Parameters`, `RedshiftIdcApplicationArn`, `RetentionPeriod`, `RevisionTarget`, `ScheduleDefinitions`, `ScheduleIdentifier`, `ScheduledActionName`, `SnapshotIdentifier`, `SubnetIds`, `SubscriptionName`, `UsageLimitId`

### Create

- Operations: `CreateAuthenticationProfile`, `CreateCluster`, `CreateClusterParameterGroup`, `CreateClusterSecurityGroup`, `CreateClusterSnapshot`, `CreateClusterSubnetGroup`, `CreateCustomDomainAssociation`, `CreateEndpointAccess`, `CreateEventSubscription`, `CreateHsmClientCertificate`, `CreateHsmConfiguration`, `CreateIntegration`, `CreateRedshiftIdcApplication`, `CreateScheduledAction`, `CreateSnapshotCopyGrant`, `CreateSnapshotSchedule`, `CreateTags`, `CreateUsageLimit`
- Common required input members in this group: `Amount`, `AuthenticationProfileContent`, `AuthenticationProfileName`, `ClusterIdentifier`, `ClusterSecurityGroupName`, `ClusterSubnetGroupName`, `CustomDomainCertificateArn`, `CustomDomainName`, `Description`, `EndpointName`, `FeatureType`, `HsmClientCertificateIdentifier`, `HsmConfigurationIdentifier`, `HsmIpAddress`, `HsmPartitionName`, `HsmPartitionPassword`, `HsmServerPublicCertificate`, `IamRole`, `IamRoleArn`, `IdcDisplayName`, `IdcInstanceArn`, `IntegrationName`, `LimitType`, `MasterUsername`, ... (+17)

### Get

- Operations: `GetClusterCredentials`, `GetClusterCredentialsWithIAM`, `GetIdentityCenterAuthToken`, `GetReservedNodeExchangeConfigurationOptions`, `GetReservedNodeExchangeOfferings`, `GetResourcePolicy`
- Traits: `paginated` (2)
- Common required input members in this group: `ActionType`, `ClusterIds`, `DbUser`, `ReservedNodeId`, `ResourceArn`

### Authorize

- Operations: `AuthorizeClusterSecurityGroupIngress`, `AuthorizeDataShare`, `AuthorizeEndpointAccess`, `AuthorizeSnapshotAccess`
- Common required input members in this group: `Account`, `AccountWithRestoreAccess`, `ClusterSecurityGroupName`, `ConsumerIdentifier`, `DataShareArn`

### Revoke

- Operations: `RevokeClusterSecurityGroupIngress`, `RevokeEndpointAccess`, `RevokeSnapshotAccess`
- Common required input members in this group: `AccountWithRestoreAccess`, `ClusterSecurityGroupName`

### Batch

- Operations: `BatchDeleteClusterSnapshots`, `BatchModifyClusterSnapshots`
- Common required input members in this group: `Identifiers`, `SnapshotIdentifierList`

### Disable

- Operations: `DisableLogging`, `DisableSnapshotCopy`
- Common required input members in this group: `ClusterIdentifier`

### Enable

- Operations: `EnableLogging`, `EnableSnapshotCopy`
- Common required input members in this group: `ClusterIdentifier`, `DestinationRegion`

### Restore

- Operations: `RestoreFromClusterSnapshot`, `RestoreTableFromClusterSnapshot`
- Common required input members in this group: `ClusterIdentifier`, `NewTableName`, `SnapshotIdentifier`, `SourceDatabaseName`, `SourceTableName`

### Accept

- Operations: `AcceptReservedNodeExchange`
- Common required input members in this group: `ReservedNodeId`, `TargetReservedNodeOfferingId`

### Add

- Operations: `AddPartner`
- Common required input members in this group: `AccountId`, `ClusterIdentifier`, `DatabaseName`, `PartnerName`

### Associate

- Operations: `AssociateDataShareConsumer`
- Common required input members in this group: `DataShareArn`

### Cancel

- Operations: `CancelResize`
- Common required input members in this group: `ClusterIdentifier`

### Copy

- Operations: `CopyClusterSnapshot`
- Common required input members in this group: `SourceSnapshotIdentifier`, `TargetSnapshotIdentifier`

### Deauthorize

- Operations: `DeauthorizeDataShare`
- Common required input members in this group: `ConsumerIdentifier`, `DataShareArn`

### Deregister

- Operations: `DeregisterNamespace`
- Common required input members in this group: `ConsumerIdentifiers`, `NamespaceIdentifier`

### Disassociate

- Operations: `DisassociateDataShareConsumer`
- Common required input members in this group: `DataShareArn`

### Failover

- Operations: `FailoverPrimaryCompute`
- Common required input members in this group: `ClusterIdentifier`

### List

- Operations: `ListRecommendations`
- Traits: `paginated` (1)

### Pause

- Operations: `PauseCluster`
- Common required input members in this group: `ClusterIdentifier`

### Purchase

- Operations: `PurchaseReservedNodeOffering`
- Common required input members in this group: `ReservedNodeOfferingId`

### Put

- Operations: `PutResourcePolicy`
- Common required input members in this group: `Policy`, `ResourceArn`

### Reboot

- Operations: `RebootCluster`
- Common required input members in this group: `ClusterIdentifier`

### Register

- Operations: `RegisterNamespace`
- Common required input members in this group: `ConsumerIdentifiers`, `NamespaceIdentifier`

### Reject

- Operations: `RejectDataShare`
- Common required input members in this group: `DataShareArn`

### Reset

- Operations: `ResetClusterParameterGroup`
- Common required input members in this group: `ParameterGroupName`

### Resize

- Operations: `ResizeCluster`
- Common required input members in this group: `ClusterIdentifier`

### Resume

- Operations: `ResumeCluster`
- Common required input members in this group: `ClusterIdentifier`

### Rotate

- Operations: `RotateEncryptionKey`
- Common required input members in this group: `ClusterIdentifier`

### Update

- Operations: `UpdatePartnerStatus`
- Common required input members in this group: `AccountId`, `ClusterIdentifier`, `DatabaseName`, `PartnerName`, `Status`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptReservedNodeExchange` | - | - | `ReservedNodeId`, `TargetReservedNodeOfferingId` | - | `AcceptReservedNodeExchangeOutputMessage` | `DependentServiceUnavailableFault`, `InvalidReservedNodeStateFault`, `ReservedNodeAlreadyExistsFault`, `ReservedNodeAlreadyMigratedFault`, `ReservedNodeNotFoundFault`, `ReservedNodeOfferingNotFoundFault`, `UnsupportedOperationFault` | Exchanges a DC1 Reserved Node for a DC2 Reserved Node with no changes to the configuration (term, payment type, or number of nodes) and no additional costs. |
| `AddPartner` | - | - | `AccountId`, `ClusterIdentifier`, `DatabaseName`, `PartnerName` | - | `PartnerIntegrationOutputMessage` | `ClusterNotFoundFault`, `PartnerNotFoundFault`, `UnauthorizedPartnerIntegrationFault`, `UnsupportedOperationFault` | Adds a partner integration to a cluster. This operation authorizes a partner to push status updates for the specified database. |
| `AssociateDataShareConsumer` | - | - | `DataShareArn` | - | `DataShare` | `InvalidDataShareFault`, `InvalidNamespaceFault` | From a datashare consumer account, associates a datashare with the account (AssociateEntireAccount) or the specified namespace (ConsumerArn). If you make this association, the consumer can consume the datashare. |
| `AuthorizeClusterSecurityGroupIngress` | - | - | `ClusterSecurityGroupName` | - | `AuthorizeClusterSecurityGroupIngressResult` | `AuthorizationAlreadyExistsFault`, `AuthorizationQuotaExceededFault`, `ClusterSecurityGroupNotFoundFault`, `InvalidClusterSecurityGroupStateFault` | Adds an inbound (ingress) rule to an Amazon Redshift security group. Depending on whether the application accessing your cluster is running on the Internet or an Amazon EC2 instance, you can authorize inbound access to either a Classless Interdomain Routing... |
| `AuthorizeDataShare` | - | - | `ConsumerIdentifier`, `DataShareArn` | - | `DataShare` | `InvalidDataShareFault` | From a data producer account, authorizes the sharing of a datashare with one or more consumer accounts or managing entities. To authorize a datashare for a data consumer, the producer account must have the correct access permissions. |
| `AuthorizeEndpointAccess` | - | - | `Account` | - | `EndpointAuthorization` | `ClusterNotFoundFault`, `EndpointAuthorizationAlreadyExistsFault`, `EndpointAuthorizationsPerClusterLimitExceededFault`, `InvalidAuthorizationStateFault`, `InvalidClusterStateFault`, `UnsupportedOperationFault` | Grants access to a cluster. |
| `AuthorizeSnapshotAccess` | - | - | `AccountWithRestoreAccess` | - | `AuthorizeSnapshotAccessResult` | `AuthorizationAlreadyExistsFault`, `AuthorizationQuotaExceededFault`, `ClusterSnapshotNotFoundFault`, `DependentServiceRequestThrottlingFault`, `InvalidClusterSnapshotStateFault`, `LimitExceededFault`, `UnsupportedOperationFault` | Authorizes the specified Amazon Web Services account to restore the specified snapshot. For more information about working with snapshots, go to Amazon Redshift Snapshots in the Amazon Redshift Cluster Management Guide . |
| `BatchDeleteClusterSnapshots` | - | - | `Identifiers` | - | `BatchDeleteClusterSnapshotsResult` | `BatchDeleteRequestSizeExceededFault` | Deletes a set of cluster snapshots. |
| `BatchModifyClusterSnapshots` | - | - | `SnapshotIdentifierList` | - | `BatchModifyClusterSnapshotsOutputMessage` | `BatchModifyClusterSnapshotsLimitExceededFault`, `InvalidRetentionPeriodFault` | Modifies the settings for a set of cluster snapshots. |
| `CancelResize` | - | - | `ClusterIdentifier` | - | `ResizeProgressMessage` | `ClusterNotFoundFault`, `InvalidClusterStateFault`, `ResizeNotFoundFault`, `UnsupportedOperationFault` | Cancels a resize operation for a cluster. |
| `CopyClusterSnapshot` | - | - | `SourceSnapshotIdentifier`, `TargetSnapshotIdentifier` | - | `CopyClusterSnapshotResult` | `ClusterNotFoundFault`, `ClusterSnapshotAlreadyExistsFault`, `ClusterSnapshotNotFoundFault`, `ClusterSnapshotQuotaExceededFault`, `InvalidClusterSnapshotStateFault`, `InvalidRetentionPeriodFault` | Copies the specified automated cluster snapshot to a new manual cluster snapshot. The source must be an automated snapshot and it must be in the available state. |
| `CreateAuthenticationProfile` | - | - | `AuthenticationProfileContent`, `AuthenticationProfileName` | - | `CreateAuthenticationProfileResult` | `AuthenticationProfileAlreadyExistsFault`, `AuthenticationProfileQuotaExceededFault`, `InvalidAuthenticationProfileRequestFault` | Creates an authentication profile with the specified parameters. |
| `CreateCluster` | - | - | `ClusterIdentifier`, `MasterUsername`, `NodeType` | - | `CreateClusterResult` | `ClusterAlreadyExistsFault`, `ClusterParameterGroupNotFoundFault`, `ClusterQuotaExceededFault`, `ClusterSecurityGroupNotFoundFault`, `ClusterSubnetGroupNotFoundFault`, `DependentServiceAccessDeniedFault`, `DependentServiceRequestThrottlingFault`, `DependentServiceUnavailableFault`, ... (+19) | Creates a new cluster with the specified parameters. To create a cluster in Virtual Private Cloud (VPC), you must provide a cluster subnet group name. |
| `CreateClusterParameterGroup` | - | - | `Description`, `ParameterGroupFamily`, `ParameterGroupName` | - | `CreateClusterParameterGroupResult` | `ClusterParameterGroupAlreadyExistsFault`, `ClusterParameterGroupQuotaExceededFault`, `InvalidTagFault`, `TagLimitExceededFault` | Creates an Amazon Redshift parameter group. Creating parameter groups is independent of creating clusters. |
| `CreateClusterSecurityGroup` | - | - | `ClusterSecurityGroupName`, `Description` | - | `CreateClusterSecurityGroupResult` | `ClusterSecurityGroupAlreadyExistsFault`, `ClusterSecurityGroupQuotaExceededFault`, `InvalidTagFault`, `TagLimitExceededFault` | Creates a new Amazon Redshift security group. You use security groups to control access to non-VPC clusters. |
| `CreateClusterSnapshot` | - | - | `ClusterIdentifier`, `SnapshotIdentifier` | - | `CreateClusterSnapshotResult` | `ClusterNotFoundFault`, `ClusterSnapshotAlreadyExistsFault`, `ClusterSnapshotQuotaExceededFault`, `InvalidClusterStateFault`, `InvalidRetentionPeriodFault`, `InvalidTagFault`, `TagLimitExceededFault` | Creates a manual snapshot of the specified cluster. The cluster must be in the `available` state. |
| `CreateClusterSubnetGroup` | - | - | `ClusterSubnetGroupName`, `Description`, `SubnetIds` | - | `CreateClusterSubnetGroupResult` | `ClusterSubnetGroupAlreadyExistsFault`, `ClusterSubnetGroupQuotaExceededFault`, `ClusterSubnetQuotaExceededFault`, `DependentServiceRequestThrottlingFault`, `InvalidSubnet`, `InvalidTagFault`, `TagLimitExceededFault`, `UnauthorizedOperation` | Creates a new Amazon Redshift subnet group. You must provide a list of one or more subnets in your existing Amazon Virtual Private Cloud (Amazon VPC) when creating Amazon Redshift subnet group. |
| `CreateCustomDomainAssociation` | - | - | `ClusterIdentifier`, `CustomDomainCertificateArn`, `CustomDomainName` | - | `CreateCustomDomainAssociationResult` | `ClusterNotFoundFault`, `CustomCnameAssociationFault`, `UnsupportedOperationFault` | Used to create a custom domain name for a cluster. Properties include the custom domain name, the cluster the custom domain is associated with, and the certificate Amazon Resource Name (ARN). |
| `CreateEndpointAccess` | - | - | `EndpointName`, `SubnetGroupName` | - | `EndpointAccess` | `AccessToClusterDeniedFault`, `ClusterNotFoundFault`, `ClusterSubnetGroupNotFoundFault`, `EndpointAlreadyExistsFault`, `EndpointsPerAuthorizationLimitExceededFault`, `EndpointsPerClusterLimitExceededFault`, `InvalidClusterSecurityGroupStateFault`, `InvalidClusterStateFault`, ... (+2) | Creates a Redshift-managed VPC endpoint. |
| `CreateEventSubscription` | - | - | `SnsTopicArn`, `SubscriptionName` | - | `CreateEventSubscriptionResult` | `EventSubscriptionQuotaExceededFault`, `InvalidTagFault`, `SNSInvalidTopicFault`, `SNSNoAuthorizationFault`, `SNSTopicArnNotFoundFault`, `SourceNotFoundFault`, `SubscriptionAlreadyExistFault`, `SubscriptionCategoryNotFoundFault`, ... (+3) | Creates an Amazon Redshift event notification subscription. This action requires an ARN (Amazon Resource Name) of an Amazon SNS topic created by either the Amazon Redshift console, the Amazon SNS console, or the Amazon SNS API. |
| `CreateHsmClientCertificate` | - | - | `HsmClientCertificateIdentifier` | - | `CreateHsmClientCertificateResult` | `HsmClientCertificateAlreadyExistsFault`, `HsmClientCertificateQuotaExceededFault`, `InvalidTagFault`, `TagLimitExceededFault` | Creates an HSM client certificate that an Amazon Redshift cluster will use to connect to the client's HSM in order to store and retrieve the keys used to encrypt the cluster databases. The command returns a public key, which you must store in the HSM. |
| `CreateHsmConfiguration` | - | - | `Description`, `HsmConfigurationIdentifier`, `HsmIpAddress`, `HsmPartitionName`, `HsmPartitionPassword`, `HsmServerPublicCertificate` | - | `CreateHsmConfigurationResult` | `HsmConfigurationAlreadyExistsFault`, `HsmConfigurationQuotaExceededFault`, `InvalidTagFault`, `TagLimitExceededFault` | Creates an HSM configuration that contains the information required by an Amazon Redshift cluster to store and use database encryption keys in a Hardware Security Module (HSM). After creating the HSM configuration, you can specify it as a parameter when... |
| `CreateIntegration` | - | - | `IntegrationName`, `SourceArn`, `TargetArn` | - | `Integration` | `IntegrationAlreadyExistsFault`, `IntegrationConflictOperationFault`, `IntegrationQuotaExceededFault`, `IntegrationSourceNotFoundFault`, `IntegrationTargetNotFoundFault`, `InvalidClusterStateFault`, `InvalidTagFault`, `TagLimitExceededFault`, ... (+1) | Creates a zero-ETL integration or S3 event integration with Amazon Redshift. |
| `CreateRedshiftIdcApplication` | - | - | `IamRoleArn`, `IdcDisplayName`, `IdcInstanceArn`, `RedshiftIdcApplicationName` | - | `CreateRedshiftIdcApplicationResult` | `DependentServiceAccessDeniedFault`, `DependentServiceUnavailableFault`, `InvalidTagFault`, `RedshiftIdcApplicationAlreadyExistsFault`, `RedshiftIdcApplicationQuotaExceededFault`, `TagLimitExceededFault`, `UnsupportedOperationFault` | Creates an Amazon Redshift application for use with IAM Identity Center. |
| `CreateScheduledAction` | - | - | `IamRole`, `Schedule`, `ScheduledActionName`, `TargetAction` | - | `ScheduledAction` | `ClusterNotFoundFault`, `InvalidScheduleFault`, `InvalidScheduledActionFault`, `ScheduledActionAlreadyExistsFault`, `ScheduledActionQuotaExceededFault`, `ScheduledActionTypeUnsupportedFault`, `UnauthorizedOperation`, `UnsupportedOperationFault` | Creates a scheduled action. A scheduled action contains a schedule and an Amazon Redshift API action. |
| `CreateSnapshotCopyGrant` | - | - | `SnapshotCopyGrantName` | - | `CreateSnapshotCopyGrantResult` | `DependentServiceRequestThrottlingFault`, `InvalidTagFault`, `LimitExceededFault`, `SnapshotCopyGrantAlreadyExistsFault`, `SnapshotCopyGrantQuotaExceededFault`, `TagLimitExceededFault` | Creates a snapshot copy grant that permits Amazon Redshift to use an encrypted symmetric key from Key Management Service (KMS) to encrypt copied snapshots in a destination region. For more information about managing snapshot copy grants, go to Amazon Redshift... |
| `CreateSnapshotSchedule` | - | - | - | - | `SnapshotSchedule` | `InvalidScheduleFault`, `InvalidTagFault`, `ScheduleDefinitionTypeUnsupportedFault`, `SnapshotScheduleAlreadyExistsFault`, `SnapshotScheduleQuotaExceededFault`, `TagLimitExceededFault` | Create a snapshot schedule that can be associated to a cluster and which overrides the default system backup schedule. |
| `CreateTags` | - | - | `ResourceName`, `Tags` | - | `Unit` | `InvalidClusterStateFault`, `InvalidTagFault`, `ResourceNotFoundFault`, `TagLimitExceededFault` | Adds tags to a cluster. A resource can have up to 50 tags. |
| `CreateUsageLimit` | - | - | `Amount`, `ClusterIdentifier`, `FeatureType`, `LimitType` | - | `UsageLimit` | `ClusterNotFoundFault`, `InvalidClusterStateFault`, `InvalidUsageLimitFault`, `LimitExceededFault`, `TagLimitExceededFault`, `UnsupportedOperationFault`, `UsageLimitAlreadyExistsFault` | Creates a usage limit for a specified Amazon Redshift feature on a cluster. The usage limit is identified by the returned usage limit identifier. |
| `DeauthorizeDataShare` | - | - | `ConsumerIdentifier`, `DataShareArn` | - | `DataShare` | `InvalidDataShareFault` | From a datashare producer account, removes authorization from the specified datashare. |
| `DeleteAuthenticationProfile` | - | - | `AuthenticationProfileName` | - | `DeleteAuthenticationProfileResult` | `AuthenticationProfileNotFoundFault`, `InvalidAuthenticationProfileRequestFault` | Deletes an authentication profile. |
| `DeleteCluster` | - | - | `ClusterIdentifier` | - | `DeleteClusterResult` | `ClusterNotFoundFault`, `ClusterSnapshotAlreadyExistsFault`, `ClusterSnapshotQuotaExceededFault`, `InvalidClusterStateFault`, `InvalidRetentionPeriodFault` | Deletes a previously provisioned cluster without its final snapshot being created. A successful response from the web service indicates that the request was received correctly. |
| `DeleteClusterParameterGroup` | - | - | `ParameterGroupName` | - | `Unit` | `ClusterParameterGroupNotFoundFault`, `InvalidClusterParameterGroupStateFault` | Deletes a specified Amazon Redshift parameter group. You cannot delete a parameter group if it is associated with a cluster. |
| `DeleteClusterSecurityGroup` | - | - | `ClusterSecurityGroupName` | - | `Unit` | `ClusterSecurityGroupNotFoundFault`, `InvalidClusterSecurityGroupStateFault` | Deletes an Amazon Redshift security group. You cannot delete a security group that is associated with any clusters. |
| `DeleteClusterSnapshot` | - | - | `SnapshotIdentifier` | - | `DeleteClusterSnapshotResult` | `ClusterSnapshotNotFoundFault`, `InvalidClusterSnapshotStateFault` | Deletes the specified manual snapshot. The snapshot must be in the `available` state, with no other users authorized to access the snapshot. |
| `DeleteClusterSubnetGroup` | - | - | `ClusterSubnetGroupName` | - | `Unit` | `ClusterSubnetGroupNotFoundFault`, `InvalidClusterSubnetGroupStateFault`, `InvalidClusterSubnetStateFault` | Deletes the specified cluster subnet group. |
| `DeleteCustomDomainAssociation` | - | - | `ClusterIdentifier`, `CustomDomainName` | - | `Unit` | `ClusterNotFoundFault`, `CustomCnameAssociationFault`, `CustomDomainAssociationNotFoundFault`, `UnsupportedOperationFault` | Contains information about deleting a custom domain association for a cluster. |
| `DeleteEndpointAccess` | - | - | `EndpointName` | - | `EndpointAccess` | `ClusterNotFoundFault`, `EndpointNotFoundFault`, `InvalidClusterSecurityGroupStateFault`, `InvalidClusterStateFault`, `InvalidEndpointStateFault` | Deletes a Redshift-managed VPC endpoint. |
| `DeleteEventSubscription` | - | - | `SubscriptionName` | - | `Unit` | `InvalidSubscriptionStateFault`, `SubscriptionNotFoundFault` | Deletes an Amazon Redshift event notification subscription. |
| `DeleteHsmClientCertificate` | - | - | `HsmClientCertificateIdentifier` | - | `Unit` | `HsmClientCertificateNotFoundFault`, `InvalidHsmClientCertificateStateFault` | Deletes the specified HSM client certificate. |
| `DeleteHsmConfiguration` | - | - | `HsmConfigurationIdentifier` | - | `Unit` | `HsmConfigurationNotFoundFault`, `InvalidHsmConfigurationStateFault` | Deletes the specified Amazon Redshift HSM configuration. |
| `DeleteIntegration` | - | - | `IntegrationArn` | - | `Integration` | `IntegrationConflictOperationFault`, `IntegrationConflictStateFault`, `IntegrationNotFoundFault`, `UnsupportedOperationFault` | Deletes a zero-ETL integration or S3 event integration with Amazon Redshift. |
| `DeletePartner` | - | - | `AccountId`, `ClusterIdentifier`, `DatabaseName`, `PartnerName` | - | `PartnerIntegrationOutputMessage` | `ClusterNotFoundFault`, `PartnerNotFoundFault`, `UnauthorizedPartnerIntegrationFault`, `UnsupportedOperationFault` | Deletes a partner integration from a cluster. Data can still flow to the cluster until the integration is deleted at the partner's website. |
| `DeleteRedshiftIdcApplication` | - | - | `RedshiftIdcApplicationArn` | - | `Unit` | `DependentServiceAccessDeniedFault`, `DependentServiceUnavailableFault`, `RedshiftIdcApplicationNotExistsFault`, `UnsupportedOperationFault` | Deletes an Amazon Redshift IAM Identity Center application. |
| `DeleteResourcePolicy` | - | - | `ResourceArn` | - | `Unit` | `ResourceNotFoundFault`, `UnsupportedOperationFault` | Deletes the resource policy for a specified resource. |
| `DeleteScheduledAction` | - | - | `ScheduledActionName` | - | `Unit` | `ScheduledActionNotFoundFault`, `UnauthorizedOperation` | Deletes a scheduled action. |
| `DeleteSnapshotCopyGrant` | - | - | `SnapshotCopyGrantName` | - | `Unit` | `InvalidSnapshotCopyGrantStateFault`, `SnapshotCopyGrantNotFoundFault` | Deletes the specified snapshot copy grant. |
| `DeleteSnapshotSchedule` | - | - | `ScheduleIdentifier` | - | `Unit` | `InvalidClusterSnapshotScheduleStateFault`, `SnapshotScheduleNotFoundFault` | Deletes a snapshot schedule. |
| `DeleteTags` | - | - | `ResourceName`, `TagKeys` | - | `Unit` | `InvalidTagFault`, `ResourceNotFoundFault` | Deletes tags from a resource. You must provide the ARN of the resource from which you want to delete the tag or tags. |
| `DeleteUsageLimit` | - | - | `UsageLimitId` | - | `Unit` | `UnsupportedOperationFault`, `UsageLimitNotFoundFault` | Deletes a usage limit from a cluster. |
| `DeregisterNamespace` | - | - | `ConsumerIdentifiers`, `NamespaceIdentifier` | - | `DeregisterNamespaceOutputMessage` | `ClusterNotFoundFault`, `InvalidClusterStateFault`, `InvalidNamespaceFault` | Deregisters a cluster or serverless namespace from the Amazon Web Services Glue Data Catalog. |
| `DescribeAccountAttributes` | - | - | - | - | `AccountAttributeList` | - | Returns a list of attributes attached to an account |
| `DescribeAuthenticationProfiles` | - | - | - | - | `DescribeAuthenticationProfilesResult` | `AuthenticationProfileNotFoundFault`, `InvalidAuthenticationProfileRequestFault` | Describes an authentication profile. |
| `DescribeClusterDbRevisions` | - | `paginated` | - | - | `ClusterDbRevisionsMessage` | `ClusterNotFoundFault`, `InvalidClusterStateFault` | Returns an array of `ClusterDbRevision` objects. |
| `DescribeClusterParameterGroups` | - | `paginated` | - | - | `ClusterParameterGroupsMessage` | `ClusterParameterGroupNotFoundFault`, `InvalidTagFault` | Returns a list of Amazon Redshift parameter groups, including parameter groups you created and the default parameter group. For each parameter group, the response includes the parameter group name, description, and parameter group family name. |
| `DescribeClusterParameters` | - | `paginated` | `ParameterGroupName` | - | `ClusterParameterGroupDetails` | `ClusterParameterGroupNotFoundFault` | Returns a detailed list of parameters contained within the specified Amazon Redshift parameter group. For each parameter the response includes information such as parameter name, description, data type, value, whether the parameter value is modifiable, and so... |
| `DescribeClusterSecurityGroups` | - | `paginated` | - | - | `ClusterSecurityGroupMessage` | `ClusterSecurityGroupNotFoundFault`, `InvalidTagFault` | Returns information about Amazon Redshift security groups. If the name of a security group is specified, the response will contain only information about only that security group. |
| `DescribeClusterSnapshots` | - | `paginated` | - | - | `SnapshotMessage` | `ClusterNotFoundFault`, `ClusterSnapshotNotFoundFault`, `InvalidTagFault`, `UnsupportedOperationFault` | Returns one or more snapshot objects, which contain metadata about your cluster snapshots. By default, this operation returns information about all snapshots of all clusters that are owned by your Amazon Web Services account. |
| `DescribeClusterSubnetGroups` | - | `paginated` | - | - | `ClusterSubnetGroupMessage` | `ClusterSubnetGroupNotFoundFault`, `InvalidTagFault` | Returns one or more cluster subnet group objects, which contain metadata about your cluster subnet groups. By default, this operation returns information about all cluster subnet groups that are defined in your Amazon Web Services account. |
| `DescribeClusterTracks` | - | `paginated` | - | - | `TrackListMessage` | `InvalidClusterTrackFault`, `UnauthorizedOperation` | Returns a list of all the available maintenance tracks. |
| `DescribeClusterVersions` | - | `paginated` | - | - | `ClusterVersionsMessage` | - | Returns descriptions of the available Amazon Redshift cluster versions. You can call this operation even before creating any clusters to learn more about the Amazon Redshift versions. |
| `DescribeClusters` | - | `paginated` | - | - | `ClustersMessage` | `ClusterNotFoundFault`, `InvalidTagFault` | Returns properties of provisioned clusters including general cluster properties, cluster database properties, maintenance and backup properties, and security and access properties. This operation supports pagination. |
| `DescribeCustomDomainAssociations` | - | `paginated` | - | - | `CustomDomainAssociationsMessage` | `CustomDomainAssociationNotFoundFault`, `UnsupportedOperationFault` | Contains information about custom domain associations for a cluster. |
| `DescribeDataShares` | - | `paginated` | - | - | `DescribeDataSharesResult` | `InvalidDataShareFault` | Shows the status of any inbound or outbound datashares available in the specified account. |
| `DescribeDataSharesForConsumer` | - | `paginated` | - | - | `DescribeDataSharesForConsumerResult` | `InvalidNamespaceFault` | Returns a list of datashares where the account identifier being called is a consumer account identifier. |
| `DescribeDataSharesForProducer` | - | `paginated` | - | - | `DescribeDataSharesForProducerResult` | `InvalidNamespaceFault` | Returns a list of datashares when the account identifier being called is a producer account identifier. |
| `DescribeDefaultClusterParameters` | - | `paginated` | `ParameterGroupFamily` | - | `DescribeDefaultClusterParametersResult` | - | Returns a list of parameter settings for the specified parameter group family. For more information about parameters and parameter groups, go to Amazon Redshift Parameter Groups in the Amazon Redshift Cluster Management Guide . |
| `DescribeEndpointAccess` | - | `paginated` | - | - | `EndpointAccessList` | `ClusterNotFoundFault`, `EndpointNotFoundFault`, `InvalidClusterStateFault` | Describes a Redshift-managed VPC endpoint. |
| `DescribeEndpointAuthorization` | - | `paginated` | - | - | `EndpointAuthorizationList` | `ClusterNotFoundFault`, `UnsupportedOperationFault` | Describes an endpoint authorization. |
| `DescribeEventCategories` | - | - | - | - | `EventCategoriesMessage` | - | Displays a list of event categories for all event source types, or for a specified source type. For a list of the event categories and source types, go to Amazon Redshift Event Notifications. |
| `DescribeEventSubscriptions` | - | `paginated` | - | - | `EventSubscriptionsMessage` | `InvalidTagFault`, `SubscriptionNotFoundFault` | Lists descriptions of all the Amazon Redshift event notification subscriptions for a customer account. If you specify a subscription name, lists the description for that subscription. |
| `DescribeEvents` | - | `paginated` | - | - | `EventsMessage` | - | Returns events related to clusters, security groups, snapshots, and parameter groups for the past 14 days. Events specific to a particular cluster, security group, snapshot or parameter group can be obtained by providing the name as a parameter. |
| `DescribeHsmClientCertificates` | - | `paginated` | - | - | `HsmClientCertificateMessage` | `HsmClientCertificateNotFoundFault`, `InvalidTagFault` | Returns information about the specified HSM client certificate. If no certificate ID is specified, returns information about all the HSM certificates owned by your Amazon Web Services account. |
| `DescribeHsmConfigurations` | - | `paginated` | - | - | `HsmConfigurationMessage` | `HsmConfigurationNotFoundFault`, `InvalidTagFault` | Returns information about the specified Amazon Redshift HSM configuration. If no configuration ID is specified, returns information about all the HSM configurations owned by your Amazon Web Services account. |
| `DescribeInboundIntegrations` | - | `paginated` | - | - | `InboundIntegrationsMessage` | `IntegrationNotFoundFault`, `InvalidNamespaceFault`, `UnsupportedOperationFault` | Returns a list of inbound integrations. |
| `DescribeIntegrations` | - | `paginated` | - | - | `IntegrationsMessage` | `IntegrationNotFoundFault`, `UnsupportedOperationFault` | Describes one or more zero-ETL or S3 event integrations with Amazon Redshift. |
| `DescribeLoggingStatus` | - | - | `ClusterIdentifier` | - | `LoggingStatus` | `ClusterNotFoundFault`, `UnsupportedOperationFault` | Describes whether information, such as queries and connection attempts, is being logged for the specified Amazon Redshift cluster. |
| `DescribeNodeConfigurationOptions` | - | `paginated` | `ActionType` | - | `NodeConfigurationOptionsMessage` | `AccessToSnapshotDeniedFault`, `ClusterNotFoundFault`, `ClusterSnapshotNotFoundFault`, `InvalidClusterSnapshotStateFault`, `UnsupportedOperationFault` | Returns properties of possible node configurations such as node type, number of nodes, and disk usage for the specified action type. |
| `DescribeOrderableClusterOptions` | - | `paginated` | - | - | `OrderableClusterOptionsMessage` | - | Returns a list of orderable cluster options. Before you create a new cluster you can use this operation to find what options are available, such as the EC2 Availability Zones (AZ) in the specific Amazon Web Services Region that you can specify, and the node... |
| `DescribePartners` | - | - | `AccountId`, `ClusterIdentifier` | - | `DescribePartnersOutputMessage` | `ClusterNotFoundFault`, `UnauthorizedPartnerIntegrationFault`, `UnsupportedOperationFault` | Returns information about the partner integrations defined for a cluster. |
| `DescribeRedshiftIdcApplications` | - | `paginated` | - | - | `DescribeRedshiftIdcApplicationsResult` | `DependentServiceAccessDeniedFault`, `DependentServiceUnavailableFault`, `RedshiftIdcApplicationNotExistsFault`, `UnsupportedOperationFault` | Lists the Amazon Redshift IAM Identity Center applications. |
| `DescribeReservedNodeExchangeStatus` | - | `paginated` | - | - | `DescribeReservedNodeExchangeStatusOutputMessage` | `ReservedNodeExchangeNotFoundFault`, `ReservedNodeNotFoundFault`, `UnsupportedOperationFault` | Returns exchange status details and associated metadata for a reserved-node exchange. Statuses include such values as in progress and requested. |
| `DescribeReservedNodeOfferings` | - | `paginated` | - | - | `ReservedNodeOfferingsMessage` | `DependentServiceUnavailableFault`, `ReservedNodeOfferingNotFoundFault`, `UnsupportedOperationFault` | Returns a list of the available reserved node offerings by Amazon Redshift with their descriptions including the node type, the fixed and recurring costs of reserving the node and duration the node will be reserved for you. These descriptions help you... |
| `DescribeReservedNodes` | - | `paginated` | - | - | `ReservedNodesMessage` | `DependentServiceUnavailableFault`, `ReservedNodeNotFoundFault` | Returns the descriptions of the reserved nodes. |
| `DescribeResize` | - | - | `ClusterIdentifier` | - | `ResizeProgressMessage` | `ClusterNotFoundFault`, `ResizeNotFoundFault`, `UnsupportedOperationFault` | Returns information about the last resize operation for the specified cluster. If no resize operation has ever been initiated for the specified cluster, a `HTTP 404` error is returned. |
| `DescribeScheduledActions` | - | `paginated` | - | - | `ScheduledActionsMessage` | `ScheduledActionNotFoundFault`, `UnauthorizedOperation` | Describes properties of scheduled actions. |
| `DescribeSnapshotCopyGrants` | - | `paginated` | - | - | `SnapshotCopyGrantMessage` | `InvalidTagFault`, `SnapshotCopyGrantNotFoundFault` | Returns a list of snapshot copy grants owned by the Amazon Web Services account in the destination region. For more information about managing snapshot copy grants, go to Amazon Redshift Database Encryption in the Amazon Redshift Cluster Management Guide . |
| `DescribeSnapshotSchedules` | - | `paginated` | - | - | `DescribeSnapshotSchedulesOutputMessage` | - | Returns a list of snapshot schedules. |
| `DescribeStorage` | - | - | - | - | `CustomerStorageMessage` | - | Returns account level backups storage size and provisional storage. |
| `DescribeTableRestoreStatus` | - | `paginated` | - | - | `TableRestoreStatusMessage` | `ClusterNotFoundFault`, `TableRestoreNotFoundFault` | Lists the status of one or more table restore requests made using the RestoreTableFromClusterSnapshot API action. If you don't specify a value for the `TableRestoreRequestId` parameter, then `DescribeTableRestoreStatus` returns the status of all table restore... |
| `DescribeTags` | - | `paginated` | - | - | `TaggedResourceListMessage` | `InvalidTagFault`, `ResourceNotFoundFault` | Returns a list of tags. You can return tags from a specific resource by specifying an ARN, or you can return all tags for a given type of resource, such as clusters, snapshots, and so on. |
| `DescribeUsageLimits` | - | `paginated` | - | - | `UsageLimitList` | `ClusterNotFoundFault`, `UnsupportedOperationFault` | Shows usage limits on a cluster. Results are filtered based on the combination of input usage limit identifier, cluster identifier, and feature type parameters: If usage limit identifier, cluster identifier, and feature type are not provided, then all usage... |
| `DisableLogging` | - | - | `ClusterIdentifier` | - | `LoggingStatus` | `ClusterNotFoundFault`, `InvalidClusterStateFault`, `UnsupportedOperationFault` | Stops logging information, such as queries and connection attempts, for the specified Amazon Redshift cluster. |
| `DisableSnapshotCopy` | - | - | `ClusterIdentifier` | - | `DisableSnapshotCopyResult` | `ClusterNotFoundFault`, `InvalidClusterStateFault`, `SnapshotCopyAlreadyDisabledFault`, `UnauthorizedOperation`, `UnsupportedOperationFault` | Disables the automatic copying of snapshots from one region to another region for a specified cluster. If your cluster and its snapshots are encrypted using an encrypted symmetric key from Key Management Service, use DeleteSnapshotCopyGrant to delete the... |
| `DisassociateDataShareConsumer` | - | - | `DataShareArn` | - | `DataShare` | `InvalidDataShareFault`, `InvalidNamespaceFault` | From a datashare consumer account, remove association for the specified datashare. |
| `EnableLogging` | - | - | `ClusterIdentifier` | - | `LoggingStatus` | `BucketNotFoundFault`, `ClusterNotFoundFault`, `InsufficientS3BucketPolicyFault`, `InvalidClusterStateFault`, `InvalidS3BucketNameFault`, `InvalidS3KeyPrefixFault`, `UnsupportedOperationFault` | Starts logging information, such as queries and connection attempts, for the specified Amazon Redshift cluster. |
| `EnableSnapshotCopy` | - | - | `ClusterIdentifier`, `DestinationRegion` | - | `EnableSnapshotCopyResult` | `ClusterNotFoundFault`, `CopyToRegionDisabledFault`, `DependentServiceRequestThrottlingFault`, `IncompatibleOrderableOptions`, `InvalidClusterStateFault`, `InvalidRetentionPeriodFault`, `LimitExceededFault`, `SnapshotCopyAlreadyEnabledFault`, ... (+3) | Enables the automatic copy of snapshots from one region to another region for a specified cluster. |
| `FailoverPrimaryCompute` | - | - | `ClusterIdentifier` | - | `FailoverPrimaryComputeResult` | `ClusterNotFoundFault`, `InvalidClusterStateFault`, `UnauthorizedOperation`, `UnsupportedOperationFault` | Fails over the primary compute unit of the specified Multi-AZ cluster to another Availability Zone. |
| `GetClusterCredentials` | - | - | `DbUser` | - | `ClusterCredentials` | `ClusterNotFoundFault`, `UnsupportedOperationFault` | Returns a database user name and temporary password with temporary authorization to log on to an Amazon Redshift database. The action returns the database user name prefixed with `IAM:` if `AutoCreate` is `False` or `IAMA:` if `AutoCreate` is `True`. |
| `GetClusterCredentialsWithIAM` | - | - | - | - | `ClusterExtendedCredentials` | `ClusterNotFoundFault`, `UnsupportedOperationFault` | Returns a database user name and temporary password with temporary authorization to log in to an Amazon Redshift database. The database user is mapped 1:1 to the source Identity and Access Management (IAM) identity. |
| `GetIdentityCenterAuthToken` | - | - | `ClusterIds` | - | `GetIdentityCenterAuthTokenResponse` | `ClusterNotFoundFault`, `InvalidClusterStateFault`, `RedshiftInvalidParameterFault`, `UnsupportedOperationFault` | Generates an encrypted authentication token that propagates the caller's Amazon Web Services IAM Identity Center identity to Amazon Redshift clusters. This API extracts the Amazon Web Services IAM Identity Center identity from enhanced credentials and creates... |
| `GetReservedNodeExchangeConfigurationOptions` | - | `paginated` | `ActionType` | - | `GetReservedNodeExchangeConfigurationOptionsOutputMessage` | `ClusterNotFoundFault`, `ClusterSnapshotNotFoundFault`, `DependentServiceUnavailableFault`, `InvalidReservedNodeStateFault`, `ReservedNodeAlreadyMigratedFault`, `ReservedNodeNotFoundFault`, `ReservedNodeOfferingNotFoundFault`, `UnsupportedOperationFault` | Gets the configuration options for the reserved-node exchange. These options include information about the source reserved node and target reserved node offering. |
| `GetReservedNodeExchangeOfferings` | - | `paginated` | `ReservedNodeId` | - | `GetReservedNodeExchangeOfferingsOutputMessage` | `DependentServiceUnavailableFault`, `InvalidReservedNodeStateFault`, `ReservedNodeAlreadyMigratedFault`, `ReservedNodeNotFoundFault`, `ReservedNodeOfferingNotFoundFault`, `UnsupportedOperationFault` | Returns an array of DC2 ReservedNodeOfferings that matches the payment type, term, and usage price of the given DC1 reserved node. |
| `GetResourcePolicy` | - | - | `ResourceArn` | - | `GetResourcePolicyResult` | `InvalidPolicyFault`, `ResourceNotFoundFault`, `UnsupportedOperationFault` | Get the resource policy for a specified resource. |
| `ListRecommendations` | - | `paginated` | - | - | `ListRecommendationsResult` | `ClusterNotFoundFault`, `UnsupportedOperationFault` | List the Amazon Redshift Advisor recommendations for one or multiple Amazon Redshift clusters in an Amazon Web Services account. |
| `ModifyAquaConfiguration` | - | - | `ClusterIdentifier` | - | `ModifyAquaOutputMessage` | `ClusterNotFoundFault`, `InvalidClusterStateFault`, `UnsupportedOperationFault` | This operation is retired. Calling this operation does not change AQUA configuration. |
| `ModifyAuthenticationProfile` | - | - | `AuthenticationProfileContent`, `AuthenticationProfileName` | - | `ModifyAuthenticationProfileResult` | `AuthenticationProfileNotFoundFault`, `AuthenticationProfileQuotaExceededFault`, `InvalidAuthenticationProfileRequestFault` | Modifies an authentication profile. |
| `ModifyCluster` | - | - | `ClusterIdentifier` | - | `ModifyClusterResult` | `ClusterAlreadyExistsFault`, `ClusterNotFoundFault`, `ClusterParameterGroupNotFoundFault`, `ClusterSecurityGroupNotFoundFault`, `CustomCnameAssociationFault`, `DependentServiceRequestThrottlingFault`, `HsmClientCertificateNotFoundFault`, `HsmConfigurationNotFoundFault`, ... (+14) | Modifies the settings for a cluster. You can also change node type and the number of nodes to scale up or down the cluster. |
| `ModifyClusterDbRevision` | - | - | `ClusterIdentifier`, `RevisionTarget` | - | `ModifyClusterDbRevisionResult` | `ClusterNotFoundFault`, `ClusterOnLatestRevisionFault`, `InvalidClusterStateFault`, `UnsupportedOperationFault` | Modifies the database revision of a cluster. The database revision is a unique revision of the database running in a cluster. |
| `ModifyClusterIamRoles` | - | - | `ClusterIdentifier` | - | `ModifyClusterIamRolesResult` | `ClusterNotFoundFault`, `InvalidClusterStateFault` | Modifies the list of Identity and Access Management (IAM) roles that can be used by the cluster to access other Amazon Web Services services. The maximum number of IAM roles that you can associate is subject to a quota. |
| `ModifyClusterMaintenance` | - | - | `ClusterIdentifier` | - | `ModifyClusterMaintenanceResult` | `ClusterNotFoundFault`, `InvalidClusterStateFault` | Modifies the maintenance settings of a cluster. |
| `ModifyClusterParameterGroup` | - | - | `ParameterGroupName`, `Parameters` | - | `ClusterParameterGroupNameMessage` | `ClusterParameterGroupNotFoundFault`, `InvalidClusterParameterGroupStateFault` | Modifies the parameters of a parameter group. For the parameters parameter, it can't contain ASCII characters. |
| `ModifyClusterSnapshot` | - | - | `SnapshotIdentifier` | - | `ModifyClusterSnapshotResult` | `ClusterSnapshotNotFoundFault`, `InvalidClusterSnapshotStateFault`, `InvalidRetentionPeriodFault` | Modifies the settings for a snapshot. This exanmple modifies the manual retention period setting for a cluster snapshot. |
| `ModifyClusterSnapshotSchedule` | - | - | `ClusterIdentifier` | - | `Unit` | `ClusterNotFoundFault`, `InvalidClusterSnapshotScheduleStateFault`, `SnapshotScheduleNotFoundFault` | Modifies a snapshot schedule for a cluster. |
| `ModifyClusterSubnetGroup` | - | - | `ClusterSubnetGroupName`, `SubnetIds` | - | `ModifyClusterSubnetGroupResult` | `ClusterSubnetGroupNotFoundFault`, `ClusterSubnetQuotaExceededFault`, `DependentServiceRequestThrottlingFault`, `InvalidSubnet`, `SubnetAlreadyInUse`, `UnauthorizedOperation` | Modifies a cluster subnet group to include the specified list of VPC subnets. The operation replaces the existing list of subnets with the new list of subnets. |
| `ModifyCustomDomainAssociation` | - | - | `ClusterIdentifier`, `CustomDomainCertificateArn`, `CustomDomainName` | - | `ModifyCustomDomainAssociationResult` | `ClusterNotFoundFault`, `CustomCnameAssociationFault`, `CustomDomainAssociationNotFoundFault`, `UnsupportedOperationFault` | Contains information for changing a custom domain association. |
| `ModifyEndpointAccess` | - | - | `EndpointName` | - | `EndpointAccess` | `ClusterNotFoundFault`, `EndpointNotFoundFault`, `InvalidClusterSecurityGroupStateFault`, `InvalidClusterStateFault`, `InvalidEndpointStateFault`, `UnauthorizedOperation` | Modifies a Redshift-managed VPC endpoint. |
| `ModifyEventSubscription` | - | - | `SubscriptionName` | - | `ModifyEventSubscriptionResult` | `InvalidSubscriptionStateFault`, `SNSInvalidTopicFault`, `SNSNoAuthorizationFault`, `SNSTopicArnNotFoundFault`, `SourceNotFoundFault`, `SubscriptionCategoryNotFoundFault`, `SubscriptionEventIdNotFoundFault`, `SubscriptionNotFoundFault`, ... (+1) | Modifies an existing Amazon Redshift event notification subscription. |
| `ModifyIntegration` | - | - | `IntegrationArn` | - | `Integration` | `IntegrationAlreadyExistsFault`, `IntegrationConflictOperationFault`, `IntegrationConflictStateFault`, `IntegrationNotFoundFault`, `UnsupportedOperationFault` | Modifies a zero-ETL integration or S3 event integration with Amazon Redshift. |
| `ModifyLakehouseConfiguration` | - | - | `ClusterIdentifier` | - | `LakehouseConfiguration` | `ClusterNotFoundFault`, `DependentServiceAccessDeniedFault`, `DependentServiceUnavailableFault`, `InvalidClusterStateFault`, `RedshiftIdcApplicationNotExistsFault`, `UnauthorizedOperation`, `UnsupportedOperationFault` | Modifies the lakehouse configuration for a cluster. This operation allows you to manage Amazon Redshift federated permissions and Amazon Web Services IAM Identity Center trusted identity propagation. |
| `ModifyRedshiftIdcApplication` | - | - | `RedshiftIdcApplicationArn` | - | `ModifyRedshiftIdcApplicationResult` | `DependentServiceAccessDeniedFault`, `DependentServiceUnavailableFault`, `RedshiftIdcApplicationNotExistsFault`, `UnsupportedOperationFault` | Changes an existing Amazon Redshift IAM Identity Center application. |
| `ModifyScheduledAction` | - | - | `ScheduledActionName` | - | `ScheduledAction` | `ClusterNotFoundFault`, `InvalidScheduleFault`, `InvalidScheduledActionFault`, `ScheduledActionNotFoundFault`, `ScheduledActionTypeUnsupportedFault`, `UnauthorizedOperation`, `UnsupportedOperationFault` | Modifies a scheduled action. |
| `ModifySnapshotCopyRetentionPeriod` | - | - | `ClusterIdentifier`, `RetentionPeriod` | - | `ModifySnapshotCopyRetentionPeriodResult` | `ClusterNotFoundFault`, `InvalidClusterStateFault`, `InvalidRetentionPeriodFault`, `SnapshotCopyDisabledFault`, `UnauthorizedOperation` | Modifies the number of days to retain snapshots in the destination Amazon Web Services Region after they are copied from the source Amazon Web Services Region. By default, this operation only changes the retention period of copied automated snapshots. |
| `ModifySnapshotSchedule` | - | - | `ScheduleDefinitions`, `ScheduleIdentifier` | - | `SnapshotSchedule` | `InvalidScheduleFault`, `SnapshotScheduleNotFoundFault`, `SnapshotScheduleUpdateInProgressFault` | Modifies a snapshot schedule. Any schedule associated with a cluster is modified asynchronously. |
| `ModifyUsageLimit` | - | - | `UsageLimitId` | - | `UsageLimit` | `InvalidUsageLimitFault`, `UnsupportedOperationFault`, `UsageLimitNotFoundFault` | Modifies a usage limit in a cluster. You can't modify the feature type or period of a usage limit. |
| `PauseCluster` | - | - | `ClusterIdentifier` | - | `PauseClusterResult` | `ClusterNotFoundFault`, `InvalidClusterStateFault`, `UnsupportedOperationFault` | Pauses a cluster. |
| `PurchaseReservedNodeOffering` | - | - | `ReservedNodeOfferingId` | - | `PurchaseReservedNodeOfferingResult` | `ReservedNodeAlreadyExistsFault`, `ReservedNodeOfferingNotFoundFault`, `ReservedNodeQuotaExceededFault`, `UnsupportedOperationFault` | Allows you to purchase reserved nodes. Amazon Redshift offers a predefined set of reserved node offerings. |
| `PutResourcePolicy` | - | - | `Policy`, `ResourceArn` | - | `PutResourcePolicyResult` | `ConflictPolicyUpdateFault`, `InvalidPolicyFault`, `ResourceNotFoundFault`, `UnsupportedOperationFault` | Updates the resource policy for a specified resource. |
| `RebootCluster` | - | - | `ClusterIdentifier` | - | `RebootClusterResult` | `ClusterNotFoundFault`, `InvalidClusterStateFault` | Reboots a cluster. This action is taken as soon as possible. |
| `RegisterNamespace` | - | - | `ConsumerIdentifiers`, `NamespaceIdentifier` | - | `RegisterNamespaceOutputMessage` | `ClusterNotFoundFault`, `InvalidClusterStateFault`, `InvalidNamespaceFault` | Registers a cluster or serverless namespace to the Amazon Web Services Glue Data Catalog. |
| `RejectDataShare` | - | - | `DataShareArn` | - | `DataShare` | `InvalidDataShareFault` | From a datashare consumer account, rejects the specified datashare. |
| `ResetClusterParameterGroup` | - | - | `ParameterGroupName` | - | `ClusterParameterGroupNameMessage` | `ClusterParameterGroupNotFoundFault`, `InvalidClusterParameterGroupStateFault` | Sets one or more parameters of the specified parameter group to their default values and sets the source values of the parameters to "engine-default". To reset the entire parameter group specify the ResetAllParameters parameter. |
| `ResizeCluster` | - | - | `ClusterIdentifier` | - | `ResizeClusterResult` | `ClusterNotFoundFault`, `DependentServiceUnavailableFault`, `InsufficientClusterCapacityFault`, `InvalidClusterStateFault`, `InvalidReservedNodeStateFault`, `LimitExceededFault`, `NumberOfNodesPerClusterLimitExceededFault`, `NumberOfNodesQuotaExceededFault`, ... (+7) | Changes the size of the cluster. You can change the cluster's type, or change the number or type of nodes. |
| `RestoreFromClusterSnapshot` | - | - | `ClusterIdentifier` | - | `RestoreFromClusterSnapshotResult` | `AccessToSnapshotDeniedFault`, `ClusterAlreadyExistsFault`, `ClusterParameterGroupNotFoundFault`, `ClusterQuotaExceededFault`, `ClusterSecurityGroupNotFoundFault`, `ClusterSnapshotNotFoundFault`, `ClusterSubnetGroupNotFoundFault`, `DependentServiceAccessDeniedFault`, ... (+27) | Creates a new cluster from a snapshot. By default, Amazon Redshift creates the resulting cluster with the same configuration as the original cluster from which the snapshot was created, except that the new cluster is created with the default cluster security... |
| `RestoreTableFromClusterSnapshot` | - | - | `ClusterIdentifier`, `NewTableName`, `SnapshotIdentifier`, `SourceDatabaseName`, `SourceTableName` | - | `RestoreTableFromClusterSnapshotResult` | `ClusterNotFoundFault`, `ClusterSnapshotNotFoundFault`, `InProgressTableRestoreQuotaExceededFault`, `InvalidClusterSnapshotStateFault`, `InvalidClusterStateFault`, `InvalidTableRestoreArgumentFault`, `UnsupportedOperationFault` | Creates a new table from a table in an Amazon Redshift cluster snapshot. You must create the new table within the Amazon Redshift cluster that the snapshot was taken from. |
| `ResumeCluster` | - | - | `ClusterIdentifier` | - | `ResumeClusterResult` | `ClusterNotFoundFault`, `InsufficientClusterCapacityFault`, `InvalidClusterStateFault`, `UnsupportedOperationFault` | Resumes a paused cluster. |
| `RevokeClusterSecurityGroupIngress` | - | - | `ClusterSecurityGroupName` | - | `RevokeClusterSecurityGroupIngressResult` | `AuthorizationNotFoundFault`, `ClusterSecurityGroupNotFoundFault`, `InvalidClusterSecurityGroupStateFault` | Revokes an ingress rule in an Amazon Redshift security group for a previously authorized IP range or Amazon EC2 security group. To add an ingress rule, see AuthorizeClusterSecurityGroupIngress. |
| `RevokeEndpointAccess` | - | - | - | - | `EndpointAuthorization` | `ClusterNotFoundFault`, `EndpointAuthorizationNotFoundFault`, `EndpointNotFoundFault`, `InvalidAuthorizationStateFault`, `InvalidClusterSecurityGroupStateFault`, `InvalidClusterStateFault`, `InvalidEndpointStateFault` | Revokes access to a cluster. |
| `RevokeSnapshotAccess` | - | - | `AccountWithRestoreAccess` | - | `RevokeSnapshotAccessResult` | `AccessToSnapshotDeniedFault`, `AuthorizationNotFoundFault`, `ClusterSnapshotNotFoundFault`, `UnsupportedOperationFault` | Removes the ability of the specified Amazon Web Services account to restore the specified snapshot. If the account is currently restoring the snapshot, the restore will run to completion. |
| `RotateEncryptionKey` | - | - | `ClusterIdentifier` | - | `RotateEncryptionKeyResult` | `ClusterNotFoundFault`, `DependentServiceRequestThrottlingFault`, `InvalidClusterStateFault`, `UnsupportedOperationFault` | Rotates the encryption keys for a cluster. |
| `UpdatePartnerStatus` | - | - | `AccountId`, `ClusterIdentifier`, `DatabaseName`, `PartnerName`, `Status` | - | `PartnerIntegrationOutputMessage` | `ClusterNotFoundFault`, `PartnerNotFoundFault`, `UnauthorizedPartnerIntegrationFault`, `UnsupportedOperationFault` | Updates the status of a partner integration. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `UnsupportedOperationFault` | `structure` | `message` | The requested operation isn't supported. |
| `ClusterNotFoundFault` | `structure` | `message` | The `ClusterIdentifier` parameter does not refer to an existing cluster. |
| `InvalidClusterStateFault` | `structure` | `message` | The specified cluster is not in the `available` state. |
| `InvalidTagFault` | `structure` | `message` | The tag is invalid. |
| `UnauthorizedOperation` | `structure` | `message` | Your account is not authorized to perform the requested operation. |
| `TagLimitExceededFault` | `structure` | `message` | You have exceeded the number of tags allowed. |
| `DependentServiceUnavailableFault` | `structure` | `message` | Your request cannot be completed because a dependent internal service is temporarily unavailable. |
| `ClusterSnapshotNotFoundFault` | `structure` | `message` | The snapshot identifier does not refer to an existing cluster snapshot. |
| `DependentServiceRequestThrottlingFault` | `structure` | `message` | The request cannot be completed because a dependent service is throttling requests made by Amazon Redshift on your behalf. |
| `InvalidRetentionPeriodFault` | `structure` | `message` | The retention period specified is either in the past or is not a valid value. |
| `InvalidClusterSecurityGroupStateFault` | `structure` | `message` | The state of the cluster security group is not `available`. |
| `LimitExceededFault` | `structure` | `message` | The encryption key has exceeded its grant limit in Amazon Web Services KMS. |
| `ClusterParameterGroupNotFoundFault` | `structure` | `message` | The parameter group name does not refer to an existing parameter group. |
| `ReservedNodeNotFoundFault` | `structure` | `message` | The specified reserved compute node not found. |
| `ReservedNodeOfferingNotFoundFault` | `structure` | `message` | Specified offering does not exist. |
| `InvalidNamespaceFault` | `structure` | `message` | The namespace isn't valid because the namespace doesn't exist. |
| `ClusterSecurityGroupNotFoundFault` | `structure` | `message` | The cluster security group name does not refer to an existing cluster security group. |
| `InvalidClusterSnapshotStateFault` | `structure` | `message` | The specified cluster snapshot is not in the `available` state, or other accounts are authorized to access the snapshot. |
| `DependentServiceAccessDeniedFault` | `structure` | `message` | A dependent service denied access for the integration. |
| `InvalidDataShareFault` | `structure` | `message` | There is an error with the datashare. |
| `ClusterSubnetGroupNotFoundFault` | `structure` | `message` | The cluster subnet group name does not refer to an existing cluster subnet group. |
| `RedshiftIdcApplicationNotExistsFault` | `structure` | `message` | The application you attempted to find doesn't exist. |
| `ResourceNotFoundFault` | `structure` | `message` | The resource could not be found. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
