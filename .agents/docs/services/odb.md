# odb

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Oracle Database@Amazon Web Services is an offering that enables you to access Oracle Exadata infrastructure managed by Oracle Cloud Infrastructure (OCI) inside Amazon Web Services data centers. You can migrate your Oracle Exadata workloads, establish low-latency connectivity with applications running on Amazon Web Services, and integrate with Amazon Web Services services. For example, you can run application servers in a Virtual Private Cloud (VPC) and access an Oracle Exadata system running in Oracle Database@Amazon Web Services. You can get started with Oracle Database@Amazon Web Services by using the familiar Amazon Web Services Management Console, APIs, or CLI. This interface reference for Oracle Database@Amazon Web Services contains documentation for a programming or command line interface that you can use to manage Oracle Database@Amazon Web Services.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for odb where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for odb by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for odb resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented odb workflows in the local mock. Key resources include `CloudAutonomousVmClusterResource`, `CloudExadataInfrastructureResource`, `CloudVmClusterResource`, `DbNodeResource`, `OdbNetworkResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Update` operation families, including `ListAutonomousVirtualMachines`, `ListCloudAutonomousVmClusters`, `ListCloudExadataInfrastructures`, `ListCloudVmClusters`, `GetCloudAutonomousVmCluster`, `GetCloudExadataInfrastructure`.

## Service Identity and Protocol

- AWS model slug: `odb`
- AWS SDK for Rust slug: `odb`
- Model version: `2024-08-20`
- Model file: `vendor/api-models-aws/models/odb/service/2024-08-20/odb-2024-08-20.json`
- SDK ID: `odb`
- Endpoint prefix: `-`
- ARN namespace: `odb`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (12), `Get` (9), `Create` (5), `Delete` (5), `Update` (3), `Accept` (1), `Associate` (1), `Disassociate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptMarketplaceRegistration`, `AssociateIamRoleToResource`, `CreateCloudAutonomousVmCluster`, `CreateCloudExadataInfrastructure`, `CreateCloudVmCluster`, `CreateOdbNetwork`, `CreateOdbPeeringConnection`, `DeleteCloudAutonomousVmCluster`, `DeleteCloudExadataInfrastructure`, `DeleteCloudVmCluster`, `DeleteOdbNetwork`, `DeleteOdbPeeringConnection`, `DisassociateIamRoleFromResource`, `StartDbNode`, `StopDbNode`, `TagResource`, `UntagResource`, `UpdateCloudExadataInfrastructure`, `UpdateOdbNetwork`, `UpdateOdbPeeringConnection`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetCloudAutonomousVmCluster`, `GetCloudExadataInfrastructure`, `GetCloudExadataInfrastructureUnallocatedResources`, `GetCloudVmCluster`, `GetDbNode`, `GetDbServer`, `GetOciOnboardingStatus`, `GetOdbNetwork`, `GetOdbPeeringConnection`, `ListAutonomousVirtualMachines`, `ListCloudAutonomousVmClusters`, `ListCloudExadataInfrastructures`, `ListCloudVmClusters`, `ListDbNodes`, `ListDbServers`, `ListDbSystemShapes`, `ListGiVersions`, `ListOdbNetworks`, `ListOdbPeeringConnections`, `ListSystemVersions`, ... (+1).
- Pagination is modelled for 11 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 14 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartDbNode`, `StopDbNode`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 43 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `EC2/VPC`, `ECS`, `STS`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `CloudAutonomousVmClusterResource` | `cloudAutonomousVmClusterId` | create: `CreateCloudAutonomousVmCluster`; read: `GetCloudAutonomousVmCluster`; delete: `DeleteCloudAutonomousVmCluster`; list: `ListCloudAutonomousVmClusters` | `ListAutonomousVirtualMachines` | - |
| `CloudExadataInfrastructureResource` | `cloudExadataInfrastructureId` | create: `CreateCloudExadataInfrastructure`; read: `GetCloudExadataInfrastructure`; update: `UpdateCloudExadataInfrastructure`; delete: `DeleteCloudExadataInfrastructure`; list: `ListCloudExadataInfrastructures` | `GetCloudExadataInfrastructureUnallocatedResources`, `GetDbServer`, `ListDbServers` | - |
| `CloudVmClusterResource` | `cloudVmClusterId` | create: `CreateCloudVmCluster`; read: `GetCloudVmCluster`; delete: `DeleteCloudVmCluster`; list: `ListCloudVmClusters` | - | - |
| `DbNodeResource` | `dbNodeId` | read: `GetDbNode`; list: `ListDbNodes` | `RebootDbNode`, `StartDbNode`, `StopDbNode` | - |
| `OdbNetworkResource` | `odbNetworkId` | create: `CreateOdbNetwork`; read: `GetOdbNetwork`; update: `UpdateOdbNetwork`; delete: `DeleteOdbNetwork`; list: `ListOdbNetworks` | - | - |
| `OdbPeeringConnectionResource` | `odbPeeringConnectionId` | create: `CreateOdbPeeringConnection`; read: `GetOdbPeeringConnection`; update: `UpdateOdbPeeringConnection`; delete: `DeleteOdbPeeringConnection`; list: `ListOdbPeeringConnections` | - | - |
## Operation Groups

### List

- Operations: `ListAutonomousVirtualMachines`, `ListCloudAutonomousVmClusters`, `ListCloudExadataInfrastructures`, `ListCloudVmClusters`, `ListDbNodes`, `ListDbServers`, `ListDbSystemShapes`, `ListGiVersions`, `ListOdbNetworks`, `ListOdbPeeringConnections`, `ListSystemVersions`, `ListTagsForResource`
- Traits: `paginated` (11), `readonly` (12)
- Common required input members in this group: `cloudAutonomousVmClusterId`, `cloudExadataInfrastructureId`, `cloudVmClusterId`, `giVersion`, `resourceArn`, `shape`

### Get

- Operations: `GetCloudAutonomousVmCluster`, `GetCloudExadataInfrastructure`, `GetCloudExadataInfrastructureUnallocatedResources`, `GetCloudVmCluster`, `GetDbNode`, `GetDbServer`, `GetOciOnboardingStatus`, `GetOdbNetwork`, `GetOdbPeeringConnection`
- Traits: `readonly` (9)
- Common required input members in this group: `cloudAutonomousVmClusterId`, `cloudExadataInfrastructureId`, `cloudVmClusterId`, `dbNodeId`, `dbServerId`, `odbNetworkId`, `odbPeeringConnectionId`

### Create

- Operations: `CreateCloudAutonomousVmCluster`, `CreateCloudExadataInfrastructure`, `CreateCloudVmCluster`, `CreateOdbNetwork`, `CreateOdbPeeringConnection`
- Traits: `idempotency-token` (5), `idempotent` (5)
- Common required input members in this group: `autonomousDataStorageSizeInTBs`, `clientSubnetCidr`, `cloudExadataInfrastructureId`, `computeCount`, `cpuCoreCount`, `cpuCoreCountPerNode`, `displayName`, `giVersion`, `hostname`, `memoryPerOracleComputeUnitInGBs`, `odbNetworkId`, `peerNetworkId`, `shape`, `sshPublicKeys`, `storageCount`, `totalContainerDatabases`

### Delete

- Operations: `DeleteCloudAutonomousVmCluster`, `DeleteCloudExadataInfrastructure`, `DeleteCloudVmCluster`, `DeleteOdbNetwork`, `DeleteOdbPeeringConnection`
- Traits: `idempotent` (5)
- Common required input members in this group: `cloudAutonomousVmClusterId`, `cloudExadataInfrastructureId`, `cloudVmClusterId`, `deleteAssociatedResources`, `odbNetworkId`, `odbPeeringConnectionId`

### Update

- Operations: `UpdateCloudExadataInfrastructure`, `UpdateOdbNetwork`, `UpdateOdbPeeringConnection`
- Common required input members in this group: `cloudExadataInfrastructureId`, `odbNetworkId`, `odbPeeringConnectionId`

### Accept

- Operations: `AcceptMarketplaceRegistration`
- Traits: `idempotent` (1)
- Common required input members in this group: `marketplaceRegistrationToken`

### Associate

- Operations: `AssociateIamRoleToResource`
- Common required input members in this group: `awsIntegration`, `iamRoleArn`, `resourceArn`

### Disassociate

- Operations: `DisassociateIamRoleFromResource`
- Common required input members in this group: `awsIntegration`, `iamRoleArn`, `resourceArn`

### Initialize

- Operations: `InitializeService`
- Traits: `idempotent` (1)

### Reboot

- Operations: `RebootDbNode`
- Common required input members in this group: `cloudVmClusterId`, `dbNodeId`

### Start

- Operations: `StartDbNode`
- Common required input members in this group: `cloudVmClusterId`, `dbNodeId`

### Stop

- Operations: `StopDbNode`
- Common required input members in this group: `cloudVmClusterId`, `dbNodeId`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptMarketplaceRegistration` | - | `idempotent` | `marketplaceRegistrationToken` | - | `AcceptMarketplaceRegistrationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Registers the Amazon Web Services Marketplace token for your Amazon Web Services account to activate your Oracle Database@Amazon Web Services subscription. |
| `AssociateIamRoleToResource` | - | - | `awsIntegration`, `iamRoleArn`, `resourceArn` | - | `AssociateIamRoleToResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates an Amazon Web Services Identity and Access Management (IAM) service role with a specified resource to enable Amazon Web Services service integration. |
| `CreateCloudAutonomousVmCluster` | - | `idempotent`, `idempotency-token` | `autonomousDataStorageSizeInTBs`, `cloudExadataInfrastructureId`, `cpuCoreCountPerNode`, `displayName`, `memoryPerOracleComputeUnitInGBs`, `odbNetworkId`, `totalContainerDatabases` | `clientToken` | `CreateCloudAutonomousVmClusterOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new Autonomous VM cluster in the specified Exadata infrastructure. |
| `CreateCloudExadataInfrastructure` | - | `idempotent`, `idempotency-token` | `computeCount`, `displayName`, `shape`, `storageCount` | `clientToken` | `CreateCloudExadataInfrastructureOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Exadata infrastructure. |
| `CreateCloudVmCluster` | - | `idempotent`, `idempotency-token` | `cloudExadataInfrastructureId`, `cpuCoreCount`, `displayName`, `giVersion`, `hostname`, `odbNetworkId`, `sshPublicKeys` | `clientToken` | `CreateCloudVmClusterOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a VM cluster on the specified Exadata infrastructure. |
| `CreateOdbNetwork` | - | `idempotent`, `idempotency-token` | `clientSubnetCidr`, `displayName` | `clientToken` | `CreateOdbNetworkOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an ODB network. |
| `CreateOdbPeeringConnection` | - | `idempotent`, `idempotency-token` | `odbNetworkId`, `peerNetworkId` | `clientToken` | `CreateOdbPeeringConnectionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a peering connection between an ODB network and a VPC. A peering connection enables private connectivity between the networks for application-tier communication. |
| `DeleteCloudAutonomousVmCluster` | - | `idempotent` | `cloudAutonomousVmClusterId` | - | `DeleteCloudAutonomousVmClusterOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes an Autonomous VM cluster. |
| `DeleteCloudExadataInfrastructure` | - | `idempotent` | `cloudExadataInfrastructureId` | - | `DeleteCloudExadataInfrastructureOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified Exadata infrastructure. Before you use this operation, make sure to delete all of the VM clusters that are hosted on this Exadata infrastructure. |
| `DeleteCloudVmCluster` | - | `idempotent` | `cloudVmClusterId` | - | `DeleteCloudVmClusterOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes the specified VM cluster. |
| `DeleteOdbNetwork` | - | `idempotent` | `deleteAssociatedResources`, `odbNetworkId` | - | `DeleteOdbNetworkOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified ODB network. |
| `DeleteOdbPeeringConnection` | - | `idempotent` | `odbPeeringConnectionId` | - | `DeleteOdbPeeringConnectionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an ODB peering connection. When you delete an ODB peering connection, the underlying VPC peering connection is also deleted. |
| `DisassociateIamRoleFromResource` | - | - | `awsIntegration`, `iamRoleArn`, `resourceArn` | - | `DisassociateIamRoleFromResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates an Amazon Web Services Identity and Access Management (IAM) service role from a specified resource to disable Amazon Web Services service integration. |
| `GetCloudAutonomousVmCluster` | - | `readonly` | `cloudAutonomousVmClusterId` | - | `GetCloudAutonomousVmClusterOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a specific Autonomous VM cluster. |
| `GetCloudExadataInfrastructure` | - | `readonly` | `cloudExadataInfrastructureId` | - | `GetCloudExadataInfrastructureOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about the specified Exadata infrastructure. |
| `GetCloudExadataInfrastructureUnallocatedResources` | - | `readonly` | `cloudExadataInfrastructureId` | - | `GetCloudExadataInfrastructureUnallocatedResourcesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about unallocated resources in a specified Cloud Exadata Infrastructure. |
| `GetCloudVmCluster` | - | `readonly` | `cloudVmClusterId` | - | `GetCloudVmClusterOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about the specified VM cluster. |
| `GetDbNode` | - | `readonly` | `cloudVmClusterId`, `dbNodeId` | - | `GetDbNodeOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about the specified DB node. |
| `GetDbServer` | - | `readonly` | `cloudExadataInfrastructureId`, `dbServerId` | - | `GetDbServerOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about the specified database server. |
| `GetOciOnboardingStatus` | - | `readonly` | - | - | `GetOciOnboardingStatusOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns the tenancy activation link and onboarding status for your Amazon Web Services account. |
| `GetOdbNetwork` | - | `readonly` | `odbNetworkId` | - | `GetOdbNetworkOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about the specified ODB network. |
| `GetOdbPeeringConnection` | - | `readonly` | `odbPeeringConnectionId` | - | `GetOdbPeeringConnectionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about an ODB peering connection. |
| `InitializeService` | - | `idempotent` | - | - | `InitializeServiceOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Initializes the ODB service for the first time in an account. |
| `ListAutonomousVirtualMachines` | - | `readonly`, `paginated` | `cloudAutonomousVmClusterId` | - | `ListAutonomousVirtualMachinesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all Autonomous VMs in an Autonomous VM cluster. |
| `ListCloudAutonomousVmClusters` | - | `readonly`, `paginated` | - | - | `ListCloudAutonomousVmClustersOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all Autonomous VM clusters in a specified Cloud Exadata infrastructure. |
| `ListCloudExadataInfrastructures` | - | `readonly`, `paginated` | - | - | `ListCloudExadataInfrastructuresOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns information about the Exadata infrastructures owned by your Amazon Web Services account. |
| `ListCloudVmClusters` | - | `readonly`, `paginated` | - | - | `ListCloudVmClustersOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about the VM clusters owned by your Amazon Web Services account or only the ones on the specified Exadata infrastructure. |
| `ListDbNodes` | - | `readonly`, `paginated` | `cloudVmClusterId` | - | `ListDbNodesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about the DB nodes for the specified VM cluster. |
| `ListDbServers` | - | `readonly`, `paginated` | `cloudExadataInfrastructureId` | - | `ListDbServersOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about the database servers that belong to the specified Exadata infrastructure. |
| `ListDbSystemShapes` | - | `readonly`, `paginated` | - | - | `ListDbSystemShapesOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns information about the shapes that are available for an Exadata infrastructure. |
| `ListGiVersions` | - | `readonly`, `paginated` | - | - | `ListGiVersionsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns information about Oracle Grid Infrastructure (GI) software versions that are available for a VM cluster for the specified shape. |
| `ListOdbNetworks` | - | `readonly`, `paginated` | - | - | `ListOdbNetworksOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns information about the ODB networks owned by your Amazon Web Services account. |
| `ListOdbPeeringConnections` | - | `readonly`, `paginated` | - | - | `ListOdbPeeringConnectionsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all ODB peering connections or those associated with a specific ODB network. |
| `ListSystemVersions` | - | `readonly`, `paginated` | `giVersion`, `shape` | - | `ListSystemVersionsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about the system versions that are available for a VM cluster for the specified `giVersion` and `shape`. |
| `ListTagsForResource` | - | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | Returns information about the tags applied to this resource. |
| `RebootDbNode` | - | - | `cloudVmClusterId`, `dbNodeId` | - | `RebootDbNodeOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Reboots the specified DB node in a VM cluster. |
| `StartDbNode` | - | - | `cloudVmClusterId`, `dbNodeId` | - | `StartDbNodeOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts the specified DB node in a VM cluster. |
| `StopDbNode` | - | - | `cloudVmClusterId`, `dbNodeId` | - | `StopDbNodeOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops the specified DB node in a VM cluster. |
| `TagResource` | - | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `ServiceQuotaExceededException` | Applies tags to the specified resource. |
| `UntagResource` | - | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException` | Removes tags from the specified resource. |
| `UpdateCloudExadataInfrastructure` | - | - | `cloudExadataInfrastructureId` | - | `UpdateCloudExadataInfrastructureOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the properties of an Exadata infrastructure resource. |
| `UpdateOdbNetwork` | - | - | `odbNetworkId` | - | `UpdateOdbNetworkOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates properties of a specified ODB network. |
| `UpdateOdbPeeringConnection` | - | - | `odbPeeringConnectionId` | - | `UpdateOdbPeeringConnectionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Modifies the settings of an Oracle Database@Amazon Web Services peering connection. You can update the display name and add or remove CIDR blocks from the peering connection. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListDbSystemShapes` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListGiVersions` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListSystemVersions` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You don't have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message`, `retryAfterSeconds` | Occurs when there is an internal failure in the Oracle Database@Amazon Web Services service. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | The request has failed validation because it is missing required fields or has invalid inputs. |
| `ThrottlingException` | `structure` | `message`, `retryAfterSeconds` | The request was denied due to request throttling. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The operation tried to access a resource that doesn't exist. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | Occurs when a conflict with the current status of your resource. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType` | You have exceeded the service quota. |
| `AcceptMarketplaceRegistrationInput` | `structure` | `marketplaceRegistrationToken` | - |
| `AcceptMarketplaceRegistrationOutput` | `structure` | - | - |
| `AssociateIamRoleToResourceInput` | `structure` | `awsIntegration`, `iamRoleArn`, `resourceArn` | - |
| `AssociateIamRoleToResourceOutput` | `structure` | - | - |
| `CreateCloudAutonomousVmClusterInput` | `structure` | `autonomousDataStorageSizeInTBs`, `clientToken`, `cloudExadataInfrastructureId`, `cpuCoreCountPerNode`, `dbServers`, `description`, `displayName`, `isMtlsEnabledVmCluster`, `licenseModel`, `maintenanceWindow`, `memoryPerOracleComputeUnitInGBs`, `odbNetworkId`, ... (+5) | - |
| `CreateCloudAutonomousVmClusterOutput` | `structure` | `cloudAutonomousVmClusterId`, `displayName`, `status`, `statusReason` | - |
| `CreateCloudExadataInfrastructureInput` | `structure` | `availabilityZone`, `availabilityZoneId`, `clientToken`, `computeCount`, `customerContactsToSendToOCI`, `databaseServerType`, `displayName`, `maintenanceWindow`, `shape`, `storageCount`, `storageServerType`, `tags` | - |
| `CreateCloudExadataInfrastructureOutput` | `structure` | `cloudExadataInfrastructureId`, `displayName`, `status`, `statusReason` | - |
| `CreateCloudVmClusterInput` | `structure` | `clientToken`, `cloudExadataInfrastructureId`, `clusterName`, `cpuCoreCount`, `dataCollectionOptions`, `dataStorageSizeInTBs`, `dbNodeStorageSizeInGBs`, `dbServers`, `displayName`, `giVersion`, `hostname`, `isLocalBackupEnabled`, ... (+9) | - |
| `CreateCloudVmClusterOutput` | `structure` | `cloudVmClusterId`, `displayName`, `status`, `statusReason` | - |
| `CreateOdbNetworkInput` | `structure` | `availabilityZone`, `availabilityZoneId`, `backupSubnetCidr`, `clientSubnetCidr`, `clientToken`, `crossRegionS3RestoreSourcesToEnable`, `customDomainName`, `defaultDnsPrefix`, `displayName`, `kmsAccess`, `kmsPolicyDocument`, `s3Access`, ... (+5) | - |
| `CreateOdbNetworkOutput` | `structure` | `displayName`, `odbNetworkId`, `status`, `statusReason` | - |
| `CreateOdbPeeringConnectionInput` | `structure` | `clientToken`, `displayName`, `odbNetworkId`, `peerNetworkCidrsToBeAdded`, `peerNetworkId`, `peerNetworkRouteTableIds`, `tags` | - |
| `CreateOdbPeeringConnectionOutput` | `structure` | `displayName`, `odbPeeringConnectionId`, `status`, `statusReason` | - |
| `DeleteCloudAutonomousVmClusterInput` | `structure` | `cloudAutonomousVmClusterId` | - |
| `DeleteCloudAutonomousVmClusterOutput` | `structure` | - | - |
| `DeleteCloudExadataInfrastructureInput` | `structure` | `cloudExadataInfrastructureId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
