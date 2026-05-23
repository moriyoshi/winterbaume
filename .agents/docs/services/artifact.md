# AWS Artifact

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This reference provides descriptions of the low-level AWS Artifact Service API.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for AWS Artifact by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: retrieve compliance reports, agreement metadata, and customer agreement documents for audit and procurement workflows.
- From the operation surface: model report discovery, agreement acceptance/termination, account or organisation agreement state, and long-lived compliance evidence retrieval.

## Service Identity and Protocol

- AWS model slug: `artifact`
- AWS SDK for Rust slug: `artifact`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/artifact/service/2018-05-10/artifact-2018-05-10.json`
- SDK ID: `Artifact`
- Endpoint prefix: `-`
- ARN namespace: `artifact`
- CloudFormation name: `-`
- CloudTrail event source: `artifact.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (4), `List` (3), `Put` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `PutAccountSettings`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccountSettings`, `GetReport`, `GetReportMetadata`, `GetTermForReport`, `ListCustomerAgreements`, `ListReportVersions`, `ListReports`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetReport`, `GetReportMetadata`, `GetTermForReport`, `ListReportVersions`, `ListReports`.
- 8 operations declare modelled service errors; parity work should map exact error names and retryability where documented.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AccountSettingsResource` | - | - | `GetAccountSettings`, `PutAccountSettings` | - |
| `CustomerAgreementResource` | - | - | `ListCustomerAgreements` | - |
| `ReportResource` | `reportId` | read: `GetReportMetadata`; list: `ListReports` | `GetReport`, `GetTermForReport`, `ListReportVersions` | - |
| `TermResource` | `termId` | - | - | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/artifact/latest/ug/what-is-aws-artifact.html
- https://docs.aws.amazon.com/artifact/latest/ug/managing-agreements.html
- https://docs.aws.amazon.com/artifact/latest/ug/accept-org-agreement.html

Research outcomes:
- AWS Artifact provides on-demand access to AWS security and compliance reports, certifications, and agreements.
- Account agreements can be accepted, terminated, and reactivated for a single AWS account.
- Organisation agreements can be accepted by management account owners and apply to member accounts when AWS Organizations all-features mode is enabled.
- Terminating an organisation agreement removes member-account coverage.
- Agreement access and report access are controlled by IAM permissions.
- Agreements and reports are compliance artefacts, not resource deployments.

Parity implications:
- Model reports, report packages, agreements, account agreement state, organisation agreement state, terms acceptance, and termination separately.
- Organisation agreement acceptance should depend on management-account context and organisation feature mode.
- Report retrieval should enforce IAM access without creating cloud resources.

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
| `AccessDeniedException` | `structure` | message | User does not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message, resourceId, resourceType | Request to create/modify content would result in a conflict. |
| `InternalServerException` | `structure` | message, retryAfterSeconds | An unknown server exception has occurred. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | Request references a resource which does not exist. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, serviceCode, quotaCode | Request would cause a service quota to be exceeded. |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds | Request was denied due to request throttling. |
| `ValidationException` | `structure` | message, reason, fieldList | Request fails to satisfy the constraints specified by an AWS service. |
| `AcceptanceType` | `enum` | PASSTHROUGH, EXPLICIT | - |
| `AgreementType` | `enum` | CUSTOM, DEFAULT, MODIFIED | - |
| `CustomerAgreementState` | `enum` | ACTIVE, CUSTOMER_TERMINATED, AWS_TERMINATED | - |
| `NotificationSubscriptionStatus` | `enum` | SUBSCRIBED, NOT_SUBSCRIBED | - |
| `PublishedState` | `enum` | PUBLISHED, UNPUBLISHED | - |
| `UploadState` | `enum` | PROCESSING, COMPLETE, FAILED, FAULT | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
