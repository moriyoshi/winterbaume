# Amazon Recycle Bin

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This is the Recycle Bin API Reference . This documentation provides descriptions and syntax for each of the actions and data types in Recycle Bin. Recycle Bin is a resource recovery feature that enables you to restore accidentally deleted EBS volumes, EBS snapshots, and EBS-backed AMIs. When using Recycle Bin, if your resources are deleted, they are retained in the Recycle Bin for a time period that you specify. You can restore a resource from the Recycle Bin at any time before its retention period expires.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-rbin/tests/scenario_test.rs`: create a retention rule, lock it, verify protected behaviour, unlock or clean up as allowed.
- Backported from `scenario_test.rs`: round-trip tags on retention rules.
- From the AWS documentation and model: model Recycle Bin retention rules for recoverable deleted resources, lock configuration, rule lifecycle, resource-type scoping, and tag-based governance.

## Service Identity and Protocol

- AWS model slug: `rbin`
- AWS SDK for Rust slug: `rbin`
- Model version: `2021-06-15`
- Model file: `vendor/api-models-aws/models/rbin/service/2021-06-15/rbin-2021-06-15.json`
- SDK ID: `rbin`
- Endpoint prefix: `rbin`
- ARN namespace: `rbin`
- CloudFormation name: `Rbin`
- CloudTrail event source: `rbin.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (2), `Create` (1), `Delete` (1), `Get` (1), `Lock` (1), `Tag` (1), `Unlock` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateRule`, `DeleteRule`, `TagResource`, `UntagResource`, `UpdateRule`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetRule`, `ListRules`, `ListTagsForResource`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 10 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Operation Groups

### List

- Operations: `ListRules`, `ListTagsForResource`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Create

- Operations: `CreateRule`
- Common required input members in this group: -

### Delete

- Operations: `DeleteRule`
- Common required input members in this group: -

### Get

- Operations: `GetRule`
- Common required input members in this group: -

### Lock

- Operations: `LockRule`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Unlock

- Operations: `UnlockRule`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Update

- Operations: `UpdateRule`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateRule` | `POST /rules` | - | `RetentionPeriod`, `ResourceType` | - | `CreateRuleResponse` | `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a Recycle Bin retention rule. You can create two types of retention rules: Tag-level retention rules - These retention rules use resource tags to identify the resources to protect. For each retention rule, yo ... |
| `DeleteRule` | `DELETE /rules/{Identifier}` | - | `Identifier` | - | `DeleteRuleResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a Recycle Bin retention rule. For more information, see Delete Recycle Bin retention rules in the Amazon Elastic Compute Cloud User Guide . |
| `GetRule` | `GET /rules/{Identifier}` | - | `Identifier` | - | `GetRuleResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets information about a Recycle Bin retention rule. |
| `ListRules` | `POST /list-rules` | `paginated` | `ResourceType` | - | `ListRulesResponse` | `InternalServerException`, `ValidationException` | Lists the Recycle Bin retention rules in the Region. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the tags assigned to a retention rule. |
| `LockRule` | `PATCH /rules/{Identifier}/lock` | - | `Identifier`, `LockConfiguration` | - | `LockRuleResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Locks a Region-level retention rule. A locked retention rule can't be modified or deleted. You can't lock tag-level retention rules, or Region-level retention rules that have exclusion tags. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Assigns tags to the specified retention rule. |
| `UnlockRule` | `PATCH /rules/{Identifier}/unlock` | - | `Identifier` | - | `UnlockRuleResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Unlocks a retention rule. After a retention rule is unlocked, it can be modified or deleted only after the unlock delay period expires. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Unassigns a tag from a retention rule. |
| `UpdateRule` | `PATCH /rules/{Identifier}` | - | `Identifier` | - | `UpdateRuleResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Updates an existing Recycle Bin retention rule. You can update a retention rule's description, resource tags, and retention period at any time after creation. You can't update a retention rule's resource type after c ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ConflictException` | `structure` | Message, Reason | The specified retention rule lock request can't be completed. |
| `InternalServerException` | `structure` | Message | The service could not respond to the request due to an internal problem. |
| `ResourceNotFoundException` | `structure` | Message, Reason | The specified resource was not found. |
| `ServiceQuotaExceededException` | `structure` | Message, Reason | The request would cause a service quota for the number of tags per resource to be exceeded. |
| `ValidationException` | `structure` | Message, Reason | One or more of the parameters in the request is not valid. |
| `CreateRuleRequest` | `structure` | RetentionPeriod, Description, Tags, ResourceType, ResourceTags, LockConfiguration, ExcludeResourceTags | - |
| `CreateRuleResponse` | `structure` | Identifier, RetentionPeriod, Description, Tags, ResourceType, ResourceTags, Status, LockConfiguration, LockState, RuleArn, ExcludeResourceTags | - |
| `DeleteRuleRequest` | `structure` | Identifier | - |
| `DeleteRuleResponse` | `structure` | **empty (no members)** | - |
| `GetRuleRequest` | `structure` | Identifier | - |
| `GetRuleResponse` | `structure` | Identifier, Description, ResourceType, RetentionPeriod, ResourceTags, Status, LockConfiguration, LockState, LockEndTime, RuleArn, ExcludeResourceTags | - |
| `ListRulesRequest` | `structure` | MaxResults, NextToken, ResourceType, ResourceTags, LockState, ExcludeResourceTags | - |
| `ListRulesResponse` | `structure` | Rules, NextToken | - |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `LockRuleRequest` | `structure` | Identifier, LockConfiguration | - |
| `LockRuleResponse` | `structure` | Identifier, Description, ResourceType, RetentionPeriod, ResourceTags, Status, LockConfiguration, LockState, RuleArn, ExcludeResourceTags | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UnlockRuleRequest` | `structure` | Identifier | - |
| `UnlockRuleResponse` | `structure` | Identifier, Description, ResourceType, RetentionPeriod, ResourceTags, Status, LockConfiguration, LockState, LockEndTime, RuleArn, ExcludeResourceTags | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `UpdateRuleRequest` | `structure` | Identifier, RetentionPeriod, Description, ResourceType, ResourceTags, ExcludeResourceTags | - |
| `UpdateRuleResponse` | `structure` | Identifier, RetentionPeriod, Description, ResourceType, ResourceTags, Status, LockState, LockEndTime, RuleArn, ExcludeResourceTags | - |
| `ConflictExceptionReason` | `enum` | INVALID_RULE_STATE | - |
| `LockState` | `enum` | LOCKED, PENDING_UNLOCK, UNLOCKED | - |
| `ResourceNotFoundExceptionReason` | `enum` | RULE_NOT_FOUND | - |
| `ResourceType` | `enum` | EBS_SNAPSHOT, EC2_IMAGE, EBS_VOLUME | - |
| `RetentionPeriodUnit` | `enum` | DAYS | - |
| `RuleStatus` | `enum` | PENDING, AVAILABLE | - |
| `ServiceQuotaExceededExceptionReason` | `enum` | SERVICE_QUOTA_EXCEEDED | - |
| `UnlockDelayUnit` | `enum` | DAYS | - |
| `ValidationExceptionReason` | `enum` | INVALID_PAGE_TOKEN, INVALID_PARAMETER_VALUE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
