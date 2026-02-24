# Route53 Recovery Cluster

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Welcome to the Routing Control (Recovery Cluster) API Reference Guide for Amazon Route 53 Application Recovery Controller. With Route 53 ARC, you can use routing control with extreme reliability to recover applications by rerouting traffic across Availability Zones or Amazon Web Services Regions. Routing controls are simple on/off switches hosted on a highly available cluster in Route 53 ARC. A cluster provides a set of five redundant Regional endpoints against which you can run API calls to get or update the state of routing controls. To implement failover, you set one routing control to ON and another one to OFF, to reroute traffic from one Availability Zone or Amazon Web Services Region to another.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Route53 Recovery Cluster workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Update`, `Get`, `List` operation families, including `UpdateRoutingControlState`, `UpdateRoutingControlStates`, `GetRoutingControlState`, `ListRoutingControls`.

## Service Identity and Protocol

- AWS model slug: `route53-recovery-cluster`
- AWS SDK for Rust slug: `route53recoverycluster`
- Model version: `2019-12-02`
- Model file: `vendor/api-models-aws/models/route53-recovery-cluster/service/2019-12-02/route53-recovery-cluster-2019-12-02.json`
- SDK ID: `Route53 Recovery Cluster`
- Endpoint prefix: `route53-recovery-cluster`
- ARN namespace: `route53-recovery-cluster`
- CloudFormation name: `Route53RecoveryCluster`
- CloudTrail event source: `route53recoverycluster.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Update` (2), `Get` (1), `List` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `UpdateRoutingControlState`, `UpdateRoutingControlStates`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetRoutingControlState`, `ListRoutingControls`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- 4 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `ECS`.

## Operation Groups

### Update

- Operations: `UpdateRoutingControlState`, `UpdateRoutingControlStates`
- Common required input members in this group: `RoutingControlArn`, `RoutingControlState`, `UpdateRoutingControlStateEntries`

### Get

- Operations: `GetRoutingControlState`
- Common required input members in this group: `RoutingControlArn`

### List

- Operations: `ListRoutingControls`
- Traits: `paginated` (1)

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetRoutingControlState` | - | - | `RoutingControlArn` | - | `GetRoutingControlStateResponse` | `AccessDeniedException`, `EndpointTemporarilyUnavailableException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the state for a routing control. A routing control is a simple on/off switch that you can use to route traffic to cells. |
| `ListRoutingControls` | - | `paginated` | - | - | `ListRoutingControlsResponse` | `AccessDeniedException`, `EndpointTemporarilyUnavailableException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List routing control names and Amazon Resource Names (ARNs), as well as the routing control state for each routing control, along with the control panel name and control panel ARN for the routing controls. If you specify a control panel ARN, this call lists... |
| `UpdateRoutingControlState` | - | - | `RoutingControlArn`, `RoutingControlState` | - | `UpdateRoutingControlStateResponse` | `AccessDeniedException`, `ConflictException`, `EndpointTemporarilyUnavailableException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Set the state of the routing control to reroute traffic. You can set the value to ON or OFF. |
| `UpdateRoutingControlStates` | - | - | `UpdateRoutingControlStateEntries` | - | `UpdateRoutingControlStatesResponse` | `AccessDeniedException`, `ConflictException`, `EndpointTemporarilyUnavailableException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceLimitExceededException`, `ThrottlingException`, `ValidationException` | Set multiple routing control states. You can set the value for each state to be ON or OFF. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You don't have sufficient permissions to perform this action. |
| `EndpointTemporarilyUnavailableException` | `structure` | `message` | The cluster endpoint isn't available. |
| `InternalServerException` | `structure` | `message`, `retryAfterSeconds` | There was an unexpected error during processing of the request. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The request references a routing control or control panel that was not found. |
| `ThrottlingException` | `structure` | `message`, `retryAfterSeconds` | The request was denied because of request throttling. |
| `ValidationException` | `structure` | `fields`, `message`, `reason` | There was a validation error on the request. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | There was a conflict with this request. |
| `GetRoutingControlStateRequest` | `structure` | `RoutingControlArn` | - |
| `GetRoutingControlStateResponse` | `structure` | `RoutingControlArn`, `RoutingControlName`, `RoutingControlState` | - |
| `ListRoutingControlsRequest` | `structure` | `ControlPanelArn`, `MaxResults`, `NextToken` | - |
| `ListRoutingControlsResponse` | `structure` | `NextToken`, `RoutingControls` | - |
| `UpdateRoutingControlStateRequest` | `structure` | `RoutingControlArn`, `RoutingControlState`, `SafetyRulesToOverride` | - |
| `UpdateRoutingControlStateResponse` | `structure` | - | - |
| `UpdateRoutingControlStatesRequest` | `structure` | `SafetyRulesToOverride`, `UpdateRoutingControlStateEntries` | - |
| `UpdateRoutingControlStatesResponse` | `structure` | - | - |
| `ServiceLimitExceededException` | `structure` | `limitCode`, `message`, `resourceId`, `resourceType`, `serviceCode` | The request can't update that many routing control states at the same time. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
