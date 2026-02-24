# Amazon WorkDocs

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Amazon WorkDocs API is designed for the following use cases: File Migration: File migration applications are supported for users who want to migrate their files from an on-premises or off-premises file system or service. Users can insert files into a user directory structure, as well as allow for basic metadata changes, such as modifications to the permissions of files. Security: Support security applications are supported for users who have additional security needs, such as antivirus or data loss prevention. The API actions, along with CloudTrail, allow these applications to detect when changes occur in Amazon WorkDocs. Then, the application can take the necessary actions and replace the target file.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon WorkDocs where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- From the AWS documentation and model: represent documented Amazon WorkDocs workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Delete`, `Describe`, `Get`, `Create`, `Update` operation families, including `DeleteComment`, `DeleteCustomMetadata`, `DeleteDocument`, `DeleteDocumentVersion`, `DescribeActivities`, `DescribeComments`.

## Service Identity and Protocol

- AWS model slug: `workdocs`
- AWS SDK for Rust slug: `workdocs`
- Model version: `2016-05-01`
- Model file: `vendor/api-models-aws/models/workdocs/service/2016-05-01/workdocs-2016-05-01.json`
- SDK ID: `WorkDocs`
- Endpoint prefix: `workdocs`
- ARN namespace: `workdocs`
- CloudFormation name: `WorkDocs`
- CloudTrail event source: `workdocs.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (9), `Describe` (9), `Get` (7), `Create` (6), `Update` (4), `Remove` (2), `Abort` (1), `Activate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddResourcePermissions`, `CreateComment`, `CreateCustomMetadata`, `CreateFolder`, `CreateLabels`, `CreateNotificationSubscription`, `CreateUser`, `DeleteComment`, `DeleteCustomMetadata`, `DeleteDocument`, `DeleteDocumentVersion`, `DeleteFolder`, `DeleteFolderContents`, `DeleteLabels`, `DeleteNotificationSubscription`, `DeleteUser`, `RemoveAllResourcePermissions`, `RemoveResourcePermission`, `RestoreDocumentVersions`, `UpdateDocument`, ... (+3).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeActivities`, `DescribeComments`, `DescribeDocumentVersions`, `DescribeFolderContents`, `DescribeGroups`, `DescribeNotificationSubscriptions`, `DescribeResourcePermissions`, `DescribeRootFolders`, `DescribeUsers`, `GetCurrentUser`, `GetDocument`, `GetDocumentPath`, `GetDocumentVersion`, `GetFolder`, `GetFolderPath`, `GetResources`, `SearchResources`.
- Pagination is modelled for 10 operations; token stability and page boundaries are observable API behaviour.
- 44 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SNS`, `EC2/VPC`.

## Operation Groups

### Delete

- Operations: `DeleteComment`, `DeleteCustomMetadata`, `DeleteDocument`, `DeleteDocumentVersion`, `DeleteFolder`, `DeleteFolderContents`, `DeleteLabels`, `DeleteNotificationSubscription`, `DeleteUser`
- Common required input members in this group: `CommentId`, `DeletePriorVersions`, `DocumentId`, `FolderId`, `OrganizationId`, `ResourceId`, `SubscriptionId`, `UserId`, `VersionId`

### Describe

- Operations: `DescribeActivities`, `DescribeComments`, `DescribeDocumentVersions`, `DescribeFolderContents`, `DescribeGroups`, `DescribeNotificationSubscriptions`, `DescribeResourcePermissions`, `DescribeRootFolders`, `DescribeUsers`
- Traits: `paginated` (9)
- Common required input members in this group: `AuthenticationToken`, `DocumentId`, `FolderId`, `OrganizationId`, `ResourceId`, `SearchQuery`, `VersionId`

### Get

- Operations: `GetCurrentUser`, `GetDocument`, `GetDocumentPath`, `GetDocumentVersion`, `GetFolder`, `GetFolderPath`, `GetResources`
- Common required input members in this group: `AuthenticationToken`, `DocumentId`, `FolderId`, `VersionId`

### Create

- Operations: `CreateComment`, `CreateCustomMetadata`, `CreateFolder`, `CreateLabels`, `CreateNotificationSubscription`, `CreateUser`
- Common required input members in this group: `CustomMetadata`, `DocumentId`, `Endpoint`, `GivenName`, `Labels`, `OrganizationId`, `ParentFolderId`, `Password`, `Protocol`, `ResourceId`, `SubscriptionType`, `Surname`, `Text`, `Username`, `VersionId`

### Update

- Operations: `UpdateDocument`, `UpdateDocumentVersion`, `UpdateFolder`, `UpdateUser`
- Common required input members in this group: `DocumentId`, `FolderId`, `UserId`, `VersionId`

### Remove

- Operations: `RemoveAllResourcePermissions`, `RemoveResourcePermission`
- Common required input members in this group: `PrincipalId`, `ResourceId`

### Abort

- Operations: `AbortDocumentVersionUpload`
- Common required input members in this group: `DocumentId`, `VersionId`

### Activate

- Operations: `ActivateUser`
- Common required input members in this group: `UserId`

### Add

- Operations: `AddResourcePermissions`
- Common required input members in this group: `Principals`, `ResourceId`

### Deactivate

- Operations: `DeactivateUser`
- Common required input members in this group: `UserId`

### Initiate

- Operations: `InitiateDocumentVersionUpload`

### Restore

- Operations: `RestoreDocumentVersions`
- Common required input members in this group: `DocumentId`

### Search

- Operations: `SearchResources`
- Traits: `paginated` (1)

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AbortDocumentVersionUpload` | `DELETE /api/v1/documents/{DocumentId}/versions/{VersionId}` | - | `DocumentId`, `VersionId` | - | `Unit` | `ConcurrentModificationException`, `EntityNotExistsException`, `FailedDependencyException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Aborts the upload of the specified document version that was previously initiated by InitiateDocumentVersionUpload. The client should make this call only when it no longer intends to upload the document version, or fails to do so. |
| `ActivateUser` | `POST /api/v1/users/{UserId}/activation` | - | `UserId` | - | `ActivateUserResponse` | `EntityNotExistsException`, `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Activates the specified user. Only active users can access Amazon WorkDocs. |
| `AddResourcePermissions` | `POST /api/v1/resources/{ResourceId}/permissions` | - | `Principals`, `ResourceId` | - | `AddResourcePermissionsResponse` | `FailedDependencyException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Creates a set of permissions for the specified folder or document. The resource permissions are overwritten if the principals already have different permissions. |
| `CreateComment` | `POST /api/v1/documents/{DocumentId}/versions/{VersionId}/comment` | - | `DocumentId`, `Text`, `VersionId` | - | `CreateCommentResponse` | `DocumentLockedForCommentsException`, `EntityNotExistsException`, `FailedDependencyException`, `InvalidCommentOperationException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Adds a new comment to the specified document version. |
| `CreateCustomMetadata` | `PUT /api/v1/resources/{ResourceId}/customMetadata` | - | `CustomMetadata`, `ResourceId` | - | `CreateCustomMetadataResponse` | `CustomMetadataLimitExceededException`, `EntityNotExistsException`, `FailedDependencyException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Adds one or more custom properties to the specified resource (a folder, document, or version). |
| `CreateFolder` | `POST /api/v1/folders` | - | `ParentFolderId` | - | `CreateFolderResponse` | `ConcurrentModificationException`, `ConflictingOperationException`, `EntityAlreadyExistsException`, `EntityNotExistsException`, `FailedDependencyException`, `LimitExceededException`, `ProhibitedStateException`, `ServiceUnavailableException`, ... (+2) | Creates a folder with the specified name and parent folder. |
| `CreateLabels` | `PUT /api/v1/resources/{ResourceId}/labels` | - | `Labels`, `ResourceId` | - | `CreateLabelsResponse` | `EntityNotExistsException`, `FailedDependencyException`, `ServiceUnavailableException`, `TooManyLabelsException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Adds the specified list of labels to the given resource (a document or folder) |
| `CreateNotificationSubscription` | `POST /api/v1/organizations/{OrganizationId}/subscriptions` | - | `Endpoint`, `OrganizationId`, `Protocol`, `SubscriptionType` | - | `CreateNotificationSubscriptionResponse` | `InvalidArgumentException`, `ServiceUnavailableException`, `TooManySubscriptionsException`, `UnauthorizedResourceAccessException` | Configure Amazon WorkDocs to use Amazon SNS notifications. The endpoint receives a confirmation message, and must confirm the subscription. |
| `CreateUser` | `POST /api/v1/users` | - | `GivenName`, `Password`, `Surname`, `Username` | - | `CreateUserResponse` | `EntityAlreadyExistsException`, `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Creates a user in a Simple AD or Microsoft AD directory. The status of a newly created user is "ACTIVE". |
| `DeactivateUser` | `DELETE /api/v1/users/{UserId}/activation` | - | `UserId` | - | `Unit` | `EntityNotExistsException`, `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Deactivates the specified user, which revokes the user's access to Amazon WorkDocs. |
| `DeleteComment` | `DELETE /api/v1/documents/{DocumentId}/versions/{VersionId}/comment/{CommentId}` | - | `CommentId`, `DocumentId`, `VersionId` | - | `Unit` | `DocumentLockedForCommentsException`, `EntityNotExistsException`, `FailedDependencyException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Deletes the specified comment from the document version. |
| `DeleteCustomMetadata` | `DELETE /api/v1/resources/{ResourceId}/customMetadata` | - | `ResourceId` | - | `DeleteCustomMetadataResponse` | `EntityNotExistsException`, `FailedDependencyException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Deletes custom metadata from the specified resource. |
| `DeleteDocument` | `DELETE /api/v1/documents/{DocumentId}` | - | `DocumentId` | - | `Unit` | `ConcurrentModificationException`, `ConflictingOperationException`, `EntityNotExistsException`, `FailedDependencyException`, `LimitExceededException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, ... (+1) | Permanently deletes the specified document and its associated metadata. |
| `DeleteDocumentVersion` | `DELETE /api/v1/documentVersions/{DocumentId}/versions/{VersionId}` | - | `DeletePriorVersions`, `DocumentId`, `VersionId` | - | `Unit` | `ConcurrentModificationException`, `ConflictingOperationException`, `EntityNotExistsException`, `FailedDependencyException`, `InvalidOperationException`, `ProhibitedStateException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Deletes a specific version of a document. |
| `DeleteFolder` | `DELETE /api/v1/folders/{FolderId}` | - | `FolderId` | - | `Unit` | `ConcurrentModificationException`, `ConflictingOperationException`, `EntityNotExistsException`, `FailedDependencyException`, `LimitExceededException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, ... (+1) | Permanently deletes the specified folder and its contents. |
| `DeleteFolderContents` | `DELETE /api/v1/folders/{FolderId}/contents` | - | `FolderId` | - | `Unit` | `ConflictingOperationException`, `EntityNotExistsException`, `FailedDependencyException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Deletes the contents of the specified folder. |
| `DeleteLabels` | `DELETE /api/v1/resources/{ResourceId}/labels` | - | `ResourceId` | - | `DeleteLabelsResponse` | `EntityNotExistsException`, `FailedDependencyException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Deletes the specified list of labels from a resource. |
| `DeleteNotificationSubscription` | `DELETE /api/v1/organizations/{OrganizationId}/subscriptions/{SubscriptionId}` | - | `OrganizationId`, `SubscriptionId` | - | `Unit` | `EntityNotExistsException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedResourceAccessException` | Deletes the specified subscription from the specified organization. |
| `DeleteUser` | `DELETE /api/v1/users/{UserId}` | - | `UserId` | - | `Unit` | `EntityNotExistsException`, `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Deletes the specified user from a Simple AD or Microsoft AD directory. Deleting a user immediately and permanently deletes all content in that user's folder structure. |
| `DescribeActivities` | `GET /api/v1/activities` | `paginated` | - | - | `DescribeActivitiesResponse` | `FailedDependencyException`, `InvalidArgumentException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Describes the user activities in a specified time period. |
| `DescribeComments` | `GET /api/v1/documents/{DocumentId}/versions/{VersionId}/comments` | `paginated` | `DocumentId`, `VersionId` | - | `DescribeCommentsResponse` | `EntityNotExistsException`, `FailedDependencyException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | List all the comments for the specified document version. |
| `DescribeDocumentVersions` | `GET /api/v1/documents/{DocumentId}/versions` | `paginated` | `DocumentId` | - | `DescribeDocumentVersionsResponse` | `EntityNotExistsException`, `FailedDependencyException`, `InvalidArgumentException`, `InvalidPasswordException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Retrieves the document versions for the specified document. By default, only active versions are returned. |
| `DescribeFolderContents` | `GET /api/v1/folders/{FolderId}/contents` | `paginated` | `FolderId` | - | `DescribeFolderContentsResponse` | `EntityNotExistsException`, `FailedDependencyException`, `InvalidArgumentException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedResourceAccessException` | Describes the contents of the specified folder, including its documents and subfolders. By default, Amazon WorkDocs returns the first 100 active document and folder metadata items. |
| `DescribeGroups` | `GET /api/v1/groups` | `paginated` | `SearchQuery` | - | `DescribeGroupsResponse` | `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Describes the groups specified by the query. Groups are defined by the underlying Active Directory. |
| `DescribeNotificationSubscriptions` | `GET /api/v1/organizations/{OrganizationId}/subscriptions` | `paginated` | `OrganizationId` | - | `DescribeNotificationSubscriptionsResponse` | `EntityNotExistsException`, `ServiceUnavailableException`, `UnauthorizedResourceAccessException` | Lists the specified notification subscriptions. |
| `DescribeResourcePermissions` | `GET /api/v1/resources/{ResourceId}/permissions` | `paginated` | `ResourceId` | - | `DescribeResourcePermissionsResponse` | `FailedDependencyException`, `InvalidArgumentException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Describes the permissions of a specified resource. |
| `DescribeRootFolders` | `GET /api/v1/me/root` | `paginated` | `AuthenticationToken` | - | `DescribeRootFoldersResponse` | `FailedDependencyException`, `InvalidArgumentException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Describes the current user's special folders; the `RootFolder` and the `RecycleBin`. `RootFolder` is the root of user's files and folders and `RecycleBin` is the root of recycled items. |
| `DescribeUsers` | `GET /api/v1/users` | `paginated` | - | - | `DescribeUsersResponse` | `EntityNotExistsException`, `FailedDependencyException`, `InvalidArgumentException`, `RequestedEntityTooLargeException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Describes the specified users. You can describe all users or filter the results (for example, by status or organization). |
| `GetCurrentUser` | `GET /api/v1/me` | - | `AuthenticationToken` | - | `GetCurrentUserResponse` | `EntityNotExistsException`, `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Retrieves details of the current user for whom the authentication token was generated. This is not a valid action for SigV4 (administrative API) clients. |
| `GetDocument` | `GET /api/v1/documents/{DocumentId}` | - | `DocumentId` | - | `GetDocumentResponse` | `EntityNotExistsException`, `FailedDependencyException`, `InvalidArgumentException`, `InvalidPasswordException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Retrieves details of a document. |
| `GetDocumentPath` | `GET /api/v1/documents/{DocumentId}/path` | - | `DocumentId` | - | `GetDocumentPathResponse` | `EntityNotExistsException`, `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Retrieves the path information (the hierarchy from the root folder) for the requested document. By default, Amazon WorkDocs returns a maximum of 100 levels upwards from the requested document and only includes the IDs of the parent folders in the path. |
| `GetDocumentVersion` | `GET /api/v1/documents/{DocumentId}/versions/{VersionId}` | - | `DocumentId`, `VersionId` | - | `GetDocumentVersionResponse` | `EntityNotExistsException`, `FailedDependencyException`, `InvalidPasswordException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Retrieves version metadata for the specified document. |
| `GetFolder` | `GET /api/v1/folders/{FolderId}` | - | `FolderId` | - | `GetFolderResponse` | `EntityNotExistsException`, `FailedDependencyException`, `InvalidArgumentException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Retrieves the metadata of the specified folder. |
| `GetFolderPath` | `GET /api/v1/folders/{FolderId}/path` | - | `FolderId` | - | `GetFolderPathResponse` | `EntityNotExistsException`, `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Retrieves the path information (the hierarchy from the root folder) for the specified folder. By default, Amazon WorkDocs returns a maximum of 100 levels upwards from the requested folder and only includes the IDs of the parent folders in the path. |
| `GetResources` | `GET /api/v1/resources` | - | - | - | `GetResourcesResponse` | `FailedDependencyException`, `InvalidArgumentException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Retrieves a collection of resources, including folders and documents. The only `CollectionType` supported is `SHARED_WITH_ME`. |
| `InitiateDocumentVersionUpload` | `POST /api/v1/documents` | - | - | - | `InitiateDocumentVersionUploadResponse` | `DraftUploadOutOfSyncException`, `EntityAlreadyExistsException`, `EntityNotExistsException`, `FailedDependencyException`, `InvalidArgumentException`, `InvalidPasswordException`, `LimitExceededException`, `ProhibitedStateException`, ... (+6) | Creates a new document object and version object. The client specifies the parent folder ID and name of the document to upload. |
| `RemoveAllResourcePermissions` | `DELETE /api/v1/resources/{ResourceId}/permissions` | - | `ResourceId` | - | `Unit` | `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Removes all the permissions from the specified resource. |
| `RemoveResourcePermission` | `DELETE /api/v1/resources/{ResourceId}/permissions/{PrincipalId}` | - | `PrincipalId`, `ResourceId` | - | `Unit` | `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Removes the permission for the specified principal from the specified resource. |
| `RestoreDocumentVersions` | `POST /api/v1/documentVersions/restore/{DocumentId}` | - | `DocumentId` | - | `Unit` | `ConcurrentModificationException`, `ConflictingOperationException`, `EntityNotExistsException`, `FailedDependencyException`, `InvalidOperationException`, `ProhibitedStateException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Recovers a deleted version of an Amazon WorkDocs document. |
| `SearchResources` | `POST /api/v1/search` | `paginated` | - | - | `SearchResourcesResponse` | `InvalidArgumentException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Searches metadata and the content of folders, documents, document versions, and comments. |
| `UpdateDocument` | `PATCH /api/v1/documents/{DocumentId}` | - | `DocumentId` | - | `Unit` | `ConcurrentModificationException`, `ConflictingOperationException`, `EntityAlreadyExistsException`, `EntityNotExistsException`, `FailedDependencyException`, `LimitExceededException`, `ProhibitedStateException`, `ServiceUnavailableException`, ... (+2) | Updates the specified attributes of a document. The user must have access to both the document and its parent folder, if applicable. |
| `UpdateDocumentVersion` | `PATCH /api/v1/documents/{DocumentId}/versions/{VersionId}` | - | `DocumentId`, `VersionId` | - | `Unit` | `ConcurrentModificationException`, `EntityNotExistsException`, `FailedDependencyException`, `InvalidOperationException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Changes the status of the document version to ACTIVE. Amazon WorkDocs also sets its document container to ACTIVE. |
| `UpdateFolder` | `PATCH /api/v1/folders/{FolderId}` | - | `FolderId` | - | `Unit` | `ConcurrentModificationException`, `ConflictingOperationException`, `EntityAlreadyExistsException`, `EntityNotExistsException`, `FailedDependencyException`, `LimitExceededException`, `ProhibitedStateException`, `ServiceUnavailableException`, ... (+2) | Updates the specified attributes of the specified folder. The user must have access to both the folder and its parent folder, if applicable. |
| `UpdateUser` | `PATCH /api/v1/users/{UserId}` | - | `UserId` | - | `UpdateUserResponse` | `DeactivatingLastSystemUserException`, `EntityNotExistsException`, `FailedDependencyException`, `IllegalUserStateException`, `InvalidArgumentException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, ... (+1) | Updates the specified attributes of the specified user, and grants or revokes administrative privileges to the Amazon WorkDocs site. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `UnauthorizedResourceAccessException` | `structure` | `Message` | The caller does not have access to perform the action on the resource. |
| `ServiceUnavailableException` | `structure` | `Message` | One or more of the dependencies is unavailable. |
| `FailedDependencyException` | `structure` | `Message` | The Directory Service cannot reach an on-premises instance. |
| `UnauthorizedOperationException` | `structure` | `Code`, `Message` | The operation is not permitted. |
| `EntityNotExistsException` | `structure` | `EntityIds`, `Message` | The resource does not exist. |
| `ProhibitedStateException` | `structure` | `Message` | The specified document version is not in the INITIALIZED state. |
| `InvalidArgumentException` | `structure` | `Message` | The pagination marker or limit fields are not valid. |
| `ConcurrentModificationException` | `structure` | `Message` | The resource hierarchy is changing. |
| `ConflictingOperationException` | `structure` | `Message` | Another operation is in progress on the resource that conflicts with the current operation. |
| `LimitExceededException` | `structure` | `Message` | The maximum of 100,000 files and folders under the parent folder has been exceeded. |
| `EntityAlreadyExistsException` | `structure` | `Message` | The resource already exists. |
| `InvalidPasswordException` | `structure` | `Message` | The password is invalid. |
| `InvalidOperationException` | `structure` | `Message` | The operation is invalid. |
| `DocumentLockedForCommentsException` | `structure` | `Message` | This exception is thrown when the document is locked for comments and user tries to create or delete a comment on that document. |
| `AbortDocumentVersionUploadRequest` | `structure` | `AuthenticationToken`, `DocumentId`, `VersionId` | - |
| `ActivateUserRequest` | `structure` | `AuthenticationToken`, `UserId` | - |
| `ActivateUserResponse` | `structure` | `User` | - |
| `AddResourcePermissionsRequest` | `structure` | `AuthenticationToken`, `NotificationOptions`, `Principals`, `ResourceId` | - |
| `AddResourcePermissionsResponse` | `structure` | `ShareResults` | - |
| `CreateCommentRequest` | `structure` | `AuthenticationToken`, `DocumentId`, `NotifyCollaborators`, `ParentId`, `Text`, `ThreadId`, `VersionId`, `Visibility` | - |
| `CreateCommentResponse` | `structure` | `Comment` | - |
| `InvalidCommentOperationException` | `structure` | `Message` | The requested operation is not allowed on the specified comment object. |
| `CreateCustomMetadataRequest` | `structure` | `AuthenticationToken`, `CustomMetadata`, `ResourceId`, `VersionId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
