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

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
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
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | Lists the tags for the specified resource. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `TooManyTagsException` | Adds the specified tags to the specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException` | Removes the specified tags from the specified resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message | The request could not be processed because of conflict in the current state of the resource. For example, if you're using a Create API (such as CreateAssist ... |
| `DependencyFailedException` | `structure` | message | The request failed because it depends on another request that failed. |
| `PreconditionFailedException` | `structure` | message | The provided revisionId does not match, indicating the content has been modified since it was last read. |
| `RequestTimeoutException` | `structure` | message | The request reached the service more than 15 minutes after the date stamp on the request or more than 15 minutes after the request expiration date (such as ... |
| `ResourceNotFoundException` | `structure` | message, resourceName | The specified resource does not exist. |
| `ServiceQuotaExceededException` | `structure` | message | You've exceeded your service quota. To perform the requested action, remove some of the relevant resources, or use service quotas to request a service quota ... |
| `ThrottlingException` | `structure` | message | The throttling limit has been exceeded. |
| `TooManyTagsException` | `structure` | message, resourceName | Amazon Q in Connect throws this exception if you have too many tags in your tag set. |
| `UnauthorizedException` | `structure` | message | You do not have permission to perform this action. |
| `UnprocessableContentException` | `structure` | message | The server has a failure of processing the message |
| `ValidationException` | `structure` | message | The input fails to satisfy the constraints specified by a service. |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
