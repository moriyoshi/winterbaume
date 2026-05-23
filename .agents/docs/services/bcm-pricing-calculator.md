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

### List

- Operations: `ListBillEstimateCommitments`, `ListBillEstimateInputCommitmentModifications`, `ListBillEstimateInputUsageModifications`, `ListBillEstimateLineItems`, `ListBillEstimates`, `ListBillScenarioCommitmentModifications`, `ListBillScenarioUsageModifications`, `ListBillScenarios`, `ListTagsForResource`, `ListWorkloadEstimateUsage`, `ListWorkloadEstimates`
- Traits: `paginated` (10), `readonly` (11)
- Common required input members in this group: `arn`, `billEstimateId`, `billScenarioId`, `workloadEstimateId`

### Batch

- Operations: `BatchCreateBillScenarioCommitmentModification`, `BatchCreateBillScenarioUsageModification`, `BatchCreateWorkloadEstimateUsage`, `BatchDeleteBillScenarioCommitmentModification`, `BatchDeleteBillScenarioUsageModification`, `BatchDeleteWorkloadEstimateUsage`, `BatchUpdateBillScenarioCommitmentModification`, `BatchUpdateBillScenarioUsageModification`, `BatchUpdateWorkloadEstimateUsage`
- Traits: `idempotency-token` (3), `idempotent` (9)
- Common required input members in this group: `billScenarioId`, `commitmentModifications`, `ids`, `usage`, `usageModifications`, `workloadEstimateId`

### Get

- Operations: `GetBillEstimate`, `GetBillScenario`, `GetPreferences`, `GetWorkloadEstimate`
- Traits: `readonly` (4)
- Common required input members in this group: `identifier`

### Update

- Operations: `UpdateBillEstimate`, `UpdateBillScenario`, `UpdatePreferences`, `UpdateWorkloadEstimate`
- Traits: `idempotent` (4)
- Common required input members in this group: `identifier`

### Create

- Operations: `CreateBillEstimate`, `CreateBillScenario`, `CreateWorkloadEstimate`
- Traits: `idempotency-token` (3), `idempotent` (3)
- Common required input members in this group: `billScenarioId`, `name`

### Delete

- Operations: `DeleteBillEstimate`, `DeleteBillScenario`, `DeleteWorkloadEstimate`
- Traits: `idempotent` (3)
- Common required input members in this group: `identifier`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `arn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `arn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchCreateBillScenarioCommitmentModification` | - | `idempotent`, `idempotency-token` | `billScenarioId`, `commitmentModifications` | `clientToken` | `BatchCreateBillScenarioCommitmentModificationResponse` | `ConflictException`, `DataUnavailableException`, `ResourceNotFoundException` | Create Compute Savings Plans, EC2 Instance Savings Plans, or EC2 Reserved Instances commitments that you want to model in a Bill Scenario. The `BatchCreateBillScenarioCommitmentModification` operation doesn't have its own IAM permission. |
| `BatchCreateBillScenarioUsageModification` | - | `idempotent`, `idempotency-token` | `billScenarioId`, `usageModifications` | `clientToken` | `BatchCreateBillScenarioUsageModificationResponse` | `ConflictException`, `DataUnavailableException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | Create Amazon Web Services service usage that you want to model in a Bill Scenario. The `BatchCreateBillScenarioUsageModification` operation doesn't have its own IAM permission. |
| `BatchCreateWorkloadEstimateUsage` | - | `idempotent`, `idempotency-token` | `usage`, `workloadEstimateId` | `clientToken` | `BatchCreateWorkloadEstimateUsageResponse` | `ConflictException`, `DataUnavailableException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | Create Amazon Web Services service usage that you want to model in a Workload Estimate. The `BatchCreateWorkloadEstimateUsage` operation doesn't have its own IAM permission. |
| `BatchDeleteBillScenarioCommitmentModification` | - | `idempotent` | `billScenarioId`, `ids` | - | `BatchDeleteBillScenarioCommitmentModificationResponse` | `ConflictException`, `DataUnavailableException`, `ResourceNotFoundException` | Delete commitment that you have created in a Bill Scenario. You can only delete a commitment that you had added and cannot model deletion (or removal) of a existing commitment. |
| `BatchDeleteBillScenarioUsageModification` | - | `idempotent` | `billScenarioId`, `ids` | - | `BatchDeleteBillScenarioUsageModificationResponse` | `ConflictException`, `DataUnavailableException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | Delete usage that you have created in a Bill Scenario. You can only delete usage that you had added and cannot model deletion (or removal) of a existing usage. |
| `BatchDeleteWorkloadEstimateUsage` | - | `idempotent` | `ids`, `workloadEstimateId` | - | `BatchDeleteWorkloadEstimateUsageResponse` | `DataUnavailableException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | Delete usage that you have created in a Workload estimate. You can only delete usage that you had added and cannot model deletion (or removal) of a existing usage. |
| `BatchUpdateBillScenarioCommitmentModification` | - | `idempotent` | `billScenarioId`, `commitmentModifications` | - | `BatchUpdateBillScenarioCommitmentModificationResponse` | `ConflictException`, `DataUnavailableException`, `ResourceNotFoundException` | Update a newly added or existing commitment. You can update the commitment group based on a commitment ID and a Bill scenario ID. |
| `BatchUpdateBillScenarioUsageModification` | - | `idempotent` | `billScenarioId`, `usageModifications` | - | `BatchUpdateBillScenarioUsageModificationResponse` | `ConflictException`, `DataUnavailableException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | Update a newly added or existing usage lines. You can update the usage amounts, usage hour, and usage group based on a usage ID and a Bill scenario ID. |
| `BatchUpdateWorkloadEstimateUsage` | - | `idempotent` | `usage`, `workloadEstimateId` | - | `BatchUpdateWorkloadEstimateUsageResponse` | `DataUnavailableException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | Update a newly added or existing usage lines. You can update the usage amounts and usage group based on a usage ID and a Workload estimate ID. |
| `CreateBillEstimate` | - | `idempotent`, `idempotency-token` | `billScenarioId`, `name` | `clientToken` | `CreateBillEstimateResponse` | `ConflictException`, `DataUnavailableException`, `ResourceNotFoundException` | Create a Bill estimate from a Bill scenario. In the Bill scenario you can model usage addition, usage changes, and usage removal. |
| `CreateBillScenario` | - | `idempotent`, `idempotency-token` | `name` | `clientToken` | `CreateBillScenarioResponse` | `ConflictException`, `DataUnavailableException`, `ServiceQuotaExceededException` | Creates a new bill scenario to model potential changes to Amazon Web Services usage and costs. |
| `CreateWorkloadEstimate` | - | `idempotent`, `idempotency-token` | `name` | `clientToken` | `CreateWorkloadEstimateResponse` | `ConflictException`, `DataUnavailableException`, `ServiceQuotaExceededException` | Creates a new workload estimate to model costs for a specific workload. |
| `DeleteBillEstimate` | - | `idempotent` | `identifier` | - | `DeleteBillEstimateResponse` | `ConflictException`, `DataUnavailableException` | Deletes an existing bill estimate. |
| `DeleteBillScenario` | - | `idempotent` | `identifier` | - | `DeleteBillScenarioResponse` | `ConflictException`, `DataUnavailableException` | Deletes an existing bill scenario. |
| `DeleteWorkloadEstimate` | - | `idempotent` | `identifier` | - | `DeleteWorkloadEstimateResponse` | `DataUnavailableException` | Deletes an existing workload estimate. |
| `GetBillEstimate` | - | `readonly` | `identifier` | - | `GetBillEstimateResponse` | `DataUnavailableException`, `ResourceNotFoundException` | Retrieves details of a specific bill estimate. |
| `GetBillScenario` | - | `readonly` | `identifier` | - | `GetBillScenarioResponse` | `DataUnavailableException`, `ResourceNotFoundException` | Retrieves details of a specific bill scenario. |
| `GetPreferences` | - | `readonly` | - | - | `GetPreferencesResponse` | `DataUnavailableException` | Retrieves the current preferences for Pricing Calculator. |
| `GetWorkloadEstimate` | - | `readonly` | `identifier` | - | `GetWorkloadEstimateResponse` | `DataUnavailableException`, `ResourceNotFoundException` | Retrieves details of a specific workload estimate. |
| `ListBillEstimateCommitments` | - | `readonly`, `paginated` | `billEstimateId` | - | `ListBillEstimateCommitmentsResponse` | `DataUnavailableException`, `ResourceNotFoundException` | Lists the commitments associated with a bill estimate. |
| `ListBillEstimateInputCommitmentModifications` | - | `readonly`, `paginated` | `billEstimateId` | - | `ListBillEstimateInputCommitmentModificationsResponse` | `DataUnavailableException`, `ResourceNotFoundException` | Lists the input commitment modifications associated with a bill estimate. |
| `ListBillEstimateInputUsageModifications` | - | `readonly`, `paginated` | `billEstimateId` | - | `ListBillEstimateInputUsageModificationsResponse` | `DataUnavailableException`, `ResourceNotFoundException` | Lists the input usage modifications associated with a bill estimate. |
| `ListBillEstimateLineItems` | - | `readonly`, `paginated` | `billEstimateId` | - | `ListBillEstimateLineItemsResponse` | `DataUnavailableException`, `ResourceNotFoundException` | Lists the line items associated with a bill estimate. |
| `ListBillEstimates` | - | `readonly`, `paginated` | - | - | `ListBillEstimatesResponse` | `DataUnavailableException` | Lists all bill estimates for the account. |
| `ListBillScenarioCommitmentModifications` | - | `readonly`, `paginated` | `billScenarioId` | - | `ListBillScenarioCommitmentModificationsResponse` | `DataUnavailableException`, `ResourceNotFoundException` | Lists the commitment modifications associated with a bill scenario. |
| `ListBillScenarioUsageModifications` | - | `readonly`, `paginated` | `billScenarioId` | - | `ListBillScenarioUsageModificationsResponse` | `DataUnavailableException`, `ResourceNotFoundException` | Lists the usage modifications associated with a bill scenario. |
| `ListBillScenarios` | - | `readonly`, `paginated` | - | - | `ListBillScenariosResponse` | `DataUnavailableException` | Lists all bill scenarios for the account. |
| `ListTagsForResource` | - | `readonly` | `arn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | Lists all tags associated with a specified resource. |
| `ListWorkloadEstimateUsage` | - | `readonly`, `paginated` | `workloadEstimateId` | - | `ListWorkloadEstimateUsageResponse` | `DataUnavailableException`, `ResourceNotFoundException` | Lists the usage associated with a workload estimate. |
| `ListWorkloadEstimates` | - | `readonly`, `paginated` | - | - | `ListWorkloadEstimatesResponse` | `DataUnavailableException` | Lists all workload estimates for the account. |
| `TagResource` | - | - | `arn`, `tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `ServiceQuotaExceededException` | Adds one or more tags to a specified resource. |
| `UntagResource` | - | - | `arn`, `tagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException` | Removes one or more tags from a specified resource. |
| `UpdateBillEstimate` | - | `idempotent` | `identifier` | - | `UpdateBillEstimateResponse` | `ConflictException`, `DataUnavailableException`, `ResourceNotFoundException` | Updates an existing bill estimate. |
| `UpdateBillScenario` | - | `idempotent` | `identifier` | - | `UpdateBillScenarioResponse` | `ConflictException`, `DataUnavailableException`, `ResourceNotFoundException` | Updates an existing bill scenario. |
| `UpdatePreferences` | - | `idempotent` | - | - | `UpdatePreferencesResponse` | `DataUnavailableException`, `ServiceQuotaExceededException` | Updates the preferences for Pricing Calculator. |
| `UpdateWorkloadEstimate` | - | `idempotent` | `identifier` | - | `UpdateWorkloadEstimateResponse` | `ConflictException`, `DataUnavailableException`, `ResourceNotFoundException` | Updates an existing workload estimate. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `DataUnavailableException` | `structure` | `message` | The requested data is currently unavailable. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The specified resource was not found. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | The request could not be processed because of conflict in the current state of the resource. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | The request would cause you to exceed your service quota. |
| `BatchCreateBillScenarioCommitmentModificationRequest` | `structure` | `billScenarioId`, `clientToken`, `commitmentModifications` | - |
| `BatchCreateBillScenarioCommitmentModificationResponse` | `structure` | `errors`, `items` | - |
| `BatchCreateBillScenarioUsageModificationRequest` | `structure` | `billScenarioId`, `clientToken`, `usageModifications` | - |
| `BatchCreateBillScenarioUsageModificationResponse` | `structure` | `errors`, `items` | - |
| `BatchCreateWorkloadEstimateUsageRequest` | `structure` | `clientToken`, `usage`, `workloadEstimateId` | - |
| `BatchCreateWorkloadEstimateUsageResponse` | `structure` | `errors`, `items` | - |
| `BatchDeleteBillScenarioCommitmentModificationRequest` | `structure` | `billScenarioId`, `ids` | - |
| `BatchDeleteBillScenarioCommitmentModificationResponse` | `structure` | `errors` | - |
| `BatchDeleteBillScenarioUsageModificationRequest` | `structure` | `billScenarioId`, `ids` | - |
| `BatchDeleteBillScenarioUsageModificationResponse` | `structure` | `errors` | - |
| `BatchDeleteWorkloadEstimateUsageRequest` | `structure` | `ids`, `workloadEstimateId` | - |
| `BatchDeleteWorkloadEstimateUsageResponse` | `structure` | `errors` | - |
| `BatchUpdateBillScenarioCommitmentModificationRequest` | `structure` | `billScenarioId`, `commitmentModifications` | - |
| `BatchUpdateBillScenarioCommitmentModificationResponse` | `structure` | `errors`, `items` | - |
| `BatchUpdateBillScenarioUsageModificationRequest` | `structure` | `billScenarioId`, `usageModifications` | - |
| `BatchUpdateBillScenarioUsageModificationResponse` | `structure` | `errors`, `items` | - |
| `BatchUpdateWorkloadEstimateUsageRequest` | `structure` | `usage`, `workloadEstimateId` | - |
| `BatchUpdateWorkloadEstimateUsageResponse` | `structure` | `errors`, `items` | - |
| `CreateBillEstimateRequest` | `structure` | `billScenarioId`, `clientToken`, `name`, `tags` | - |
| `CreateBillEstimateResponse` | `structure` | `billInterval`, `costCategoryGroupSharingPreferenceArn`, `costCategoryGroupSharingPreferenceEffectiveDate`, `costSummary`, `createdAt`, `expiresAt`, `failureMessage`, `groupSharingPreference`, `id`, `name`, `status` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
