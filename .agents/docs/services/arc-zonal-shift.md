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

### Update

- Operations: `UpdateAutoshiftObserverNotificationStatus`, `UpdatePracticeRunConfiguration`, `UpdateZonalAutoshiftConfiguration`, `UpdateZonalShift`
- Traits: `idempotent` (2)
- Common required input members in this group: `resourceIdentifier`, `status`, `zonalAutoshiftStatus`, `zonalShiftId`

### List

- Operations: `ListAutoshifts`, `ListManagedResources`, `ListZonalShifts`
- Traits: `paginated` (3), `readonly` (3)

### Cancel

- Operations: `CancelPracticeRun`, `CancelZonalShift`
- Traits: `idempotent` (1)
- Common required input members in this group: `zonalShiftId`

### Get

- Operations: `GetAutoshiftObserverNotificationStatus`, `GetManagedResource`
- Traits: `readonly` (2)
- Common required input members in this group: `resourceIdentifier`

### Start

- Operations: `StartPracticeRun`, `StartZonalShift`
- Common required input members in this group: `awayFrom`, `comment`, `expiresIn`, `resourceIdentifier`

### Create

- Operations: `CreatePracticeRunConfiguration`
- Common required input members in this group: `outcomeAlarms`, `resourceIdentifier`

### Delete

- Operations: `DeletePracticeRunConfiguration`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceIdentifier`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelPracticeRun` | `DELETE /practiceruns/{zonalShiftId}` | `idempotent` | `zonalShiftId` | - | `CancelPracticeRunResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancel an in-progress practice run zonal shift in Amazon Application Recovery Controller. |
| `CancelZonalShift` | `DELETE /zonalshifts/{zonalShiftId}` | - | `zonalShiftId` | - | `ZonalShift` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancel a zonal shift in Amazon Application Recovery Controller. To cancel the zonal shift, specify the zonal shift ID. |
| `CreatePracticeRunConfiguration` | `POST /configuration` | - | `outcomeAlarms`, `resourceIdentifier` | - | `CreatePracticeRunConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | A practice run configuration for zonal autoshift is required when you enable zonal autoshift. A practice run configuration includes specifications for blocked dates and blocked time windows, and for Amazon CloudWatch alarms that you create to use with... |
| `DeletePracticeRunConfiguration` | `DELETE /configuration/{resourceIdentifier}` | `idempotent` | `resourceIdentifier` | - | `DeletePracticeRunConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the practice run configuration for a resource. Before you can delete a practice run configuration for a resource., you must disable zonal autoshift for the resource. |
| `GetAutoshiftObserverNotificationStatus` | `GET /autoshift-observer-notification` | `readonly` | - | - | `GetAutoshiftObserverNotificationStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException` | Returns the status of the autoshift observer notification. Autoshift observer notifications notify you through Amazon EventBridge when there is an autoshift event for zonal autoshift. |
| `GetManagedResource` | `GET /managedresources/{resourceIdentifier}` | `readonly` | `resourceIdentifier` | - | `GetManagedResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get information about a resource that's been registered for zonal shifts with Amazon Application Recovery Controller in this Amazon Web Services Region. Resources that are registered for zonal shifts are managed resources in ARC. |
| `ListAutoshifts` | `GET /autoshifts` | `readonly`, `paginated` | - | - | `ListAutoshiftsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns the autoshifts for an Amazon Web Services Region. By default, the call returns only `ACTIVE` autoshifts. |
| `ListManagedResources` | `GET /managedresources` | `readonly`, `paginated` | - | - | `ListManagedResourcesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all the resources in your Amazon Web Services account in this Amazon Web Services Region that are managed for zonal shifts in Amazon Application Recovery Controller, and information about them. The information includes the zonal autoshift status for the... |
| `ListZonalShifts` | `GET /zonalshifts` | `readonly`, `paginated` | - | - | `ListZonalShiftsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all active and completed zonal shifts in Amazon Application Recovery Controller in your Amazon Web Services account in this Amazon Web Services Region. `ListZonalShifts` returns customer-initiated zonal shifts, as well as practice run zonal shifts that... |
| `StartPracticeRun` | `POST /practiceruns` | - | `awayFrom`, `comment`, `resourceIdentifier` | - | `StartPracticeRunResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Start an on-demand practice run zonal shift in Amazon Application Recovery Controller. With zonal autoshift enabled, you can start an on-demand practice run to verify preparedness at any time. |
| `StartZonalShift` | `POST /zonalshifts` | - | `awayFrom`, `comment`, `expiresIn`, `resourceIdentifier` | - | `ZonalShift` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | You start a zonal shift to temporarily move load balancer traffic away from an Availability Zone in an Amazon Web Services Region, to help your application recover immediately, for example, from a developer's bad code deployment or from an Amazon Web Services... |
| `UpdateAutoshiftObserverNotificationStatus` | `PUT /autoshift-observer-notification` | `idempotent` | `status` | - | `UpdateAutoshiftObserverNotificationStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Update the status of autoshift observer notification. Autoshift observer notification enables you to be notified, through Amazon EventBridge, when there is an autoshift event for zonal autoshift. |
| `UpdatePracticeRunConfiguration` | `PATCH /configuration/{resourceIdentifier}` | - | `resourceIdentifier` | - | `UpdatePracticeRunConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a practice run configuration to change one or more of the following: add, change, or remove the blocking alarm; change the outcome alarm; or add, change, or remove blocking dates or time windows. |
| `UpdateZonalAutoshiftConfiguration` | `PUT /managedresources/{resourceIdentifier}` | `idempotent` | `resourceIdentifier`, `zonalAutoshiftStatus` | - | `UpdateZonalAutoshiftConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | The zonal autoshift configuration for a resource includes the practice run configuration and the status for running autoshifts, zonal autoshift status. When a resource has a practice run configuration, ARC starts weekly zonal shifts for the resource, to shift... |
| `UpdateZonalShift` | `PATCH /zonalshifts/{zonalShiftId}` | - | `zonalShiftId` | - | `ZonalShift` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update an active zonal shift in Amazon Application Recovery Controller in your Amazon Web Services account. You can update a zonal shift to set a new expiration, or edit or replace the comment for the zonal shift. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message` | There was an internal server error. |
| `ThrottlingException` | `structure` | `message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `message`, `reason` | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `ResourceNotFoundException` | `structure` | `message` | The input requested a resource that was not found. |
| `ConflictException` | `structure` | `message`, `reason`, `zonalShiftId` | The request could not be processed because of conflict in the current state of the resource. |
| `ZonalShift` | `structure` | `awayFrom`, `comment`, `expiryTime`, `resourceIdentifier`, `startTime`, `status`, `zonalShiftId` | - |
| `CancelPracticeRunRequest` | `structure` | `zonalShiftId` | - |
| `CancelPracticeRunResponse` | `structure` | `awayFrom`, `comment`, `expiryTime`, `resourceIdentifier`, `startTime`, `status`, `zonalShiftId` | - |
| `CancelZonalShiftRequest` | `structure` | `zonalShiftId` | - |
| `CreatePracticeRunConfigurationRequest` | `structure` | `allowedWindows`, `blockedDates`, `blockedWindows`, `blockingAlarms`, `outcomeAlarms`, `resourceIdentifier` | - |
| `CreatePracticeRunConfigurationResponse` | `structure` | `arn`, `name`, `practiceRunConfiguration`, `zonalAutoshiftStatus` | - |
| `DeletePracticeRunConfigurationRequest` | `structure` | `resourceIdentifier` | - |
| `DeletePracticeRunConfigurationResponse` | `structure` | `arn`, `name`, `zonalAutoshiftStatus` | - |
| `GetAutoshiftObserverNotificationStatusRequest` | `structure` | - | - |
| `GetAutoshiftObserverNotificationStatusResponse` | `structure` | `status` | - |
| `GetManagedResourceRequest` | `structure` | `resourceIdentifier` | - |
| `GetManagedResourceResponse` | `structure` | `appliedWeights`, `arn`, `autoshifts`, `name`, `practiceRunConfiguration`, `zonalAutoshiftStatus`, `zonalShifts` | - |
| `ListAutoshiftsRequest` | `structure` | `maxResults`, `nextToken`, `status` | - |
| `ListAutoshiftsResponse` | `structure` | `items`, `nextToken` | - |
| `ListManagedResourcesRequest` | `structure` | `maxResults`, `nextToken` | - |
| `ListManagedResourcesResponse` | `structure` | `items`, `nextToken` | - |
| `ListZonalShiftsRequest` | `structure` | `maxResults`, `nextToken`, `resourceIdentifier`, `status` | - |
| `ListZonalShiftsResponse` | `structure` | `items`, `nextToken` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
