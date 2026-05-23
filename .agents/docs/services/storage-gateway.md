# AWS Storage Gateway

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Storage Gateway Service Amazon FSx File Gateway is no longer available to new customers. Existing customers of FSx File Gateway can continue to use the service normally. For capabilities similar to FSx File Gateway, visit this blog post. Storage Gateway is the service that connects an on-premises software appliance with cloud-based storage to provide seamless and secure integration between an organization's on-premises IT environment and the Amazon Web Services storage infrastructure. The service enables you to securely upload data to the Amazon Web Services Cloud for cost effective backup and rapid disaster recovery.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Storage Gateway where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for AWS Storage Gateway by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for AWS Storage Gateway by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS Storage Gateway workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Update`, `List`, `Delete`, `Create` operation families, including `DescribeAvailabilityMonitorTest`, `DescribeBandwidthRateLimit`, `DescribeBandwidthRateLimitSchedule`, `DescribeCache`, `UpdateAutomaticTapeCreationPolicy`, `UpdateBandwidthRateLimit`.

## Service Identity and Protocol

- AWS model slug: `storage-gateway`
- AWS SDK for Rust slug: `storagegateway`
- Model version: `2013-06-30`
- Model file: `vendor/api-models-aws/models/storage-gateway/service/2013-06-30/storage-gateway-2013-06-30.json`
- SDK ID: `Storage Gateway`
- Endpoint prefix: `storagegateway`
- ARN namespace: `storagegateway`
- CloudFormation name: `StorageGateway`
- CloudTrail event source: `storagegateway.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (21), `Update` (15), `List` (12), `Delete` (11), `Create` (9), `Add` (4), `Cancel` (3), `Start` (3).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddCache`, `AddTagsToResource`, `AddUploadBuffer`, `AddWorkingStorage`, `AssociateFileSystem`, `AttachVolume`, `CancelArchival`, `CancelCacheReport`, `CancelRetrieval`, `CreateCachediSCSIVolume`, `CreateNFSFileShare`, `CreateSMBFileShare`, `CreateSnapshot`, `CreateSnapshotFromVolumeRecoveryPoint`, `CreateStorediSCSIVolume`, `CreateTapePool`, `CreateTapeWithBarcode`, `CreateTapes`, `DeleteAutomaticTapeCreationPolicy`, `DeleteBandwidthRateLimit`, ... (+33).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAvailabilityMonitorTest`, `DescribeBandwidthRateLimit`, `DescribeBandwidthRateLimitSchedule`, `DescribeCache`, `DescribeCacheReport`, `DescribeCachediSCSIVolumes`, `DescribeChapCredentials`, `DescribeFileSystemAssociations`, `DescribeGatewayInformation`, `DescribeMaintenanceStartTime`, `DescribeNFSFileShares`, `DescribeSMBFileShares`, `DescribeSMBSettings`, `DescribeSnapshotSchedule`, `DescribeStorediSCSIVolumes`, `DescribeTapeArchives`, `DescribeTapeRecoveryPoints`, `DescribeTapes`, `DescribeUploadBuffer`, `DescribeVTLDevices`, ... (+13).
- Pagination is modelled for 12 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelArchival`, `CancelCacheReport`, `CancelRetrieval`, `DeleteCacheReport`, `DescribeCacheReport`, `ListCacheReports`, `StartAvailabilityMonitorTest`, `StartCacheReport`, `StartGateway`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 96 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EventBridge`, `EC2/VPC`, `ECS`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Describe

- Operations: `DescribeAvailabilityMonitorTest`, `DescribeBandwidthRateLimit`, `DescribeBandwidthRateLimitSchedule`, `DescribeCache`, `DescribeCachediSCSIVolumes`, `DescribeCacheReport`, `DescribeChapCredentials`, `DescribeFileSystemAssociations`, `DescribeGatewayInformation`, `DescribeMaintenanceStartTime`, `DescribeNFSFileShares`, `DescribeSMBFileShares`, `DescribeSMBSettings`, `DescribeSnapshotSchedule`, `DescribeStorediSCSIVolumes`, `DescribeTapeArchives`, `DescribeTapeRecoveryPoints`, `DescribeTapes`, `DescribeUploadBuffer`, `DescribeVTLDevices`, `DescribeWorkingStorage`
- Traits: `paginated` (4)
- Common required input members in this group: `GatewayARN`, `VolumeARNs`, `FileShareARNList`

### Update

- Operations: `UpdateAutomaticTapeCreationPolicy`, `UpdateBandwidthRateLimit`, `UpdateBandwidthRateLimitSchedule`, `UpdateChapCredentials`, `UpdateFileSystemAssociation`, `UpdateGatewayInformation`, `UpdateGatewaySoftwareNow`, `UpdateMaintenanceStartTime`, `UpdateNFSFileShare`, `UpdateSMBFileShare`, `UpdateSMBFileShareVisibility`, `UpdateSMBLocalGroups`, `UpdateSMBSecurityStrategy`, `UpdateSnapshotSchedule`, `UpdateVTLDeviceType`
- Common required input members in this group: `GatewayARN`, `FileShareARN`

### List

- Operations: `ListAutomaticTapeCreationPolicies`, `ListCacheReports`, `ListFileShares`, `ListFileSystemAssociations`, `ListGateways`, `ListLocalDisks`, `ListTagsForResource`, `ListTapePools`, `ListTapes`, `ListVolumeInitiators`, `ListVolumeRecoveryPoints`, `ListVolumes`
- Traits: `paginated` (8)
- Common required input members in this group: `GatewayARN`

### Delete

- Operations: `DeleteAutomaticTapeCreationPolicy`, `DeleteBandwidthRateLimit`, `DeleteCacheReport`, `DeleteChapCredentials`, `DeleteFileShare`, `DeleteGateway`, `DeleteSnapshotSchedule`, `DeleteTape`, `DeleteTapeArchive`, `DeleteTapePool`, `DeleteVolume`
- Common required input members in this group: `GatewayARN`, `VolumeARN`, `TapeARN`

### Create

- Operations: `CreateCachediSCSIVolume`, `CreateNFSFileShare`, `CreateSMBFileShare`, `CreateSnapshot`, `CreateSnapshotFromVolumeRecoveryPoint`, `CreateStorediSCSIVolume`, `CreateTapePool`, `CreateTapes`, `CreateTapeWithBarcode`
- Common required input members in this group: `GatewayARN`, `TargetName`, `NetworkInterfaceId`, `ClientToken`, `Role`, `LocationARN`, `VolumeARN`, `SnapshotDescription`, `TapeSizeInBytes`

### Add

- Operations: `AddCache`, `AddTagsToResource`, `AddUploadBuffer`, `AddWorkingStorage`
- Common required input members in this group: `GatewayARN`, `DiskIds`

### Cancel

- Operations: `CancelArchival`, `CancelCacheReport`, `CancelRetrieval`
- Common required input members in this group: `GatewayARN`, `TapeARN`

### Start

- Operations: `StartAvailabilityMonitorTest`, `StartCacheReport`, `StartGateway`
- Common required input members in this group: `GatewayARN`

### Retrieve

- Operations: `RetrieveTapeArchive`, `RetrieveTapeRecoveryPoint`
- Common required input members in this group: `TapeARN`, `GatewayARN`

### Set

- Operations: `SetLocalConsolePassword`, `SetSMBGuestPassword`
- Common required input members in this group: `GatewayARN`

### Activate

- Operations: `ActivateGateway`
- Common required input members in this group: -

### Assign

- Operations: `AssignTapePool`
- Common required input members in this group: -

### Associate

- Operations: `AssociateFileSystem`
- Common required input members in this group: -

### Attach

- Operations: `AttachVolume`
- Common required input members in this group: -

### Detach

- Operations: `DetachVolume`
- Common required input members in this group: -

### Disable

- Operations: `DisableGateway`
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateFileSystem`
- Common required input members in this group: -

### Evict

- Operations: `EvictFilesFailingUpload`
- Common required input members in this group: -

### Join

- Operations: `JoinDomain`
- Common required input members in this group: -

### Notify

- Operations: `NotifyWhenUploaded`
- Common required input members in this group: -

### Refresh

- Operations: `RefreshCache`
- Common required input members in this group: -

### Remove

- Operations: `RemoveTagsFromResource`
- Common required input members in this group: -

### Reset

- Operations: `ResetCache`
- Common required input members in this group: -

### Shutdown

- Operations: `ShutdownGateway`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ActivateGateway` | `-` | - | `ActivationKey`, `GatewayName`, `GatewayTimezone`, `GatewayRegion` | - | `ActivateGatewayOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Activates the gateway you previously deployed on your host. In the activation process, you specify information such as the Amazon Web Services Region that you want to use for storing snapshots or tapes, the time zone ... |
| `AddCache` | `-` | - | `GatewayARN`, `DiskIds` | - | `AddCacheOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Configures one or more gateway local disks as cache for a gateway. This operation is only supported in the cached volume, tape, and file gateway type (see How Storage Gateway works (architecture) . In the request, yo ... |
| `AddTagsToResource` | `-` | - | `ResourceARN`, `Tags` | - | `AddTagsToResourceOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Adds one or more tags to the specified resource. You use tags to add metadata to resources, which you can use to categorize these resources. For example, you can categorize resources by purpose, owner, environment, o ... |
| `AddUploadBuffer` | `-` | - | `GatewayARN`, `DiskIds` | - | `AddUploadBufferOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Configures one or more gateway local disks as upload buffer for a specified gateway. This operation is supported for the stored volume, cached volume, and tape gateway types. In the request, you specify the gateway A ... |
| `AddWorkingStorage` | `-` | - | `GatewayARN`, `DiskIds` | - | `AddWorkingStorageOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Configures one or more gateway local disks as working storage for a gateway. This operation is only supported in the stored volume gateway type. This operation is deprecated in cached volume API version 20120630. Use ... |
| `AssignTapePool` | `-` | - | `TapeARN`, `PoolId` | - | `AssignTapePoolOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Assigns a tape to a tape pool for archiving. The tape assigned to a pool is archived in the S3 storage class that is associated with the pool. When you use your backup application to eject the tape, the tape is archi ... |
| `AssociateFileSystem` | `-` | - | `UserName`, `Password`, `ClientToken`, `GatewayARN`, `LocationARN` | - | `AssociateFileSystemOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Associate an Amazon FSx file system with the FSx File Gateway. After the association process is complete, the file shares on the Amazon FSx file system are available for access through the gateway. This operation onl ... |
| `AttachVolume` | `-` | - | `GatewayARN`, `VolumeARN`, `NetworkInterfaceId` | - | `AttachVolumeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Connects a volume to an iSCSI connection and then attaches the volume to the specified gateway. Detaching and attaching a volume enables you to recover your data from one gateway to a different gateway without creati ... |
| `CancelArchival` | `-` | - | `GatewayARN`, `TapeARN` | - | `CancelArchivalOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Cancels archiving of a virtual tape to the virtual tape shelf (VTS) after the archiving process is initiated. This operation is only supported in the tape gateway type. |
| `CancelCacheReport` | `-` | - | `CacheReportARN` | - | `CancelCacheReportOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Cancels generation of a specified cache report. You can use this operation to manually cancel an IN-PROGRESS report for any reason. This action changes the report status from IN-PROGRESS to CANCELLED. You can only ca ... |
| `CancelRetrieval` | `-` | - | `GatewayARN`, `TapeARN` | - | `CancelRetrievalOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Cancels retrieval of a virtual tape from the virtual tape shelf (VTS) to a gateway after the retrieval process is initiated. The virtual tape is returned to the VTS. This operation is only supported in the tape gatew ... |
| `CreateCachediSCSIVolume` | `-` | - | `GatewayARN`, `VolumeSizeInBytes`, `TargetName`, `NetworkInterfaceId`, `ClientToken` | - | `CreateCachediSCSIVolumeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Creates a cached volume on a specified cached volume gateway. This operation is only supported in the cached volume gateway type. Cache storage must be allocated to the gateway before you can create a cached volume. ... |
| `CreateNFSFileShare` | `-` | - | `ClientToken`, `GatewayARN`, `Role`, `LocationARN` | - | `CreateNFSFileShareOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Creates a Network File System (NFS) file share on an existing S3 File Gateway. In Storage Gateway, a file share is a file system mount point backed by Amazon S3 cloud storage. Storage Gateway exposes file shares usin ... |
| `CreateSMBFileShare` | `-` | - | `ClientToken`, `GatewayARN`, `Role`, `LocationARN` | - | `CreateSMBFileShareOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Creates a Server Message Block (SMB) file share on an existing S3 File Gateway. In Storage Gateway, a file share is a file system mount point backed by Amazon S3 cloud storage. Storage Gateway exposes file shares usi ... |
| `CreateSnapshot` | `-` | - | `VolumeARN`, `SnapshotDescription` | - | `CreateSnapshotOutput` | `InternalServerError`, `InvalidGatewayRequestException`, `ServiceUnavailableError` | Initiates a snapshot of a volume. Storage Gateway provides the ability to back up point-in-time snapshots of your data to Amazon Simple Storage (Amazon S3) for durable off-site recovery, and also import the data to a ... |
| `CreateSnapshotFromVolumeRecoveryPoint` | `-` | - | `VolumeARN`, `SnapshotDescription` | - | `CreateSnapshotFromVolumeRecoveryPointOutput` | `InternalServerError`, `InvalidGatewayRequestException`, `ServiceUnavailableError` | Initiates a snapshot of a gateway from a volume recovery point. This operation is only supported in the cached volume gateway type. A volume recovery point is a point in time at which all data of the volume is consis ... |
| `CreateStorediSCSIVolume` | `-` | - | `GatewayARN`, `DiskId`, `PreserveExistingData`, `TargetName`, `NetworkInterfaceId` | - | `CreateStorediSCSIVolumeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Creates a volume on a specified gateway. This operation is only supported in the stored volume gateway type. The size of the volume to create is inferred from the disk size. You can choose to preserve existing data o ... |
| `CreateTapePool` | `-` | - | `PoolName`, `StorageClass` | - | `CreateTapePoolOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Creates a new custom tape pool. You can use custom tape pool to enable tape retention lock on tapes that are archived in the custom pool. |
| `CreateTapes` | `-` | - | `GatewayARN`, `TapeSizeInBytes`, `ClientToken`, `NumTapesToCreate`, `TapeBarcodePrefix` | - | `CreateTapesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Creates one or more virtual tapes. You write data to the virtual tapes and then archive the tapes. This operation is only supported in the tape gateway type. Cache storage must be allocated to the gateway before you ... |
| `CreateTapeWithBarcode` | `-` | - | `GatewayARN`, `TapeSizeInBytes`, `TapeBarcode` | - | `CreateTapeWithBarcodeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Creates a virtual tape by using your own barcode. You write data to the virtual tape and then archive the tape. A barcode is unique and cannot be reused if it has already been used on a tape. This applies to barcodes ... |
| `DeleteAutomaticTapeCreationPolicy` | `-` | - | `GatewayARN` | - | `DeleteAutomaticTapeCreationPolicyOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes the automatic tape creation policy of a gateway. If you delete this policy, new virtual tapes must be created manually. Use the Amazon Resource Name (ARN) of the gateway in your request to remove the policy. |
| `DeleteBandwidthRateLimit` | `-` | - | `GatewayARN`, `BandwidthType` | - | `DeleteBandwidthRateLimitOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes the bandwidth rate limits of a gateway. You can delete either the upload and download bandwidth rate limit, or you can delete both. If you delete only one of the limits, the other limit remains unchanged. To ... |
| `DeleteCacheReport` | `-` | - | `CacheReportARN` | - | `DeleteCacheReportOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes the specified cache report and any associated tags from the Storage Gateway database. You can only delete completed reports. If the status of the report you attempt to delete still IN-PROGRESS, the delete ope ... |
| `DeleteChapCredentials` | `-` | - | `TargetARN`, `InitiatorName` | - | `DeleteChapCredentialsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes Challenge-Handshake Authentication Protocol (CHAP) credentials for a specified iSCSI target and initiator pair. This operation is supported in volume and tape gateway types. |
| `DeleteFileShare` | `-` | - | `FileShareARN` | - | `DeleteFileShareOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes a file share from an S3 File Gateway. This operation is only supported for S3 File Gateways. |
| `DeleteGateway` | `-` | - | `GatewayARN` | - | `DeleteGatewayOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes a gateway. To specify which gateway to delete, use the Amazon Resource Name (ARN) of the gateway in your request. The operation deletes the gateway; however, it does not delete the gateway virtual machine (VM ... |
| `DeleteSnapshotSchedule` | `-` | - | `VolumeARN` | - | `DeleteSnapshotScheduleOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes a snapshot of a volume. You can take snapshots of your gateway volumes on a scheduled or ad hoc basis. This API action enables you to delete a snapshot schedule for a volume. For more information, see Backing ... |
| `DeleteTape` | `-` | - | `GatewayARN`, `TapeARN` | - | `DeleteTapeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes the specified virtual tape. This operation is only supported in the tape gateway type. |
| `DeleteTapeArchive` | `-` | - | `TapeARN` | - | `DeleteTapeArchiveOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes the specified virtual tape from the virtual tape shelf (VTS). This operation is only supported in the tape gateway type. |
| `DeleteTapePool` | `-` | - | `PoolARN` | - | `DeleteTapePoolOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Delete a custom tape pool. A custom tape pool can only be deleted if there are no tapes in the pool and if there are no automatic tape creation policies that reference the custom tape pool. |
| `DeleteVolume` | `-` | - | `VolumeARN` | - | `DeleteVolumeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes the specified storage volume that you previously created using the CreateCachediSCSIVolume or CreateStorediSCSIVolume API. This operation is only supported in the cached volume and stored volume types. For st ... |
| `DescribeAvailabilityMonitorTest` | `-` | - | `GatewayARN` | - | `DescribeAvailabilityMonitorTestOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns information about the most recent high availability monitoring test that was performed on the host in a cluster. If a test isn't performed, the status and start time in the response would be null. |
| `DescribeBandwidthRateLimit` | `-` | - | `GatewayARN` | - | `DescribeBandwidthRateLimitOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns the bandwidth rate limits of a gateway. By default, these limits are not set, which means no bandwidth rate limiting is in effect. This operation is supported only for the stored volume, cached volume, and ta ... |
| `DescribeBandwidthRateLimitSchedule` | `-` | - | `GatewayARN` | - | `DescribeBandwidthRateLimitScheduleOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns information about the bandwidth rate limit schedule of a gateway. By default, gateways do not have bandwidth rate limit schedules, which means no bandwidth rate limiting is in effect. This operation is suppor ... |
| `DescribeCache` | `-` | - | `GatewayARN` | - | `DescribeCacheOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns information about the cache of a gateway. This operation is only supported in the cached volume, tape, and file gateway types. The response includes disk IDs that are configured as cache, and it includes the ... |
| `DescribeCachediSCSIVolumes` | `-` | - | `VolumeARNs` | - | `DescribeCachediSCSIVolumesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns a description of the gateway volumes specified in the request. This operation is only supported in the cached volume gateway types. The list of gateway volumes in the request must be from one gateway. In the ... |
| `DescribeCacheReport` | `-` | - | `CacheReportARN` | - | `DescribeCacheReportOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns information about the specified cache report, including completion status and generation progress. |
| `DescribeChapCredentials` | `-` | - | `TargetARN` | - | `DescribeChapCredentialsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns an array of Challenge-Handshake Authentication Protocol (CHAP) credentials information for a specified iSCSI target, one for each target-initiator pair. This operation is supported in the volume and tape gate ... |
| `DescribeFileSystemAssociations` | `-` | - | `FileSystemAssociationARNList` | - | `DescribeFileSystemAssociationsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Gets the file system association information. This operation is only supported for FSx File Gateways. |
| `DescribeGatewayInformation` | `-` | - | `GatewayARN` | - | `DescribeGatewayInformationOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns metadata about a gateway such as its name, network interfaces, time zone, status, and software version. To specify which gateway to describe, use the Amazon Resource Name (ARN) of the gateway in your request. |
| `DescribeMaintenanceStartTime` | `-` | - | `GatewayARN` | - | `DescribeMaintenanceStartTimeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns your gateway's maintenance window schedule information, with values for monthly or weekly cadence, specific day and time to begin maintenance, and which types of updates to apply. Time values returned are for ... |
| `DescribeNFSFileShares` | `-` | - | `FileShareARNList` | - | `DescribeNFSFileSharesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Gets a description for one or more Network File System (NFS) file shares from an S3 File Gateway. This operation is only supported for S3 File Gateways. |
| `DescribeSMBFileShares` | `-` | - | `FileShareARNList` | - | `DescribeSMBFileSharesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Gets a description for one or more Server Message Block (SMB) file shares from a S3 File Gateway. This operation is only supported for S3 File Gateways. |
| `DescribeSMBSettings` | `-` | - | `GatewayARN` | - | `DescribeSMBSettingsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Gets a description of a Server Message Block (SMB) file share settings from a file gateway. This operation is only supported for file gateways. |
| `DescribeSnapshotSchedule` | `-` | - | `VolumeARN` | - | `DescribeSnapshotScheduleOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Describes the snapshot schedule for the specified gateway volume. The snapshot schedule information includes intervals at which snapshots are automatically initiated on the volume. This operation is only supported in ... |
| `DescribeStorediSCSIVolumes` | `-` | - | `VolumeARNs` | - | `DescribeStorediSCSIVolumesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns the description of the gateway volumes specified in the request. The list of gateway volumes in the request must be from one gateway. In the response, Storage Gateway returns volume information sorted by volu ... |
| `DescribeTapeArchives` | `-` | `paginated` | - | - | `DescribeTapeArchivesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns a description of specified virtual tapes in the virtual tape shelf (VTS). This operation is only supported in the tape gateway type. If a specific TapeARN is not specified, Storage Gateway returns a descripti ... |
| `DescribeTapeRecoveryPoints` | `-` | `paginated` | `GatewayARN` | - | `DescribeTapeRecoveryPointsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns a list of virtual tape recovery points that are available for the specified tape gateway. A recovery point is a point-in-time view of a virtual tape at which all the data on the virtual tape is consistent. If ... |
| `DescribeTapes` | `-` | `paginated` | `GatewayARN` | - | `DescribeTapesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns a description of virtual tapes that correspond to the specified Amazon Resource Names (ARNs). If TapeARN is not specified, returns a description of the virtual tapes associated with the specified gateway. Thi ... |
| `DescribeUploadBuffer` | `-` | - | `GatewayARN` | - | `DescribeUploadBufferOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns information about the upload buffer of a gateway. This operation is supported for the stored volume, cached volume, and tape gateway types. The response includes disk IDs that are configured as upload buffer ... |
| `DescribeVTLDevices` | `-` | `paginated` | `GatewayARN` | - | `DescribeVTLDevicesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns a description of virtual tape library (VTL) devices for the specified tape gateway. In the response, Storage Gateway returns VTL device information. This operation is only supported in the tape gateway type. |
| `DescribeWorkingStorage` | `-` | - | `GatewayARN` | - | `DescribeWorkingStorageOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns information about the working storage of a gateway. This operation is only supported in the stored volumes gateway type. This operation is deprecated in cached volumes API version (20120630). Use DescribeUplo ... |
| `DetachVolume` | `-` | - | `VolumeARN` | - | `DetachVolumeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Disconnects a volume from an iSCSI connection and then detaches the volume from the specified gateway. Detaching and attaching a volume enables you to recover your data from one gateway to a different gateway without ... |
| `DisableGateway` | `-` | - | `GatewayARN` | - | `DisableGatewayOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Disables a tape gateway when the gateway is no longer functioning. For example, if your gateway VM is damaged, you can disable the gateway so you can recover virtual tapes. Use this operation for a tape gateway that ... |
| `DisassociateFileSystem` | `-` | - | `FileSystemAssociationARN` | - | `DisassociateFileSystemOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Disassociates an Amazon FSx file system from the specified gateway. After the disassociation process finishes, the gateway can no longer access the Amazon FSx file system. This operation is only supported in the FSx ... |
| `EvictFilesFailingUpload` | `-` | - | `FileShareARN` | - | `EvictFilesFailingUploadOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Starts a process that cleans the specified file share's cache of file entries that are failing upload to Amazon S3. This API operation reports success if the request is received with valid arguments, and there are no ... |
| `JoinDomain` | `-` | - | `GatewayARN`, `DomainName`, `UserName`, `Password` | - | `JoinDomainOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Adds a file gateway to an Active Directory domain. This operation is only supported for file gateways that support the SMB file protocol. Joining a domain creates an Active Directory computer account in the default o ... |
| `ListAutomaticTapeCreationPolicies` | `-` | - | - | - | `ListAutomaticTapeCreationPoliciesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Lists the automatic tape creation policies for a gateway. If there are no automatic tape creation policies for the gateway, it returns an empty list. This operation is only supported for tape gateways. |
| `ListCacheReports` | `-` | `paginated` | - | - | `ListCacheReportsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns a list of existing cache reports for all file shares associated with your Amazon Web Services account. This list includes all information provided by the DescribeCacheReport action, such as report name, statu ... |
| `ListFileShares` | `-` | `paginated` | - | - | `ListFileSharesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Gets a list of the file shares for a specific S3 File Gateway, or the list of file shares that belong to the calling Amazon Web Services account. This operation is only supported for S3 File Gateways. |
| `ListFileSystemAssociations` | `-` | `paginated` | - | - | `ListFileSystemAssociationsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Gets a list of FileSystemAssociationSummary objects. Each object contains a summary of a file system association. This operation is only supported for FSx File Gateways. |
| `ListGateways` | `-` | `paginated` | - | - | `ListGatewaysOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Lists gateways owned by an Amazon Web Services account in an Amazon Web Services Region specified in the request. The returned list is ordered by gateway Amazon Resource Name (ARN). By default, the operation returns ... |
| `ListLocalDisks` | `-` | - | `GatewayARN` | - | `ListLocalDisksOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns a list of the gateway's local disks. To specify which gateway to describe, you use the Amazon Resource Name (ARN) of the gateway in the body of the request. The request returns a list of all disks, specifying ... |
| `ListTagsForResource` | `-` | `paginated` | `ResourceARN` | - | `ListTagsForResourceOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Lists the tags that have been added to the specified resource. This operation is supported in storage gateways of all types. |
| `ListTapePools` | `-` | `paginated` | - | - | `ListTapePoolsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Lists custom tape pools. You specify custom tape pools to list by specifying one or more custom tape pool Amazon Resource Names (ARNs). If you don't specify a custom tape pool ARN, the operation lists all custom tape ... |
| `ListTapes` | `-` | `paginated` | - | - | `ListTapesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Lists virtual tapes in your virtual tape library (VTL) and your virtual tape shelf (VTS). You specify the tapes to list by specifying one or more tape Amazon Resource Names (ARNs). If you don't specify a tape ARN, th ... |
| `ListVolumeInitiators` | `-` | - | `VolumeARN` | - | `ListVolumeInitiatorsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Lists iSCSI initiators that are connected to a volume. You can use this operation to determine whether a volume is being used or not. This operation is only supported in the cached volume and stored volume gateway types. |
| `ListVolumeRecoveryPoints` | `-` | - | `GatewayARN` | - | `ListVolumeRecoveryPointsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Lists the recovery points for a specified gateway. This operation is only supported in the cached volume gateway type. Each cache volume has one recovery point. A volume recovery point is a point in time at which all ... |
| `ListVolumes` | `-` | `paginated` | - | - | `ListVolumesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Lists the iSCSI stored volumes of a gateway. Results are sorted by volume ARN. The response includes only the volume ARNs. If you want additional volume information, use the DescribeStorediSCSIVolumes or the Describe ... |
| `NotifyWhenUploaded` | `-` | - | `FileShareARN` | - | `NotifyWhenUploadedOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Sends you notification through Amazon EventBridge when all files written to your file share have been uploaded to Amazon S3. Storage Gateway can send a notification through Amazon EventBridge when all files written t ... |
| `RefreshCache` | `-` | - | `FileShareARN` | - | `RefreshCacheOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Refreshes the cached inventory of objects for the specified file share. This operation finds objects in the Amazon S3 bucket that were added, removed, or replaced since the gateway last listed the bucket's contents a ... |
| `RemoveTagsFromResource` | `-` | - | `ResourceARN`, `TagKeys` | - | `RemoveTagsFromResourceOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Removes one or more tags from the specified resource. This operation is supported in storage gateways of all types. |
| `ResetCache` | `-` | - | `GatewayARN` | - | `ResetCacheOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Resets all cache disks that have encountered an error and makes the disks available for reconfiguration as cache storage. If your cache disk encounters an error, the gateway prevents read and write operations on virt ... |
| `RetrieveTapeArchive` | `-` | - | `TapeARN`, `GatewayARN` | - | `RetrieveTapeArchiveOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Retrieves an archived virtual tape from the virtual tape shelf (VTS) to a tape gateway. Virtual tapes archived in the VTS are not associated with any gateway. However after a tape is retrieved, it is associated with ... |
| `RetrieveTapeRecoveryPoint` | `-` | - | `TapeARN`, `GatewayARN` | - | `RetrieveTapeRecoveryPointOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Retrieves the recovery point for the specified virtual tape. This operation is only supported in the tape gateway type. A recovery point is a point in time view of a virtual tape at which all the data on the tape is ... |
| `SetLocalConsolePassword` | `-` | - | `GatewayARN`, `LocalConsolePassword` | - | `SetLocalConsolePasswordOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Sets the password for your VM local console. When you log in to the local console for the first time, you log in to the VM with the default credentials. We recommend that you set a new password. You don't need to kno ... |
| `SetSMBGuestPassword` | `-` | - | `GatewayARN`, `Password` | - | `SetSMBGuestPasswordOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Sets the password for the guest user smbguest . The smbguest user is the user when the authentication method for the file share is set to GuestAccess . This operation only supported for S3 File Gateways |
| `ShutdownGateway` | `-` | - | `GatewayARN` | - | `ShutdownGatewayOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Shuts down a Tape Gateway or Volume Gateway. To specify which gateway to shut down, use the Amazon Resource Name (ARN) of the gateway in the body of your request. This API action cannot be used to shut down S3 File G ... |
| `StartAvailabilityMonitorTest` | `-` | - | `GatewayARN` | - | `StartAvailabilityMonitorTestOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Start a test that verifies that the specified gateway is configured for High Availability monitoring in your host environment. This request only initiates the test and that a successful response only indicates that t ... |
| `StartCacheReport` | `-` | - | `FileShareARN`, `Role`, `LocationARN`, `BucketRegion`, `ClientToken` | - | `StartCacheReportOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Starts generating a report of the file metadata currently cached by an S3 File Gateway for a specific file share. You can use this report to identify and resolve issues if you have files failing upload from your gate ... |
| `StartGateway` | `-` | - | `GatewayARN` | - | `StartGatewayOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Starts a gateway that you previously shut down (see ShutdownGateway ). After the gateway starts, you can then make other API calls, your applications can read from or write to the gateway's storage volumes and you wi ... |
| `UpdateAutomaticTapeCreationPolicy` | `-` | - | `AutomaticTapeCreationRules`, `GatewayARN` | - | `UpdateAutomaticTapeCreationPolicyOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates the automatic tape creation policy of a gateway. Use this to update the policy with a new set of automatic tape creation rules. This is only supported for tape gateways. By default, there is no automatic tape ... |
| `UpdateBandwidthRateLimit` | `-` | - | `GatewayARN` | - | `UpdateBandwidthRateLimitOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates the bandwidth rate limits of a gateway. You can update both the upload and download bandwidth rate limit or specify only one of the two. If you don't set a bandwidth rate limit, the existing rate limit remain ... |
| `UpdateBandwidthRateLimitSchedule` | `-` | - | `GatewayARN`, `BandwidthRateLimitIntervals` | - | `UpdateBandwidthRateLimitScheduleOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates the bandwidth rate limit schedule for a specified gateway. By default, gateways do not have bandwidth rate limit schedules, which means no bandwidth rate limiting is in effect. Use this to initiate or update ... |
| `UpdateChapCredentials` | `-` | - | `TargetARN`, `SecretToAuthenticateInitiator`, `InitiatorName` | - | `UpdateChapCredentialsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates the Challenge-Handshake Authentication Protocol (CHAP) credentials for a specified iSCSI target. By default, a gateway does not have CHAP enabled; however, for added security, you might use it. This operation ... |
| `UpdateFileSystemAssociation` | `-` | - | `FileSystemAssociationARN` | - | `UpdateFileSystemAssociationOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates a file system association. This operation is only supported in the FSx File Gateways. |
| `UpdateGatewayInformation` | `-` | - | `GatewayARN` | - | `UpdateGatewayInformationOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates a gateway's metadata, which includes the gateway's name, time zone, and metadata cache size. To specify which gateway to update, use the Amazon Resource Name (ARN) of the gateway in your request. For gateways ... |
| `UpdateGatewaySoftwareNow` | `-` | - | `GatewayARN` | - | `UpdateGatewaySoftwareNowOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates the gateway virtual machine (VM) software. The request immediately triggers the software update. When you make this request, you get a 200 OK success response immediately. However, it might take some time for ... |
| `UpdateMaintenanceStartTime` | `-` | - | `GatewayARN` | - | `UpdateMaintenanceStartTimeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates a gateway's maintenance window schedule, with settings for monthly or weekly cadence, specific day and time to begin maintenance, and which types of updates to apply. Time configuration uses the gateway's tim ... |
| `UpdateNFSFileShare` | `-` | - | `FileShareARN` | - | `UpdateNFSFileShareOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates a Network File System (NFS) file share. This operation is only supported in S3 File Gateways. To leave a file share field unchanged, set the corresponding input field to null. Updates the following file share ... |
| `UpdateSMBFileShare` | `-` | - | `FileShareARN` | - | `UpdateSMBFileShareOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates a Server Message Block (SMB) file share. This operation is only supported for S3 File Gateways. To leave a file share field unchanged, set the corresponding input field to null. File gateways require Security ... |
| `UpdateSMBFileShareVisibility` | `-` | - | `GatewayARN`, `FileSharesVisible` | - | `UpdateSMBFileShareVisibilityOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Controls whether the shares on an S3 File Gateway are visible in a net view or browse list. The operation is only supported for S3 File Gateways. |
| `UpdateSMBLocalGroups` | `-` | - | `GatewayARN`, `SMBLocalGroups` | - | `UpdateSMBLocalGroupsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates the list of Active Directory users and groups that have special permissions for SMB file shares on the gateway. |
| `UpdateSMBSecurityStrategy` | `-` | - | `GatewayARN`, `SMBSecurityStrategy` | - | `UpdateSMBSecurityStrategyOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates the SMB security strategy level for an Amazon S3 file gateway. This action is only supported for Amazon S3 file gateways. For information about configuring this setting using the Amazon Web Services console, ... |
| `UpdateSnapshotSchedule` | `-` | - | `VolumeARN`, `StartAt`, `RecurrenceInHours` | - | `UpdateSnapshotScheduleOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates a snapshot schedule configured for a gateway volume. This operation is only supported in the cached volume and stored volume gateway types. The default snapshot schedule for volume is once every 24 hours, sta ... |
| `UpdateVTLDeviceType` | `-` | - | `VTLDeviceARN`, `DeviceType` | - | `UpdateVTLDeviceTypeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates the type of medium changer in a tape gateway. When you activate a tape gateway, you select a medium changer type for the tape gateway. This operation enables you to select a different type of medium changer a ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerError` | `structure` | message, error | An internal server error has occurred during the request. For more information, see the error and message fields. |
| `InvalidGatewayRequestException` | `structure` | message, error | An exception occurred because an invalid gateway request was issued to the service. For more information, see the error and message fields. |
| `ServiceUnavailableError` | `structure` | message, error | An internal server error has occurred because the service is unavailable. For more information, see the error and message fields. |
| `ActivateGatewayInput` | `structure` | ActivationKey, GatewayName, GatewayTimezone, GatewayRegion, GatewayType, TapeDriveType, MediumChangerType, Tags | A JSON object containing one or more of the following fields: ActivateGatewayInput$ActivationKey ActivateGatewayInput$GatewayName ActivateGatewayInput$Gatew ... |
| `ActivateGatewayOutput` | `structure` | GatewayARN | Storage Gateway returns the Amazon Resource Name (ARN) of the activated gateway. It is a string made of information such as your account, gateway name, and ... |
| `AddCacheInput` | `structure` | GatewayARN, DiskIds | - |
| `AddCacheOutput` | `structure` | GatewayARN | - |
| `AddTagsToResourceInput` | `structure` | ResourceARN, Tags | AddTagsToResourceInput |
| `AddTagsToResourceOutput` | `structure` | ResourceARN | AddTagsToResourceOutput |
| `AddUploadBufferInput` | `structure` | GatewayARN, DiskIds | - |
| `AddUploadBufferOutput` | `structure` | GatewayARN | - |
| `AddWorkingStorageInput` | `structure` | GatewayARN, DiskIds | A JSON object containing one or more of the following fields: AddWorkingStorageInput$DiskIds |
| `AddWorkingStorageOutput` | `structure` | GatewayARN | A JSON object containing the Amazon Resource Name (ARN) of the gateway for which working storage was configured. |
| `AssignTapePoolInput` | `structure` | TapeARN, PoolId, BypassGovernanceRetention | - |
| `AssignTapePoolOutput` | `structure` | TapeARN | - |
| `AssociateFileSystemInput` | `structure` | UserName, Password, ClientToken, GatewayARN, LocationARN, Tags, AuditDestinationARN, CacheAttributes, EndpointNetworkConfiguration | - |
| `AssociateFileSystemOutput` | `structure` | FileSystemAssociationARN | - |
| `AttachVolumeInput` | `structure` | GatewayARN, TargetName, VolumeARN, NetworkInterfaceId, DiskId | AttachVolumeInput |
| `AttachVolumeOutput` | `structure` | VolumeARN, TargetARN | AttachVolumeOutput |
| `CancelArchivalInput` | `structure` | GatewayARN, TapeARN | CancelArchivalInput |
| `CancelArchivalOutput` | `structure` | TapeARN | CancelArchivalOutput |
| `CancelCacheReportInput` | `structure` | CacheReportARN | - |
| `CancelCacheReportOutput` | `structure` | CacheReportARN | - |
| `CancelRetrievalInput` | `structure` | GatewayARN, TapeARN | CancelRetrievalInput |
| `CancelRetrievalOutput` | `structure` | TapeARN | CancelRetrievalOutput |
| `CreateCachediSCSIVolumeInput` | `structure` | GatewayARN, VolumeSizeInBytes, SnapshotId, TargetName, SourceVolumeARN, NetworkInterfaceId, ClientToken, KMSEncrypted, KMSKey, Tags | - |
| `CreateCachediSCSIVolumeOutput` | `structure` | VolumeARN, TargetARN | - |
| `CreateNFSFileShareInput` | `structure` | ClientToken, NFSFileShareDefaults, GatewayARN, EncryptionType, KMSEncrypted, KMSKey, Role, LocationARN, DefaultStorageClass, ObjectACL, ClientList, Squash, ... (+10) | CreateNFSFileShareInput |
| `CreateNFSFileShareOutput` | `structure` | FileShareARN | CreateNFSFileShareOutput |
| `CreateSMBFileShareInput` | `structure` | ClientToken, GatewayARN, EncryptionType, KMSEncrypted, KMSKey, Role, LocationARN, DefaultStorageClass, ObjectACL, ReadOnly, GuessMIMETypeEnabled, RequesterPays, ... (+15) | CreateSMBFileShareInput |
| `CreateSMBFileShareOutput` | `structure` | FileShareARN | CreateSMBFileShareOutput |
| `CreateSnapshotInput` | `structure` | VolumeARN, SnapshotDescription, Tags | A JSON object containing one or more of the following fields: CreateSnapshotInput$SnapshotDescription CreateSnapshotInput$VolumeARN |
| `CreateSnapshotOutput` | `structure` | VolumeARN, SnapshotId | A JSON object containing the following fields: |
| `CreateSnapshotFromVolumeRecoveryPointInput` | `structure` | VolumeARN, SnapshotDescription, Tags | - |
| `CreateSnapshotFromVolumeRecoveryPointOutput` | `structure` | SnapshotId, VolumeARN, VolumeRecoveryPointTime | - |
| `CreateStorediSCSIVolumeInput` | `structure` | GatewayARN, DiskId, SnapshotId, PreserveExistingData, TargetName, NetworkInterfaceId, KMSEncrypted, KMSKey, Tags | A JSON object containing one or more of the following fields: CreateStorediSCSIVolumeInput$DiskId CreateStorediSCSIVolumeInput$NetworkInterfaceId CreateStor ... |
| `CreateStorediSCSIVolumeOutput` | `structure` | VolumeARN, VolumeSizeInBytes, TargetARN | A JSON object containing the following fields: |
| `CreateTapePoolInput` | `structure` | PoolName, StorageClass, RetentionLockType, RetentionLockTimeInDays, Tags | - |
| `CreateTapePoolOutput` | `structure` | PoolARN | - |
| `CreateTapesInput` | `structure` | GatewayARN, TapeSizeInBytes, ClientToken, NumTapesToCreate, TapeBarcodePrefix, KMSEncrypted, KMSKey, PoolId, Worm, Tags | CreateTapesInput |
| `ActiveDirectoryStatus` | `enum` | ACCESS_DENIED, DETACHED, JOINED, JOINING, NETWORK_ERROR, TIMEOUT, UNKNOWN_ERROR, INSUFFICIENT_PERMISSIONS | - |
| `AutomaticUpdatePolicy` | `enum` | ALL_VERSIONS, EMERGENCY_VERSIONS_ONLY | - |
| `AvailabilityMonitorTestStatus` | `enum` | COMPLETE, FAILED, PENDING | - |
| `CacheReportFilterName` | `enum` | UploadState, UploadFailureReason | - |
| `CacheReportStatus` | `enum` | IN_PROGRESS, COMPLETED, CANCELED, FAILED, ERROR | - |
| `CaseSensitivity` | `enum` | ClientSpecified, CaseSensitive | - |
| `EncryptionType` | `enum` | SseS3, SseKms, DsseKms | - |
| `ErrorCode` | `enum` | ActivationKeyExpired, ActivationKeyInvalid, ActivationKeyNotFound, GatewayInternalError, GatewayNotConnected, GatewayNotFound, GatewayProxyNetworkConnectionBusy, AuthenticationFailure, BandwidthThrottleScheduleNotFound, Blocked, CannotExportSnapshot, ChapCredentialNotFound, ... (+50) | - |
| `FileShareType` | `enum` | NFS, SMB | The type of the file share. |
| `GatewayCapacity` | `enum` | Small, Medium, Large | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
