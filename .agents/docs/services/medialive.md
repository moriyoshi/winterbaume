# AWS Elemental MediaLive

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

API for AWS Elemental MediaLive

## Possible Usage Scenarios
- Backported from `crates/winterbaume-medialive/tests/scenario_test.rs`: configure a channel broadcast pipeline and move it through create/describe/start/stop/delete style operations.
- Backported from `scenario_test.rs`: provision an input and associate it with a channel.
- Scenario insight from EC2: exercise account or service defaults for AWS Elemental MediaLive by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: add full state-machine walks for AWS Elemental MediaLive resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: support live video channel assembly, inputs/input security groups, multiplexes, schedules, reservations, tags, and stateful channel operation.

## Service Identity and Protocol

- AWS model slug: `medialive`
- AWS SDK for Rust slug: `medialive`
- Model version: `2017-10-14`
- Model file: `vendor/api-models-aws/models/medialive/service/2017-10-14/medialive-2017-10-14.json`
- SDK ID: `MediaLive`
- Endpoint prefix: `medialive`
- ARN namespace: `medialive`
- CloudFormation name: `MediaLive`
- CloudTrail event source: `medialive.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (24), `Update` (19), `Create` (18), `Delete` (18), `Describe` (17), `Start` (7), `Get` (5), `Batch` (4).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptInputDeviceTransfer`, `BatchDelete`, `BatchStart`, `BatchStop`, `BatchUpdateSchedule`, `CancelInputDeviceTransfer`, `CreateChannel`, `CreateChannelPlacementGroup`, `CreateCloudWatchAlarmTemplate`, `CreateCloudWatchAlarmTemplateGroup`, `CreateCluster`, `CreateEventBridgeRuleTemplate`, `CreateEventBridgeRuleTemplateGroup`, `CreateInput`, `CreateInputSecurityGroup`, `CreateMultiplex`, `CreateMultiplexProgram`, `CreateNetwork`, `CreateNode`, `CreateNodeRegistrationScript`, ... (+52).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeAccountConfiguration`, `DescribeChannel`, `DescribeChannelPlacementGroup`, `DescribeCluster`, `DescribeInput`, `DescribeInputDevice`, `DescribeInputDeviceThumbnail`, `DescribeInputSecurityGroup`, `DescribeMultiplex`, `DescribeMultiplexProgram`, `DescribeNetwork`, `DescribeNode`, `DescribeOffering`, `DescribeReservation`, `DescribeSchedule`, `DescribeSdiSource`, `DescribeThumbnails`, `GetCloudWatchAlarmTemplate`, `GetCloudWatchAlarmTemplateGroup`, `GetEventBridgeRuleTemplate`, ... (+26).
- Pagination is modelled for 23 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 17 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelInputDeviceTransfer`, `StartChannel`, `StartDeleteMonitorDeployment`, `StartInputDevice`, `StartInputDeviceMaintenanceWindow`, `StartMonitorDeployment`, `StartMultiplex`, `StartUpdateSignalMap`, `StopChannel`, `StopInputDevice`, `StopMultiplex`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 123 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `EventBridge`, `EC2/VPC`, `ECS`, `Secrets Manager`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Current Network Resource Stub Semantics

MediaLive currently has shallow networking placeholders.

- `CreateInputSecurityGroup` is a no-state stub that returns a fixed input-security-group-looking ARN and ID with empty or request-shaped rule data.
- Input records can carry security group IDs and VPC-style nested values in service-local state or snapshot JSON slots.
- Channels and inputs do not allocate ENIs, attach to subnets, or validate whitelist rules against EC2 networking.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Operation Groups

### List

- Operations: `ListAlerts`, `ListChannelPlacementGroups`, `ListChannels`, `ListCloudWatchAlarmTemplateGroups`, `ListCloudWatchAlarmTemplates`, `ListClusterAlerts`, `ListClusters`, `ListEventBridgeRuleTemplateGroups`, `ListEventBridgeRuleTemplates`, `ListInputDeviceTransfers`, `ListInputDevices`, `ListInputSecurityGroups`, `ListInputs`, `ListMultiplexAlerts`, `ListMultiplexPrograms`, `ListMultiplexes`, `ListNetworks`, `ListNodes`, `ListOfferings`, `ListReservations`, `ListSdiSources`, `ListSignalMaps`, `ListTagsForResource`, `ListVersions`
- Traits: `paginated` (22)
- Common required input members in this group: `ChannelId`, `ClusterId`, `MultiplexId`, `ResourceArn`, `TransferType`

### Update

- Operations: `UpdateAccountConfiguration`, `UpdateChannel`, `UpdateChannelClass`, `UpdateChannelPlacementGroup`, `UpdateCloudWatchAlarmTemplate`, `UpdateCloudWatchAlarmTemplateGroup`, `UpdateCluster`, `UpdateEventBridgeRuleTemplate`, `UpdateEventBridgeRuleTemplateGroup`, `UpdateInput`, `UpdateInputDevice`, `UpdateInputSecurityGroup`, `UpdateMultiplex`, `UpdateMultiplexProgram`, `UpdateNetwork`, `UpdateNode`, `UpdateNodeState`, `UpdateReservation`, `UpdateSdiSource`
- Common required input members in this group: `ChannelClass`, `ChannelId`, `ChannelPlacementGroupId`, `ClusterId`, `Identifier`, `InputDeviceId`, `InputId`, `InputSecurityGroupId`, `MultiplexId`, `NetworkId`, `NodeId`, `ProgramName`, `ReservationId`, `SdiSourceId`

### Create

- Operations: `CreateChannel`, `CreateChannelPlacementGroup`, `CreateCloudWatchAlarmTemplate`, `CreateCloudWatchAlarmTemplateGroup`, `CreateCluster`, `CreateEventBridgeRuleTemplate`, `CreateEventBridgeRuleTemplateGroup`, `CreateInput`, `CreateInputSecurityGroup`, `CreateMultiplex`, `CreateMultiplexProgram`, `CreateNetwork`, `CreateNode`, `CreateNodeRegistrationScript`, `CreatePartnerInput`, `CreateSdiSource`, `CreateSignalMap`, `CreateTags`
- Traits: `idempotency-token` (16)
- Common required input members in this group: `AvailabilityZones`, `ClusterId`, `ComparisonOperator`, `DiscoveryEntryPointArn`, `EvaluationPeriods`, `EventType`, `GroupIdentifier`, `InputId`, `MetricName`, `MultiplexId`, `MultiplexProgramSettings`, `MultiplexSettings`, `Name`, `Period`, `ProgramName`, `RequestId`, `ResourceArn`, `Statistic`, `TargetResourceType`, `Threshold`, `TreatMissingData`

### Delete

- Operations: `DeleteChannel`, `DeleteChannelPlacementGroup`, `DeleteCloudWatchAlarmTemplate`, `DeleteCloudWatchAlarmTemplateGroup`, `DeleteCluster`, `DeleteEventBridgeRuleTemplate`, `DeleteEventBridgeRuleTemplateGroup`, `DeleteInput`, `DeleteInputSecurityGroup`, `DeleteMultiplex`, `DeleteMultiplexProgram`, `DeleteNetwork`, `DeleteNode`, `DeleteReservation`, `DeleteSchedule`, `DeleteSdiSource`, `DeleteSignalMap`, `DeleteTags`
- Common required input members in this group: `ChannelId`, `ChannelPlacementGroupId`, `ClusterId`, `Identifier`, `InputId`, `InputSecurityGroupId`, `MultiplexId`, `NetworkId`, `NodeId`, `ProgramName`, `ReservationId`, `ResourceArn`, `SdiSourceId`, `TagKeys`

### Describe

- Operations: `DescribeAccountConfiguration`, `DescribeChannel`, `DescribeChannelPlacementGroup`, `DescribeCluster`, `DescribeInput`, `DescribeInputDevice`, `DescribeInputDeviceThumbnail`, `DescribeInputSecurityGroup`, `DescribeMultiplex`, `DescribeMultiplexProgram`, `DescribeNetwork`, `DescribeNode`, `DescribeOffering`, `DescribeReservation`, `DescribeSchedule`, `DescribeSdiSource`, `DescribeThumbnails`
- Traits: `paginated` (1)
- Common required input members in this group: `Accept`, `ChannelId`, `ChannelPlacementGroupId`, `ClusterId`, `InputDeviceId`, `InputId`, `InputSecurityGroupId`, `MultiplexId`, `NetworkId`, `NodeId`, `OfferingId`, `PipelineId`, `ProgramName`, `ReservationId`, `SdiSourceId`, `ThumbnailType`

### Start

- Operations: `StartChannel`, `StartDeleteMonitorDeployment`, `StartInputDevice`, `StartInputDeviceMaintenanceWindow`, `StartMonitorDeployment`, `StartMultiplex`, `StartUpdateSignalMap`
- Common required input members in this group: `ChannelId`, `Identifier`, `InputDeviceId`, `MultiplexId`

### Get

- Operations: `GetCloudWatchAlarmTemplate`, `GetCloudWatchAlarmTemplateGroup`, `GetEventBridgeRuleTemplate`, `GetEventBridgeRuleTemplateGroup`, `GetSignalMap`
- Common required input members in this group: `Identifier`

### Batch

- Operations: `BatchDelete`, `BatchStart`, `BatchStop`, `BatchUpdateSchedule`
- Common required input members in this group: `ChannelId`

### Stop

- Operations: `StopChannel`, `StopInputDevice`, `StopMultiplex`
- Common required input members in this group: `ChannelId`, `InputDeviceId`, `MultiplexId`

### Accept

- Operations: `AcceptInputDeviceTransfer`
- Common required input members in this group: `InputDeviceId`

### Cancel

- Operations: `CancelInputDeviceTransfer`
- Common required input members in this group: `InputDeviceId`

### Claim

- Operations: `ClaimDevice`

### Purchase

- Operations: `PurchaseOffering`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Count`, `OfferingId`

### Reboot

- Operations: `RebootInputDevice`
- Common required input members in this group: `InputDeviceId`

### Reject

- Operations: `RejectInputDeviceTransfer`
- Common required input members in this group: `InputDeviceId`

### Restart

- Operations: `RestartChannelPipelines`
- Common required input members in this group: `ChannelId`

### Transfer

- Operations: `TransferInputDevice`
- Common required input members in this group: `InputDeviceId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptInputDeviceTransfer` | `POST /prod/inputDevices/{InputDeviceId}/accept` | - | `InputDeviceId` | - | `AcceptInputDeviceTransferResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException`, ... (+1) | Accept an incoming input device transfer. The ownership of the device will transfer to your AWS account. |
| `BatchDelete` | `POST /prod/batch/delete` | - | - | - | `BatchDeleteResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Starts delete of resources. |
| `BatchStart` | `POST /prod/batch/start` | - | - | - | `BatchStartResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Starts existing resources |
| `BatchStop` | `POST /prod/batch/stop` | - | - | - | `BatchStopResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Stops running resources |
| `BatchUpdateSchedule` | `PUT /prod/channels/{ChannelId}/schedule` | - | `ChannelId` | - | `BatchUpdateScheduleResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException`, `UnprocessableEntityException` | Update a channel schedule |
| `CancelInputDeviceTransfer` | `POST /prod/inputDevices/{InputDeviceId}/cancel` | - | `InputDeviceId` | - | `CancelInputDeviceTransferResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException`, ... (+1) | Cancel an input device transfer that you have requested. |
| `ClaimDevice` | `POST /prod/claimDevice` | - | - | - | `ClaimDeviceResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException`, `UnprocessableEntityException` | Send a request to claim an AWS Elemental device that you have purchased from a third-party vendor. After the request succeeds, you will own the device. |
| `CreateChannel` | `POST /prod/channels` | `idempotency-token` | - | `RequestId` | `CreateChannelResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException`, `UnprocessableEntityException` | Creates a new channel |
| `CreateChannelPlacementGroup` | `POST /prod/clusters/{ClusterId}/channelplacementgroups` | `idempotency-token` | `ClusterId` | `RequestId` | `CreateChannelPlacementGroupResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException`, `UnprocessableEntityException` | Create a ChannelPlacementGroup in the specified Cluster. As part of the create operation, you specify the Nodes to attach the group to.After you create a ChannelPlacementGroup, you add Channels to the group (you do this by modifying the Channels to add them... |
| `CreateCloudWatchAlarmTemplate` | `POST /prod/cloudwatch-alarm-templates` | `idempotency-token` | `ComparisonOperator`, `EvaluationPeriods`, `GroupIdentifier`, `MetricName`, `Name`, `Period`, `Statistic`, `TargetResourceType`, `Threshold`, `TreatMissingData` | `RequestId` | `CreateCloudWatchAlarmTemplateResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Creates a cloudwatch alarm template to dynamically generate cloudwatch metric alarms on targeted resource types. |
| `CreateCloudWatchAlarmTemplateGroup` | `POST /prod/cloudwatch-alarm-template-groups` | `idempotency-token` | `Name` | `RequestId` | `CreateCloudWatchAlarmTemplateGroupResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Creates a cloudwatch alarm template group to group your cloudwatch alarm templates and to attach to signal maps for dynamically creating alarms. |
| `CreateCluster` | `POST /prod/clusters` | `idempotency-token` | - | `RequestId` | `CreateClusterResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Create a new Cluster. |
| `CreateEventBridgeRuleTemplate` | `POST /prod/eventbridge-rule-templates` | `idempotency-token` | `EventType`, `GroupIdentifier`, `Name` | `RequestId` | `CreateEventBridgeRuleTemplateResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Creates an eventbridge rule template to monitor events and send notifications to your targeted resources. |
| `CreateEventBridgeRuleTemplateGroup` | `POST /prod/eventbridge-rule-template-groups` | `idempotency-token` | `Name` | `RequestId` | `CreateEventBridgeRuleTemplateGroupResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Creates an eventbridge rule template group to group your eventbridge rule templates and to attach to signal maps for dynamically creating notification rules. |
| `CreateInput` | `POST /prod/inputs` | `idempotency-token` | - | `RequestId` | `CreateInputResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Create an input |
| `CreateInputSecurityGroup` | `POST /prod/inputSecurityGroups` | - | - | - | `CreateInputSecurityGroupResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Creates a Input Security Group |
| `CreateMultiplex` | `POST /prod/multiplexes` | `idempotency-token` | `AvailabilityZones`, `MultiplexSettings`, `Name`, `RequestId` | `RequestId` | `CreateMultiplexResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException`, `UnprocessableEntityException` | Create a new multiplex. |
| `CreateMultiplexProgram` | `POST /prod/multiplexes/{MultiplexId}/programs` | `idempotency-token` | `MultiplexId`, `MultiplexProgramSettings`, `ProgramName`, `RequestId` | `RequestId` | `CreateMultiplexProgramResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException`, `UnprocessableEntityException` | Create a new program in the multiplex. |
| `CreateNetwork` | `POST /prod/networks` | `idempotency-token` | - | `RequestId` | `CreateNetworkResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Create as many Networks as you need. You will associate one or more Clusters with each Network.Each Network provides MediaLive Anywhere with required information about the network in your organization that you are using for video encoding using MediaLive. |
| `CreateNode` | `POST /prod/clusters/{ClusterId}/nodes` | `idempotency-token` | `ClusterId` | `RequestId` | `CreateNodeResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException`, `UnprocessableEntityException` | Create a Node in the specified Cluster. You can also create Nodes using the CreateNodeRegistrationScript. |
| `CreateNodeRegistrationScript` | `POST /prod/clusters/{ClusterId}/nodeRegistrationScript` | `idempotency-token` | `ClusterId` | `RequestId` | `CreateNodeRegistrationScriptResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Create the Register Node script for all the nodes intended for a specific Cluster. You will then run the script on each hardware unit that is intended for that Cluster. |
| `CreatePartnerInput` | `POST /prod/inputs/{InputId}/partners` | `idempotency-token` | `InputId` | `RequestId` | `CreatePartnerInputResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Create a partner input |
| `CreateSdiSource` | `POST /prod/sdiSources` | `idempotency-token` | - | `RequestId` | `CreateSdiSourceResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Create an SdiSource for each video source that uses the SDI protocol. You will reference the SdiSource when you create an SDI input in MediaLive. |
| `CreateSignalMap` | `POST /prod/signal-maps` | `idempotency-token` | `DiscoveryEntryPointArn`, `Name` | `RequestId` | `CreateSignalMapResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Initiates the creation of a new signal map. Will discover a new mediaResourceMap based on the provided discoveryEntryPointArn. |
| `CreateTags` | `POST /prod/tags/{ResourceArn}` | - | `ResourceArn` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Create tags for a resource |
| `DeleteChannel` | `DELETE /prod/channels/{ChannelId}` | - | `ChannelId` | - | `DeleteChannelResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Starts deletion of channel. The associated outputs are also deleted. |
| `DeleteChannelPlacementGroup` | `DELETE /prod/clusters/{ClusterId}/channelplacementgroups/{ChannelPlacementGroupId}` | - | `ChannelPlacementGroupId`, `ClusterId` | - | `DeleteChannelPlacementGroupResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Delete the specified ChannelPlacementGroup that exists in the specified Cluster. |
| `DeleteCloudWatchAlarmTemplate` | `DELETE /prod/cloudwatch-alarm-templates/{Identifier}` | - | `Identifier` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Deletes a cloudwatch alarm template. |
| `DeleteCloudWatchAlarmTemplateGroup` | `DELETE /prod/cloudwatch-alarm-template-groups/{Identifier}` | - | `Identifier` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Deletes a cloudwatch alarm template group. You must detach this group from all signal maps and ensure its existing templates are moved to another group or deleted. |
| `DeleteCluster` | `DELETE /prod/clusters/{ClusterId}` | - | `ClusterId` | - | `DeleteClusterResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Delete a Cluster. The Cluster must be idle. |
| `DeleteEventBridgeRuleTemplate` | `DELETE /prod/eventbridge-rule-templates/{Identifier}` | - | `Identifier` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Deletes an eventbridge rule template. |
| `DeleteEventBridgeRuleTemplateGroup` | `DELETE /prod/eventbridge-rule-template-groups/{Identifier}` | - | `Identifier` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Deletes an eventbridge rule template group. You must detach this group from all signal maps and ensure its existing templates are moved to another group or deleted. |
| `DeleteInput` | `DELETE /prod/inputs/{InputId}` | - | `InputId` | - | `DeleteInputResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Deletes the input end point |
| `DeleteInputSecurityGroup` | `DELETE /prod/inputSecurityGroups/{InputSecurityGroupId}` | - | `InputSecurityGroupId` | - | `DeleteInputSecurityGroupResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Deletes an Input Security Group |
| `DeleteMultiplex` | `DELETE /prod/multiplexes/{MultiplexId}` | - | `MultiplexId` | - | `DeleteMultiplexResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Delete a multiplex. The multiplex must be idle. |
| `DeleteMultiplexProgram` | `DELETE /prod/multiplexes/{MultiplexId}/programs/{ProgramName}` | - | `MultiplexId`, `ProgramName` | - | `DeleteMultiplexProgramResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Delete a program from a multiplex. |
| `DeleteNetwork` | `DELETE /prod/networks/{NetworkId}` | - | `NetworkId` | - | `DeleteNetworkResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Delete a Network. The Network must have no resources associated with it. |
| `DeleteNode` | `DELETE /prod/clusters/{ClusterId}/nodes/{NodeId}` | - | `ClusterId`, `NodeId` | - | `DeleteNodeResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Delete a Node. The Node must be IDLE. |
| `DeleteReservation` | `DELETE /prod/reservations/{ReservationId}` | - | `ReservationId` | - | `DeleteReservationResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Delete an expired reservation. |
| `DeleteSchedule` | `DELETE /prod/channels/{ChannelId}/schedule` | - | `ChannelId` | - | `DeleteScheduleResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Delete all schedule actions on a channel. |
| `DeleteSdiSource` | `DELETE /prod/sdiSources/{SdiSourceId}` | - | `SdiSourceId` | - | `DeleteSdiSourceResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Delete an SdiSource. The SdiSource must not be part of any SidSourceMapping and must not be attached to any input. |
| `DeleteSignalMap` | `DELETE /prod/signal-maps/{Identifier}` | - | `Identifier` | - | `Unit` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Deletes the specified signal map. |
| `DeleteTags` | `DELETE /prod/tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `Unit` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Removes tags for a resource |
| `DescribeAccountConfiguration` | `GET /prod/accountConfiguration` | - | - | - | `DescribeAccountConfigurationResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Describe account configuration |
| `DescribeChannel` | `GET /prod/channels/{ChannelId}` | - | `ChannelId` | - | `DescribeChannelResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Gets details about a channel |
| `DescribeChannelPlacementGroup` | `GET /prod/clusters/{ClusterId}/channelplacementgroups/{ChannelPlacementGroupId}` | - | `ChannelPlacementGroupId`, `ClusterId` | - | `DescribeChannelPlacementGroupResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Get details about a ChannelPlacementGroup. |
| `DescribeCluster` | `GET /prod/clusters/{ClusterId}` | - | `ClusterId` | - | `DescribeClusterResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Get details about a Cluster. |
| `DescribeInput` | `GET /prod/inputs/{InputId}` | - | `InputId` | - | `DescribeInputResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Produces details about an input |
| `DescribeInputDevice` | `GET /prod/inputDevices/{InputDeviceId}` | - | `InputDeviceId` | - | `DescribeInputDeviceResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Gets the details for the input device |
| `DescribeInputDeviceThumbnail` | `GET /prod/inputDevices/{InputDeviceId}/thumbnailData` | - | `Accept`, `InputDeviceId` | - | `DescribeInputDeviceThumbnailResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Get the latest thumbnail data for the input device. |
| `DescribeInputSecurityGroup` | `GET /prod/inputSecurityGroups/{InputSecurityGroupId}` | - | `InputSecurityGroupId` | - | `DescribeInputSecurityGroupResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Produces a summary of an Input Security Group |
| `DescribeMultiplex` | `GET /prod/multiplexes/{MultiplexId}` | - | `MultiplexId` | - | `DescribeMultiplexResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Gets details about a multiplex. |
| `DescribeMultiplexProgram` | `GET /prod/multiplexes/{MultiplexId}/programs/{ProgramName}` | - | `MultiplexId`, `ProgramName` | - | `DescribeMultiplexProgramResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Get the details for a program in a multiplex. |
| `DescribeNetwork` | `GET /prod/networks/{NetworkId}` | - | `NetworkId` | - | `DescribeNetworkResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Get details about a Network. |
| `DescribeNode` | `GET /prod/clusters/{ClusterId}/nodes/{NodeId}` | - | `ClusterId`, `NodeId` | - | `DescribeNodeResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Get details about a Node in the specified Cluster. |
| `DescribeOffering` | `GET /prod/offerings/{OfferingId}` | - | `OfferingId` | - | `DescribeOfferingResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Get details for an offering. |
| `DescribeReservation` | `GET /prod/reservations/{ReservationId}` | - | `ReservationId` | - | `DescribeReservationResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Get details for a reservation. |
| `DescribeSchedule` | `GET /prod/channels/{ChannelId}/schedule` | `paginated` | `ChannelId` | - | `DescribeScheduleResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Get a channel schedule |
| `DescribeSdiSource` | `GET /prod/sdiSources/{SdiSourceId}` | - | `SdiSourceId` | - | `DescribeSdiSourceResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Gets details about a SdiSource. |
| `DescribeThumbnails` | `GET /prod/channels/{ChannelId}/thumbnails` | - | `ChannelId`, `PipelineId`, `ThumbnailType` | - | `DescribeThumbnailsResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Describe the latest thumbnails data. |
| `GetCloudWatchAlarmTemplate` | `GET /prod/cloudwatch-alarm-templates/{Identifier}` | - | `Identifier` | - | `GetCloudWatchAlarmTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Retrieves the specified cloudwatch alarm template. |
| `GetCloudWatchAlarmTemplateGroup` | `GET /prod/cloudwatch-alarm-template-groups/{Identifier}` | - | `Identifier` | - | `GetCloudWatchAlarmTemplateGroupResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Retrieves the specified cloudwatch alarm template group. |
| `GetEventBridgeRuleTemplate` | `GET /prod/eventbridge-rule-templates/{Identifier}` | - | `Identifier` | - | `GetEventBridgeRuleTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Retrieves the specified eventbridge rule template. |
| `GetEventBridgeRuleTemplateGroup` | `GET /prod/eventbridge-rule-template-groups/{Identifier}` | - | `Identifier` | - | `GetEventBridgeRuleTemplateGroupResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Retrieves the specified eventbridge rule template group. |
| `GetSignalMap` | `GET /prod/signal-maps/{Identifier}` | - | `Identifier` | - | `GetSignalMapResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Retrieves the specified signal map. |
| `ListAlerts` | `GET /prod/channels/{ChannelId}/alerts` | `paginated` | `ChannelId` | - | `ListAlertsResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | List the alerts for a channel with optional filtering based on alert state. |
| `ListChannelPlacementGroups` | `GET /prod/clusters/{ClusterId}/channelplacementgroups` | `paginated` | `ClusterId` | - | `ListChannelPlacementGroupsResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Retrieve the list of ChannelPlacementGroups in the specified Cluster. |
| `ListChannels` | `GET /prod/channels` | `paginated` | - | - | `ListChannelsResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Produces list of channels that have been created |
| `ListCloudWatchAlarmTemplateGroups` | `GET /prod/cloudwatch-alarm-template-groups` | `paginated` | - | - | `ListCloudWatchAlarmTemplateGroupsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Lists cloudwatch alarm template groups. |
| `ListCloudWatchAlarmTemplates` | `GET /prod/cloudwatch-alarm-templates` | `paginated` | - | - | `ListCloudWatchAlarmTemplatesResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Lists cloudwatch alarm templates. |
| `ListClusterAlerts` | `GET /prod/clusters/{ClusterId}/alerts` | `paginated` | `ClusterId` | - | `ListClusterAlertsResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | List the alerts for a cluster with optional filtering based on alert state. |
| `ListClusters` | `GET /prod/clusters` | `paginated` | - | - | `ListClustersResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Retrieve the list of Clusters. |
| `ListEventBridgeRuleTemplateGroups` | `GET /prod/eventbridge-rule-template-groups` | `paginated` | - | - | `ListEventBridgeRuleTemplateGroupsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Lists eventbridge rule template groups. |
| `ListEventBridgeRuleTemplates` | `GET /prod/eventbridge-rule-templates` | `paginated` | - | - | `ListEventBridgeRuleTemplatesResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Lists eventbridge rule templates. |
| `ListInputDeviceTransfers` | `GET /prod/inputDeviceTransfers` | `paginated` | `TransferType` | - | `ListInputDeviceTransfersResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException`, `UnprocessableEntityException` | List input devices that are currently being transferred. List input devices that you are transferring from your AWS account or input devices that another AWS account is transferring to you. |
| `ListInputDevices` | `GET /prod/inputDevices` | `paginated` | - | - | `ListInputDevicesResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | List input devices |
| `ListInputSecurityGroups` | `GET /prod/inputSecurityGroups` | `paginated` | - | - | `ListInputSecurityGroupsResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Produces a list of Input Security Groups for an account |
| `ListInputs` | `GET /prod/inputs` | `paginated` | - | - | `ListInputsResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Produces list of inputs that have been created |
| `ListMultiplexAlerts` | `GET /prod/multiplexes/{MultiplexId}/alerts` | `paginated` | `MultiplexId` | - | `ListMultiplexAlertsResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | List the alerts for a multiplex with optional filtering based on alert state. |
| `ListMultiplexPrograms` | `GET /prod/multiplexes/{MultiplexId}/programs` | `paginated` | `MultiplexId` | - | `ListMultiplexProgramsResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | List the programs that currently exist for a specific multiplex. |
| `ListMultiplexes` | `GET /prod/multiplexes` | `paginated` | - | - | `ListMultiplexesResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Retrieve a list of the existing multiplexes. |
| `ListNetworks` | `GET /prod/networks` | `paginated` | - | - | `ListNetworksResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Retrieve the list of Networks. |
| `ListNodes` | `GET /prod/clusters/{ClusterId}/nodes` | `paginated` | `ClusterId` | - | `ListNodesResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Retrieve the list of Nodes. |
| `ListOfferings` | `GET /prod/offerings` | `paginated` | - | - | `ListOfferingsResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | List offerings available for purchase. |
| `ListReservations` | `GET /prod/reservations` | `paginated` | - | - | `ListReservationsResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | List purchased reservations. |
| `ListSdiSources` | `GET /prod/sdiSources` | `paginated` | - | - | `ListSdiSourcesResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | List all the SdiSources in the AWS account. |
| `ListSignalMaps` | `GET /prod/signal-maps` | `paginated` | - | - | `ListSignalMapsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Lists signal maps. |
| `ListTagsForResource` | `GET /prod/tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException` | Produces list of tags that have been created for a resource |
| `ListVersions` | `GET /prod/versions` | - | - | - | `ListVersionsResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Retrieves an array of all the encoder engine versions that are available in this AWS account. |
| `PurchaseOffering` | `POST /prod/offerings/{OfferingId}/purchase` | `idempotency-token` | `Count`, `OfferingId` | `RequestId` | `PurchaseOfferingResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Purchase an offering and create a reservation. |
| `RebootInputDevice` | `POST /prod/inputDevices/{InputDeviceId}/reboot` | - | `InputDeviceId` | - | `RebootInputDeviceResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException`, `UnprocessableEntityException` | Send a reboot command to the specified input device. The device will begin rebooting within a few seconds of sending the command. |
| `RejectInputDeviceTransfer` | `POST /prod/inputDevices/{InputDeviceId}/reject` | - | `InputDeviceId` | - | `RejectInputDeviceTransferResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException`, ... (+1) | Reject the transfer of the specified input device to your AWS account. |
| `RestartChannelPipelines` | `POST /prod/channels/{ChannelId}/restartChannelPipelines` | - | `ChannelId` | - | `RestartChannelPipelinesResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Restart pipelines in one channel that is currently running. |
| `StartChannel` | `POST /prod/channels/{ChannelId}/start` | - | `ChannelId` | - | `StartChannelResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Starts an existing channel |
| `StartDeleteMonitorDeployment` | `DELETE /prod/signal-maps/{Identifier}/monitor-deployment` | - | `Identifier` | - | `StartDeleteMonitorDeploymentResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Initiates a deployment to delete the monitor of the specified signal map. |
| `StartInputDevice` | `POST /prod/inputDevices/{InputDeviceId}/start` | - | `InputDeviceId` | - | `StartInputDeviceResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException`, `UnprocessableEntityException` | Start an input device that is attached to a MediaConnect flow. (There is no need to start a device that is attached to a MediaLive input; MediaLive starts the device when the channel starts.) |
| `StartInputDeviceMaintenanceWindow` | `POST /prod/inputDevices/{InputDeviceId}/startInputDeviceMaintenanceWindow` | - | `InputDeviceId` | - | `StartInputDeviceMaintenanceWindowResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException`, `UnprocessableEntityException` | Start a maintenance window for the specified input device. Starting a maintenance window will give the device up to two hours to install software. |
| `StartMonitorDeployment` | `POST /prod/signal-maps/{Identifier}/monitor-deployment` | - | `Identifier` | - | `StartMonitorDeploymentResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Initiates a deployment to deploy the latest monitor of the specified signal map. |
| `StartMultiplex` | `POST /prod/multiplexes/{MultiplexId}/start` | - | `MultiplexId` | - | `StartMultiplexResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Start (run) the multiplex. Starting the multiplex does not start the channels. |
| `StartUpdateSignalMap` | `PATCH /prod/signal-maps/{Identifier}` | - | `Identifier` | - | `StartUpdateSignalMapResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Initiates an update for the specified signal map. Will discover a new signal map if a changed discoveryEntryPointArn is provided. |
| `StopChannel` | `POST /prod/channels/{ChannelId}/stop` | - | `ChannelId` | - | `StopChannelResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Stops a running channel |
| `StopInputDevice` | `POST /prod/inputDevices/{InputDeviceId}/stop` | - | `InputDeviceId` | - | `StopInputDeviceResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException`, `UnprocessableEntityException` | Stop an input device that is attached to a MediaConnect flow. (There is no need to stop a device that is attached to a MediaLive input; MediaLive automatically stops the device when the channel stops.) |
| `StopMultiplex` | `POST /prod/multiplexes/{MultiplexId}/stop` | - | `MultiplexId` | - | `StopMultiplexResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Stops a running multiplex. If the multiplex isn't running, this action has no effect. |
| `TransferInputDevice` | `POST /prod/inputDevices/{InputDeviceId}/transfer` | - | `InputDeviceId` | - | `TransferInputDeviceResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException`, ... (+1) | Start an input device transfer to another AWS account. After you make the request, the other account must accept or reject the transfer. |
| `UpdateAccountConfiguration` | `PUT /prod/accountConfiguration` | - | - | - | `UpdateAccountConfigurationResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException`, `UnprocessableEntityException` | Update account configuration |
| `UpdateChannel` | `PUT /prod/channels/{ChannelId}` | - | `ChannelId` | - | `UpdateChannelResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `UnprocessableEntityException` | Updates a channel. |
| `UpdateChannelClass` | `PUT /prod/channels/{ChannelId}/channelClass` | - | `ChannelClass`, `ChannelId` | - | `UpdateChannelClassResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException`, ... (+1) | Changes the class of the channel. |
| `UpdateChannelPlacementGroup` | `PUT /prod/clusters/{ClusterId}/channelplacementgroups/{ChannelPlacementGroupId}` | - | `ChannelPlacementGroupId`, `ClusterId` | - | `UpdateChannelPlacementGroupResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException`, `UnprocessableEntityException` | Change the settings for a ChannelPlacementGroup. |
| `UpdateCloudWatchAlarmTemplate` | `PATCH /prod/cloudwatch-alarm-templates/{Identifier}` | - | `Identifier` | - | `UpdateCloudWatchAlarmTemplateResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Updates the specified cloudwatch alarm template. |
| `UpdateCloudWatchAlarmTemplateGroup` | `PATCH /prod/cloudwatch-alarm-template-groups/{Identifier}` | - | `Identifier` | - | `UpdateCloudWatchAlarmTemplateGroupResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Updates the specified cloudwatch alarm template group. |
| `UpdateCluster` | `PUT /prod/clusters/{ClusterId}` | - | `ClusterId` | - | `UpdateClusterResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Change the settings for a Cluster. |
| `UpdateEventBridgeRuleTemplate` | `PATCH /prod/eventbridge-rule-templates/{Identifier}` | - | `Identifier` | - | `UpdateEventBridgeRuleTemplateResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Updates the specified eventbridge rule template. |
| `UpdateEventBridgeRuleTemplateGroup` | `PATCH /prod/eventbridge-rule-template-groups/{Identifier}` | - | `Identifier` | - | `UpdateEventBridgeRuleTemplateGroupResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Updates the specified eventbridge rule template group. |
| `UpdateInput` | `PUT /prod/inputs/{InputId}` | - | `InputId` | - | `UpdateInputResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException` | Updates an input. |
| `UpdateInputDevice` | `PUT /prod/inputDevices/{InputDeviceId}` | - | `InputDeviceId` | - | `UpdateInputDeviceResponse` | `BadGatewayException`, `BadRequestException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException`, `UnprocessableEntityException` | Updates the parameters for the input device. |
| `UpdateInputSecurityGroup` | `PUT /prod/inputSecurityGroups/{InputSecurityGroupId}` | - | `InputSecurityGroupId` | - | `UpdateInputSecurityGroupResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException` | Update an Input Security Group's Whilelists. |
| `UpdateMultiplex` | `PUT /prod/multiplexes/{MultiplexId}` | - | `MultiplexId` | - | `UpdateMultiplexResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `UnprocessableEntityException` | Updates a multiplex. |
| `UpdateMultiplexProgram` | `PUT /prod/multiplexes/{MultiplexId}/programs/{ProgramName}` | - | `MultiplexId`, `ProgramName` | - | `UpdateMultiplexProgramResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `UnprocessableEntityException` | Update a program in a multiplex. |
| `UpdateNetwork` | `PUT /prod/networks/{NetworkId}` | - | `NetworkId` | - | `UpdateNetworkResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Change the settings for a Network. |
| `UpdateNode` | `PUT /prod/clusters/{ClusterId}/nodes/{NodeId}` | - | `ClusterId`, `NodeId` | - | `UpdateNodeResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Change the settings for a Node. |
| `UpdateNodeState` | `PUT /prod/clusters/{ClusterId}/nodes/{NodeId}/state` | - | `ClusterId`, `NodeId` | - | `UpdateNodeStateResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException`, `UnprocessableEntityException` | Update the state of a node. |
| `UpdateReservation` | `PUT /prod/reservations/{ReservationId}` | - | `ReservationId` | - | `UpdateReservationResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `NotFoundException`, `TooManyRequestsException` | Update reservation. |
| `UpdateSdiSource` | `PUT /prod/sdiSources/{SdiSourceId}` | - | `SdiSourceId` | - | `UpdateSdiSourceResponse` | `BadGatewayException`, `BadRequestException`, `ConflictException`, `ForbiddenException`, `GatewayTimeoutException`, `InternalServerErrorException`, `TooManyRequestsException` | Change some of the settings in an SdiSource. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `Message` | Placeholder documentation for BadRequestException |
| `ForbiddenException` | `structure` | `Message` | Placeholder documentation for ForbiddenException |
| `InternalServerErrorException` | `structure` | `Message` | Placeholder documentation for InternalServerErrorException |
| `TooManyRequestsException` | `structure` | `Message` | Placeholder documentation for TooManyRequestsException |
| `BadGatewayException` | `structure` | `Message` | Placeholder documentation for BadGatewayException |
| `GatewayTimeoutException` | `structure` | `Message` | Placeholder documentation for GatewayTimeoutException |
| `NotFoundException` | `structure` | `Message` | Placeholder documentation for NotFoundException |
| `ConflictException` | `structure` | `Message` | Placeholder documentation for ConflictException |
| `UnprocessableEntityException` | `structure` | `Message`, `ValidationErrors` | Placeholder documentation for UnprocessableEntityException |
| `AcceptInputDeviceTransferRequest` | `structure` | `InputDeviceId` | Placeholder documentation for AcceptInputDeviceTransferRequest |
| `AcceptInputDeviceTransferResponse` | `structure` | - | Placeholder documentation for AcceptInputDeviceTransferResponse |
| `BatchDeleteRequest` | `structure` | `ChannelIds`, `InputIds`, `InputSecurityGroupIds`, `MultiplexIds` | A request to delete resources |
| `BatchDeleteResponse` | `structure` | `Failed`, `Successful` | Placeholder documentation for BatchDeleteResponse |
| `BatchStartRequest` | `structure` | `ChannelIds`, `MultiplexIds` | A request to start resources |
| `BatchStartResponse` | `structure` | `Failed`, `Successful` | Placeholder documentation for BatchStartResponse |
| `BatchStopRequest` | `structure` | `ChannelIds`, `MultiplexIds` | A request to stop resources |
| `BatchStopResponse` | `structure` | `Failed`, `Successful` | Placeholder documentation for BatchStopResponse |
| `BatchUpdateScheduleRequest` | `structure` | `ChannelId`, `Creates`, `Deletes` | List of actions to create and list of actions to delete. |
| `BatchUpdateScheduleResponse` | `structure` | `Creates`, `Deletes` | Placeholder documentation for BatchUpdateScheduleResponse |
| `CancelInputDeviceTransferRequest` | `structure` | `InputDeviceId` | Placeholder documentation for CancelInputDeviceTransferRequest |
| `CancelInputDeviceTransferResponse` | `structure` | - | Placeholder documentation for CancelInputDeviceTransferResponse |
| `ClaimDeviceRequest` | `structure` | `Id` | A request to claim an AWS Elemental device that you have purchased from a third-party vendor. |
| `ClaimDeviceResponse` | `structure` | - | Placeholder documentation for ClaimDeviceResponse |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
