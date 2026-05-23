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

- Operations: `ListCaseRules`, `ListCasesForContact`, `ListDomains`, `ListFieldOptions`, `ListFields`, `ListLayouts`, `ListTagsForResource`, `ListTemplates`
- Traits: `idempotent` (1), `paginated` (7), `readonly` (7)
- Common required input members in this group: `arn`, `contactArn`, `domainId`, `fieldId`

### Create

- Operations: `CreateCase`, `CreateCaseRule`, `CreateDomain`, `CreateField`, `CreateLayout`, `CreateRelatedItem`, `CreateTemplate`
- Traits: `idempotency-token` (1), `idempotent` (6)
- Common required input members in this group: `caseId`, `content`, `domainId`, `fields`, `name`, `rule`, `templateId`, `type`

### Delete

- Operations: `DeleteCase`, `DeleteCaseRule`, `DeleteDomain`, `DeleteField`, `DeleteLayout`, `DeleteRelatedItem`, `DeleteTemplate`
- Traits: `idempotent` (7)
- Common required input members in this group: `caseId`, `caseRuleId`, `domainId`, `fieldId`, `layoutId`, `relatedItemId`, `templateId`

### Get

- Operations: `GetCase`, `GetCaseAuditEvents`, `GetCaseEventConfiguration`, `GetDomain`, `GetLayout`, `GetTemplate`
- Traits: `paginated` (2), `readonly` (6)
- Common required input members in this group: `caseId`, `domainId`, `fields`, `layoutId`, `templateId`

### Update

- Operations: `UpdateCase`, `UpdateCaseRule`, `UpdateField`, `UpdateLayout`, `UpdateTemplate`
- Traits: `idempotent` (4)
- Common required input members in this group: `caseId`, `caseRuleId`, `domainId`, `fieldId`, `fields`, `layoutId`, `templateId`

### Batch

- Operations: `BatchGetCaseRule`, `BatchGetField`, `BatchPutFieldOptions`
- Traits: `idempotent` (1), `readonly` (2)
- Common required input members in this group: `caseRules`, `domainId`, `fieldId`, `fields`, `options`

### Search

- Operations: `SearchAllRelatedItems`, `SearchCases`, `SearchRelatedItems`
- Traits: `paginated` (3), `readonly` (3)
- Common required input members in this group: `caseId`, `domainId`

### Put

- Operations: `PutCaseEventConfiguration`
- Common required input members in this group: `domainId`, `eventBridge`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `arn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `arn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchGetCaseRule` | `POST /domains/{domainId}/rules-batch` | `readonly` | `caseRules`, `domainId` | - | `BatchGetCaseRuleResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a batch of case rules. In the Amazon Connect admin website, case rules are known as case field conditions . |
| `BatchGetField` | `POST /domains/{domainId}/fields-batch` | `readonly` | `domainId`, `fields` | - | `BatchGetFieldResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the description for the list of fields in the request parameters. |
| `BatchPutFieldOptions` | `PUT /domains/{domainId}/fields/{fieldId}/options` | `idempotent` | `domainId`, `fieldId`, `options` | - | `BatchPutFieldOptionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates and updates a set of field options for a single select field in a Cases domain. |
| `CreateCase` | `POST /domains/{domainId}/cases` | `idempotent`, `idempotency-token` | `domainId`, `fields`, `templateId` | `clientToken` | `CreateCaseResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | If you provide a value for `PerformedBy.UserArn` you must also have connect:DescribeUser permission on the User ARN resource that you provide Creates a case in the specified Cases domain. Case system and custom fields are taken as an array id/value pairs with... |
| `CreateCaseRule` | `POST /domains/{domainId}/case-rules` | `idempotent` | `domainId`, `name`, `rule` | - | `CreateCaseRuleResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new case rule. In the Amazon Connect admin website, case rules are known as case field conditions . |
| `CreateDomain` | `POST /domains` | `idempotent` | `name` | - | `CreateDomainResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a domain, which is a container for all case data, such as cases, fields, templates and layouts. Each Amazon Connect instance can be associated with only one Cases domain. |
| `CreateField` | `POST /domains/{domainId}/fields` | `idempotent` | `domainId`, `name`, `type` | - | `CreateFieldResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a field in the Cases domain. This field is used to define the case object model (that is, defines what data can be captured on cases) in a Cases domain. |
| `CreateLayout` | `POST /domains/{domainId}/layouts` | - | `content`, `domainId`, `name` | - | `CreateLayoutResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a layout in the Cases domain. Layouts define the following configuration in the top section and More Info tab of the Cases user interface: Fields to display to the users Field ordering Title and Status fields cannot be part of layouts since they are... |
| `CreateRelatedItem` | `POST /domains/{domainId}/cases/{caseId}/related-items/` | `idempotent` | `caseId`, `content`, `domainId`, `type` | - | `CreateRelatedItemResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a related item (comments, tasks, and contacts) and associates it with a case. There's a quota for the number of fields allowed in a Custom type related item. |
| `CreateTemplate` | `POST /domains/{domainId}/templates` | `idempotent` | `domainId`, `name` | - | `CreateTemplateResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a template in the Cases domain. This template is used to define the case object model (that is, to define what data can be captured on cases) in a Cases domain. |
| `DeleteCase` | `DELETE /domains/{domainId}/cases/{caseId}` | `idempotent` | `caseId`, `domainId` | - | `DeleteCaseResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The DeleteCase API permanently deletes a case and all its associated resources from the cases data store. After a successful deletion, you cannot: Retrieve related items Access audit history Perform any operations that require the CaseID This action is... |
| `DeleteCaseRule` | `DELETE /domains/{domainId}/case-rules/{caseRuleId}` | `idempotent` | `caseRuleId`, `domainId` | - | `DeleteCaseRuleResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Deletes a case rule. In the Amazon Connect admin website, case rules are known as case field conditions . |
| `DeleteDomain` | `DELETE /domains/{domainId}` | `idempotent` | `domainId` | - | `DeleteDomainResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a Cases domain. After deleting your domain you must disassociate the deleted domain from your Amazon Connect instance with another API call before being able to use Cases again with this Amazon Connect instance. |
| `DeleteField` | `DELETE /domains/{domainId}/fields/{fieldId}` | `idempotent` | `domainId`, `fieldId` | - | `DeleteFieldResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes a field from a cases template. After a field is deleted: You can still retrieve the field by calling `BatchGetField`. |
| `DeleteLayout` | `DELETE /domains/{domainId}/layouts/{layoutId}` | `idempotent` | `domainId`, `layoutId` | - | `DeleteLayoutResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a layout from a cases template. You can delete up to 100 layouts per domain. |
| `DeleteRelatedItem` | `DELETE /domains/{domainId}/cases/{caseId}/related-items/{relatedItemId}` | `idempotent` | `caseId`, `domainId`, `relatedItemId` | - | `DeleteRelatedItemResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the related item resource under a case. This API cannot be used on a FILE type related attachment. |
| `DeleteTemplate` | `DELETE /domains/{domainId}/templates/{templateId}` | `idempotent` | `domainId`, `templateId` | - | `DeleteTemplateResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a cases template. You can delete up to 100 templates per domain. |
| `GetCase` | `POST /domains/{domainId}/cases/{caseId}` | `readonly`, `paginated` | `caseId`, `domainId`, `fields` | - | `GetCaseResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a specific case if it exists. |
| `GetCaseAuditEvents` | `POST /domains/{domainId}/cases/{caseId}/audit-history` | `readonly`, `paginated` | `caseId`, `domainId` | - | `GetCaseAuditEventsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the audit history about a specific case if it exists. |
| `GetCaseEventConfiguration` | `POST /domains/{domainId}/case-event-configuration` | `readonly` | `domainId` | - | `GetCaseEventConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the case event publishing configuration. |
| `GetDomain` | `POST /domains/{domainId}` | `readonly` | `domainId` | - | `GetDomainResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information about a specific domain if it exists. |
| `GetLayout` | `POST /domains/{domainId}/layouts/{layoutId}` | `readonly` | `domainId`, `layoutId` | - | `GetLayoutResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the details for the requested layout. |
| `GetTemplate` | `POST /domains/{domainId}/templates/{templateId}` | `readonly` | `domainId`, `templateId` | - | `GetTemplateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns the details for the requested template. Other template APIs are: CreateTemplate DeleteTemplate ListTemplates UpdateTemplate |
| `ListCaseRules` | `POST /domains/{domainId}/rules-list/` | `readonly`, `paginated` | `domainId` | - | `ListCaseRulesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all case rules in a Cases domain. In the Amazon Connect admin website, case rules are known as case field conditions . |
| `ListCasesForContact` | `POST /domains/{domainId}/list-cases-for-contact` | `readonly`, `paginated` | `contactArn`, `domainId` | - | `ListCasesForContactResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists cases for a given contact. |
| `ListDomains` | `POST /domains-list` | `readonly`, `paginated` | - | - | `ListDomainsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all cases domains in the Amazon Web Services account. Each list item is a condensed summary object of the domain. |
| `ListFieldOptions` | `POST /domains/{domainId}/fields/{fieldId}/options-list` | `readonly`, `paginated` | `domainId`, `fieldId` | - | `ListFieldOptionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all of the field options for a field identifier in the domain. |
| `ListFields` | `POST /domains/{domainId}/fields-list` | `readonly`, `paginated` | `domainId` | - | `ListFieldsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all fields in a Cases domain. |
| `ListLayouts` | `POST /domains/{domainId}/layouts-list` | `readonly`, `paginated` | `domainId` | - | `ListLayoutsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all layouts in the given cases domain. Each list item is a condensed summary object of the layout. |
| `ListTagsForResource` | `GET /tags/{arn}` | `idempotent` | `arn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists tags for a resource. |
| `ListTemplates` | `POST /domains/{domainId}/templates-list` | `readonly`, `paginated` | `domainId` | - | `ListTemplatesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all of the templates in a Cases domain. Each list item is a condensed summary object of the template. |
| `PutCaseEventConfiguration` | `PUT /domains/{domainId}/case-event-configuration` | - | `domainId`, `eventBridge` | - | `PutCaseEventConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds case event publishing configuration. For a complete list of fields you can add to the event message, see Create case fields in the Amazon Connect Administrator Guide |
| `SearchAllRelatedItems` | `POST /domains/{domainId}/related-items-search` | `readonly`, `paginated` | `domainId` | - | `SearchAllRelatedItemsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Searches for related items across all cases within a domain. This is a global search operation that returns related items from multiple cases, unlike the case-specific SearchRelatedItems API. |
| `SearchCases` | `POST /domains/{domainId}/cases-search` | `readonly`, `paginated` | `domainId` | - | `SearchCasesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Searches for cases within their associated Cases domain. Search results are returned as a paginated list of abridged case documents. |
| `SearchRelatedItems` | `POST /domains/{domainId}/cases/{caseId}/related-items-search` | `readonly`, `paginated` | `caseId`, `domainId` | - | `SearchRelatedItemsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Searches for related items that are associated with a case. If no filters are provided, this returns all related items associated with a case. |
| `TagResource` | `POST /tags/{arn}` | `idempotent` | `arn`, `tags` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds tags to a resource. |
| `UntagResource` | `DELETE /tags/{arn}` | `idempotent` | `arn`, `tagKeys` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Untags a resource. |
| `UpdateCase` | `PUT /domains/{domainId}/cases/{caseId}` | - | `caseId`, `domainId`, `fields` | - | `UpdateCaseResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | If you provide a value for `PerformedBy.UserArn` you must also have connect:DescribeUser permission on the User ARN resource that you provide Updates the values of fields on a case. Fields to be updated are received as an array of id/value pairs identical to... |
| `UpdateCaseRule` | `PUT /domains/{domainId}/case-rules/{caseRuleId}` | `idempotent` | `caseRuleId`, `domainId` | - | `UpdateCaseRuleResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates a case rule. In the Amazon Connect admin website, case rules are known as case field conditions . |
| `UpdateField` | `PUT /domains/{domainId}/fields/{fieldId}` | `idempotent` | `domainId`, `fieldId` | - | `UpdateFieldResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the properties of an existing field. |
| `UpdateLayout` | `PUT /domains/{domainId}/layouts/{layoutId}` | `idempotent` | `domainId`, `layoutId` | - | `UpdateLayoutResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the attributes of an existing layout. If the action is successful, the service sends back an HTTP 200 response with an empty HTTP body. |
| `UpdateTemplate` | `PUT /domains/{domainId}/templates/{templateId}` | `idempotent` | `domainId`, `templateId` | - | `UpdateTemplateResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the attributes of an existing template. The template attributes that can be modified include `name`, `description`, `layoutConfiguration`, `requiredFields`, and `status`. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message`, `retryAfterSeconds` | We couldn't process your request because of an issue with the server. |
| `ThrottlingException` | `structure` | `message` | The rate has been exceeded for this API. |
| `ValidationException` | `structure` | `message` | The request isn't valid. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | We couldn't find the requested resource. |
| `ConflictException` | `structure` | `message` | The requested operation would cause a conflict with the current state of a service resource associated with the request. |
| `ServiceQuotaExceededException` | `structure` | `message` | The service quota has been exceeded. |
| `BatchGetCaseRuleRequest` | `structure` | `caseRules`, `domainId` | - |
| `BatchGetCaseRuleResponse` | `structure` | `caseRules`, `errors`, `unprocessedCaseRules` | - |
| `BatchGetFieldRequest` | `structure` | `domainId`, `fields` | - |
| `BatchGetFieldResponse` | `structure` | `errors`, `fields` | - |
| `BatchPutFieldOptionsRequest` | `structure` | `domainId`, `fieldId`, `options` | - |
| `BatchPutFieldOptionsResponse` | `structure` | `errors` | - |
| `CreateCaseRequest` | `structure` | `clientToken`, `domainId`, `fields`, `performedBy`, `tags`, `templateId` | - |
| `CreateCaseResponse` | `structure` | `caseArn`, `caseId` | - |
| `CreateCaseRuleRequest` | `structure` | `description`, `domainId`, `name`, `rule` | - |
| `CreateCaseRuleResponse` | `structure` | `caseRuleArn`, `caseRuleId` | - |
| `CreateDomainRequest` | `structure` | `name` | - |
| `CreateDomainResponse` | `structure` | `domainArn`, `domainId`, `domainStatus` | - |
| `CreateFieldRequest` | `structure` | `attributes`, `description`, `domainId`, `name`, `type` | - |
| `CreateFieldResponse` | `structure` | `fieldArn`, `fieldId` | - |
| `CreateLayoutRequest` | `structure` | `content`, `domainId`, `name` | - |
| `CreateLayoutResponse` | `structure` | `layoutArn`, `layoutId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
