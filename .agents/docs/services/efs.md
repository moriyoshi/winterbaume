# Amazon Elastic File System

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Elastic File System Amazon Elastic File System (Amazon EFS) provides simple, scalable file storage for use with Amazon EC2 Linux and Mac instances in the Amazon Web Services Cloud. With Amazon EFS, storage capacity is elastic, growing and shrinking automatically as you add and remove files, so that your applications have the storage they need, when they need it. For more information, see the Amazon Elastic File System API Reference and the Amazon Elastic File System User Guide.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for Amazon Elastic File System by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon Elastic File System by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon Elastic File System workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Describe`, `Delete`, `Create`, `Put`, `Update` operation families, including `DescribeAccessPoints`, `DescribeAccountPreferences`, `DescribeBackupPolicy`, `DescribeFileSystemPolicy`, `DeleteAccessPoint`, `DeleteFileSystem`.

## Service Identity and Protocol

- AWS model slug: `efs`
- AWS SDK for Rust slug: `efs`
- Model version: `2015-02-01`
- Model file: `vendor/api-models-aws/models/efs/service/2015-02-01/efs-2015-02-01.json`
- SDK ID: `EFS`
- Endpoint prefix: `elasticfilesystem`
- ARN namespace: `elasticfilesystem`
- CloudFormation name: `EFS`
- CloudTrail event source: `elasticfilesystem.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (10), `Delete` (6), `Create` (5), `Put` (4), `Update` (2), `List` (1), `Modify` (1), `Tag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateAccessPoint`, `CreateFileSystem`, `CreateMountTarget`, `CreateReplicationConfiguration`, `CreateTags`, `DeleteAccessPoint`, `DeleteFileSystem`, `DeleteFileSystemPolicy`, `DeleteMountTarget`, `DeleteReplicationConfiguration`, `DeleteTags`, `ModifyMountTargetSecurityGroups`, `PutAccountPreferences`, `PutBackupPolicy`, `PutFileSystemPolicy`, `PutLifecycleConfiguration`, `TagResource`, `UntagResource`, `UpdateFileSystem`, `UpdateFileSystemProtection`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccessPoints`, `DescribeAccountPreferences`, `DescribeBackupPolicy`, `DescribeFileSystemPolicy`, `DescribeFileSystems`, `DescribeLifecycleConfiguration`, `DescribeMountTargetSecurityGroups`, `DescribeMountTargets`, `DescribeReplicationConfigurations`, `DescribeTags`, `ListTagsForResource`.
- Pagination is modelled for 6 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 3 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 31 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/efs/latest/ug/performance.html
- https://docs.aws.amazon.com/efs/latest/ug/manage-fs-access-create-delete-mount-targets.html
- https://docs.aws.amazon.com/efs/latest/ug/enforce-root-directory-access-point.html
- https://docs.aws.amazon.com/efs/latest/ug/enforce-identity-access-points.html

Research outcomes:
- EFS performance depends on file system type, performance mode, throughput mode, storage class, and request size.
- Elastic throughput is recommended for spiky or unpredictable workloads; Provisioned and Bursting modes have different quota and credit behaviour.
- Bursting throughput earns credits based on Standard storage size. Without credits, throughput falls to baseline, but each file system has at least 1 MiBps.
- Switching away from Provisioned throughput or decreasing provisioned throughput is blocked for 24 hours after a Provisioned change.
- Mount targets are created for one VPC at a time. A file system can have only one mount target in a given Availability Zone, and One Zone file systems can have only one mount target in their own zone.
- Access points can enforce a root directory. If the directory does not exist, EFS creates it only when owner UID, owner GID, and permissions were provided.
- Access points can enforce POSIX user and group IDs, replacing the NFS client's IDs for all file operations.
- Access point root identity enforcement involving UID or GID 0 requires ClientRootAccess IAM permission.

Parity implications:
- Model file systems, lifecycle/throughput/performance settings, mount targets, network attributes, access points, and POSIX access-point metadata separately.
- Enforce one-VPC and one-mount-target-per-AZ constraints, plus One Zone placement restrictions.
- Access point mount behaviour should apply root-directory remapping, optional directory creation, POSIX identity replacement, and root-access permission checks.

## Current Network Resource Stub Semantics

EFS currently models mount targets with service-local network metadata.

- `CreateMountTarget` stores the supplied subnet ID and security groups, mints a synthetic mount target ID and network interface ID, and returns a VPC ID from local EFS logic rather than EC2 subnet state.
- Mount target lifecycle and file-system mount target counts are maintained inside EFS state.
- `DescribeMountTargetSecurityGroups` and `ModifyMountTargetSecurityGroups` read and replace the stored security group list only.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Describe

- Operations: `DescribeAccessPoints`, `DescribeAccountPreferences`, `DescribeBackupPolicy`, `DescribeFileSystemPolicy`, `DescribeFileSystems`, `DescribeLifecycleConfiguration`, `DescribeMountTargets`, `DescribeMountTargetSecurityGroups`, `DescribeReplicationConfigurations`, `DescribeTags`
- Traits: `paginated` (5)
- Common required input members in this group: `FileSystemId`

### Delete

- Operations: `DeleteAccessPoint`, `DeleteFileSystem`, `DeleteFileSystemPolicy`, `DeleteMountTarget`, `DeleteReplicationConfiguration`, `DeleteTags`
- Common required input members in this group: `FileSystemId`

### Create

- Operations: `CreateAccessPoint`, `CreateFileSystem`, `CreateMountTarget`, `CreateReplicationConfiguration`, `CreateTags`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `FileSystemId`

### Put

- Operations: `PutAccountPreferences`, `PutBackupPolicy`, `PutFileSystemPolicy`, `PutLifecycleConfiguration`
- Common required input members in this group: `FileSystemId`

### Update

- Operations: `UpdateFileSystem`, `UpdateFileSystemProtection`
- Traits: `idempotent` (1)
- Common required input members in this group: `FileSystemId`

### List

- Operations: `ListTagsForResource`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Modify

- Operations: `ModifyMountTargetSecurityGroups`
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
| `CreateAccessPoint` | `POST /2015-02-01/access-points` | `idempotency-token` | `ClientToken`, `FileSystemId` | `ClientToken` | `AccessPointDescription` | `AccessPointAlreadyExists`, `AccessPointLimitExceeded`, `BadRequest`, `FileSystemNotFound`, `IncorrectFileSystemLifeCycleState`, `InternalServerError`, `ThrottlingException` | Creates an EFS access point. An access point is an application-specific view into an EFS file system that applies an operating system user and group, and a file system path, to any file system request made through th ... |
| `CreateFileSystem` | `POST /2015-02-01/file-systems` | `idempotency-token` | `CreationToken` | `CreationToken` | `FileSystemDescription` | `BadRequest`, `FileSystemAlreadyExists`, `FileSystemLimitExceeded`, `InsufficientThroughputCapacity`, `InternalServerError`, `ThroughputLimitExceeded`, `UnsupportedAvailabilityZone` | Creates a new, empty file system. The operation requires a creation token in the request that Amazon EFS uses to ensure idempotent creation (calling the operation with same creation token has no effect). If a file sy ... |
| `CreateMountTarget` | `POST /2015-02-01/mount-targets` | - | `FileSystemId`, `SubnetId` | - | `MountTargetDescription` | `AvailabilityZonesMismatch`, `BadRequest`, `FileSystemNotFound`, `IncorrectFileSystemLifeCycleState`, `InternalServerError`, `IpAddressInUse`, `MountTargetConflict`, `NetworkInterfaceLimitExceeded`, `NoFreeAddressesInSubnet`, `SecurityGroupLimitExceeded`, `SecurityGroupNotFound`, `SubnetNotFound`, `UnsupportedAvailabilityZone` | Creates a mount target for a file system. You can then mount the file system on EC2 instances by using the mount target. You can create one mount target in each Availability Zone in your VPC. All EC2 instances in a V ... |
| `CreateReplicationConfiguration` | `POST /2015-02-01/file-systems/{SourceFileSystemId}/replication-configuration` | - | `SourceFileSystemId`, `Destinations` | - | `ReplicationConfigurationDescription` | `BadRequest`, `ConflictException`, `FileSystemLimitExceeded`, `FileSystemNotFound`, `IncorrectFileSystemLifeCycleState`, `InsufficientThroughputCapacity`, `InternalServerError`, `ReplicationNotFound`, `ThroughputLimitExceeded`, `UnsupportedAvailabilityZone`, `ValidationException` | Creates a replication conﬁguration to either a new or existing EFS file system. For more information, see Amazon EFS replication in the Amazon EFS User Guide . The replication configuration specifies the following: S ... |
| `CreateTags` | `POST /2015-02-01/create-tags/{FileSystemId}` | - | `FileSystemId`, `Tags` | - | `Unit` | `BadRequest`, `FileSystemNotFound`, `InternalServerError` | DEPRECATED - CreateTags is deprecated and not maintained. To create tags for EFS resources, use the API action. Creates or overwrites tags associated with a file system. Each tag is a key-value pair. If a tag key spe ... |
| `DeleteAccessPoint` | `DELETE /2015-02-01/access-points/{AccessPointId}` | - | `AccessPointId` | - | `Unit` | `AccessPointNotFound`, `BadRequest`, `InternalServerError` | Deletes the specified access point. After deletion is complete, new clients can no longer connect to the access points. Clients connected to the access point at the time of deletion will continue to function until th ... |
| `DeleteFileSystem` | `DELETE /2015-02-01/file-systems/{FileSystemId}` | - | `FileSystemId` | - | `Unit` | `BadRequest`, `FileSystemInUse`, `FileSystemNotFound`, `InternalServerError` | Deletes a file system, permanently severing access to its contents. Upon return, the file system no longer exists and you can't access any contents of the deleted file system. You need to manually delete mount target ... |
| `DeleteFileSystemPolicy` | `DELETE /2015-02-01/file-systems/{FileSystemId}/policy` | - | `FileSystemId` | - | `Unit` | `BadRequest`, `FileSystemNotFound`, `IncorrectFileSystemLifeCycleState`, `InternalServerError` | Deletes the FileSystemPolicy for the specified file system. The default FileSystemPolicy goes into effect once the existing policy is deleted. For more information about the default file system policy, see Using Reso ... |
| `DeleteMountTarget` | `DELETE /2015-02-01/mount-targets/{MountTargetId}` | - | `MountTargetId` | - | `Unit` | `BadRequest`, `DependencyTimeout`, `InternalServerError`, `MountTargetNotFound` | Deletes the specified mount target. This operation forcibly breaks any mounts of the file system by using the mount target that is being deleted, which might disrupt instances or applications using those mounts. To a ... |
| `DeleteReplicationConfiguration` | `DELETE /2015-02-01/file-systems/{SourceFileSystemId}/replication-configuration` | - | `SourceFileSystemId` | - | `Unit` | `BadRequest`, `FileSystemNotFound`, `InternalServerError`, `ReplicationNotFound` | Deletes a replication configuration. Deleting a replication configuration ends the replication process. After a replication configuration is deleted, the destination file system becomes Writeable and its replication ... |
| `DeleteTags` | `POST /2015-02-01/delete-tags/{FileSystemId}` | - | `FileSystemId`, `TagKeys` | - | `Unit` | `BadRequest`, `FileSystemNotFound`, `InternalServerError` | DEPRECATED - DeleteTags is deprecated and not maintained. To remove tags from EFS resources, use the API action. Deletes the specified tags from a file system. If the DeleteTags request includes a tag key that doesn' ... |
| `DescribeAccessPoints` | `GET /2015-02-01/access-points` | `paginated` | - | - | `DescribeAccessPointsResponse` | `AccessPointNotFound`, `BadRequest`, `FileSystemNotFound`, `InternalServerError` | Returns the description of a specific Amazon EFS access point if the AccessPointId is provided. If you provide an EFS FileSystemId , it returns descriptions of all access points for that file system. You can provide ... |
| `DescribeAccountPreferences` | `GET /2015-02-01/account-preferences` | - | - | - | `DescribeAccountPreferencesResponse` | `InternalServerError` | Returns the account preferences settings for the Amazon Web Services account associated with the user making the request, in the current Amazon Web Services Region. |
| `DescribeBackupPolicy` | `GET /2015-02-01/file-systems/{FileSystemId}/backup-policy` | - | `FileSystemId` | - | `BackupPolicyDescription` | `BadRequest`, `FileSystemNotFound`, `InternalServerError`, `PolicyNotFound`, `ValidationException` | Returns the backup policy for the specified EFS file system. |
| `DescribeFileSystemPolicy` | `GET /2015-02-01/file-systems/{FileSystemId}/policy` | - | `FileSystemId` | - | `FileSystemPolicyDescription` | `BadRequest`, `FileSystemNotFound`, `InternalServerError`, `PolicyNotFound` | Returns the FileSystemPolicy for the specified EFS file system. This operation requires permissions for the elasticfilesystem:DescribeFileSystemPolicy action. |
| `DescribeFileSystems` | `GET /2015-02-01/file-systems` | `paginated` | - | - | `DescribeFileSystemsResponse` | `BadRequest`, `FileSystemNotFound`, `InternalServerError` | Returns the description of a specific Amazon EFS file system if either the file system CreationToken or the FileSystemId is provided. Otherwise, it returns descriptions of all file systems owned by the caller's Amazo ... |
| `DescribeLifecycleConfiguration` | `GET /2015-02-01/file-systems/{FileSystemId}/lifecycle-configuration` | - | `FileSystemId` | - | `LifecycleConfigurationDescription` | `BadRequest`, `FileSystemNotFound`, `InternalServerError` | Returns the current LifecycleConfiguration object for the specified EFS file system. Lifecycle management uses the LifecycleConfiguration object to identify when to move files between storage classes. For a file syst ... |
| `DescribeMountTargets` | `GET /2015-02-01/mount-targets` | `paginated` | - | - | `DescribeMountTargetsResponse` | `AccessPointNotFound`, `BadRequest`, `FileSystemNotFound`, `InternalServerError`, `MountTargetNotFound` | Returns the descriptions of all the current mount targets, or a specific mount target, for a file system. When requesting all of the current mount targets, the order of mount targets returned in the response is unspe ... |
| `DescribeMountTargetSecurityGroups` | `GET /2015-02-01/mount-targets/{MountTargetId}/security-groups` | - | `MountTargetId` | - | `DescribeMountTargetSecurityGroupsResponse` | `BadRequest`, `IncorrectMountTargetState`, `InternalServerError`, `MountTargetNotFound` | Returns the security groups currently in effect for a mount target. This operation requires that the network interface of the mount target has been created and the lifecycle state of the mount target is not deleted . ... |
| `DescribeReplicationConfigurations` | `GET /2015-02-01/file-systems/replication-configurations` | `paginated` | - | - | `DescribeReplicationConfigurationsResponse` | `BadRequest`, `FileSystemNotFound`, `InternalServerError`, `ReplicationNotFound`, `ValidationException` | Retrieves the replication configuration for a specific file system. If a file system is not specified, all of the replication configurations for the Amazon Web Services account in an Amazon Web Services Region are re ... |
| `DescribeTags` | `GET /2015-02-01/tags/{FileSystemId}` | `paginated` | `FileSystemId` | - | `DescribeTagsResponse` | `BadRequest`, `FileSystemNotFound`, `InternalServerError` | DEPRECATED - The DescribeTags action is deprecated and not maintained. To view tags associated with EFS resources, use the ListTagsForResource API action. Returns the tags associated with a file system. The order of ... |
| `ListTagsForResource` | `GET /2015-02-01/resource-tags/{ResourceId}` | `paginated` | `ResourceId` | - | `ListTagsForResourceResponse` | `AccessPointNotFound`, `BadRequest`, `FileSystemNotFound`, `InternalServerError` | Lists all tags for a top-level EFS resource. You must provide the ID of the resource that you want to retrieve the tags for. This operation requires permissions for the elasticfilesystem:DescribeAccessPoints action. |
| `ModifyMountTargetSecurityGroups` | `PUT /2015-02-01/mount-targets/{MountTargetId}/security-groups` | - | `MountTargetId` | - | `Unit` | `BadRequest`, `IncorrectMountTargetState`, `InternalServerError`, `MountTargetNotFound`, `SecurityGroupLimitExceeded`, `SecurityGroupNotFound` | Modifies the set of security groups in effect for a mount target. When you create a mount target, Amazon EFS also creates a new network interface. For more information, see CreateMountTarget . This operation replaces ... |
| `PutAccountPreferences` | `PUT /2015-02-01/account-preferences` | - | `ResourceIdType` | - | `PutAccountPreferencesResponse` | `BadRequest`, `InternalServerError` | Use this operation to set the account preference in the current Amazon Web Services Region to use long 17 character (63 bit) or short 8 character (32 bit) resource IDs for new EFS file system and mount target resourc ... |
| `PutBackupPolicy` | `PUT /2015-02-01/file-systems/{FileSystemId}/backup-policy` | - | `FileSystemId`, `BackupPolicy` | - | `BackupPolicyDescription` | `BadRequest`, `FileSystemNotFound`, `IncorrectFileSystemLifeCycleState`, `InternalServerError`, `ValidationException` | Updates the file system's backup policy. Use this action to start or stop automatic backups of the file system. |
| `PutFileSystemPolicy` | `PUT /2015-02-01/file-systems/{FileSystemId}/policy` | - | `FileSystemId`, `Policy` | - | `FileSystemPolicyDescription` | `BadRequest`, `FileSystemNotFound`, `IncorrectFileSystemLifeCycleState`, `InternalServerError`, `InvalidPolicyException` | Applies an Amazon EFS FileSystemPolicy to an Amazon EFS file system. A file system policy is an IAM resource-based policy and can contain multiple policy statements. A file system always has exactly one file system p ... |
| `PutLifecycleConfiguration` | `PUT /2015-02-01/file-systems/{FileSystemId}/lifecycle-configuration` | - | `FileSystemId`, `LifecyclePolicies` | - | `LifecycleConfigurationDescription` | `BadRequest`, `FileSystemNotFound`, `IncorrectFileSystemLifeCycleState`, `InternalServerError` | Use this action to manage storage for your file system. A LifecycleConfiguration consists of one or more LifecyclePolicy objects that define the following: TransitionToIA – When to move files in the file system from ... |
| `TagResource` | `POST /2015-02-01/resource-tags/{ResourceId}` | - | `ResourceId`, `Tags` | - | `Unit` | `AccessPointNotFound`, `BadRequest`, `FileSystemNotFound`, `InternalServerError` | Creates a tag for an EFS resource. You can create tags for EFS file systems and access points using this API operation. This operation requires permissions for the elasticfilesystem:TagResource action. |
| `UntagResource` | `DELETE /2015-02-01/resource-tags/{ResourceId}` | - | `ResourceId`, `TagKeys` | - | `Unit` | `AccessPointNotFound`, `BadRequest`, `FileSystemNotFound`, `InternalServerError` | Removes tags from an EFS resource. You can remove tags from EFS file systems and access points using this API operation. This operation requires permissions for the elasticfilesystem:UntagResource action. |
| `UpdateFileSystem` | `PUT /2015-02-01/file-systems/{FileSystemId}` | - | `FileSystemId` | - | `FileSystemDescription` | `BadRequest`, `FileSystemNotFound`, `IncorrectFileSystemLifeCycleState`, `InsufficientThroughputCapacity`, `InternalServerError`, `ThroughputLimitExceeded`, `TooManyRequests` | Updates the throughput mode or the amount of provisioned throughput of an existing file system. |
| `UpdateFileSystemProtection` | `PUT /2015-02-01/file-systems/{FileSystemId}/protection` | `idempotent` | `FileSystemId` | - | `FileSystemProtectionDescription` | `BadRequest`, `FileSystemNotFound`, `IncorrectFileSystemLifeCycleState`, `InsufficientThroughputCapacity`, `InternalServerError`, `ReplicationAlreadyExists`, `ThroughputLimitExceeded`, `TooManyRequests` | Updates protection on the file system. This operation requires permissions for the elasticfilesystem:UpdateFileSystemProtection action. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteReplicationConfiguration` | - | `DeletionMode -> deletionMode` | - | - |
| `DescribeAccessPoints` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken`, `AccessPointId -> AccessPointId`, `FileSystemId -> FileSystemId` | - | - |
| `DescribeFileSystems` | - | `MaxItems -> MaxItems`, `Marker -> Marker`, `CreationToken -> CreationToken`, `FileSystemId -> FileSystemId` | - | - |
| `DescribeMountTargets` | - | `MaxItems -> MaxItems`, `Marker -> Marker`, `FileSystemId -> FileSystemId`, `MountTargetId -> MountTargetId`, `AccessPointId -> AccessPointId` | - | - |
| `DescribeReplicationConfigurations` | - | `FileSystemId -> FileSystemId`, `NextToken -> NextToken`, `MaxResults -> MaxResults` | - | - |
| `DescribeTags` | - | `MaxItems -> MaxItems`, `Marker -> Marker` | - | - |
| `ListTagsForResource` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessPointAlreadyExists` | `structure` | ErrorCode, Message, AccessPointId | Returned if the access point that you are trying to create already exists, with the creation token you provided in the request. |
| `AccessPointLimitExceeded` | `structure` | ErrorCode, Message | Returned if the Amazon Web Services account has already created the maximum number of access points allowed per file system. For more informaton, see https: ... |
| `AccessPointNotFound` | `structure` | ErrorCode, Message | Returned if the specified AccessPointId value doesn't exist in the requester's Amazon Web Services account. |
| `AvailabilityZonesMismatch` | `structure` | ErrorCode, Message | Returned if the Availability Zone that was specified for a mount target is different from the Availability Zone that was specified for One Zone storage. For ... |
| `BadRequest` | `structure` | ErrorCode, Message | Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter. |
| `ConflictException` | `structure` | ErrorCode, Message | Returned if the source file system in a replication is encrypted but the destination file system is unencrypted. |
| `DependencyTimeout` | `structure` | ErrorCode, Message | The service timed out trying to fulfill the request, and the client should try the call again. |
| `FileSystemAlreadyExists` | `structure` | ErrorCode, Message, FileSystemId | Returned if the file system you are trying to create already exists, with the creation token you provided. |
| `FileSystemInUse` | `structure` | ErrorCode, Message | Returned if a file system has mount targets. |
| `FileSystemLimitExceeded` | `structure` | ErrorCode, Message | Returned if the Amazon Web Services account has already created the maximum number of file systems allowed per account. |
| `FileSystemNotFound` | `structure` | ErrorCode, Message | Returned if the specified FileSystemId value doesn't exist in the requester's Amazon Web Services account. |
| `IncorrectFileSystemLifeCycleState` | `structure` | ErrorCode, Message | Returned if the file system's lifecycle state is not "available". |
| `IncorrectMountTargetState` | `structure` | ErrorCode, Message | Returned if the mount target is not in the correct state for the operation. |
| `InsufficientThroughputCapacity` | `structure` | ErrorCode, Message | Returned if there's not enough capacity to provision additional throughput. This value might be returned when you try to create a file system in provisioned ... |
| `InternalServerError` | `structure` | ErrorCode, Message | Returned if an error occurred on the server side. |
| `InvalidPolicyException` | `structure` | ErrorCode, Message | Returned if the FileSystemPolicy is malformed or contains an error such as a parameter value that is not valid or a missing required parameter. Returned in ... |
| `IpAddressInUse` | `structure` | ErrorCode, Message | Returned if the request specified an IpAddress that is already in use in the subnet. |
| `MountTargetConflict` | `structure` | ErrorCode, Message | Returned if the mount target would violate one of the specified restrictions based on the file system's existing mount targets. |
| `MountTargetNotFound` | `structure` | ErrorCode, Message | Returned if there is no mount target with the specified ID found in the caller's Amazon Web Services account. |
| `NetworkInterfaceLimitExceeded` | `structure` | ErrorCode, Message | The calling account has reached the limit for elastic network interfaces for the specific Amazon Web Services Region. Either delete some network interfaces ... |
| `NoFreeAddressesInSubnet` | `structure` | ErrorCode, Message | Returned if IpAddress was not specified in the request and there are no free IP addresses in the subnet. |
| `PolicyNotFound` | `structure` | ErrorCode, Message | Returned if no backup is specified for a One Zone EFS file system. |
| `ReplicationAlreadyExists` | `structure` | ErrorCode, Message | Returned if the file system is already included in a replication configuration.> |
| `ReplicationNotFound` | `structure` | ErrorCode, Message | Returned if the specified file system does not have a replication configuration. |
| `SecurityGroupLimitExceeded` | `structure` | ErrorCode, Message | Returned if the number of SecurityGroups specified in the request is greater than the limit, which is based on account quota. Either delete some security gr ... |
| `SecurityGroupNotFound` | `structure` | ErrorCode, Message | Returned if one of the specified security groups doesn't exist in the subnet's virtual private cloud (VPC). |
| `SubnetNotFound` | `structure` | ErrorCode, Message | Returned if there is no subnet with ID SubnetId provided in the request. |
| `ThrottlingException` | `structure` | ErrorCode, Message | Returned when the CreateAccessPoint API action is called too quickly and the number of Access Points on the file system is nearing the limit of 120 . |
| `ThroughputLimitExceeded` | `structure` | ErrorCode, Message | Returned if the throughput mode or amount of provisioned throughput can't be changed because the throughput limit of 1024 MiB/s has been reached. |
| `TooManyRequests` | `structure` | ErrorCode, Message | Returned if you don’t wait at least 24 hours before either changing the throughput mode, or decreasing the Provisioned Throughput value. |
| `UnsupportedAvailabilityZone` | `structure` | ErrorCode, Message | Returned if the requested Amazon EFS functionality is not available in the specified Availability Zone. |
| `ValidationException` | `structure` | ErrorCode, Message | Returned if the Backup service is not available in the Amazon Web Services Region in which the request was made. |
| `CreateAccessPointRequest` | `structure` | ClientToken, Tags, FileSystemId, PosixUser, RootDirectory | - |
| `AccessPointDescription` | `structure` | ClientToken, Name, Tags, AccessPointId, AccessPointArn, FileSystemId, PosixUser, RootDirectory, OwnerId, LifeCycleState | Provides a description of an EFS file system access point. |
| `CreateFileSystemRequest` | `structure` | CreationToken, PerformanceMode, Encrypted, KmsKeyId, ThroughputMode, ProvisionedThroughputInMibps, AvailabilityZoneName, Backup, Tags | - |
| `FileSystemDescription` | `structure` | OwnerId, CreationToken, FileSystemId, FileSystemArn, CreationTime, LifeCycleState, Name, NumberOfMountTargets, SizeInBytes, PerformanceMode, Encrypted, KmsKeyId, ... (+6) | A description of the file system. |
| `CreateMountTargetRequest` | `structure` | FileSystemId, SubnetId, IpAddress, Ipv6Address, IpAddressType, SecurityGroups | - |
| `MountTargetDescription` | `structure` | OwnerId, MountTargetId, FileSystemId, SubnetId, LifeCycleState, IpAddress, Ipv6Address, NetworkInterfaceId, AvailabilityZoneId, AvailabilityZoneName, VpcId | Provides a description of a mount target. |
| `CreateReplicationConfigurationRequest` | `structure` | SourceFileSystemId, Destinations | - |
| `ReplicationConfigurationDescription` | `structure` | SourceFileSystemId, SourceFileSystemRegion, SourceFileSystemArn, OriginalSourceFileSystemArn, CreationTime, Destinations, SourceFileSystemOwnerId | Describes the replication configuration for a specific file system. |
| `DeletionMode` | `enum` | ALL_CONFIGURATIONS, LOCAL_CONFIGURATION_ONLY | - |
| `IpAddressType` | `enum` | IPV4_ONLY, IPV6_ONLY, DUAL_STACK | - |
| `LifeCycleState` | `enum` | CREATING, AVAILABLE, UPDATING, DELETING, DELETED, ERROR | - |
| `PerformanceMode` | `enum` | GENERAL_PURPOSE, MAX_IO | - |
| `ReplicationOverwriteProtection` | `enum` | ENABLED, DISABLED, REPLICATING | - |
| `ReplicationStatus` | `enum` | ENABLED, ENABLING, DELETING, ERROR, PAUSED, PAUSING | - |
| `Resource` | `enum` | FileSystem, MountTarget | An EFS resource, for example a file system or a mount target. |
| `ResourceIdType` | `enum` | LongId, ShortId | A preference indicating a choice to use 63bit/32bit IDs for all applicable resources. |
| `Status` | `enum` | ENABLED, ENABLING, DISABLED, DISABLING | - |
| `ThroughputMode` | `enum` | BURSTING, PROVISIONED, ELASTIC | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
