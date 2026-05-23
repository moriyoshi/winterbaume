# AWS Account

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Operations for Amazon Web Services Account Management

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for AWS Account by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: manage account-level contact, alternate contact, Region, and primary contact metadata across an AWS account estate.
- From the operation surface: read and update account attributes, enable or disable Regions, and use deletion or close-account flows where account lifecycle automation is in scope.

## Service Identity and Protocol

- AWS model slug: `account`
- AWS SDK for Rust slug: `account`
- Model version: `2021-02-01`
- Model file: `vendor/api-models-aws/models/account/service/2021-02-01/account-2021-02-01.json`
- SDK ID: `Account`
- Endpoint prefix: `-`
- ARN namespace: `account`
- CloudFormation name: `-`
- CloudTrail event source: `CLOUDTRAIL_EVENT_SOURCE`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (6), `Put` (3), `Accept` (1), `Delete` (1), `Disable` (1), `Enable` (1), `List` (1), `Start` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptPrimaryEmailUpdate`, `DeleteAlternateContact`, `DisableRegion`, `EnableRegion`, `PutAccountName`, `PutAlternateContact`, `PutContactInformation`, `StartPrimaryEmailUpdate`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccountInformation`, `GetAlternateContact`, `GetContactInformation`, `GetGovCloudAccountInformation`, `GetPrimaryEmail`, `GetRegionOptStatus`, `ListRegions`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartPrimaryEmailUpdate`.
- 15 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `STS`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AccountNameResource` | - | put: `PutAccountName`; read: `GetAccountInformation` | - | - |
| `AlternateContactResource` | `AlternateContactType` | put: `PutAlternateContact`; read: `GetAlternateContact`; delete: `DeleteAlternateContact` | - | - |
| `CommercialToGovCloudGatewayResource` | - | - | `GetGovCloudAccountInformation` | - |
| `ContactInformationResource` | - | put: `PutContactInformation`; read: `GetContactInformation` | - | - |
| `PrimaryEmailResource` | - | - | `AcceptPrimaryEmailUpdate`, `GetPrimaryEmail`, `StartPrimaryEmailUpdate` | - |
| `RegionOptResource` | - | - | `DisableRegion`, `EnableRegion`, `GetRegionOptStatus`, `ListRegions` | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/accounts/latest/reference/manage-acct-regions.html
- https://docs.aws.amazon.com/accounts/latest/reference/manage-acct-identifiers.html
- https://docs.aws.amazon.com/accounts/latest/reference/manage-acct-alias.html

Research outcomes:
- AWS Account Management APIs expose account-level attributes such as account identifiers, contact data, alternate contacts, Region opt-in state, and account alias configuration.
- Account identifiers are stable resource identifiers used by IAM policies, ARNs, billing views, and organisation relationships.
- Region opt-in controls whether opt-in Regions are enabled or disabled for an account. Disabled Regions should not be treated as usable deployment targets.
- Account aliases are friendly sign-in aliases, distinct from the immutable 12-digit account id.
- Alternate contacts are account-scoped billing, operations, and security contacts used for AWS communications and should not be confused with IAM identities.
- Management-account and member-account visibility depends on the AWS Organizations relationship and the caller's delegated or management permissions.

Parity implications:
- Model immutable account id, account alias, primary/alternate contacts, and Region opt-in state independently.
- Region enablement should affect service availability checks in opt-in Regions.
- Account metadata APIs should distinguish standalone, management, delegated, and member-account access paths.

## Operation Groups

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message, errorType | The operation failed because the calling identity doesn't have the minimum required permissions. |
| `ConflictException` | `structure` | message, errorType | The request could not be processed because of a conflict in the current status of the resource. For example, this happens if you try to enable a Region that ... |
| `InternalServerException` | `structure` | message, errorType | The operation failed because of an error internal to Amazon Web Services. Try your operation again later. |
| `ResourceNotFoundException` | `structure` | message, errorType | The operation failed because it specified a resource that can't be found. |
| `ResourceUnavailableException` | `structure` | message, errorType | The operation failed because it specified a resource that is not currently available. |
| `TooManyRequestsException` | `structure` | message, errorType | The operation failed because it was called too frequently and exceeded a throttle limit. |
| `ValidationException` | `structure` | message, reason, fieldList | The operation failed because one of the input parameters was invalid. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
