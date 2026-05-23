# Compute Optimizer Automation

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Automation is a feature within Amazon Web Services Compute Optimizer that enables you to apply optimization recommendations to your Amazon Web Services resources, reducing costs and improving performance. You can apply recommended actions directly or create automation rules that implement recommendations on a recurring schedule when they match your specified criteria. With automation rules, set criteria such as Amazon Web Services Region and Resource Tags to target specific geographies and workloads. Configure rules to run daily, weekly, or monthly, and Compute Optimizer continuously evaluates new recommendations against your criteria. Track automation events over time, examine detailed step history, estimate savings achieved, and reverse actions directly from Compute Optimizer when needed.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Compute Optimizer Automation where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Compute Optimizer Automation by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Compute Optimizer Automation workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Update`, `Associate`, `Create` operation families, including `ListAccounts`, `ListAutomationEventSteps`, `ListAutomationEventSummaries`, `ListAutomationEvents`, `GetAutomationEvent`, `GetAutomationRule`.

## Service Identity and Protocol

- AWS model slug: `compute-optimizer-automation`
- AWS SDK for Rust slug: `computeoptimizerautomation`
- Model version: `2025-09-22`
- Model file: `vendor/api-models-aws/models/compute-optimizer-automation/service/2025-09-22/compute-optimizer-automation-2025-09-22.json`
- SDK ID: `Compute Optimizer Automation`
- Endpoint prefix: `aco-automation`
- ARN namespace: `compute-optimizer`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (10), `Get` (3), `Update` (2), `Associate` (1), `Create` (1), `Delete` (1), `Disassociate` (1), `Rollback` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateAccounts`, `CreateAutomationRule`, `DeleteAutomationRule`, `DisassociateAccounts`, `StartAutomationEvent`, `TagResource`, `UntagResource`, `UpdateAutomationRule`, `UpdateEnrollmentConfiguration`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAutomationEvent`, `GetAutomationRule`, `GetEnrollmentConfiguration`, `ListAccounts`, `ListAutomationEventSteps`, `ListAutomationEventSummaries`, `ListAutomationEvents`, `ListAutomationRulePreview`, `ListAutomationRulePreviewSummaries`, `ListAutomationRules`, `ListRecommendedActionSummaries`, `ListRecommendedActions`, `ListTagsForResource`.
- Pagination is modelled for 9 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 10 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartAutomationEvent`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 23 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListAccounts`, `ListAutomationEventSteps`, `ListAutomationEventSummaries`, `ListAutomationEvents`, `ListAutomationRulePreview`, `ListAutomationRulePreviewSummaries`, `ListAutomationRules`, `ListRecommendedActionSummaries`, `ListRecommendedActions`, `ListTagsForResource`
- Traits: `paginated` (9), `readonly` (4)
- Common required input members in this group: `eventId`, `recommendedActionTypes`, `resourceArn`, `ruleType`

### Get

- Operations: `GetAutomationEvent`, `GetAutomationRule`, `GetEnrollmentConfiguration`
- Traits: `readonly` (1)
- Common required input members in this group: `eventId`, `ruleArn`

### Update

- Operations: `UpdateAutomationRule`, `UpdateEnrollmentConfiguration`
- Traits: `idempotency-token` (2), `idempotent` (2)
- Common required input members in this group: `ruleArn`, `ruleRevision`, `status`

### Associate

- Operations: `AssociateAccounts`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `accountIds`

### Create

- Operations: `CreateAutomationRule`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `name`, `recommendedActionTypes`, `ruleType`, `schedule`, `status`

### Delete

- Operations: `DeleteAutomationRule`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `ruleArn`, `ruleRevision`

### Disassociate

- Operations: `DisassociateAccounts`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `accountIds`

### Rollback

- Operations: `RollbackAutomationEvent`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `eventId`

### Start

- Operations: `StartAutomationEvent`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `recommendedActionId`

### Tag

- Operations: `TagResource`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `resourceArn`, `ruleRevision`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `resourceArn`, `ruleRevision`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateAccounts` | - | `idempotent`, `idempotency-token` | `accountIds` | `clientToken` | `AssociateAccountsResponse` | `AccessDeniedException`, `ForbiddenException`, `IdempotencyTokenInUseException`, `IdempotentParameterMismatchException`, `InternalServerException`, `InvalidParameterValueException`, `NotManagementAccountException`, `OptInRequiredException`, ... (+2) | Associates one or more member accounts with your organization's management account, enabling centralized implementation of optimization actions across those accounts. Once associated, the management account (or a delegated administrator) can apply recommended... |
| `CreateAutomationRule` | - | `idempotent`, `idempotency-token` | `name`, `recommendedActionTypes`, `ruleType`, `schedule`, `status` | `clientToken` | `CreateAutomationRuleResponse` | `AccessDeniedException`, `ForbiddenException`, `IdempotencyTokenInUseException`, `IdempotentParameterMismatchException`, `InternalServerException`, `InvalidParameterValueException`, `OptInRequiredException`, `ResourceNotFoundException`, ... (+3) | Creates a new automation rule to apply recommended actions to resources based on specified criteria. |
| `DeleteAutomationRule` | - | `idempotent`, `idempotency-token` | `ruleArn`, `ruleRevision` | `clientToken` | `DeleteAutomationRuleResponse` | `AccessDeniedException`, `ForbiddenException`, `IdempotencyTokenInUseException`, `IdempotentParameterMismatchException`, `InternalServerException`, `InvalidParameterValueException`, `OptInRequiredException`, `ResourceNotFoundException`, ... (+2) | Deletes an existing automation rule. |
| `DisassociateAccounts` | - | `idempotent`, `idempotency-token` | `accountIds` | `clientToken` | `DisassociateAccountsResponse` | `AccessDeniedException`, `ForbiddenException`, `IdempotencyTokenInUseException`, `IdempotentParameterMismatchException`, `InternalServerException`, `InvalidParameterValueException`, `NotManagementAccountException`, `OptInRequiredException`, ... (+2) | Disassociates member accounts from your organization's management account, removing centralized automation capabilities. Once disassociated, organization rules no longer apply to the member account, and the management account (or delegated administrator)... |
| `GetAutomationEvent` | - | - | `eventId` | - | `GetAutomationEventResponse` | `AccessDeniedException`, `ForbiddenException`, `InternalServerException`, `InvalidParameterValueException`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Retrieves details about a specific automation event. |
| `GetAutomationRule` | - | - | `ruleArn` | - | `GetAutomationRuleResponse` | `AccessDeniedException`, `ForbiddenException`, `InternalServerException`, `InvalidParameterValueException`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Retrieves details about a specific automation rule. |
| `GetEnrollmentConfiguration` | - | `readonly` | - | - | `GetEnrollmentConfigurationResponse` | `AccessDeniedException`, `ForbiddenException`, `InternalServerException`, `InvalidParameterValueException`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Retrieves the current enrollment configuration for Compute Optimizer Automation. |
| `ListAccounts` | - | `readonly`, `paginated` | - | - | `ListAccountsResponse` | `AccessDeniedException`, `ForbiddenException`, `InternalServerException`, `InvalidParameterValueException`, `NotManagementAccountException`, `OptInRequiredException`, `ServiceUnavailableException`, `ThrottlingException` | Lists the accounts in your organization that are enrolled in Compute Optimizer and whether they have enabled Automation. Only the management account or a delegated administrator can perform this action. |
| `ListAutomationEventSteps` | - | `paginated` | `eventId` | - | `ListAutomationEventStepsResponse` | `AccessDeniedException`, `ForbiddenException`, `InternalServerException`, `InvalidParameterValueException`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Lists the steps for a specific automation event. You can only list steps for events created within the past year. |
| `ListAutomationEventSummaries` | - | `paginated` | - | - | `ListAutomationEventSummariesResponse` | `AccessDeniedException`, `ForbiddenException`, `InternalServerException`, `InvalidParameterValueException`, `OptInRequiredException`, `ServiceUnavailableException`, `ThrottlingException` | Provides a summary of automation events based on specified filters. Only events created within the past year will be included in the summary. |
| `ListAutomationEvents` | - | `paginated` | - | - | `ListAutomationEventsResponse` | `AccessDeniedException`, `ForbiddenException`, `InternalServerException`, `InvalidParameterValueException`, `OptInRequiredException`, `ServiceUnavailableException`, `ThrottlingException` | Lists automation events based on specified filters. You can retrieve events that were created within the past year. |
| `ListAutomationRulePreview` | - | `readonly`, `paginated` | `recommendedActionTypes`, `ruleType` | - | `ListAutomationRulePreviewResponse` | `AccessDeniedException`, `ForbiddenException`, `InternalServerException`, `InvalidParameterValueException`, `OptInRequiredException`, `ServiceUnavailableException`, `ThrottlingException` | Returns a preview of the recommended actions that match your Automation rule's configuration and criteria. |
| `ListAutomationRulePreviewSummaries` | - | `readonly`, `paginated` | `recommendedActionTypes`, `ruleType` | - | `ListAutomationRulePreviewSummariesResponse` | `AccessDeniedException`, `ForbiddenException`, `InternalServerException`, `InvalidParameterValueException`, `OptInRequiredException`, `ServiceUnavailableException`, `ThrottlingException` | Returns a summary of the recommended actions that match your rule preview configuration and criteria. |
| `ListAutomationRules` | - | `paginated` | - | - | `ListAutomationRulesResponse` | `AccessDeniedException`, `ForbiddenException`, `InternalServerException`, `InvalidParameterValueException`, `OptInRequiredException`, `ServiceUnavailableException`, `ThrottlingException` | Lists the automation rules that match specified filters. |
| `ListRecommendedActionSummaries` | - | `paginated` | - | - | `ListRecommendedActionSummariesResponse` | `AccessDeniedException`, `ForbiddenException`, `InternalServerException`, `InvalidParameterValueException`, `OptInRequiredException`, `ServiceUnavailableException`, `ThrottlingException` | Provides a summary of recommended actions based on specified filters. Management accounts and delegated administrators can retrieve recommended actions that include associated member accounts. |
| `ListRecommendedActions` | - | `paginated` | - | - | `ListRecommendedActionsResponse` | `AccessDeniedException`, `ForbiddenException`, `InternalServerException`, `InvalidParameterValueException`, `OptInRequiredException`, `ServiceUnavailableException`, `ThrottlingException` | Lists the recommended actions based that match specified filters. Management accounts and delegated administrators can retrieve recommended actions that include associated member accounts. |
| `ListTagsForResource` | - | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `ForbiddenException`, `InternalServerException`, `InvalidParameterValueException`, `OptInRequiredException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException` | Lists the tags for a specified resource. |
| `RollbackAutomationEvent` | - | `idempotent`, `idempotency-token` | `eventId` | `clientToken` | `RollbackAutomationEventResponse` | `AccessDeniedException`, `ForbiddenException`, `IdempotencyTokenInUseException`, `IdempotentParameterMismatchException`, `InternalServerException`, `InvalidParameterValueException`, `OptInRequiredException`, `ResourceNotFoundException`, ... (+2) | Initiates a rollback for a completed automation event. Management accounts and delegated administrators can only initiate a rollback for events belonging to associated member accounts. |
| `StartAutomationEvent` | - | `idempotent`, `idempotency-token` | `recommendedActionId` | `clientToken` | `StartAutomationEventResponse` | `AccessDeniedException`, `ForbiddenException`, `IdempotencyTokenInUseException`, `IdempotentParameterMismatchException`, `InternalServerException`, `InvalidParameterValueException`, `OptInRequiredException`, `ResourceNotFoundException`, ... (+3) | Initiates a one-time, on-demand automation for the specified recommended action. Management accounts and delegated administrators can only initiate recommended actions for associated member accounts. |
| `TagResource` | - | `idempotent`, `idempotency-token` | `resourceArn`, `ruleRevision`, `tags` | `clientToken` | `TagResourceResponse` | `AccessDeniedException`, `ForbiddenException`, `IdempotencyTokenInUseException`, `IdempotentParameterMismatchException`, `InternalServerException`, `InvalidParameterValueException`, `OptInRequiredException`, `ResourceNotFoundException`, ... (+2) | Adds tags to the specified resource. |
| `UntagResource` | - | `idempotent`, `idempotency-token` | `resourceArn`, `ruleRevision`, `tagKeys` | `clientToken` | `UntagResourceResponse` | `AccessDeniedException`, `ForbiddenException`, `IdempotencyTokenInUseException`, `IdempotentParameterMismatchException`, `InternalServerException`, `InvalidParameterValueException`, `OptInRequiredException`, `ResourceNotFoundException`, ... (+2) | Removes tags from the specified resource. |
| `UpdateAutomationRule` | - | `idempotent`, `idempotency-token` | `ruleArn`, `ruleRevision` | `clientToken` | `UpdateAutomationRuleResponse` | `AccessDeniedException`, `ForbiddenException`, `IdempotencyTokenInUseException`, `IdempotentParameterMismatchException`, `InternalServerException`, `InvalidParameterValueException`, `OptInRequiredException`, `ResourceNotFoundException`, ... (+2) | Updates an existing automation rule. |
| `UpdateEnrollmentConfiguration` | - | `idempotent`, `idempotency-token` | `status` | `clientToken` | `UpdateEnrollmentConfigurationResponse` | `AccessDeniedException`, `ForbiddenException`, `IdempotencyTokenInUseException`, `IdempotentParameterMismatchException`, `InternalServerException`, `InvalidParameterValueException`, `NotManagementAccountException`, `OptInRequiredException`, ... (+3) | Updates your account’s Compute Optimizer Automation enrollment configuration. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient permissions to perform this action. |
| `ForbiddenException` | `structure` | `message` | You are not authorized to perform this action. |
| `InternalServerException` | `structure` | `message` | An internal error occurred while processing the request. |
| `InvalidParameterValueException` | `structure` | `message` | One or more parameter values are not valid. |
| `OptInRequiredException` | `structure` | `message` | The account must be opted in to Compute Optimizer Automation before performing this action. |
| `ServiceUnavailableException` | `structure` | `message` | The service is temporarily unavailable. |
| `ThrottlingException` | `structure` | `message` | The request was denied due to request throttling. |
| `ResourceNotFoundException` | `structure` | `message` | The specified resource was not found. |
| `IdempotencyTokenInUseException` | `structure` | `message` | The specified client token is already in use. |
| `IdempotentParameterMismatchException` | `structure` | `message` | Exception thrown when the same client token is used with different parameters, indicating a mismatch in idempotent request parameters. |
| `NotManagementAccountException` | `structure` | `message` | The operation can only be performed by a management account. |
| `ServiceQuotaExceededException` | `structure` | `message` | The request would exceed service quotas. |
| `AssociateAccountsRequest` | `structure` | `accountIds`, `clientToken` | - |
| `AssociateAccountsResponse` | `structure` | `accountIds`, `errors` | - |
| `CreateAutomationRuleRequest` | `structure` | `clientToken`, `criteria`, `description`, `name`, `organizationConfiguration`, `priority`, `recommendedActionTypes`, `ruleType`, `schedule`, `status`, `tags` | - |
| `CreateAutomationRuleResponse` | `structure` | `createdTimestamp`, `criteria`, `description`, `name`, `organizationConfiguration`, `priority`, `recommendedActionTypes`, `ruleArn`, `ruleId`, `ruleRevision`, `ruleType`, `schedule`, ... (+2) | - |
| `DeleteAutomationRuleRequest` | `structure` | `clientToken`, `ruleArn`, `ruleRevision` | - |
| `DeleteAutomationRuleResponse` | `structure` | - | - |
| `DisassociateAccountsRequest` | `structure` | `accountIds`, `clientToken` | - |
| `DisassociateAccountsResponse` | `structure` | `accountIds`, `errors` | - |
| `GetAutomationEventRequest` | `structure` | `eventId` | - |
| `GetAutomationEventResponse` | `structure` | `accountId`, `completedTimestamp`, `createdTimestamp`, `estimatedMonthlySavings`, `eventDescription`, `eventId`, `eventStatus`, `eventStatusReason`, `eventType`, `recommendedActionId`, `region`, `resourceArn`, ... (+3) | - |
| `GetAutomationRuleRequest` | `structure` | `ruleArn` | - |
| `GetAutomationRuleResponse` | `structure` | `accountId`, `createdTimestamp`, `criteria`, `description`, `lastUpdatedTimestamp`, `name`, `organizationConfiguration`, `priority`, `recommendedActionTypes`, `ruleArn`, `ruleId`, `ruleRevision`, ... (+4) | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
