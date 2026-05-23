# Amazon FSx

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon FSx is a fully managed service that makes it easy for storage and application administrators to launch and use shared file storage.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon FSx where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon FSx by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for Amazon FSx resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon FSx workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Create`, `Describe`, `Delete`, `Update`, `Copy` operation families, including `CreateAndAttachS3AccessPoint`, `CreateBackup`, `CreateDataRepositoryAssociation`, `CreateDataRepositoryTask`, `DescribeBackups`, `DescribeDataRepositoryAssociations`.

## Service Identity and Protocol

- AWS model slug: `fsx`
- AWS SDK for Rust slug: `fsx`
- Model version: `2018-03-01`
- Model file: `vendor/api-models-aws/models/fsx/service/2018-03-01/fsx-2018-03-01.json`
- SDK ID: `FSx`
- Endpoint prefix: `fsx`
- ARN namespace: `fsx`
- CloudFormation name: `FSx`
- CloudTrail event source: `fsx.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Create` (11), `Describe` (11), `Delete` (7), `Update` (7), `Copy` (2), `Associate` (1), `Cancel` (1), `Detach` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateFileSystemAliases`, `CancelDataRepositoryTask`, `CreateAndAttachS3AccessPoint`, `CreateBackup`, `CreateDataRepositoryAssociation`, `CreateDataRepositoryTask`, `CreateFileCache`, `CreateFileSystem`, `CreateFileSystemFromBackup`, `CreateSnapshot`, `CreateStorageVirtualMachine`, `CreateVolume`, `CreateVolumeFromBackup`, `DeleteBackup`, `DeleteDataRepositoryAssociation`, `DeleteFileCache`, `DeleteFileSystem`, `DeleteSnapshot`, `DeleteStorageVirtualMachine`, `DeleteVolume`, ... (+13).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeBackups`, `DescribeDataRepositoryAssociations`, `DescribeDataRepositoryTasks`, `DescribeFileCaches`, `DescribeFileSystemAliases`, `DescribeFileSystems`, `DescribeS3AccessPointAttachments`, `DescribeSharedVpcConfiguration`, `DescribeSnapshots`, `DescribeStorageVirtualMachines`, `DescribeVolumes`, `ListTagsForResource`.
- Pagination is modelled for 11 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 39 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelDataRepositoryTask`, `CreateDataRepositoryTask`, `DescribeDataRepositoryTasks`, `StartMisconfiguredStateRecovery`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 48 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `EC2/VPC`, `ECR`, `ECS`, `STS`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/what-is-fsx-ontap.html
- https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/how-it-works-fsx-ontap.html
- https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/managing-file-systems.html

Research outcomes:
- Amazon FSx provides managed file systems for multiple engines, including NetApp ONTAP, Windows File Server, Lustre, and OpenZFS.
- FSx for ONTAP file systems contain storage virtual machines and volumes, support multi-protocol access, and can use SSD storage plus capacity-tiering.
- Storage virtual machines provide isolated file-serving endpoints and identities within an ONTAP file system.
- Volumes are thin-provisioned data containers that expose storage through protocols such as NFS, SMB, and iSCSI depending on configuration.
- FSx supports Single-AZ and Multi-AZ deployment patterns depending on engine and file system type.
- FSx resources integrate with backups, snapshots, tags, security groups, subnets, and KMS encryption.
- Some engines have engine-specific resources and lifecycle rules, so a single generic file-system model is insufficient.

Parity implications:
- Model file systems, engine type, deployment type, backups, storage virtual machines, volumes, endpoints, security/networking, tags, and KMS settings separately.
- Engine-specific validation should drive create/modify fields and child resources.
- Backup and restore behaviour should preserve engine-specific metadata.

## Current Network Resource Stub Semantics

FSx currently treats file-system networking fields as local file-system metadata.

- Create paths store subnet IDs, security group IDs, preferred subnet IDs, endpoint IP address ranges, and route-table IDs where the implemented file-system type supports them.
- The service mints FSx lifecycle and endpoint-style data internally and returns the stored network fields through describe calls.
- There is no EC2 subnet, security group, route table, or ENI reconciliation.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Create

- Operations: `CreateAndAttachS3AccessPoint`, `CreateBackup`, `CreateDataRepositoryAssociation`, `CreateDataRepositoryTask`, `CreateFileCache`, `CreateFileSystem`, `CreateFileSystemFromBackup`, `CreateSnapshot`, `CreateStorageVirtualMachine`, `CreateVolume`, `CreateVolumeFromBackup`
- Traits: `idempotency-token` (11), `idempotent` (5)
- Common required input members in this group: `Name`, `Type`, `FileSystemId`, `SubnetIds`, `BackupId`

### Describe

- Operations: `DescribeBackups`, `DescribeDataRepositoryAssociations`, `DescribeDataRepositoryTasks`, `DescribeFileCaches`, `DescribeFileSystemAliases`, `DescribeFileSystems`, `DescribeS3AccessPointAttachments`, `DescribeSharedVpcConfiguration`, `DescribeSnapshots`, `DescribeStorageVirtualMachines`, `DescribeVolumes`
- Traits: `paginated` (10), `idempotent` (2), `idempotency-token` (1)
- Common required input members in this group: -

### Delete

- Operations: `DeleteBackup`, `DeleteDataRepositoryAssociation`, `DeleteFileCache`, `DeleteFileSystem`, `DeleteSnapshot`, `DeleteStorageVirtualMachine`, `DeleteVolume`
- Traits: `idempotent` (5), `idempotency-token` (7)
- Common required input members in this group: -

### Update

- Operations: `UpdateDataRepositoryAssociation`, `UpdateFileCache`, `UpdateFileSystem`, `UpdateSharedVpcConfiguration`, `UpdateSnapshot`, `UpdateStorageVirtualMachine`, `UpdateVolume`
- Traits: `idempotent` (3), `idempotency-token` (7)
- Common required input members in this group: -

### Copy

- Operations: `CopyBackup`, `CopySnapshotAndUpdateVolume`
- Traits: `idempotent` (2), `idempotency-token` (2)
- Common required input members in this group: -

### Associate

- Operations: `AssociateFileSystemAliases`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Cancel

- Operations: `CancelDataRepositoryTask`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Detach

- Operations: `DetachAndDeleteS3AccessPoint`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateFileSystemAliases`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### List

- Operations: `ListTagsForResource`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Release

- Operations: `ReleaseFileSystemNfsV3Locks`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Restore

- Operations: `RestoreVolumeFromSnapshot`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Start

- Operations: `StartMisconfiguredStateRecovery`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateFileSystemAliases` | `-` | `idempotency-token` | `FileSystemId`, `Aliases` | `ClientRequestToken` | `AssociateFileSystemAliasesResponse` | `BadRequest`, `FileSystemNotFound`, `InternalServerError` | Use this action to associate one or more Domain Name Server (DNS) aliases with an existing Amazon FSx for Windows File Server file system. A file system can have a maximum of 50 DNS aliases associated with it at any ... |
| `CancelDataRepositoryTask` | `-` | `idempotent` | `TaskId` | - | `CancelDataRepositoryTaskResponse` | `BadRequest`, `DataRepositoryTaskEnded`, `DataRepositoryTaskNotFound`, `InternalServerError`, `UnsupportedOperation` | Cancels an existing Amazon FSx for Lustre data repository task if that task is in either the PENDING or EXECUTING state. When you cancel an export task, Amazon FSx does the following. Any files that FSx has already e ... |
| `CopyBackup` | `-` | `idempotent`, `idempotency-token` | `SourceBackupId` | `ClientRequestToken` | `CopyBackupResponse` | `BackupNotFound`, `BadRequest`, `IncompatibleParameterError`, `IncompatibleRegionForMultiAZ`, `InternalServerError`, `InvalidDestinationKmsKey`, `InvalidRegion`, `InvalidSourceKmsKey`, `ServiceLimitExceeded`, `SourceBackupUnavailable`, `UnsupportedOperation` | Copies an existing backup within the same Amazon Web Services account to another Amazon Web Services Region (cross-Region copy) or within the same Amazon Web Services Region (in-Region copy). You can have up to five ... |
| `CopySnapshotAndUpdateVolume` | `-` | `idempotent`, `idempotency-token` | `VolumeId`, `SourceSnapshotARN` | `ClientRequestToken` | `CopySnapshotAndUpdateVolumeResponse` | `BadRequest`, `IncompatibleParameterError`, `InternalServerError`, `ServiceLimitExceeded` | Updates an existing volume by using a snapshot from another Amazon FSx for OpenZFS file system. For more information, see on-demand data replication in the Amazon FSx for OpenZFS User Guide. |
| `CreateAndAttachS3AccessPoint` | `-` | `idempotency-token` | `Name`, `Type` | `ClientRequestToken` | `CreateAndAttachS3AccessPointResponse` | `AccessPointAlreadyOwnedByYou`, `BadRequest`, `IncompatibleParameterError`, `InternalServerError`, `InvalidAccessPoint`, `InvalidRequest`, `TooManyAccessPoints`, `UnsupportedOperation`, `VolumeNotFound` | Creates an S3 access point and attaches it to an Amazon FSx volume. For FSx for OpenZFS file systems, the volume must be hosted on a high-availability file system, either Single-AZ or Multi-AZ. For more information, ... |
| `CreateBackup` | `-` | `idempotent`, `idempotency-token` | - | `ClientRequestToken` | `CreateBackupResponse` | `BackupInProgress`, `BadRequest`, `FileSystemNotFound`, `IncompatibleParameterError`, `InternalServerError`, `ServiceLimitExceeded`, `UnsupportedOperation`, `VolumeNotFound` | Creates a backup of an existing Amazon FSx for Windows File Server file system, Amazon FSx for Lustre file system, Amazon FSx for NetApp ONTAP volume, or Amazon FSx for OpenZFS file system. We recommend creating regu ... |
| `CreateDataRepositoryAssociation` | `-` | `idempotent`, `idempotency-token` | `FileSystemId`, `DataRepositoryPath` | `ClientRequestToken` | `CreateDataRepositoryAssociationResponse` | `BadRequest`, `FileSystemNotFound`, `IncompatibleParameterError`, `InternalServerError`, `ServiceLimitExceeded`, `UnsupportedOperation` | Creates an Amazon FSx for Lustre data repository association (DRA). A data repository association is a link between a directory on the file system and an Amazon S3 bucket or prefix. You can have a maximum of 8 data r ... |
| `CreateDataRepositoryTask` | `-` | `idempotent`, `idempotency-token` | `Type`, `FileSystemId`, `Report` | `ClientRequestToken` | `CreateDataRepositoryTaskResponse` | `BadRequest`, `DataRepositoryTaskExecuting`, `FileSystemNotFound`, `IncompatibleParameterError`, `InternalServerError`, `ServiceLimitExceeded`, `UnsupportedOperation` | Creates an Amazon FSx for Lustre data repository task. A CreateDataRepositoryTask operation will fail if a data repository is not linked to the FSx file system. You use import and export data repository tasks to perf ... |
| `CreateFileCache` | `-` | `idempotent`, `idempotency-token` | `FileCacheType`, `FileCacheTypeVersion`, `StorageCapacity`, `SubnetIds` | `ClientRequestToken` | `CreateFileCacheResponse` | `BadRequest`, `IncompatibleParameterError`, `InternalServerError`, `InvalidNetworkSettings`, `InvalidPerUnitStorageThroughput`, `MissingFileCacheConfiguration`, `ServiceLimitExceeded` | Creates a new Amazon File Cache resource. You can use this operation with a client request token in the request that Amazon File Cache uses to ensure idempotent creation. If a cache with the specified client request ... |
| `CreateFileSystem` | `-` | `idempotency-token` | `FileSystemType`, `SubnetIds` | `ClientRequestToken` | `CreateFileSystemResponse` | `ActiveDirectoryError`, `BadRequest`, `IncompatibleParameterError`, `InternalServerError`, `InvalidExportPath`, `InvalidImportPath`, `InvalidNetworkSettings`, `InvalidPerUnitStorageThroughput`, `MissingFileSystemConfiguration`, `ServiceLimitExceeded` | Creates a new, empty Amazon FSx file system. You can create the following supported Amazon FSx file systems using the CreateFileSystem API operation: Amazon FSx for Lustre Amazon FSx for NetApp ONTAP Amazon FSx for O ... |
| `CreateFileSystemFromBackup` | `-` | `idempotency-token` | `BackupId`, `SubnetIds` | `ClientRequestToken` | `CreateFileSystemFromBackupResponse` | `ActiveDirectoryError`, `BackupNotFound`, `BadRequest`, `IncompatibleParameterError`, `InternalServerError`, `InvalidNetworkSettings`, `InvalidPerUnitStorageThroughput`, `MissingFileSystemConfiguration`, `ServiceLimitExceeded` | Creates a new Amazon FSx for Lustre, Amazon FSx for Windows File Server, or Amazon FSx for OpenZFS file system from an existing Amazon FSx backup. If a file system with the specified client request token exists and t ... |
| `CreateSnapshot` | `-` | `idempotent`, `idempotency-token` | `Name`, `VolumeId` | `ClientRequestToken` | `CreateSnapshotResponse` | `BadRequest`, `InternalServerError`, `ServiceLimitExceeded`, `VolumeNotFound` | Creates a snapshot of an existing Amazon FSx for OpenZFS volume. With snapshots, you can easily undo file changes and compare file versions by restoring the volume to a previous version. If a snapshot with the specif ... |
| `CreateStorageVirtualMachine` | `-` | `idempotency-token` | `FileSystemId`, `Name` | `ClientRequestToken` | `CreateStorageVirtualMachineResponse` | `ActiveDirectoryError`, `BadRequest`, `FileSystemNotFound`, `IncompatibleParameterError`, `InternalServerError`, `ServiceLimitExceeded`, `UnsupportedOperation` | Creates a storage virtual machine (SVM) for an Amazon FSx for ONTAP file system. |
| `CreateVolume` | `-` | `idempotency-token` | `VolumeType`, `Name` | `ClientRequestToken` | `CreateVolumeResponse` | `BadRequest`, `FileSystemNotFound`, `IncompatibleParameterError`, `InternalServerError`, `MissingVolumeConfiguration`, `ServiceLimitExceeded`, `StorageVirtualMachineNotFound`, `UnsupportedOperation` | Creates an FSx for ONTAP or Amazon FSx for OpenZFS storage volume. |
| `CreateVolumeFromBackup` | `-` | `idempotency-token` | `BackupId`, `Name` | `ClientRequestToken` | `CreateVolumeFromBackupResponse` | `BackupNotFound`, `BadRequest`, `FileSystemNotFound`, `IncompatibleParameterError`, `InternalServerError`, `MissingVolumeConfiguration`, `ServiceLimitExceeded`, `StorageVirtualMachineNotFound` | Creates a new Amazon FSx for NetApp ONTAP volume from an existing Amazon FSx volume backup. |
| `DeleteBackup` | `-` | `idempotent`, `idempotency-token` | `BackupId` | `ClientRequestToken` | `DeleteBackupResponse` | `BackupBeingCopied`, `BackupInProgress`, `BackupNotFound`, `BackupRestoring`, `BadRequest`, `IncompatibleParameterError`, `InternalServerError` | Deletes an Amazon FSx backup. After deletion, the backup no longer exists, and its data is gone. The DeleteBackup call returns instantly. The backup won't show up in later DescribeBackups calls. The data in a deleted ... |
| `DeleteDataRepositoryAssociation` | `-` | `idempotent`, `idempotency-token` | `AssociationId` | `ClientRequestToken` | `DeleteDataRepositoryAssociationResponse` | `BadRequest`, `DataRepositoryAssociationNotFound`, `IncompatibleParameterError`, `InternalServerError`, `ServiceLimitExceeded` | Deletes a data repository association on an Amazon FSx for Lustre file system. Deleting the data repository association unlinks the file system from the Amazon S3 bucket. When deleting a data repository association, ... |
| `DeleteFileCache` | `-` | `idempotent`, `idempotency-token` | `FileCacheId` | `ClientRequestToken` | `DeleteFileCacheResponse` | `BadRequest`, `FileCacheNotFound`, `IncompatibleParameterError`, `InternalServerError`, `ServiceLimitExceeded` | Deletes an Amazon File Cache resource. After deletion, the cache no longer exists, and its data is gone. The DeleteFileCache operation returns while the cache has the DELETING status. You can check the cache deletion ... |
| `DeleteFileSystem` | `-` | `idempotent`, `idempotency-token` | `FileSystemId` | `ClientRequestToken` | `DeleteFileSystemResponse` | `BadRequest`, `FileSystemNotFound`, `IncompatibleParameterError`, `InternalServerError`, `ServiceLimitExceeded` | Deletes a file system. After deletion, the file system no longer exists, and its data is gone. Any existing automatic backups and snapshots are also deleted. To delete an Amazon FSx for NetApp ONTAP file system, firs ... |
| `DeleteSnapshot` | `-` | `idempotent`, `idempotency-token` | `SnapshotId` | `ClientRequestToken` | `DeleteSnapshotResponse` | `BadRequest`, `InternalServerError`, `SnapshotNotFound` | Deletes an Amazon FSx for OpenZFS snapshot. After deletion, the snapshot no longer exists, and its data is gone. Deleting a snapshot doesn't affect snapshots stored in a file system backup. The DeleteSnapshot operati ... |
| `DeleteStorageVirtualMachine` | `-` | `idempotency-token` | `StorageVirtualMachineId` | `ClientRequestToken` | `DeleteStorageVirtualMachineResponse` | `BadRequest`, `IncompatibleParameterError`, `InternalServerError`, `StorageVirtualMachineNotFound` | Deletes an existing Amazon FSx for ONTAP storage virtual machine (SVM). Prior to deleting an SVM, you must delete all non-root volumes in the SVM, otherwise the operation will fail. |
| `DeleteVolume` | `-` | `idempotency-token` | `VolumeId` | `ClientRequestToken` | `DeleteVolumeResponse` | `BadRequest`, `IncompatibleParameterError`, `InternalServerError`, `ServiceLimitExceeded`, `VolumeNotFound` | Deletes an Amazon FSx for NetApp ONTAP or Amazon FSx for OpenZFS volume. |
| `DescribeBackups` | `-` | `paginated` | - | - | `DescribeBackupsResponse` | `BackupNotFound`, `BadRequest`, `FileSystemNotFound`, `InternalServerError`, `VolumeNotFound` | Returns the description of a specific Amazon FSx backup, if a BackupIds value is provided for that backup. Otherwise, it returns all backups owned by your Amazon Web Services account in the Amazon Web Services Region ... |
| `DescribeDataRepositoryAssociations` | `-` | `idempotent`, `paginated` | - | - | `DescribeDataRepositoryAssociationsResponse` | `BadRequest`, `DataRepositoryAssociationNotFound`, `FileSystemNotFound`, `InternalServerError`, `InvalidDataRepositoryType` | Returns the description of specific Amazon FSx for Lustre or Amazon File Cache data repository associations, if one or more AssociationIds values are provided in the request, or if filters are used in the request. Da ... |
| `DescribeDataRepositoryTasks` | `-` | `paginated` | - | - | `DescribeDataRepositoryTasksResponse` | `BadRequest`, `DataRepositoryTaskNotFound`, `FileSystemNotFound`, `InternalServerError` | Returns the description of specific Amazon FSx for Lustre or Amazon File Cache data repository tasks, if one or more TaskIds values are provided in the request, or if filters are used in the request. You can use filt ... |
| `DescribeFileCaches` | `-` | `idempotent`, `paginated` | - | - | `DescribeFileCachesResponse` | `BadRequest`, `FileCacheNotFound`, `InternalServerError` | Returns the description of a specific Amazon File Cache resource, if a FileCacheIds value is provided for that cache. Otherwise, it returns descriptions of all caches owned by your Amazon Web Services account in the ... |
| `DescribeFileSystemAliases` | `-` | `paginated`, `idempotency-token` | `FileSystemId` | `ClientRequestToken` | `DescribeFileSystemAliasesResponse` | `BadRequest`, `FileSystemNotFound`, `InternalServerError` | Returns the DNS aliases that are associated with the specified Amazon FSx for Windows File Server file system. A history of all DNS aliases that have been associated with and disassociated from the file system is ava ... |
| `DescribeFileSystems` | `-` | `paginated` | - | - | `DescribeFileSystemsResponse` | `BadRequest`, `FileSystemNotFound`, `InternalServerError` | Returns the description of specific Amazon FSx file systems, if a FileSystemIds value is provided for that file system. Otherwise, it returns descriptions of all file systems owned by your Amazon Web Services account ... |
| `DescribeS3AccessPointAttachments` | `-` | `paginated` | - | - | `DescribeS3AccessPointAttachmentsResponse` | `BadRequest`, `InternalServerError`, `S3AccessPointAttachmentNotFound`, `UnsupportedOperation` | Describes one or more S3 access points attached to Amazon FSx volumes. The requester requires the following permission to perform this action: fsx:DescribeS3AccessPointAttachments |
| `DescribeSharedVpcConfiguration` | `-` | - | - | - | `DescribeSharedVpcConfigurationResponse` | `BadRequest`, `InternalServerError` | Indicates whether participant accounts in your organization can create Amazon FSx for NetApp ONTAP Multi-AZ file systems in subnets that are shared by a virtual private cloud (VPC) owner. For more information, see Cr ... |
| `DescribeSnapshots` | `-` | `paginated` | - | - | `DescribeSnapshotsResponse` | `BadRequest`, `InternalServerError`, `SnapshotNotFound` | Returns the description of specific Amazon FSx for OpenZFS snapshots, if a SnapshotIds value is provided. Otherwise, this operation returns all snapshots owned by your Amazon Web Services account in the Amazon Web Se ... |
| `DescribeStorageVirtualMachines` | `-` | `paginated` | - | - | `DescribeStorageVirtualMachinesResponse` | `BadRequest`, `InternalServerError`, `StorageVirtualMachineNotFound` | Describes one or more Amazon FSx for NetApp ONTAP storage virtual machines (SVMs). |
| `DescribeVolumes` | `-` | `paginated` | - | - | `DescribeVolumesResponse` | `BadRequest`, `InternalServerError`, `VolumeNotFound` | Describes one or more Amazon FSx for NetApp ONTAP or Amazon FSx for OpenZFS volumes. |
| `DetachAndDeleteS3AccessPoint` | `-` | `idempotency-token` | `Name` | `ClientRequestToken` | `DetachAndDeleteS3AccessPointResponse` | `BadRequest`, `IncompatibleParameterError`, `InternalServerError`, `S3AccessPointAttachmentNotFound`, `UnsupportedOperation` | Detaches an S3 access point from an Amazon FSx volume and deletes the S3 access point. The requester requires the following permission to perform this action: fsx:DetachAndDeleteS3AccessPoint s3:DeleteAccessPoint |
| `DisassociateFileSystemAliases` | `-` | `idempotency-token` | `FileSystemId`, `Aliases` | `ClientRequestToken` | `DisassociateFileSystemAliasesResponse` | `BadRequest`, `FileSystemNotFound`, `InternalServerError` | Use this action to disassociate, or remove, one or more Domain Name Service (DNS) aliases from an Amazon FSx for Windows File Server file system. If you attempt to disassociate a DNS alias that is not associated with ... |
| `ListTagsForResource` | `-` | `paginated` | `ResourceARN` | - | `ListTagsForResourceResponse` | `BadRequest`, `InternalServerError`, `NotServiceResourceError`, `ResourceDoesNotSupportTagging`, `ResourceNotFound` | Lists tags for Amazon FSx resources. When retrieving all tags, you can optionally specify the MaxResults parameter to limit the number of tags in a response. If more tags remain, Amazon FSx returns a NextToken value ... |
| `ReleaseFileSystemNfsV3Locks` | `-` | `idempotent`, `idempotency-token` | `FileSystemId` | `ClientRequestToken` | `ReleaseFileSystemNfsV3LocksResponse` | `BadRequest`, `FileSystemNotFound`, `IncompatibleParameterError`, `InternalServerError`, `ServiceLimitExceeded` | Releases the file system lock from an Amazon FSx for OpenZFS file system. |
| `RestoreVolumeFromSnapshot` | `-` | `idempotent`, `idempotency-token` | `VolumeId`, `SnapshotId` | `ClientRequestToken` | `RestoreVolumeFromSnapshotResponse` | `BadRequest`, `InternalServerError`, `VolumeNotFound` | Returns an Amazon FSx for OpenZFS volume to the state saved by the specified snapshot. |
| `StartMisconfiguredStateRecovery` | `-` | `idempotency-token` | `FileSystemId` | `ClientRequestToken` | `StartMisconfiguredStateRecoveryResponse` | `BadRequest`, `FileSystemNotFound`, `InternalServerError` | After performing steps to repair the Active Directory configuration of an FSx for Windows File Server file system, use this action to initiate the process of Amazon FSx attempting to reconnect to the file system. |
| `TagResource` | `-` | `idempotent` | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `BadRequest`, `InternalServerError`, `NotServiceResourceError`, `ResourceDoesNotSupportTagging`, `ResourceNotFound` | Tags an Amazon FSx resource. |
| `UntagResource` | `-` | `idempotent` | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `BadRequest`, `InternalServerError`, `NotServiceResourceError`, `ResourceDoesNotSupportTagging`, `ResourceNotFound` | This action removes a tag from an Amazon FSx resource. |
| `UpdateDataRepositoryAssociation` | `-` | `idempotent`, `idempotency-token` | `AssociationId` | `ClientRequestToken` | `UpdateDataRepositoryAssociationResponse` | `BadRequest`, `DataRepositoryAssociationNotFound`, `IncompatibleParameterError`, `InternalServerError`, `ServiceLimitExceeded` | Updates the configuration of an existing data repository association on an Amazon FSx for Lustre file system. Data repository associations are supported on all FSx for Lustre 2.12 and 2.15 file systems, excluding scr ... |
| `UpdateFileCache` | `-` | `idempotent`, `idempotency-token` | `FileCacheId` | `ClientRequestToken` | `UpdateFileCacheResponse` | `BadRequest`, `FileCacheNotFound`, `IncompatibleParameterError`, `InternalServerError`, `MissingFileCacheConfiguration`, `ServiceLimitExceeded`, `UnsupportedOperation` | Updates the configuration of an existing Amazon File Cache resource. You can update multiple properties in a single request. |
| `UpdateFileSystem` | `-` | `idempotency-token` | `FileSystemId` | `ClientRequestToken` | `UpdateFileSystemResponse` | `BadRequest`, `FileSystemNotFound`, `IncompatibleParameterError`, `InternalServerError`, `InvalidNetworkSettings`, `MissingFileSystemConfiguration`, `ServiceLimitExceeded`, `UnsupportedOperation` | Use this operation to update the configuration of an existing Amazon FSx file system. You can update multiple properties in a single request. For FSx for Windows File Server file systems, you can update the following ... |
| `UpdateSharedVpcConfiguration` | `-` | `idempotency-token` | - | `ClientRequestToken` | `UpdateSharedVpcConfigurationResponse` | `BadRequest`, `IncompatibleParameterError`, `InternalServerError` | Configures whether participant accounts in your organization can create Amazon FSx for NetApp ONTAP Multi-AZ file systems in subnets that are shared by a virtual private cloud (VPC) owner. For more information, see t ... |
| `UpdateSnapshot` | `-` | `idempotent`, `idempotency-token` | `Name`, `SnapshotId` | `ClientRequestToken` | `UpdateSnapshotResponse` | `BadRequest`, `InternalServerError`, `SnapshotNotFound` | Updates the name of an Amazon FSx for OpenZFS snapshot. |
| `UpdateStorageVirtualMachine` | `-` | `idempotency-token` | `StorageVirtualMachineId` | `ClientRequestToken` | `UpdateStorageVirtualMachineResponse` | `BadRequest`, `IncompatibleParameterError`, `InternalServerError`, `StorageVirtualMachineNotFound`, `UnsupportedOperation` | Updates an FSx for ONTAP storage virtual machine (SVM). |
| `UpdateVolume` | `-` | `idempotency-token` | `VolumeId` | `ClientRequestToken` | `UpdateVolumeResponse` | `BadRequest`, `IncompatibleParameterError`, `InternalServerError`, `MissingVolumeConfiguration`, `VolumeNotFound` | Updates the configuration of an Amazon FSx for NetApp ONTAP or Amazon FSx for OpenZFS volume. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessPointAlreadyOwnedByYou` | `structure` | ErrorCode, Message | An access point with that name already exists in the Amazon Web Services Region in your Amazon Web Services account. |
| `ActiveDirectoryError` | `structure` | ActiveDirectoryId, Type, Message | An Active Directory error. |
| `BackupBeingCopied` | `structure` | Message, BackupId | You can't delete a backup while it's being copied. |
| `BackupInProgress` | `structure` | Message | Another backup is already under way. Wait for completion before initiating additional backups of this file system. |
| `BackupNotFound` | `structure` | Message | No Amazon FSx backups were found based upon the supplied parameters. |
| `BackupRestoring` | `structure` | Message, FileSystemId | You can't delete a backup while it's being used to restore a file system. |
| `BadRequest` | `structure` | Message | A generic error indicating a failure with a client request. |
| `DataRepositoryAssociationNotFound` | `structure` | Message | No data repository associations were found based upon the supplied parameters. |
| `DataRepositoryTaskEnded` | `structure` | Message | The data repository task could not be canceled because the task has already ended. |
| `DataRepositoryTaskExecuting` | `structure` | Message | An existing data repository task is currently executing on the file system. Wait until the existing task has completed, then create the new task. |
| `DataRepositoryTaskNotFound` | `structure` | Message | The data repository task or tasks you specified could not be found. |
| `FileCacheNotFound` | `structure` | Message | No caches were found based upon supplied parameters. |
| `FileSystemNotFound` | `structure` | Message | No Amazon FSx file systems were found based upon supplied parameters. |
| `IncompatibleParameterError` | `structure` | Parameter, Message | The error returned when a second request is received with the same client request token but different parameters settings. A client request token should alw ... |
| `IncompatibleRegionForMultiAZ` | `structure` | Message | Amazon FSx doesn't support Multi-AZ Windows File Server copy backup in the destination Region, so the copied backup can't be restored. |
| `InternalServerError` | `structure` | Message | A generic error indicating a server-side failure. |
| `InvalidAccessPoint` | `structure` | ErrorCode, Message | The access point specified doesn't exist. |
| `InvalidDataRepositoryType` | `structure` | Message | You have filtered the response to a data repository type that is not supported. |
| `InvalidDestinationKmsKey` | `structure` | Message | The Key Management Service (KMS) key of the destination backup is not valid. |
| `InvalidExportPath` | `structure` | Message | The path provided for data repository export isn't valid. |
| `InvalidImportPath` | `structure` | Message | The path provided for data repository import isn't valid. |
| `InvalidNetworkSettings` | `structure` | Message, InvalidSubnetId, InvalidSecurityGroupId, InvalidRouteTableId | One or more network settings specified in the request are invalid. |
| `InvalidPerUnitStorageThroughput` | `structure` | Message | An invalid value for PerUnitStorageThroughput was provided. Please create your file system again, using a valid value. |
| `InvalidRegion` | `structure` | Message | The Region provided for SourceRegion is not valid or is in a different Amazon Web Services partition. |
| `InvalidRequest` | `structure` | ErrorCode, Message | The action or operation requested is invalid. Verify that the action is typed correctly. |
| `InvalidSourceKmsKey` | `structure` | Message | The Key Management Service (KMS) key of the source backup is not valid. |
| `MissingFileCacheConfiguration` | `structure` | Message | A cache configuration is required for this operation. |
| `MissingFileSystemConfiguration` | `structure` | Message | A file system configuration is required for this operation. |
| `MissingVolumeConfiguration` | `structure` | Message | A volume configuration is required for this operation. |
| `NotServiceResourceError` | `structure` | ResourceARN, Message | The resource specified for the tagging operation is not a resource type owned by Amazon FSx. Use the API of the relevant service to perform the operation. |
| `ResourceDoesNotSupportTagging` | `structure` | ResourceARN, Message | The resource specified does not support tagging. |
| `ResourceNotFound` | `structure` | ResourceARN, Message | The resource specified by the Amazon Resource Name (ARN) can't be found. |
| `S3AccessPointAttachmentNotFound` | `structure` | Message | The access point specified was not found. |
| `ServiceLimitExceeded` | `structure` | Limit, Message | An error indicating that a particular service limit was exceeded. You can increase some service limits by contacting Amazon Web Services Support. |
| `SnapshotNotFound` | `structure` | Message | No Amazon FSx snapshots were found based on the supplied parameters. |
| `SourceBackupUnavailable` | `structure` | Message, BackupId | The request was rejected because the lifecycle status of the source backup isn't AVAILABLE . |
| `StorageVirtualMachineNotFound` | `structure` | Message | No FSx for ONTAP SVMs were found based upon the supplied parameters. |
| `TooManyAccessPoints` | `structure` | ErrorCode, Message | You have reached the maximum number of S3 access points attachments allowed for your account in this Amazon Web Services Region, or for the file system. For ... |
| `UnsupportedOperation` | `structure` | Message | The requested operation is not supported for this resource or API. |
| `VolumeNotFound` | `structure` | Message | No Amazon FSx volumes were found based upon the supplied parameters. |
| `ActiveDirectoryErrorType` | `enum` | DOMAIN_NOT_FOUND, INCOMPATIBLE_DOMAIN_MODE, WRONG_VPC, INVALID_NETWORK_TYPE, INVALID_DOMAIN_STAGE | The type of error relating to Microsoft Active Directory. NOT_FOUND means that no directory was found by specifying the given directory. INCOMPATIBLE_MODE m ... |
| `AdministrativeActionType` | `enum` | FILE_SYSTEM_UPDATE, STORAGE_OPTIMIZATION, FILE_SYSTEM_ALIAS_ASSOCIATION, FILE_SYSTEM_ALIAS_DISASSOCIATION, VOLUME_UPDATE, SNAPSHOT_UPDATE, RELEASE_NFS_V3_LOCKS, VOLUME_RESTORE, THROUGHPUT_OPTIMIZATION, IOPS_OPTIMIZATION, STORAGE_TYPE_OPTIMIZATION, MISCONFIGURED_STATE_RECOVERY, ... (+3) | Describes the type of administrative action, as follows: FILE_SYSTEM_UPDATE - A file system update administrative action initiated from the Amazon FSx conso ... |
| `AliasLifecycle` | `enum` | AVAILABLE, CREATING, DELETING, CREATE_FAILED, DELETE_FAILED | - |
| `AutoImportPolicyType` | `enum` | NONE, NEW, NEW_CHANGED, NEW_CHANGED_DELETED | - |
| `AutocommitPeriodType` | `enum` | MINUTES, HOURS, DAYS, MONTHS, YEARS, NONE | - |
| `BackupLifecycle` | `enum` | AVAILABLE, CREATING, TRANSFERRING, DELETED, FAILED, PENDING, COPYING | The lifecycle status of the backup. AVAILABLE - The backup is fully available. PENDING - For user-initiated backups on Lustre file systems only; Amazon FSx ... |
| `BackupType` | `enum` | AUTOMATIC, USER_INITIATED, AWS_BACKUP | The type of the backup. |
| `DataCompressionType` | `enum` | NONE, LZ4 | - |
| `DataRepositoryLifecycle` | `enum` | CREATING, AVAILABLE, MISCONFIGURED, UPDATING, DELETING, FAILED | - |
| `DataRepositoryTaskFilterName` | `enum` | FILE_SYSTEM_ID, TASK_LIFECYCLE, DATA_REPO_ASSOCIATION_ID, FILE_CACHE_ID | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
