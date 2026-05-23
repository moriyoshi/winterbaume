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

- Operations: `DescribeAvailabilityMonitorTest`, `DescribeBandwidthRateLimit`, `DescribeBandwidthRateLimitSchedule`, `DescribeCache`, `DescribeCacheReport`, `DescribeCachediSCSIVolumes`, `DescribeChapCredentials`, `DescribeFileSystemAssociations`, `DescribeGatewayInformation`, `DescribeMaintenanceStartTime`, `DescribeNFSFileShares`, `DescribeSMBFileShares`, `DescribeSMBSettings`, `DescribeSnapshotSchedule`, `DescribeStorediSCSIVolumes`, `DescribeTapeArchives`, `DescribeTapeRecoveryPoints`, `DescribeTapes`, `DescribeUploadBuffer`, `DescribeVTLDevices`, `DescribeWorkingStorage`
- Traits: `paginated` (4)
- Common required input members in this group: `CacheReportARN`, `FileShareARNList`, `FileSystemAssociationARNList`, `GatewayARN`, `TargetARN`, `VolumeARN`, `VolumeARNs`

### Update

- Operations: `UpdateAutomaticTapeCreationPolicy`, `UpdateBandwidthRateLimit`, `UpdateBandwidthRateLimitSchedule`, `UpdateChapCredentials`, `UpdateFileSystemAssociation`, `UpdateGatewayInformation`, `UpdateGatewaySoftwareNow`, `UpdateMaintenanceStartTime`, `UpdateNFSFileShare`, `UpdateSMBFileShare`, `UpdateSMBFileShareVisibility`, `UpdateSMBLocalGroups`, `UpdateSMBSecurityStrategy`, `UpdateSnapshotSchedule`, `UpdateVTLDeviceType`
- Common required input members in this group: `AutomaticTapeCreationRules`, `BandwidthRateLimitIntervals`, `DeviceType`, `FileShareARN`, `FileSharesVisible`, `FileSystemAssociationARN`, `GatewayARN`, `InitiatorName`, `RecurrenceInHours`, `SMBLocalGroups`, `SMBSecurityStrategy`, `SecretToAuthenticateInitiator`, `StartAt`, `TargetARN`, `VTLDeviceARN`, `VolumeARN`

### List

- Operations: `ListAutomaticTapeCreationPolicies`, `ListCacheReports`, `ListFileShares`, `ListFileSystemAssociations`, `ListGateways`, `ListLocalDisks`, `ListTagsForResource`, `ListTapePools`, `ListTapes`, `ListVolumeInitiators`, `ListVolumeRecoveryPoints`, `ListVolumes`
- Traits: `paginated` (8)
- Common required input members in this group: `GatewayARN`, `ResourceARN`, `VolumeARN`

### Delete

- Operations: `DeleteAutomaticTapeCreationPolicy`, `DeleteBandwidthRateLimit`, `DeleteCacheReport`, `DeleteChapCredentials`, `DeleteFileShare`, `DeleteGateway`, `DeleteSnapshotSchedule`, `DeleteTape`, `DeleteTapeArchive`, `DeleteTapePool`, `DeleteVolume`
- Common required input members in this group: `BandwidthType`, `CacheReportARN`, `FileShareARN`, `GatewayARN`, `InitiatorName`, `PoolARN`, `TapeARN`, `TargetARN`, `VolumeARN`

### Create

- Operations: `CreateCachediSCSIVolume`, `CreateNFSFileShare`, `CreateSMBFileShare`, `CreateSnapshot`, `CreateSnapshotFromVolumeRecoveryPoint`, `CreateStorediSCSIVolume`, `CreateTapePool`, `CreateTapeWithBarcode`, `CreateTapes`
- Common required input members in this group: `ClientToken`, `DiskId`, `GatewayARN`, `LocationARN`, `NetworkInterfaceId`, `NumTapesToCreate`, `PoolName`, `PreserveExistingData`, `Role`, `SnapshotDescription`, `StorageClass`, `TapeBarcode`, `TapeBarcodePrefix`, `TapeSizeInBytes`, `TargetName`, `VolumeARN`, `VolumeSizeInBytes`

### Add

- Operations: `AddCache`, `AddTagsToResource`, `AddUploadBuffer`, `AddWorkingStorage`
- Common required input members in this group: `DiskIds`, `GatewayARN`, `ResourceARN`, `Tags`

### Cancel

- Operations: `CancelArchival`, `CancelCacheReport`, `CancelRetrieval`
- Common required input members in this group: `CacheReportARN`, `GatewayARN`, `TapeARN`

### Start

- Operations: `StartAvailabilityMonitorTest`, `StartCacheReport`, `StartGateway`
- Common required input members in this group: `BucketRegion`, `ClientToken`, `FileShareARN`, `GatewayARN`, `LocationARN`, `Role`

### Retrieve

- Operations: `RetrieveTapeArchive`, `RetrieveTapeRecoveryPoint`
- Common required input members in this group: `GatewayARN`, `TapeARN`

### Set

- Operations: `SetLocalConsolePassword`, `SetSMBGuestPassword`
- Common required input members in this group: `GatewayARN`, `LocalConsolePassword`, `Password`

### Activate

- Operations: `ActivateGateway`
- Common required input members in this group: `ActivationKey`, `GatewayName`, `GatewayRegion`, `GatewayTimezone`

### Assign

- Operations: `AssignTapePool`
- Common required input members in this group: `PoolId`, `TapeARN`

### Associate

- Operations: `AssociateFileSystem`
- Common required input members in this group: `ClientToken`, `GatewayARN`, `LocationARN`, `Password`, `UserName`

### Attach

- Operations: `AttachVolume`
- Common required input members in this group: `GatewayARN`, `NetworkInterfaceId`, `VolumeARN`

### Detach

- Operations: `DetachVolume`
- Common required input members in this group: `VolumeARN`

### Disable

- Operations: `DisableGateway`
- Common required input members in this group: `GatewayARN`

### Disassociate

- Operations: `DisassociateFileSystem`
- Common required input members in this group: `FileSystemAssociationARN`

### Evict

- Operations: `EvictFilesFailingUpload`
- Common required input members in this group: `FileShareARN`

### Join

- Operations: `JoinDomain`
- Common required input members in this group: `DomainName`, `GatewayARN`, `Password`, `UserName`

### Notify

- Operations: `NotifyWhenUploaded`
- Common required input members in this group: `FileShareARN`

### Refresh

- Operations: `RefreshCache`
- Common required input members in this group: `FileShareARN`

### Remove

- Operations: `RemoveTagsFromResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

### Reset

- Operations: `ResetCache`
- Common required input members in this group: `GatewayARN`

### Shutdown

- Operations: `ShutdownGateway`
- Common required input members in this group: `GatewayARN`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ActivateGateway` | - | - | `ActivationKey`, `GatewayName`, `GatewayRegion`, `GatewayTimezone` | - | `ActivateGatewayOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Activates the gateway you previously deployed on your host. In the activation process, you specify information such as the Amazon Web Services Region that you want to use for storing snapshots or tapes, the time zone for scheduled snapshots the gateway... |
| `AddCache` | - | - | `DiskIds`, `GatewayARN` | - | `AddCacheOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Configures one or more gateway local disks as cache for a gateway. This operation is only supported in the cached volume, tape, and file gateway type (see How Storage Gateway works (architecture). |
| `AddTagsToResource` | - | - | `ResourceARN`, `Tags` | - | `AddTagsToResourceOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Adds one or more tags to the specified resource. You use tags to add metadata to resources, which you can use to categorize these resources. |
| `AddUploadBuffer` | - | - | `DiskIds`, `GatewayARN` | - | `AddUploadBufferOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Configures one or more gateway local disks as upload buffer for a specified gateway. This operation is supported for the stored volume, cached volume, and tape gateway types. |
| `AddWorkingStorage` | - | - | `DiskIds`, `GatewayARN` | - | `AddWorkingStorageOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Configures one or more gateway local disks as working storage for a gateway. This operation is only supported in the stored volume gateway type. |
| `AssignTapePool` | - | - | `PoolId`, `TapeARN` | - | `AssignTapePoolOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Assigns a tape to a tape pool for archiving. The tape assigned to a pool is archived in the S3 storage class that is associated with the pool. |
| `AssociateFileSystem` | - | - | `ClientToken`, `GatewayARN`, `LocationARN`, `Password`, `UserName` | - | `AssociateFileSystemOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Associate an Amazon FSx file system with the FSx File Gateway. After the association process is complete, the file shares on the Amazon FSx file system are available for access through the gateway. |
| `AttachVolume` | - | - | `GatewayARN`, `NetworkInterfaceId`, `VolumeARN` | - | `AttachVolumeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Connects a volume to an iSCSI connection and then attaches the volume to the specified gateway. Detaching and attaching a volume enables you to recover your data from one gateway to a different gateway without creating a snapshot. |
| `CancelArchival` | - | - | `GatewayARN`, `TapeARN` | - | `CancelArchivalOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Cancels archiving of a virtual tape to the virtual tape shelf (VTS) after the archiving process is initiated. This operation is only supported in the tape gateway type. |
| `CancelCacheReport` | - | - | `CacheReportARN` | - | `CancelCacheReportOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Cancels generation of a specified cache report. You can use this operation to manually cancel an IN-PROGRESS report for any reason. |
| `CancelRetrieval` | - | - | `GatewayARN`, `TapeARN` | - | `CancelRetrievalOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Cancels retrieval of a virtual tape from the virtual tape shelf (VTS) to a gateway after the retrieval process is initiated. The virtual tape is returned to the VTS. |
| `CreateCachediSCSIVolume` | - | - | `ClientToken`, `GatewayARN`, `NetworkInterfaceId`, `TargetName`, `VolumeSizeInBytes` | - | `CreateCachediSCSIVolumeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Creates a cached volume on a specified cached volume gateway. This operation is only supported in the cached volume gateway type. |
| `CreateNFSFileShare` | - | - | `ClientToken`, `GatewayARN`, `LocationARN`, `Role` | - | `CreateNFSFileShareOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Creates a Network File System (NFS) file share on an existing S3 File Gateway. In Storage Gateway, a file share is a file system mount point backed by Amazon S3 cloud storage. |
| `CreateSMBFileShare` | - | - | `ClientToken`, `GatewayARN`, `LocationARN`, `Role` | - | `CreateSMBFileShareOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Creates a Server Message Block (SMB) file share on an existing S3 File Gateway. In Storage Gateway, a file share is a file system mount point backed by Amazon S3 cloud storage. |
| `CreateSnapshot` | - | - | `SnapshotDescription`, `VolumeARN` | - | `CreateSnapshotOutput` | `InternalServerError`, `InvalidGatewayRequestException`, `ServiceUnavailableError` | Initiates a snapshot of a volume. Storage Gateway provides the ability to back up point-in-time snapshots of your data to Amazon Simple Storage (Amazon S3) for durable off-site recovery, and also import the data to an Amazon Elastic Block Store (EBS) volume... |
| `CreateSnapshotFromVolumeRecoveryPoint` | - | - | `SnapshotDescription`, `VolumeARN` | - | `CreateSnapshotFromVolumeRecoveryPointOutput` | `InternalServerError`, `InvalidGatewayRequestException`, `ServiceUnavailableError` | Initiates a snapshot of a gateway from a volume recovery point. This operation is only supported in the cached volume gateway type. |
| `CreateStorediSCSIVolume` | - | - | `DiskId`, `GatewayARN`, `NetworkInterfaceId`, `PreserveExistingData`, `TargetName` | - | `CreateStorediSCSIVolumeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Creates a volume on a specified gateway. This operation is only supported in the stored volume gateway type. |
| `CreateTapePool` | - | - | `PoolName`, `StorageClass` | - | `CreateTapePoolOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Creates a new custom tape pool. You can use custom tape pool to enable tape retention lock on tapes that are archived in the custom pool. |
| `CreateTapeWithBarcode` | - | - | `GatewayARN`, `TapeBarcode`, `TapeSizeInBytes` | - | `CreateTapeWithBarcodeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Creates a virtual tape by using your own barcode. You write data to the virtual tape and then archive the tape. |
| `CreateTapes` | - | - | `ClientToken`, `GatewayARN`, `NumTapesToCreate`, `TapeBarcodePrefix`, `TapeSizeInBytes` | - | `CreateTapesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Creates one or more virtual tapes. You write data to the virtual tapes and then archive the tapes. |
| `DeleteAutomaticTapeCreationPolicy` | - | - | `GatewayARN` | - | `DeleteAutomaticTapeCreationPolicyOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes the automatic tape creation policy of a gateway. If you delete this policy, new virtual tapes must be created manually. |
| `DeleteBandwidthRateLimit` | - | - | `BandwidthType`, `GatewayARN` | - | `DeleteBandwidthRateLimitOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes the bandwidth rate limits of a gateway. You can delete either the upload and download bandwidth rate limit, or you can delete both. |
| `DeleteCacheReport` | - | - | `CacheReportARN` | - | `DeleteCacheReportOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes the specified cache report and any associated tags from the Storage Gateway database. You can only delete completed reports. |
| `DeleteChapCredentials` | - | - | `InitiatorName`, `TargetARN` | - | `DeleteChapCredentialsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes Challenge-Handshake Authentication Protocol (CHAP) credentials for a specified iSCSI target and initiator pair. This operation is supported in volume and tape gateway types. |
| `DeleteFileShare` | - | - | `FileShareARN` | - | `DeleteFileShareOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes a file share from an S3 File Gateway. This operation is only supported for S3 File Gateways. |
| `DeleteGateway` | - | - | `GatewayARN` | - | `DeleteGatewayOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes a gateway. To specify which gateway to delete, use the Amazon Resource Name (ARN) of the gateway in your request. |
| `DeleteSnapshotSchedule` | - | - | `VolumeARN` | - | `DeleteSnapshotScheduleOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes a snapshot of a volume. You can take snapshots of your gateway volumes on a scheduled or ad hoc basis. |
| `DeleteTape` | - | - | `GatewayARN`, `TapeARN` | - | `DeleteTapeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes the specified virtual tape. This operation is only supported in the tape gateway type. |
| `DeleteTapeArchive` | - | - | `TapeARN` | - | `DeleteTapeArchiveOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes the specified virtual tape from the virtual tape shelf (VTS). This operation is only supported in the tape gateway type. |
| `DeleteTapePool` | - | - | `PoolARN` | - | `DeleteTapePoolOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Delete a custom tape pool. A custom tape pool can only be deleted if there are no tapes in the pool and if there are no automatic tape creation policies that reference the custom tape pool. |
| `DeleteVolume` | - | - | `VolumeARN` | - | `DeleteVolumeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Deletes the specified storage volume that you previously created using the CreateCachediSCSIVolume or CreateStorediSCSIVolume API. This operation is only supported in the cached volume and stored volume types. |
| `DescribeAvailabilityMonitorTest` | - | - | `GatewayARN` | - | `DescribeAvailabilityMonitorTestOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns information about the most recent high availability monitoring test that was performed on the host in a cluster. If a test isn't performed, the status and start time in the response would be null. |
| `DescribeBandwidthRateLimit` | - | - | `GatewayARN` | - | `DescribeBandwidthRateLimitOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns the bandwidth rate limits of a gateway. By default, these limits are not set, which means no bandwidth rate limiting is in effect. |
| `DescribeBandwidthRateLimitSchedule` | - | - | `GatewayARN` | - | `DescribeBandwidthRateLimitScheduleOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns information about the bandwidth rate limit schedule of a gateway. By default, gateways do not have bandwidth rate limit schedules, which means no bandwidth rate limiting is in effect. |
| `DescribeCache` | - | - | `GatewayARN` | - | `DescribeCacheOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns information about the cache of a gateway. This operation is only supported in the cached volume, tape, and file gateway types. |
| `DescribeCacheReport` | - | - | `CacheReportARN` | - | `DescribeCacheReportOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns information about the specified cache report, including completion status and generation progress. |
| `DescribeCachediSCSIVolumes` | - | - | `VolumeARNs` | - | `DescribeCachediSCSIVolumesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns a description of the gateway volumes specified in the request. This operation is only supported in the cached volume gateway types. |
| `DescribeChapCredentials` | - | - | `TargetARN` | - | `DescribeChapCredentialsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns an array of Challenge-Handshake Authentication Protocol (CHAP) credentials information for a specified iSCSI target, one for each target-initiator pair. This operation is supported in the volume and tape gateway types. |
| `DescribeFileSystemAssociations` | - | - | `FileSystemAssociationARNList` | - | `DescribeFileSystemAssociationsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Gets the file system association information. This operation is only supported for FSx File Gateways. |
| `DescribeGatewayInformation` | - | - | `GatewayARN` | - | `DescribeGatewayInformationOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns metadata about a gateway such as its name, network interfaces, time zone, status, and software version. To specify which gateway to describe, use the Amazon Resource Name (ARN) of the gateway in your request. |
| `DescribeMaintenanceStartTime` | - | - | `GatewayARN` | - | `DescribeMaintenanceStartTimeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns your gateway's maintenance window schedule information, with values for monthly or weekly cadence, specific day and time to begin maintenance, and which types of updates to apply. Time values returned are for the gateway's time zone. |
| `DescribeNFSFileShares` | - | - | `FileShareARNList` | - | `DescribeNFSFileSharesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Gets a description for one or more Network File System (NFS) file shares from an S3 File Gateway. This operation is only supported for S3 File Gateways. |
| `DescribeSMBFileShares` | - | - | `FileShareARNList` | - | `DescribeSMBFileSharesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Gets a description for one or more Server Message Block (SMB) file shares from a S3 File Gateway. This operation is only supported for S3 File Gateways. |
| `DescribeSMBSettings` | - | - | `GatewayARN` | - | `DescribeSMBSettingsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Gets a description of a Server Message Block (SMB) file share settings from a file gateway. This operation is only supported for file gateways. |
| `DescribeSnapshotSchedule` | - | - | `VolumeARN` | - | `DescribeSnapshotScheduleOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Describes the snapshot schedule for the specified gateway volume. The snapshot schedule information includes intervals at which snapshots are automatically initiated on the volume. |
| `DescribeStorediSCSIVolumes` | - | - | `VolumeARNs` | - | `DescribeStorediSCSIVolumesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns the description of the gateway volumes specified in the request. The list of gateway volumes in the request must be from one gateway. |
| `DescribeTapeArchives` | - | `paginated` | - | - | `DescribeTapeArchivesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns a description of specified virtual tapes in the virtual tape shelf (VTS). This operation is only supported in the tape gateway type. |
| `DescribeTapeRecoveryPoints` | - | `paginated` | `GatewayARN` | - | `DescribeTapeRecoveryPointsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns a list of virtual tape recovery points that are available for the specified tape gateway. A recovery point is a point-in-time view of a virtual tape at which all the data on the virtual tape is consistent. |
| `DescribeTapes` | - | `paginated` | `GatewayARN` | - | `DescribeTapesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns a description of virtual tapes that correspond to the specified Amazon Resource Names (ARNs). If `TapeARN` is not specified, returns a description of the virtual tapes associated with the specified gateway. |
| `DescribeUploadBuffer` | - | - | `GatewayARN` | - | `DescribeUploadBufferOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns information about the upload buffer of a gateway. This operation is supported for the stored volume, cached volume, and tape gateway types. |
| `DescribeVTLDevices` | - | `paginated` | `GatewayARN` | - | `DescribeVTLDevicesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns a description of virtual tape library (VTL) devices for the specified tape gateway. In the response, Storage Gateway returns VTL device information. |
| `DescribeWorkingStorage` | - | - | `GatewayARN` | - | `DescribeWorkingStorageOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns information about the working storage of a gateway. This operation is only supported in the stored volumes gateway type. |
| `DetachVolume` | - | - | `VolumeARN` | - | `DetachVolumeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Disconnects a volume from an iSCSI connection and then detaches the volume from the specified gateway. Detaching and attaching a volume enables you to recover your data from one gateway to a different gateway without creating a snapshot. |
| `DisableGateway` | - | - | `GatewayARN` | - | `DisableGatewayOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Disables a tape gateway when the gateway is no longer functioning. For example, if your gateway VM is damaged, you can disable the gateway so you can recover virtual tapes. |
| `DisassociateFileSystem` | - | - | `FileSystemAssociationARN` | - | `DisassociateFileSystemOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Disassociates an Amazon FSx file system from the specified gateway. After the disassociation process finishes, the gateway can no longer access the Amazon FSx file system. |
| `EvictFilesFailingUpload` | - | - | `FileShareARN` | - | `EvictFilesFailingUploadOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Starts a process that cleans the specified file share's cache of file entries that are failing upload to Amazon S3. This API operation reports success if the request is received with valid arguments, and there are no other cache clean operations currently... |
| `JoinDomain` | - | - | `DomainName`, `GatewayARN`, `Password`, `UserName` | - | `JoinDomainOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Adds a file gateway to an Active Directory domain. This operation is only supported for file gateways that support the SMB file protocol. |
| `ListAutomaticTapeCreationPolicies` | - | - | - | - | `ListAutomaticTapeCreationPoliciesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Lists the automatic tape creation policies for a gateway. If there are no automatic tape creation policies for the gateway, it returns an empty list. |
| `ListCacheReports` | - | `paginated` | - | - | `ListCacheReportsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns a list of existing cache reports for all file shares associated with your Amazon Web Services account. This list includes all information provided by the `DescribeCacheReport` action, such as report name, status, completion progress, start time, end... |
| `ListFileShares` | - | `paginated` | - | - | `ListFileSharesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Gets a list of the file shares for a specific S3 File Gateway, or the list of file shares that belong to the calling Amazon Web Services account. This operation is only supported for S3 File Gateways. |
| `ListFileSystemAssociations` | - | `paginated` | - | - | `ListFileSystemAssociationsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Gets a list of `FileSystemAssociationSummary` objects. Each object contains a summary of a file system association. |
| `ListGateways` | - | `paginated` | - | - | `ListGatewaysOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Lists gateways owned by an Amazon Web Services account in an Amazon Web Services Region specified in the request. The returned list is ordered by gateway Amazon Resource Name (ARN). |
| `ListLocalDisks` | - | - | `GatewayARN` | - | `ListLocalDisksOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Returns a list of the gateway's local disks. To specify which gateway to describe, you use the Amazon Resource Name (ARN) of the gateway in the body of the request. |
| `ListTagsForResource` | - | `paginated` | `ResourceARN` | - | `ListTagsForResourceOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Lists the tags that have been added to the specified resource. This operation is supported in storage gateways of all types. |
| `ListTapePools` | - | `paginated` | - | - | `ListTapePoolsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Lists custom tape pools. You specify custom tape pools to list by specifying one or more custom tape pool Amazon Resource Names (ARNs). |
| `ListTapes` | - | `paginated` | - | - | `ListTapesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Lists virtual tapes in your virtual tape library (VTL) and your virtual tape shelf (VTS). You specify the tapes to list by specifying one or more tape Amazon Resource Names (ARNs). |
| `ListVolumeInitiators` | - | - | `VolumeARN` | - | `ListVolumeInitiatorsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Lists iSCSI initiators that are connected to a volume. You can use this operation to determine whether a volume is being used or not. |
| `ListVolumeRecoveryPoints` | - | - | `GatewayARN` | - | `ListVolumeRecoveryPointsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Lists the recovery points for a specified gateway. This operation is only supported in the cached volume gateway type. |
| `ListVolumes` | - | `paginated` | - | - | `ListVolumesOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Lists the iSCSI stored volumes of a gateway. Results are sorted by volume ARN. |
| `NotifyWhenUploaded` | - | - | `FileShareARN` | - | `NotifyWhenUploadedOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Sends you notification through Amazon EventBridge when all files written to your file share have been uploaded to Amazon S3. Storage Gateway can send a notification through Amazon EventBridge when all files written to your file share up to that point in time... |
| `RefreshCache` | - | - | `FileShareARN` | - | `RefreshCacheOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Refreshes the cached inventory of objects for the specified file share. This operation finds objects in the Amazon S3 bucket that were added, removed, or replaced since the gateway last listed the bucket's contents and cached the results. |
| `RemoveTagsFromResource` | - | - | `ResourceARN`, `TagKeys` | - | `RemoveTagsFromResourceOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Removes one or more tags from the specified resource. This operation is supported in storage gateways of all types. |
| `ResetCache` | - | - | `GatewayARN` | - | `ResetCacheOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Resets all cache disks that have encountered an error and makes the disks available for reconfiguration as cache storage. If your cache disk encounters an error, the gateway prevents read and write operations on virtual tapes in the gateway. |
| `RetrieveTapeArchive` | - | - | `GatewayARN`, `TapeARN` | - | `RetrieveTapeArchiveOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Retrieves an archived virtual tape from the virtual tape shelf (VTS) to a tape gateway. Virtual tapes archived in the VTS are not associated with any gateway. |
| `RetrieveTapeRecoveryPoint` | - | - | `GatewayARN`, `TapeARN` | - | `RetrieveTapeRecoveryPointOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Retrieves the recovery point for the specified virtual tape. This operation is only supported in the tape gateway type. |
| `SetLocalConsolePassword` | - | - | `GatewayARN`, `LocalConsolePassword` | - | `SetLocalConsolePasswordOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Sets the password for your VM local console. When you log in to the local console for the first time, you log in to the VM with the default credentials. |
| `SetSMBGuestPassword` | - | - | `GatewayARN`, `Password` | - | `SetSMBGuestPasswordOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Sets the password for the guest user `smbguest`. The `smbguest` user is the user when the authentication method for the file share is set to `GuestAccess`. |
| `ShutdownGateway` | - | - | `GatewayARN` | - | `ShutdownGatewayOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Shuts down a Tape Gateway or Volume Gateway. To specify which gateway to shut down, use the Amazon Resource Name (ARN) of the gateway in the body of your request. |
| `StartAvailabilityMonitorTest` | - | - | `GatewayARN` | - | `StartAvailabilityMonitorTestOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Start a test that verifies that the specified gateway is configured for High Availability monitoring in your host environment. This request only initiates the test and that a successful response only indicates that the test was started. |
| `StartCacheReport` | - | - | `BucketRegion`, `ClientToken`, `FileShareARN`, `LocationARN`, `Role` | - | `StartCacheReportOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Starts generating a report of the file metadata currently cached by an S3 File Gateway for a specific file share. You can use this report to identify and resolve issues if you have files failing upload from your gateway to Amazon S3. |
| `StartGateway` | - | - | `GatewayARN` | - | `StartGatewayOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Starts a gateway that you previously shut down (see ShutdownGateway). After the gateway starts, you can then make other API calls, your applications can read from or write to the gateway's storage volumes and you will be able to take snapshot backups. |
| `UpdateAutomaticTapeCreationPolicy` | - | - | `AutomaticTapeCreationRules`, `GatewayARN` | - | `UpdateAutomaticTapeCreationPolicyOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates the automatic tape creation policy of a gateway. Use this to update the policy with a new set of automatic tape creation rules. |
| `UpdateBandwidthRateLimit` | - | - | `GatewayARN` | - | `UpdateBandwidthRateLimitOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates the bandwidth rate limits of a gateway. You can update both the upload and download bandwidth rate limit or specify only one of the two. |
| `UpdateBandwidthRateLimitSchedule` | - | - | `BandwidthRateLimitIntervals`, `GatewayARN` | - | `UpdateBandwidthRateLimitScheduleOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates the bandwidth rate limit schedule for a specified gateway. By default, gateways do not have bandwidth rate limit schedules, which means no bandwidth rate limiting is in effect. |
| `UpdateChapCredentials` | - | - | `InitiatorName`, `SecretToAuthenticateInitiator`, `TargetARN` | - | `UpdateChapCredentialsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates the Challenge-Handshake Authentication Protocol (CHAP) credentials for a specified iSCSI target. By default, a gateway does not have CHAP enabled; however, for added security, you might use it. |
| `UpdateFileSystemAssociation` | - | - | `FileSystemAssociationARN` | - | `UpdateFileSystemAssociationOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates a file system association. This operation is only supported in the FSx File Gateways. |
| `UpdateGatewayInformation` | - | - | `GatewayARN` | - | `UpdateGatewayInformationOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates a gateway's metadata, which includes the gateway's name, time zone, and metadata cache size. To specify which gateway to update, use the Amazon Resource Name (ARN) of the gateway in your request. |
| `UpdateGatewaySoftwareNow` | - | - | `GatewayARN` | - | `UpdateGatewaySoftwareNowOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates the gateway virtual machine (VM) software. The request immediately triggers the software update. |
| `UpdateMaintenanceStartTime` | - | - | `GatewayARN` | - | `UpdateMaintenanceStartTimeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates a gateway's maintenance window schedule, with settings for monthly or weekly cadence, specific day and time to begin maintenance, and which types of updates to apply. Time configuration uses the gateway's time zone. |
| `UpdateNFSFileShare` | - | - | `FileShareARN` | - | `UpdateNFSFileShareOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates a Network File System (NFS) file share. This operation is only supported in S3 File Gateways. |
| `UpdateSMBFileShare` | - | - | `FileShareARN` | - | `UpdateSMBFileShareOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates a Server Message Block (SMB) file share. This operation is only supported for S3 File Gateways. |
| `UpdateSMBFileShareVisibility` | - | - | `FileSharesVisible`, `GatewayARN` | - | `UpdateSMBFileShareVisibilityOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Controls whether the shares on an S3 File Gateway are visible in a net view or browse list. The operation is only supported for S3 File Gateways. |
| `UpdateSMBLocalGroups` | - | - | `GatewayARN`, `SMBLocalGroups` | - | `UpdateSMBLocalGroupsOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates the list of Active Directory users and groups that have special permissions for SMB file shares on the gateway. |
| `UpdateSMBSecurityStrategy` | - | - | `GatewayARN`, `SMBSecurityStrategy` | - | `UpdateSMBSecurityStrategyOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates the SMB security strategy level for an Amazon S3 file gateway. This action is only supported for Amazon S3 file gateways. |
| `UpdateSnapshotSchedule` | - | - | `RecurrenceInHours`, `StartAt`, `VolumeARN` | - | `UpdateSnapshotScheduleOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates a snapshot schedule configured for a gateway volume. This operation is only supported in the cached volume and stored volume gateway types. |
| `UpdateVTLDeviceType` | - | - | `DeviceType`, `VTLDeviceARN` | - | `UpdateVTLDeviceTypeOutput` | `InternalServerError`, `InvalidGatewayRequestException` | Updates the type of medium changer in a tape gateway. When you activate a tape gateway, you select a medium changer type for the tape gateway. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerError` | `structure` | `error`, `message` | An internal server error has occurred during the request. |
| `InvalidGatewayRequestException` | `structure` | `error`, `message` | An exception occurred because an invalid gateway request was issued to the service. |
| `ServiceUnavailableError` | `structure` | `error`, `message` | An internal server error has occurred because the service is unavailable. |
| `ActivateGatewayInput` | `structure` | `ActivationKey`, `GatewayName`, `GatewayRegion`, `GatewayTimezone`, `GatewayType`, `MediumChangerType`, `Tags`, `TapeDriveType` | A JSON object containing one or more of the following fields: ActivateGatewayInput$ActivationKey ActivateGatewayInput$GatewayName ActivateGatewayInput$GatewayRegion... |
| `ActivateGatewayOutput` | `structure` | `GatewayARN` | Storage Gateway returns the Amazon Resource Name (ARN) of the activated gateway. |
| `AddCacheInput` | `structure` | `DiskIds`, `GatewayARN` | - |
| `AddCacheOutput` | `structure` | `GatewayARN` | - |
| `AddTagsToResourceInput` | `structure` | `ResourceARN`, `Tags` | AddTagsToResourceInput |
| `AddTagsToResourceOutput` | `structure` | `ResourceARN` | AddTagsToResourceOutput |
| `AddUploadBufferInput` | `structure` | `DiskIds`, `GatewayARN` | - |
| `AddUploadBufferOutput` | `structure` | `GatewayARN` | - |
| `AddWorkingStorageInput` | `structure` | `DiskIds`, `GatewayARN` | A JSON object containing one or more of the following fields: AddWorkingStorageInput$DiskIds |
| `AddWorkingStorageOutput` | `structure` | `GatewayARN` | A JSON object containing the Amazon Resource Name (ARN) of the gateway for which working storage was configured. |
| `AssignTapePoolInput` | `structure` | `BypassGovernanceRetention`, `PoolId`, `TapeARN` | - |
| `AssignTapePoolOutput` | `structure` | `TapeARN` | - |
| `AssociateFileSystemInput` | `structure` | `AuditDestinationARN`, `CacheAttributes`, `ClientToken`, `EndpointNetworkConfiguration`, `GatewayARN`, `LocationARN`, `Password`, `Tags`, `UserName` | - |
| `AssociateFileSystemOutput` | `structure` | `FileSystemAssociationARN` | - |
| `AttachVolumeInput` | `structure` | `DiskId`, `GatewayARN`, `NetworkInterfaceId`, `TargetName`, `VolumeARN` | AttachVolumeInput |
| `AttachVolumeOutput` | `structure` | `TargetARN`, `VolumeARN` | AttachVolumeOutput |
| `CancelArchivalInput` | `structure` | `GatewayARN`, `TapeARN` | CancelArchivalInput |
| `CancelArchivalOutput` | `structure` | `TapeARN` | CancelArchivalOutput |
| `CancelCacheReportInput` | `structure` | `CacheReportARN` | - |
| `CancelCacheReportOutput` | `structure` | `CacheReportARN` | - |
| `CancelRetrievalInput` | `structure` | `GatewayARN`, `TapeARN` | CancelRetrievalInput |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
