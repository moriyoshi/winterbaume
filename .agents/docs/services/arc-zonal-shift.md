# AWS ARC - Zonal Shift

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Welcome to the API Reference Guide for zonal shift and zonal autoshift in Amazon Application Recovery Controller (ARC). You can start a zonal shift to move traffic for a load balancer resource away from an Availability Zone to help your application recover quickly from an impairment in an Availability Zone. For example, you can recover your application from a developer's bad code deployment or from an Amazon Web Services infrastructure failure in a single Availability Zone. You can also configure zonal autoshift for supported load balancer resources. Zonal autoshift is a capability in ARC where you authorize Amazon Web Services to shift away application resource traffic from an Availability Zone during events, on your behalf, to help reduce your time to recovery.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-arczonalshift/tests/scenario_test.rs`: start a zonal shift, list and describe it, update the expiry/comment, cancel it, and verify terminal state.
- Backported from `scenario_test.rs`: configure practice runs and enable the zonal autoshift pipeline for a resource.
- From the AWS documentation and model: represent availability-zone evacuation drills, resource registration, autoshift readiness, practice-run scheduling, and recovery-oriented traffic movement.

## Service Identity and Protocol

- AWS model slug: `arc-zonal-shift`
- AWS SDK for Rust slug: `arczonalshift`
- Model version: `2022-10-30`
- Model file: `vendor/api-models-aws/models/arc-zonal-shift/service/2022-10-30/arc-zonal-shift-2022-10-30.json`
- SDK ID: `ARC Zonal Shift`
- Endpoint prefix: `arc-zonal-shift`
- ARN namespace: `-`
- CloudFormation name: `ARCZonalShift`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Update` (4), `List` (3), `Cancel` (2), `Get` (2), `Start` (2), `Create` (1), `Delete` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelPracticeRun`, `CancelZonalShift`, `CreatePracticeRunConfiguration`, `DeletePracticeRunConfiguration`, `StartPracticeRun`, `StartZonalShift`, `UpdateAutoshiftObserverNotificationStatus`, `UpdatePracticeRunConfiguration`, `UpdateZonalAutoshiftConfiguration`, `UpdateZonalShift`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAutoshiftObserverNotificationStatus`, `GetManagedResource`, `ListAutoshifts`, `ListManagedResources`, `ListZonalShifts`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 4 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelPracticeRun`, `CancelZonalShift`, `StartPracticeRun`, `StartZonalShift`.
- 15 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `CloudWatch`, `EventBridge`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Autoshift` | - | - | `ListAutoshifts` | - |
| `AutoshiftObserverNotification` | - | - | `GetAutoshiftObserverNotificationStatus`, `UpdateAutoshiftObserverNotificationStatus` | - |
| `AutoshiftTriggerResource` | - | - | - | - |
| `ManagedResource` | `resourceIdentifier` | read: `GetManagedResource`; list: `ListManagedResources` | `UpdateZonalAutoshiftConfiguration` | - |
| `PracticeRunConfigurationResource` | `resourceIdentifier` | update: `UpdatePracticeRunConfiguration`; delete: `DeletePracticeRunConfiguration` | `CreatePracticeRunConfiguration` | - |
| `ZonalShiftResource` | `zonalShiftId` | - | `CancelPracticeRun`, `CancelZonalShift`, `UpdateZonalShift` | - |
| `ZonalShifts` | `resourceIdentifier` | list: `ListZonalShifts` | `StartPracticeRun`, `StartZonalShift` | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-autoshift.html
- https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-autoshift.how-it-works.html
- https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-autoshift.start-practice-run.html

Research outcomes:
- ARC Zonal Shift shifts traffic for supported resources away from a specified Availability Zone.
- Zonal autoshift can automatically shift traffic away from an impaired Availability Zone for supported resources.
- Autoshift requires practice runs to validate that applications can tolerate shifting traffic away from an Availability Zone.
- Practice runs are scheduled and can send EventBridge notifications.
- Users can start and cancel zonal shifts, enable or disable autoshift, and configure practice-run settings.
- Zonal shifts have expiry/duration and resource-specific eligibility constraints.

Parity implications:
- Model managed resources, zonal shifts, autoshift state, practice-run configuration, practice-run executions, expiry, cancellation, and EventBridge notifications separately.
- StartZonalShift should validate resource eligibility and create time-bounded traffic-shift state.
- Autoshift and practice runs should be separate from manual zonal shifts even though they share traffic-shift effects.

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
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message, reason, zonalShiftId | The request could not be processed because of conflict in the current state of the resource. |
| `InternalServerException` | `structure` | message | There was an internal server error. |
| `ResourceNotFoundException` | `structure` | message | The input requested a resource that was not found. |
| `ThrottlingException` | `structure` | message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message, reason | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `AppliedStatus` | `enum` | APPLIED, NOT_APPLIED | - |
| `AutoshiftAppliedStatus` | `enum` | APPLIED, NOT_APPLIED | - |
| `AutoshiftExecutionStatus` | `enum` | ACTIVE, COMPLETED | - |
| `AutoshiftObserverNotificationStatus` | `enum` | ENABLED, DISABLED | - |
| `ConflictExceptionReason` | `enum` | ZONAL_SHIFT_ALREADY_EXISTS, ZONAL_SHIFT_STATUS_NOT_ACTIVE, SIMULTANEOUS_ZONAL_SHIFTS_CONFLICT, PRACTICE_CONFIGURATION_ALREADY_EXISTS, AUTOSHIFT_ENABLED, PRACTICE_CONFIGURATION_DOES_NOT_EXIST, ZONAL_AUTOSHIFT_ACTIVE, PRACTICE_OUTCOME_ALARMS_RED, PRACTICE_BLOCKING_ALARMS_RED, PRACTICE_IN_BLOCKED_DATES, PRACTICE_IN_BLOCKED_WINDOWS, PRACTICE_OUTSIDE_ALLOWED_WINDOWS | - |
| `ControlConditionType` | `enum` | CLOUDWATCH | - |
| `PracticeRunOutcome` | `enum` | FAILED, INTERRUPTED, PENDING, SUCCEEDED, CAPACITY_CHECK_FAILED | - |
| `ShiftType` | `enum` | ZONAL_SHIFT, PRACTICE_RUN, FIS_EXPERIMENT, ZONAL_AUTOSHIFT | - |
| `ValidationExceptionReason` | `enum` | INVALID_EXPIRES_IN, INVALID_STATUS, MISSING_VALUE, INVALID_TOKEN, INVALID_RESOURCE_IDENTIFIER, INVALID_AZ, UNSUPPORTED_AZ, INVALID_ALARM_CONDITION, INVALID_CONDITION_TYPE, INVALID_PRACTICE_BLOCKER, FIS_EXPERIMENT_UPDATE_NOT_ALLOWED, AUTOSHIFT_UPDATE_NOT_ALLOWED, ... (+3) | - |
| `ZonalAutoshiftStatus` | `enum` | ENABLED, DISABLED | - |
| `ZonalShiftStatus` | `enum` | ACTIVE, EXPIRED, CANCELED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
