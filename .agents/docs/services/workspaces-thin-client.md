# Amazon WorkSpaces Thin Client

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon WorkSpaces Thin Client is an affordable device built to work with Amazon Web Services End User Computing (EUC) virtual desktops to provide users with a complete cloud desktop solution. WorkSpaces Thin Client is a compact device designed to connect up to two monitors and USB devices like a keyboard, mouse, headset, and webcam. To maximize endpoint security, WorkSpaces Thin Client devices do not allow local data storage or installation of unapproved applications. The WorkSpaces Thin Client device ships preloaded with device management software. You can use these APIs to complete WorkSpaces Thin Client tasks, such as creating environments or viewing devices.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon WorkSpaces Thin Client workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Update`, `Delete`, `Create` operation families, including `ListDevices`, `ListEnvironments`, `ListSoftwareSets`, `ListTagsForResource`, `GetDevice`, `GetEnvironment`.

## Service Identity and Protocol

- AWS model slug: `workspaces-thin-client`
- AWS SDK for Rust slug: `workspacesthinclient`
- Model version: `2023-08-22`
- Model file: `vendor/api-models-aws/models/workspaces-thin-client/service/2023-08-22/workspaces-thin-client-2023-08-22.json`
- SDK ID: `WorkSpaces Thin Client`
- Endpoint prefix: `-`
- ARN namespace: `thinclient`
- CloudFormation name: `-`
- CloudTrail event source: `thinclient.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Get` (3), `Update` (3), `Delete` (2), `Create` (1), `Deregister` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateEnvironment`, `DeleteDevice`, `DeleteEnvironment`, `DeregisterDevice`, `TagResource`, `UntagResource`, `UpdateDevice`, `UpdateEnvironment`, `UpdateSoftwareSet`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetDevice`, `GetEnvironment`, `GetSoftwareSet`, `ListDevices`, `ListEnvironments`, `ListSoftwareSets`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 8 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 16 operations declare modelled service errors; parity work should map exact error names and retryability where documented.

## Operation Groups

### List

- Operations: `ListDevices`, `ListEnvironments`, `ListSoftwareSets`, `ListTagsForResource`
- Traits: `endpoint-bound` (4), `paginated` (3), `readonly` (4)
- Common required input members in this group: `resourceArn`

### Get

- Operations: `GetDevice`, `GetEnvironment`, `GetSoftwareSet`
- Traits: `endpoint-bound` (3), `readonly` (3)
- Common required input members in this group: `id`

### Update

- Operations: `UpdateDevice`, `UpdateEnvironment`, `UpdateSoftwareSet`
- Traits: `endpoint-bound` (3), `idempotent` (3)
- Common required input members in this group: `id`, `validationStatus`

### Delete

- Operations: `DeleteDevice`, `DeleteEnvironment`
- Traits: `endpoint-bound` (2), `idempotency-token` (2), `idempotent` (2)
- Common required input members in this group: `id`

### Create

- Operations: `CreateEnvironment`
- Traits: `endpoint-bound` (1), `idempotency-token` (1)
- Common required input members in this group: `desktopArn`

### Deregister

- Operations: `DeregisterDevice`
- Traits: `endpoint-bound` (1), `idempotency-token` (1), `idempotent` (1)
- Common required input members in this group: `id`

### Tag

- Operations: `TagResource`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `endpoint-bound` (1), `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateEnvironment` | `POST /environments` | `endpoint-bound`, `idempotency-token` | `desktopArn` | `clientToken` | `CreateEnvironmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an environment for your thin client devices. |
| `DeleteDevice` | `DELETE /devices/{id}` | `idempotent`, `endpoint-bound`, `idempotency-token` | `id` | `clientToken` | `DeleteDeviceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a thin client device. |
| `DeleteEnvironment` | `DELETE /environments/{id}` | `idempotent`, `endpoint-bound`, `idempotency-token` | `id` | `clientToken` | `DeleteEnvironmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an environment. |
| `DeregisterDevice` | `POST /deregister-device/{id}` | `idempotent`, `endpoint-bound`, `idempotency-token` | `id` | `clientToken` | `DeregisterDeviceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deregisters a thin client device. |
| `GetDevice` | `GET /devices/{id}` | `readonly`, `endpoint-bound` | `id` | - | `GetDeviceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information for a thin client device. |
| `GetEnvironment` | `GET /environments/{id}` | `readonly`, `endpoint-bound` | `id` | - | `GetEnvironmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information for an environment. |
| `GetSoftwareSet` | `GET /softwaresets/{id}` | `readonly`, `endpoint-bound` | `id` | - | `GetSoftwareSetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information for a software set. |
| `ListDevices` | `GET /devices` | `readonly`, `paginated`, `endpoint-bound` | - | - | `ListDevicesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of thin client devices. |
| `ListEnvironments` | `GET /environments` | `readonly`, `paginated`, `endpoint-bound` | - | - | `ListEnvironmentsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of environments. |
| `ListSoftwareSets` | `GET /softwaresets` | `readonly`, `paginated`, `endpoint-bound` | - | - | `ListSoftwareSetsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of software sets. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly`, `endpoint-bound` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of tags for a resource. |
| `TagResource` | `POST /tags/{resourceArn}` | `endpoint-bound` | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Assigns one or more tags (key-value pairs) to the specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent`, `endpoint-bound` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a tag or tags from a resource. |
| `UpdateDevice` | `PATCH /devices/{id}` | `idempotent`, `endpoint-bound` | `id` | - | `UpdateDeviceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a thin client device. |
| `UpdateEnvironment` | `PATCH /environments/{id}` | `idempotent`, `endpoint-bound` | `id` | - | `UpdateEnvironmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an environment. |
| `UpdateSoftwareSet` | `PATCH /softwaresets/{id}` | `idempotent`, `endpoint-bound` | `id`, `validationStatus` | - | `UpdateSoftwareSetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a software set. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message`, `retryAfterSeconds` | The server encountered an internal error and is unable to complete the request. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | The input fails to satisfy the specified constraints. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The resource specified in the request was not found. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | The requested operation would cause a conflict with the current state of a service resource associated with the request. |
| `CreateEnvironmentRequest` | `structure` | `clientToken`, `desiredSoftwareSetId`, `desktopArn`, `desktopEndpoint`, `deviceCreationTags`, `kmsKeyArn`, `maintenanceWindow`, `name`, `softwareSetUpdateMode`, `softwareSetUpdateSchedule`, `tags` | - |
| `CreateEnvironmentResponse` | `structure` | `environment` | - |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | Your request exceeds a service quota. |
| `DeleteDeviceRequest` | `structure` | `clientToken`, `id` | - |
| `DeleteDeviceResponse` | `structure` | - | - |
| `DeleteEnvironmentRequest` | `structure` | `clientToken`, `id` | - |
| `DeleteEnvironmentResponse` | `structure` | - | - |
| `DeregisterDeviceRequest` | `structure` | `clientToken`, `id`, `targetDeviceStatus` | - |
| `DeregisterDeviceResponse` | `structure` | - | - |
| `GetDeviceRequest` | `structure` | `id` | - |
| `GetDeviceResponse` | `structure` | `device` | - |
| `GetEnvironmentRequest` | `structure` | `id` | - |
| `GetEnvironmentResponse` | `structure` | `environment` | - |
| `GetSoftwareSetRequest` | `structure` | `id` | - |
| `GetSoftwareSetResponse` | `structure` | `softwareSet` | - |
| `ListDevicesRequest` | `structure` | `maxResults`, `nextToken` | - |
| `ListDevicesResponse` | `structure` | `devices`, `nextToken` | - |
| `ListEnvironmentsRequest` | `structure` | `maxResults`, `nextToken` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
