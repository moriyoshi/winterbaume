# Amazon Connect Cases

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Cases actions Cases data types With Amazon Connect Cases, your agents can track and manage customer issues that require multiple interactions, follow-up tasks, and teams in your contact center. A case represents a customer issue. It records the issue, the steps and interactions taken to resolve the issue, and the outcome. For more information, see Amazon Connect Cases in the Amazon Connect Administrator Guide .

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Connect Cases workflows in the local mock. Key resources include `Case`, `CaseRule`, `Domain`, `Field`, `Layout`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Delete`, `Get`, `Update` operation families, including `ListCaseRules`, `ListCasesForContact`, `ListDomains`, `ListFieldOptions`, `CreateCase`, `CreateCaseRule`.

## Service Identity and Protocol

- AWS model slug: `connectcases`
- AWS SDK for Rust slug: `connectcases`
- Model version: `2022-10-03`
- Model file: `vendor/api-models-aws/models/connectcases/service/2022-10-03/connectcases-2022-10-03.json`
- SDK ID: `ConnectCases`
- Endpoint prefix: `cases`
- ARN namespace: `cases`
- CloudFormation name: `-`
- CloudTrail event source: `event-source-placeholder`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (8), `Create` (7), `Delete` (7), `Get` (6), `Update` (5), `Batch` (3), `Search` (3), `Put` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchGetCaseRule`, `BatchGetField`, `BatchPutFieldOptions`, `CreateCase`, `CreateCaseRule`, `CreateDomain`, `CreateField`, `CreateLayout`, `CreateRelatedItem`, `CreateTemplate`, `DeleteCase`, `DeleteCaseRule`, `DeleteDomain`, `DeleteField`, `DeleteLayout`, `DeleteRelatedItem`, `DeleteTemplate`, `PutCaseEventConfiguration`, `TagResource`, `UntagResource`, ... (+5).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `BatchGetCaseRule`, `BatchGetField`, `GetCase`, `GetCaseAuditEvents`, `GetCaseEventConfiguration`, `GetDomain`, `GetLayout`, `GetTemplate`, `ListCaseRules`, `ListCasesForContact`, `ListDomains`, `ListFieldOptions`, `ListFields`, `ListLayouts`, `ListTagsForResource`, `ListTemplates`, `SearchAllRelatedItems`, `SearchCases`, `SearchRelatedItems`.
- Pagination is modelled for 12 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 21 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 42 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EventBridge`, `EC2/VPC`, `STS`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Case` | `caseId`, `domainId` | create: `CreateCase`; read: `GetCase`; update: `UpdateCase`; delete: `DeleteCase` | `ListCasesForContact`, `SearchCases`, `GetCaseAuditEvents` | - |
| `CaseRule` | `caseRuleId`, `domainId` | create: `CreateCaseRule`; update: `UpdateCaseRule`; delete: `DeleteCaseRule`; list: `ListCaseRules` | `BatchGetCaseRule` | - |
| `Domain` | `domainId` | create: `CreateDomain`; read: `GetDomain`; delete: `DeleteDomain`; list: `ListDomains` | `GetCaseEventConfiguration`, `PutCaseEventConfiguration`, `SearchAllRelatedItems` | - |
| `Field` | `domainId`, `fieldId` | create: `CreateField`; update: `UpdateField`; delete: `DeleteField`; list: `ListFields` | `BatchGetField`, `BatchPutFieldOptions`, `ListFieldOptions` | - |
| `Layout` | `domainId`, `layoutId` | create: `CreateLayout`; read: `GetLayout`; update: `UpdateLayout`; delete: `DeleteLayout`; list: `ListLayouts` | - | - |
| `RelatedItem` | `caseId`, `domainId`, `relatedItemId` | create: `CreateRelatedItem`; delete: `DeleteRelatedItem`; list: `SearchRelatedItems` | - | - |
| `Template` | `domainId`, `templateId` | create: `CreateTemplate`; read: `GetTemplate`; update: `UpdateTemplate`; delete: `DeleteTemplate`; list: `ListTemplates` | - | - |
## Operation Groups

### List

- Operations: `ListTagsForResource`
- Traits: `idempotent` (1)
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
| `ListTagsForResource` | `GET /tags/{arn}` | `idempotent` | `arn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists tags for a resource. |
| `TagResource` | `POST /tags/{arn}` | `idempotent` | `arn`, `tags` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds tags to a resource. |
| `UntagResource` | `DELETE /tags/{arn}` | `idempotent` | `arn`, `tagKeys` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Untags a resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message | The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retryin ... |
| `InternalServerException` | `structure` | message, retryAfterSeconds | We couldn't process your request because of an issue with the server. Try again later. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | We couldn't find the requested resource. Check that your resources exists and were created in the same Amazon Web Services Region as your request, and try y ... |
| `ServiceQuotaExceededException` | `structure` | message | The service quota has been exceeded. For a list of service quotas, see Amazon Connect Service Quotas in the Amazon Connect Administrator Guide . |
| `ThrottlingException` | `structure` | message | The rate has been exceeded for this API. Please try again after a few minutes. |
| `ValidationException` | `structure` | message | The request isn't valid. Check the syntax and try again. |
| `ListTagsForResourceRequest` | `structure` | arn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | arn, tags | - |
| `UntagResourceRequest` | `structure` | arn, tagKeys | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
