# QBusiness

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This is the Amazon Q Business API Reference. Amazon Q Business is a fully managed, generative-AI powered enterprise chat assistant that you can deploy within your organization. Amazon Q Business enhances employee productivity by supporting key tasks such as question-answering, knowledge discovery, writing email messages, summarizing text, drafting document outlines, and brainstorming ideas. Users ask questions of Amazon Q Business and get answers that are presented in a conversational manner. For an introduction to the service, see the Amazon Q Business User Guide .

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for QBusiness where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for QBusiness by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for QBusiness resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented QBusiness workflows in the local mock. Key resources include `ApplicationResource`, `DataAccessorResource`, `DataSourceResource`, `IndexResource`, `IntegrationResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Delete`, `Create`, `Update` operation families, including `ListApplications`, `ListAttachments`, `ListChatResponseConfigurations`, `ListConversations`, `GetApplication`, `GetChatControlsConfiguration`.

## Service Identity and Protocol

- AWS model slug: `qbusiness`
- AWS SDK for Rust slug: `qbusiness`
- Model version: `2023-11-27`
- Model file: `vendor/api-models-aws/models/qbusiness/service/2023-11-27/qbusiness-2023-11-27.json`
- SDK ID: `QBusiness`
- Endpoint prefix: `-`
- ARN namespace: `qbusiness`
- CloudFormation name: `QBusiness`
- CloudTrail event source: `qbusiness.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (19), `Get` (14), `Delete` (13), `Create` (11), `Update` (11), `Batch` (2), `Chat` (2), `Put` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociatePermission`, `BatchDeleteDocument`, `BatchPutDocument`, `CancelSubscription`, `CreateAnonymousWebExperienceUrl`, `CreateApplication`, `CreateChatResponseConfiguration`, `CreateDataAccessor`, `CreateDataSource`, `CreateIndex`, `CreatePlugin`, `CreateRetriever`, `CreateSubscription`, `CreateUser`, `CreateWebExperience`, `DeleteApplication`, `DeleteAttachment`, `DeleteChatControlsConfiguration`, `DeleteChatResponseConfiguration`, `DeleteConversation`, ... (+26).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `CheckDocumentAccess`, `GetApplication`, `GetChatControlsConfiguration`, `GetChatResponseConfiguration`, `GetDataAccessor`, `GetDataSource`, `GetDocumentContent`, `GetGroup`, `GetIndex`, `GetMedia`, `GetPlugin`, `GetPolicy`, `GetRetriever`, `GetUser`, `GetWebExperience`, `ListApplications`, `ListAttachments`, `ListChatResponseConfigurations`, `ListConversations`, `ListDataAccessors`, ... (+15).
- Pagination is modelled for 20 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 41 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelSubscription`, `ListDataSourceSyncJobs`, `StartDataSourceSyncJob`, `StopDataSourceSyncJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 83 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `SNS`, `Lambda`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ApplicationResource` | `applicationId` | create: `CreateApplication`; read: `GetApplication`; update: `UpdateApplication`; delete: `DeleteApplication`; list: `ListApplications` | - | - |
| `DataAccessorResource` | `applicationId`, `dataAccessorId` | create: `CreateDataAccessor`; read: `GetDataAccessor`; update: `UpdateDataAccessor`; delete: `DeleteDataAccessor`; list: `ListDataAccessors` | - | - |
| `DataSourceResource` | `applicationId`, `dataSourceId`, `indexId` | create: `CreateDataSource`; read: `GetDataSource`; update: `UpdateDataSource`; delete: `DeleteDataSource`; list: `ListDataSources` | - | - |
| `IndexResource` | `applicationId`, `indexId` | create: `CreateIndex`; read: `GetIndex`; update: `UpdateIndex`; delete: `DeleteIndex`; list: `ListIndices` | - | - |
| `IntegrationResource` | `applicationId`, `integrationId` | - | - | - |
| `PluginResource` | `applicationId`, `pluginId` | create: `CreatePlugin`; read: `GetPlugin`; update: `UpdatePlugin`; delete: `DeletePlugin`; list: `ListPlugins` | - | - |
| `RetrieverResource` | `applicationId`, `retrieverId` | create: `CreateRetriever`; read: `GetRetriever`; update: `UpdateRetriever`; delete: `DeleteRetriever`; list: `ListRetrievers` | - | - |
| `WebExperienceResource` | `applicationId`, `webExperienceId` | create: `CreateWebExperience`; read: `GetWebExperience`; update: `UpdateWebExperience`; delete: `DeleteWebExperience`; list: `ListWebExperiences` | - | - |
## Operation Groups

### List

- Operations: `ListAttachments`, `ListChatResponseConfigurations`, `ListConversations`, `ListDataSourceSyncJobs`, `ListDocuments`, `ListGroups`, `ListMessages`, `ListPluginActions`, `ListPluginTypeActions`, `ListPluginTypeMetadata`, `ListSubscriptions`, `ListTagsForResource`
- Traits: `readonly` (12), `paginated` (11)
- Common required input members in this group: `applicationId`, `indexId`

### Get

- Operations: `GetChatControlsConfiguration`, `GetChatResponseConfiguration`, `GetDocumentContent`, `GetGroup`, `GetMedia`, `GetPolicy`, `GetUser`
- Traits: `readonly` (7), `paginated` (1)
- Common required input members in this group: `applicationId`, `indexId`

### Delete

- Operations: `DeleteAttachment`, `DeleteChatControlsConfiguration`, `DeleteChatResponseConfiguration`, `DeleteConversation`, `DeleteGroup`, `DeleteUser`
- Traits: `idempotent` (6)
- Common required input members in this group: `applicationId`, `conversationId`

### Create

- Operations: `CreateAnonymousWebExperienceUrl`, `CreateChatResponseConfiguration`, `CreateSubscription`, `CreateUser`
- Traits: `idempotent` (3), `idempotency-token` (3)
- Common required input members in this group: `applicationId`

### Update

- Operations: `UpdateChatControlsConfiguration`, `UpdateChatResponseConfiguration`, `UpdateSubscription`, `UpdateUser`
- Traits: `idempotent` (4), `idempotency-token` (2)
- Common required input members in this group: `applicationId`

### Batch

- Operations: `BatchDeleteDocument`, `BatchPutDocument`
- Common required input members in this group: `applicationId`, `indexId`, `documents`

### Chat

- Operations: `Chat`, `ChatSync`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `applicationId`

### Put

- Operations: `PutFeedback`, `PutGroup`
- Traits: `idempotent` (1)
- Common required input members in this group: `applicationId`

### Associate

- Operations: `AssociatePermission`
- Common required input members in this group: -

### Cancel

- Operations: `CancelSubscription`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Check

- Operations: `CheckDocumentAccess`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociatePermission`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Search

- Operations: `SearchRelevantContent`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Start

- Operations: `StartDataSourceSyncJob`
- Common required input members in this group: -

### Stop

- Operations: `StopDataSourceSyncJob`
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
| `AssociatePermission` | `POST /applications/{applicationId}/policy` | - | `applicationId`, `statementId`, `actions`, `principal` | - | `AssociatePermissionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds or updates a permission policy for a Amazon Q Business application, allowing cross-account access for an ISV. This operation creates a new policy statement for the specified Amazon Q Business application. The po ... |
| `BatchDeleteDocument` | `POST /applications/{applicationId}/indices/{indexId}/documents/delete` | - | `applicationId`, `indexId`, `documents` | - | `BatchDeleteDocumentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Asynchronously deletes one or more documents added using the BatchPutDocument API from an Amazon Q Business index. You can see the progress of the deletion, and any error messages related to the process, by using Clo ... |
| `BatchPutDocument` | `POST /applications/{applicationId}/indices/{indexId}/documents` | - | `applicationId`, `indexId`, `documents` | - | `BatchPutDocumentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds one or more documents to an Amazon Q Business index. You use this API to: ingest your structured and unstructured documents and documents stored in an Amazon S3 bucket into an Amazon Q Business index. add custom ... |
| `CancelSubscription` | `DELETE /applications/{applicationId}/subscriptions/{subscriptionId}` | `idempotent` | `applicationId`, `subscriptionId` | - | `CancelSubscriptionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Unsubscribes a user or a group from their pricing tier in an Amazon Q Business application. An unsubscribed user or group loses all Amazon Q Business feature access at the start of next month. |
| `Chat` | `POST /applications/{applicationId}/conversations` | `idempotency-token` | `applicationId` | `clientToken` | `ChatOutput` | `AccessDeniedException`, `ConflictException`, `ExternalResourceException`, `InternalServerException`, `LicenseNotFoundException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts or continues a streaming Amazon Q Business conversation. |
| `ChatSync` | `POST /applications/{applicationId}/conversations?sync` | `idempotency-token` | `applicationId` | `clientToken` | `ChatSyncOutput` | `AccessDeniedException`, `ConflictException`, `ExternalResourceException`, `InternalServerException`, `LicenseNotFoundException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts or continues a non-streaming Amazon Q Business conversation. |
| `CheckDocumentAccess` | `GET /applications/{applicationId}/index/{indexId}/users/{userId}/documents/{documentId}/check-document-access` | `readonly` | `applicationId`, `indexId`, `userId`, `documentId` | - | `CheckDocumentAccessResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Verifies if a user has access permissions for a specified document and returns the actual ACL attached to the document. Resolves user access on the document via user aliases and groups when verifying user access. |
| `CreateAnonymousWebExperienceUrl` | `POST /applications/{applicationId}/experiences/{webExperienceId}/anonymous-url` | - | `applicationId`, `webExperienceId` | - | `CreateAnonymousWebExperienceUrlResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a unique URL for anonymous Amazon Q Business web experience. This URL can only be used once and must be used within 5 minutes after it's generated. |
| `CreateChatResponseConfiguration` | `POST /applications/{applicationId}/chatresponseconfigurations` | `idempotent`, `idempotency-token` | `applicationId`, `displayName`, `responseConfigurations` | `clientToken` | `CreateChatResponseConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new chat response configuration for an Amazon Q Business application. This operation establishes a set of parameters that define how the system generates and formats responses to user queries in chat intera ... |
| `CreateSubscription` | `POST /applications/{applicationId}/subscriptions` | `idempotent`, `idempotency-token` | `applicationId`, `principal`, `type` | `clientToken` | `CreateSubscriptionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Subscribes an IAM Identity Center user or a group to a pricing tier for an Amazon Q Business application. Amazon Q Business offers two subscription tiers: Q_LITE and Q_BUSINESS . Subscription tier determines feature ... |
| `CreateUser` | `POST /applications/{applicationId}/users` | `idempotent`, `idempotency-token` | `applicationId`, `userId` | `clientToken` | `CreateUserResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a universally unique identifier (UUID) mapped to a list of local user ids within an application. |
| `DeleteAttachment` | `DELETE /applications/{applicationId}/conversations/{conversationId}/attachments/{attachmentId}` | `idempotent` | `applicationId`, `conversationId`, `attachmentId` | - | `DeleteAttachmentResponse` | `AccessDeniedException`, `InternalServerException`, `LicenseNotFoundException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an attachment associated with a specific Amazon Q Business conversation. |
| `DeleteChatControlsConfiguration` | `DELETE /applications/{applicationId}/chatcontrols` | `idempotent` | `applicationId` | - | `DeleteChatControlsConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes chat controls configured for an existing Amazon Q Business application. |
| `DeleteChatResponseConfiguration` | `DELETE /applications/{applicationId}/chatresponseconfigurations/{chatResponseConfigurationId}` | `idempotent` | `applicationId`, `chatResponseConfigurationId` | - | `DeleteChatResponseConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a specified chat response configuration from an Amazon Q Business application. |
| `DeleteConversation` | `DELETE /applications/{applicationId}/conversations/{conversationId}` | `idempotent` | `conversationId`, `applicationId` | - | `DeleteConversationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LicenseNotFoundException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an Amazon Q Business web experience conversation. |
| `DeleteGroup` | `DELETE /applications/{applicationId}/indices/{indexId}/groups/{groupName}` | `idempotent` | `applicationId`, `indexId`, `groupName` | - | `DeleteGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a group so that all users and sub groups that belong to the group can no longer access documents only available to that group. For example, after deleting the group "Summer Interns", all interns who belonged ... |
| `DeleteUser` | `DELETE /applications/{applicationId}/users/{userId}` | `idempotent` | `applicationId`, `userId` | - | `DeleteUserResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a user by email id. |
| `DisassociatePermission` | `DELETE /applications/{applicationId}/policy/{statementId}` | `idempotent` | `applicationId`, `statementId` | - | `DisassociatePermissionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a permission policy from a Amazon Q Business application, revoking the cross-account access that was previously granted to an ISV. This operation deletes the specified policy statement from the application's ... |
| `GetChatControlsConfiguration` | `GET /applications/{applicationId}/chatcontrols` | `readonly`, `paginated` | `applicationId` | - | `GetChatControlsConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about chat controls configured for an existing Amazon Q Business application. |
| `GetChatResponseConfiguration` | `GET /applications/{applicationId}/chatresponseconfigurations/{chatResponseConfigurationId}` | `readonly` | `applicationId`, `chatResponseConfigurationId` | - | `GetChatResponseConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a specific chat response configuration from an Amazon Q Business application. This operation returns the complete configuration settings and metadata. |
| `GetDocumentContent` | `GET /applications/{applicationId}/index/{indexId}/documents/{documentId}/content` | `readonly` | `applicationId`, `indexId`, `documentId` | - | `GetDocumentContentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the content of a document that was ingested into Amazon Q Business. This API validates user authorization against document ACLs before returning a pre-signed URL for secure document access. You can download ... |
| `GetGroup` | `GET /applications/{applicationId}/indices/{indexId}/groups/{groupName}` | `readonly` | `applicationId`, `indexId`, `groupName` | - | `GetGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes a group by group name. |
| `GetMedia` | `GET /applications/{applicationId}/conversations/{conversationId}/messages/{messageId}/media/{mediaId}` | `readonly` | `applicationId`, `conversationId`, `messageId`, `mediaId` | - | `GetMediaResponse` | `AccessDeniedException`, `InternalServerException`, `LicenseNotFoundException`, `MediaTooLargeException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the image bytes corresponding to a media object. If you have implemented your own application with the Chat and ChatSync APIs, and have enabled content extraction from visual data in Amazon Q Business, you us ... |
| `GetPolicy` | `GET /applications/{applicationId}/policy` | `readonly` | `applicationId` | - | `GetPolicyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the current permission policy for a Amazon Q Business application. The policy is returned as a JSON-formatted string and defines the IAM actions that are allowed or denied for the application's resources. |
| `GetUser` | `GET /applications/{applicationId}/users/{userId}` | `readonly` | `applicationId`, `userId` | - | `GetUserResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Describes the universally unique identifier (UUID) associated with a local user in a data source. |
| `ListAttachments` | `GET /applications/{applicationId}/attachments` | `readonly`, `paginated` | `applicationId` | - | `ListAttachmentsResponse` | `AccessDeniedException`, `InternalServerException`, `LicenseNotFoundException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a list of attachments associated with an Amazon Q Business web experience or a list of attachements associated with a specific Amazon Q Business conversation. |
| `ListChatResponseConfigurations` | `GET /applications/{applicationId}/chatresponseconfigurations` | `readonly`, `paginated` | `applicationId` | - | `ListChatResponseConfigurationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of all chat response configurations available in a specified Amazon Q Business application. This operation returns summary information about each configuration to help administrators manage and selec ... |
| `ListConversations` | `GET /applications/{applicationId}/conversations` | `readonly`, `paginated` | `applicationId` | - | `ListConversationsResponse` | `AccessDeniedException`, `InternalServerException`, `LicenseNotFoundException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists one or more Amazon Q Business conversations. |
| `ListDataSourceSyncJobs` | `GET /applications/{applicationId}/indices/{indexId}/datasources/{dataSourceId}/syncjobs` | `readonly`, `paginated` | `dataSourceId`, `applicationId`, `indexId` | - | `ListDataSourceSyncJobsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get information about an Amazon Q Business data source connector synchronization. |
| `ListDocuments` | `GET /applications/{applicationId}/index/{indexId}/documents` | `readonly`, `paginated` | `applicationId`, `indexId` | - | `ListDocumentsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | A list of documents attached to an index. |
| `ListGroups` | `GET /applications/{applicationId}/indices/{indexId}/groups` | `readonly`, `paginated` | `applicationId`, `indexId`, `updatedEarlierThan` | - | `ListGroupsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provides a list of groups that are mapped to users. |
| `ListMessages` | `GET /applications/{applicationId}/conversations/{conversationId}` | `readonly`, `paginated` | `conversationId`, `applicationId` | - | `ListMessagesResponse` | `AccessDeniedException`, `InternalServerException`, `LicenseNotFoundException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a list of messages associated with an Amazon Q Business web experience. |
| `ListPluginActions` | `GET /applications/{applicationId}/plugins/{pluginId}/actions` | `readonly`, `paginated` | `applicationId`, `pluginId` | - | `ListPluginActionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists configured Amazon Q Business actions for a specific plugin in an Amazon Q Business application. |
| `ListPluginTypeActions` | `GET /pluginTypes/{pluginType}/actions` | `readonly`, `paginated` | `pluginType` | - | `ListPluginTypeActionsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists configured Amazon Q Business actions for any plugin type—both built-in and custom. |
| `ListPluginTypeMetadata` | `GET /pluginTypeMetadata` | `readonly`, `paginated` | - | - | `ListPluginTypeMetadataResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists metadata for all Amazon Q Business plugin types. |
| `ListSubscriptions` | `GET /applications/{applicationId}/subscriptions` | `readonly`, `paginated` | `applicationId` | - | `ListSubscriptionsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all subscriptions created in an Amazon Q Business application. |
| `ListTagsForResource` | `GET /v1/tags/{resourceARN}` | `readonly` | `resourceARN` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a list of tags associated with a specified resource. Amazon Q Business applications and data sources can have tags associated with them. |
| `PutFeedback` | `POST /applications/{applicationId}/conversations/{conversationId}/messages/{messageId}/feedback` | - | `applicationId`, `conversationId`, `messageId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables your end user to provide feedback on their Amazon Q Business generated chat responses. |
| `PutGroup` | `PUT /applications/{applicationId}/indices/{indexId}/groups` | `idempotent` | `applicationId`, `indexId`, `groupName`, `type`, `groupMembers` | - | `PutGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Create, or updates, a mapping of users—who have access to a document—to groups. You can also map sub groups to groups. For example, the group "Company Intellectual Property Teams" includes sub groups "Research" and " ... |
| `SearchRelevantContent` | `POST /applications/{applicationId}/relevant-content` | `paginated` | `applicationId`, `queryText`, `contentSource` | - | `SearchRelevantContentResponse` | `AccessDeniedException`, `InternalServerException`, `LicenseNotFoundException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Searches for relevant content in a Amazon Q Business application based on a query. This operation takes a search query text, the Amazon Q Business application identifier, and optional filters (such as content source ... |
| `StartDataSourceSyncJob` | `POST /applications/{applicationId}/indices/{indexId}/datasources/{dataSourceId}/startsync` | - | `dataSourceId`, `applicationId`, `indexId` | - | `StartDataSourceSyncJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Starts a data source connector synchronization job. If a synchronization job is already in progress, Amazon Q Business returns a ConflictException . |
| `StopDataSourceSyncJob` | `POST /applications/{applicationId}/indices/{indexId}/datasources/{dataSourceId}/stopsync` | - | `dataSourceId`, `applicationId`, `indexId` | - | `StopDataSourceSyncJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops an Amazon Q Business data source connector synchronization job already in progress. |
| `TagResource` | `POST /v1/tags/{resourceARN}` | `idempotent` | `resourceARN`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds the specified tag to the specified Amazon Q Business application or data source resource. If the tag already exists, the existing value is replaced with the new value. |
| `UntagResource` | `DELETE /v1/tags/{resourceARN}` | `idempotent` | `resourceARN`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a tag from an Amazon Q Business application or a data source. |
| `UpdateChatControlsConfiguration` | `PATCH /applications/{applicationId}/chatcontrols` | `idempotent`, `idempotency-token` | `applicationId` | `clientToken` | `UpdateChatControlsConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates a set of chat controls configured for an existing Amazon Q Business application. |
| `UpdateChatResponseConfiguration` | `PUT /applications/{applicationId}/chatresponseconfigurations/{chatResponseConfigurationId}` | `idempotent`, `idempotency-token` | `applicationId`, `chatResponseConfigurationId`, `responseConfigurations` | `clientToken` | `UpdateChatResponseConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing chat response configuration in an Amazon Q Business application. This operation allows administrators to modify configuration settings, display name, and response parameters to refine how the syst ... |
| `UpdateSubscription` | `PUT /applications/{applicationId}/subscriptions/{subscriptionId}` | `idempotent` | `applicationId`, `subscriptionId`, `type` | - | `UpdateSubscriptionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the pricing tier for an Amazon Q Business subscription. Upgrades are instant. Downgrades apply at the start of the next month. Subscription tier determines feature access for the user. For more information on ... |
| `UpdateUser` | `PUT /applications/{applicationId}/users/{userId}` | `idempotent` | `applicationId`, `userId` | - | `UpdateUserResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates a information associated with a user id. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `Chat` | - | `userId -> userId`, `userGroups -> userGroups`, `conversationId -> conversationId`, `parentMessageId -> parentMessageId`, `clientToken -> clientToken` | - | `inputStream` |
| `ChatSync` | - | `userId -> userId`, `userGroups -> userGroups` | - | - |
| `CheckDocumentAccess` | - | `dataSourceId -> dataSourceId` | - | - |
| `DeleteAttachment` | - | `userId -> userId` | - | - |
| `DeleteConversation` | - | `userId -> userId` | - | - |
| `DeleteGroup` | - | `dataSourceId -> dataSourceId` | - | - |
| `GetChatControlsConfiguration` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `GetDocumentContent` | - | `dataSourceId -> dataSourceId`, `outputFormat -> outputFormat` | - | - |
| `GetGroup` | - | `dataSourceId -> dataSourceId` | - | - |
| `ListAttachments` | - | `conversationId -> conversationId`, `userId -> userId`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListChatResponseConfigurations` | - | `maxResults -> maxResults`, `nextToken -> nextToken` | - | - |
| `ListConversations` | - | `userId -> userId`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListDataSourceSyncJobs` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `startTime -> startTime`, `endTime -> endTime`, `statusFilter -> syncStatus` | - | - |
| `ListDocuments` | - | `dataSourceIds -> dataSourceIds`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListGroups` | - | `updatedEarlierThan -> updatedEarlierThan`, `dataSourceId -> dataSourceId`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListMessages` | - | `userId -> userId`, `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListPluginActions` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListPluginTypeActions` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListPluginTypeMetadata` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListSubscriptions` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `PutFeedback` | - | `userId -> userId` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have access to perform this action. Make sure you have the required permission policies and user accounts and try again. |
| `ConflictException` | `structure` | message, resourceId, resourceType | You are trying to perform an action that conflicts with the current status of your resource. Fix any inconsistencies with your resources and try again. |
| `ExternalResourceException` | `structure` | message | An external resource that you configured with your application is returning errors and preventing this operation from succeeding. Fix those errors and try a ... |
| `InternalServerException` | `structure` | message | An issue occurred with the internal server used for your Amazon Q Business service. Wait some minutes and try again, or contact Support for help. |
| `LicenseNotFoundException` | `structure` | message | You don't have permissions to perform the action because your license is inactive. Ask your admin to activate your license and try again after your licence ... |
| `MediaTooLargeException` | `structure` | message | The requested media object is too large to be returned. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | The application or plugin resource you want to use doesn’t exist. Make sure you have provided the correct resource and try again. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType | You have exceeded the set limits for your Amazon Q Business service. |
| `ThrottlingException` | `structure` | message | The request was denied due to throttling. Reduce the number of requests and try again. |
| `ValidationException` | `structure` | message, reason, fields | The input doesn't meet the constraints set by the Amazon Q Business service. Provide the correct input and try again. |
| `AssociatePermissionRequest` | `structure` | applicationId, statementId, actions, conditions, principal | - |
| `AssociatePermissionResponse` | `structure` | statement | - |
| `BatchDeleteDocumentRequest` | `structure` | applicationId, indexId, documents, dataSourceSyncId | - |
| `BatchDeleteDocumentResponse` | `structure` | failedDocuments | - |
| `BatchPutDocumentRequest` | `structure` | applicationId, indexId, documents, roleArn, dataSourceSyncId | - |
| `BatchPutDocumentResponse` | `structure` | failedDocuments | - |
| `CancelSubscriptionRequest` | `structure` | applicationId, subscriptionId | - |
| `CancelSubscriptionResponse` | `structure` | subscriptionArn, currentSubscription, nextSubscription | - |
| `ChatInput` | `structure` | applicationId, userId, userGroups, conversationId, parentMessageId, clientToken, inputStream | - |
| `ChatOutput` | `structure` | outputStream | - |
| `ChatSyncInput` | `structure` | applicationId, userId, userGroups, userMessage, attachments, actionExecution, authChallengeResponse, conversationId, parentMessageId, attributeFilter, chatMode, chatModeConfiguration, ... (+1) | - |
| `ChatSyncOutput` | `structure` | conversationId, systemMessage, systemMessageId, userMessageId, actionReview, authChallengeRequest, sourceAttributions, failedAttachments | - |
| `CheckDocumentAccessRequest` | `structure` | applicationId, indexId, userId, documentId, dataSourceId | - |
| `CheckDocumentAccessResponse` | `structure` | userGroups, userAliases, hasAccess, documentAcl | - |
| `CreateAnonymousWebExperienceUrlRequest` | `structure` | applicationId, webExperienceId, sessionDurationInMinutes | - |
| `CreateAnonymousWebExperienceUrlResponse` | `structure` | anonymousUrl | - |
| `CreateChatResponseConfigurationRequest` | `structure` | applicationId, displayName, clientToken, responseConfigurations, tags | - |
| `CreateChatResponseConfigurationResponse` | `structure` | chatResponseConfigurationId, chatResponseConfigurationArn | - |
| `CreateSubscriptionRequest` | `structure` | applicationId, principal, type, clientToken | - |
| `CreateSubscriptionResponse` | `structure` | subscriptionId, subscriptionArn, currentSubscription, nextSubscription | - |
| `CreateUserRequest` | `structure` | applicationId, userId, userAliases, clientToken | - |
| `CreateUserResponse` | `structure` | **empty (no members)** | - |
| `DeleteAttachmentRequest` | `structure` | applicationId, conversationId, attachmentId, userId | - |
| `DeleteAttachmentResponse` | `structure` | **empty (no members)** | - |
| `DeleteChatControlsConfigurationRequest` | `structure` | applicationId | - |
| `DeleteChatControlsConfigurationResponse` | `structure` | **empty (no members)** | - |
| `DeleteChatResponseConfigurationRequest` | `structure` | applicationId, chatResponseConfigurationId | - |
| `DeleteChatResponseConfigurationResponse` | `structure` | **empty (no members)** | - |
| `DeleteConversationRequest` | `structure` | conversationId, applicationId, userId | - |
| `DeleteConversationResponse` | `structure` | **empty (no members)** | - |
| `APISchemaType` | `enum` | OPEN_API_V3 | - |
| `ActionPayloadFieldType` | `enum` | STRING, NUMBER, ARRAY, BOOLEAN | - |
| `ApplicationStatus` | `enum` | CREATING, ACTIVE, DELETING, FAILED, UPDATING | - |
| `AttachmentStatus` | `enum` | FAILED, SUCCESS | - |
| `AttachmentsControlMode` | `enum` | ENABLED, DISABLED | - |
| `AttributeType` | `enum` | STRING, STRING_LIST, NUMBER, DATE | - |
| `AttributeValueOperator` | `enum` | DELETE | - |
| `AudioExtractionStatus` | `enum` | ENABLED, DISABLED | - |
| `AudioExtractionType` | `enum` | TRANSCRIPT, SUMMARY | - |
| `AutoSubscriptionStatus` | `enum` | ENABLED, DISABLED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
