# Security Incident Response

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This guide documents the action and response elements for use of the service.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Security Incident Response where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Security Incident Response by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for Security Incident Response resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Security Incident Response workflows in the local mock. Key resources include `Case`, `Membership`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Update`, `Get`, `Create`, `Batch` operation families, including `ListCaseEdits`, `ListCases`, `ListComments`, `ListInvestigations`, `UpdateCase`, `UpdateCaseComment`.

## Service Identity and Protocol

- AWS model slug: `security-ir`
- AWS SDK for Rust slug: `securityir`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/security-ir/service/2018-05-10/security-ir-2018-05-10.json`
- SDK ID: `Security IR`
- Endpoint prefix: `-`
- ARN namespace: `security-ir`
- CloudFormation name: `-`
- CloudTrail event source: `security-ir.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (6), `Update` (5), `Get` (4), `Create` (3), `Batch` (1), `Cancel` (1), `Close` (1), `Send` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchGetMemberAccountDetails`, `CancelMembership`, `CreateCase`, `CreateCaseComment`, `CreateMembership`, `TagResource`, `UntagResource`, `UpdateCase`, `UpdateCaseComment`, `UpdateCaseStatus`, `UpdateMembership`, `UpdateResolverType`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `BatchGetMemberAccountDetails`, `GetCase`, `GetCaseAttachmentDownloadUrl`, `GetCaseAttachmentUploadUrl`, `GetMembership`, `ListCaseEdits`, `ListCases`, `ListComments`, `ListInvestigations`, `ListMemberships`, `ListTagsForResource`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 8 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelMembership`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 3 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `STS`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Case` | `caseId` | create: `CreateCase`; read: `GetCase`; update: `UpdateCase`; list: `ListCases` | `CloseCase`, `CreateCaseComment`, `GetCaseAttachmentDownloadUrl`, `GetCaseAttachmentUploadUrl`, `ListCaseEdits`, `ListComments`, `ListInvestigations`, `SendFeedback`, `UpdateCaseComment`, `UpdateCaseStatus`, ... (+1) | Represents a case that is used to track the incident response lifecycle |
| `Membership` | `membershipId` | create: `CreateMembership`; read: `GetMembership`; update: `UpdateMembership`; list: `ListMemberships` | `BatchGetMemberAccountDetails`, `CancelMembership` | Represents a membership that is used to manage service level preferences |
## Operation Groups

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListTagsForResource` | `GET /v1/tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Returns currently configured tags on a resource. |
| `TagResource` | `POST /v1/tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceOutput` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Adds a tag(s) to a designated resource. |
| `UntagResource` | `DELETE /v1/tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Removes a tag(s) from a designate resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | - |
| `ConflictException` | `structure` | message, resourceId, resourceType | - |
| `InternalServerException` | `structure` | message, retryAfterSeconds | - |
| `InvalidTokenException` | `structure` | message | - |
| `ResourceNotFoundException` | `structure` | message | - |
| `SecurityIncidentResponseNotActiveException` | `structure` | message | - |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, serviceCode, quotaCode | - |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds | - |
| `ValidationException` | `structure` | message, reason, fieldList | - |
| `ListTagsForResourceInput` | `structure` | resourceArn | - |
| `ListTagsForResourceOutput` | `structure` | tags | - |
| `TagResourceInput` | `structure` | resourceArn, tags | - |
| `TagResourceOutput` | `structure` | **empty (no members)** | - |
| `UntagResourceInput` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceOutput` | `structure` | **empty (no members)** | - |
| `ActionType` | `enum` | EVIDENCE_COLLECTION, INVESTIGATION_ANALYSIS, SUMMARIZATION | - |
| `AwsRegion` | `enum` | AF_SOUTH_1, AP_EAST_1, AP_EAST_2, AP_NORTHEAST_1, AP_NORTHEAST_2, AP_NORTHEAST_3, AP_SOUTH_1, AP_SOUTH_2, AP_SOUTHEAST_1, AP_SOUTHEAST_2, AP_SOUTHEAST_3, AP_SOUTHEAST_4, ... (+24) | - |
| `CaseAttachmentStatus` | `enum` | VERIFIED, FAILED, PENDING | - |
| `CaseStatus` | `enum` | SUBMITTED, ACKNOWLEDGED, DETECTION_AND_ANALYSIS, CONTAINMENT_ERADICATION_AND_RECOVERY, POST_INCIDENT_ACTIVITIES, READY_TO_CLOSE, CLOSED | - |
| `ClosureCode` | `enum` | INVESTIGATION_COMPLETED, NOT_RESOLVED, FALSE_POSITIVE, DUPLICATE | - |
| `CommunicationType` | `enum` | CASE_CREATED, CASE_UPDATED, CASE_ACKNOWLEDGED, CASE_CLOSED, CASE_UPDATED_TO_SERVICE_MANAGED, CASE_UPDATE_CASE_STATUS, CASE_PENDING_CUSTOMER_ACTION_REMINDER, CASE_ATTACHMENT_URL_UPLOADED, CASE_COMMENT_ADDED, CASE_COMMENT_UPDATED, MEMBERSHIP_CREATED, MEMBERSHIP_UPDATED, ... (+4) | - |
| `CustomerType` | `enum` | STANDALONE, ORGANIZATION | - |
| `EngagementType` | `enum` | SECURITY_INCIDENT, INVESTIGATION | - |
| `ExecutionStatus` | `enum` | PENDING, IN_PROGRESS, WAITING, COMPLETED, FAILED, CANCELLED | - |
| `MembershipAccountRelationshipStatus` | `enum` | ASSOCIATED, DISASSOCIATED, UNASSOCIATED | - |
| `MembershipAccountRelationshipType` | `enum` | ORGANIZATION, UNRELATED | - |
| `MembershipStatus` | `enum` | ACTIVE, CANCELLED, TERMINATED | - |
| `OptInFeatureName` | `enum` | TRIAGE | - |
| `PendingAction` | `enum` | CUSTOMER, NONE | - |
| `ResolverType` | `enum` | AWS, SELF | - |
| `SelfManagedCaseStatus` | `enum` | SUBMITTED, DETECTION_AND_ANALYSIS, CONTAINMENT_ERADICATION_AND_RECOVERY, POST_INCIDENT_ACTIVITIES | - |
| `UsefulnessRating` | `enum` | USEFUL, NOT_USEFUL | - |
| `ValidationExceptionReason` | `enum` | UNKNOWN_OPERATION, CANNOT_PARSE, FIELD_VALIDATION_FAILED, OTHER | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
