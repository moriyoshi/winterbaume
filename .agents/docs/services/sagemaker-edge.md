# Amazon Sagemaker Edge Manager

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

SageMaker Edge Manager dataplane service for communicating with active agents.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Sagemaker Edge Manager workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Get`, `Send` operation families, including `GetDeployments`, `GetDeviceRegistration`, `SendHeartbeat`.

## Service Identity and Protocol

- AWS model slug: `sagemaker-edge`
- AWS SDK for Rust slug: `sagemaker`
- Model version: `2020-09-23`
- Model file: `vendor/api-models-aws/models/sagemaker-edge/service/2020-09-23/sagemaker-edge-2020-09-23.json`
- SDK ID: `Sagemaker Edge`
- Endpoint prefix: `edge.sagemaker`
- ARN namespace: `sagemaker`
- CloudFormation name: `SagemakerEdge`
- CloudTrail event source: `sagemakeredge.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (2), `Send` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetDeployments`, `GetDeviceRegistration`.
- 3 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetDeployments`, `GetDeviceRegistration`
- Common required input members in this group: `DeviceFleetName`, `DeviceName`

### Send

- Operations: `SendHeartbeat`
- Common required input members in this group: `AgentVersion`, `DeviceFleetName`, `DeviceName`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetDeployments` | `POST /GetDeployments` | - | `DeviceFleetName`, `DeviceName` | - | `GetDeploymentsResult` | `InternalServiceException` | Use to get the active deployments from a device. |
| `GetDeviceRegistration` | `POST /GetDeviceRegistration` | - | `DeviceFleetName`, `DeviceName` | - | `GetDeviceRegistrationResult` | `InternalServiceException` | Use to check if a device is registered with SageMaker Edge Manager. |
| `SendHeartbeat` | `POST /SendHeartbeat` | - | `AgentVersion`, `DeviceFleetName`, `DeviceName` | - | `Unit` | `InternalServiceException` | Use to get the current status of devices registered on SageMaker Edge Manager. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServiceException` | `structure` | `Message` | An internal failure occurred. |
| `GetDeploymentsRequest` | `structure` | `DeviceFleetName`, `DeviceName` | - |
| `GetDeploymentsResult` | `structure` | `Deployments` | - |
| `GetDeviceRegistrationRequest` | `structure` | `DeviceFleetName`, `DeviceName` | - |
| `GetDeviceRegistrationResult` | `structure` | `CacheTTL`, `DeviceRegistration` | - |
| `SendHeartbeatRequest` | `structure` | `AgentMetrics`, `AgentVersion`, `DeploymentResult`, `DeviceFleetName`, `DeviceName`, `Models` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
