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
| `PreconditionFailedException` | `structure` | message | The provided revisionId does not match, indicating the content has been modified since it was last read. |
| `RequestTimeoutException` | `structure` | message | The request reached the service more than 15 minutes after the date stamp on the request or more than 15 minutes after the request expiration date (such as ... |
| `ResourceNotFoundException` | `structure` | message, resourceName | The specified resource does not exist. |
| `ServiceQuotaExceededException` | `structure` | message | You've exceeded your service quota. To perform the requested action, remove some of the relevant resources, or use service quotas to request a service quota ... |
| `TooManyTagsException` | `structure` | message, resourceName | Amazon Connect Wisdom throws this exception if you have too many tags in your tag set. |
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
