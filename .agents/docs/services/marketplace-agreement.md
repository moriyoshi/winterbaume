# AWS Marketplace Agreement Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Marketplace is a curated digital catalog that customers can use to find, buy, deploy, and manage third-party software, data, and services to build solutions and run their businesses. The AWS Marketplace Agreement Service provides an API interface that helps AWS Marketplace sellers manage their product-related agreements, including listing, searching, and filtering agreements. To manage agreements in AWS Marketplace, you must ensure that your AWS Identity and Access Management (IAM) policies and roles are set up. The user must have the required policies/permissions that allow them to carry out the actions in AWS: `DescribeAgreement` – Grants permission to users to obtain detailed meta data about any of their agreements. `GetAgreementTerms` – Grants permission to users to obtain details about the terms of an agreement.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Marketplace Agreement Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Describe`, `Get`, `Search` operation families, including `DescribeAgreement`, `GetAgreementTerms`, `SearchAgreements`.

## Service Identity and Protocol

- AWS model slug: `marketplace-agreement`
- AWS SDK for Rust slug: `marketplaceagreement`
- Model version: `2020-03-01`
- Model file: `vendor/api-models-aws/models/marketplace-agreement/service/2020-03-01/marketplace-agreement-2020-03-01.json`
- SDK ID: `Marketplace Agreement`
- Endpoint prefix: `agreement-marketplace`
- ARN namespace: `-`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Describe` (1), `Get` (1), `Search` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAgreement`, `GetAgreementTerms`, `SearchAgreements`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- 3 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Describe

- Operations: `DescribeAgreement`
- Common required input members in this group: `agreementId`

### Get

- Operations: `GetAgreementTerms`
- Traits: `paginated` (1)
- Common required input members in this group: `agreementId`

### Search

- Operations: `SearchAgreements`
- Traits: `paginated` (1)

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DescribeAgreement` | - | - | `agreementId` | - | `DescribeAgreementOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provides details about an agreement, such as the proposer, acceptor, start date, and end date. |
| `GetAgreementTerms` | - | `paginated` | `agreementId` | - | `GetAgreementTermsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Obtains details about the terms in an agreement that you participated in as proposer or acceptor. The details include: `TermType` – The type of term, such as `LegalTerm`, `RenewalTerm`, or `ConfigurableUpfrontPricingTerm`. |
| `SearchAgreements` | - | `paginated` | - | - | `SearchAgreementsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Searches across all agreements that a proposer has in AWS Marketplace. The search returns a list of agreements with basic agreement information. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message`, `requestId` | User does not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message`, `requestId` | Unexpected error during processing of request. |
| `ThrottlingException` | `structure` | `message`, `requestId` | Request was denied due to request throttling. |
| `ValidationException` | `structure` | `fields`, `message`, `reason`, `requestId` | The input fails to satisfy the constraints specified by the service. |
| `ResourceNotFoundException` | `structure` | `message`, `requestId`, `resourceId`, `resourceType` | Request references a resource which does not exist. |
| `DescribeAgreementInput` | `structure` | `agreementId` | - |
| `DescribeAgreementOutput` | `structure` | `acceptanceTime`, `acceptor`, `agreementId`, `agreementType`, `endTime`, `estimatedCharges`, `proposalSummary`, `proposer`, `startTime`, `status` | - |
| `GetAgreementTermsInput` | `structure` | `agreementId`, `maxResults`, `nextToken` | - |
| `GetAgreementTermsOutput` | `structure` | `acceptedTerms`, `nextToken` | - |
| `SearchAgreementsInput` | `structure` | `catalog`, `filters`, `maxResults`, `nextToken`, `sort` | - |
| `SearchAgreementsOutput` | `structure` | `agreementViewSummaries`, `nextToken` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
