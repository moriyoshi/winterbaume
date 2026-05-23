# AWS Billing and Cost Management Pricing Calculator

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

You can use the Pricing Calculator API to programmatically create estimates for your planned cloud use. You can model usage and commitments such as Savings Plans and Reserved Instances, and generate estimated costs using your discounts and benefit sharing preferences. The Pricing Calculator API provides the following endpoint: `https://bcm-pricing-calculator.us-east-1.api.aws`

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for AWS Billing and Cost Management Pricing Calculator by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: create bill scenarios, workload estimates, bill estimates, and usage or commitment modifications for cost modelling.
- From the operation surface: support what-if billing analysis, usage-line mutation, commitment comparison, preferences, and paginated estimate detail retrieval.

## Service Identity and Protocol

- AWS model slug: `bcm-pricing-calculator`
- AWS SDK for Rust slug: `bcmpricingcalculator`
- Model version: `2024-06-19`
- Model file: `vendor/api-models-aws/models/bcm-pricing-calculator/service/2024-06-19/bcm-pricing-calculator-2024-06-19.json`
- SDK ID: `BCM Pricing Calculator`
- Endpoint prefix: `-`
- ARN namespace: `bcm-pricing-calculator`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (11), `Batch` (9), `Get` (4), `Update` (4), `Create` (3), `Delete` (3), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchCreateBillScenarioCommitmentModification`, `BatchCreateBillScenarioUsageModification`, `BatchCreateWorkloadEstimateUsage`, `BatchDeleteBillScenarioCommitmentModification`, `BatchDeleteBillScenarioUsageModification`, `BatchDeleteWorkloadEstimateUsage`, `BatchUpdateBillScenarioCommitmentModification`, `BatchUpdateBillScenarioUsageModification`, `BatchUpdateWorkloadEstimateUsage`, `CreateBillEstimate`, `CreateBillScenario`, `CreateWorkloadEstimate`, `DeleteBillEstimate`, `DeleteBillScenario`, `DeleteWorkloadEstimate`, `TagResource`, `UntagResource`, `UpdateBillEstimate`, `UpdateBillScenario`, `UpdatePreferences`, ... (+1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetBillEstimate`, `GetBillScenario`, `GetPreferences`, `GetWorkloadEstimate`, `ListBillEstimateCommitments`, `ListBillEstimateInputCommitmentModifications`, `ListBillEstimateInputUsageModifications`, `ListBillEstimateLineItems`, `ListBillEstimates`, `ListBillScenarioCommitmentModifications`, `ListBillScenarioUsageModifications`, `ListBillScenarios`, `ListTagsForResource`, `ListWorkloadEstimateUsage`, `ListWorkloadEstimates`.
- Pagination is modelled for 10 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 19 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 36 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `BillEstimate` | `billEstimateId` | create: `CreateBillEstimate`; read: `GetBillEstimate`; update: `UpdateBillEstimate`; delete: `DeleteBillEstimate`; list: `ListBillEstimates` | - | Represents a bill estimate that is created from a bill scenario |
| `BillEstimateCommitment` | `billEstimateId`, `id`, `resultCommitmentId` | list: `ListBillEstimateCommitments` | - | - |
| `BillEstimateInputCommitmentModification` | `billEstimateId`, `id` | list: `ListBillEstimateInputCommitmentModifications` | - | - |
| `BillEstimateInputUsageModification` | `billEstimateId`, `id` | list: `ListBillEstimateInputUsageModifications` | - | - |
| `BillEstimateLineItem` | `billEstimateId`, `id` | list: `ListBillEstimateLineItems` | - | - |
| `BillScenario` | `billScenarioId` | create: `CreateBillScenario`; read: `GetBillScenario`; update: `UpdateBillScenario`; delete: `DeleteBillScenario`; list: `ListBillScenarios` | - | Represents a bill scenario that allows usage and commitment modeling |
| `BillScenarioCommitmentModification` | `billScenarioId`, `id` | list: `ListBillScenarioCommitmentModifications` | `BatchCreateBillScenarioCommitmentModification`, `BatchDeleteBillScenarioCommitmentModification`, `BatchUpdateBillScenarioCommitmentModification` | - |
| `BillScenarioUsageModification` | `billScenarioId`, `id` | list: `ListBillScenarioUsageModifications` | `BatchCreateBillScenarioUsageModification`, `BatchDeleteBillScenarioUsageModification`, `BatchUpdateBillScenarioUsageModification` | - |
| `WorkloadEstimate` | `workloadEstimateId` | create: `CreateWorkloadEstimate`; read: `GetWorkloadEstimate`; update: `UpdateWorkloadEstimate`; delete: `DeleteWorkloadEstimate`; list: `ListWorkloadEstimates` | - | Represents a workload estimate that allows usage modeling |
| `WorkloadEstimateUsage` | `id`, `workloadEstimateId` | list: `ListWorkloadEstimateUsage` | `BatchCreateWorkloadEstimateUsage`, `BatchDeleteWorkloadEstimateUsage`, `BatchUpdateWorkloadEstimateUsage` | - |
## Operation Groups

### Get

- Operations: `GetPreferences`
- Traits: `readonly` (1)
- Common required input members in this group: -

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Update

- Operations: `UpdatePreferences`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetPreferences` | `-` | `readonly` | - | - | `GetPreferencesResponse` | `DataUnavailableException` | Retrieves the current preferences for Pricing Calculator. |
| `ListTagsForResource` | `-` | `readonly` | `arn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | Lists all tags associated with a specified resource. |
| `TagResource` | `-` | - | `arn`, `tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `ServiceQuotaExceededException` | Adds one or more tags to a specified resource. |
| `UntagResource` | `-` | - | `arn`, `tagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException` | Removes one or more tags from a specified resource. |
| `UpdatePreferences` | `-` | `idempotent` | - | - | `UpdatePreferencesResponse` | `DataUnavailableException`, `ServiceQuotaExceededException` | Updates the preferences for Pricing Calculator. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message, resourceId, resourceType | The request could not be processed because of conflict in the current state of the resource. |
| `DataUnavailableException` | `structure` | message | The requested data is currently unavailable. |
| `InternalServerException` | `structure` | message, retryAfterSeconds | An internal error has occurred. Retry your request, but if the problem persists, contact Amazon Web Services support. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | The specified resource was not found. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, serviceCode, quotaCode | The request would cause you to exceed your service quota. |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message, reason, fieldList | The input provided fails to satisfy the constraints specified by an Amazon Web Services service. |
| `GetPreferencesRequest` | `structure` | **empty (no members)** | - |
| `GetPreferencesResponse` | `structure` | managementAccountRateTypeSelections, memberAccountRateTypeSelections, standaloneAccountRateTypeSelections | - |
| `ListTagsForResourceRequest` | `structure` | arn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | arn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | arn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `UpdatePreferencesRequest` | `structure` | managementAccountRateTypeSelections, memberAccountRateTypeSelections, standaloneAccountRateTypeSelections | - |
| `UpdatePreferencesResponse` | `structure` | managementAccountRateTypeSelections, memberAccountRateTypeSelections, standaloneAccountRateTypeSelections | - |
| `BatchCreateBillScenarioCommitmentModificationErrorCode` | `enum` | CONFLICT, INTERNAL_SERVER_ERROR, INVALID_ACCOUNT | - |
| `BatchCreateBillScenarioUsageModificationErrorCode` | `enum` | BAD_REQUEST, NOT_FOUND, CONFLICT, INTERNAL_SERVER_ERROR | - |
| `BatchCreateWorkloadEstimateUsageCode` | `enum` | BAD_REQUEST, NOT_FOUND, CONFLICT, INTERNAL_SERVER_ERROR | - |
| `BatchDeleteBillScenarioCommitmentModificationErrorCode` | `enum` | BAD_REQUEST, CONFLICT, INTERNAL_SERVER_ERROR | - |
| `BatchDeleteBillScenarioUsageModificationErrorCode` | `enum` | BAD_REQUEST, CONFLICT, INTERNAL_SERVER_ERROR | - |
| `BatchUpdateBillScenarioCommitmentModificationErrorCode` | `enum` | BAD_REQUEST, NOT_FOUND, CONFLICT, INTERNAL_SERVER_ERROR | - |
| `BatchUpdateBillScenarioUsageModificationErrorCode` | `enum` | BAD_REQUEST, NOT_FOUND, CONFLICT, INTERNAL_SERVER_ERROR | - |
| `BillEstimateStatus` | `enum` | IN_PROGRESS, COMPLETE, FAILED | - |
| `BillScenarioStatus` | `enum` | READY, LOCKED, FAILED, STALE | - |
| `CurrencyCode` | `enum` | USD | - |
| `GroupSharingPreferenceEnum` | `enum` | OPEN, PRIORITIZED, RESTRICTED | - |
| `ListBillEstimateLineItemsFilterName` | `enum` | USAGE_ACCOUNT_ID, SERVICE_CODE, USAGE_TYPE, OPERATION, LOCATION, LINE_ITEM_TYPE | - |
| `ListBillEstimatesFilterName` | `enum` | STATUS, NAME | - |
| `ListBillScenariosFilterName` | `enum` | STATUS, NAME, GROUP_SHARING_PREFERENCE, COST_CATEGORY_ARN | - |
| `ListUsageFilterName` | `enum` | USAGE_ACCOUNT_ID, SERVICE_CODE, USAGE_TYPE, OPERATION, LOCATION, USAGE_GROUP, HISTORICAL_USAGE_ACCOUNT_ID, HISTORICAL_SERVICE_CODE, HISTORICAL_USAGE_TYPE, HISTORICAL_OPERATION, HISTORICAL_LOCATION | - |
| `ListWorkloadEstimatesFilterName` | `enum` | STATUS, NAME | - |
| `MatchOption` | `enum` | EQUALS, STARTS_WITH, CONTAINS | - |
| `PurchaseAgreementType` | `enum` | SAVINGS_PLANS, RESERVED_INSTANCE | - |
| `RateType` | `enum` | BEFORE_DISCOUNTS, AFTER_DISCOUNTS, AFTER_DISCOUNTS_AND_COMMITMENTS | - |
| `ValidationExceptionReason` | `enum` | UNKNOWN_OPERATION, CANNOT_PARSE, FIELD_VALIDATION_FAILED, INVALID_REQUEST_FROM_MEMBER, DISALLOWED_RATE, OTHER | - |
| `WorkloadEstimateCostStatus` | `enum` | VALID, INVALID, STALE | - |
| `WorkloadEstimateRateType` | `enum` | BEFORE_DISCOUNTS, AFTER_DISCOUNTS, AFTER_DISCOUNTS_AND_COMMITMENTS | - |
| `WorkloadEstimateStatus` | `enum` | UPDATING, VALID, INVALID, ACTION_NEEDED | - |
| `WorkloadEstimateUpdateUsageErrorCode` | `enum` | BAD_REQUEST, NOT_FOUND, CONFLICT, INTERNAL_SERVER_ERROR | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
