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
- Common required input members in this group: `ResourceArn`, `ResourceType`

### Create

- Operations: `CreateRule`
- Common required input members in this group: `ResourceType`, `RetentionPeriod`

### Delete

- Operations: `DeleteRule`
- Common required input members in this group: `Identifier`

### Get

- Operations: `GetRule`
- Common required input members in this group: `Identifier`

### Lock

- Operations: `LockRule`
- Common required input members in this group: `Identifier`, `LockConfiguration`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Unlock

- Operations: `UnlockRule`
- Common required input members in this group: `Identifier`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Update

- Operations: `UpdateRule`
- Common required input members in this group: `Identifier`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateRule` | `POST /rules` | - | `ResourceType`, `RetentionPeriod` | - | `CreateRuleResponse` | `InternalServerException`, `ServiceQuotaExceededException`, `ValidationException` | Creates a Recycle Bin retention rule. You can create two types of retention rules: Tag-level retention rules - These retention rules use resource tags to identify the resources to protect. |
| `DeleteRule` | `DELETE /rules/{Identifier}` | - | `Identifier` | - | `DeleteRuleResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a Recycle Bin retention rule. For more information, see Delete Recycle Bin retention rules in the Amazon Elastic Compute Cloud User Guide . |
| `GetRule` | `GET /rules/{Identifier}` | - | `Identifier` | - | `GetRuleResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets information about a Recycle Bin retention rule. |
| `ListRules` | `POST /list-rules` | `paginated` | `ResourceType` | - | `ListRulesResponse` | `InternalServerException`, `ValidationException` | Lists the Recycle Bin retention rules in the Region. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists the tags assigned to a retention rule. |
| `LockRule` | `PATCH /rules/{Identifier}/lock` | - | `Identifier`, `LockConfiguration` | - | `LockRuleResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Locks a Region-level retention rule. A locked retention rule can't be modified or deleted. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Assigns tags to the specified retention rule. |
| `UnlockRule` | `PATCH /rules/{Identifier}/unlock` | - | `Identifier` | - | `UnlockRuleResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Unlocks a retention rule. After a retention rule is unlocked, it can be modified or deleted only after the unlock delay period expires. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Unassigns a tag from a retention rule. |
| `UpdateRule` | `PATCH /rules/{Identifier}` | - | `Identifier` | - | `UpdateRuleResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Updates an existing Recycle Bin retention rule. You can update a retention rule's description, resource tags, and retention period at any time after creation. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `Message` | The service could not respond to the request due to an internal problem. |
| `ValidationException` | `structure` | `Message`, `Reason` | One or more of the parameters in the request is not valid. |
| `ResourceNotFoundException` | `structure` | `Message`, `Reason` | The specified resource was not found. |
| `ConflictException` | `structure` | `Message`, `Reason` | The specified retention rule lock request can't be completed. |
| `ServiceQuotaExceededException` | `structure` | `Message`, `Reason` | The request would cause a service quota for the number of tags per resource to be exceeded. |
| `CreateRuleRequest` | `structure` | `Description`, `ExcludeResourceTags`, `LockConfiguration`, `ResourceTags`, `ResourceType`, `RetentionPeriod`, `Tags` | - |
| `CreateRuleResponse` | `structure` | `Description`, `ExcludeResourceTags`, `Identifier`, `LockConfiguration`, `LockState`, `ResourceTags`, `ResourceType`, `RetentionPeriod`, `RuleArn`, `Status`, `Tags` | - |
| `DeleteRuleRequest` | `structure` | `Identifier` | - |
| `DeleteRuleResponse` | `structure` | - | - |
| `GetRuleRequest` | `structure` | `Identifier` | - |
| `GetRuleResponse` | `structure` | `Description`, `ExcludeResourceTags`, `Identifier`, `LockConfiguration`, `LockEndTime`, `LockState`, `ResourceTags`, `ResourceType`, `RetentionPeriod`, `RuleArn`, `Status` | - |
| `ListRulesRequest` | `structure` | `ExcludeResourceTags`, `LockState`, `MaxResults`, `NextToken`, `ResourceTags`, `ResourceType` | - |
| `ListRulesResponse` | `structure` | `NextToken`, `Rules` | - |
| `ListTagsForResourceRequest` | `structure` | `ResourceArn` | - |
| `ListTagsForResourceResponse` | `structure` | `Tags` | - |
| `LockRuleRequest` | `structure` | `Identifier`, `LockConfiguration` | - |
| `LockRuleResponse` | `structure` | `Description`, `ExcludeResourceTags`, `Identifier`, `LockConfiguration`, `LockState`, `ResourceTags`, `ResourceType`, `RetentionPeriod`, `RuleArn`, `Status` | - |
| `TagResourceRequest` | `structure` | `ResourceArn`, `Tags` | - |
| `TagResourceResponse` | `structure` | - | - |
| `UnlockRuleRequest` | `structure` | `Identifier` | - |
| `UnlockRuleResponse` | `structure` | `Description`, `ExcludeResourceTags`, `Identifier`, `LockConfiguration`, `LockEndTime`, `LockState`, `ResourceTags`, `ResourceType`, `RetentionPeriod`, `RuleArn`, `Status` | - |
| `UntagResourceRequest` | `structure` | `ResourceArn`, `TagKeys` | - |
| `UntagResourceResponse` | `structure` | - | - |
| `UpdateRuleRequest` | `structure` | `Description`, `ExcludeResourceTags`, `Identifier`, `ResourceTags`, `ResourceType`, `RetentionPeriod` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
