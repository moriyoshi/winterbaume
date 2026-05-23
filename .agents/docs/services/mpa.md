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

- Operations: `ListPolicies`, `ListPolicyVersions`, `ListResourcePolicies`, `ListTagsForResource`
- Traits: `readonly` (4), `paginated` (3)
- Common required input members in this group: `ResourceArn`

### Get

- Operations: `GetPolicyVersion`, `GetResourcePolicy`
- Traits: `readonly` (2)
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
| `GetPolicyVersion` | `GET /policy-versions/{PolicyVersionArn}` | `readonly` | `PolicyVersionArn` | - | `GetPolicyVersionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns details for the version of a policy. Policies define the permissions for team resources. |
| `GetResourcePolicy` | `POST /GetResourcePolicy` | `readonly` | `ResourceArn`, `PolicyName`, `PolicyType` | - | `GetResourcePolicyResponse` | `AccessDeniedException`, `InvalidParameterException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns details about a policy for a resource. |
| `ListPolicies` | `POST /policies/?List` | `readonly`, `paginated` | - | - | `ListPoliciesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of policies. Policies define the permissions for team resources. |
| `ListPolicyVersions` | `POST /policies/{PolicyArn}/?List` | `readonly`, `paginated` | `PolicyArn` | - | `ListPolicyVersionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of the versions for policies. Policies define the permissions for team resources. |
| `ListResourcePolicies` | `POST /resource-policies/{ResourceArn}/?List` | `readonly`, `paginated` | `ResourceArn` | - | `ListResourcePoliciesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of policies for a resource. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of the tags for a resource. |
| `TagResource` | `PUT /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `TooManyTagsException`, `ValidationException` | Creates or updates a resource tag. Each tag is a label consisting of a user-defined key and value. Tags can help you manage, identify, organize, search for, and filter resources. |
| `UntagResource` | `POST /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a resource tag. Each tag is a label consisting of a user-defined key and value. Tags can help you manage, identify, organize, search for, and filter resources. |

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
| `AccessDeniedException` | `structure` | Message | You do not have sufficient access to perform this action. Check your permissions, and try again. |
| `ConflictException` | `structure` | Message | The request cannot be completed because it conflicts with the current state of a resource. |
| `InternalServerException` | `structure` | Message | The service encountered an internal error. Try your request again. If the problem persists, contact Amazon Web Services Support. |
| `InvalidParameterException` | `structure` | Message | The request contains an invalid parameter value. |
| `ResourceNotFoundException` | `structure` | Message | The specified resource doesn't exist. Check the resource ID, and try again. |
| `ServiceQuotaExceededException` | `structure` | Message | The request exceeds the service quota for your account. Request a quota increase or reduce your request size. |
| `ThrottlingException` | `structure` | Message | The request was denied due to request throttling. |
| `TooManyTagsException` | `structure` | Message, ResourceName | The request exceeds the maximum number of tags allowed for this resource. Remove some tags, and try again. |
| `ValidationException` | `structure` | Message | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `GetPolicyVersionRequest` | `structure` | PolicyVersionArn | - |
| `GetPolicyVersionResponse` | `structure` | PolicyVersion | - |
| `GetResourcePolicyRequest` | `structure` | ResourceArn, PolicyName, PolicyType | - |
| `GetResourcePolicyResponse` | `structure` | ResourceArn, PolicyType, PolicyVersionArn, PolicyName, PolicyDocument | - |
| `ListPoliciesRequest` | `structure` | MaxResults, NextToken | - |
| `ListPoliciesResponse` | `structure` | NextToken, Policies | - |
| `ListPolicyVersionsRequest` | `structure` | MaxResults, NextToken, PolicyArn | - |
| `ListPolicyVersionsResponse` | `structure` | NextToken, PolicyVersions | - |
| `ListResourcePoliciesRequest` | `structure` | ResourceArn, MaxResults, NextToken | - |
| `ListResourcePoliciesResponse` | `structure` | NextToken, ResourcePolicies | - |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `ActionCompletionStrategy` | `enum` | AUTO_COMPLETION_UPON_APPROVAL | - |
| `AdditionalSecurityRequirement` | `enum` | APPROVER_VERIFICATION_REQUIRED | Additional security requirements applied to a session or invitation APPROVER_VERIFICATION_REQUIRED : Approvers will be required to perform an MFA challenge ... |
| `ApprovalTeamStatus` | `enum` | ACTIVE, INACTIVE, DELETING, PENDING | - |
| `ApprovalTeamStatusCode` | `enum` | VALIDATING, PENDING_ACTIVATION, FAILED_VALIDATION, FAILED_ACTIVATION, UPDATE_PENDING_APPROVAL, UPDATE_PENDING_ACTIVATION, UPDATE_FAILED_APPROVAL, UPDATE_FAILED_ACTIVATION, UPDATE_FAILED_VALIDATION, DELETE_PENDING_APPROVAL, DELETE_FAILED_APPROVAL, DELETE_FAILED_VALIDATION | - |
| `ApproverLastActivity` | `enum` | VOTED, BASELINED, RESPONDED_TO_INVITATION | - |
| `FilterField` | `enum` | ACTION_NAME, APPROVAL_TEAM_NAME, VOTING_TIME, VOTE, SESSION_STATUS, INITIATION_TIME | - |
| `IdentitySourceStatus` | `enum` | CREATING, ACTIVE, DELETING, ERROR | - |
| `IdentitySourceStatusCode` | `enum` | ACCESS_DENIED, DELETION_FAILED, IDC_INSTANCE_NOT_FOUND, IDC_INSTANCE_NOT_VALID | - |
| `IdentitySourceType` | `enum` | IAM_IDENTITY_CENTER | - |
| `IdentityStatus` | `enum` | PENDING, ACCEPTED, REJECTED, INVALID | - |
| `MfaSyncStatus` | `enum` | IN_SYNC, OUT_OF_SYNC | Indicates if the approver's MFA device is in-sync with the Identity Source IN_SYNC : The approver's MFA device is in-sync with the Identity Source OUT_OF_SY ... |
| `MfaType` | `enum` | EMAIL_OTP | The type of MFA device used by the approver EMAIL_OTP : The approver will receive emailed one-time passwords to their primary email |
| `Operator` | `enum` | EQUALS, NOT_EQUALS, GREATER_THAN, LESS_THAN, GREATER_THAN_OR_EQUAL_TO, LESS_THAN_OR_EQUAL_TO, CONTAINS, DOES_NOT_CONTAIN, BETWEEN | - |
| `PolicyStatus` | `enum` | ATTACHABLE, DEPRECATED | - |
| `PolicyType` | `enum` | AWS_MANAGED, AWS_RAM | - |
| `SessionExecutionStatus` | `enum` | EXECUTED, FAILED, PENDING | - |
| `SessionResponse` | `enum` | APPROVED, REJECTED, NO_RESPONSE | - |
| `SessionStatus` | `enum` | PENDING, CANCELLED, APPROVED, FAILED, CREATING | - |
| `SessionStatusCode` | `enum` | REJECTED, EXPIRED, CONFIGURATION_CHANGED, ALL_APPROVERS_IN_SESSION | - |
| `UpdateAction` | `enum` | SYNCHRONIZE_MFA_DEVICES | Actions that can be taken when updating an approval team SYNCHRONIZE_MFA_DEVICES : Synchronize MFA devices for all approvers on the team |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
