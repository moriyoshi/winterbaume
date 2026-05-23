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

- Operations: `ListPlanExecutionEvents`, `ListPlanExecutions`, `ListPlansInRegion`, `ListRoute53HealthChecks`, `ListRoute53HealthChecksInRegion`
- Traits: `readonly` (5), `paginated` (5)
- Common required input members in this group: `planArn`, `arn`

### Get

- Operations: `GetPlanEvaluationStatus`, `GetPlanExecution`, `GetPlanInRegion`
- Traits: `readonly` (3), `paginated` (2)
- Common required input members in this group: `planArn`

### Update

- Operations: `UpdatePlanExecution`, `UpdatePlanExecutionStep`
- Common required input members in this group: `planArn`, `executionId`

### Approve

- Operations: `ApprovePlanExecutionStep`
- Common required input members in this group: -

### Cancel

- Operations: `CancelPlanExecution`
- Common required input members in this group: -

### Start

- Operations: `StartPlanExecution`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ApprovePlanExecutionStep` | `-` | - | `planArn`, `executionId`, `stepName`, `approval` | - | `ApprovePlanExecutionStepResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Approves a step in a plan execution that requires manual approval. When you create a plan, you can include approval steps that require manual intervention before the execution can proceed. This operation allows you t ... |
| `CancelPlanExecution` | `-` | - | `planArn`, `executionId` | - | `CancelPlanExecutionResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Cancels an in-progress plan execution. This operation stops the execution of the plan and prevents any further steps from being processed. You must specify the plan ARN and execution ID. You can also provide an optio ... |
| `GetPlanEvaluationStatus` | `-` | `readonly`, `paginated` | `planArn` | - | `GetPlanEvaluationStatusResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Retrieves the evaluation status of a Region switch plan. The evaluation status provides information about the last time the plan was evaluated and any warnings or issues detected. |
| `GetPlanExecution` | `-` | `readonly`, `paginated` | `planArn`, `executionId` | - | `GetPlanExecutionResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Retrieves detailed information about a specific plan execution. You must specify the plan ARN and execution ID. |
| `GetPlanInRegion` | `-` | `readonly` | `arn` | - | `GetPlanInRegionResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Retrieves information about a Region switch plan in a specific Amazon Web Services Region. This operation is useful for getting Region-specific information about a plan. |
| `ListPlanExecutionEvents` | `-` | `readonly`, `paginated` | `planArn`, `executionId` | - | `ListPlanExecutionEventsResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Lists the events that occurred during a plan execution. These events provide a detailed timeline of the execution process. |
| `ListPlanExecutions` | `-` | `readonly`, `paginated` | `planArn` | - | `ListPlanExecutionsResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Lists the executions of a Region switch plan. This operation returns information about both current and historical executions. |
| `ListPlansInRegion` | `-` | `readonly`, `paginated` | - | - | `ListPlansInRegionResponse` | `AccessDeniedException` | Lists all Region switch plans in your Amazon Web Services account that are available in the current Amazon Web Services Region. |
| `ListRoute53HealthChecks` | `-` | `readonly`, `paginated` | `arn` | - | `ListRoute53HealthChecksResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException` | List the Amazon Route 53 health checks. |
| `ListRoute53HealthChecksInRegion` | `-` | `readonly`, `paginated` | `arn` | - | `ListRoute53HealthChecksInRegionResponse` | `AccessDeniedException`, `IllegalArgumentException`, `InternalServerException`, `ResourceNotFoundException` | List the Amazon Route 53 health checks in a specific Amazon Web Services Region. |
| `StartPlanExecution` | `-` | - | `planArn`, `targetRegion`, `action` | - | `StartPlanExecutionResponse` | `AccessDeniedException`, `IllegalArgumentException`, `IllegalStateException`, `ResourceNotFoundException` | Starts the execution of a Region switch plan. You can execute a plan in either graceful or ungraceful mode. Specifing ungraceful mode either changes the behavior of the execution blocks in a workflow or skips specifi ... |
| `UpdatePlanExecution` | `-` | - | `planArn`, `executionId`, `action` | - | `UpdatePlanExecutionResponse` | `AccessDeniedException`, `IllegalStateException`, `ResourceNotFoundException` | Updates an in-progress plan execution. This operation allows you to modify certain aspects of the execution, such as adding a comment or changing the action. |
| `UpdatePlanExecutionStep` | `-` | - | `planArn`, `executionId`, `comment`, `stepName`, `actionToTake` | - | `UpdatePlanExecutionStepResponse` | `AccessDeniedException`, `ResourceNotFoundException` | Updates a specific step in an in-progress plan execution. This operation allows you to modify the step's comment or action. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. HTTP Status Code: 403 |
| `IllegalArgumentException` | `structure` | message | The request processing has an invalid argument. |
| `IllegalStateException` | `structure` | message | The operation failed because the current state of the resource doesn't allow the operation to proceed. HTTP Status Code: 400 |
| `InternalServerException` | `structure` | message | The request processing has failed because of an unknown error, exception, or failure. HTTP Status Code: 500 |
| `ResourceNotFoundException` | `structure` | message | The specified resource was not found. HTTP Status Code: 404 |
| `ApprovePlanExecutionStepRequest` | `structure` | planArn, executionId, stepName, approval, comment | - |
| `ApprovePlanExecutionStepResponse` | `structure` | **empty (no members)** | - |
| `CancelPlanExecutionRequest` | `structure` | planArn, executionId, comment | - |
| `CancelPlanExecutionResponse` | `structure` | **empty (no members)** | - |
| `GetPlanEvaluationStatusRequest` | `structure` | planArn, maxResults, nextToken | - |
| `GetPlanEvaluationStatusResponse` | `structure` | planArn, lastEvaluationTime, lastEvaluatedVersion, region, evaluationState, warnings, nextToken | - |
| `GetPlanExecutionRequest` | `structure` | planArn, executionId, maxResults, nextToken | - |
| `GetPlanExecutionResponse` | `structure` | planArn, executionId, version, updatedAt, comment, startTime, endTime, mode, executionState, executionAction, executionRegion, recoveryExecutionId, ... (+5) | - |
| `GetPlanInRegionRequest` | `structure` | arn | - |
| `GetPlanInRegionResponse` | `structure` | plan | - |
| `ListPlanExecutionEventsRequest` | `structure` | planArn, executionId, maxResults, nextToken, name | - |
| `ListPlanExecutionEventsResponse` | `structure` | items, nextToken | - |
| `ListPlanExecutionsRequest` | `structure` | planArn, maxResults, nextToken, state | - |
| `ListPlanExecutionsResponse` | `structure` | items, nextToken | - |
| `ListPlansInRegionRequest` | `structure` | maxResults, nextToken | - |
| `ListPlansInRegionResponse` | `structure` | plans, nextToken | - |
| `ListRoute53HealthChecksRequest` | `structure` | arn, hostedZoneId, recordName, maxResults, nextToken | - |
| `ListRoute53HealthChecksResponse` | `structure` | healthChecks, nextToken | - |
| `ListRoute53HealthChecksInRegionRequest` | `structure` | arn, hostedZoneId, recordName, maxResults, nextToken | - |
| `ListRoute53HealthChecksInRegionResponse` | `structure` | healthChecks, nextToken | - |
| `StartPlanExecutionRequest` | `structure` | planArn, targetRegion, action, mode, comment, latestVersion, recoveryExecutionId | - |
| `StartPlanExecutionResponse` | `structure` | executionId, plan, planVersion, activateRegion, deactivateRegion | - |
| `UpdatePlanExecutionRequest` | `structure` | planArn, executionId, action, comment | - |
| `UpdatePlanExecutionResponse` | `structure` | **empty (no members)** | - |
| `UpdatePlanExecutionStepRequest` | `structure` | planArn, executionId, comment, stepName, actionToTake | - |
| `UpdatePlanExecutionStepResponse` | `structure` | **empty (no members)** | - |
| `AlarmCondition` | `enum` | RED, GREEN | - |
| `AlarmType` | `enum` | APPLICATION_HEALTH, TRIGGER | - |
| `Approval` | `enum` | APPROVE, DECLINE | - |
| `DocumentDbDefaultBehavior` | `enum` | SWITCHOVER_ONLY, FAILOVER | - |
| `DocumentDbUngracefulBehavior` | `enum` | FAILOVER | - |
| `Ec2AsgCapacityMonitoringApproach` | `enum` | SAMPLED_MAX_IN_LAST_24_HOURS, AUTOSCALING_MAX_IN_LAST_24_HOURS | - |
| `EcsCapacityMonitoringApproach` | `enum` | SAMPLED_MAX_IN_LAST_24_HOURS, CONTAINER_INSIGHTS_MAX_IN_LAST_24_HOURS | - |
| `EksCapacityMonitoringApproach` | `enum` | SAMPLED_MAX_IN_LAST_24_HOURS | - |
| `EvaluationStatus` | `enum` | PASSED, ACTION_REQUIRED, PENDING_EVALUATION, UNKNOWN | - |
| `ExecutionAction` | `enum` | ACTIVATE, DEACTIVATE, POST_RECOVERY | - |
| `ExecutionBlockType` | `enum` | CUSTOM_ACTION_LAMBDA, EXECUTION_APPROVAL, AURORA, EC2_ASG, ROUTING_CONTROL, REGION_SWITCH, PARALLEL, ECS, EKS_RESOURCE_SCALING, ROUTE53_HEALTH_CHECK, DOCUMENTDB, RDS_PROMOTE_READ_REPLICA, ... (+1) | - |
| `ExecutionEventType` | `enum` | UNKNOWN, EXECUTION_PENDING, EXECUTION_STARTED, EXECUTION_SUCCEEDED, EXECUTION_FAILED, EXECUTION_PAUSING, EXECUTION_PAUSED, EXECUTION_CANCELING, EXECUTION_CANCELED, EXECUTION_PENDING_APPROVAL, EXECUTION_BEHAVIOR_CHANGED_TO_UNGRACEFUL, EXECUTION_BEHAVIOR_CHANGED_TO_GRACEFUL, ... (+14) | - |
| `ExecutionMode` | `enum` | GRACEFUL, UNGRACEFUL | - |
| `ExecutionState` | `enum` | IN_PROGRESS, PAUSED_BY_FAILED_STEP, PAUSED_BY_OPERATOR, COMPLETED, COMPLETED_WITH_EXCEPTIONS, CANCELLED, PLAN_EXECUTION_TIMED_OUT, PENDING_MANUAL_APPROVAL, FAILED, PENDING, COMPLETED_MONITORING_APPLICATION_HEALTH | - |
| `FailedReportErrorCode` | `enum` | INSUFFICIENT_PERMISSIONS, INVALID_RESOURCE, CONFIGURATION_ERROR | - |
| `GlobalAuroraDefaultBehavior` | `enum` | SWITCHOVER_ONLY, FAILOVER | - |
| `GlobalAuroraUngracefulBehavior` | `enum` | FAILOVER | - |
| `LambdaUngracefulBehavior` | `enum` | SKIP | - |
| `RecoveryApproach` | `enum` | ACTIVE_ACTIVE, ACTIVE_PASSIVE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
