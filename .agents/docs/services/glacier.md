# Amazon Glacier

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Glacier (Glacier) is a storage solution for "cold data." Glacier is an extremely low-cost storage service that provides secure, durable, and easy-to-use storage for data backup and archival. With Glacier, customers can store their data cost effectively for months, years, or decades. Glacier also enables customers to offload the administrative burdens of operating and scaling storage to AWS, so they don't have to worry about capacity planning, hardware provisioning, data replication, hardware failure and recovery, or time-consuming hardware migrations. Glacier is a great storage choice when low storage cost is paramount and your data is rarely retrieved. If your application requires fast or frequent access to your data, consider using Amazon S3.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon Glacier resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Glacier workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `List`, `Get`, `Delete`, `Initiate`, `Set` operation families, including `ListJobs`, `ListMultipartUploads`, `ListParts`, `ListProvisionedCapacity`, `GetDataRetrievalPolicy`, `GetJobOutput`.

## Service Identity and Protocol

- AWS model slug: `glacier`
- AWS SDK for Rust slug: `glacier`
- Model version: `2012-06-01`
- Model file: `vendor/api-models-aws/models/glacier/service/2012-06-01/glacier-2012-06-01.json`
- SDK ID: `Glacier`
- Endpoint prefix: `glacier`
- ARN namespace: `glacier`
- CloudFormation name: `Glacier`
- CloudTrail event source: `glacier.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (6), `Get` (5), `Delete` (4), `Initiate` (3), `Set` (3), `Abort` (2), `Complete` (2), `Describe` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddTagsToVault`, `CreateVault`, `DeleteArchive`, `DeleteVault`, `DeleteVaultAccessPolicy`, `DeleteVaultNotifications`, `RemoveTagsFromVault`, `SetDataRetrievalPolicy`, `SetVaultAccessPolicy`, `SetVaultNotifications`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeJob`, `DescribeVault`, `GetDataRetrievalPolicy`, `GetJobOutput`, `GetVaultAccessPolicy`, `GetVaultLock`, `GetVaultNotifications`, `ListJobs`, `ListMultipartUploads`, `ListParts`, `ListProvisionedCapacity`, `ListTagsForVault`, `ListVaults`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeJob`, `GetJobOutput`, `InitiateJob`, `ListJobs`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 33 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SNS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/amazonglacier/latest/dev/introduction.html
- https://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-vaults.html
- https://docs.aws.amazon.com/amazonglacier/latest/dev/working-with-archives.html
- https://docs.aws.amazon.com/amazonglacier/latest/dev/downloading-an-archive-two-steps.html

Research outcomes:
- The original Amazon Glacier service stores archives in vaults and exposes vault, archive, multipart upload, job, notification, and inventory operations.
- Archives are immutable data objects stored in vaults. Clients must maintain archive metadata such as archive ids and descriptions.
- Archive retrieval is asynchronous. A retrieval job is initiated, optionally sends SNS notification, and later output is downloaded with Get Job Output.
- Retrieval options include expedited, standard, and bulk where available.
- Vault inventory retrieval is also asynchronous and uses jobs.
- Vault notifications can be configured to publish job-completion events to SNS.
- Multipart upload is used for large archives and has its own initiation, part upload, completion, and abort lifecycle.
- AWS documentation encourages migration from Glacier vault APIs to Amazon S3 Glacier storage classes for many use cases, but the vault API remains behaviourally distinct.

Parity implications:
- Model vaults, archives, multipart uploads, jobs, job output, inventory snapshots, notification configuration, and retrieval tiers separately.
- DownloadArchive and inventory retrieval should require completed jobs rather than direct synchronous access.
- Archive metadata should not be discoverable unless stored by the client or through delayed inventory jobs.

## Operation Groups

### List

- Operations: `ListJobs`, `ListMultipartUploads`, `ListParts`, `ListProvisionedCapacity`, `ListTagsForVault`, `ListVaults`
- Traits: `paginated` (4)
- Common required input members in this group: `accountId`, `vaultName`

### Get

- Operations: `GetDataRetrievalPolicy`, `GetJobOutput`, `GetVaultAccessPolicy`, `GetVaultLock`, `GetVaultNotifications`
- Common required input members in this group: `accountId`, `vaultName`

### Delete

- Operations: `DeleteArchive`, `DeleteVault`, `DeleteVaultAccessPolicy`, `DeleteVaultNotifications`
- Common required input members in this group: `accountId`, `vaultName`

### Initiate

- Operations: `InitiateJob`, `InitiateMultipartUpload`, `InitiateVaultLock`
- Common required input members in this group: `accountId`, `vaultName`

### Set

- Operations: `SetDataRetrievalPolicy`, `SetVaultAccessPolicy`, `SetVaultNotifications`
- Common required input members in this group: `accountId`, `vaultName`

### Abort

- Operations: `AbortMultipartUpload`, `AbortVaultLock`
- Common required input members in this group: `accountId`, `vaultName`

### Complete

- Operations: `CompleteMultipartUpload`, `CompleteVaultLock`
- Common required input members in this group: `accountId`, `vaultName`

### Describe

- Operations: `DescribeJob`, `DescribeVault`
- Common required input members in this group: `accountId`, `vaultName`

### Upload

- Operations: `UploadArchive`, `UploadMultipartPart`
- Common required input members in this group: `vaultName`, `accountId`

### Add

- Operations: `AddTagsToVault`
- Common required input members in this group: -

### Create

- Operations: `CreateVault`
- Common required input members in this group: -

### Purchase

- Operations: `PurchaseProvisionedCapacity`
- Common required input members in this group: -

### Remove

- Operations: `RemoveTagsFromVault`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AbortMultipartUpload` | `DELETE /{accountId}/vaults/{vaultName}/multipart-uploads/{uploadId}` | - | `accountId`, `vaultName`, `uploadId` | - | `Unit` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation aborts a multipart upload identified by the upload ID. After the Abort Multipart Upload request succeeds, you cannot upload any more parts to the multipart upload or complete the multipart upload. Abor ... |
| `AbortVaultLock` | `DELETE /{accountId}/vaults/{vaultName}/lock-policy` | - | `accountId`, `vaultName` | - | `Unit` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation aborts the vault locking process if the vault lock is not in the Locked state. If the vault lock is in the Locked state when this operation is requested, the operation returns an AccessDeniedException ... |
| `AddTagsToVault` | `POST /{accountId}/vaults/{vaultName}/tags?operation=add` | - | `accountId`, `vaultName` | - | `Unit` | `InvalidParameterValueException`, `LimitExceededException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation adds the specified tags to a vault. Each tag is composed of a key and a value. Each vault can have up to 10 tags. If your request would cause the tag limit for the vault to be exceeded, the operation t ... |
| `CompleteMultipartUpload` | `POST /{accountId}/vaults/{vaultName}/multipart-uploads/{uploadId}` | - | `accountId`, `vaultName`, `uploadId` | - | `ArchiveCreationOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | You call this operation to inform Amazon Glacier (Glacier) that all the archive parts have been uploaded and that Glacier can now assemble the archive from the uploaded parts. After assembling and saving the archive ... |
| `CompleteVaultLock` | `POST /{accountId}/vaults/{vaultName}/lock-policy/{lockId}` | - | `accountId`, `vaultName`, `lockId` | - | `Unit` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation completes the vault locking process by transitioning the vault lock from the InProgress state to the Locked state, which causes the vault lock policy to become unchangeable. A vault lock is put into th ... |
| `CreateVault` | `PUT /{accountId}/vaults/{vaultName}` | - | `accountId`, `vaultName` | - | `CreateVaultOutput` | `InvalidParameterValueException`, `LimitExceededException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ServiceUnavailableException` | This operation creates a new vault with the specified name. The name of the vault must be unique within a region for an AWS account. You can create up to 1,000 vaults per account. If you need to create more vaults, c ... |
| `DeleteArchive` | `DELETE /{accountId}/vaults/{vaultName}/archives/{archiveId}` | - | `accountId`, `vaultName`, `archiveId` | - | `Unit` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation deletes an archive from a vault. Subsequent requests to initiate a retrieval of this archive will fail. Archive retrievals that are in progress for this archive ID may or may not succeed according to t ... |
| `DeleteVault` | `DELETE /{accountId}/vaults/{vaultName}` | - | `accountId`, `vaultName` | - | `Unit` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation deletes a vault. Amazon Glacier will delete a vault only if there are no archives in the vault as of the last inventory and there have been no writes to the vault since the last inventory. If either of ... |
| `DeleteVaultAccessPolicy` | `DELETE /{accountId}/vaults/{vaultName}/access-policy` | - | `accountId`, `vaultName` | - | `Unit` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation deletes the access policy associated with the specified vault. The operation is eventually consistent; that is, it might take some time for Amazon Glacier to completely remove the access policy, and yo ... |
| `DeleteVaultNotifications` | `DELETE /{accountId}/vaults/{vaultName}/notification-configuration` | - | `accountId`, `vaultName` | - | `Unit` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation deletes the notification configuration set for a vault. The operation is eventually consistent; that is, it might take some time for Amazon Glacier to completely disable the notifications and you might ... |
| `DescribeJob` | `GET /{accountId}/vaults/{vaultName}/jobs/{jobId}` | - | `accountId`, `vaultName`, `jobId` | - | `GlacierJobDescription` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation returns information about a job you previously initiated, including the job initiation date, the user who initiated the job, the job status code/message and the Amazon SNS topic to notify after Amazon ... |
| `DescribeVault` | `GET /{accountId}/vaults/{vaultName}` | - | `accountId`, `vaultName` | - | `DescribeVaultOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation returns information about a vault, including the vault's Amazon Resource Name (ARN), the date the vault was created, the number of archives it contains, and the total size of all the archives in the va ... |
| `GetDataRetrievalPolicy` | `GET /{accountId}/policies/data-retrieval` | - | `accountId` | - | `GetDataRetrievalPolicyOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ServiceUnavailableException` | This operation returns the current data retrieval policy for the account and region specified in the GET request. For more information about data retrieval policies, see Amazon Glacier Data Retrieval Policies . |
| `GetJobOutput` | `GET /{accountId}/vaults/{vaultName}/jobs/{jobId}/output` | - | `accountId`, `vaultName`, `jobId` | - | `GetJobOutputOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation downloads the output of the job you initiated using InitiateJob . Depending on the job type you specified when you initiated the job, the output will be either the content of an archive or a vault inve ... |
| `GetVaultAccessPolicy` | `GET /{accountId}/vaults/{vaultName}/access-policy` | - | `accountId`, `vaultName` | - | `GetVaultAccessPolicyOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation retrieves the access-policy subresource set on the vault; for more information on setting this subresource, see Set Vault Access Policy (PUT access-policy) . If there is no access policy set on the vau ... |
| `GetVaultLock` | `GET /{accountId}/vaults/{vaultName}/lock-policy` | - | `accountId`, `vaultName` | - | `GetVaultLockOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation retrieves the following attributes from the lock-policy subresource set on the specified vault: The vault lock policy set on the vault. The state of the vault lock, which is either InProgess or Locked ... |
| `GetVaultNotifications` | `GET /{accountId}/vaults/{vaultName}/notification-configuration` | - | `accountId`, `vaultName` | - | `GetVaultNotificationsOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation retrieves the notification-configuration subresource of the specified vault. For information about setting a notification configuration on a vault, see SetVaultNotifications . If a notification configu ... |
| `InitiateJob` | `POST /{accountId}/vaults/{vaultName}/jobs` | - | `accountId`, `vaultName` | - | `InitiateJobOutput` | `InsufficientCapacityException`, `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `PolicyEnforcedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation initiates a job of the specified type, which can be a select, an archival retrieval, or a vault retrieval. For more information about using this operation, see the documentation for the underlying REST ... |
| `InitiateMultipartUpload` | `POST /{accountId}/vaults/{vaultName}/multipart-uploads` | - | `accountId`, `vaultName` | - | `InitiateMultipartUploadOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation initiates a multipart upload. Amazon Glacier creates a multipart upload resource and returns its ID in the response. The multipart upload ID is used in subsequent requests to upload parts of an archive ... |
| `InitiateVaultLock` | `POST /{accountId}/vaults/{vaultName}/lock-policy` | - | `accountId`, `vaultName` | - | `InitiateVaultLockOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation initiates the vault locking process by doing the following: Installing a vault lock policy on the specified vault. Setting the lock state of vault lock to InProgress . Returning a lock ID, which is use ... |
| `ListJobs` | `GET /{accountId}/vaults/{vaultName}/jobs` | `paginated` | `accountId`, `vaultName` | - | `ListJobsOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation lists jobs for a vault, including jobs that are in-progress and jobs that have recently finished. The List Job operation returns a list of these jobs sorted by job initiation time. Amazon Glacier retai ... |
| `ListMultipartUploads` | `GET /{accountId}/vaults/{vaultName}/multipart-uploads` | `paginated` | `accountId`, `vaultName` | - | `ListMultipartUploadsOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation lists in-progress multipart uploads for the specified vault. An in-progress multipart upload is a multipart upload that has been initiated by an InitiateMultipartUpload request, but has not yet been co ... |
| `ListParts` | `GET /{accountId}/vaults/{vaultName}/multipart-uploads/{uploadId}` | `paginated` | `accountId`, `vaultName`, `uploadId` | - | `ListPartsOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation lists the parts of an archive that have been uploaded in a specific multipart upload. You can make this request at any time during an in-progress multipart upload before you complete the upload (see Co ... |
| `ListProvisionedCapacity` | `GET /{accountId}/provisioned-capacity` | - | `accountId` | - | `ListProvisionedCapacityOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ServiceUnavailableException` | This operation lists the provisioned capacity units for the specified AWS account. |
| `ListTagsForVault` | `GET /{accountId}/vaults/{vaultName}/tags` | - | `accountId`, `vaultName` | - | `ListTagsForVaultOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation lists all the tags attached to a vault. The operation returns an empty map if there are no tags. For more information about tags, see Tagging Amazon Glacier Resources . |
| `ListVaults` | `GET /{accountId}/vaults` | `paginated` | `accountId` | - | `ListVaultsOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation lists all vaults owned by the calling user's account. The list returned in the response is ASCII-sorted by vault name. By default, this operation returns up to 10 items. If there are more vaults to lis ... |
| `PurchaseProvisionedCapacity` | `POST /{accountId}/provisioned-capacity` | - | `accountId` | - | `PurchaseProvisionedCapacityOutput` | `InvalidParameterValueException`, `LimitExceededException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ServiceUnavailableException` | This operation purchases a provisioned capacity unit for an AWS account. |
| `RemoveTagsFromVault` | `POST /{accountId}/vaults/{vaultName}/tags?operation=remove` | - | `accountId`, `vaultName` | - | `Unit` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation removes one or more tags from the set of tags attached to a vault. For more information about tags, see Tagging Amazon Glacier Resources . This operation is idempotent. The operation will be successful ... |
| `SetDataRetrievalPolicy` | `PUT /{accountId}/policies/data-retrieval` | - | `accountId` | - | `Unit` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ServiceUnavailableException` | This operation sets and then enacts a data retrieval policy in the region specified in the PUT request. You can set one policy per region for an AWS account. The policy is enacted within a few minutes of a successful ... |
| `SetVaultAccessPolicy` | `PUT /{accountId}/vaults/{vaultName}/access-policy` | - | `accountId`, `vaultName` | - | `Unit` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation configures an access policy for a vault and will overwrite an existing policy. To configure a vault access policy, send a PUT request to the access-policy subresource of the vault. An access policy is ... |
| `SetVaultNotifications` | `PUT /{accountId}/vaults/{vaultName}/notification-configuration` | - | `accountId`, `vaultName` | - | `Unit` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation configures notifications that will be sent when specific events happen to a vault. By default, you don't get any notifications. To configure vault notifications, send a PUT request to the notification- ... |
| `UploadArchive` | `POST /{accountId}/vaults/{vaultName}/archives` | - | `vaultName`, `accountId` | - | `ArchiveCreationOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `RequestTimeoutException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation adds an archive to a vault. This is a synchronous operation, and for a successful upload, your data is durably persisted. Amazon Glacier returns the archive ID in the x-amz-archive-id header of the res ... |
| `UploadMultipartPart` | `PUT /{accountId}/vaults/{vaultName}/multipart-uploads/{uploadId}` | - | `accountId`, `vaultName`, `uploadId` | - | `UploadMultipartPartOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `NoLongerSupportedException`, `RequestTimeoutException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation uploads a part of an archive. You can upload archive parts in any order. You can also upload them in parallel. You can upload up to 10,000 parts for a multipart upload. Amazon Glacier rejects your uplo ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `CompleteMultipartUpload` | `archiveSize -> x-amz-archive-size`, `checksum -> x-amz-sha256-tree-hash` | - | - | - |
| `GetJobOutput` | `range -> Range` | - | - | - |
| `InitiateJob` | - | - | - | `jobParameters` |
| `InitiateMultipartUpload` | `archiveDescription -> x-amz-archive-description`, `partSize -> x-amz-part-size` | - | - | - |
| `InitiateVaultLock` | - | - | - | `policy` |
| `ListJobs` | - | `limit -> limit`, `marker -> marker`, `statuscode -> statuscode`, `completed -> completed` | - | - |
| `ListMultipartUploads` | - | `limit -> limit`, `marker -> marker` | - | - |
| `ListParts` | - | `marker -> marker`, `limit -> limit` | - | - |
| `ListVaults` | - | `marker -> marker`, `limit -> limit` | - | - |
| `SetVaultAccessPolicy` | - | - | - | `policy` |
| `SetVaultNotifications` | - | - | - | `vaultNotificationConfig` |
| `UploadArchive` | `archiveDescription -> x-amz-archive-description`, `checksum -> x-amz-sha256-tree-hash` | - | - | `body` |
| `UploadMultipartPart` | `checksum -> x-amz-sha256-tree-hash`, `range -> Content-Range` | - | - | `body` |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InsufficientCapacityException` | `structure` | type, code, message | Returned if there is insufficient capacity to process this expedited request. This error only applies to expedited retrievals and not to standard or bulk re ... |
| `InvalidParameterValueException` | `structure` | type, code, message | Returned if a parameter of the request is incorrectly specified. |
| `LimitExceededException` | `structure` | type, code, message | Returned if the request results in a vault or account limit being exceeded. |
| `MissingParameterValueException` | `structure` | type, code, message | Returned if a required header or parameter is missing from the request. |
| `NoLongerSupportedException` | `structure` | type, code, message | Returned if the request was made by a customer with no Amazon Glacier storage. The request is denied as the API is no longer supported for new customers. Pl ... |
| `PolicyEnforcedException` | `structure` | type, code, message | Returned if a retrieval job would exceed the current data policy's retrieval rate limit. For more information about data retrieval policies, |
| `RequestTimeoutException` | `structure` | type, code, message | Returned if, when uploading an archive, Amazon Glacier times out while receiving the upload. |
| `ResourceNotFoundException` | `structure` | type, code, message | Returned if the specified resource (such as a vault, upload ID, or job ID) doesn't exist. |
| `ServiceUnavailableException` | `structure` | type, code, message | Returned if the service cannot complete the request. |
| `AbortMultipartUploadInput` | `structure` | accountId, vaultName, uploadId | Provides options to abort a multipart upload identified by the upload ID. For information about the underlying REST API, see Abort Multipart Upload . For co ... |
| `AbortVaultLockInput` | `structure` | accountId, vaultName | The input values for AbortVaultLock . |
| `AddTagsToVaultInput` | `structure` | accountId, vaultName, Tags | The input values for AddTagsToVault . |
| `CompleteMultipartUploadInput` | `structure` | accountId, vaultName, uploadId, archiveSize, checksum | Provides options to complete a multipart upload operation. This informs Amazon Glacier that all the archive parts have been uploaded and Amazon Glacier (Gla ... |
| `ArchiveCreationOutput` | `structure` | location, checksum, archiveId | Contains the Amazon Glacier response to your request. For information about the underlying REST API, see Upload Archive . For conceptual information, see Wo ... |
| `CompleteVaultLockInput` | `structure` | accountId, vaultName, lockId | The input values for CompleteVaultLock . |
| `CreateVaultInput` | `structure` | accountId, vaultName | Provides options to create a vault. |
| `CreateVaultOutput` | `structure` | location | Contains the Amazon Glacier response to your request. |
| `DeleteArchiveInput` | `structure` | accountId, vaultName, archiveId | Provides options for deleting an archive from an Amazon Glacier vault. |
| `DeleteVaultInput` | `structure` | accountId, vaultName | Provides options for deleting a vault from Amazon Glacier. |
| `DeleteVaultAccessPolicyInput` | `structure` | accountId, vaultName | DeleteVaultAccessPolicy input. |
| `DeleteVaultNotificationsInput` | `structure` | accountId, vaultName | Provides options for deleting a vault notification configuration from an Amazon Glacier vault. |
| `DescribeJobInput` | `structure` | accountId, vaultName, jobId | Provides options for retrieving a job description. |
| `GlacierJobDescription` | `structure` | JobId, JobDescription, Action, ArchiveId, VaultARN, CreationDate, Completed, StatusCode, StatusMessage, ArchiveSizeInBytes, InventorySizeInBytes, SNSTopic, ... (+9) | Contains the description of an Amazon S3 Glacier job. |
| `DescribeVaultInput` | `structure` | accountId, vaultName | Provides options for retrieving metadata for a specific vault in Amazon Glacier. |
| `DescribeVaultOutput` | `structure` | VaultARN, VaultName, CreationDate, LastInventoryDate, NumberOfArchives, SizeInBytes | Contains the Amazon Glacier response to your request. |
| `GetDataRetrievalPolicyInput` | `structure` | accountId | Input for GetDataRetrievalPolicy. |
| `GetDataRetrievalPolicyOutput` | `structure` | Policy | Contains the Amazon Glacier response to the GetDataRetrievalPolicy request. |
| `GetJobOutputInput` | `structure` | accountId, vaultName, jobId, range | Provides options for downloading output of an Amazon Glacier job. |
| `GetJobOutputOutput` | `structure` | body, checksum, status, contentRange, acceptRanges, contentType, archiveDescription | Contains the Amazon Glacier response to your request. |
| `GetVaultAccessPolicyInput` | `structure` | accountId, vaultName | Input for GetVaultAccessPolicy. |
| `GetVaultAccessPolicyOutput` | `structure` | policy | Output for GetVaultAccessPolicy. |
| `GetVaultLockInput` | `structure` | accountId, vaultName | The input values for GetVaultLock . |
| `GetVaultLockOutput` | `structure` | Policy, State, ExpirationDate, CreationDate | Contains the Amazon Glacier response to your request. |
| `GetVaultNotificationsInput` | `structure` | accountId, vaultName | Provides options for retrieving the notification configuration set on an Amazon Glacier vault. |
| `GetVaultNotificationsOutput` | `structure` | vaultNotificationConfig | Contains the Amazon Glacier response to your request. |
| `InitiateJobInput` | `structure` | accountId, vaultName, jobParameters | Provides options for initiating an Amazon Glacier job. |
| `InitiateJobOutput` | `structure` | location, jobId, jobOutputPath | Contains the Amazon Glacier response to your request. |
| `InitiateMultipartUploadInput` | `structure` | accountId, vaultName, archiveDescription, partSize | Provides options for initiating a multipart upload to an Amazon Glacier vault. |
| `InitiateMultipartUploadOutput` | `structure` | location, uploadId | The Amazon Glacier response to your request. |
| `InitiateVaultLockInput` | `structure` | accountId, vaultName, policy | The input values for InitiateVaultLock . |
| `ActionCode` | `enum` | ArchiveRetrieval, InventoryRetrieval, Select | - |
| `CannedACL` | `enum` | Private, PublicRead, PublicReadWrite, AwsExecRead, AuthenticatedRead, BucketOwnerRead, BucketOwnerFullControl | - |
| `EncryptionType` | `enum` | KMS, S3 | - |
| `ExpressionType` | `enum` | SQL | - |
| `FileHeaderInfo` | `enum` | Use, Ignore, None | - |
| `Permission` | `enum` | FULL_CONTROL, WRITE, WRITE_ACP, READ, READ_ACP | - |
| `QuoteFields` | `enum` | Always, AsNeeded | - |
| `StatusCode` | `enum` | InProgress, Succeeded, Failed | - |
| `StorageClass` | `enum` | Standard, ReducedRedundancy, StandardInfrequentAccess | - |
| `Type` | `enum` | AmazonCustomerByEmail, CanonicalUser, Group | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
