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

- Operations: `ListApplications`, `ListStreamGroups`, `ListStreamSessions`, `ListStreamSessionsByAccount`, `ListTagsForResource`
- Traits: `paginated` (4), `readonly` (5)
- Common required input members in this group: `Identifier`, `ResourceArn`

### Create

- Operations: `CreateApplication`, `CreateStreamGroup`, `CreateStreamSessionConnection`
- Traits: `idempotency-token` (3), `idempotent` (1)
- Common required input members in this group: `ApplicationSourceUri`, `Description`, `ExecutablePath`, `Identifier`, `RuntimeEnvironment`, `SignalRequest`, `StreamClass`, `StreamSessionIdentifier`

### Get

- Operations: `GetApplication`, `GetStreamGroup`, `GetStreamSession`
- Traits: `readonly` (3)
- Common required input members in this group: `Identifier`, `StreamSessionIdentifier`

### Delete

- Operations: `DeleteApplication`, `DeleteStreamGroup`
- Traits: `idempotent` (2)
- Common required input members in this group: `Identifier`

### Update

- Operations: `UpdateApplication`, `UpdateStreamGroup`
- Common required input members in this group: `Identifier`

### Add

- Operations: `AddStreamGroupLocations`
- Common required input members in this group: `Identifier`, `LocationConfigurations`

### Associate

- Operations: `AssociateApplications`
- Traits: `idempotent` (1)
- Common required input members in this group: `ApplicationIdentifiers`, `Identifier`

### Disassociate

- Operations: `DisassociateApplications`
- Traits: `idempotent` (1)
- Common required input members in this group: `ApplicationIdentifiers`, `Identifier`

### Export

- Operations: `ExportStreamSessionFiles`
- Traits: `idempotent` (1)
- Common required input members in this group: `Identifier`, `OutputUri`, `StreamSessionIdentifier`

### Remove

- Operations: `RemoveStreamGroupLocations`
- Traits: `idempotent` (1)
- Common required input members in this group: `Identifier`, `Locations`

### Start

- Operations: `StartStreamSession`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `ApplicationIdentifier`, `Identifier`, `Protocol`, `SignalRequest`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `Tags`

### Terminate

- Operations: `TerminateStreamSession`
- Traits: `idempotent` (1)
- Common required input members in this group: `Identifier`, `StreamSessionIdentifier`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AddStreamGroupLocations` | `POST /streamgroups/{Identifier}/locations` | - | `Identifier`, `LocationConfigurations` | - | `AddStreamGroupLocationsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Add locations that can host stream sessions. To add a location, the stream group must be in `ACTIVE` status. |
| `AssociateApplications` | `POST /streamgroups/{Identifier}/associations` | `idempotent` | `ApplicationIdentifiers`, `Identifier` | - | `AssociateApplicationsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | When you associate, or link, an application with a stream group, then Amazon GameLift Streams can launch the application using the stream group's allocated compute resources. The stream group must be in `ACTIVE` status. |
| `CreateApplication` | `POST /applications` | `idempotency-token` | `ApplicationSourceUri`, `Description`, `ExecutablePath`, `RuntimeEnvironment` | `ClientToken` | `CreateApplicationOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an application resource in Amazon GameLift Streams, which specifies the application content you want to stream, such as a game build or other software, and configures the settings to run it. Before you create an application, upload your application... |
| `CreateStreamGroup` | `POST /streamgroups` | `idempotent`, `idempotency-token` | `Description`, `StreamClass` | `ClientToken` | `CreateStreamGroupOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Stream groups manage how Amazon GameLift Streams allocates resources and handles concurrent streams, allowing you to effectively manage capacity and costs. Within a stream group, you specify an application to stream, streaming locations and their capacity... |
| `CreateStreamSessionConnection` | `POST /streamgroups/{Identifier}/streamsessions/{StreamSessionIdentifier}/connections` | `idempotency-token` | `Identifier`, `SignalRequest`, `StreamSessionIdentifier` | `ClientToken` | `CreateStreamSessionConnectionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables clients to reconnect to a stream session while preserving all session state and data in the disconnected session. This reconnection process can be initiated when a stream session is in either `PENDING_CLIENT_RECONNECTION` or `ACTIVE` status. |
| `DeleteApplication` | `DELETE /applications/{Identifier}` | `idempotent` | `Identifier` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Permanently deletes an Amazon GameLift Streams application resource. This also deletes the application content files stored with Amazon GameLift Streams. |
| `DeleteStreamGroup` | `DELETE /streamgroups/{Identifier}` | `idempotent` | `Identifier` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Permanently deletes all compute resources and information related to a stream group. To delete a stream group, specify the unique stream group identifier. |
| `DisassociateApplications` | `POST /streamgroups/{Identifier}/disassociations` | `idempotent` | `ApplicationIdentifiers`, `Identifier` | - | `DisassociateApplicationsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | When you disassociate, or unlink, an application from a stream group, you can no longer stream this application by using that stream group's allocated compute resources. Any streams in process will continue until they terminate, which helps avoid interrupting... |
| `ExportStreamSessionFiles` | `PUT /streamgroups/{Identifier}/streamsessions/{StreamSessionIdentifier}/exportfiles` | `idempotent` | `Identifier`, `OutputUri`, `StreamSessionIdentifier` | - | `ExportStreamSessionFilesOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Export the files that your application modifies or generates in a stream session, which can help you debug or verify your application. When your application runs, it generates output files such as logs, diagnostic information, crash dumps, save files, user... |
| `GetApplication` | `GET /applications/{Identifier}` | `readonly` | `Identifier` | - | `GetApplicationOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves properties for an Amazon GameLift Streams application resource. Specify the ID of the application that you want to retrieve. |
| `GetStreamGroup` | `GET /streamgroups/{Identifier}` | `readonly` | `Identifier` | - | `GetStreamGroupOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves properties for a Amazon GameLift Streams stream group resource. Specify the ID of the stream group that you want to retrieve. |
| `GetStreamSession` | `GET /streamgroups/{Identifier}/streamsessions/{StreamSessionIdentifier}` | `readonly` | `Identifier`, `StreamSessionIdentifier` | - | `GetStreamSessionOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves properties for a Amazon GameLift Streams stream session resource. Specify the Amazon Resource Name (ARN) of the stream session that you want to retrieve and its stream group ARN. |
| `ListApplications` | `GET /applications` | `readonly`, `paginated` | - | - | `ListApplicationsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of all Amazon GameLift Streams applications that are associated with the Amazon Web Services account in use. This operation returns applications in all statuses, in no particular order. |
| `ListStreamGroups` | `GET /streamgroups` | `readonly`, `paginated` | - | - | `ListStreamGroupsOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of all Amazon GameLift Streams stream groups that are associated with the Amazon Web Services account in use. This operation returns stream groups in all statuses, in no particular order. |
| `ListStreamSessions` | `GET /streamgroups/{Identifier}/streamsessions` | `readonly`, `paginated` | `Identifier` | - | `ListStreamSessionsOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of Amazon GameLift Streams stream sessions that a stream group is hosting. To retrieve stream sessions, specify the stream group, and optionally filter by stream session status. |
| `ListStreamSessionsByAccount` | `GET /streamsessions` | `readonly`, `paginated` | - | - | `ListStreamSessionsByAccountOutput` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of Amazon GameLift Streams stream sessions that this user account has access to. In the returned list of stream sessions, the `ExportFilesMetadata` property only shows the `Status` value. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves all tags assigned to a Amazon GameLift Streams resource. To list tags for a resource, specify the ARN value for the resource. |
| `RemoveStreamGroupLocations` | `DELETE /streamgroups/{Identifier}/locations` | `idempotent` | `Identifier`, `Locations` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a set of remote locations from this stream group. To remove a location, the stream group must be in `ACTIVE` status. |
| `StartStreamSession` | `POST /streamgroups/{Identifier}/streamsessions` | `idempotency-token` | `ApplicationIdentifier`, `Identifier`, `Protocol`, `SignalRequest` | `ClientToken` | `StartStreamSessionOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This action initiates a new stream session and outputs connection information that clients can use to access the stream. A stream session refers to an instance of a stream that Amazon GameLift Streams transmits from the server to the end-user. |
| `TagResource` | `POST /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Assigns one or more tags to a Amazon GameLift Streams resource. Use tags to organize Amazon Web Services resources for a range of purposes. |
| `TerminateStreamSession` | `DELETE /streamgroups/{Identifier}/streamsessions/{StreamSessionIdentifier}` | `idempotent` | `Identifier`, `StreamSessionIdentifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Permanently terminates an active stream session. When called, the stream session status changes to `TERMINATING`. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Removes one or more tags from a Amazon GameLift Streams resource. To remove tags, specify the Amazon GameLift Streams resource and a list of one or more tags to remove. |
| `UpdateApplication` | `PATCH /applications/{Identifier}` | - | `Identifier` | - | `UpdateApplicationOutput` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the mutable configuration settings for a Amazon GameLift Streams application resource. You can change the `Description`, `ApplicationLogOutputUri`, and `ApplicationLogPaths`. |
| `UpdateStreamGroup` | `PATCH /streamgroups/{Identifier}` | - | `Identifier` | - | `UpdateStreamGroupOutput` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the configuration settings for an Amazon GameLift Streams stream group resource. To update a stream group, it must be in `ACTIVE` status. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You don't have the required permissions to access this Amazon GameLift Streams resource. |
| `InternalServerException` | `structure` | `Message` | The service encountered an internal error and is unable to complete the request. |
| `ThrottlingException` | `structure` | `Message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `Message` | One or more parameter values in the request fail to satisfy the specified constraints. |
| `ResourceNotFoundException` | `structure` | `Message` | The resource specified in the request was not found. |
| `ConflictException` | `structure` | `Message` | The requested operation would cause a conflict with the current state of a service resource associated with the request. |
| `ServiceQuotaExceededException` | `structure` | `Message` | The request would cause the resource to exceed an allowed service quota. |
| `AddStreamGroupLocationsInput` | `structure` | `Identifier`, `LocationConfigurations` | - |
| `AddStreamGroupLocationsOutput` | `structure` | `Identifier`, `Locations` | - |
| `AssociateApplicationsInput` | `structure` | `ApplicationIdentifiers`, `Identifier` | - |
| `AssociateApplicationsOutput` | `structure` | `ApplicationArns`, `Arn` | - |
| `CreateApplicationInput` | `structure` | `ApplicationLogOutputUri`, `ApplicationLogPaths`, `ApplicationSourceUri`, `ClientToken`, `Description`, `ExecutablePath`, `RuntimeEnvironment`, `Tags` | - |
| `CreateApplicationOutput` | `structure` | `ApplicationLogOutputUri`, `ApplicationLogPaths`, `ApplicationSourceUri`, `Arn`, `AssociatedStreamGroups`, `CreatedAt`, `Description`, `ExecutablePath`, `Id`, `LastUpdatedAt`, `ReplicationStatuses`, `RuntimeEnvironment`, ... (+2) | - |
| `CreateStreamGroupInput` | `structure` | `ClientToken`, `DefaultApplicationIdentifier`, `Description`, `LocationConfigurations`, `StreamClass`, `Tags` | - |
| `CreateStreamGroupOutput` | `structure` | `Arn`, `AssociatedApplications`, `CreatedAt`, `DefaultApplication`, `Description`, `ExpiresAt`, `Id`, `LastUpdatedAt`, `LocationStates`, `Status`, `StatusReason`, `StreamClass` | - |
| `CreateStreamSessionConnectionInput` | `structure` | `ClientToken`, `Identifier`, `SignalRequest`, `StreamSessionIdentifier` | - |
| `CreateStreamSessionConnectionOutput` | `structure` | `SignalResponse` | - |
| `DeleteApplicationInput` | `structure` | `Identifier` | - |
| `DeleteStreamGroupInput` | `structure` | `Identifier` | - |
| `DisassociateApplicationsInput` | `structure` | `ApplicationIdentifiers`, `Identifier` | - |
| `DisassociateApplicationsOutput` | `structure` | `ApplicationArns`, `Arn` | - |
| `ExportStreamSessionFilesInput` | `structure` | `Identifier`, `OutputUri`, `StreamSessionIdentifier` | - |
| `ExportStreamSessionFilesOutput` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
