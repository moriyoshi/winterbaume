# Partner Central Account API

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Partner Central Account service provides APIs for managing partner accounts, connections, and profiles within the AWS Partner Network. This service enables partners to create and manage their partner profiles, establish connections with other partners, and maintain their account information.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Partner Central Account API where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Partner Central Account API by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Partner Central Account API by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Partner Central Account API workflows in the local mock. Key resources include `ConnectionInvitation`, `ConnectionPreferences`, `ConnectionResource`, `Partner`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Cancel`, `Create`, `Put` operation families, including `GetAllianceLeadContact`, `GetConnection`, `GetConnectionInvitation`, `GetConnectionPreferences`, `ListConnectionInvitations`, `ListConnections`.

## Service Identity and Protocol

- AWS model slug: `partnercentral-account`
- AWS SDK for Rust slug: `partnercentralaccount`
- Model version: `2025-04-04`
- Model file: `vendor/api-models-aws/models/partnercentral-account/service/2025-04-04/partnercentral-account-2025-04-04.json`
- SDK ID: `PartnerCentral Account`
- Endpoint prefix: `partnercentral-account`
- ARN namespace: `partnercentral`
- CloudFormation name: `-`
- CloudTrail event source: `partnercentral-account.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (8), `List` (4), `Cancel` (3), `Create` (2), `Put` (2), `Start` (2), `Accept` (1), `Associate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptConnectionInvitation`, `AssociateAwsTrainingCertificationEmailDomain`, `CancelConnection`, `CancelConnectionInvitation`, `CancelProfileUpdateTask`, `CreateConnectionInvitation`, `CreatePartner`, `DisassociateAwsTrainingCertificationEmailDomain`, `PutAllianceLeadContact`, `PutProfileVisibility`, `RejectConnectionInvitation`, `StartProfileUpdateTask`, `StartVerification`, `TagResource`, `UntagResource`, `UpdateConnectionPreferences`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAllianceLeadContact`, `GetConnection`, `GetConnectionInvitation`, `GetConnectionPreferences`, `GetPartner`, `GetProfileUpdateTask`, `GetProfileVisibility`, `GetVerification`, `ListConnectionInvitations`, `ListConnections`, `ListPartners`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 14 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelConnection`, `CancelConnectionInvitation`, `CancelProfileUpdateTask`, `GetProfileUpdateTask`, `StartProfileUpdateTask`, `StartVerification`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 29 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `ECS`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ConnectionInvitation` | `Catalog`, `Identifier` | create: `CreateConnectionInvitation`; read: `GetConnectionInvitation`; list: `ListConnectionInvitations` | `AcceptConnectionInvitation`, `CancelConnectionInvitation`, `RejectConnectionInvitation` | - |
| `ConnectionPreferences` | `Catalog` | read: `GetConnectionPreferences` | `UpdateConnectionPreferences` | - |
| `ConnectionResource` | `Catalog`, `Identifier` | read: `GetConnection`; list: `ListConnections` | `CancelConnection` | - |
| `Partner` | `Catalog`, `Identifier` | create: `CreatePartner`; read: `GetPartner`; list: `ListPartners` | `AssociateAwsTrainingCertificationEmailDomain`, `CancelProfileUpdateTask`, `DisassociateAwsTrainingCertificationEmailDomain`, `GetAllianceLeadContact`, `GetProfileUpdateTask`, `GetProfileVisibility`, `PutAllianceLeadContact`, `PutProfileVisibility`, `StartProfileUpdateTask` | - |
## Operation Groups

### Get

- Operations: `GetAllianceLeadContact`, `GetConnection`, `GetConnectionInvitation`, `GetConnectionPreferences`, `GetPartner`, `GetProfileUpdateTask`, `GetProfileVisibility`, `GetVerification`
- Traits: `readonly` (8)
- Common required input members in this group: `Catalog`, `Identifier`, `VerificationType`

### List

- Operations: `ListConnectionInvitations`, `ListConnections`, `ListPartners`, `ListTagsForResource`
- Traits: `paginated` (3), `readonly` (4)
- Common required input members in this group: `Catalog`, `ResourceArn`

### Cancel

- Operations: `CancelConnection`, `CancelConnectionInvitation`, `CancelProfileUpdateTask`
- Traits: `idempotency-token` (3)
- Common required input members in this group: `Catalog`, `ClientToken`, `ConnectionType`, `Identifier`, `Reason`, `TaskId`

### Create

- Operations: `CreateConnectionInvitation`, `CreatePartner`
- Traits: `idempotency-token` (2), `idempotent` (1)
- Common required input members in this group: `AllianceLeadContact`, `Catalog`, `ClientToken`, `ConnectionType`, `Email`, `EmailVerificationCode`, `LegalName`, `Message`, `Name`, `PrimarySolutionType`, `ReceiverIdentifier`

### Put

- Operations: `PutAllianceLeadContact`, `PutProfileVisibility`
- Common required input members in this group: `AllianceLeadContact`, `Catalog`, `Identifier`, `Visibility`

### Start

- Operations: `StartProfileUpdateTask`, `StartVerification`
- Traits: `idempotency-token` (2), `idempotent` (2)
- Common required input members in this group: `Catalog`, `Identifier`, `TaskDetails`

### Accept

- Operations: `AcceptConnectionInvitation`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Catalog`, `ClientToken`, `Identifier`

### Associate

- Operations: `AssociateAwsTrainingCertificationEmailDomain`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `Catalog`, `Email`, `EmailVerificationCode`, `Identifier`

### Disassociate

- Operations: `DisassociateAwsTrainingCertificationEmailDomain`
- Traits: `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `Catalog`, `DomainName`, `Identifier`

### Reject

- Operations: `RejectConnectionInvitation`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Catalog`, `ClientToken`, `Identifier`

### Send

- Operations: `SendEmailVerificationCode`
- Common required input members in this group: `Catalog`, `Email`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Update

- Operations: `UpdateConnectionPreferences`
- Traits: `idempotent` (1)
- Common required input members in this group: `AccessType`, `Catalog`, `Revision`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptConnectionInvitation` | - | `idempotency-token` | `Catalog`, `ClientToken`, `Identifier` | `ClientToken` | `AcceptConnectionInvitationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Accepts a connection invitation from another partner, establishing a formal partnership connection between the two parties. |
| `AssociateAwsTrainingCertificationEmailDomain` | - | `idempotent`, `idempotency-token` | `Catalog`, `Email`, `EmailVerificationCode`, `Identifier` | `ClientToken` | `AssociateAwsTrainingCertificationEmailDomainResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates an email domain with AWS training and certification for the partner account, enabling automatic verification of employee certifications. |
| `CancelConnection` | - | `idempotency-token` | `Catalog`, `ClientToken`, `ConnectionType`, `Identifier`, `Reason` | `ClientToken` | `CancelConnectionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels an existing connection between partners, terminating the partnership relationship. |
| `CancelConnectionInvitation` | - | `idempotency-token` | `Catalog`, `ClientToken`, `Identifier` | `ClientToken` | `CancelConnectionInvitationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels a pending connection invitation before it has been accepted or rejected. |
| `CancelProfileUpdateTask` | - | `idempotency-token` | `Catalog`, `Identifier`, `TaskId` | `ClientToken` | `CancelProfileUpdateTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels an in-progress profile update task, stopping any pending changes to the partner profile. |
| `CreateConnectionInvitation` | - | `idempotency-token` | `Catalog`, `ClientToken`, `ConnectionType`, `Email`, `Message`, `Name`, `ReceiverIdentifier` | `ClientToken` | `CreateConnectionInvitationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a new connection invitation to establish a partnership with another organization. |
| `CreatePartner` | - | `idempotent`, `idempotency-token` | `AllianceLeadContact`, `Catalog`, `EmailVerificationCode`, `LegalName`, `PrimarySolutionType` | `ClientToken` | `CreatePartnerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a new partner account in the AWS Partner Network with the specified details and configuration. |
| `DisassociateAwsTrainingCertificationEmailDomain` | - | `idempotent`, `idempotency-token` | `Catalog`, `DomainName`, `Identifier` | `ClientToken` | `DisassociateAwsTrainingCertificationEmailDomainResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the association between an email domain and AWS training and certification for the partner account. |
| `GetAllianceLeadContact` | - | `readonly` | `Catalog`, `Identifier` | - | `GetAllianceLeadContactResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the alliance lead contact information for a partner account. |
| `GetConnection` | - | `readonly` | `Catalog`, `Identifier` | - | `GetConnectionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a specific connection between partners. |
| `GetConnectionInvitation` | - | `readonly` | `Catalog`, `Identifier` | - | `GetConnectionInvitationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a specific connection invitation. |
| `GetConnectionPreferences` | - | `readonly` | `Catalog` | - | `GetConnectionPreferencesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves the connection preferences for a partner account, including access settings and exclusions. |
| `GetPartner` | - | `readonly` | `Catalog`, `Identifier` | - | `GetPartnerResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves detailed information about a specific partner account. |
| `GetProfileUpdateTask` | - | `readonly` | `Catalog`, `Identifier` | - | `GetProfileUpdateTaskResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a specific profile update task. |
| `GetProfileVisibility` | - | `readonly` | `Catalog`, `Identifier` | - | `GetProfileVisibilityResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the visibility settings for a partner profile, determining who can see the profile information. |
| `GetVerification` | - | `readonly` | `VerificationType` | - | `GetVerificationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the current status and details of a verification process for a partner account. This operation allows partners to check the progress and results of business or registrant verification processes. |
| `ListConnectionInvitations` | - | `readonly`, `paginated` | `Catalog` | - | `ListConnectionInvitationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists connection invitations for the partner account, with optional filtering by status, type, and other criteria. |
| `ListConnections` | - | `readonly`, `paginated` | `Catalog` | - | `ListConnectionsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists active connections for the partner account, with optional filtering by connection type and participant. |
| `ListPartners` | - | `readonly`, `paginated` | `Catalog` | - | `ListPartnersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists partner accounts in the catalog, providing a summary view of all partners. |
| `ListTagsForResource` | - | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all tags associated with a specific AWS Partner Central Account resource. |
| `PutAllianceLeadContact` | - | - | `AllianceLeadContact`, `Catalog`, `Identifier` | - | `PutAllianceLeadContactResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates or updates the alliance lead contact information for a partner account. |
| `PutProfileVisibility` | - | - | `Catalog`, `Identifier`, `Visibility` | - | `PutProfileVisibilityResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sets the visibility level for a partner profile, controlling who can view the profile information. |
| `RejectConnectionInvitation` | - | `idempotency-token` | `Catalog`, `ClientToken`, `Identifier` | `ClientToken` | `RejectConnectionInvitationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Rejects a connection invitation from another partner, declining the partnership request. |
| `SendEmailVerificationCode` | - | - | `Catalog`, `Email` | - | `SendEmailVerificationCodeResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Sends an email verification code to the specified email address for account verification purposes. |
| `StartProfileUpdateTask` | - | `idempotent`, `idempotency-token` | `Catalog`, `Identifier`, `TaskDetails` | `ClientToken` | `StartProfileUpdateTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Initiates a profile update task to modify partner profile information asynchronously. |
| `StartVerification` | - | `idempotent`, `idempotency-token` | - | `ClientToken` | `StartVerificationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Initiates a new verification process for a partner account. This operation begins the verification workflow for either business registration or individual registrant identity verification as required by AWS Partner Central. |
| `TagResource` | - | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds or updates tags for a specified AWS Partner Central Account resource. |
| `UntagResource` | - | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes specified tags from an AWS Partner Central Account resource. |
| `UpdateConnectionPreferences` | - | `idempotent` | `AccessType`, `Catalog`, `Revision` | - | `UpdateConnectionPreferencesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Updates the connection preferences for a partner account, modifying access settings and exclusions. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message`, `Reason` | The request was denied due to insufficient permissions. |
| `ThrottlingException` | `structure` | `Message`, `QuotaCode`, `ServiceCode` | The request was throttled due to too many requests being sent in a short period of time. |
| `ValidationException` | `structure` | `ErrorDetails`, `Message`, `Reason` | The request failed validation. |
| `InternalServerException` | `structure` | `Message` | An internal server error occurred while processing the request. |
| `ResourceNotFoundException` | `structure` | `Message`, `Reason` | The specified resource could not be found. |
| `ConflictException` | `structure` | `Message`, `Reason` | The request could not be completed due to a conflict with the current state of the resource. |
| `ServiceQuotaExceededException` | `structure` | `Message`, `Reason` | The request was rejected because it would exceed a service quota or limit. |
| `AcceptConnectionInvitationRequest` | `structure` | `Catalog`, `ClientToken`, `Identifier` | - |
| `AcceptConnectionInvitationResponse` | `structure` | `Connection` | - |
| `AssociateAwsTrainingCertificationEmailDomainRequest` | `structure` | `Catalog`, `ClientToken`, `Email`, `EmailVerificationCode`, `Identifier` | - |
| `AssociateAwsTrainingCertificationEmailDomainResponse` | `structure` | - | - |
| `CancelConnectionRequest` | `structure` | `Catalog`, `ClientToken`, `ConnectionType`, `Identifier`, `Reason` | - |
| `CancelConnectionResponse` | `structure` | `Arn`, `Catalog`, `ConnectionTypes`, `Id`, `OtherParticipantAccountId`, `UpdatedAt` | - |
| `CancelConnectionInvitationRequest` | `structure` | `Catalog`, `ClientToken`, `Identifier` | - |
| `CancelConnectionInvitationResponse` | `structure` | `Arn`, `Catalog`, `ConnectionId`, `ConnectionType`, `CreatedAt`, `ExpiresAt`, `Id`, `InvitationMessage`, `InviterEmail`, `InviterName`, `OtherParticipantIdentifier`, `ParticipantType`, ... (+2) | - |
| `CancelProfileUpdateTaskRequest` | `structure` | `Catalog`, `ClientToken`, `Identifier`, `TaskId` | - |
| `CancelProfileUpdateTaskResponse` | `structure` | `Arn`, `Catalog`, `EndedAt`, `ErrorDetailList`, `Id`, `StartedAt`, `Status`, `TaskDetails`, `TaskId` | - |
| `CreateConnectionInvitationRequest` | `structure` | `Catalog`, `ClientToken`, `ConnectionType`, `Email`, `Message`, `Name`, `ReceiverIdentifier` | - |
| `CreateConnectionInvitationResponse` | `structure` | `Arn`, `Catalog`, `ConnectionId`, `ConnectionType`, `CreatedAt`, `ExpiresAt`, `Id`, `InvitationMessage`, `InviterEmail`, `InviterName`, `OtherParticipantIdentifier`, `ParticipantType`, ... (+2) | - |
| `CreatePartnerRequest` | `structure` | `AllianceLeadContact`, `Catalog`, `ClientToken`, `EmailVerificationCode`, `LegalName`, `PrimarySolutionType`, `Tags` | - |
| `CreatePartnerResponse` | `structure` | `AllianceLeadContact`, `Arn`, `AwsTrainingCertificationEmailDomains`, `Catalog`, `CreatedAt`, `Id`, `LegalName`, `Profile` | - |
| `DisassociateAwsTrainingCertificationEmailDomainRequest` | `structure` | `Catalog`, `ClientToken`, `DomainName`, `Identifier` | - |
| `DisassociateAwsTrainingCertificationEmailDomainResponse` | `structure` | - | - |
| `GetAllianceLeadContactRequest` | `structure` | `Catalog`, `Identifier` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
