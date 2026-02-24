# Amazon S3 Files

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

S3 Files makes S3 buckets accessible as high-performance file systems powered by EFS. It exposes S3 data through file system interfaces with low-latency mount targets, while preserving a linked S3 bucket as the object storage backing store. The service is aimed at workloads that need both file system and object access to the same data, including AI/ML, media processing, and hybrid storage workflows.

## Possible Usage Scenarios

- From the AWS documentation and model: represent documented Amazon S3 Files workflows in the local mock. Key conceptual resources include file systems, mount targets, access points, file system policies, synchronisation configurations, and tags.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Create`, `Get`, `List`, `Delete`, `Put`, and `Update` operation families, including `CreateFileSystem`, `CreateMountTarget`, `CreateAccessPoint`, `PutSynchronizationConfiguration`, and `UpdateMountTarget`.
- Exercise S3-backed file system creation with bucket ARN, optional prefix scoping, IAM role ARN, KMS key ID, idempotency token, and the `acceptBucketWarning` override for large-prefix warnings.
- Exercise mount target networking rules: one VPC at a time per file system, at most one mount target per Availability Zone, subnet/IP address type handling, security groups, and security group updates.
- Exercise access point enforcement of POSIX user identity, root directory restriction, optional root directory creation permissions, immutable access point settings, and quota conflict behaviour near the access point limit.
- Exercise synchronisation configuration rules, including import triggers, prefix specificity, size thresholds, expiration by last access, and optimistic concurrency through `latestVersionNumber`.

## Service Identity and Protocol

- AWS model slug: `s3files`
- AWS SDK for Rust slug: `s3files`
- Model version: `2025-05-05`
- Model file: `vendor/api-models-aws/models/s3files/service/2025-05-05/s3files-2025-05-05.json`
- SDK ID: `S3Files`
- Endpoint prefix: `s3files`
- ARN namespace: `s3files`
- CloudFormation name: `S3Files`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (5), `Delete` (4), `List` (4), `Create` (3), `Put` (2), `Tag` (1), `Untag` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, deletion, and in-use semantics: `CreateAccessPoint`, `CreateFileSystem`, `CreateMountTarget`, `DeleteAccessPoint`, `DeleteFileSystem`, `DeleteFileSystemPolicy`, `DeleteMountTarget`, `PutFileSystemPolicy`, `PutSynchronizationConfiguration`, `TagResource`, `UntagResource`, and `UpdateMountTarget`.
- Read/list operations should define not-found behaviour, filtering, ordering, pagination, and empty-result shapes: `GetAccessPoint`, `GetFileSystem`, `GetFileSystemPolicy`, `GetMountTarget`, `GetSynchronizationConfiguration`, `ListAccessPoints`, `ListFileSystems`, `ListMountTargets`, and `ListTagsForResource`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 11 operations. `CreateAccessPoint` and `CreateFileSystem` also expose explicit `clientToken` idempotency tokens.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags resource ID validation.
- 21 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `EFS`, `IAM`, `KMS`, and `EC2/VPC`.
- The service has no Smithy `resource` shapes in the vendored model, but the public CloudFormation surface exposes `AWS::S3Files::FileSystem`, `AWS::S3Files::MountTarget`, `AWS::S3Files::AccessPoint`, and `AWS::S3Files::FileSystemPolicy`.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-files-resources.html
- https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-files-file-systems-creating.html
- https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-files-file-systems-deleting.html
- https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-files-mount-targets-creating.html
- https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-files-access-points-creating.html
- https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-files-file-system-policies-creating.html
- https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-files-synchronization-customizing.html
- https://docs.aws.amazon.com/AmazonS3/latest/API/API_Operations_Amazon_S3_Files.html
- https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference/AWS_S3Files.html

Research outcomes:
- A file system links to an S3 general purpose bucket or prefix and uses an IAM role that S3 Files assumes to read from and write to the bucket.
- File system states documented by the user guide are `AVAILABLE`, `CREATING`, `DELETING`, `DELETED`, and `ERROR`; the Smithy enum uses lower-case wire values and also includes `updating`.
- Console creation applies defaults that are not automatic in the API workflow: encryption from the source bucket, an IAM role, mount targets in every Availability Zone in the chosen VPC, and one access point.
- File system deletion requires associated mount targets and access points to be removed first. If unsynchronised changes exist, deletion fails unless the caller uses the force-delete option.
- Mount targets provide network access in one Availability Zone and VPC. A file system can have mount targets in only one VPC at a time, and only one mount target in a given Availability Zone.
- Access points can enforce a POSIX identity and root directory. They cannot be edited after creation; updates require delete-and-recreate.
- If access point root directory ownership and permissions are omitted and the directory does not already exist, S3 Files does not create the directory and mounts through that access point fail.
- A file system either has the default empty file system policy or one explicit policy. Policy changes can take several minutes to take effect, and policy text has a documented 20,000 character limit.
- Synchronisation configuration has import data rules and expiration data rules. Import rules are prefix based, the most specific prefix wins, there must be exactly one root rule, and the API accepts up to 10 import rules. Expiration removes data after a last-access window once changes have already been synchronised to S3.

Parity implications:
- Model file systems as S3-backed resources with lifecycle state, bucket/prefix scope, IAM role, optional KMS key, tags, unsynchronised-change state, and optional file system policy.
- Model mount targets separately from file systems, including VPC/subnet/AZ, IP address type, generated network interface ID, security groups, lifecycle state, and updateable security groups.
- Model access points separately from file systems, with immutable POSIX user and root-directory settings plus quota and quick-successive-create conflict/throttling cases.
- Implement synchronisation configuration as versioned file-system state so stale `latestVersionNumber` writes return `ConflictException`.
- Keep console-created defaults out of API behaviour unless a scenario explicitly seeds them; CLI/API users create file systems, mount targets, access points, policies, and tags as separate operations.

## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `FileSystem` | `fileSystemId`, `fileSystemArn`, `bucket`, `prefix` | `CreateFileSystem`, `DeleteFileSystem` | `GetFileSystem`, `ListFileSystems`, `PutFileSystemPolicy`, `DeleteFileSystemPolicy`, `GetFileSystemPolicy`, `PutSynchronizationConfiguration`, `GetSynchronizationConfiguration`, `TagResource`, `UntagResource`, `ListTagsForResource` | Shared file system linked to an S3 bucket or prefix. |
| `MountTarget` | `mountTargetId`, `fileSystemId`, `subnetId`, `vpcId`, `availabilityZoneId`, `networkInterfaceId` | `CreateMountTarget`, `DeleteMountTarget` | `GetMountTarget`, `ListMountTargets`, `UpdateMountTarget` | Network endpoint for mounting a file system from compute resources in one Availability Zone. |
| `AccessPoint` | `accessPointId`, `accessPointArn`, `fileSystemId` | `CreateAccessPoint`, `DeleteAccessPoint` | `GetAccessPoint`, `ListAccessPoints` | Application-specific file system entry point that can enforce POSIX identity and a root directory. |
| `FileSystemPolicy` | `fileSystemId` | `PutFileSystemPolicy`, `DeleteFileSystemPolicy` | `GetFileSystemPolicy` | Optional IAM resource policy controlling NFS client access. |
| `SynchronizationConfiguration` | `fileSystemId`, `latestVersionNumber` | `PutSynchronizationConfiguration` | `GetSynchronizationConfiguration` | File-system-level rules for importing data from S3 and expiring cached data. |

## Current Network Resource Stub Semantics

S3Files currently has one of the more detailed local network stubs for mount targets.

- `CreateMountTarget` requires a subnet ID, stores security groups, derives a VPC ID deterministically from the subnet ID, derives an availability-zone ID from the subnet ID, and mints an `eni-` network interface ID.
- File-system mount targets are constrained inside S3Files state to one derived VPC per file system and one mount target per derived availability zone.
- `ListMountTargets` can filter by stored VPC ID, and `UpdateMountTarget` replaces the stored security group list.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### Get

- Operations: `GetAccessPoint`, `GetFileSystem`, `GetFileSystemPolicy`, `GetMountTarget`, `GetSynchronizationConfiguration`
- Traits: `readonly` (5)
- Common required input members in this group: `accessPointId`, `fileSystemId`, `mountTargetId`

### Delete

- Operations: `DeleteAccessPoint`, `DeleteFileSystem`, `DeleteFileSystemPolicy`, `DeleteMountTarget`
- Traits: `idempotent` (4)
- Common required input members in this group: `accessPointId`, `fileSystemId`, `mountTargetId`

### List

- Operations: `ListAccessPoints`, `ListFileSystems`, `ListMountTargets`, `ListTagsForResource`
- Traits: `paginated` (4), `readonly` (4)
- Common required input members in this group: `fileSystemId`, `resourceId`

### Create

- Operations: `CreateAccessPoint`, `CreateFileSystem`, `CreateMountTarget`
- Traits: `idempotent` (3), `idempotency-token` (2)
- Common required input members in this group: `bucket`, `fileSystemId`, `roleArn`, `subnetId`

### Put

- Operations: `PutFileSystemPolicy`, `PutSynchronizationConfiguration`
- Traits: `idempotent` (2)
- Common required input members in this group: `expirationDataRules`, `fileSystemId`, `importDataRules`, `policy`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceId`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceId`, `tagKeys`

### Update

- Operations: `UpdateMountTarget`
- Traits: `idempotent` (1)
- Common required input members in this group: `mountTargetId`, `securityGroups`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateAccessPoint` | `PUT /access-points` | `idempotent`, `idempotency-token` | `fileSystemId` | `clientToken` | `CreateAccessPointResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates an S3 File System Access Point for application-specific access with POSIX user identity and root directory enforcement. |
| `CreateFileSystem` | `PUT /file-systems` | `idempotent`, `idempotency-token` | `bucket`, `roleArn` | `clientToken` | `CreateFileSystemResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates an S3 File System resource scoped to a bucket or prefix within a bucket, using an IAM role that grants S3 Files bucket access. |
| `CreateMountTarget` | `PUT /mount-targets` | `idempotent` | `fileSystemId`, `subnetId` | - | `CreateMountTargetResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a mount target endpoint for mounting the S3 File System from compute resources in a specific Availability Zone and VPC. |
| `DeleteAccessPoint` | `DELETE /access-points/{accessPointId}` | `idempotent` | `accessPointId` | - | `Unit` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes an S3 File System Access Point. This operation is irreversible. |
| `DeleteFileSystem` | `DELETE /file-systems/{fileSystemId}` | `idempotent` | `fileSystemId` | - | `Unit` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes an S3 File System, optionally forcing deletion when pending export data exists. |
| `DeleteFileSystemPolicy` | `DELETE /file-systems/{fileSystemId}/policy` | `idempotent` | `fileSystemId` | - | `Unit` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes the IAM resource policy of an S3 File System. |
| `DeleteMountTarget` | `DELETE /mount-targets/{mountTargetId}` | `idempotent` | `mountTargetId` | - | `Unit` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes the specified mount target. This operation is irreversible. |
| `GetAccessPoint` | `GET /access-points/{accessPointId}` | `readonly` | `accessPointId` | - | `GetAccessPointResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns resource information for an S3 File System Access Point. |
| `GetFileSystem` | `GET /file-systems/{fileSystemId}` | `readonly` | `fileSystemId` | - | `GetFileSystemResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns resource information for the specified S3 File System, including status, configuration, and metadata. |
| `GetFileSystemPolicy` | `GET /file-systems/{fileSystemId}/policy` | `readonly` | `fileSystemId` | - | `GetFileSystemPolicyResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns the IAM resource policy of an S3 File System. |
| `GetMountTarget` | `GET /mount-targets/{mountTargetId}` | `readonly` | `mountTargetId` | - | `GetMountTargetResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns detailed resource information for the specified mount target, including network configuration. |
| `GetSynchronizationConfiguration` | `GET /file-systems/{fileSystemId}/synchronization-configuration` | `readonly` | `fileSystemId` | - | `GetSynchronizationConfigurationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns the synchronisation configuration for the specified S3 File System, including import data rules and expiration data rules. |
| `ListAccessPoints` | `GET /access-points` | `readonly`, `paginated` | `fileSystemId` | - | `ListAccessPointsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns resource information for all S3 File System Access Points associated with the specified S3 File System. |
| `ListFileSystems` | `GET /file-systems` | `readonly`, `paginated` | - | - | `ListFileSystemsResponse` | `InternalServerException`, `ValidationException` | Returns a list of all S3 File Systems owned by the account, with optional filtering by bucket. |
| `ListMountTargets` | `GET /mount-targets` | `readonly`, `paginated` | - | - | `ListMountTargetsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns resource information for all mount targets, with optional filtering by file system, access point, and VPC. |
| `ListTagsForResource` | `GET /resource-tags/{resourceId}` | `readonly`, `paginated` | `resourceId` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists all tags for S3 Files resources. |
| `PutFileSystemPolicy` | `PUT /file-systems/{fileSystemId}/policy` | `idempotent` | `fileSystemId`, `policy` | - | `PutFileSystemPolicyResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Creates or replaces the IAM resource policy for an S3 File System to control access permissions. |
| `PutSynchronizationConfiguration` | `PUT /file-systems/{fileSystemId}/synchronization-configuration` | `idempotent` | `fileSystemId`, `importDataRules`, `expirationDataRules` | - | `PutSynchronizationConfigurationResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Creates or updates the synchronisation configuration for the specified S3 File System, including import data rules and expiration data rules. |
| `TagResource` | `POST /resource-tags/{resourceId}` | - | `resourceId`, `tags` | - | `Unit` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Creates tags for S3 Files resources using standard AWS tagging APIs. |
| `UntagResource` | `DELETE /resource-tags/{resourceId}` | `idempotent` | `resourceId`, `tagKeys` | - | `Unit` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes tags from S3 Files resources. |
| `UpdateMountTarget` | `PUT /mount-targets/{mountTargetId}` | `idempotent` | `mountTargetId`, `securityGroups` | - | `UpdateMountTargetResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates the mount target resource, specifically security group configurations. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ConflictException` | `structure` | `errorCode`, `message`, `resourceId`, `resourceType` | The request conflicts with the current resource state, such as creating an existing resource or deleting an in-use resource. |
| `InternalServerException` | `structure` | `errorCode`, `message` | An internal server error occurred. Retry the request. |
| `ResourceNotFoundException` | `structure` | `errorCode`, `message` | The specified resource was not found. |
| `ServiceQuotaExceededException` | `structure` | `errorCode`, `message` | The request would exceed a service quota. |
| `ThrottlingException` | `structure` | `errorCode`, `message` | The request was throttled. |
| `ValidationException` | `structure` | `errorCode`, `message` | The input parameters are not valid. |
| `CreateFileSystemRequest` | `structure` | `bucket`, `prefix`, `clientToken`, `kmsKeyId`, `roleArn`, `tags`, `acceptBucketWarning` | Creates a file system scoped to an S3 bucket or optional prefix. |
| `CreateFileSystemResponse` | `structure` | `creationTime`, `fileSystemArn`, `fileSystemId`, `bucket`, `prefix`, `clientToken`, `kmsKeyId`, `status`, `statusMessage`, `roleArn`, `ownerId`, `tags`, `name` | Describes the created file system. |
| `CreateMountTargetRequest` | `structure` | `fileSystemId`, `subnetId`, `ipv4Address`, `ipv6Address`, `ipAddressType`, `securityGroups` | Creates a mount target in a subnet with optional IP and security group configuration. |
| `CreateMountTargetResponse` | `structure` | `availabilityZoneId`, `ownerId`, `mountTargetId`, `fileSystemId`, `subnetId`, `ipv4Address`, `ipv6Address`, `networkInterfaceId`, `vpcId`, `securityGroups`, `status`, `statusMessage` | Describes the created mount target and managed network interface. |
| `CreateAccessPointRequest` | `structure` | `clientToken`, `tags`, `fileSystemId`, `posixUser`, `rootDirectory` | Creates an access point with optional POSIX user and root directory enforcement. |
| `CreateAccessPointResponse` | `structure` | `accessPointArn`, `accessPointId`, `clientToken`, `fileSystemId`, `status`, `ownerId`, `posixUser`, `rootDirectory`, `tags`, `name` | Describes the created access point. |
| `DeleteFileSystemRequest` | `structure` | `fileSystemId`, `forceDelete` | Deletes a file system, optionally accepting loss of unsynchronised changes. |
| `PutFileSystemPolicyRequest` | `structure` | `fileSystemId`, `policy` | Replaces the file system's IAM resource policy. |
| `PutSynchronizationConfigurationRequest` | `structure` | `fileSystemId`, `latestVersionNumber`, `importDataRules`, `expirationDataRules` | Updates synchronisation configuration with optional optimistic concurrency. |
| `GetSynchronizationConfigurationResponse` | `structure` | `latestVersionNumber`, `importDataRules`, `expirationDataRules` | Returns the current synchronisation configuration and version number. |
| `ListFileSystemsDescription` | `structure` | `creationTime`, `fileSystemArn`, `fileSystemId`, `name`, `bucket`, `status`, `statusMessage`, `roleArn`, `ownerId` | File system summary returned by list operations. |
| `ListMountTargetsDescription` | `structure` | `availabilityZoneId`, `fileSystemId`, `ipv4Address`, `ipv6Address`, `status`, `statusMessage`, `mountTargetId`, `networkInterfaceId`, `ownerId`, `subnetId`, `vpcId` | Mount target summary returned by list operations. |
| `ListAccessPointsDescription` | `structure` | `accessPointArn`, `accessPointId`, `fileSystemId`, `status`, `ownerId`, `posixUser`, `rootDirectory`, `name` | Access point summary returned by list operations. |
| `PosixUser` | `structure` | `uid`, `gid`, `secondaryGids` | POSIX identity enforced for requests through an access point. |
| `RootDirectory` | `structure` | `path`, `creationPermissions` | Access point root directory and optional creation permissions. |
| `ImportDataRule` | `structure` | `prefix`, `trigger`, `sizeLessThan` | Controls how data is imported from S3 into the file system. |
| `ExpirationDataRule` | `structure` | `daysAfterLastAccess` | Controls when cached data expires from the file system. |
| `ImportTrigger` | `enum` | `ON_DIRECTORY_FIRST_ACCESS`, `ON_FILE_ACCESS` | Import timing mode for import data rules. |
| `IpAddressType` | `enum` | `IPV4_ONLY`, `IPV6_ONLY`, `DUAL_STACK` | Requested IP address type for a mount target. |
| `LifeCycleState` | `enum` | `available`, `creating`, `deleting`, `deleted`, `error`, `updating` | Lifecycle state for file systems, mount targets, and access points. |

## Winterbaume LTM Notes

Sources: .agents/docs/LTM/new-service-implementation-patterns.md, .agents/docs/LTM/core-service-expansion-and-coverage.md, .agents/docs/LTM/quality-gate-workflow-and-recurring-findings.md.

Mode: full distillation.

- Winterbaume now covers all 21 S3 Files operations. The first pass implemented FileSystem lifecycle plus tags; the second pass added mount targets, access points, file-system policy, synchronisation configuration, and mount-target update. No 501 stubs remain.
- `CreateFileSystem` and `CreateAccessPoint` idempotency currently replay by `clientToken` only. Real AWS rejects mismatched token replays with `ConflictException`; Winterbaume returns the first resource for the token.
- File systems become `available` immediately after creation. Real S3 Files documents lifecycle states such as `creating`, `available`, `deleting`, `deleted`, `error`, and Smithy also includes `updating`; transitions are a future parity area.
- ARN and id shapes are plausible but provisional: file system ids use `fs-{uuid_simple}` and ARNs use `arn:aws:s3files:{region}:{account_id}:file-system/{fs_id}`. Re-check the published AWS shape before relying on exact formatting.
- `ListFileSystems` supports the optional `bucket` query filter. Pagination tokens are not modelled yet.
- `UntagResource` parses repeated query parameters such as `?tagKeys=a&tagKeys=b`. This is the REST-JSON list-query convention and should eventually move to a shared multi-value query helper in `winterbaume-core`.
- Mount-target state is local to S3 Files rather than validated against EC2. The mock parses subnet ids shaped `subnet-<vpc-tag>-<az-tag>` to derive VPC grouping, hashes subnet bytes modulo six to derive `use1-az1` through `use1-az6`, and enforces one derived VPC per file system plus one mount target per derived AZ. Tests that create multiple mount targets must choose subnet strings that avoid unintended AZ hash collisions.
- `DeleteFileSystem` returns `ConflictException` while any mount target or access point still references the file system. The `forceDelete` query parameter is parsed but currently a no-op for this dependency check: deletion without dependants succeeds, and deletion with dependants fails until they are explicitly removed.
- File-system policy support enforces the documented 20,000 character limit. `GetFileSystemPolicy` returns `ResourceNotFoundException` when the file system exists but no policy is set.
- Synchronisation configuration stores a version, rejects stale `latestVersionNumber` writes with `ConflictException`, bumps the version on each successful Put, and enforces the 10-rule cap on `importDataRules`. The exactly-one-root-rule and most-specific-prefix semantics from AWS documentation are not modelled yet.
- SDK accessor shapes vary within this service: some required-looking outputs expose `&str` getters while nearby outputs expose `Option<&str>`. Let the compiled SDK type guide tests instead of inferring getter optionality from generated Winterbaume wire structs.
- State-view tests should prefer small fixture builders plus `..Default::default()` over raw `S3FilesStateView` / `FileSystemView` literals. Adding policy, synchronisation configuration, mount-target, and access-point view fields otherwise breaks every literal fixture.
- No Terraform converter or Terraform E2E test is expected until the Terraform AWS provider exposes `aws_s3files_*` resources. `/write-tests s3files` remains the publication-readiness follow-up for scenario inventory and documentation-derived scenarios.

## Research Checklist for Parity Work

- Confirm lifecycle transitions for file systems, mount targets, and access points, including creation/deletion timing and when deleted resources remain observable.
- Confirm exact not-found, already-exists, conflict, validation, throttling, quota, and in-use error names and messages.
- Confirm pagination token format, result ordering, default limits, and empty collection shape for all list operations.
- Confirm idempotency-token behaviour for `CreateFileSystem` and `CreateAccessPoint`, especially mismatched replay parameters.
- Confirm S3 bucket/prefix validation, large-prefix warning behaviour, force delete behaviour, and unsynchronised-change state.
- Confirm VPC/AZ/subnet/security group constraints for mount target creation and security group update.
- Confirm access point POSIX and root-directory mount semantics, including directory creation failures when creation permissions are omitted.
- Confirm synchronisation configuration defaults, rule validation, most-specific-prefix matching, and stale-version `ConflictException` behaviour.
