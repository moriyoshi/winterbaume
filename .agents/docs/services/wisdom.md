# Amazon Connect Wisdom Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Connect Wisdom delivers agents the information they need to solve customer issues as they're actively speaking with customers. Agents can search across connected repositories from within their agent desktop to find answers quickly. Use Amazon Connect Wisdom to create an assistant and a knowledge base, for example, or manage content by uploading custom files.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Connect Wisdom Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon Connect Wisdom Service by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for Amazon Connect Wisdom Service resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Connect Wisdom Service workflows in the local mock. Key resources include `Assistant`, `AssistantAssociation`, `Content`, `KnowledgeBase`, `QuickResponse`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Search` operation families, including `GetAssistant`, `GetAssistantAssociation`, `GetContent`, `GetContentSummary`, `ListAssistantAssociations`, `ListAssistants`.

## Service Identity and Protocol

- AWS model slug: `wisdom`
- AWS SDK for Rust slug: `wisdom`
- Model version: `2020-10-19`
- Model file: `vendor/api-models-aws/models/wisdom/service/2020-10-19/wisdom-2020-10-19.json`
- SDK ID: `Wisdom`
- Endpoint prefix: `-`
- ARN namespace: `wisdom`
- CloudFormation name: `Wisdom`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (9), `List` (7), `Create` (6), `Delete` (6), `Search` (3), `Update` (3), `Start` (2), `Notify` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateAssistant`, `CreateAssistantAssociation`, `CreateContent`, `CreateKnowledgeBase`, `CreateQuickResponse`, `CreateSession`, `DeleteAssistant`, `DeleteAssistantAssociation`, `DeleteContent`, `DeleteImportJob`, `DeleteKnowledgeBase`, `DeleteQuickResponse`, `RemoveKnowledgeBaseTemplateUri`, `StartContentUpload`, `StartImportJob`, `TagResource`, `UntagResource`, `UpdateContent`, `UpdateKnowledgeBaseTemplateUri`, `UpdateQuickResponse`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAssistant`, `GetAssistantAssociation`, `GetContent`, `GetContentSummary`, `GetImportJob`, `GetKnowledgeBase`, `GetQuickResponse`, `GetRecommendations`, `GetSession`, `ListAssistantAssociations`, `ListAssistants`, `ListContents`, `ListImportJobs`, `ListKnowledgeBases`, `ListQuickResponses`, `ListTagsForResource`, `QueryAssistant`, `SearchContent`, `SearchQuickResponses`, `SearchSessions`.
- Pagination is modelled for 10 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 16 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DeleteImportJob`, `GetImportJob`, `ListImportJobs`, `StartContentUpload`, `StartImportJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 41 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `SQS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Assistant` | `assistantId` | create: `CreateAssistant`; read: `GetAssistant`; delete: `DeleteAssistant`; list: `ListAssistants` | `GetRecommendations`, `NotifyRecommendationsReceived`, `QueryAssistant`, `SearchSessions` | - |
| `AssistantAssociation` | `assistantAssociationId`, `assistantId` | create: `CreateAssistantAssociation`; read: `GetAssistantAssociation`; delete: `DeleteAssistantAssociation`; list: `ListAssistantAssociations` | - | - |
| `Content` | `contentId`, `knowledgeBaseId` | create: `CreateContent`; read: `GetContent`; update: `UpdateContent`; delete: `DeleteContent`; list: `ListContents` | `GetContentSummary` | - |
| `KnowledgeBase` | `knowledgeBaseId` | create: `CreateKnowledgeBase`; read: `GetKnowledgeBase`; delete: `DeleteKnowledgeBase`; list: `ListKnowledgeBases` | `DeleteImportJob`, `GetImportJob`, `ListImportJobs`, `RemoveKnowledgeBaseTemplateUri`, `SearchContent`, `SearchQuickResponses`, `StartContentUpload`, `StartImportJob`, `UpdateKnowledgeBaseTemplateUri` | - |
| `QuickResponse` | `knowledgeBaseId`, `quickResponseId` | create: `CreateQuickResponse`; read: `GetQuickResponse`; update: `UpdateQuickResponse`; delete: `DeleteQuickResponse`; list: `ListQuickResponses` | - | - |
| `Session` | `assistantId`, `sessionId` | create: `CreateSession`; read: `GetSession` | - | - |
## Operation Groups

### Get

- Operations: `GetAssistant`, `GetAssistantAssociation`, `GetContent`, `GetContentSummary`, `GetImportJob`, `GetKnowledgeBase`, `GetQuickResponse`, `GetRecommendations`, `GetSession`
- Traits: `readonly` (9)
- Common required input members in this group: `assistantAssociationId`, `assistantId`, `contentId`, `importJobId`, `knowledgeBaseId`, `quickResponseId`, `sessionId`

### List

- Operations: `ListAssistantAssociations`, `ListAssistants`, `ListContents`, `ListImportJobs`, `ListKnowledgeBases`, `ListQuickResponses`, `ListTagsForResource`
- Traits: `paginated` (6), `readonly` (7)
- Common required input members in this group: `assistantId`, `knowledgeBaseId`, `resourceArn`

### Create

- Operations: `CreateAssistant`, `CreateAssistantAssociation`, `CreateContent`, `CreateKnowledgeBase`, `CreateQuickResponse`, `CreateSession`
- Traits: `idempotency-token` (6), `idempotent` (6)
- Common required input members in this group: `assistantId`, `association`, `associationType`, `content`, `knowledgeBaseId`, `knowledgeBaseType`, `name`, `type`, `uploadId`

### Delete

- Operations: `DeleteAssistant`, `DeleteAssistantAssociation`, `DeleteContent`, `DeleteImportJob`, `DeleteKnowledgeBase`, `DeleteQuickResponse`
- Traits: `idempotent` (6)
- Common required input members in this group: `assistantAssociationId`, `assistantId`, `contentId`, `importJobId`, `knowledgeBaseId`, `quickResponseId`

### Search

- Operations: `SearchContent`, `SearchQuickResponses`, `SearchSessions`
- Traits: `paginated` (3), `readonly` (2)
- Common required input members in this group: `assistantId`, `knowledgeBaseId`, `searchExpression`

### Update

- Operations: `UpdateContent`, `UpdateKnowledgeBaseTemplateUri`, `UpdateQuickResponse`
- Common required input members in this group: `contentId`, `knowledgeBaseId`, `quickResponseId`, `templateUri`

### Start

- Operations: `StartContentUpload`, `StartImportJob`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `contentType`, `importJobType`, `knowledgeBaseId`, `uploadId`

### Notify

- Operations: `NotifyRecommendationsReceived`
- Traits: `idempotent` (1)
- Common required input members in this group: `assistantId`, `recommendationIds`, `sessionId`

### Query

- Operations: `QueryAssistant`
- Traits: `paginated` (1), `readonly` (1)
- Common required input members in this group: `assistantId`, `queryText`

### Remove

- Operations: `RemoveKnowledgeBaseTemplateUri`
- Common required input members in this group: `knowledgeBaseId`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateAssistant` | `POST /assistants` | `idempotent`, `idempotency-token` | `name`, `type` | `clientToken` | `CreateAssistantResponse` | `AccessDeniedException`, `ConflictException`, `ServiceQuotaExceededException`, `ValidationException` | Creates an Amazon Connect Wisdom assistant. |
| `CreateAssistantAssociation` | `POST /assistants/{assistantId}/associations` | `idempotent`, `idempotency-token` | `assistantId`, `association`, `associationType` | `clientToken` | `CreateAssistantAssociationResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates an association between an Amazon Connect Wisdom assistant and another resource. Currently, the only supported association is with a knowledge base. |
| `CreateContent` | `POST /knowledgeBases/{knowledgeBaseId}/contents` | `idempotent`, `idempotency-token` | `knowledgeBaseId`, `name`, `uploadId` | `clientToken` | `CreateContentResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates Wisdom content. Before to calling this API, use StartContentUpload to upload an asset. |
| `CreateKnowledgeBase` | `POST /knowledgeBases` | `idempotent`, `idempotency-token` | `knowledgeBaseType`, `name` | `clientToken` | `CreateKnowledgeBaseResponse` | `AccessDeniedException`, `ConflictException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a knowledge base. When using this API, you cannot reuse Amazon AppIntegrations DataIntegrations with external knowledge bases such as Salesforce and ServiceNow. |
| `CreateQuickResponse` | `POST /knowledgeBases/{knowledgeBaseId}/quickResponses` | `idempotent`, `idempotency-token` | `content`, `knowledgeBaseId`, `name` | `clientToken` | `CreateQuickResponseResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a Wisdom quick response. |
| `CreateSession` | `POST /assistants/{assistantId}/sessions` | `idempotent`, `idempotency-token` | `assistantId`, `name` | `clientToken` | `CreateSessionResponse` | `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Creates a session. A session is a contextual container used for generating recommendations. |
| `DeleteAssistant` | `DELETE /assistants/{assistantId}` | `idempotent` | `assistantId` | - | `DeleteAssistantResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Deletes an assistant. |
| `DeleteAssistantAssociation` | `DELETE /assistants/{assistantId}/associations/{assistantAssociationId}` | `idempotent` | `assistantAssociationId`, `assistantId` | - | `DeleteAssistantAssociationResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Deletes an assistant association. |
| `DeleteContent` | `DELETE /knowledgeBases/{knowledgeBaseId}/contents/{contentId}` | `idempotent` | `contentId`, `knowledgeBaseId` | - | `DeleteContentResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Deletes the content. |
| `DeleteImportJob` | `DELETE /knowledgeBases/{knowledgeBaseId}/importJobs/{importJobId}` | `idempotent` | `importJobId`, `knowledgeBaseId` | - | `DeleteImportJobResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Deletes the quick response import job. |
| `DeleteKnowledgeBase` | `DELETE /knowledgeBases/{knowledgeBaseId}` | `idempotent` | `knowledgeBaseId` | - | `DeleteKnowledgeBaseResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ValidationException` | Deletes the knowledge base. When you use this API to delete an external knowledge base such as Salesforce or ServiceNow, you must also delete the Amazon AppIntegrations DataIntegration. |
| `DeleteQuickResponse` | `DELETE /knowledgeBases/{knowledgeBaseId}/quickResponses/{quickResponseId}` | `idempotent` | `knowledgeBaseId`, `quickResponseId` | - | `DeleteQuickResponseResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Deletes a quick response. |
| `GetAssistant` | `GET /assistants/{assistantId}` | `readonly` | `assistantId` | - | `GetAssistantResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Retrieves information about an assistant. |
| `GetAssistantAssociation` | `GET /assistants/{assistantId}/associations/{assistantAssociationId}` | `readonly` | `assistantAssociationId`, `assistantId` | - | `GetAssistantAssociationResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Retrieves information about an assistant association. |
| `GetContent` | `GET /knowledgeBases/{knowledgeBaseId}/contents/{contentId}` | `readonly` | `contentId`, `knowledgeBaseId` | - | `GetContentResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Retrieves content, including a pre-signed URL to download the content. |
| `GetContentSummary` | `GET /knowledgeBases/{knowledgeBaseId}/contents/{contentId}/summary` | `readonly` | `contentId`, `knowledgeBaseId` | - | `GetContentSummaryResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Retrieves summary information about the content. |
| `GetImportJob` | `GET /knowledgeBases/{knowledgeBaseId}/importJobs/{importJobId}` | `readonly` | `importJobId`, `knowledgeBaseId` | - | `GetImportJobResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Retrieves the started import job. |
| `GetKnowledgeBase` | `GET /knowledgeBases/{knowledgeBaseId}` | `readonly` | `knowledgeBaseId` | - | `GetKnowledgeBaseResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Retrieves information about the knowledge base. |
| `GetQuickResponse` | `GET /knowledgeBases/{knowledgeBaseId}/quickResponses/{quickResponseId}` | `readonly` | `knowledgeBaseId`, `quickResponseId` | - | `GetQuickResponseResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Retrieves the quick response. |
| `GetRecommendations` | `GET /assistants/{assistantId}/sessions/{sessionId}/recommendations` | `readonly` | `assistantId`, `sessionId` | - | `GetRecommendationsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Retrieves recommendations for the specified session. To avoid retrieving the same recommendations in subsequent calls, use NotifyRecommendationsReceived. |
| `GetSession` | `GET /assistants/{assistantId}/sessions/{sessionId}` | `readonly` | `assistantId`, `sessionId` | - | `GetSessionResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Retrieves information for a specified session. |
| `ListAssistantAssociations` | `GET /assistants/{assistantId}/associations` | `readonly`, `paginated` | `assistantId` | - | `ListAssistantAssociationsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Lists information about assistant associations. |
| `ListAssistants` | `GET /assistants` | `readonly`, `paginated` | - | - | `ListAssistantsResponse` | `AccessDeniedException`, `ValidationException` | Lists information about assistants. |
| `ListContents` | `GET /knowledgeBases/{knowledgeBaseId}/contents` | `readonly`, `paginated` | `knowledgeBaseId` | - | `ListContentsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Lists the content. |
| `ListImportJobs` | `GET /knowledgeBases/{knowledgeBaseId}/importJobs` | `readonly`, `paginated` | `knowledgeBaseId` | - | `ListImportJobsResponse` | `AccessDeniedException`, `ValidationException` | Lists information about import jobs. |
| `ListKnowledgeBases` | `GET /knowledgeBases` | `readonly`, `paginated` | - | - | `ListKnowledgeBasesResponse` | `AccessDeniedException`, `ValidationException` | Lists the knowledge bases. |
| `ListQuickResponses` | `GET /knowledgeBases/{knowledgeBaseId}/quickResponses` | `readonly`, `paginated` | `knowledgeBaseId` | - | `ListQuickResponsesResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Lists information about quick response. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | Lists the tags for the specified resource. |
| `NotifyRecommendationsReceived` | `POST /assistants/{assistantId}/sessions/{sessionId}/recommendations/notify` | `idempotent` | `assistantId`, `recommendationIds`, `sessionId` | - | `NotifyRecommendationsReceivedResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Removes the specified recommendations from the specified assistant's queue of newly available recommendations. You can use this API in conjunction with GetRecommendations and a `waitTimeSeconds` input for long-polling behavior and avoiding duplicate... |
| `QueryAssistant` | `POST /assistants/{assistantId}/query` | `readonly`, `paginated` | `assistantId`, `queryText` | - | `QueryAssistantResponse` | `AccessDeniedException`, `RequestTimeoutException`, `ResourceNotFoundException`, `ValidationException` | Performs a manual search against the specified assistant. To retrieve recommendations for an assistant, use GetRecommendations. |
| `RemoveKnowledgeBaseTemplateUri` | `DELETE /knowledgeBases/{knowledgeBaseId}/templateUri` | - | `knowledgeBaseId` | - | `RemoveKnowledgeBaseTemplateUriResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Removes a URI template from a knowledge base. |
| `SearchContent` | `POST /knowledgeBases/{knowledgeBaseId}/search` | `readonly`, `paginated` | `knowledgeBaseId`, `searchExpression` | - | `SearchContentResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Searches for content in a specified knowledge base. Can be used to get a specific content resource by its name. |
| `SearchQuickResponses` | `POST /knowledgeBases/{knowledgeBaseId}/search/quickResponses` | `paginated` | `knowledgeBaseId`, `searchExpression` | - | `SearchQuickResponsesResponse` | `AccessDeniedException`, `RequestTimeoutException`, `ResourceNotFoundException`, `ValidationException` | Searches existing Wisdom quick responses in a Wisdom knowledge base. |
| `SearchSessions` | `POST /assistants/{assistantId}/searchSessions` | `readonly`, `paginated` | `assistantId`, `searchExpression` | - | `SearchSessionsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Searches for sessions. |
| `StartContentUpload` | `POST /knowledgeBases/{knowledgeBaseId}/upload` | - | `contentType`, `knowledgeBaseId` | - | `StartContentUploadResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Get a URL to upload content to a knowledge base. To upload content, first make a PUT request to the returned URL with your file, making sure to include the required headers. |
| `StartImportJob` | `POST /knowledgeBases/{knowledgeBaseId}/importJobs` | `idempotent`, `idempotency-token` | `importJobType`, `knowledgeBaseId`, `uploadId` | `clientToken` | `StartImportJobResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Start an asynchronous job to import Wisdom resources from an uploaded source file. Before calling this API, use StartContentUpload to upload an asset that contains the resource data. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `TooManyTagsException` | Adds the specified tags to the specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException` | Removes the specified tags from the specified resource. |
| `UpdateContent` | `POST /knowledgeBases/{knowledgeBaseId}/contents/{contentId}` | - | `contentId`, `knowledgeBaseId` | - | `UpdateContentResponse` | `AccessDeniedException`, `PreconditionFailedException`, `ResourceNotFoundException`, `ValidationException` | Updates information about the content. |
| `UpdateKnowledgeBaseTemplateUri` | `POST /knowledgeBases/{knowledgeBaseId}/templateUri` | - | `knowledgeBaseId`, `templateUri` | - | `UpdateKnowledgeBaseTemplateUriResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Updates the template URI of a knowledge base. This is only supported for knowledge bases of type EXTERNAL. |
| `UpdateQuickResponse` | `POST /knowledgeBases/{knowledgeBaseId}/quickResponses/{quickResponseId}` | - | `knowledgeBaseId`, `quickResponseId` | - | `UpdateQuickResponseResponse` | `AccessDeniedException`, `ConflictException`, `PreconditionFailedException`, `ResourceNotFoundException`, `ValidationException` | Updates an existing Wisdom quick response. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ValidationException` | `structure` | `message` | The input fails to satisfy the constraints specified by a service. |
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceName` | The specified resource does not exist. |
| `ConflictException` | `structure` | `message` | The request could not be processed because of conflict in the current state of the resource. |
| `ServiceQuotaExceededException` | `structure` | `message` | You've exceeded your service quota. |
| `RequestTimeoutException` | `structure` | `message` | The request reached the service more than 15 minutes after the date stamp on the request or more than 15 minutes after the request expiration date (such as for pre-signed URLs)... |
| `PreconditionFailedException` | `structure` | `message` | The provided `revisionId` does not match, indicating the content has been modified since it was last read. |
| `CreateAssistantRequest` | `structure` | `clientToken`, `description`, `name`, `serverSideEncryptionConfiguration`, `tags`, `type` | - |
| `CreateAssistantResponse` | `structure` | `assistant` | - |
| `CreateAssistantAssociationRequest` | `structure` | `assistantId`, `association`, `associationType`, `clientToken`, `tags` | - |
| `CreateAssistantAssociationResponse` | `structure` | `assistantAssociation` | - |
| `CreateContentRequest` | `structure` | `clientToken`, `knowledgeBaseId`, `metadata`, `name`, `overrideLinkOutUri`, `tags`, `title`, `uploadId` | - |
| `CreateContentResponse` | `structure` | `content` | - |
| `CreateKnowledgeBaseRequest` | `structure` | `clientToken`, `description`, `knowledgeBaseType`, `name`, `renderingConfiguration`, `serverSideEncryptionConfiguration`, `sourceConfiguration`, `tags` | - |
| `CreateKnowledgeBaseResponse` | `structure` | `knowledgeBase` | - |
| `CreateQuickResponseRequest` | `structure` | `channels`, `clientToken`, `content`, `contentType`, `description`, `groupingConfiguration`, `isActive`, `knowledgeBaseId`, `language`, `name`, `shortcutKey`, `tags` | - |
| `CreateQuickResponseResponse` | `structure` | `quickResponse` | - |
| `CreateSessionRequest` | `structure` | `assistantId`, `clientToken`, `description`, `name`, `tags` | - |
| `CreateSessionResponse` | `structure` | `session` | - |
| `DeleteAssistantRequest` | `structure` | `assistantId` | - |
| `DeleteAssistantResponse` | `structure` | - | - |
| `DeleteAssistantAssociationRequest` | `structure` | `assistantAssociationId`, `assistantId` | - |
| `DeleteAssistantAssociationResponse` | `structure` | - | - |
| `DeleteContentRequest` | `structure` | `contentId`, `knowledgeBaseId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
