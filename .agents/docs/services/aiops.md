# AWS AI Ops

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The CloudWatch investigations feature is a generative AI-powered assistant that can help you respond to incidents in your system. It uses generative AI to scan your system's telemetry and quickly surface suggestions that might be related to your issue. These suggestions include metrics, logs, deployment events, and root-cause hypotheses. You can use API actions to create, manage, and delete investigation groups and investigation group policies. To start and manage investigations, you must use the CloudWatch console.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-aiops/tests/scenario_test.rs`: create, update, tag, list, and delete an investigation group as the primary operational container for incident analysis.
- Backported from `scenario_test.rs`: configure cross-account investigation-group access and verify that the configuration can be described and removed.
- From the AWS documentation and model: support operational investigation workflows that group related telemetry, findings, and account access configuration for AI-assisted incident triage.

## Service Identity and Protocol

- AWS model slug: `aiops`
- AWS SDK for Rust slug: `aiops`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/aiops/service/2018-05-10/aiops-2018-05-10.json`
- SDK ID: `AIOps`
- Endpoint prefix: `-`
- ARN namespace: `aiops`
- CloudFormation name: `-`
- CloudTrail event source: `aiops.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (2), `Get` (2), `List` (2), `Create` (1), `Put` (1), `Tag` (1), `Untag` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateInvestigationGroup`, `DeleteInvestigationGroup`, `DeleteInvestigationGroupPolicy`, `PutInvestigationGroupPolicy`, `TagResource`, `UntagResource`, `UpdateInvestigationGroup`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetInvestigationGroup`, `GetInvestigationGroupPolicy`, `ListInvestigationGroups`, `ListTagsForResource`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 7 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 11 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `KMS`, `CloudWatch`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `InvestigationGroup` | `identifier` | create: `CreateInvestigationGroup`; read: `GetInvestigationGroup`; update: `UpdateInvestigationGroup`; delete: `DeleteInvestigationGroup`; list: `ListInvestigationGroups` | - | - |
| `InvestigationGroupPolicy` | `identifier` | put: `PutInvestigationGroupPolicy`; read: `GetInvestigationGroupPolicy`; delete: `DeleteInvestigationGroupPolicy` | - | - |
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
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Displays the tags associated with a CloudWatch investigations resource. Currently, investigation groups support tagging. |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Assigns one or more tags (key-value pairs) to the specified resource. Tags can help you organize and categorize your resources. You can also use them to scope user permissions by granting a user permission to access ... |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes one or more tags from the specified resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have sufficient permissions to perform this action. |
| `ConflictException` | `structure` | message | This operation couldn't be completed because of a conflict in resource states. |
| `ForbiddenException` | `structure` | message | Access id denied for this operation, or this operation is not valid for the specified resource. |
| `InternalServerException` | `structure` | message | An internal server error occurred. You can try again later. |
| `ResourceNotFoundException` | `structure` | message | The specified resource doesn't exist. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, serviceCode, quotaCode | This request exceeds a service quota. |
| `ThrottlingException` | `structure` | message | The request was throttled because of quota limits. You can try again later. |
| `ValidationException` | `structure` | message | This operation or its parameters aren't formatted correctly. |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceOutput` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `EncryptionConfigurationType` | `enum` | AWS_OWNED_KEY, CUSTOMER_MANAGED_KMS_KEY | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
