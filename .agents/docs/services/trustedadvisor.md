# TrustedAdvisor Public API

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

TrustedAdvisor Public API

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented TrustedAdvisor Public API workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `List`, `Get`, `Update`, `Batch` operation families, including `ListChecks`, `ListOrganizationRecommendationAccounts`, `ListOrganizationRecommendationResources`, `ListOrganizationRecommendations`, `GetOrganizationRecommendation`, `GetRecommendation`.

## Service Identity and Protocol

- AWS model slug: `trustedadvisor`
- AWS SDK for Rust slug: `trustedadvisor`
- Model version: `2022-09-15`
- Model file: `vendor/api-models-aws/models/trustedadvisor/service/2022-09-15/trustedadvisor-2022-09-15.json`
- SDK ID: `TrustedAdvisor`
- Endpoint prefix: `-`
- ARN namespace: `trustedadvisor`
- CloudFormation name: `-`
- CloudTrail event source: `trustedadvisor.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (6), `Get` (2), `Update` (2), `Batch` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchUpdateRecommendationResourceExclusion`, `UpdateOrganizationRecommendationLifecycle`, `UpdateRecommendationLifecycle`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetOrganizationRecommendation`, `GetRecommendation`, `ListChecks`, `ListOrganizationRecommendationAccounts`, `ListOrganizationRecommendationResources`, `ListOrganizationRecommendations`, `ListRecommendationResources`, `ListRecommendations`.
- Pagination is modelled for 6 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 3 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- 11 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListChecks`, `ListOrganizationRecommendationAccounts`, `ListOrganizationRecommendationResources`, `ListOrganizationRecommendations`, `ListRecommendationResources`, `ListRecommendations`
- Traits: `readonly` (6), `paginated` (6)
- Common required input members in this group: `organizationRecommendationIdentifier`

### Get

- Operations: `GetOrganizationRecommendation`, `GetRecommendation`
- Traits: `readonly` (2)
- Common required input members in this group: -

### Update

- Operations: `UpdateOrganizationRecommendationLifecycle`, `UpdateRecommendationLifecycle`
- Traits: `idempotent` (2)
- Common required input members in this group: `lifecycleStage`

### Batch

- Operations: `BatchUpdateRecommendationResourceExclusion`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchUpdateRecommendationResourceExclusion` | `PUT /v1/batch-update-recommendation-resource-exclusion` | `idempotent` | `recommendationResourceExclusions` | - | `BatchUpdateRecommendationResourceExclusionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Update one or more exclusion statuses for a list of recommendation resources. This API supports up to 25 unique recommendation resource ARNs per request. This API currently doesn't support prioritized recommendation ... |
| `GetOrganizationRecommendation` | `GET /v1/organization-recommendations/{organizationRecommendationIdentifier}` | `readonly` | `organizationRecommendationIdentifier` | - | `GetOrganizationRecommendationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get a specific recommendation within an AWS Organizations organization. This API supports only prioritized recommendations and provides global priority recommendations, eliminating the need to call the API in each AW ... |
| `GetRecommendation` | `GET /v1/recommendations/{recommendationIdentifier}` | `readonly` | `recommendationIdentifier` | - | `GetRecommendationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get a specific Recommendation. This API provides global recommendations, eliminating the need to call the API in each AWS Region. |
| `ListChecks` | `GET /v1/checks` | `readonly`, `paginated` | - | - | `ListChecksResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List a filterable set of Checks. This API provides global recommendations, eliminating the need to call the API in each AWS Region. |
| `ListOrganizationRecommendationAccounts` | `GET /v1/organization-recommendations/{organizationRecommendationIdentifier}/accounts` | `readonly`, `paginated` | `organizationRecommendationIdentifier` | - | `ListOrganizationRecommendationAccountsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the accounts that own the resources for an organization aggregate recommendation. This API only supports prioritized recommendations and provides global priority recommendations, eliminating the need to call th ... |
| `ListOrganizationRecommendationResources` | `GET /v1/organization-recommendations/{organizationRecommendationIdentifier}/resources` | `readonly`, `paginated` | `organizationRecommendationIdentifier` | - | `ListOrganizationRecommendationResourcesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List Resources of a Recommendation within an Organization. This API only supports prioritized recommendations and provides global priority recommendations, eliminating the need to call the API in each AWS Region. |
| `ListOrganizationRecommendations` | `GET /v1/organization-recommendations` | `readonly`, `paginated` | - | - | `ListOrganizationRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List a filterable set of Recommendations within an Organization. This API only supports prioritized recommendations and provides global priority recommendations, eliminating the need to call the API in each AWS Region. |
| `ListRecommendationResources` | `GET /v1/recommendations/{recommendationIdentifier}/resources` | `readonly`, `paginated` | `recommendationIdentifier` | - | `ListRecommendationResourcesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List Resources of a Recommendation. This API provides global recommendations, eliminating the need to call the API in each AWS Region. |
| `ListRecommendations` | `GET /v1/recommendations` | `readonly`, `paginated` | - | - | `ListRecommendationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List a filterable set of Recommendations. This API provides global recommendations, eliminating the need to call the API in each AWS Region. |
| `UpdateOrganizationRecommendationLifecycle` | `PUT /v1/organization-recommendations/{organizationRecommendationIdentifier}/lifecycle` | `idempotent` | `lifecycleStage`, `organizationRecommendationIdentifier` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update the lifecycle of a Recommendation within an Organization. This API only supports prioritized recommendations and updates global priority recommendations, eliminating the need to call the API in each AWS Region. |
| `UpdateRecommendationLifecycle` | `PUT /v1/recommendations/{recommendationIdentifier}/lifecycle` | `idempotent` | `lifecycleStage`, `recommendationIdentifier` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update the lifecyle of a Recommendation. This API only supports prioritized recommendations and updates global priority recommendations, eliminating the need to call the API in each AWS Region. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetRecommendation` | - | `language -> language` | - | - |
| `ListChecks` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `pillar -> pillar`, `awsService -> awsService`, `source -> source`, `language -> language` | - | - |
| `ListOrganizationRecommendationAccounts` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `affectedAccountId -> affectedAccountId` | - | - |
| `ListOrganizationRecommendationResources` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `status -> status`, `exclusionStatus -> exclusionStatus`, `regionCode -> regionCode`, `affectedAccountId -> affectedAccountId` | - | - |
| `ListOrganizationRecommendations` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `type -> type`, `status -> status`, `pillar -> pillar`, `awsService -> awsService`, `source -> source`, `checkIdentifier -> checkIdentifier`, `afterLastUpdatedAt -> afterLastUpdatedAt`, `beforeLastUpdatedAt -> beforeLastUpdatedAt` | - | - |
| `ListRecommendationResources` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `status -> status`, `exclusionStatus -> exclusionStatus`, `regionCode -> regionCode`, `language -> language` | - | - |
| `ListRecommendations` | - | `nextToken -> nextToken`, `maxResults -> maxResults`, `type -> type`, `status -> status`, `pillar -> pillar`, `awsService -> awsService`, `source -> source`, `checkIdentifier -> checkIdentifier`, `afterLastUpdatedAt -> afterLastUpdatedAt`, `beforeLastUpdatedAt -> beforeLastUpdatedAt`, `language -> language` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | Exception that access has been denied due to insufficient access |
| `ConflictException` | `structure` | message | Exception that the request was denied due to conflictions in state |
| `InternalServerException` | `structure` | message | Exception to notify that an unexpected internal error occurred during processing of the request |
| `ResourceNotFoundException` | `structure` | message | Exception that the requested resource has not been found |
| `ThrottlingException` | `structure` | message | Exception to notify that requests are being throttled |
| `ValidationException` | `structure` | message | Exception that the request failed to satisfy service constraints |
| `BatchUpdateRecommendationResourceExclusionRequest` | `structure` | recommendationResourceExclusions | - |
| `BatchUpdateRecommendationResourceExclusionResponse` | `structure` | batchUpdateRecommendationResourceExclusionErrors | - |
| `GetOrganizationRecommendationRequest` | `structure` | organizationRecommendationIdentifier | - |
| `GetOrganizationRecommendationResponse` | `structure` | organizationRecommendation | - |
| `GetRecommendationRequest` | `structure` | recommendationIdentifier, language | - |
| `GetRecommendationResponse` | `structure` | recommendation | - |
| `ListChecksRequest` | `structure` | nextToken, maxResults, pillar, awsService, source, language | - |
| `ListChecksResponse` | `structure` | nextToken, checkSummaries | - |
| `ListOrganizationRecommendationAccountsRequest` | `structure` | nextToken, maxResults, organizationRecommendationIdentifier, affectedAccountId | - |
| `ListOrganizationRecommendationAccountsResponse` | `structure` | nextToken, accountRecommendationLifecycleSummaries | - |
| `ListOrganizationRecommendationResourcesRequest` | `structure` | nextToken, maxResults, status, exclusionStatus, regionCode, organizationRecommendationIdentifier, affectedAccountId | - |
| `ListOrganizationRecommendationResourcesResponse` | `structure` | nextToken, organizationRecommendationResourceSummaries | - |
| `ListOrganizationRecommendationsRequest` | `structure` | nextToken, maxResults, type, status, pillar, awsService, source, checkIdentifier, afterLastUpdatedAt, beforeLastUpdatedAt | - |
| `ListOrganizationRecommendationsResponse` | `structure` | nextToken, organizationRecommendationSummaries | - |
| `ListRecommendationResourcesRequest` | `structure` | nextToken, maxResults, status, exclusionStatus, regionCode, recommendationIdentifier, language | - |
| `ListRecommendationResourcesResponse` | `structure` | nextToken, recommendationResourceSummaries | - |
| `ListRecommendationsRequest` | `structure` | nextToken, maxResults, type, status, pillar, awsService, source, checkIdentifier, afterLastUpdatedAt, beforeLastUpdatedAt, language | - |
| `ListRecommendationsResponse` | `structure` | nextToken, recommendationSummaries | - |
| `UpdateOrganizationRecommendationLifecycleRequest` | `structure` | lifecycleStage, updateReason, updateReasonCode, organizationRecommendationIdentifier | - |
| `UpdateRecommendationLifecycleRequest` | `structure` | lifecycleStage, updateReason, updateReasonCode, recommendationIdentifier | - |
| `ExclusionStatus` | `enum` | EXCLUDED, INCLUDED | - |
| `RecommendationLanguage` | `enum` | ENGLISH, JAPANESE, CHINESE, FRENCH, GERMAN, KOREAN, TRADITIONAL_CHINESE, ITALIAN, SPANISH, BRAZILIAN_PORTUGUESE, BAHASA_INDONESIA | - |
| `RecommendationLifecycleStage` | `enum` | IN_PROGRESS, PENDING_RESPONSE, DISMISSED, RESOLVED | - |
| `RecommendationPillar` | `enum` | COST_OPTIMIZING, PERFORMANCE, SECURITY, SERVICE_LIMITS, FAULT_TOLERANCE, OPERATIONAL_EXCELLENCE | - |
| `RecommendationSource` | `enum` | AWS_CONFIG, COMPUTE_OPTIMIZER, COST_EXPLORER, LSE, MANUAL, PSE, RDS, RESILIENCE, RESILIENCE_HUB, SECURITY_HUB, STIR, TA_CHECK, ... (+2) | - |
| `RecommendationStatus` | `enum` | OK, WARNING, ERROR | - |
| `RecommendationType` | `enum` | STANDARD, PRIORITY | - |
| `ResourceStatus` | `enum` | OK, WARNING, ERROR | - |
| `StatusReason` | `enum` | NO_DATA_OK | - |
| `UpdateRecommendationLifecycleStage` | `enum` | PENDING_RESPONSE, IN_PROGRESS, DISMISSED, RESOLVED | - |
| `UpdateRecommendationLifecycleStageReasonCode` | `enum` | NON_CRITICAL_ACCOUNT, TEMPORARY_ACCOUNT, VALID_BUSINESS_CASE, OTHER_METHODS_AVAILABLE, LOW_PRIORITY, NOT_APPLICABLE, OTHER | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
