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

### List

- Operations: `ListConfigs`, `ListContacts`, `ListDataflowEndpointGroups`, `ListEphemerides`, `ListGroundStations`, `ListMissionProfiles`, `ListSatellites`, `ListTagsForResource`
- Traits: `paginated` (7), `readonly` (8)
- Common required input members in this group: `endTime`, `resourceArn`, `startTime`, `statusList`

### Get

- Operations: `GetAgentConfiguration`, `GetAgentTaskResponseUrl`, `GetConfig`, `GetDataflowEndpointGroup`, `GetMinuteUsage`, `GetMissionProfile`, `GetSatellite`
- Traits: `readonly` (7)
- Common required input members in this group: `agentId`, `configId`, `configType`, `dataflowEndpointGroupId`, `missionProfileId`, `month`, `satelliteId`, `taskId`, `year`

### Create

- Operations: `CreateConfig`, `CreateDataflowEndpointGroup`, `CreateDataflowEndpointGroupV2`, `CreateEphemeris`, `CreateMissionProfile`
- Common required input members in this group: `configData`, `dataflowEdges`, `endpointDetails`, `endpoints`, `minimumViableContactDurationSeconds`, `name`, `trackingConfigArn`

### Delete

- Operations: `DeleteConfig`, `DeleteDataflowEndpointGroup`, `DeleteEphemeris`, `DeleteMissionProfile`
- Traits: `idempotent` (4)
- Common required input members in this group: `configId`, `configType`, `dataflowEndpointGroupId`, `ephemerisId`, `missionProfileId`

### Update

- Operations: `UpdateAgentStatus`, `UpdateConfig`, `UpdateEphemeris`, `UpdateMissionProfile`
- Traits: `idempotent` (4)
- Common required input members in this group: `agentId`, `aggregateStatus`, `componentStatuses`, `configData`, `configId`, `configType`, `enabled`, `ephemerisId`, `missionProfileId`, `name`, `taskId`

### Describe

- Operations: `DescribeContact`, `DescribeEphemeris`
- Traits: `readonly` (2)
- Common required input members in this group: `contactId`, `ephemerisId`

### Cancel

- Operations: `CancelContact`
- Traits: `idempotent` (1)
- Common required input members in this group: `contactId`

### Register

- Operations: `RegisterAgent`
- Common required input members in this group: `agentDetails`, `discoveryData`

### Reserve

- Operations: `ReserveContact`
- Common required input members in this group: `endTime`, `groundStation`, `missionProfileArn`, `startTime`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CancelContact` | `DELETE /contact/{contactId}` | `idempotent` | `contactId` | - | `ContactIdResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Cancels or stops a contact with a specified contact ID based on its position in the contact lifecycle. For contacts that: Have yet to start, the contact will be cancelled. |
| `CreateConfig` | `POST /config` | - | `configData`, `name` | - | `ConfigIdResponse` | `DependencyException`, `InvalidParameterException`, `ResourceLimitExceededException`, `ResourceNotFoundException` | Creates a `Config` with the specified `configData` parameters. Only one type of `configData` can be specified. |
| `CreateDataflowEndpointGroup` | `POST /dataflowEndpointGroup` | - | `endpointDetails` | - | `DataflowEndpointGroupIdResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Creates a `DataflowEndpoint` group containing the specified list of ` DataflowEndpoint` objects. The `name` field in each endpoint is used in your mission profile ` DataflowEndpointConfig` to specify which endpoints to use during a contact. |
| `CreateDataflowEndpointGroupV2` | `POST /dataflowEndpointGroupV2` | - | `endpoints` | - | `CreateDataflowEndpointGroupV2Response` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | Creates a `DataflowEndpoint` group containing the specified list of Ground Station Agent based endpoints. The `name` field in each endpoint is used in your mission profile ` DataflowEndpointConfig` to specify which endpoints to use during a contact. |
| `CreateEphemeris` | `POST /ephemeris` | - | `name` | - | `EphemerisIdResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Create an ephemeris with your specified EphemerisData. |
| `CreateMissionProfile` | `POST /missionprofile` | - | `dataflowEdges`, `minimumViableContactDurationSeconds`, `name`, `trackingConfigArn` | - | `MissionProfileIdResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Creates a mission profile. `dataflowEdges` is a list of lists of strings. |
| `DeleteConfig` | `DELETE /config/{configType}/{configId}` | `idempotent` | `configId`, `configType` | - | `ConfigIdResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Deletes a `Config`. |
| `DeleteDataflowEndpointGroup` | `DELETE /dataflowEndpointGroup/{dataflowEndpointGroupId}` | `idempotent` | `dataflowEndpointGroupId` | - | `DataflowEndpointGroupIdResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Deletes a dataflow endpoint group. |
| `DeleteEphemeris` | `DELETE /ephemeris/{ephemerisId}` | `idempotent` | `ephemerisId` | - | `EphemerisIdResponse` | `DependencyException`, `InvalidParameterException`, `ResourceInUseException`, `ResourceNotFoundException` | Delete an ephemeris. |
| `DeleteMissionProfile` | `DELETE /missionprofile/{missionProfileId}` | `idempotent` | `missionProfileId` | - | `MissionProfileIdResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Deletes a mission profile. |
| `DescribeContact` | `GET /contact/{contactId}` | `readonly` | `contactId` | - | `DescribeContactResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Describes an existing contact. |
| `DescribeEphemeris` | `GET /ephemeris/{ephemerisId}` | `readonly` | `ephemerisId` | - | `DescribeEphemerisResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Retrieve information about an existing ephemeris. |
| `GetAgentConfiguration` | `GET /agent/{agentId}/configuration` | `readonly` | `agentId` | - | `GetAgentConfigurationResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | For use by AWS Ground Station Agent and shouldn't be called directly. Gets the latest configuration information for a registered agent. |
| `GetAgentTaskResponseUrl` | `GET /agentResponseUrl/{agentId}/{taskId}` | `readonly` | `agentId`, `taskId` | - | `GetAgentTaskResponseUrlResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | For use by AWS Ground Station Agent and shouldn't be called directly. Gets a presigned URL for uploading agent task response logs. |
| `GetConfig` | `GET /config/{configType}/{configId}` | `readonly` | `configId`, `configType` | - | `GetConfigResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Returns `Config` information. Only one `Config` response can be returned. |
| `GetDataflowEndpointGroup` | `GET /dataflowEndpointGroup/{dataflowEndpointGroupId}` | `readonly` | `dataflowEndpointGroupId` | - | `GetDataflowEndpointGroupResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Returns the dataflow endpoint group. |
| `GetMinuteUsage` | `POST /minute-usage` | `readonly` | `month`, `year` | - | `GetMinuteUsageResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Returns the number of reserved minutes used by account. |
| `GetMissionProfile` | `GET /missionprofile/{missionProfileId}` | `readonly` | `missionProfileId` | - | `GetMissionProfileResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Returns a mission profile. |
| `GetSatellite` | `GET /satellite/{satelliteId}` | `readonly` | `satelliteId` | - | `GetSatelliteResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Returns a satellite. |
| `ListConfigs` | `GET /config` | `readonly`, `paginated` | - | - | `ListConfigsResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Returns a list of `Config` objects. |
| `ListContacts` | `POST /contacts` | `readonly`, `paginated` | `endTime`, `startTime`, `statusList` | - | `ListContactsResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Returns a list of contacts. If `statusList` contains AVAILABLE, the request must include ` groundStation`, `missionprofileArn`, and `satelliteArn`. |
| `ListDataflowEndpointGroups` | `GET /dataflowEndpointGroup` | `readonly`, `paginated` | - | - | `ListDataflowEndpointGroupsResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Returns a list of `DataflowEndpoint` groups. |
| `ListEphemerides` | `POST /ephemerides` | `readonly`, `paginated` | `endTime`, `startTime` | - | `ListEphemeridesResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | List your existing ephemerides. |
| `ListGroundStations` | `GET /groundstation` | `readonly`, `paginated` | - | - | `ListGroundStationsResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Returns a list of ground stations. |
| `ListMissionProfiles` | `GET /missionprofile` | `readonly`, `paginated` | - | - | `ListMissionProfilesResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Returns a list of mission profiles. |
| `ListSatellites` | `GET /satellite` | `readonly`, `paginated` | - | - | `ListSatellitesResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Returns a list of satellites. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Returns a list of tags for a specified resource. |
| `RegisterAgent` | `POST /agent` | - | `agentDetails`, `discoveryData` | - | `RegisterAgentResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | For use by AWS Ground Station Agent and shouldn't be called directly. Registers a new agent with AWS Ground Station. |
| `ReserveContact` | `POST /contact` | - | `endTime`, `groundStation`, `missionProfileArn`, `startTime` | - | `ContactIdResponse` | `DependencyException`, `InvalidParameterException`, `ResourceLimitExceededException`, `ResourceNotFoundException` | Reserves a contact using specified parameters. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Assigns a tag to a resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Deassigns a resource tag. |
| `UpdateAgentStatus` | `PUT /agent/{agentId}` | `idempotent` | `agentId`, `aggregateStatus`, `componentStatuses`, `taskId` | - | `UpdateAgentStatusResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | For use by AWS Ground Station Agent and shouldn't be called directly. Update the status of the agent. |
| `UpdateConfig` | `PUT /config/{configType}/{configId}` | `idempotent` | `configData`, `configId`, `configType`, `name` | - | `ConfigIdResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Updates the `Config` used when scheduling contacts. Updating a `Config` will not update the execution parameters for existing future contacts scheduled with this `Config`. |
| `UpdateEphemeris` | `PUT /ephemeris/{ephemerisId}` | `idempotent` | `enabled`, `ephemerisId` | - | `EphemerisIdResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Update an existing ephemeris. |
| `UpdateMissionProfile` | `PUT /missionprofile/{missionProfileId}` | `idempotent` | `missionProfileId` | - | `MissionProfileIdResponse` | `DependencyException`, `InvalidParameterException`, `ResourceNotFoundException` | Updates a mission profile. Updating a mission profile will not update the execution parameters for existing future contacts. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `DependencyException` | `structure` | `message`, `parameterName` | Dependency encountered an error. |
| `InvalidParameterException` | `structure` | `message`, `parameterName` | One or more parameters are not valid. |
| `ResourceNotFoundException` | `structure` | `message` | Resource was not found. |
| `ConfigIdResponse` | `structure` | `configArn`, `configId`, `configType` | - |
| `EphemerisIdResponse` | `structure` | `ephemerisId` | - |
| `MissionProfileIdResponse` | `structure` | `missionProfileId` | - |
| `ContactIdResponse` | `structure` | `contactId` | - |
| `ResourceLimitExceededException` | `structure` | `message`, `parameterName` | Account limits for this resource have been exceeded. |
| `DataflowEndpointGroupIdResponse` | `structure` | `dataflowEndpointGroupId` | - |
| `CancelContactRequest` | `structure` | `contactId` | - |
| `CreateConfigRequest` | `structure` | `configData`, `name`, `tags` | - |
| `CreateDataflowEndpointGroupRequest` | `structure` | `contactPostPassDurationSeconds`, `contactPrePassDurationSeconds`, `endpointDetails`, `tags` | - |
| `CreateDataflowEndpointGroupV2Request` | `structure` | `contactPostPassDurationSeconds`, `contactPrePassDurationSeconds`, `endpoints`, `tags` | - |
| `CreateDataflowEndpointGroupV2Response` | `structure` | `dataflowEndpointGroupId` | - |
| `ServiceQuotaExceededException` | `structure` | `message`, `parameterName` | Request would cause a service quota to be exceeded. |
| `CreateEphemerisRequest` | `structure` | `enabled`, `ephemeris`, `expirationTime`, `kmsKeyArn`, `name`, `priority`, `satelliteId`, `tags` | - |
| `CreateMissionProfileRequest` | `structure` | `contactPostPassDurationSeconds`, `contactPrePassDurationSeconds`, `dataflowEdges`, `minimumViableContactDurationSeconds`, `name`, `streamsKmsKey`, `streamsKmsRole`, `tags`, `telemetrySinkConfigArn`, `trackingConfigArn` | - |
| `DeleteConfigRequest` | `structure` | `configId`, `configType` | - |
| `DeleteDataflowEndpointGroupRequest` | `structure` | `dataflowEndpointGroupId` | - |
| `DeleteEphemerisRequest` | `structure` | `ephemerisId` | - |
| `ResourceInUseException` | `structure` | `message` | The specified resource is in use by non-terminal state contacts and cannot be modified or deleted. |
| `DeleteMissionProfileRequest` | `structure` | `missionProfileId` | - |
| `DescribeContactRequest` | `structure` | `contactId` | - |
| `DescribeContactResponse` | `structure` | `contactId`, `contactStatus`, `dataflowList`, `endTime`, `ephemeris`, `errorMessage`, `groundStation`, `maximumElevation`, `missionProfileArn`, `postPassEndTime`, `prePassStartTime`, `region`, ... (+6) | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
