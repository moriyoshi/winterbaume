# QApps

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Amazon Q Apps feature capability within Amazon Q Business allows web experience users to create lightweight, purpose-built AI apps to fulfill specific tasks from within their web experience. For example, users can create a Q App that exclusively generates marketing-related content to improve your marketing team's productivity or a Q App for writing customer emails and creating promotional content using a certain style of voice, tone, and branding. For more information on the capabilities, see Amazon Q Apps capabilities in the Amazon Q Business User Guide . For an overview of the Amazon Q App APIs, see Overview of Amazon Q Apps API operations. For information about the IAM access control permissions you need to use the Amazon Q Apps API, see IAM role for the Amazon Q Business web experience including Amazon Q Apps in the Amazon Q Business User Guide .

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for QApps where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for QApps by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for QApps resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented QApps workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model setup and mutation workflows that create or update service resources across the `Update`, `List`, `Get`, `Batch`, `Create` operation families, including `UpdateLibraryItem`, `UpdateLibraryItemMetadata`, `UpdateQApp`, `UpdateQAppPermissions`, `ListCategories`, `ListLibraryItems`.

## Service Identity and Protocol

- AWS model slug: `qapps`
- AWS SDK for Rust slug: `qapps`
- Model version: `2023-11-27`
- Model file: `vendor/api-models-aws/models/qapps/service/2023-11-27/qapps-2023-11-27.json`
- SDK ID: `QApps`
- Endpoint prefix: `data.qapps`
- ARN namespace: `qapps`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Update` (6), `List` (5), `Get` (4), `Batch` (3), `Create` (3), `Associate` (2), `Delete` (2), `Disassociate` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateLibraryItemReview`, `AssociateQAppWithUser`, `BatchCreateCategory`, `BatchDeleteCategory`, `BatchUpdateCategory`, `CreateLibraryItem`, `CreatePresignedUrl`, `CreateQApp`, `DeleteLibraryItem`, `DeleteQApp`, `DisassociateLibraryItemReview`, `DisassociateQAppFromUser`, `ImportDocument`, `StartQAppSession`, `StopQAppSession`, `TagResource`, `UntagResource`, `UpdateLibraryItem`, `UpdateLibraryItemMetadata`, `UpdateQApp`, ... (+3).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeQAppPermissions`, `GetLibraryItem`, `GetQApp`, `GetQAppSession`, `GetQAppSessionMetadata`, `ListCategories`, `ListLibraryItems`, `ListQAppSessionData`, `ListQApps`, `ListTagsForResource`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 5 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ExportQAppSessionData`, `ImportDocument`, `StartQAppSession`, `StopQAppSession`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 35 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Update

- Operations: `UpdateLibraryItem`, `UpdateLibraryItemMetadata`, `UpdateQApp`, `UpdateQAppPermissions`, `UpdateQAppSession`, `UpdateQAppSessionMetadata`
- Traits: `idempotent` (1)
- Common required input members in this group: `appId`, `instanceId`, `libraryItemId`, `sessionId`, `sharingConfiguration`

### List

- Operations: `ListCategories`, `ListLibraryItems`, `ListQAppSessionData`, `ListQApps`, `ListTagsForResource`
- Traits: `paginated` (2), `readonly` (5)
- Common required input members in this group: `instanceId`, `resourceARN`, `sessionId`

### Get

- Operations: `GetLibraryItem`, `GetQApp`, `GetQAppSession`, `GetQAppSessionMetadata`
- Traits: `readonly` (4)
- Common required input members in this group: `appId`, `instanceId`, `libraryItemId`, `sessionId`

### Batch

- Operations: `BatchCreateCategory`, `BatchDeleteCategory`, `BatchUpdateCategory`
- Common required input members in this group: `categories`, `instanceId`

### Create

- Operations: `CreateLibraryItem`, `CreatePresignedUrl`, `CreateQApp`
- Common required input members in this group: `appDefinition`, `appId`, `appVersion`, `cardId`, `categories`, `fileContentsSha256`, `fileName`, `instanceId`, `scope`, `title`

### Associate

- Operations: `AssociateLibraryItemReview`, `AssociateQAppWithUser`
- Common required input members in this group: `appId`, `instanceId`, `libraryItemId`

### Delete

- Operations: `DeleteLibraryItem`, `DeleteQApp`
- Traits: `idempotent` (2)
- Common required input members in this group: `appId`, `instanceId`, `libraryItemId`

### Disassociate

- Operations: `DisassociateLibraryItemReview`, `DisassociateQAppFromUser`
- Common required input members in this group: `appId`, `instanceId`, `libraryItemId`

### Describe

- Operations: `DescribeQAppPermissions`
- Traits: `readonly` (1)
- Common required input members in this group: `appId`, `instanceId`

### Export

- Operations: `ExportQAppSessionData`
- Common required input members in this group: `instanceId`, `sessionId`

### Import

- Operations: `ImportDocument`
- Common required input members in this group: `appId`, `cardId`, `fileContentsBase64`, `fileName`, `instanceId`, `scope`

### Predict

- Operations: `PredictQApp`
- Common required input members in this group: `instanceId`

### Start

- Operations: `StartQAppSession`
- Common required input members in this group: `appId`, `appVersion`, `instanceId`

### Stop

- Operations: `StopQAppSession`
- Common required input members in this group: `instanceId`, `sessionId`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceARN`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceARN`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateLibraryItemReview` | `POST /catalog.associateItemRating` | - | `instanceId`, `libraryItemId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Associates a rating or review for a library item with the user submitting the request. This increments the rating count for the specified library item. |
| `AssociateQAppWithUser` | `POST /apps.install` | - | `appId`, `instanceId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | This operation creates a link between the user's identity calling the operation and a specific Q App. This is useful to mark the Q App as a favorite for the user if the user doesn't own the Amazon Q App so they can still run it and see it in their inventory... |
| `BatchCreateCategory` | `POST /catalog.createCategories` | - | `categories`, `instanceId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Creates Categories for the Amazon Q Business application environment instance. Web experience users use Categories to tag and filter library items. |
| `BatchDeleteCategory` | `POST /catalog.deleteCategories` | - | `categories`, `instanceId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Deletes Categories for the Amazon Q Business application environment instance. Web experience users use Categories to tag and filter library items. |
| `BatchUpdateCategory` | `POST /catalog.updateCategories` | - | `categories`, `instanceId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Updates Categories for the Amazon Q Business application environment instance. Web experience users use Categories to tag and filter library items. |
| `CreateLibraryItem` | `POST /catalog.createItem` | - | `appId`, `appVersion`, `categories`, `instanceId` | - | `CreateLibraryItemOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Creates a new library item for an Amazon Q App, allowing it to be discovered and used by other allowed users. |
| `CreatePresignedUrl` | `POST /apps.createPresignedUrl` | - | `appId`, `cardId`, `fileContentsSha256`, `fileName`, `instanceId`, `scope` | - | `CreatePresignedUrlOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Creates a presigned URL for an S3 POST operation to upload a file. You can use this URL to set a default file for a `FileUploadCard` in a Q App definition or to provide a file for a single Q App run. |
| `CreateQApp` | `POST /apps.create` | - | `appDefinition`, `instanceId`, `title` | - | `CreateQAppOutput` | `AccessDeniedException`, `ConflictException`, `ContentTooLargeException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Creates a new Amazon Q App based on the provided definition. The Q App definition specifies the cards and flow of the Q App. |
| `DeleteLibraryItem` | `POST /catalog.deleteItem` | `idempotent` | `instanceId`, `libraryItemId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Deletes a library item for an Amazon Q App, removing it from the library so it can no longer be discovered or used by other users. |
| `DeleteQApp` | `POST /apps.delete` | `idempotent` | `appId`, `instanceId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Deletes an Amazon Q App owned by the user. If the Q App was previously published to the library, it is also removed from the library. |
| `DescribeQAppPermissions` | `GET /apps.describeQAppPermissions` | `readonly` | `appId`, `instanceId` | - | `DescribeQAppPermissionsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Describes read permissions for a Amazon Q App in Amazon Q Business application environment instance. |
| `DisassociateLibraryItemReview` | `POST /catalog.disassociateItemRating` | - | `instanceId`, `libraryItemId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Removes a rating or review previously submitted by the user for a library item. |
| `DisassociateQAppFromUser` | `POST /apps.uninstall` | - | `appId`, `instanceId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Disassociates a Q App from a user removing the user's access to run the Q App. |
| `ExportQAppSessionData` | `POST /runtime.exportQAppSessionData` | - | `instanceId`, `sessionId` | - | `ExportQAppSessionDataOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Exports the collected data of a Q App data collection session. |
| `GetLibraryItem` | `GET /catalog.getItem` | `readonly` | `instanceId`, `libraryItemId` | - | `GetLibraryItemOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Retrieves details about a library item for an Amazon Q App, including its metadata, categories, ratings, and usage statistics. |
| `GetQApp` | `GET /apps.get` | `readonly` | `appId`, `instanceId` | - | `GetQAppOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Retrieves the full details of an Q App, including its definition specifying the cards and flow. |
| `GetQAppSession` | `GET /runtime.getQAppSession` | `readonly` | `instanceId`, `sessionId` | - | `GetQAppSessionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Retrieves the current state and results for an active session of an Amazon Q App. |
| `GetQAppSessionMetadata` | `GET /runtime.getQAppSessionMetadata` | `readonly` | `instanceId`, `sessionId` | - | `GetQAppSessionMetadataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Retrieves the current configuration of a Q App session. |
| `ImportDocument` | `POST /apps.importDocument` | - | `appId`, `cardId`, `fileContentsBase64`, `fileName`, `instanceId`, `scope` | - | `ImportDocumentOutput` | `AccessDeniedException`, `ContentTooLargeException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Uploads a file that can then be used either as a default in a `FileUploadCard` from Q App definition or as a file that is used inside a single Q App run. The purpose of the document is determined by a scope parameter that indicates whether it is at the app... |
| `ListCategories` | `GET /catalog.listCategories` | `readonly` | `instanceId` | - | `ListCategoriesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Lists the categories of a Amazon Q Business application environment instance. For more information, see Custom labels for Amazon Q Apps. |
| `ListLibraryItems` | `GET /catalog.list` | `readonly`, `paginated` | `instanceId` | - | `ListLibraryItemsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Lists the library items for Amazon Q Apps that are published and available for users in your Amazon Web Services account. |
| `ListQAppSessionData` | `GET /runtime.listQAppSessionData` | `readonly` | `instanceId`, `sessionId` | - | `ListQAppSessionDataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Lists the collected data of a Q App data collection session. |
| `ListQApps` | `GET /apps.list` | `readonly`, `paginated` | `instanceId` | - | `ListQAppsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Lists the Amazon Q Apps owned by or associated with the user either because they created it or because they used it from the library in the past. The user identity is extracted from the credentials used to invoke this operation.. |
| `ListTagsForResource` | `GET /tags/{resourceARN}` | `readonly` | `resourceARN` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags associated with an Amazon Q Apps resource. |
| `PredictQApp` | `POST /apps.predictQApp` | - | `instanceId` | - | `PredictQAppOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Generates an Amazon Q App definition based on either a conversation or a problem statement provided as input.The resulting app definition can be used to call `CreateQApp`. This API doesn't create Amazon Q Apps directly. |
| `StartQAppSession` | `POST /runtime.startQAppSession` | - | `appId`, `appVersion`, `instanceId` | - | `StartQAppSessionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Starts a new session for an Amazon Q App, allowing inputs to be provided and the app to be run. Each Q App session will be condensed into a single conversation in the web experience. |
| `StopQAppSession` | `POST /runtime.deleteMiniAppRun` | - | `instanceId`, `sessionId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Stops an active session for an Amazon Q App.This deletes all data related to the session and makes it invalid for future uses. The results of the session will be persisted as part of the conversation. |
| `TagResource` | `POST /tags/{resourceARN}` | `idempotent` | `resourceARN`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates tags with an Amazon Q Apps resource. |
| `UntagResource` | `DELETE /tags/{resourceARN}` | `idempotent` | `resourceARN`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates tags from an Amazon Q Apps resource. |
| `UpdateLibraryItem` | `POST /catalog.updateItem` | - | `instanceId`, `libraryItemId` | - | `UpdateLibraryItemOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Updates the library item for an Amazon Q App. |
| `UpdateLibraryItemMetadata` | `POST /catalog.updateItemMetadata` | `idempotent` | `instanceId`, `libraryItemId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Updates the verification status of a library item for an Amazon Q App. |
| `UpdateQApp` | `POST /apps.update` | - | `appId`, `instanceId` | - | `UpdateQAppOutput` | `AccessDeniedException`, `ContentTooLargeException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Updates an existing Amazon Q App, allowing modifications to its title, description, and definition. |
| `UpdateQAppPermissions` | `POST /apps.updateQAppPermissions` | - | `appId`, `instanceId` | - | `UpdateQAppPermissionsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Updates read permissions for a Amazon Q App in Amazon Q Business application environment instance. |
| `UpdateQAppSession` | `POST /runtime.updateQAppSession` | - | `instanceId`, `sessionId` | - | `UpdateQAppSessionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Updates the session for a given Q App `sessionId`. This is only valid when at least one card of the session is in the `WAITING` state. |
| `UpdateQAppSessionMetadata` | `POST /runtime.updateQAppSessionMetadata` | - | `instanceId`, `sessionId`, `sharingConfiguration` | - | `UpdateQAppSessionMetadataOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Updates the configuration metadata of a session for a given Q App `sessionId`. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `AssociateLibraryItemReview` | `instanceId -> instance-id` | - | - | - |
| `AssociateQAppWithUser` | `instanceId -> instance-id` | - | - | - |
| `BatchCreateCategory` | `instanceId -> instance-id` | - | - | - |
| `BatchDeleteCategory` | `instanceId -> instance-id` | - | - | - |
| `BatchUpdateCategory` | `instanceId -> instance-id` | - | - | - |
| `CreateLibraryItem` | `instanceId -> instance-id` | - | - | - |
| `CreatePresignedUrl` | `instanceId -> instance-id` | - | - | - |
| `CreateQApp` | `instanceId -> instance-id` | - | - | - |
| `DeleteLibraryItem` | `instanceId -> instance-id` | - | - | - |
| `DeleteQApp` | `instanceId -> instance-id` | - | - | - |
| `DescribeQAppPermissions` | `instanceId -> instance-id` | `appId -> appId` | - | - |
| `DisassociateLibraryItemReview` | `instanceId -> instance-id` | - | - | - |
| `DisassociateQAppFromUser` | `instanceId -> instance-id` | - | - | - |
| `ExportQAppSessionData` | `instanceId -> instance-id` | - | - | - |
| `GetLibraryItem` | `instanceId -> instance-id` | `libraryItemId -> libraryItemId`, `appId -> appId` | - | - |
| `GetQApp` | `instanceId -> instance-id` | `appId -> appId`, `appVersion -> appVersion` | - | - |
| `GetQAppSession` | `instanceId -> instance-id` | `sessionId -> sessionId` | - | - |
| `GetQAppSessionMetadata` | `instanceId -> instance-id` | `sessionId -> sessionId` | - | - |
| `ImportDocument` | `instanceId -> instance-id` | - | - | - |
| `ListCategories` | `instanceId -> instance-id` | - | - | - |
| `ListLibraryItems` | `instanceId -> instance-id` | `limit -> limit`, `nextToken -> nextToken`, `categoryId -> categoryId` | - | - |
| `ListQApps` | `instanceId -> instance-id` | `limit -> limit`, `nextToken -> nextToken` | - | - |
| `ListQAppSessionData` | `instanceId -> instance-id` | `sessionId -> sessionId` | - | - |
| `PredictQApp` | `instanceId -> instance-id` | - | - | - |
| `StartQAppSession` | `instanceId -> instance-id` | - | - | - |
| `StopQAppSession` | `instanceId -> instance-id` | - | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |
| `UpdateLibraryItem` | `instanceId -> instance-id` | - | - | - |
| `UpdateLibraryItemMetadata` | `instanceId -> instance-id` | - | - | - |
| `UpdateQApp` | `instanceId -> instance-id` | - | - | - |
| `UpdateQAppPermissions` | `instanceId -> instance-id` | - | - | - |
| `UpdateQAppSession` | `instanceId -> instance-id` | - | - | - |
| `UpdateQAppSessionMetadata` | `instanceId -> instance-id` | - | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | The client is not authorized to perform the requested operation. |
| `InternalServerException` | `structure` | `message`, `retryAfterSeconds` | An internal service error occurred while processing the request. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | The requested operation could not be completed because too many requests were sent at once. |
| `ValidationException` | `structure` | `message` | The input failed to satisfy the constraints specified by the service. |
| `UnauthorizedException` | `structure` | `message` | The client is not authenticated or authorized to perform the requested operation. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The requested resource could not be found. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | The requested operation could not be completed because it would exceed the service's quota or limit. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | The requested operation could not be completed due to a conflict with the current state of the resource. |
| `ContentTooLargeException` | `structure` | `message`, `resourceId`, `resourceType` | The requested operation could not be completed because the content exceeds the maximum allowed size. |
| `AssociateLibraryItemReviewInput` | `structure` | `instanceId`, `libraryItemId` | - |
| `AssociateQAppWithUserInput` | `structure` | `appId`, `instanceId` | - |
| `BatchCreateCategoryInput` | `structure` | `categories`, `instanceId` | - |
| `BatchDeleteCategoryInput` | `structure` | `categories`, `instanceId` | - |
| `BatchUpdateCategoryInput` | `structure` | `categories`, `instanceId` | - |
| `CreateLibraryItemInput` | `structure` | `appId`, `appVersion`, `categories`, `instanceId` | - |
| `CreateLibraryItemOutput` | `structure` | `createdAt`, `createdBy`, `isVerified`, `libraryItemId`, `ratingCount`, `status`, `updatedAt`, `updatedBy` | - |
| `CreatePresignedUrlInput` | `structure` | `appId`, `cardId`, `fileContentsSha256`, `fileName`, `instanceId`, `scope`, `sessionId` | - |
| `CreatePresignedUrlOutput` | `structure` | `fileId`, `presignedUrl`, `presignedUrlExpiration`, `presignedUrlFields` | - |
| `CreateQAppInput` | `structure` | `appDefinition`, `description`, `instanceId`, `tags`, `title` | - |
| `CreateQAppOutput` | `structure` | `appArn`, `appId`, `appVersion`, `createdAt`, `createdBy`, `description`, `initialPrompt`, `requiredCapabilities`, `status`, `title`, `updatedAt`, `updatedBy` | - |
| `DeleteLibraryItemInput` | `structure` | `instanceId`, `libraryItemId` | - |
| `DeleteQAppInput` | `structure` | `appId`, `instanceId` | - |
| `DescribeQAppPermissionsInput` | `structure` | `appId`, `instanceId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
