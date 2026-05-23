# AWS Ground Station

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Welcome to the AWS Ground Station API Reference. AWS Ground Station is a fully managed service that enables you to control satellite communications, downlink and process satellite data, and scale your satellite operations efficiently and cost-effectively without having to build or manage your own ground station infrastructure.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Ground Station workflows in the local mock. Key resources include `Agent`, `Config`, `Contact`, `DataflowEndpointGroup`, `DataflowEndpointGroupV2`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Update` operation families, including `ListConfigs`, `ListContacts`, `ListDataflowEndpointGroups`, `ListEphemerides`, `GetAgentConfiguration`, `GetAgentTaskResponseUrl`.

## Service Identity and Protocol

- AWS model slug: `groundstation`
- AWS SDK for Rust slug: `groundstation`
- Model version: `2019-05-23`
- Model file: `vendor/api-models-aws/models/groundstation/service/2019-05-23/groundstation-2019-05-23.json`
- SDK ID: `GroundStation`
- Endpoint prefix: `-`
- ARN namespace: `groundstation`
- CloudFormation name: `AWSGroundStation`
- CloudTrail event source: `groundstation.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (8), `Get` (7), `Create` (5), `Delete` (4), `Update` (4), `Describe` (2), `Cancel` (1), `Register` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CancelContact`, `CreateConfig`, `CreateDataflowEndpointGroup`, `CreateDataflowEndpointGroupV2`, `CreateEphemeris`, `CreateMissionProfile`, `DeleteConfig`, `DeleteDataflowEndpointGroup`, `DeleteEphemeris`, `DeleteMissionProfile`, `RegisterAgent`, `TagResource`, `UntagResource`, `UpdateAgentStatus`, `UpdateConfig`, `UpdateEphemeris`, `UpdateMissionProfile`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeContact`, `DescribeEphemeris`, `GetAgentConfiguration`, `GetAgentTaskResponseUrl`, `GetConfig`, `GetDataflowEndpointGroup`, `GetMinuteUsage`, `GetMissionProfile`, `GetSatellite`, `ListConfigs`, `ListContacts`, `ListDataflowEndpointGroups`, `ListEphemerides`, `ListGroundStations`, `ListMissionProfiles`, `ListSatellites`, `ListTagsForResource`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 10 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelContact`, `GetAgentTaskResponseUrl`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 35 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `KMS`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Agent` | `agentId` | create: `RegisterAgent`; read: `GetAgentConfiguration`; update: `UpdateAgentStatus` | - | - |
| `Config` | `configId`, `configType` | create: `CreateConfig`; read: `GetConfig`; update: `UpdateConfig`; delete: `DeleteConfig`; list: `ListConfigs` | - | - |
| `Contact` | `contactId` | create: `ReserveContact`; read: `DescribeContact`; delete: `CancelContact`; list: `ListContacts` | - | - |
| `DataflowEndpointGroup` | `dataflowEndpointGroupId` | create: `CreateDataflowEndpointGroup`; read: `GetDataflowEndpointGroup`; delete: `DeleteDataflowEndpointGroup`; list: `ListDataflowEndpointGroups` | - | - |
| `DataflowEndpointGroupV2` | `dataflowEndpointGroupId` | create: `CreateDataflowEndpointGroupV2` | - | - |
| `Ephemeris` | `ephemerisId` | create: `CreateEphemeris`; read: `DescribeEphemeris`; update: `UpdateEphemeris`; delete: `DeleteEphemeris`; list: `ListEphemerides` | - | - |
| `GroundStationResource` | `groundStationId` | list: `ListGroundStations` | - | - |
| `MissionProfile` | `missionProfileId` | create: `CreateMissionProfile`; read: `GetMissionProfile`; update: `UpdateMissionProfile`; delete: `DeleteMissionProfile`; list: `ListMissionProfiles` | - | - |
| `Satellite` | `satelliteId` | read: `GetSatellite`; list: `ListSatellites` | - | - |
## Operation Groups

### Get

- Operations: `GetAgentTaskResponseUrl`, `GetMinuteUsage`
- Traits: `readonly` (2)
- Common required input members in this group: -

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
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
| `GetAgentTaskResponseUrl` | `GET /agentResponseUrl/{agentId}/{taskId}` | `readonly` | `agentId`, `taskId` | - | `GetAgentTaskResponseUrlResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | For use by AWS Ground Station Agent and shouldn't be called directly. Gets a presigned URL for uploading agent task response logs. |
| `GetMinuteUsage` | `POST /minute-usage` | `readonly` | `month`, `year` | - | `GetMinuteUsageResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Returns the number of reserved minutes used by account. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Returns a list of tags for a specified resource. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Assigns a tag to a resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Deassigns a resource tag. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `DependencyException` | `structure` | message, parameterName | Dependency encountered an error. |
| `InvalidParameterException` | `structure` | message, parameterName | One or more parameters are not valid. |
| `ResourceInUseException` | `structure` | message | The specified resource is in use by non-terminal state contacts and cannot be modified or deleted. |
| `ResourceLimitExceededException` | `structure` | message, parameterName | Account limits for this resource have been exceeded. |
| `ResourceNotFoundException` | `structure` | message | Resource was not found. |
| `ServiceQuotaExceededException` | `structure` | message, parameterName | Request would cause a service quota to be exceeded. |
| `GetAgentTaskResponseUrlRequest` | `structure` | agentId, taskId | - |
| `GetAgentTaskResponseUrlResponse` | `structure` | agentId, taskId, presignedLogUrl | - |
| `GetMinuteUsageRequest` | `structure` | month, year | Input for the GetMinuteUsage operation. |
| `GetMinuteUsageResponse` | `structure` | isReservedMinutesCustomer, totalReservedMinuteAllocation, upcomingMinutesScheduled, totalScheduledMinutes, estimatedMinutesRemaining | Output for the GetMinuteUsage operation. |
| `ListTagsForResourceRequest` | `structure` | resourceArn | Input for the ListTagsForResource operation. |
| `ListTagsForResourceResponse` | `structure` | tags | Output for the ListTagsForResource operation. |
| `TagResourceRequest` | `structure` | resourceArn, tags | Input for the TagResource operation. |
| `TagResourceResponse` | `structure` | **empty (no members)** | Output for the TagResource operation. |
| `UntagResourceRequest` | `structure` | resourceArn, tagKeys | Input for the UntagResource operation. |
| `UntagResourceResponse` | `structure` | **empty (no members)** | Output for the UntagResource operation. |
| `AgentStatus` | `enum` | SUCCESS, FAILED, ACTIVE, INACTIVE | - |
| `AngleUnits` | `enum` | DEGREE_ANGLE, RADIAN | - |
| `AuditResults` | `enum` | HEALTHY, UNHEALTHY | - |
| `BandwidthUnits` | `enum` | GHZ, MHZ, KHZ | - |
| `CapabilityHealth` | `enum` | HEALTHY, UNHEALTHY | - |
| `CapabilityHealthReason` | `enum` | NO_REGISTERED_AGENT, INVALID_IP_OWNERSHIP, NOT_AUTHORIZED_TO_CREATE_SLR, UNVERIFIED_IP_OWNERSHIP, INITIALIZING_DATAPLANE, DATAPLANE_FAILURE, HEALTHY | - |
| `ConfigCapabilityType` | `enum` | ANTENNA_DOWNLINK, ANTENNA_DOWNLINK_DEMOD_DECODE, TRACKING, DATAFLOW_ENDPOINT, ANTENNA_UPLINK, UPLINK_ECHO, S3_RECORDING, TELEMETRY_SINK | - |
| `ContactStatus` | `enum` | SCHEDULING, FAILED_TO_SCHEDULE, SCHEDULED, CANCELLED, AWS_CANCELLED, PREPASS, PASS, POSTPASS, COMPLETED, FAILED, AVAILABLE, CANCELLING, ... (+1) | - |
| `Criticality` | `enum` | REQUIRED, PREFERRED, REMOVED | - |
| `EirpUnits` | `enum` | DBW | - |
| `EndpointStatus` | `enum` | created, creating, deleted, deleting, failed | - |
| `EphemerisErrorCode` | `enum` | INTERNAL_ERROR, MISMATCHED_SATCAT_ID, OEM_VERSION_UNSUPPORTED, ORIGINATOR_MISSING, CREATION_DATE_MISSING, OBJECT_NAME_MISSING, OBJECT_ID_MISSING, REF_FRAME_UNSUPPORTED, REF_FRAME_EPOCH_UNSUPPORTED, TIME_SYSTEM_UNSUPPORTED, CENTER_BODY_UNSUPPORTED, INTERPOLATION_MISSING, ... (+26) | - |
| `EphemerisInvalidReason` | `enum` | METADATA_INVALID, TIME_RANGE_INVALID, TRAJECTORY_INVALID, KMS_KEY_INVALID, VALIDATION_ERROR | - |
| `EphemerisSource` | `enum` | CUSTOMER_PROVIDED, SPACE_TRACK | - |
| `EphemerisStatus` | `enum` | VALIDATING, INVALID, ERROR, ENABLED, DISABLED, EXPIRED | - |
| `EphemerisType` | `enum` | TLE, OEM, AZ_EL, SERVICE_MANAGED | - |
| `FrequencyUnits` | `enum` | GHZ, MHZ, KHZ | - |
| `MaintenanceType` | `enum` | PLANNED, UNPLANNED | - |
| `Polarization` | `enum` | RIGHT_HAND, LEFT_HAND, NONE | - |
| `ReservationType` | `enum` | MAINTENANCE, CONTACT | - |
| `TelemetrySinkType` | `enum` | KINESIS_DATA_STREAM | - |
| `VersionFailureReasonCode` | `enum` | INTERNAL_ERROR, INVALID_SATELLITE_ARN, INVALID_UPDATE_CONTACT_REQUEST, EPHEMERIS_NOT_FOUND, EPHEMERIS_TIME_RANGE_INVALID, EPHEMERIS_NOT_ENABLED, SATELLITE_DOES_NOT_MATCH_EPHEMERIS, NOT_ONBOARDED_TO_AZEL_EPHEMERIS, AZEL_EPHEMERIS_NOT_FOUND, AZEL_EPHEMERIS_WRONG_GROUND_STATION, AZEL_EPHEMERIS_INVALID_STATUS, AZEL_EPHEMERIS_TIME_RANGE_INVALID | - |
| `VersionStatus` | `enum` | UPDATING, ACTIVE, SUPERSEDED, FAILED_TO_UPDATE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
