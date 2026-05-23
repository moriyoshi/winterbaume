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
- Traits: `readonly` (4), `paginated` (3)
- Common required input members in this group: -

### Get

- Operations: `GetDevice`, `GetEnvironment`, `GetSoftwareSet`
- Traits: `readonly` (3)
- Common required input members in this group: `id`

### Update

- Operations: `UpdateDevice`, `UpdateEnvironment`, `UpdateSoftwareSet`
- Traits: `idempotent` (3)
- Common required input members in this group: `id`

### Delete

- Operations: `DeleteDevice`, `DeleteEnvironment`
- Traits: `idempotent` (2), `idempotency-token` (2)
- Common required input members in this group: `id`

### Create

- Operations: `CreateEnvironment`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Deregister

- Operations: `DeregisterDevice`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateEnvironment` | `POST /environments` | `idempotency-token` | `desktopArn` | `clientToken` | `CreateEnvironmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an environment for your thin client devices. |
| `DeleteDevice` | `DELETE /devices/{id}` | `idempotent`, `idempotency-token` | `id` | `clientToken` | `DeleteDeviceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a thin client device. |
| `DeleteEnvironment` | `DELETE /environments/{id}` | `idempotent`, `idempotency-token` | `id` | `clientToken` | `DeleteEnvironmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an environment. |
| `DeregisterDevice` | `POST /deregister-device/{id}` | `idempotent`, `idempotency-token` | `id` | `clientToken` | `DeregisterDeviceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deregisters a thin client device. |
| `GetDevice` | `GET /devices/{id}` | `readonly` | `id` | - | `GetDeviceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information for a thin client device. |
| `GetEnvironment` | `GET /environments/{id}` | `readonly` | `id` | - | `GetEnvironmentResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information for an environment. |
| `GetSoftwareSet` | `GET /softwaresets/{id}` | `readonly` | `id` | - | `GetSoftwareSetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns information for a software set. |
| `ListDevices` | `GET /devices` | `readonly`, `paginated` | - | - | `ListDevicesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of thin client devices. |
| `ListEnvironments` | `GET /environments` | `readonly`, `paginated` | - | - | `ListEnvironmentsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of environments. |
| `ListSoftwareSets` | `GET /softwaresets` | `readonly`, `paginated` | - | - | `ListSoftwareSetsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of software sets. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of tags for a resource. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Assigns one or more tags (key-value pairs) to the specified resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a tag or tags from a resource. |
| `UpdateDevice` | `PATCH /devices/{id}` | `idempotent` | `id` | - | `UpdateDeviceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a thin client device. |
| `UpdateEnvironment` | `PATCH /environments/{id}` | `idempotent` | `id` | - | `UpdateEnvironmentResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an environment. |
| `UpdateSoftwareSet` | `PATCH /softwaresets/{id}` | `idempotent` | `id`, `validationStatus` | - | `UpdateSoftwareSetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a software set. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteDevice` | - | `clientToken -> clientToken` | - | - |
| `DeleteEnvironment` | - | `clientToken -> clientToken` | - | - |
| `ListDevices` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListEnvironments` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListSoftwareSets` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message, resourceId, resourceType | The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retryin ... |
| `InternalServerException` | `structure` | message, retryAfterSeconds | The server encountered an internal error and is unable to complete the request. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | The resource specified in the request was not found. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, serviceCode, quotaCode | Your request exceeds a service quota. |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message, reason, fieldList | The input fails to satisfy the specified constraints. |
| `CreateEnvironmentRequest` | `structure` | name, desktopArn, desktopEndpoint, softwareSetUpdateSchedule, maintenanceWindow, softwareSetUpdateMode, desiredSoftwareSetId, kmsKeyArn, clientToken, tags, deviceCreationTags | - |
| `CreateEnvironmentResponse` | `structure` | environment | - |
| `DeleteDeviceRequest` | `structure` | id, clientToken | - |
| `DeleteDeviceResponse` | `structure` | **empty (no members)** | - |
| `DeleteEnvironmentRequest` | `structure` | id, clientToken | - |
| `DeleteEnvironmentResponse` | `structure` | **empty (no members)** | - |
| `DeregisterDeviceRequest` | `structure` | id, targetDeviceStatus, clientToken | - |
| `DeregisterDeviceResponse` | `structure` | **empty (no members)** | - |
| `GetDeviceRequest` | `structure` | id | - |
| `GetDeviceResponse` | `structure` | device | - |
| `GetEnvironmentRequest` | `structure` | id | - |
| `GetEnvironmentResponse` | `structure` | environment | - |
| `GetSoftwareSetRequest` | `structure` | id | - |
| `GetSoftwareSetResponse` | `structure` | softwareSet | - |
| `ListDevicesRequest` | `structure` | nextToken, maxResults | - |
| `ListDevicesResponse` | `structure` | devices, nextToken | - |
| `ListEnvironmentsRequest` | `structure` | nextToken, maxResults | - |
| `ListEnvironmentsResponse` | `structure` | environments, nextToken | - |
| `ListSoftwareSetsRequest` | `structure` | nextToken, maxResults | - |
| `ListSoftwareSetsResponse` | `structure` | softwareSets, nextToken | - |
| `ListTagsForResourceRequest` | `structure` | resourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `TagResourceRequest` | `structure` | resourceArn, tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `UpdateDeviceRequest` | `structure` | id, name, desiredSoftwareSetId, softwareSetUpdateSchedule | - |
| `UpdateDeviceResponse` | `structure` | device | - |
| `UpdateEnvironmentRequest` | `structure` | id, name, desktopArn, desktopEndpoint, softwareSetUpdateSchedule, maintenanceWindow, softwareSetUpdateMode, desiredSoftwareSetId, deviceCreationTags | - |
| `UpdateEnvironmentResponse` | `structure` | environment | - |
| `UpdateSoftwareSetRequest` | `structure` | id, validationStatus | - |
| `UpdateSoftwareSetResponse` | `structure` | **empty (no members)** | - |
| `ApplyTimeOf` | `enum` | UTC, DEVICE | - |
| `DayOfWeek` | `enum` | MONDAY, TUESDAY, WEDNESDAY, THURSDAY, FRIDAY, SATURDAY, SUNDAY | - |
| `DesktopType` | `enum` | WORKSPACES, APPSTREAM, WORKSPACES_WEB | - |
| `DeviceSoftwareSetComplianceStatus` | `enum` | NONE, COMPLIANT, NOT_COMPLIANT | - |
| `DeviceStatus` | `enum` | REGISTERED, DEREGISTERING, DEREGISTERED, ARCHIVED | - |
| `EnvironmentSoftwareSetComplianceStatus` | `enum` | NO_REGISTERED_DEVICES, COMPLIANT, NOT_COMPLIANT | - |
| `MaintenanceWindowType` | `enum` | SYSTEM, CUSTOM | - |
| `SoftwareSetUpdateMode` | `enum` | USE_LATEST, USE_DESIRED | - |
| `SoftwareSetUpdateSchedule` | `enum` | USE_MAINTENANCE_WINDOW, APPLY_IMMEDIATELY | - |
| `SoftwareSetUpdateStatus` | `enum` | AVAILABLE, IN_PROGRESS, UP_TO_DATE | - |
| `SoftwareSetValidationStatus` | `enum` | VALIDATED, NOT_VALIDATED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
