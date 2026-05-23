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
- Common required input members in this group: `DocumentId`, `VersionId`, `ResourceId`, `FolderId`

### Describe

- Operations: `DescribeActivities`, `DescribeComments`, `DescribeDocumentVersions`, `DescribeFolderContents`, `DescribeGroups`, `DescribeNotificationSubscriptions`, `DescribeResourcePermissions`, `DescribeRootFolders`, `DescribeUsers`
- Traits: `paginated` (9)
- Common required input members in this group: `DocumentId`

### Get

- Operations: `GetCurrentUser`, `GetDocument`, `GetDocumentPath`, `GetDocumentVersion`, `GetFolder`, `GetFolderPath`, `GetResources`
- Common required input members in this group: `DocumentId`, `FolderId`

### Create

- Operations: `CreateComment`, `CreateCustomMetadata`, `CreateFolder`, `CreateLabels`, `CreateNotificationSubscription`, `CreateUser`
- Common required input members in this group: `ResourceId`

### Update

- Operations: `UpdateDocument`, `UpdateDocumentVersion`, `UpdateFolder`, `UpdateUser`
- Common required input members in this group: `DocumentId`

### Remove

- Operations: `RemoveAllResourcePermissions`, `RemoveResourcePermission`
- Common required input members in this group: `ResourceId`

### Abort

- Operations: `AbortDocumentVersionUpload`
- Common required input members in this group: -

### Activate

- Operations: `ActivateUser`
- Common required input members in this group: -

### Add

- Operations: `AddResourcePermissions`
- Common required input members in this group: -

### Deactivate

- Operations: `DeactivateUser`
- Common required input members in this group: -

### Initiate

- Operations: `InitiateDocumentVersionUpload`
- Common required input members in this group: -

### Restore

- Operations: `RestoreDocumentVersions`
- Common required input members in this group: -

### Search

- Operations: `SearchResources`
- Traits: `paginated` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AbortDocumentVersionUpload` | `DELETE /api/v1/documents/{DocumentId}/versions/{VersionId}` | - | `DocumentId`, `VersionId` | - | `Unit` | `ConcurrentModificationException`, `EntityNotExistsException`, `FailedDependencyException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Aborts the upload of the specified document version that was previously initiated by InitiateDocumentVersionUpload . The client should make this call only when it no longer intends to upload the document version, or ... |
| `ActivateUser` | `POST /api/v1/users/{UserId}/activation` | - | `UserId` | - | `ActivateUserResponse` | `EntityNotExistsException`, `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Activates the specified user. Only active users can access Amazon WorkDocs. |
| `AddResourcePermissions` | `POST /api/v1/resources/{ResourceId}/permissions` | - | `ResourceId`, `Principals` | - | `AddResourcePermissionsResponse` | `FailedDependencyException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Creates a set of permissions for the specified folder or document. The resource permissions are overwritten if the principals already have different permissions. |
| `CreateComment` | `POST /api/v1/documents/{DocumentId}/versions/{VersionId}/comment` | - | `DocumentId`, `VersionId`, `Text` | - | `CreateCommentResponse` | `DocumentLockedForCommentsException`, `EntityNotExistsException`, `FailedDependencyException`, `InvalidCommentOperationException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Adds a new comment to the specified document version. |
| `CreateCustomMetadata` | `PUT /api/v1/resources/{ResourceId}/customMetadata` | - | `ResourceId`, `CustomMetadata` | - | `CreateCustomMetadataResponse` | `CustomMetadataLimitExceededException`, `EntityNotExistsException`, `FailedDependencyException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Adds one or more custom properties to the specified resource (a folder, document, or version). |
| `CreateFolder` | `POST /api/v1/folders` | - | `ParentFolderId` | - | `CreateFolderResponse` | `ConcurrentModificationException`, `ConflictingOperationException`, `EntityAlreadyExistsException`, `EntityNotExistsException`, `FailedDependencyException`, `LimitExceededException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Creates a folder with the specified name and parent folder. |
| `CreateLabels` | `PUT /api/v1/resources/{ResourceId}/labels` | - | `ResourceId`, `Labels` | - | `CreateLabelsResponse` | `EntityNotExistsException`, `FailedDependencyException`, `ServiceUnavailableException`, `TooManyLabelsException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Adds the specified list of labels to the given resource (a document or folder) |
| `CreateNotificationSubscription` | `POST /api/v1/organizations/{OrganizationId}/subscriptions` | - | `OrganizationId`, `Endpoint`, `Protocol`, `SubscriptionType` | - | `CreateNotificationSubscriptionResponse` | `InvalidArgumentException`, `ServiceUnavailableException`, `TooManySubscriptionsException`, `UnauthorizedResourceAccessException` | Configure Amazon WorkDocs to use Amazon SNS notifications. The endpoint receives a confirmation message, and must confirm the subscription. For more information, see Setting up notifications for an IAM user or role i ... |
| `CreateUser` | `POST /api/v1/users` | - | `Username`, `GivenName`, `Surname`, `Password` | - | `CreateUserResponse` | `EntityAlreadyExistsException`, `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Creates a user in a Simple AD or Microsoft AD directory. The status of a newly created user is "ACTIVE". New users can access Amazon WorkDocs. |
| `DeactivateUser` | `DELETE /api/v1/users/{UserId}/activation` | - | `UserId` | - | `Unit` | `EntityNotExistsException`, `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Deactivates the specified user, which revokes the user's access to Amazon WorkDocs. |
| `DeleteComment` | `DELETE /api/v1/documents/{DocumentId}/versions/{VersionId}/comment/{CommentId}` | - | `DocumentId`, `VersionId`, `CommentId` | - | `Unit` | `DocumentLockedForCommentsException`, `EntityNotExistsException`, `FailedDependencyException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Deletes the specified comment from the document version. |
| `DeleteCustomMetadata` | `DELETE /api/v1/resources/{ResourceId}/customMetadata` | - | `ResourceId` | - | `DeleteCustomMetadataResponse` | `EntityNotExistsException`, `FailedDependencyException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Deletes custom metadata from the specified resource. |
| `DeleteDocument` | `DELETE /api/v1/documents/{DocumentId}` | - | `DocumentId` | - | `Unit` | `ConcurrentModificationException`, `ConflictingOperationException`, `EntityNotExistsException`, `FailedDependencyException`, `LimitExceededException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Permanently deletes the specified document and its associated metadata. |
| `DeleteDocumentVersion` | `DELETE /api/v1/documentVersions/{DocumentId}/versions/{VersionId}` | - | `DocumentId`, `VersionId`, `DeletePriorVersions` | - | `Unit` | `ConcurrentModificationException`, `ConflictingOperationException`, `EntityNotExistsException`, `FailedDependencyException`, `InvalidOperationException`, `ProhibitedStateException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Deletes a specific version of a document. |
| `DeleteFolder` | `DELETE /api/v1/folders/{FolderId}` | - | `FolderId` | - | `Unit` | `ConcurrentModificationException`, `ConflictingOperationException`, `EntityNotExistsException`, `FailedDependencyException`, `LimitExceededException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Permanently deletes the specified folder and its contents. |
| `DeleteFolderContents` | `DELETE /api/v1/folders/{FolderId}/contents` | - | `FolderId` | - | `Unit` | `ConflictingOperationException`, `EntityNotExistsException`, `FailedDependencyException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Deletes the contents of the specified folder. |
| `DeleteLabels` | `DELETE /api/v1/resources/{ResourceId}/labels` | - | `ResourceId` | - | `DeleteLabelsResponse` | `EntityNotExistsException`, `FailedDependencyException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Deletes the specified list of labels from a resource. |
| `DeleteNotificationSubscription` | `DELETE /api/v1/organizations/{OrganizationId}/subscriptions/{SubscriptionId}` | - | `SubscriptionId`, `OrganizationId` | - | `Unit` | `EntityNotExistsException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedResourceAccessException` | Deletes the specified subscription from the specified organization. |
| `DeleteUser` | `DELETE /api/v1/users/{UserId}` | - | `UserId` | - | `Unit` | `EntityNotExistsException`, `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Deletes the specified user from a Simple AD or Microsoft AD directory. Deleting a user immediately and permanently deletes all content in that user's folder structure. Site retention policies do NOT apply to this typ ... |
| `DescribeActivities` | `GET /api/v1/activities` | `paginated` | - | - | `DescribeActivitiesResponse` | `FailedDependencyException`, `InvalidArgumentException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Describes the user activities in a specified time period. |
| `DescribeComments` | `GET /api/v1/documents/{DocumentId}/versions/{VersionId}/comments` | `paginated` | `DocumentId`, `VersionId` | - | `DescribeCommentsResponse` | `EntityNotExistsException`, `FailedDependencyException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | List all the comments for the specified document version. |
| `DescribeDocumentVersions` | `GET /api/v1/documents/{DocumentId}/versions` | `paginated` | `DocumentId` | - | `DescribeDocumentVersionsResponse` | `EntityNotExistsException`, `FailedDependencyException`, `InvalidArgumentException`, `InvalidPasswordException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Retrieves the document versions for the specified document. By default, only active versions are returned. |
| `DescribeFolderContents` | `GET /api/v1/folders/{FolderId}/contents` | `paginated` | `FolderId` | - | `DescribeFolderContentsResponse` | `EntityNotExistsException`, `FailedDependencyException`, `InvalidArgumentException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedResourceAccessException` | Describes the contents of the specified folder, including its documents and subfolders. By default, Amazon WorkDocs returns the first 100 active document and folder metadata items. If there are more results, the resp ... |
| `DescribeGroups` | `GET /api/v1/groups` | `paginated` | `SearchQuery` | - | `DescribeGroupsResponse` | `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Describes the groups specified by the query. Groups are defined by the underlying Active Directory. |
| `DescribeNotificationSubscriptions` | `GET /api/v1/organizations/{OrganizationId}/subscriptions` | `paginated` | `OrganizationId` | - | `DescribeNotificationSubscriptionsResponse` | `EntityNotExistsException`, `ServiceUnavailableException`, `UnauthorizedResourceAccessException` | Lists the specified notification subscriptions. |
| `DescribeResourcePermissions` | `GET /api/v1/resources/{ResourceId}/permissions` | `paginated` | `ResourceId` | - | `DescribeResourcePermissionsResponse` | `FailedDependencyException`, `InvalidArgumentException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Describes the permissions of a specified resource. |
| `DescribeRootFolders` | `GET /api/v1/me/root` | `paginated` | `AuthenticationToken` | - | `DescribeRootFoldersResponse` | `FailedDependencyException`, `InvalidArgumentException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Describes the current user's special folders; the RootFolder and the RecycleBin . RootFolder is the root of user's files and folders and RecycleBin is the root of recycled items. This is not a valid action for SigV4 ... |
| `DescribeUsers` | `GET /api/v1/users` | `paginated` | - | - | `DescribeUsersResponse` | `EntityNotExistsException`, `FailedDependencyException`, `InvalidArgumentException`, `RequestedEntityTooLargeException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Describes the specified users. You can describe all users or filter the results (for example, by status or organization). By default, Amazon WorkDocs returns the first 24 active or pending users. If there are more re ... |
| `GetCurrentUser` | `GET /api/v1/me` | - | `AuthenticationToken` | - | `GetCurrentUserResponse` | `EntityNotExistsException`, `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Retrieves details of the current user for whom the authentication token was generated. This is not a valid action for SigV4 (administrative API) clients. This action requires an authentication token. To get an authen ... |
| `GetDocument` | `GET /api/v1/documents/{DocumentId}` | - | `DocumentId` | - | `GetDocumentResponse` | `EntityNotExistsException`, `FailedDependencyException`, `InvalidArgumentException`, `InvalidPasswordException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Retrieves details of a document. |
| `GetDocumentPath` | `GET /api/v1/documents/{DocumentId}/path` | - | `DocumentId` | - | `GetDocumentPathResponse` | `EntityNotExistsException`, `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Retrieves the path information (the hierarchy from the root folder) for the requested document. By default, Amazon WorkDocs returns a maximum of 100 levels upwards from the requested document and only includes the ID ... |
| `GetDocumentVersion` | `GET /api/v1/documents/{DocumentId}/versions/{VersionId}` | - | `DocumentId`, `VersionId` | - | `GetDocumentVersionResponse` | `EntityNotExistsException`, `FailedDependencyException`, `InvalidPasswordException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Retrieves version metadata for the specified document. |
| `GetFolder` | `GET /api/v1/folders/{FolderId}` | - | `FolderId` | - | `GetFolderResponse` | `EntityNotExistsException`, `FailedDependencyException`, `InvalidArgumentException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Retrieves the metadata of the specified folder. |
| `GetFolderPath` | `GET /api/v1/folders/{FolderId}/path` | - | `FolderId` | - | `GetFolderPathResponse` | `EntityNotExistsException`, `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Retrieves the path information (the hierarchy from the root folder) for the specified folder. By default, Amazon WorkDocs returns a maximum of 100 levels upwards from the requested folder and only includes the IDs of ... |
| `GetResources` | `GET /api/v1/resources` | - | - | - | `GetResourcesResponse` | `FailedDependencyException`, `InvalidArgumentException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Retrieves a collection of resources, including folders and documents. The only CollectionType supported is SHARED_WITH_ME . |
| `InitiateDocumentVersionUpload` | `POST /api/v1/documents` | - | - | - | `InitiateDocumentVersionUploadResponse` | `DraftUploadOutOfSyncException`, `EntityAlreadyExistsException`, `EntityNotExistsException`, `FailedDependencyException`, `InvalidArgumentException`, `InvalidPasswordException`, `LimitExceededException`, `ProhibitedStateException`, `ResourceAlreadyCheckedOutException`, `ServiceUnavailableException`, `StorageLimitExceededException`, `StorageLimitWillExceedException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Creates a new document object and version object. The client specifies the parent folder ID and name of the document to upload. The ID is optionally specified when creating a new version of an existing document. This ... |
| `RemoveAllResourcePermissions` | `DELETE /api/v1/resources/{ResourceId}/permissions` | - | `ResourceId` | - | `Unit` | `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Removes all the permissions from the specified resource. |
| `RemoveResourcePermission` | `DELETE /api/v1/resources/{ResourceId}/permissions/{PrincipalId}` | - | `ResourceId`, `PrincipalId` | - | `Unit` | `FailedDependencyException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Removes the permission for the specified principal from the specified resource. |
| `RestoreDocumentVersions` | `POST /api/v1/documentVersions/restore/{DocumentId}` | - | `DocumentId` | - | `Unit` | `ConcurrentModificationException`, `ConflictingOperationException`, `EntityNotExistsException`, `FailedDependencyException`, `InvalidOperationException`, `ProhibitedStateException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Recovers a deleted version of an Amazon WorkDocs document. |
| `SearchResources` | `POST /api/v1/search` | `paginated` | - | - | `SearchResourcesResponse` | `InvalidArgumentException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Searches metadata and the content of folders, documents, document versions, and comments. |
| `UpdateDocument` | `PATCH /api/v1/documents/{DocumentId}` | - | `DocumentId` | - | `Unit` | `ConcurrentModificationException`, `ConflictingOperationException`, `EntityAlreadyExistsException`, `EntityNotExistsException`, `FailedDependencyException`, `LimitExceededException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Updates the specified attributes of a document. The user must have access to both the document and its parent folder, if applicable. |
| `UpdateDocumentVersion` | `PATCH /api/v1/documents/{DocumentId}/versions/{VersionId}` | - | `DocumentId`, `VersionId` | - | `Unit` | `ConcurrentModificationException`, `EntityNotExistsException`, `FailedDependencyException`, `InvalidOperationException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Changes the status of the document version to ACTIVE. Amazon WorkDocs also sets its document container to ACTIVE. This is the last step in a document upload, after the client uploads the document to an S3-presigned U ... |
| `UpdateFolder` | `PATCH /api/v1/folders/{FolderId}` | - | `FolderId` | - | `Unit` | `ConcurrentModificationException`, `ConflictingOperationException`, `EntityAlreadyExistsException`, `EntityNotExistsException`, `FailedDependencyException`, `LimitExceededException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Updates the specified attributes of the specified folder. The user must have access to both the folder and its parent folder, if applicable. |
| `UpdateUser` | `PATCH /api/v1/users/{UserId}` | - | `UserId` | - | `UpdateUserResponse` | `DeactivatingLastSystemUserException`, `EntityNotExistsException`, `FailedDependencyException`, `IllegalUserStateException`, `InvalidArgumentException`, `ProhibitedStateException`, `ServiceUnavailableException`, `UnauthorizedOperationException`, `UnauthorizedResourceAccessException` | Updates the specified attributes of the specified user, and grants or revokes administrative privileges to the Amazon WorkDocs site. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `AbortDocumentVersionUpload` | `AuthenticationToken -> Authentication` | - | - | - |
| `ActivateUser` | `AuthenticationToken -> Authentication` | - | - | - |
| `AddResourcePermissions` | `AuthenticationToken -> Authentication` | - | - | - |
| `CreateComment` | `AuthenticationToken -> Authentication` | - | - | - |
| `CreateCustomMetadata` | `AuthenticationToken -> Authentication` | `VersionId -> versionid` | - | - |
| `CreateFolder` | `AuthenticationToken -> Authentication` | - | - | - |
| `CreateLabels` | `AuthenticationToken -> Authentication` | - | - | - |
| `CreateUser` | `AuthenticationToken -> Authentication` | - | - | - |
| `DeactivateUser` | `AuthenticationToken -> Authentication` | - | - | - |
| `DeleteComment` | `AuthenticationToken -> Authentication` | - | - | - |
| `DeleteCustomMetadata` | `AuthenticationToken -> Authentication` | `VersionId -> versionId`, `Keys -> keys`, `DeleteAll -> deleteAll` | - | - |
| `DeleteDocument` | `AuthenticationToken -> Authentication` | - | - | - |
| `DeleteDocumentVersion` | `AuthenticationToken -> Authentication` | `DeletePriorVersions -> deletePriorVersions` | - | - |
| `DeleteFolder` | `AuthenticationToken -> Authentication` | - | - | - |
| `DeleteFolderContents` | `AuthenticationToken -> Authentication` | - | - | - |
| `DeleteLabels` | `AuthenticationToken -> Authentication` | `Labels -> labels`, `DeleteAll -> deleteAll` | - | - |
| `DeleteUser` | `AuthenticationToken -> Authentication` | - | - | - |
| `DescribeActivities` | `AuthenticationToken -> Authentication` | `StartTime -> startTime`, `EndTime -> endTime`, `OrganizationId -> organizationId`, `ActivityTypes -> activityTypes`, `ResourceId -> resourceId`, `UserId -> userId`, `IncludeIndirectActivities -> includeIndirectActivities`, `Limit -> limit`, `Marker -> marker` | - | - |
| `DescribeComments` | `AuthenticationToken -> Authentication` | `Limit -> limit`, `Marker -> marker` | - | - |
| `DescribeDocumentVersions` | `AuthenticationToken -> Authentication` | `Marker -> marker`, `Limit -> limit`, `Include -> include`, `Fields -> fields` | - | - |
| `DescribeFolderContents` | `AuthenticationToken -> Authentication` | `Sort -> sort`, `Order -> order`, `Limit -> limit`, `Marker -> marker`, `Type -> type`, `Include -> include` | - | - |
| `DescribeGroups` | `AuthenticationToken -> Authentication` | `SearchQuery -> searchQuery`, `OrganizationId -> organizationId`, `Marker -> marker`, `Limit -> limit` | - | - |
| `DescribeNotificationSubscriptions` | - | `Marker -> marker`, `Limit -> limit` | - | - |
| `DescribeResourcePermissions` | `AuthenticationToken -> Authentication` | `PrincipalId -> principalId`, `Limit -> limit`, `Marker -> marker` | - | - |
| `DescribeRootFolders` | `AuthenticationToken -> Authentication` | `Limit -> limit`, `Marker -> marker` | - | - |
| `DescribeUsers` | `AuthenticationToken -> Authentication` | `OrganizationId -> organizationId`, `UserIds -> userIds`, `Query -> query`, `Include -> include`, `Order -> order`, `Sort -> sort`, `Marker -> marker`, `Limit -> limit`, `Fields -> fields` | - | - |
| `GetCurrentUser` | `AuthenticationToken -> Authentication` | - | - | - |
| `GetDocument` | `AuthenticationToken -> Authentication` | `IncludeCustomMetadata -> includeCustomMetadata` | - | - |
| `GetDocumentPath` | `AuthenticationToken -> Authentication` | `Limit -> limit`, `Fields -> fields`, `Marker -> marker` | - | - |
| `GetDocumentVersion` | `AuthenticationToken -> Authentication` | `Fields -> fields`, `IncludeCustomMetadata -> includeCustomMetadata` | - | - |
| `GetFolder` | `AuthenticationToken -> Authentication` | `IncludeCustomMetadata -> includeCustomMetadata` | - | - |
| `GetFolderPath` | `AuthenticationToken -> Authentication` | `Limit -> limit`, `Fields -> fields`, `Marker -> marker` | - | - |
| `GetResources` | `AuthenticationToken -> Authentication` | `UserId -> userId`, `CollectionType -> collectionType`, `Limit -> limit`, `Marker -> marker` | - | - |
| `InitiateDocumentVersionUpload` | `AuthenticationToken -> Authentication` | - | - | - |
| `RemoveAllResourcePermissions` | `AuthenticationToken -> Authentication` | - | - | - |
| `RemoveResourcePermission` | `AuthenticationToken -> Authentication` | `PrincipalType -> type` | - | - |
| `RestoreDocumentVersions` | `AuthenticationToken -> Authentication` | - | - | - |
| `SearchResources` | `AuthenticationToken -> Authentication` | - | - | - |
| `UpdateDocument` | `AuthenticationToken -> Authentication` | - | - | - |
| `UpdateDocumentVersion` | `AuthenticationToken -> Authentication` | - | - | - |
| `UpdateFolder` | `AuthenticationToken -> Authentication` | - | - | - |
| `UpdateUser` | `AuthenticationToken -> Authentication` | - | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ConcurrentModificationException` | `structure` | Message | The resource hierarchy is changing. |
| `ConflictingOperationException` | `structure` | Message | Another operation is in progress on the resource that conflicts with the current operation. |
| `CustomMetadataLimitExceededException` | `structure` | Message | The limit has been reached on the number of custom properties for the specified resource. |
| `DeactivatingLastSystemUserException` | `structure` | Message, Code | The last user in the organization is being deactivated. |
| `DocumentLockedForCommentsException` | `structure` | Message | This exception is thrown when the document is locked for comments and user tries to create or delete a comment on that document. |
| `DraftUploadOutOfSyncException` | `structure` | Message | This exception is thrown when a valid checkout ID is not presented on document version upload calls for a document that has been checked out from Web client. |
| `EntityAlreadyExistsException` | `structure` | Message | The resource already exists. |
| `EntityNotExistsException` | `structure` | Message, EntityIds | The resource does not exist. |
| `FailedDependencyException` | `structure` | Message | The Directory Service cannot reach an on-premises instance. Or a dependency under the control of the organization is failing, such as a connected Active Dir ... |
| `IllegalUserStateException` | `structure` | Message | The user is undergoing transfer of ownership. |
| `InvalidArgumentException` | `structure` | Message | The pagination marker or limit fields are not valid. |
| `InvalidCommentOperationException` | `structure` | Message | The requested operation is not allowed on the specified comment object. |
| `InvalidOperationException` | `structure` | Message | The operation is invalid. |
| `InvalidPasswordException` | `structure` | Message | The password is invalid. |
| `LimitExceededException` | `structure` | Message | The maximum of 100,000 files and folders under the parent folder has been exceeded. |
| `ProhibitedStateException` | `structure` | Message | The specified document version is not in the INITIALIZED state. |
| `RequestedEntityTooLargeException` | `structure` | Message | The response is too large to return. The request must include a filter to reduce the size of the response. |
| `ResourceAlreadyCheckedOutException` | `structure` | Message | The resource is already checked out. |
| `ServiceUnavailableException` | `structure` | Message | One or more of the dependencies is unavailable. |
| `StorageLimitExceededException` | `structure` | Message | The storage limit has been exceeded. |
| `StorageLimitWillExceedException` | `structure` | Message | The storage limit will be exceeded. |
| `TooManyLabelsException` | `structure` | Message | The limit has been reached on the number of labels for the specified resource. |
| `TooManySubscriptionsException` | `structure` | Message | You've reached the limit on the number of subscriptions for the WorkDocs instance. |
| `UnauthorizedOperationException` | `structure` | Message, Code | The operation is not permitted. |
| `UnauthorizedResourceAccessException` | `structure` | Message | The caller does not have access to perform the action on the resource. |
| `AbortDocumentVersionUploadRequest` | `structure` | AuthenticationToken, DocumentId, VersionId | - |
| `ActivateUserRequest` | `structure` | UserId, AuthenticationToken | - |
| `ActivateUserResponse` | `structure` | User | - |
| `AddResourcePermissionsRequest` | `structure` | AuthenticationToken, ResourceId, Principals, NotificationOptions | - |
| `AddResourcePermissionsResponse` | `structure` | ShareResults | - |
| `CreateCommentRequest` | `structure` | AuthenticationToken, DocumentId, VersionId, ParentId, ThreadId, Text, Visibility, NotifyCollaborators | - |
| `CreateCommentResponse` | `structure` | Comment | - |
| `CreateCustomMetadataRequest` | `structure` | AuthenticationToken, ResourceId, VersionId, CustomMetadata | - |
| `CreateCustomMetadataResponse` | `structure` | **empty (no members)** | - |
| `CreateFolderRequest` | `structure` | AuthenticationToken, Name, ParentFolderId | - |
| `CreateFolderResponse` | `structure` | Metadata | - |
| `CreateLabelsRequest` | `structure` | ResourceId, Labels, AuthenticationToken | - |
| `CreateLabelsResponse` | `structure` | **empty (no members)** | - |
| `CreateNotificationSubscriptionRequest` | `structure` | OrganizationId, Endpoint, Protocol, SubscriptionType | - |
| `CreateNotificationSubscriptionResponse` | `structure` | Subscription | - |
| `ActivityType` | `enum` | DOCUMENT_CHECKED_IN, DOCUMENT_CHECKED_OUT, DOCUMENT_RENAMED, DOCUMENT_VERSION_UPLOADED, DOCUMENT_VERSION_DELETED, DOCUMENT_VERSION_VIEWED, DOCUMENT_VERSION_DOWNLOADED, DOCUMENT_RECYCLED, DOCUMENT_RESTORED, DOCUMENT_REVERTED, DOCUMENT_SHARED, DOCUMENT_UNSHARED, ... (+21) | - |
| `AdditionalResponseFieldType` | `enum` | WEBURL | - |
| `BooleanEnumType` | `enum` | TRUE, FALSE | - |
| `CommentStatusType` | `enum` | DRAFT, PUBLISHED, DELETED | - |
| `CommentVisibilityType` | `enum` | PUBLIC, PRIVATE | - |
| `ContentCategoryType` | `enum` | IMAGE, DOCUMENT, PDF, SPREADSHEET, PRESENTATION, AUDIO, VIDEO, SOURCE_CODE, OTHER | - |
| `DocumentSourceType` | `enum` | ORIGINAL, WITH_COMMENTS | - |
| `DocumentStatusType` | `enum` | INITIALIZED, ACTIVE | - |
| `DocumentThumbnailType` | `enum` | SMALL, SMALL_HQ, LARGE | - |
| `DocumentVersionStatus` | `enum` | ACTIVE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
