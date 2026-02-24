# Amazon Q Connect

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Q actions Amazon Q data types Powered by Amazon Bedrock : Amazon Web Services implements automated abuse detection. Because Amazon Q in Connect is built on Amazon Bedrock, users can take full advantage of the controls implemented in Amazon Bedrock to enforce safety, security, and the responsible use of artificial intelligence (AI). Amazon Q in Connect is a generative AI customer service assistant. It is an LLM-enhanced evolution of Amazon Connect Wisdom that delivers real-time recommendations to help contact center agents resolve customer issues quickly and accurately. Amazon Q in Connect automatically detects customer intent during calls and chats using conversational analytics and natural language understanding (NLU).

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Q Connect where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon Q Connect by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for Amazon Q Connect resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon Q Connect workflows in the local mock. Key resources include `AIAgent`, `AIGuardrail`, `AIPrompt`, `Assistant`, `AssistantAssociation`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Delete`, `Get`, `Update` operation families, including `ListAIAgentVersions`, `ListAIAgents`, `ListAIGuardrailVersions`, `ListAIGuardrails`, `CreateAIAgent`, `CreateAIAgentVersion`.

## Service Identity and Protocol

- AWS model slug: `qconnect`
- AWS SDK for Rust slug: `qconnect`
- Model version: `2020-10-19`
- Model file: `vendor/api-models-aws/models/qconnect/service/2020-10-19/qconnect-2020-10-19.json`
- SDK ID: `QConnect`
- Endpoint prefix: `-`
- ARN namespace: `wisdom`
- CloudFormation name: `Wisdom`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (18), `Create` (16), `Delete` (15), `Get` (15), `Update` (11), `Search` (4), `Remove` (2), `Start` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateAIAgent`, `CreateAIAgentVersion`, `CreateAIGuardrail`, `CreateAIGuardrailVersion`, `CreateAIPrompt`, `CreateAIPromptVersion`, `CreateAssistant`, `CreateAssistantAssociation`, `CreateContent`, `CreateContentAssociation`, `CreateKnowledgeBase`, `CreateMessageTemplate`, `CreateMessageTemplateAttachment`, `CreateMessageTemplateVersion`, `CreateQuickResponse`, `CreateSession`, `DeleteAIAgent`, `DeleteAIAgentVersion`, `DeleteAIGuardrail`, `DeleteAIGuardrailVersion`, ... (+29).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAIAgent`, `GetAIGuardrail`, `GetAIPrompt`, `GetAssistant`, `GetAssistantAssociation`, `GetContent`, `GetContentAssociation`, `GetContentSummary`, `GetImportJob`, `GetKnowledgeBase`, `GetMessageTemplate`, `GetNextMessage`, `GetQuickResponse`, `GetRecommendations`, `GetSession`, `ListAIAgentVersions`, `ListAIAgents`, `ListAIGuardrailVersions`, `ListAIGuardrails`, `ListAIPromptVersions`, ... (+19).
- Pagination is modelled for 22 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 40 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DeleteImportJob`, `GetImportJob`, `ListImportJobs`, `StartContentUpload`, `StartImportJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 93 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `SNS`, `SQS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AIAgent` | `aiAgentId`, `assistantId` | create: `CreateAIAgent`; read: `GetAIAgent`; update: `UpdateAIAgent`; delete: `DeleteAIAgent`; list: `ListAIAgents` | `CreateAIAgentVersion`, `DeleteAIAgentVersion`, `ListAIAgentVersions` | - |
| `AIGuardrail` | `aiGuardrailId`, `assistantId` | create: `CreateAIGuardrail`; read: `GetAIGuardrail`; update: `UpdateAIGuardrail`; delete: `DeleteAIGuardrail`; list: `ListAIGuardrails` | `CreateAIGuardrailVersion`, `DeleteAIGuardrailVersion`, `ListAIGuardrailVersions` | - |
| `AIPrompt` | `aiPromptId`, `assistantId` | create: `CreateAIPrompt`; read: `GetAIPrompt`; update: `UpdateAIPrompt`; delete: `DeleteAIPrompt`; list: `ListAIPrompts` | `CreateAIPromptVersion`, `DeleteAIPromptVersion`, `ListAIPromptVersions` | - |
| `Assistant` | `assistantId` | create: `CreateAssistant`; read: `GetAssistant`; delete: `DeleteAssistant`; list: `ListAssistants` | `GetRecommendations`, `NotifyRecommendationsReceived`, `PutFeedback`, `QueryAssistant`, `RemoveAssistantAIAgent`, `Retrieve`, `SearchSessions`, `UpdateAssistantAIAgent` | - |
| `AssistantAssociation` | `assistantAssociationId`, `assistantId` | create: `CreateAssistantAssociation`; read: `GetAssistantAssociation`; delete: `DeleteAssistantAssociation`; list: `ListAssistantAssociations` | - | - |
| `Content` | `contentId`, `knowledgeBaseId` | create: `CreateContent`; read: `GetContent`; update: `UpdateContent`; delete: `DeleteContent`; list: `ListContents` | `GetContentSummary` | - |
| `ContentAssociation` | `contentAssociationId`, `contentId`, `knowledgeBaseId` | create: `CreateContentAssociation`; read: `GetContentAssociation`; delete: `DeleteContentAssociation`; list: `ListContentAssociations` | - | - |
| `KnowledgeBase` | `knowledgeBaseId` | create: `CreateKnowledgeBase`; read: `GetKnowledgeBase`; delete: `DeleteKnowledgeBase`; list: `ListKnowledgeBases` | `DeleteImportJob`, `GetImportJob`, `ListImportJobs`, `RemoveKnowledgeBaseTemplateUri`, `SearchContent`, `SearchMessageTemplates`, `SearchQuickResponses`, `StartContentUpload`, `StartImportJob`, `UpdateKnowledgeBaseTemplateUri` | - |
| `MessageTemplate` | `knowledgeBaseId`, `messageTemplateId` | create: `CreateMessageTemplate`; read: `GetMessageTemplate`; update: `UpdateMessageTemplate`; delete: `DeleteMessageTemplate`; list: `ListMessageTemplates` | `ActivateMessageTemplate`, `CreateMessageTemplateAttachment`, `CreateMessageTemplateVersion`, `DeactivateMessageTemplate`, `DeleteMessageTemplateAttachment`, `ListMessageTemplateVersions`, `RenderMessageTemplate`, `UpdateMessageTemplateMetadata` | - |
| `QuickResponse` | `knowledgeBaseId`, `quickResponseId` | create: `CreateQuickResponse`; read: `GetQuickResponse`; update: `UpdateQuickResponse`; delete: `DeleteQuickResponse`; list: `ListQuickResponses` | - | - |
| `Session` | `assistantId`, `sessionId` | create: `CreateSession`; read: `GetSession`; update: `UpdateSession` | `GetNextMessage`, `ListMessages`, `ListSpans`, `SendMessage`, `UpdateSessionData` | - |
## Operation Groups

### List

- Operations: `ListAIAgentVersions`, `ListAIAgents`, `ListAIGuardrailVersions`, `ListAIGuardrails`, `ListAIPromptVersions`, `ListAIPrompts`, `ListAssistantAssociations`, `ListAssistants`, `ListContentAssociations`, `ListContents`, `ListImportJobs`, `ListKnowledgeBases`, `ListMessageTemplateVersions`, `ListMessageTemplates`, `ListMessages`, `ListQuickResponses`, `ListSpans`, `ListTagsForResource`
- Traits: `paginated` (17), `readonly` (18)
- Common required input members in this group: `aiAgentId`, `aiGuardrailId`, `aiPromptId`, `assistantId`, `contentId`, `knowledgeBaseId`, `messageTemplateId`, `resourceArn`, `sessionId`

### Create

- Operations: `CreateAIAgent`, `CreateAIAgentVersion`, `CreateAIGuardrail`, `CreateAIGuardrailVersion`, `CreateAIPrompt`, `CreateAIPromptVersion`, `CreateAssistant`, `CreateAssistantAssociation`, `CreateContent`, `CreateContentAssociation`, `CreateKnowledgeBase`, `CreateMessageTemplate`, `CreateMessageTemplateAttachment`, `CreateMessageTemplateVersion`, `CreateQuickResponse`, `CreateSession`
- Traits: `idempotency-token` (14), `idempotent` (14)
- Common required input members in this group: `aiAgentId`, `aiGuardrailId`, `aiPromptId`, `apiFormat`, `assistantId`, `association`, `associationType`, `blockedInputMessaging`, `blockedOutputsMessaging`, `body`, `channelSubtype`, `configuration`, `content`, `contentDisposition`, `contentId`, `knowledgeBaseId`, `knowledgeBaseType`, `messageTemplateId`, `modelId`, `name`, `templateConfiguration`, `templateType`, `type`, `uploadId`, ... (+1)

### Delete

- Operations: `DeleteAIAgent`, `DeleteAIAgentVersion`, `DeleteAIGuardrail`, `DeleteAIGuardrailVersion`, `DeleteAIPrompt`, `DeleteAIPromptVersion`, `DeleteAssistant`, `DeleteAssistantAssociation`, `DeleteContent`, `DeleteContentAssociation`, `DeleteImportJob`, `DeleteKnowledgeBase`, `DeleteMessageTemplate`, `DeleteMessageTemplateAttachment`, `DeleteQuickResponse`
- Traits: `idempotent` (15)
- Common required input members in this group: `aiAgentId`, `aiGuardrailId`, `aiPromptId`, `assistantAssociationId`, `assistantId`, `attachmentId`, `contentAssociationId`, `contentId`, `importJobId`, `knowledgeBaseId`, `messageTemplateId`, `quickResponseId`, `versionNumber`

### Get

- Operations: `GetAIAgent`, `GetAIGuardrail`, `GetAIPrompt`, `GetAssistant`, `GetAssistantAssociation`, `GetContent`, `GetContentAssociation`, `GetContentSummary`, `GetImportJob`, `GetKnowledgeBase`, `GetMessageTemplate`, `GetNextMessage`, `GetQuickResponse`, `GetRecommendations`, `GetSession`
- Traits: `readonly` (15)
- Common required input members in this group: `aiAgentId`, `aiGuardrailId`, `aiPromptId`, `assistantAssociationId`, `assistantId`, `contentAssociationId`, `contentId`, `importJobId`, `knowledgeBaseId`, `messageTemplateId`, `nextMessageToken`, `quickResponseId`, `sessionId`

### Update

- Operations: `UpdateAIAgent`, `UpdateAIGuardrail`, `UpdateAIPrompt`, `UpdateAssistantAIAgent`, `UpdateContent`, `UpdateKnowledgeBaseTemplateUri`, `UpdateMessageTemplate`, `UpdateMessageTemplateMetadata`, `UpdateQuickResponse`, `UpdateSession`, `UpdateSessionData`
- Traits: `idempotency-token` (3), `idempotent` (3)
- Common required input members in this group: `aiAgentId`, `aiAgentType`, `aiGuardrailId`, `aiPromptId`, `assistantId`, `blockedInputMessaging`, `blockedOutputsMessaging`, `configuration`, `contentId`, `data`, `knowledgeBaseId`, `messageTemplateId`, `quickResponseId`, `sessionId`, `templateUri`, `visibilityStatus`

### Search

- Operations: `SearchContent`, `SearchMessageTemplates`, `SearchQuickResponses`, `SearchSessions`
- Traits: `paginated` (4), `readonly` (4)
- Common required input members in this group: `assistantId`, `knowledgeBaseId`, `searchExpression`

### Remove

- Operations: `RemoveAssistantAIAgent`, `RemoveKnowledgeBaseTemplateUri`
- Traits: `idempotent` (1)
- Common required input members in this group: `aiAgentType`, `assistantId`, `knowledgeBaseId`

### Start

- Operations: `StartContentUpload`, `StartImportJob`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `contentType`, `importJobType`, `knowledgeBaseId`, `uploadId`

### Activate

- Operations: `ActivateMessageTemplate`
- Common required input members in this group: `knowledgeBaseId`, `messageTemplateId`, `versionNumber`

### Deactivate

- Operations: `DeactivateMessageTemplate`
- Common required input members in this group: `knowledgeBaseId`, `messageTemplateId`, `versionNumber`

### Notify

- Operations: `NotifyRecommendationsReceived`
- Traits: `idempotent` (1)
- Common required input members in this group: `assistantId`, `recommendationIds`, `sessionId`

### Put

- Operations: `PutFeedback`
- Traits: `idempotent` (1)
- Common required input members in this group: `assistantId`, `contentFeedback`, `targetId`, `targetType`

### Query

- Operations: `QueryAssistant`
- Traits: `paginated` (1), `readonly` (1)
- Common required input members in this group: `assistantId`

### Render

- Operations: `RenderMessageTemplate`
- Common required input members in this group: `attributes`, `knowledgeBaseId`, `messageTemplateId`

### Retrieve

- Operations: `Retrieve`
- Traits: `readonly` (1)
- Common required input members in this group: `assistantId`, `retrievalConfiguration`, `retrievalQuery`

### Send

- Operations: `SendMessage`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `assistantId`, `message`, `sessionId`, `type`

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
| `ActivateMessageTemplate` | `POST /knowledgeBases/{knowledgeBaseId}/messageTemplates/{messageTemplateId}/activate` | - | `knowledgeBaseId`, `messageTemplateId`, `versionNumber` | - | `ActivateMessageTemplateResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Activates a specific version of the Amazon Q in Connect message template. After the version is activated, the previous active version will be deactivated automatically. |
| `CreateAIAgent` | `POST /assistants/{assistantId}/aiagents` | `idempotent`, `idempotency-token` | `assistantId`, `configuration`, `name`, `type`, `visibilityStatus` | `clientToken` | `CreateAIAgentResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Creates an Amazon Q in Connect AI Agent. |
| `CreateAIAgentVersion` | `POST /assistants/{assistantId}/aiagents/{aiAgentId}/versions` | `idempotent`, `idempotency-token` | `aiAgentId`, `assistantId` | `clientToken` | `CreateAIAgentVersionResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Creates and Amazon Q in Connect AI Agent version. |
| `CreateAIGuardrail` | `POST /assistants/{assistantId}/aiguardrails` | `idempotent`, `idempotency-token` | `assistantId`, `blockedInputMessaging`, `blockedOutputsMessaging`, `name`, `visibilityStatus` | `clientToken` | `CreateAIGuardrailResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Creates an Amazon Q in Connect AI Guardrail. |
| `CreateAIGuardrailVersion` | `POST /assistants/{assistantId}/aiguardrails/{aiGuardrailId}/versions` | `idempotent`, `idempotency-token` | `aiGuardrailId`, `assistantId` | `clientToken` | `CreateAIGuardrailVersionResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Creates an Amazon Q in Connect AI Guardrail version. |
| `CreateAIPrompt` | `POST /assistants/{assistantId}/aiprompts` | `idempotent`, `idempotency-token` | `apiFormat`, `assistantId`, `modelId`, `name`, `templateConfiguration`, `templateType`, `type`, `visibilityStatus` | `clientToken` | `CreateAIPromptResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Creates an Amazon Q in Connect AI Prompt. |
| `CreateAIPromptVersion` | `POST /assistants/{assistantId}/aiprompts/{aiPromptId}/versions` | `idempotent`, `idempotency-token` | `aiPromptId`, `assistantId` | `clientToken` | `CreateAIPromptVersionResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Creates an Amazon Q in Connect AI Prompt version. |
| `CreateAssistant` | `POST /assistants` | `idempotent`, `idempotency-token` | `name`, `type` | `clientToken` | `CreateAssistantResponse` | `AccessDeniedException`, `ConflictException`, `ServiceQuotaExceededException`, `UnauthorizedException`, `ValidationException` | Creates an Amazon Q in Connect assistant. |
| `CreateAssistantAssociation` | `POST /assistants/{assistantId}/associations` | `idempotent`, `idempotency-token` | `assistantId`, `association`, `associationType` | `clientToken` | `CreateAssistantAssociationResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Creates an association between an Amazon Q in Connect assistant and another resource. Currently, the only supported association is with a knowledge base. |
| `CreateContent` | `POST /knowledgeBases/{knowledgeBaseId}/contents` | `idempotent`, `idempotency-token` | `knowledgeBaseId`, `name`, `uploadId` | `clientToken` | `CreateContentResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `UnauthorizedException`, `ValidationException` | Creates Amazon Q in Connect content. Before to calling this API, use StartContentUpload to upload an asset. |
| `CreateContentAssociation` | `POST /knowledgeBases/{knowledgeBaseId}/contents/{contentId}/associations` | `idempotency-token` | `association`, `associationType`, `contentId`, `knowledgeBaseId` | `clientToken` | `CreateContentAssociationResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Creates an association between a content resource in a knowledge base and step-by-step guides. Step-by-step guides offer instructions to agents for resolving common customer issues. |
| `CreateKnowledgeBase` | `POST /knowledgeBases` | `idempotent`, `idempotency-token` | `knowledgeBaseType`, `name` | `clientToken` | `CreateKnowledgeBaseResponse` | `AccessDeniedException`, `ConflictException`, `ServiceQuotaExceededException`, `UnauthorizedException`, `ValidationException` | Creates a knowledge base. When using this API, you cannot reuse Amazon AppIntegrations DataIntegrations with external knowledge bases such as Salesforce and ServiceNow. |
| `CreateMessageTemplate` | `POST /knowledgeBases/{knowledgeBaseId}/messageTemplates` | `idempotent`, `idempotency-token` | `channelSubtype`, `knowledgeBaseId` | `clientToken` | `CreateMessageTemplateResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an Amazon Q in Connect message template. The name of the message template has to be unique for each knowledge base. |
| `CreateMessageTemplateAttachment` | `POST /knowledgeBases/{knowledgeBaseId}/messageTemplates/{messageTemplateId}/attachments` | `idempotent` | `body`, `contentDisposition`, `knowledgeBaseId`, `messageTemplateId`, `name` | - | `CreateMessageTemplateAttachmentResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Uploads an attachment file to the specified Amazon Q in Connect message template. The name of the message template attachment has to be unique for each message template referenced by the `$LATEST` qualifier. |
| `CreateMessageTemplateVersion` | `POST /knowledgeBases/{knowledgeBaseId}/messageTemplates/{messageTemplateId}/versions` | - | `knowledgeBaseId`, `messageTemplateId` | - | `CreateMessageTemplateVersionResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new Amazon Q in Connect message template version from the current content and configuration of a message template. Versions are immutable and monotonically increasing. |
| `CreateQuickResponse` | `POST /knowledgeBases/{knowledgeBaseId}/quickResponses` | `idempotent`, `idempotency-token` | `content`, `knowledgeBaseId`, `name` | `clientToken` | `CreateQuickResponseResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `UnauthorizedException`, `ValidationException` | Creates an Amazon Q in Connect quick response. |
| `CreateSession` | `POST /assistants/{assistantId}/sessions` | `idempotent`, `idempotency-token` | `assistantId`, `name` | `clientToken` | `CreateSessionResponse` | `AccessDeniedException`, `ConflictException`, `DependencyFailedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Creates a session. A session is a contextual container used for generating recommendations. |
| `DeactivateMessageTemplate` | `POST /knowledgeBases/{knowledgeBaseId}/messageTemplates/{messageTemplateId}/deactivate` | - | `knowledgeBaseId`, `messageTemplateId`, `versionNumber` | - | `DeactivateMessageTemplateResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deactivates a specific version of the Amazon Q in Connect message template . After the version is deactivated, you can no longer use the `$ACTIVE_VERSION` qualifier to reference the version in active status. |
| `DeleteAIAgent` | `DELETE /assistants/{assistantId}/aiagents/{aiAgentId}` | `idempotent` | `aiAgentId`, `assistantId` | - | `DeleteAIAgentResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Deletes an Amazon Q in Connect AI Agent. |
| `DeleteAIAgentVersion` | `DELETE /assistants/{assistantId}/aiagents/{aiAgentId}/versions/{versionNumber}` | `idempotent` | `aiAgentId`, `assistantId`, `versionNumber` | - | `DeleteAIAgentVersionResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Deletes an Amazon Q in Connect AI Agent Version. |
| `DeleteAIGuardrail` | `DELETE /assistants/{assistantId}/aiguardrails/{aiGuardrailId}` | `idempotent` | `aiGuardrailId`, `assistantId` | - | `DeleteAIGuardrailResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Deletes an Amazon Q in Connect AI Guardrail. |
| `DeleteAIGuardrailVersion` | `DELETE /assistants/{assistantId}/aiguardrails/{aiGuardrailId}/versions/{versionNumber}` | `idempotent` | `aiGuardrailId`, `assistantId`, `versionNumber` | - | `DeleteAIGuardrailVersionResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Delete and Amazon Q in Connect AI Guardrail version. |
| `DeleteAIPrompt` | `DELETE /assistants/{assistantId}/aiprompts/{aiPromptId}` | `idempotent` | `aiPromptId`, `assistantId` | - | `DeleteAIPromptResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Deletes an Amazon Q in Connect AI Prompt. |
| `DeleteAIPromptVersion` | `DELETE /assistants/{assistantId}/aiprompts/{aiPromptId}/versions/{versionNumber}` | `idempotent` | `aiPromptId`, `assistantId`, `versionNumber` | - | `DeleteAIPromptVersionResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Delete and Amazon Q in Connect AI Prompt version. |
| `DeleteAssistant` | `DELETE /assistants/{assistantId}` | `idempotent` | `assistantId` | - | `DeleteAssistantResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Deletes an assistant. |
| `DeleteAssistantAssociation` | `DELETE /assistants/{assistantId}/associations/{assistantAssociationId}` | `idempotent` | `assistantAssociationId`, `assistantId` | - | `DeleteAssistantAssociationResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Deletes an assistant association. |
| `DeleteContent` | `DELETE /knowledgeBases/{knowledgeBaseId}/contents/{contentId}` | `idempotent` | `contentId`, `knowledgeBaseId` | - | `DeleteContentResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Deletes the content. |
| `DeleteContentAssociation` | `DELETE /knowledgeBases/{knowledgeBaseId}/contents/{contentId}/associations/{contentAssociationId}` | `idempotent` | `contentAssociationId`, `contentId`, `knowledgeBaseId` | - | `DeleteContentAssociationResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Deletes the content association. For more information about content associations--what they are and when they are used--see Integrate Amazon Q in Connect with step-by-step guides in the Amazon Connect Administrator Guide . |
| `DeleteImportJob` | `DELETE /knowledgeBases/{knowledgeBaseId}/importJobs/{importJobId}` | `idempotent` | `importJobId`, `knowledgeBaseId` | - | `DeleteImportJobResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Deletes the quick response import job. |
| `DeleteKnowledgeBase` | `DELETE /knowledgeBases/{knowledgeBaseId}` | `idempotent` | `knowledgeBaseId` | - | `DeleteKnowledgeBaseResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Deletes the knowledge base. When you use this API to delete an external knowledge base such as Salesforce or ServiceNow, you must also delete the Amazon AppIntegrations DataIntegration. |
| `DeleteMessageTemplate` | `DELETE /knowledgeBases/{knowledgeBaseId}/messageTemplates/{messageTemplateId}` | `idempotent` | `knowledgeBaseId`, `messageTemplateId` | - | `DeleteMessageTemplateResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an Amazon Q in Connect message template entirely or a specific version of the message template if version is supplied in the request. You can provide the message template identifier as ` : ` to delete a specific version of the message template. |
| `DeleteMessageTemplateAttachment` | `DELETE /knowledgeBases/{knowledgeBaseId}/messageTemplates/{messageTemplateId}/attachments/{attachmentId}` | `idempotent` | `attachmentId`, `knowledgeBaseId`, `messageTemplateId` | - | `DeleteMessageTemplateAttachmentResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the attachment file from the Amazon Q in Connect message template that is referenced by `$LATEST` qualifier. Attachments on available message template versions will remain unchanged. |
| `DeleteQuickResponse` | `DELETE /knowledgeBases/{knowledgeBaseId}/quickResponses/{quickResponseId}` | `idempotent` | `knowledgeBaseId`, `quickResponseId` | - | `DeleteQuickResponseResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Deletes a quick response. |
| `GetAIAgent` | `GET /assistants/{assistantId}/aiagents/{aiAgentId}` | `readonly` | `aiAgentId`, `assistantId` | - | `GetAIAgentResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Gets an Amazon Q in Connect AI Agent. |
| `GetAIGuardrail` | `GET /assistants/{assistantId}/aiguardrails/{aiGuardrailId}` | `readonly` | `aiGuardrailId`, `assistantId` | - | `GetAIGuardrailResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Gets the Amazon Q in Connect AI Guardrail. |
| `GetAIPrompt` | `GET /assistants/{assistantId}/aiprompts/{aiPromptId}` | `readonly` | `aiPromptId`, `assistantId` | - | `GetAIPromptResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Gets and Amazon Q in Connect AI Prompt. |
| `GetAssistant` | `GET /assistants/{assistantId}` | `readonly` | `assistantId` | - | `GetAssistantResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Retrieves information about an assistant. |
| `GetAssistantAssociation` | `GET /assistants/{assistantId}/associations/{assistantAssociationId}` | `readonly` | `assistantAssociationId`, `assistantId` | - | `GetAssistantAssociationResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Retrieves information about an assistant association. |
| `GetContent` | `GET /knowledgeBases/{knowledgeBaseId}/contents/{contentId}` | `readonly` | `contentId`, `knowledgeBaseId` | - | `GetContentResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Retrieves content, including a pre-signed URL to download the content. |
| `GetContentAssociation` | `GET /knowledgeBases/{knowledgeBaseId}/contents/{contentId}/associations/{contentAssociationId}` | `readonly` | `contentAssociationId`, `contentId`, `knowledgeBaseId` | - | `GetContentAssociationResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Returns the content association. For more information about content associations--what they are and when they are used--see Integrate Amazon Q in Connect with step-by-step guides in the Amazon Connect Administrator Guide . |
| `GetContentSummary` | `GET /knowledgeBases/{knowledgeBaseId}/contents/{contentId}/summary` | `readonly` | `contentId`, `knowledgeBaseId` | - | `GetContentSummaryResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Retrieves summary information about the content. |
| `GetImportJob` | `GET /knowledgeBases/{knowledgeBaseId}/importJobs/{importJobId}` | `readonly` | `importJobId`, `knowledgeBaseId` | - | `GetImportJobResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Retrieves the started import job. |
| `GetKnowledgeBase` | `GET /knowledgeBases/{knowledgeBaseId}` | `readonly` | `knowledgeBaseId` | - | `GetKnowledgeBaseResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Retrieves information about the knowledge base. |
| `GetMessageTemplate` | `GET /knowledgeBases/{knowledgeBaseId}/messageTemplates/{messageTemplateId}` | `readonly` | `knowledgeBaseId`, `messageTemplateId` | - | `GetMessageTemplateResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Retrieves the Amazon Q in Connect message template. The message template identifier can contain an optional qualifier, for example, ` : `, which is either an actual version number or an Amazon Q Connect managed qualifier `$ACTIVE_VERSION` \| `$LATEST`. |
| `GetNextMessage` | `GET /assistants/{assistantId}/sessions/{sessionId}/messages/next` | `readonly` | `assistantId`, `nextMessageToken`, `sessionId` | - | `GetNextMessageResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `UnprocessableContentException`, `ValidationException` | Retrieves next message on an Amazon Q in Connect session. |
| `GetQuickResponse` | `GET /knowledgeBases/{knowledgeBaseId}/quickResponses/{quickResponseId}` | `readonly` | `knowledgeBaseId`, `quickResponseId` | - | `GetQuickResponseResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Retrieves the quick response. |
| `GetRecommendations` | `GET /assistants/{assistantId}/sessions/{sessionId}/recommendations` | `readonly` | `assistantId`, `sessionId` | - | `GetRecommendationsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | This API will be discontinued starting June 1, 2024. To receive generative responses after March 1, 2024, you will need to create a new Assistant in the Amazon Connect console and integrate the Amazon Q in Connect JavaScript library (amazon-q-connectjs) into... |
| `GetSession` | `GET /assistants/{assistantId}/sessions/{sessionId}` | `readonly` | `assistantId`, `sessionId` | - | `GetSessionResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Retrieves information for a specified session. |
| `ListAIAgentVersions` | `GET /assistants/{assistantId}/aiagents/{aiAgentId}/versions` | `readonly`, `paginated` | `aiAgentId`, `assistantId` | - | `ListAIAgentVersionsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | List AI Agent versions. |
| `ListAIAgents` | `GET /assistants/{assistantId}/aiagents` | `readonly`, `paginated` | `assistantId` | - | `ListAIAgentsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Lists AI Agents. |
| `ListAIGuardrailVersions` | `GET /assistants/{assistantId}/aiguardrails/{aiGuardrailId}/versions` | `readonly`, `paginated` | `aiGuardrailId`, `assistantId` | - | `ListAIGuardrailVersionsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Lists AI Guardrail versions. |
| `ListAIGuardrails` | `GET /assistants/{assistantId}/aiguardrails` | `readonly`, `paginated` | `assistantId` | - | `ListAIGuardrailsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Lists the AI Guardrails available on the Amazon Q in Connect assistant. |
| `ListAIPromptVersions` | `GET /assistants/{assistantId}/aiprompts/{aiPromptId}/versions` | `readonly`, `paginated` | `aiPromptId`, `assistantId` | - | `ListAIPromptVersionsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Lists AI Prompt versions. |
| `ListAIPrompts` | `GET /assistants/{assistantId}/aiprompts` | `readonly`, `paginated` | `assistantId` | - | `ListAIPromptsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Lists the AI Prompts available on the Amazon Q in Connect assistant. |
| `ListAssistantAssociations` | `GET /assistants/{assistantId}/associations` | `readonly`, `paginated` | `assistantId` | - | `ListAssistantAssociationsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Lists information about assistant associations. |
| `ListAssistants` | `GET /assistants` | `readonly`, `paginated` | - | - | `ListAssistantsResponse` | `AccessDeniedException`, `UnauthorizedException`, `ValidationException` | Lists information about assistants. |
| `ListContentAssociations` | `GET /knowledgeBases/{knowledgeBaseId}/contents/{contentId}/associations` | `readonly`, `paginated` | `contentId`, `knowledgeBaseId` | - | `ListContentAssociationsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Lists the content associations. For more information about content associations--what they are and when they are used--see Integrate Amazon Q in Connect with step-by-step guides in the Amazon Connect Administrator Guide . |
| `ListContents` | `GET /knowledgeBases/{knowledgeBaseId}/contents` | `readonly`, `paginated` | `knowledgeBaseId` | - | `ListContentsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Lists the content. |
| `ListImportJobs` | `GET /knowledgeBases/{knowledgeBaseId}/importJobs` | `readonly`, `paginated` | `knowledgeBaseId` | - | `ListImportJobsResponse` | `AccessDeniedException`, `ValidationException` | Lists information about import jobs. |
| `ListKnowledgeBases` | `GET /knowledgeBases` | `readonly`, `paginated` | - | - | `ListKnowledgeBasesResponse` | `AccessDeniedException`, `ValidationException` | Lists the knowledge bases. |
| `ListMessageTemplateVersions` | `GET /knowledgeBases/{knowledgeBaseId}/messageTemplates/{messageTemplateId}/versions` | `readonly`, `paginated` | `knowledgeBaseId`, `messageTemplateId` | - | `ListMessageTemplateVersionsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all the available versions for the specified Amazon Q in Connect message template. |
| `ListMessageTemplates` | `GET /knowledgeBases/{knowledgeBaseId}/messageTemplates` | `readonly`, `paginated` | `knowledgeBaseId` | - | `ListMessageTemplatesResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all the available Amazon Q in Connect message templates for the specified knowledge base. |
| `ListMessages` | `GET /assistants/{assistantId}/sessions/{sessionId}/messages` | `readonly`, `paginated` | `assistantId`, `sessionId` | - | `ListMessagesResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Lists messages on an Amazon Q in Connect session. |
| `ListQuickResponses` | `GET /knowledgeBases/{knowledgeBaseId}/quickResponses` | `readonly`, `paginated` | `knowledgeBaseId` | - | `ListQuickResponsesResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Lists information about quick response. |
| `ListSpans` | `GET /assistants/{assistantId}/sessions/{sessionId}/spans` | `readonly`, `paginated` | `assistantId`, `sessionId` | - | `ListSpansResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Retrieves AI agent execution traces for a session, providing granular visibility into agent orchestration flows, LLM interactions, and tool invocations. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | Lists the tags for the specified resource. |
| `NotifyRecommendationsReceived` | `POST /assistants/{assistantId}/sessions/{sessionId}/recommendations/notify` | `idempotent` | `assistantId`, `recommendationIds`, `sessionId` | - | `NotifyRecommendationsReceivedResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Removes the specified recommendations from the specified assistant's queue of newly available recommendations. You can use this API in conjunction with GetRecommendations and a `waitTimeSeconds` input for long-polling behavior and avoiding duplicate... |
| `PutFeedback` | `PUT /assistants/{assistantId}/feedback` | `idempotent` | `assistantId`, `contentFeedback`, `targetId`, `targetType` | - | `PutFeedbackResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Provides feedback against the specified assistant for the specified target. This API only supports generative targets. |
| `QueryAssistant` | `POST /assistants/{assistantId}/query` | `readonly`, `paginated` | `assistantId` | - | `QueryAssistantResponse` | `AccessDeniedException`, `RequestTimeoutException`, `ResourceNotFoundException`, `ValidationException` | This API will be discontinued starting June 1, 2024. To receive generative responses after March 1, 2024, you will need to create a new Assistant in the Amazon Connect console and integrate the Amazon Q in Connect JavaScript library (amazon-q-connectjs) into... |
| `RemoveAssistantAIAgent` | `DELETE /assistants/{assistantId}/aiagentConfiguration` | `idempotent` | `aiAgentType`, `assistantId` | - | `RemoveAssistantAIAgentResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the AI Agent that is set for use by default on an Amazon Q in Connect Assistant. |
| `RemoveKnowledgeBaseTemplateUri` | `DELETE /knowledgeBases/{knowledgeBaseId}/templateUri` | - | `knowledgeBaseId` | - | `RemoveKnowledgeBaseTemplateUriResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Removes a URI template from a knowledge base. |
| `RenderMessageTemplate` | `POST /knowledgeBases/{knowledgeBaseId}/messageTemplates/{messageTemplateId}/render` | - | `attributes`, `knowledgeBaseId`, `messageTemplateId` | - | `RenderMessageTemplateResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Renders the Amazon Q in Connect message template based on the attribute values provided and generates the message content. For any variable present in the message template, if the attribute value is neither provided in the attribute request parameter nor the... |
| `Retrieve` | `POST /assistants/{assistantId}/retrieve` | `readonly` | `assistantId`, `retrievalConfiguration`, `retrievalQuery` | - | `RetrieveResponse` | `AccessDeniedException`, `ConflictException`, `DependencyFailedException`, `RequestTimeoutException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves content from knowledge sources based on a query. |
| `SearchContent` | `POST /knowledgeBases/{knowledgeBaseId}/search` | `readonly`, `paginated` | `knowledgeBaseId`, `searchExpression` | - | `SearchContentResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Searches for content in a specified knowledge base. Can be used to get a specific content resource by its name. |
| `SearchMessageTemplates` | `POST /knowledgeBases/{knowledgeBaseId}/search/messageTemplates` | `readonly`, `paginated` | `knowledgeBaseId`, `searchExpression` | - | `SearchMessageTemplatesResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Searches for Amazon Q in Connect message templates in the specified knowledge base. |
| `SearchQuickResponses` | `POST /knowledgeBases/{knowledgeBaseId}/search/quickResponses` | `readonly`, `paginated` | `knowledgeBaseId`, `searchExpression` | - | `SearchQuickResponsesResponse` | `AccessDeniedException`, `RequestTimeoutException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Searches existing Amazon Q in Connect quick responses in an Amazon Q in Connect knowledge base. |
| `SearchSessions` | `POST /assistants/{assistantId}/searchSessions` | `readonly`, `paginated` | `assistantId`, `searchExpression` | - | `SearchSessionsResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Searches for sessions. |
| `SendMessage` | `POST /assistants/{assistantId}/sessions/{sessionId}/message` | `idempotent`, `idempotency-token` | `assistantId`, `message`, `sessionId`, `type` | `clientToken` | `SendMessageResponse` | `AccessDeniedException`, `ConflictException`, `DependencyFailedException`, `RequestTimeoutException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Submits a message to the Amazon Q in Connect session. |
| `StartContentUpload` | `POST /knowledgeBases/{knowledgeBaseId}/upload` | - | `contentType`, `knowledgeBaseId` | - | `StartContentUploadResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Get a URL to upload content to a knowledge base. To upload content, first make a PUT request to the returned URL with your file, making sure to include the required headers. |
| `StartImportJob` | `POST /knowledgeBases/{knowledgeBaseId}/importJobs` | `idempotent`, `idempotency-token` | `importJobType`, `knowledgeBaseId`, `uploadId` | `clientToken` | `StartImportJobResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `UnauthorizedException`, `ValidationException` | Start an asynchronous job to import Amazon Q in Connect resources from an uploaded source file. Before calling this API, use StartContentUpload to upload an asset that contains the resource data. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `TooManyTagsException` | Adds the specified tags to the specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException` | Removes the specified tags from the specified resource. |
| `UpdateAIAgent` | `POST /assistants/{assistantId}/aiagents/{aiAgentId}` | `idempotent`, `idempotency-token` | `aiAgentId`, `assistantId`, `visibilityStatus` | `clientToken` | `UpdateAIAgentResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Updates an AI Agent. |
| `UpdateAIGuardrail` | `POST /assistants/{assistantId}/aiguardrails/{aiGuardrailId}` | `idempotent`, `idempotency-token` | `aiGuardrailId`, `assistantId`, `blockedInputMessaging`, `blockedOutputsMessaging`, `visibilityStatus` | `clientToken` | `UpdateAIGuardrailResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Updates an AI Guardrail. |
| `UpdateAIPrompt` | `POST /assistants/{assistantId}/aiprompts/{aiPromptId}` | `idempotent`, `idempotency-token` | `aiPromptId`, `assistantId`, `visibilityStatus` | `clientToken` | `UpdateAIPromptResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Updates an AI Prompt. |
| `UpdateAssistantAIAgent` | `POST /assistants/{assistantId}/aiagentConfiguration` | - | `aiAgentType`, `assistantId`, `configuration` | - | `UpdateAssistantAIAgentResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the AI Agent that is set for use by default on an Amazon Q in Connect Assistant. |
| `UpdateContent` | `POST /knowledgeBases/{knowledgeBaseId}/contents/{contentId}` | - | `contentId`, `knowledgeBaseId` | - | `UpdateContentResponse` | `AccessDeniedException`, `PreconditionFailedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Updates information about the content. |
| `UpdateKnowledgeBaseTemplateUri` | `POST /knowledgeBases/{knowledgeBaseId}/templateUri` | - | `knowledgeBaseId`, `templateUri` | - | `UpdateKnowledgeBaseTemplateUriResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Updates the template URI of a knowledge base. This is only supported for knowledge bases of type EXTERNAL. |
| `UpdateMessageTemplate` | `POST /knowledgeBases/{knowledgeBaseId}/messageTemplates/{messageTemplateId}` | - | `knowledgeBaseId`, `messageTemplateId` | - | `UpdateMessageTemplateResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the Amazon Q in Connect message template. Partial update is supported. |
| `UpdateMessageTemplateMetadata` | `POST /knowledgeBases/{knowledgeBaseId}/messageTemplates/{messageTemplateId}/metadata` | - | `knowledgeBaseId`, `messageTemplateId` | - | `UpdateMessageTemplateMetadataResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the Amazon Q in Connect message template metadata. Note that any modification to the message template’s name, description and grouping configuration will applied to the message template pointed by the `$LATEST` qualifier and all available versions. |
| `UpdateQuickResponse` | `POST /knowledgeBases/{knowledgeBaseId}/quickResponses/{quickResponseId}` | - | `knowledgeBaseId`, `quickResponseId` | - | `UpdateQuickResponseResponse` | `AccessDeniedException`, `ConflictException`, `PreconditionFailedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Updates an existing Amazon Q in Connect quick response. |
| `UpdateSession` | `POST /assistants/{assistantId}/sessions/{sessionId}` | - | `assistantId`, `sessionId` | - | `UpdateSessionResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Updates a session. A session is a contextual container used for generating recommendations. |
| `UpdateSessionData` | `PATCH /assistants/{assistantId}/sessions/{sessionId}/data` | - | `assistantId`, `data`, `sessionId` | - | `UpdateSessionDataResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `UnauthorizedException`, `ValidationException` | Updates the data stored on an Amazon Q in Connect Session. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `ValidationException` | `structure` | `message` | The input fails to satisfy the constraints specified by a service. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceName` | The specified resource does not exist. |
| `UnauthorizedException` | `structure` | `message` | You do not have permission to perform this action. |
| `ThrottlingException` | `structure` | `message` | The throttling limit has been exceeded. |
| `ConflictException` | `structure` | `message` | The request could not be processed because of conflict in the current state of the resource. |
| `ServiceQuotaExceededException` | `structure` | `message` | You've exceeded your service quota. |
| `RequestTimeoutException` | `structure` | `message` | The request reached the service more than 15 minutes after the date stamp on the request or more than 15 minutes after the request expiration date (such as for pre-signed URLs)... |
| `DependencyFailedException` | `structure` | `message` | The request failed because it depends on another request that failed. |
| `PreconditionFailedException` | `structure` | `message` | The provided `revisionId` does not match, indicating the content has been modified since it was last read. |
| `ActivateMessageTemplateRequest` | `structure` | `knowledgeBaseId`, `messageTemplateId`, `versionNumber` | - |
| `ActivateMessageTemplateResponse` | `structure` | `messageTemplateArn`, `messageTemplateId`, `versionNumber` | - |
| `CreateAIAgentRequest` | `structure` | `assistantId`, `clientToken`, `configuration`, `description`, `name`, `tags`, `type`, `visibilityStatus` | - |
| `CreateAIAgentResponse` | `structure` | `aiAgent` | - |
| `CreateAIAgentVersionRequest` | `structure` | `aiAgentId`, `assistantId`, `clientToken`, `modifiedTime` | - |
| `CreateAIAgentVersionResponse` | `structure` | `aiAgent`, `versionNumber` | - |
| `CreateAIGuardrailRequest` | `structure` | `assistantId`, `blockedInputMessaging`, `blockedOutputsMessaging`, `clientToken`, `contentPolicyConfig`, `contextualGroundingPolicyConfig`, `description`, `name`, `sensitiveInformationPolicyConfig`, `tags`, `topicPolicyConfig`, `visibilityStatus`, ... (+1) | - |
| `CreateAIGuardrailResponse` | `structure` | `aiGuardrail` | - |
| `CreateAIGuardrailVersionRequest` | `structure` | `aiGuardrailId`, `assistantId`, `clientToken`, `modifiedTime` | - |
| `CreateAIGuardrailVersionResponse` | `structure` | `aiGuardrail`, `versionNumber` | - |
| `CreateAIPromptRequest` | `structure` | `apiFormat`, `assistantId`, `clientToken`, `description`, `inferenceConfiguration`, `modelId`, `name`, `tags`, `templateConfiguration`, `templateType`, `type`, `visibilityStatus` | - |
| `CreateAIPromptResponse` | `structure` | `aiPrompt` | - |
| `CreateAIPromptVersionRequest` | `structure` | `aiPromptId`, `assistantId`, `clientToken`, `modifiedTime` | - |
| `CreateAIPromptVersionResponse` | `structure` | `aiPrompt`, `versionNumber` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
