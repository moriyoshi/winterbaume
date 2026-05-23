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

- Operations: `GetVerification`
- Traits: `readonly` (1)
- Common required input members in this group: -

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Send

- Operations: `SendEmailVerificationCode`
- Common required input members in this group: -

### Start

- Operations: `StartVerification`
- Traits: `idempotent` (1), `idempotency-token` (1)
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
| `GetVerification` | `-` | `readonly` | `VerificationType` | - | `GetVerificationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the current status and details of a verification process for a partner account. This operation allows partners to check the progress and results of business or registrant verification processes. |
| `ListTagsForResource` | `-` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all tags associated with a specific AWS Partner Central Account resource. |
| `SendEmailVerificationCode` | `-` | - | `Catalog`, `Email` | - | `SendEmailVerificationCodeResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Sends an email verification code to the specified email address for account verification purposes. |
| `StartVerification` | `-` | `idempotent`, `idempotency-token` | - | `ClientToken` | `StartVerificationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Initiates a new verification process for a partner account. This operation begins the verification workflow for either business registration or individual registrant identity verification as required by AWS Partner C ... |
| `TagResource` | `-` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds or updates tags for a specified AWS Partner Central Account resource. |
| `UntagResource` | `-` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes specified tags from an AWS Partner Central Account resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message, Reason | The request was denied due to insufficient permissions. The caller does not have the required permissions to perform this operation. |
| `ConflictException` | `structure` | Message, Reason | The request could not be completed due to a conflict with the current state of the resource. This typically occurs when trying to create a resource that alr ... |
| `InternalServerException` | `structure` | Message | An internal server error occurred while processing the request. This is typically a temporary condition and the request may be retried. |
| `ResourceNotFoundException` | `structure` | Message, Reason | The specified resource could not be found. This may occur when referencing a resource that does not exist or has been deleted. |
| `ServiceQuotaExceededException` | `structure` | Message, Reason | The request was rejected because it would exceed a service quota or limit. This may occur when trying to create more resources than allowed by the service l ... |
| `ThrottlingException` | `structure` | Message, ServiceCode, QuotaCode | The request was throttled due to too many requests being sent in a short period of time. The client should implement exponential backoff and retry the request. |
| `ValidationException` | `structure` | Message, Reason, ErrorDetails | The request failed validation. One or more input parameters are invalid, missing, or do not meet the required format or constraints. |
| `GetVerificationRequest` | `structure` | VerificationType | - |
| `GetVerificationResponse` | `structure` | VerificationType, VerificationStatus, VerificationStatusReason, VerificationResponseDetails, StartedAt, CompletedAt | - |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | ResourceArn, Tags | - |
| `SendEmailVerificationCodeRequest` | `structure` | Catalog, Email | - |
| `SendEmailVerificationCodeResponse` | `structure` | **empty (no members)** | - |
| `StartVerificationRequest` | `structure` | ClientToken, VerificationDetails | - |
| `StartVerificationResponse` | `structure` | VerificationType, VerificationStatus, VerificationStatusReason, VerificationResponseDetails, StartedAt, CompletedAt | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `AccessDeniedExceptionReason` | `enum` | ACCESS_DENIED, INCOMPATIBLE_BENEFIT_AWS_PARTNER_STATE | - |
| `AccessType` | `enum` | ALLOW_ALL, DENY_ALL, ALLOW_BY_DEFAULT_DENY_SOME | - |
| `BusinessValidationCode` | `enum` | INCOMPATIBLE_CONNECTION_INVITATION_REQUEST, INCOMPATIBLE_LEGAL_NAME, INCOMPATIBLE_KNOW_YOUR_BUSINESS_STATUS, INCOMPATIBLE_IDENTITY_VERIFICATION_STATUS, INVALID_ACCOUNT_LINKING_STATUS, INVALID_ACCOUNT_STATE, INCOMPATIBLE_DOMAIN | - |
| `ConflictExceptionReason` | `enum` | CONFLICT_CLIENT_TOKEN, DUPLICATE_PARTNER, INCOMPATIBLE_PROFILE_STATE, INCOMPATIBLE_PARTNER_PROFILE_TASK_STATE, DUPLICATE_CONNECTION_INVITATION, INCOMPATIBLE_CONNECTION_INVITATION_STATE, INCOMPATIBLE_CONNECTION_INVITATION_RECEIVER, DUPLICATE_CONNECTION, INCOMPATIBLE_CONNECTION_STATE, INCOMPATIBLE_CONNECTION_PREFERENCES_REVISION, ACCOUNT_ALREADY_VERIFIED, VERIFICATION_ALREADY_IN_PROGRESS | - |
| `ConnectionType` | `enum` | OPPORTUNITY_COLLABORATION, SUBSIDIARY | - |
| `ConnectionTypeStatus` | `enum` | ACTIVE, CANCELED | - |
| `FieldValidationCode` | `enum` | REQUIRED_FIELD_MISSING, DUPLICATE_VALUE, INVALID_VALUE, INVALID_STRING_FORMAT, TOO_MANY_VALUES, ACTION_NOT_PERMITTED, INVALID_ENUM_VALUE | - |
| `IndustrySegment` | `enum` | AGRICULTURE_MINING, BIOTECHNOLOGY, BUSINESS_CONSUMER_SERVICES, BUSINESS_SERV, COMMUNICATIONS, COMPUTER_HARDWARE, COMPUTERS_ELECTRONICS, COMPUTER_SOFTWARE, CONSUMER_GOODS, CONSUMER_RELATED, EDUCATION, ENERGY_UTILITIES, ... (+25) | - |
| `InvitationStatus` | `enum` | PENDING, ACCEPTED, REJECTED, CANCELED, EXPIRED | - |
| `ParticipantType` | `enum` | SENDER, RECEIVER | - |
| `PrimarySolutionType` | `enum` | SOFTWARE_PRODUCTS, CONSULTING_SERVICES, PROFESSIONAL_SERVICES, MANAGED_SERVICES, HARDWARE_PRODUCTS, COMMUNICATION_SERVICES, VALUE_ADDED_RESALE_AWS_SERVICES, TRAINING_SERVICES | - |
| `ProfileTaskStatus` | `enum` | IN_PROGRESS, CANCELED, SUCCEEDED, FAILED | - |
| `ProfileValidationErrorReason` | `enum` | INVALID_CONTENT, DUPLICATE_PROFILE, INVALID_LOGO, INVALID_LOGO_URL, INVALID_LOGO_FILE, INVALID_LOGO_SIZE, INVALID_WEBSITE_URL | - |
| `ProfileVisibility` | `enum` | PRIVATE, PUBLIC | - |
| `ResourceNotFoundExceptionReason` | `enum` | PARTNER_NOT_FOUND, PARTNER_PROFILE_NOT_FOUND, PARTNER_PROFILE_TASK_NOT_FOUND, PARTNER_DOMAIN_NOT_FOUND, SENDER_PROFILE_NOT_FOUND, RECEIVER_PROFILE_NOT_FOUND, CONNECTION_INVITATION_NOT_FOUND, CONNECTION_NOT_FOUND, VERIFICATION_NOT_FOUND | - |
| `ServiceQuotaExceededExceptionReason` | `enum` | LIMIT_EXCEEDED_NUMBER_OF_EMAIL, LIMIT_EXCEEDED_NUMBER_OF_DOMAIN, LIMIT_EXCEEDED_NUMBER_OF_CONNECTION_INVITATION_PER_DAY, LIMIT_EXCEEDED_NUMBER_OF_ACTIVE_CONNECTION, LIMIT_EXCEEDED_NUMBER_OF_OPEN_CONNECTION_INVITATION | - |
| `ValidationExceptionReason` | `enum` | REQUEST_VALIDATION_FAILED, BUSINESS_VALIDATION_FAILED | - |
| `VerificationStatus` | `enum` | PENDING_CUSTOMER_ACTION, IN_PROGRESS, FAILED, SUCCEEDED, REJECTED | - |
| `VerificationType` | `enum` | BUSINESS_VERIFICATION, REGISTRANT_VERIFICATION | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
