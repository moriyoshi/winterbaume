# ARC - Region switch

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Application Recovery Controller (ARC) Region switch helps you to quickly and reliably shift traffic away from an impaired Amazon Web Services Region to a healthy Region. With Region switch, you can create plans that define the steps to shift traffic for your application from one Amazon Web Services Region to another. Region switch provides a structured approach to multi-Region failover, helping you to meet your recovery time objectives (RTOs) and maintain business continuity during regional disruptions. For more information, see Region switch in ARC in the Amazon Application Recovery Controller User Guide .

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for ARC - Region switch resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: define region-switch plans, execution roles, routing controls, and workflows for shifting applications between Regions.
- From the operation surface: model failover execution, plan validation, step state, managed-resource references, and recovery orchestration.

## Service Identity and Protocol

- AWS model slug: `arc-region-switch`
- AWS SDK for Rust slug: `arcregionswitch`
- Model version: `2022-07-26`
- Model file: `vendor/api-models-aws/models/arc-region-switch/service/2022-07-26/arc-region-switch-2022-07-26.json`
- SDK ID: `ARC Region switch`
- Endpoint prefix: `arc-region-switch`
- ARN namespace: `arc-region-switch`
- CloudFormation name: `ARCRegionSwitch`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseControlPlaneEndpoint`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (7), `Get` (4), `Update` (3), `Approve` (1), `Cancel` (1), `Create` (1), `Delete` (1), `Start` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelPlanExecution`, `CreatePlan`, `DeletePlan`, `StartPlanExecution`, `TagResource`, `UntagResource`, `UpdatePlan`, `UpdatePlanExecution`, `UpdatePlanExecutionStep`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetPlan`, `GetPlanEvaluationStatus`, `GetPlanExecution`, `GetPlanInRegion`, `ListPlanExecutionEvents`, `ListPlanExecutions`, `ListPlans`, `ListPlansInRegion`, `ListRoute53HealthChecks`, `ListRoute53HealthChecksInRegion`, `ListTagsForResource`.
- Pagination is modelled for 8 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ApprovePlanExecutionStep`, `CancelPlanExecution`, `GetPlanExecution`, `ListPlanExecutionEvents`, `ListPlanExecutions`, `StartPlanExecution`, `UpdatePlanExecution`, `UpdatePlanExecutionStep`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 19 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `Lambda`, `EC2/VPC`, `ECS`, `EKS`, `RDS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `RegionSwitchPlan` | `arn` | create: `CreatePlan`; read: `GetPlan`; update: `UpdatePlan`; delete: `DeletePlan`; list: `ListPlans` | `ListTagsForResource`, `TagResource`, `UntagResource` | - |
## Operation Groups

### List

- Operations: `ListPlanExecutionEvents`, `ListPlanExecutions`, `ListPlans`, `ListPlansInRegion`, `ListRoute53HealthChecks`, `ListRoute53HealthChecksInRegion`, `ListTagsForResource`
- Traits: `paginated` (6), `readonly` (7)
- Common required input members in this group: `arn`, `executionId`, `planArn`

### Get

- Operations: `GetPlan`, `GetPlanEvaluationStatus`, `GetPlanExecution`, `GetPlanInRegion`
- Traits: `paginated` (2), `readonly` (4)
- Common required input members in this group: `arn`, `executionId`, `planArn`

### Update

- Operations: `UpdatePlan`, `UpdatePlanExecution`, `UpdatePlanExecutionStep`
- Common required input members in this group: `action`, `actionToTake`, `arn`, `comment`, `executionId`, `executionRole`, `planArn`, `stepName`, `workflows`

### Approve

- Operations: `ApprovePlanExecutionStep`
- Common required input members in this group: `approval`, `executionId`, `planArn`, `stepName`

### Cancel

- Operations: `CancelPlanExecution`
- Common required input members in this group: `executionId`, `planArn`

### Create

- Operations: `CreatePlan`
- Common required input members in this group: `executionRole`, `name`, `recoveryApproach`, `regions`, `workflows`

### Delete

- Operations: `DeletePlan`
- Traits: `idempotent` (1)
- Common required input members in this group: `arn`

### Start

- Operations: `StartPlanExecution`
- Common required input members in this group: `action`, `planArn`, `targetRegion`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `arn`, `tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `arn`, `resourceTagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ApprovePlanExecutionStep` | - | - | `approval`, `executionId`, `planArn`, `stepName` | - | `ApprovePlanExecutionStepResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Approves a step in a plan execution that requires manual approval. When you create a plan, you can include approval steps that require manual intervention before the execution can proceed. |
| `CancelPlanExecution` | - | - | `executionId`, `planArn` | - | `CancelPlanExecutionResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Cancels an in-progress plan execution. This operation stops the execution of the plan and prevents any further steps from being processed. |
| `CreatePlan` | - | - | `executionRole`, `name`, `recoveryApproach`, `regions`, `workflows` | - | `CreatePlanResponse` | - | Creates a new Region switch plan. A plan defines the steps required to shift traffic from one Amazon Web Services Region to another. |
| `DeletePlan` | - | `idempotent` | `arn` | - | `DeletePlanResponse` | `IllegalStateException`, `ResourceNotFoundException` | Deletes a Region switch plan. You must specify the ARN of the plan to delete. |
| `GetPlan` | - | `readonly` | `arn` | - | `GetPlanResponse` | `ResourceNotFoundException` | Retrieves detailed information about a Region switch plan. You must specify the ARN of the plan. |
| `GetPlanEvaluationStatus` | - | `readonly`, `paginated` | `planArn` | - | `GetPlanEvaluationStatusResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Retrieves the evaluation status of a Region switch plan. The evaluation status provides information about the last time the plan was evaluated and any warnings or issues detected. |
| `GetPlanExecution` | - | `readonly`, `paginated` | `executionId`, `planArn` | - | `GetPlanExecutionResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Retrieves detailed information about a specific plan execution. You must specify the plan ARN and execution ID. |
| `GetPlanInRegion` | - | `readonly` | `arn` | - | `GetPlanInRegionResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Retrieves information about a Region switch plan in a specific Amazon Web Services Region. This operation is useful for getting Region-specific information about a plan. |
| `ListPlanExecutionEvents` | - | `readonly`, `paginated` | `executionId`, `planArn` | - | `ListPlanExecutionEventsResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Lists the events that occurred during a plan execution. These events provide a detailed timeline of the execution process. |
| `ListPlanExecutions` | - | `readonly`, `paginated` | `planArn` | - | `ListPlanExecutionsResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Lists the executions of a Region switch plan. This operation returns information about both current and historical executions. |
| `ListPlans` | - | `readonly`, `paginated` | - | - | `ListPlansResponse` | - | Lists all Region switch plans in your Amazon Web Services account. |
| `ListPlansInRegion` | - | `readonly`, `paginated` | - | - | `ListPlansInRegionResponse` | `AccessDeniedException` | Lists all Region switch plans in your Amazon Web Services account that are available in the current Amazon Web Services Region. |
| `ListRoute53HealthChecks` | - | `readonly`, `paginated` | `arn` | - | `ListRoute53HealthChecksResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException` | List the Amazon Route 53 health checks. |
| `ListRoute53HealthChecksInRegion` | - | `readonly`, `paginated` | `arn` | - | `ListRoute53HealthChecksInRegionResponse` | `AccessDeniedException`, `IllegalArgumentException`, `InternalServerException`, `ResourceNotFoundException` | List the Amazon Route 53 health checks in a specific Amazon Web Services Region. |
| `ListTagsForResource` | - | `readonly` | `arn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException` | Lists the tags attached to a Region switch resource. |
| `StartPlanExecution` | - | - | `action`, `planArn`, `targetRegion` | - | `StartPlanExecutionResponse` | `AccessDeniedException`, `IllegalArgumentException`, `IllegalStateException`, `ResourceNotFoundException` | Starts the execution of a Region switch plan. You can execute a plan in either `graceful` or `ungraceful` mode. |
| `TagResource` | - | - | `arn`, `tags` | - | `TagResourceResponse` | `InternalServerException`, `ResourceNotFoundException` | Adds or updates tags for a Region switch resource. You can assign metadata to your resources in the form of tags, which are key-value pairs. |
| `UntagResource` | - | - | `arn`, `resourceTagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException` | Removes tags from a Region switch resource. |
| `UpdatePlan` | - | - | `arn`, `executionRole`, `workflows` | - | `UpdatePlanResponse` | `ResourceNotFoundException` | Updates an existing Region switch plan. You can modify the plan's description, workflows, execution role, recovery time objective, associated alarms, and triggers. |
| `UpdatePlanExecution` | - | - | `action`, `executionId`, `planArn` | - | `UpdatePlanExecutionResponse` | `AccessDeniedException`, `IllegalStateException`, `ResourceNotFoundException` | Updates an in-progress plan execution. This operation allows you to modify certain aspects of the execution, such as adding a comment or changing the action. |
| `UpdatePlanExecutionStep` | - | - | `actionToTake`, `comment`, `executionId`, `planArn`, `stepName` | - | `UpdatePlanExecutionStepResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Updates a specific step in an in-progress plan execution. This operation allows you to modify the step's comment or action. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ResourceNotFoundException` | `structure` | `message` | The specified resource was not found. |
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message` | The request processing has failed because of an unknown error, exception, or failure. |
| `IllegalStateException` | `structure` | `message` | The operation failed because the current state of the resource doesn't allow the operation to proceed. |
| `IllegalArgumentException` | `structure` | `message` | The request processing has an invalid argument. |
| `ApprovePlanExecutionStepRequest` | `structure` | `approval`, `comment`, `executionId`, `planArn`, `stepName` | - |
| `ApprovePlanExecutionStepResponse` | `structure` | - | - |
| `CancelPlanExecutionRequest` | `structure` | `comment`, `executionId`, `planArn` | - |
| `CancelPlanExecutionResponse` | `structure` | - | - |
| `CreatePlanRequest` | `structure` | `associatedAlarms`, `description`, `executionRole`, `name`, `primaryRegion`, `recoveryApproach`, `recoveryTimeObjectiveMinutes`, `regions`, `reportConfiguration`, `tags`, `triggers`, `workflows` | - |
| `CreatePlanResponse` | `structure` | `plan` | - |
| `DeletePlanRequest` | `structure` | `arn` | - |
| `DeletePlanResponse` | `structure` | - | - |
| `GetPlanRequest` | `structure` | `arn` | - |
| `GetPlanResponse` | `structure` | `plan` | - |
| `GetPlanEvaluationStatusRequest` | `structure` | `maxResults`, `nextToken`, `planArn` | - |
| `GetPlanEvaluationStatusResponse` | `structure` | `evaluationState`, `lastEvaluatedVersion`, `lastEvaluationTime`, `nextToken`, `planArn`, `region`, `warnings` | - |
| `GetPlanExecutionRequest` | `structure` | `executionId`, `maxResults`, `nextToken`, `planArn` | - |
| `GetPlanExecutionResponse` | `structure` | `actualRecoveryTime`, `comment`, `endTime`, `executionAction`, `executionId`, `executionRegion`, `executionState`, `generatedReportDetails`, `mode`, `nextToken`, `plan`, `planArn`, ... (+5) | - |
| `GetPlanInRegionRequest` | `structure` | `arn` | - |
| `GetPlanInRegionResponse` | `structure` | `plan` | - |
| `ListPlanExecutionEventsRequest` | `structure` | `executionId`, `maxResults`, `name`, `nextToken`, `planArn` | - |
| `ListPlanExecutionEventsResponse` | `structure` | `items`, `nextToken` | - |
| `ListPlanExecutionsRequest` | `structure` | `maxResults`, `nextToken`, `planArn`, `state` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
