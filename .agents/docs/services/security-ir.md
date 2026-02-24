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

- Operations: `ListCaseEdits`, `ListCases`, `ListComments`, `ListInvestigations`, `ListMemberships`, `ListTagsForResource`
- Traits: `paginated` (5), `readonly` (6)
- Common required input members in this group: `caseId`, `resourceArn`

### Update

- Operations: `UpdateCase`, `UpdateCaseComment`, `UpdateCaseStatus`, `UpdateMembership`, `UpdateResolverType`
- Traits: `idempotent` (2)
- Common required input members in this group: `body`, `caseId`, `caseStatus`, `commentId`, `membershipId`, `resolverType`

### Get

- Operations: `GetCase`, `GetCaseAttachmentDownloadUrl`, `GetCaseAttachmentUploadUrl`, `GetMembership`
- Traits: `idempotency-token` (1), `idempotent` (1), `readonly` (3)
- Common required input members in this group: `attachmentId`, `caseId`, `contentLength`, `fileName`, `membershipId`

### Create

- Operations: `CreateCase`, `CreateCaseComment`, `CreateMembership`
- Traits: `idempotency-token` (3), `idempotent` (3)
- Common required input members in this group: `body`, `caseId`, `description`, `engagementType`, `impactedAccounts`, `incidentResponseTeam`, `membershipName`, `reportedIncidentStartDate`, `resolverType`, `title`, `watchers`

### Batch

- Operations: `BatchGetMemberAccountDetails`
- Traits: `readonly` (1)
- Common required input members in this group: `accountIds`, `membershipId`

### Cancel

- Operations: `CancelMembership`
- Traits: `idempotent` (1)
- Common required input members in this group: `membershipId`

### Close

- Operations: `CloseCase`
- Common required input members in this group: `caseId`

### Send

- Operations: `SendFeedback`
- Common required input members in this group: `caseId`, `resultId`, `usefulness`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchGetMemberAccountDetails` | `POST /v1/membership/{membershipId}/batch-member-details` | `readonly` | `accountIds`, `membershipId` | - | `BatchGetMemberAccountDetailsResponse` | - | Provides information on whether the supplied account IDs are associated with a membership. AWS account ID's may appear less than 12 characters and need to be zero-prepended. |
| `CancelMembership` | `PUT /v1/membership/{membershipId}` | `idempotent` | `membershipId` | - | `CancelMembershipResponse` | - | Cancels an existing membership. |
| `CloseCase` | `POST /v1/cases/{caseId}/close-case` | - | `caseId` | - | `CloseCaseResponse` | - | Closes an existing case. |
| `CreateCase` | `POST /v1/create-case` | `idempotent`, `idempotency-token` | `description`, `engagementType`, `impactedAccounts`, `reportedIncidentStartDate`, `resolverType`, `title`, `watchers` | `clientToken` | `CreateCaseResponse` | - | Creates a new case. |
| `CreateCaseComment` | `POST /v1/cases/{caseId}/create-comment` | `idempotent`, `idempotency-token` | `body`, `caseId` | `clientToken` | `CreateCaseCommentResponse` | - | Adds a comment to an existing case. |
| `CreateMembership` | `POST /v1/membership` | `idempotent`, `idempotency-token` | `incidentResponseTeam`, `membershipName` | `clientToken` | `CreateMembershipResponse` | - | Creates a new membership. |
| `GetCase` | `GET /v1/cases/{caseId}/get-case` | `readonly` | `caseId` | - | `GetCaseResponse` | - | Returns the attributes of a case. |
| `GetCaseAttachmentDownloadUrl` | `GET /v1/cases/{caseId}/get-presigned-url/{attachmentId}` | `readonly` | `attachmentId`, `caseId` | - | `GetCaseAttachmentDownloadUrlResponse` | - | Returns a Pre-Signed URL for uploading attachments into a case. |
| `GetCaseAttachmentUploadUrl` | `POST /v1/cases/{caseId}/get-presigned-url` | `idempotent`, `idempotency-token` | `caseId`, `contentLength`, `fileName` | `clientToken` | `GetCaseAttachmentUploadUrlResponse` | - | Uploads an attachment to a case. |
| `GetMembership` | `GET /v1/membership/{membershipId}` | `readonly` | `membershipId` | - | `GetMembershipResponse` | - | Returns the attributes of a membership. |
| `ListCaseEdits` | `POST /v1/cases/{caseId}/list-case-edits` | `readonly`, `paginated` | `caseId` | - | `ListCaseEditsResponse` | - | Views the case history for edits made to a designated case. |
| `ListCases` | `POST /v1/list-cases` | `readonly`, `paginated` | - | - | `ListCasesResponse` | - | Lists all cases the requester has access to. |
| `ListComments` | `POST /v1/cases/{caseId}/list-comments` | `readonly`, `paginated` | `caseId` | - | `ListCommentsResponse` | - | Returns comments for a designated case. |
| `ListInvestigations` | `GET /v1/cases/{caseId}/list-investigations` | `readonly`, `paginated` | `caseId` | - | `ListInvestigationsResponse` | - | Investigation performed by an agent for a security incident... |
| `ListMemberships` | `POST /v1/memberships` | `readonly`, `paginated` | - | - | `ListMembershipsResponse` | - | Returns the memberships that the calling principal can access. |
| `ListTagsForResource` | `GET /v1/tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceOutput` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Returns currently configured tags on a resource. |
| `SendFeedback` | `POST /v1/cases/{caseId}/feedback/{resultId}/send-feedback` | - | `caseId`, `resultId`, `usefulness` | - | `SendFeedbackResponse` | - | Send feedback based on response investigation action |
| `TagResource` | `POST /v1/tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceOutput` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Adds a tag(s) to a designated resource. |
| `UntagResource` | `DELETE /v1/tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceOutput` | `AccessDeniedException`, `ResourceNotFoundException`, `ValidationException` | Removes a tag(s) from a designate resource. |
| `UpdateCase` | `POST /v1/cases/{caseId}/update-case` | - | `caseId` | - | `UpdateCaseResponse` | - | Updates an existing case. |
| `UpdateCaseComment` | `PUT /v1/cases/{caseId}/update-case-comment/{commentId}` | `idempotent` | `body`, `caseId`, `commentId` | - | `UpdateCaseCommentResponse` | - | Updates an existing case comment. |
| `UpdateCaseStatus` | `POST /v1/cases/{caseId}/update-case-status` | - | `caseId`, `caseStatus` | - | `UpdateCaseStatusResponse` | - | Updates the state transitions for a designated cases. Self-managed : the following states are available for self-managed cases. |
| `UpdateMembership` | `PUT /v1/membership/{membershipId}/update-membership` | `idempotent` | `membershipId` | - | `UpdateMembershipResponse` | - | Updates membership configuration. |
| `UpdateResolverType` | `POST /v1/cases/{caseId}/update-resolver-type` | - | `caseId`, `resolverType` | - | `UpdateResolverTypeResponse` | - | Updates the resolver type for a case. This is a one-way action and cannot be reversed. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | - |
| `ResourceNotFoundException` | `structure` | `message` | - |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | - |
| `BatchGetMemberAccountDetailsRequest` | `structure` | `accountIds`, `membershipId` | - |
| `BatchGetMemberAccountDetailsResponse` | `structure` | `errors`, `items` | - |
| `CancelMembershipRequest` | `structure` | `membershipId` | - |
| `CancelMembershipResponse` | `structure` | `membershipId` | - |
| `CloseCaseRequest` | `structure` | `caseId` | - |
| `CloseCaseResponse` | `structure` | `caseStatus`, `closedDate` | - |
| `CreateCaseRequest` | `structure` | `clientToken`, `description`, `engagementType`, `impactedAccounts`, `impactedAwsRegions`, `impactedServices`, `reportedIncidentStartDate`, `resolverType`, `tags`, `threatActorIpAddresses`, `title`, `watchers` | - |
| `CreateCaseResponse` | `structure` | `caseId` | - |
| `CreateCaseCommentRequest` | `structure` | `body`, `caseId`, `clientToken` | - |
| `CreateCaseCommentResponse` | `structure` | `commentId` | - |
| `CreateMembershipRequest` | `structure` | `clientToken`, `coverEntireOrganization`, `incidentResponseTeam`, `membershipName`, `optInFeatures`, `tags` | - |
| `CreateMembershipResponse` | `structure` | `membershipId` | - |
| `GetCaseRequest` | `structure` | `caseId` | - |
| `GetCaseResponse` | `structure` | `actualIncidentStartDate`, `caseArn`, `caseAttachments`, `caseMetadata`, `caseStatus`, `closedDate`, `closureCode`, `createdDate`, `description`, `engagementType`, `impactedAccounts`, `impactedAwsRegions`, ... (+8) | - |
| `GetCaseAttachmentDownloadUrlRequest` | `structure` | `attachmentId`, `caseId` | - |
| `GetCaseAttachmentDownloadUrlResponse` | `structure` | `attachmentPresignedUrl` | - |
| `GetCaseAttachmentUploadUrlRequest` | `structure` | `caseId`, `clientToken`, `contentLength`, `fileName` | - |
| `GetCaseAttachmentUploadUrlResponse` | `structure` | `attachmentPresignedUrl` | - |
| `GetMembershipRequest` | `structure` | `membershipId` | - |
| `GetMembershipResponse` | `structure` | `accountId`, `customerType`, `incidentResponseTeam`, `membershipAccountsConfigurations`, `membershipActivationTimestamp`, `membershipArn`, `membershipDeactivationTimestamp`, `membershipId`, `membershipName`, `membershipStatus`, `numberOfAccountsCovered`, `optInFeatures`, ... (+1) | - |
| `ListCaseEditsRequest` | `structure` | `caseId`, `maxResults`, `nextToken` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
