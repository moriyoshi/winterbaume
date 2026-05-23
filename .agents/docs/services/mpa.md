# AWS Multi-party Approval

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Multi-party approval is a capability of Organizations that allows you to protect a predefined list of operations through a distributed approval process. Use Multi-party approval to establish approval workflows and transform security processes into team-based decisions. When to use Multi-party approval : You need to align with the Zero Trust principle of "never trust, always verify" You need to make sure that the right humans have access to the right things in the right way You need distributed decision-making for sensitive or critical operations You need to protect against unintended operations on sensitive or critical resources You need formal reviews and approvals for auditing or compliance reasons For more information, see What is Multi-party approval in the Multi-party approval User Guide .

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS Multi-party Approval resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Multi-party Approval workflows in the local mock. Key resources include `ApprovalTeam`, `IdentitySource`, `Session`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Start` operation families, including `ListApprovalTeams`, `ListIdentitySources`, `ListPolicies`, `ListPolicyVersions`, `GetApprovalTeam`, `GetIdentitySource`.

## Service Identity and Protocol

- AWS model slug: `mpa`
- AWS SDK for Rust slug: `mpa`
- Model version: `2022-07-26`
- Model file: `vendor/api-models-aws/models/mpa/service/2022-07-26/mpa-2022-07-26.json`
- SDK ID: `MPA`
- Endpoint prefix: `-`
- ARN namespace: `mpa`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (7), `Get` (5), `Create` (2), `Delete` (2), `Start` (2), `Cancel` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelSession`, `CreateApprovalTeam`, `CreateIdentitySource`, `DeleteIdentitySource`, `DeleteInactiveApprovalTeamVersion`, `StartActiveApprovalTeamDeletion`, `StartApprovalTeamBaseline`, `TagResource`, `UntagResource`, `UpdateApprovalTeam`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetApprovalTeam`, `GetIdentitySource`, `GetPolicyVersion`, `GetResourcePolicy`, `GetSession`, `ListApprovalTeams`, `ListIdentitySources`, `ListPolicies`, `ListPolicyVersions`, `ListResourcePolicies`, `ListSessions`, `ListTagsForResource`.
- Pagination is modelled for 6 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 9 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelSession`, `StartActiveApprovalTeamDeletion`, `StartApprovalTeamBaseline`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 22 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ApprovalTeam` | `Arn` | create: `CreateApprovalTeam`; read: `GetApprovalTeam`; update: `UpdateApprovalTeam`; delete: `DeleteInactiveApprovalTeamVersion`; list: `ListApprovalTeams` | `StartActiveApprovalTeamDeletion`, `StartApprovalTeamBaseline` | Represents a team that is responsible for approving protected operations |
| `IdentitySource` | `IdentitySourceArn` | create: `CreateIdentitySource`; read: `GetIdentitySource`; delete: `DeleteIdentitySource`; list: `ListIdentitySources` | - | Represents an association with an IAM Identity Center instance that manages the user authentication for approvers |
| `Session` | `SessionArn` | read: `GetSession`; update: `CancelSession`; list: `ListSessions` | - | Represents an approval workflow that is initiated when a request is made to execute a protected operation |
## Operation Groups

### List

- Operations: `ListApprovalTeams`, `ListIdentitySources`, `ListPolicies`, `ListPolicyVersions`, `ListResourcePolicies`, `ListSessions`, `ListTagsForResource`
- Traits: `paginated` (6), `readonly` (7)
- Common required input members in this group: `ApprovalTeamArn`, `PolicyArn`, `ResourceArn`

### Get

- Operations: `GetApprovalTeam`, `GetIdentitySource`, `GetPolicyVersion`, `GetResourcePolicy`, `GetSession`
- Traits: `readonly` (5)
- Common required input members in this group: `Arn`, `IdentitySourceArn`, `PolicyName`, `PolicyType`, `PolicyVersionArn`, `ResourceArn`, `SessionArn`

### Create

- Operations: `CreateApprovalTeam`, `CreateIdentitySource`
- Traits: `idempotency-token` (2), `idempotent` (2)
- Common required input members in this group: `ApprovalStrategy`, `Approvers`, `Description`, `IdentitySourceParameters`, `Name`, `Policies`

### Delete

- Operations: `DeleteIdentitySource`, `DeleteInactiveApprovalTeamVersion`
- Traits: `idempotent` (2)
- Common required input members in this group: `Arn`, `IdentitySourceArn`, `VersionId`

### Start

- Operations: `StartActiveApprovalTeamDeletion`, `StartApprovalTeamBaseline`
- Traits: `idempotent` (1)
- Common required input members in this group: `Arn`

### Cancel

- Operations: `CancelSession`
- Traits: `idempotent` (1)
- Common required input members in this group: `SessionArn`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Update

- Operations: `UpdateApprovalTeam`
- Traits: `idempotent` (1)
- Common required input members in this group: `Arn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelSession` | `PUT /sessions/{SessionArn}` | `idempotent` | `SessionArn` | - | `CancelSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels an approval session. For more information, see Session in the Multi-party approval User Guide . |
| `CreateApprovalTeam` | `POST /approval-teams` | `idempotent`, `idempotency-token` | `ApprovalStrategy`, `Approvers`, `Description`, `Name`, `Policies` | `ClientToken` | `CreateApprovalTeamResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new approval team. For more information, see Approval team in the Multi-party approval User Guide . |
| `CreateIdentitySource` | `POST /identity-sources` | `idempotent`, `idempotency-token` | `IdentitySourceParameters` | `ClientToken` | `CreateIdentitySourceResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new identity source. For more information, see Identity Source in the Multi-party approval User Guide . |
| `DeleteIdentitySource` | `DELETE /identity-sources/{IdentitySourceArn}` | `idempotent` | `IdentitySourceArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes an identity source. For more information, see Identity Source in the Multi-party approval User Guide . |
| `DeleteInactiveApprovalTeamVersion` | `DELETE /approval-teams/{Arn}/{VersionId}` | `idempotent` | `Arn`, `VersionId` | - | `DeleteInactiveApprovalTeamVersionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an inactive approval team. For more information, see Team health in the Multi-party approval User Guide . |
| `GetApprovalTeam` | `GET /approval-teams/{Arn}` | `readonly` | `Arn` | - | `GetApprovalTeamResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns details for an approval team. |
| `GetIdentitySource` | `GET /identity-sources/{IdentitySourceArn}` | `readonly` | `IdentitySourceArn` | - | `GetIdentitySourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns details for an identity source. For more information, see Identity Source in the Multi-party approval User Guide . |
| `GetPolicyVersion` | `GET /policy-versions/{PolicyVersionArn}` | `readonly` | `PolicyVersionArn` | - | `GetPolicyVersionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns details for the version of a policy. Policies define the permissions for team resources. |
| `GetResourcePolicy` | `POST /GetResourcePolicy` | `readonly` | `PolicyName`, `PolicyType`, `ResourceArn` | - | `GetResourcePolicyResponse` | `AccessDeniedException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns details about a policy for a resource. |
| `GetSession` | `GET /sessions/{SessionArn}` | `readonly` | `SessionArn` | - | `GetSessionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns details for an approval session. For more information, see Session in the Multi-party approval User Guide . |
| `ListApprovalTeams` | `POST /approval-teams/?List` | `readonly`, `paginated` | - | - | `ListApprovalTeamsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of approval teams. |
| `ListIdentitySources` | `POST /identity-sources/?List` | `readonly`, `paginated` | - | - | `ListIdentitySourcesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of identity sources. For more information, see Identity Source in the Multi-party approval User Guide . |
| `ListPolicies` | `POST /policies/?List` | `readonly`, `paginated` | - | - | `ListPoliciesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of policies. Policies define the permissions for team resources. |
| `ListPolicyVersions` | `POST /policies/{PolicyArn}/?List` | `readonly`, `paginated` | `PolicyArn` | - | `ListPolicyVersionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of the versions for policies. Policies define the permissions for team resources. |
| `ListResourcePolicies` | `POST /resource-policies/{ResourceArn}/?List` | `readonly`, `paginated` | `ResourceArn` | - | `ListResourcePoliciesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of policies for a resource. |
| `ListSessions` | `POST /approval-teams/{ApprovalTeamArn}/sessions/?List` | `readonly`, `paginated` | `ApprovalTeamArn` | - | `ListSessionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of approval sessions. For more information, see Session in the Multi-party approval User Guide . |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of the tags for a resource. |
| `StartActiveApprovalTeamDeletion` | `POST /approval-teams/{Arn}?Delete` | `idempotent` | `Arn` | - | `StartActiveApprovalTeamDeletionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts the deletion process for an active approval team. Deletions require team approval Requests to delete an active team must be approved by the team. |
| `StartApprovalTeamBaseline` | `POST /approval-teams/{Arn}/baseline` | - | `Arn` | - | `StartApprovalTeamBaselineResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts a baseline session for specified approvers on an `ACTIVE` approval team. |
| `TagResource` | `PUT /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `TooManyTagsException`, `ValidationException` | Creates or updates a resource tag. Each tag is a label consisting of a user-defined key and value. |
| `UntagResource` | `POST /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a resource tag. Each tag is a label consisting of a user-defined key and value. |
| `UpdateApprovalTeam` | `PATCH /approval-teams/{Arn}` | `idempotent` | `Arn` | - | `UpdateApprovalTeamResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates an approval team. You can request to update the team description, approval threshold, and approvers in the team. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListPolicies` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListPolicyVersions` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |
| `ListResourcePolicies` | - | `MaxResults -> MaxResults`, `NextToken -> NextToken` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You do not have sufficient access to perform this action. |
| `ThrottlingException` | `structure` | `Message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `Message` | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `InternalServerException` | `structure` | `Message` | The service encountered an internal error. |
| `ResourceNotFoundException` | `structure` | `Message` | The specified resource doesn't exist. |
| `ConflictException` | `structure` | `Message` | The request cannot be completed because it conflicts with the current state of a resource. |
| `ServiceQuotaExceededException` | `structure` | `Message` | The request exceeds the service quota for your account. |
| `CancelSessionRequest` | `structure` | `SessionArn` | - |
| `CancelSessionResponse` | `structure` | - | - |
| `CreateApprovalTeamRequest` | `structure` | `ApprovalStrategy`, `Approvers`, `ClientToken`, `Description`, `Name`, `Policies`, `Tags` | - |
| `CreateApprovalTeamResponse` | `structure` | `Arn`, `CreationTime`, `Name`, `VersionId` | - |
| `CreateIdentitySourceRequest` | `structure` | `ClientToken`, `IdentitySourceParameters`, `Tags` | - |
| `CreateIdentitySourceResponse` | `structure` | `CreationTime`, `IdentitySourceArn`, `IdentitySourceType` | - |
| `DeleteIdentitySourceRequest` | `structure` | `IdentitySourceArn` | - |
| `DeleteInactiveApprovalTeamVersionRequest` | `structure` | `Arn`, `VersionId` | - |
| `DeleteInactiveApprovalTeamVersionResponse` | `structure` | - | - |
| `GetApprovalTeamRequest` | `structure` | `Arn` | - |
| `GetApprovalTeamResponse` | `structure` | `ApprovalStrategy`, `Approvers`, `Arn`, `CreationTime`, `Description`, `LastUpdateTime`, `Name`, `NumberOfApprovers`, `PendingUpdate`, `Policies`, `Status`, `StatusCode`, ... (+3) | - |
| `GetIdentitySourceRequest` | `structure` | `IdentitySourceArn` | - |
| `GetIdentitySourceResponse` | `structure` | `CreationTime`, `IdentitySourceArn`, `IdentitySourceParameters`, `IdentitySourceType`, `Status`, `StatusCode`, `StatusMessage` | - |
| `GetPolicyVersionRequest` | `structure` | `PolicyVersionArn` | - |
| `GetPolicyVersionResponse` | `structure` | `PolicyVersion` | - |
| `GetResourcePolicyRequest` | `structure` | `PolicyName`, `PolicyType`, `ResourceArn` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
