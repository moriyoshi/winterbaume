# Amazon GameLift Streams

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon GameLift Streams Amazon GameLift Streams provides a global cloud solution for content streaming experiences. Use Amazon GameLift Streams tools to upload and configure content for streaming, deploy and scale computing resources to host streams, and manage stream session placement to meet customer demand. This Reference Guide describes the Amazon GameLift Streams service API. You can use the API through the Amazon Web Services SDK, the Command Line Interface (CLI), or by making direct REST calls through HTTPS. See the Amazon GameLift Streams Developer Guide for more information on how Amazon GameLift Streams works and how to work with it.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon GameLift Streams where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon GameLift Streams by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for Amazon GameLift Streams resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon GameLift Streams workflows in the local mock. Key resources include `ApplicationResource`, `StreamGroupResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Create`, `Get`, `Delete`, `Update` operation families, including `ListApplications`, `ListStreamGroups`, `ListStreamSessions`, `ListStreamSessionsByAccount`, `CreateApplication`, `CreateStreamGroup`.

## Service Identity and Protocol

- AWS model slug: `gameliftstreams`
- AWS SDK for Rust slug: `gameliftstreams`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/gameliftstreams/service/2018-05-10/gameliftstreams-2018-05-10.json`
- SDK ID: `GameLiftStreams`
- Endpoint prefix: `-`
- ARN namespace: `gameliftstreams`
- CloudFormation name: `GameLiftStreams`
- CloudTrail event source: `gameliftstreams.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (5), `Create` (3), `Get` (3), `Delete` (2), `Update` (2), `Add` (1), `Associate` (1), `Disassociate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AddStreamGroupLocations`, `AssociateApplications`, `CreateApplication`, `CreateStreamGroup`, `CreateStreamSessionConnection`, `DeleteApplication`, `DeleteStreamGroup`, `DisassociateApplications`, `RemoveStreamGroupLocations`, `StartStreamSession`, `TagResource`, `TerminateStreamSession`, `UntagResource`, `UpdateApplication`, `UpdateStreamGroup`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetApplication`, `GetStreamGroup`, `GetStreamSession`, `ListApplications`, `ListStreamGroups`, `ListStreamSessions`, `ListStreamSessionsByAccount`, `ListTagsForResource`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 13 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ExportStreamSessionFiles`, `StartStreamSession`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 24 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ApplicationResource` | `Arn` | create: `CreateApplication`; read: `GetApplication`; update: `UpdateApplication`; delete: `DeleteApplication`; list: `ListApplications` | - | - |
| `StreamGroupResource` | `Arn` | create: `CreateStreamGroup`; read: `GetStreamGroup`; update: `UpdateStreamGroup`; delete: `DeleteStreamGroup`; list: `ListStreamGroups` | - | - |
## Operation Groups

### List

- Operations: `ListStreamSessions`, `ListStreamSessionsByAccount`, `ListTagsForResource`
- Traits: `readonly` (3), `paginated` (2)
- Common required input members in this group: -

### Add

- Operations: `AddStreamGroupLocations`
- Common required input members in this group: -

### Associate

- Operations: `AssociateApplications`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Create

- Operations: `CreateStreamSessionConnection`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateApplications`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Export

- Operations: `ExportStreamSessionFiles`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Get

- Operations: `GetStreamSession`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Remove

- Operations: `RemoveStreamGroupLocations`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Start

- Operations: `StartStreamSession`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Terminate

- Operations: `TerminateStreamSession`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddStreamGroupLocations` | `POST /streamgroups/{Identifier}/locations` | - | `Identifier`, `LocationConfigurations` | - | `AddStreamGroupLocationsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Add locations that can host stream sessions. To add a location, the stream group must be in ACTIVE status. You configure locations and their corresponding capacity for each stream group. Creating a stream group in a ... |
| `AssociateApplications` | `POST /streamgroups/{Identifier}/associations` | `idempotent` | `Identifier`, `ApplicationIdentifiers` | - | `AssociateApplicationsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | When you associate, or link, an application with a stream group, then Amazon GameLift Streams can launch the application using the stream group's allocated compute resources. The stream group must be in ACTIVE status ... |
| `CreateStreamSessionConnection` | `POST /streamgroups/{Identifier}/streamsessions/{StreamSessionIdentifier}/connections` | `idempotency-token` | `Identifier`, `StreamSessionIdentifier`, `SignalRequest` | `ClientToken` | `CreateStreamSessionConnectionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables clients to reconnect to a stream session while preserving all session state and data in the disconnected session. This reconnection process can be initiated when a stream session is in either PENDING_CLIENT_R ... |
| `DisassociateApplications` | `POST /streamgroups/{Identifier}/disassociations` | `idempotent` | `Identifier`, `ApplicationIdentifiers` | - | `DisassociateApplicationsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | When you disassociate, or unlink, an application from a stream group, you can no longer stream this application by using that stream group's allocated compute resources. Any streams in process will continue until the ... |
| `ExportStreamSessionFiles` | `PUT /streamgroups/{Identifier}/streamsessions/{StreamSessionIdentifier}/exportfiles` | `idempotent` | `Identifier`, `StreamSessionIdentifier`, `OutputUri` | - | `ExportStreamSessionFilesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Export the files that your application modifies or generates in a stream session, which can help you debug or verify your application. When your application runs, it generates output files such as logs, diagnostic in ... |
| `GetStreamSession` | `GET /streamgroups/{Identifier}/streamsessions/{StreamSessionIdentifier}` | `readonly` | `Identifier`, `StreamSessionIdentifier` | - | `GetStreamSessionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves properties for a Amazon GameLift Streams stream session resource. Specify the Amazon Resource Name (ARN) of the stream session that you want to retrieve and its stream group ARN. If the operation is success ... |
| `ListStreamSessions` | `GET /streamgroups/{Identifier}/streamsessions` | `readonly`, `paginated` | `Identifier` | - | `ListStreamSessionsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of Amazon GameLift Streams stream sessions that a stream group is hosting. To retrieve stream sessions, specify the stream group, and optionally filter by stream session status. You can paginate the ... |
| `ListStreamSessionsByAccount` | `GET /streamsessions` | `readonly`, `paginated` | - | - | `ListStreamSessionsByAccountOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of Amazon GameLift Streams stream sessions that this user account has access to. In the returned list of stream sessions, the ExportFilesMetadata property only shows the Status value. To get the Outp ... |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves all tags assigned to a Amazon GameLift Streams resource. To list tags for a resource, specify the ARN value for the resource. Learn more Tagging Amazon Web Services Resources in the Amazon Web Services Gene ... |
| `RemoveStreamGroupLocations` | `DELETE /streamgroups/{Identifier}/locations` | `idempotent` | `Identifier`, `Locations` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a set of remote locations from this stream group. To remove a location, the stream group must be in ACTIVE status. When you remove a location, Amazon GameLift Streams releases allocated compute resources in t ... |
| `StartStreamSession` | `POST /streamgroups/{Identifier}/streamsessions` | `idempotency-token` | `Identifier`, `Protocol`, `SignalRequest`, `ApplicationIdentifier` | `ClientToken` | `StartStreamSessionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This action initiates a new stream session and outputs connection information that clients can use to access the stream. A stream session refers to an instance of a stream that Amazon GameLift Streams transmits from ... |
| `TagResource` | `POST /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Assigns one or more tags to a Amazon GameLift Streams resource. Use tags to organize Amazon Web Services resources for a range of purposes. You can assign tags to the following Amazon GameLift Streams resource types: ... |
| `TerminateStreamSession` | `DELETE /streamgroups/{Identifier}/streamsessions/{StreamSessionIdentifier}` | `idempotent` | `Identifier`, `StreamSessionIdentifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Permanently terminates an active stream session. When called, the stream session status changes to TERMINATING . You can terminate a stream session in any status except ACTIVATING . If the stream session is in ACTIVA ... |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Removes one or more tags from a Amazon GameLift Streams resource. To remove tags, specify the Amazon GameLift Streams resource and a list of one or more tags to remove. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListStreamSessions` | - | `Status -> Status`, `ExportFilesStatus -> ExportFilesStatus`, `NextToken -> NextToken`, `MaxResults -> MaxResults` | - | - |
| `ListStreamSessionsByAccount` | - | `Status -> Status`, `ExportFilesStatus -> ExportFilesStatus`, `NextToken -> NextToken`, `MaxResults -> MaxResults` | - | - |
| `RemoveStreamGroupLocations` | - | `Locations -> locations` | - | - |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You don't have the required permissions to access this Amazon GameLift Streams resource. Correct the permissions before you try again. |
| `ConflictException` | `structure` | Message | The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retryin ... |
| `InternalServerException` | `structure` | Message | The service encountered an internal error and is unable to complete the request. |
| `ResourceNotFoundException` | `structure` | Message | The resource specified in the request was not found. Correct the request before you try again. |
| `ServiceQuotaExceededException` | `structure` | Message | The request would cause the resource to exceed an allowed service quota. Resolve the issue before you try again. |
| `ThrottlingException` | `structure` | Message | The request was denied due to request throttling. Retry the request after the suggested wait time. |
| `ValidationException` | `structure` | Message | One or more parameter values in the request fail to satisfy the specified constraints. Correct the invalid parameter values before retrying the request. |
| `AddStreamGroupLocationsInput` | `structure` | Identifier, LocationConfigurations | - |
| `AddStreamGroupLocationsOutput` | `structure` | Identifier, Locations | - |
| `AssociateApplicationsInput` | `structure` | Identifier, ApplicationIdentifiers | - |
| `AssociateApplicationsOutput` | `structure` | Arn, ApplicationArns | - |
| `CreateStreamSessionConnectionInput` | `structure` | ClientToken, Identifier, StreamSessionIdentifier, SignalRequest | - |
| `CreateStreamSessionConnectionOutput` | `structure` | SignalResponse | - |
| `DisassociateApplicationsInput` | `structure` | Identifier, ApplicationIdentifiers | - |
| `DisassociateApplicationsOutput` | `structure` | Arn, ApplicationArns | - |
| `ExportStreamSessionFilesInput` | `structure` | Identifier, StreamSessionIdentifier, OutputUri | - |
| `ExportStreamSessionFilesOutput` | `structure` | **empty (no members)** | - |
| `GetStreamSessionInput` | `structure` | Identifier, StreamSessionIdentifier | - |
| `GetStreamSessionOutput` | `structure` | Arn, Description, StreamGroupId, UserId, Status, StatusReason, Protocol, Location, SignalRequest, SignalResponse, ConnectionTimeoutSeconds, SessionLengthSeconds, ... (+9) | - |
| `ListStreamSessionsInput` | `structure` | Status, ExportFilesStatus, NextToken, MaxResults, Identifier | - |
| `ListStreamSessionsOutput` | `structure` | Items, NextToken | - |
| `ListStreamSessionsByAccountInput` | `structure` | Status, ExportFilesStatus, NextToken, MaxResults | - |
| `ListStreamSessionsByAccountOutput` | `structure` | Items, NextToken | - |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `RemoveStreamGroupLocationsInput` | `structure` | Identifier, Locations | - |
| `StartStreamSessionInput` | `structure` | ClientToken, Description, Identifier, Protocol, SignalRequest, ApplicationIdentifier, UserId, Locations, ConnectionTimeoutSeconds, SessionLengthSeconds, AdditionalLaunchArgs, AdditionalEnvironmentVariables, ... (+1) | - |
| `StartStreamSessionOutput` | `structure` | Arn, Description, StreamGroupId, UserId, Status, StatusReason, Protocol, Location, SignalRequest, SignalResponse, ConnectionTimeoutSeconds, SessionLengthSeconds, ... (+9) | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `TerminateStreamSessionInput` | `structure` | Identifier, StreamSessionIdentifier | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `ApplicationStatus` | `enum` | INITIALIZED, PROCESSING, READY, DELETING, ERROR | - |
| `ApplicationStatusReason` | `enum` | INTERNAL_ERROR, ACCESS_DENIED, SOURCE_MODIFIED | - |
| `ExportFilesStatus` | `enum` | SUCCEEDED, FAILED, PENDING | - |
| `Protocol` | `enum` | WEBRTC | - |
| `ReplicationStatusType` | `enum` | REPLICATING, COMPLETED | - |
| `RuntimeEnvironmentType` | `enum` | PROTON, WINDOWS, UBUNTU | - |
| `StreamClass` | `enum` | gen4n_high, gen4n_ultra, gen4n_win2022, gen5n_high, gen5n_ultra, gen5n_win2022, gen6n_small, gen6n_medium, gen6n_high, gen6n_ultra, gen6n_ultra_win2022, gen6n_pro, ... (+5) | - |
| `StreamGroupLocationStatus` | `enum` | ACTIVATING, ACTIVE, ERROR, REMOVING | - |
| `StreamGroupStatus` | `enum` | ACTIVATING, UPDATING_LOCATIONS, ACTIVE, ACTIVE_WITH_ERRORS, ERROR, DELETING, EXPIRED | - |
| `StreamGroupStatusReason` | `enum` | INTERNAL_ERROR, NO_AVAILABLE_INSTANCES | - |
| `StreamSessionStatus` | `enum` | ACTIVATING, ACTIVE, CONNECTED, PENDING_CLIENT_RECONNECTION, RECONNECTING, TERMINATING, TERMINATED, ERROR | - |
| `StreamSessionStatusReason` | `enum` | INTERNAL_ERROR, INVALID_SIGNAL_REQUEST, PLACEMENT_TIMEOUT, APP_LOG_S3_DESTINATION_ERROR, APPLICATION_EXIT, CONNECTION_TIMEOUT, RECONNECTION_TIMEOUT, MAX_SESSION_LENGTH_TIMEOUT, IDLE_TIMEOUT, API_TERMINATED | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
