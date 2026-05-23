# AWS IoT Wireless

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS IoT Wireless provides bi-directional communication between internet-connected wireless devices and the AWS Cloud. To onboard both LoRaWAN and Sidewalk devices to AWS IoT, use the IoT Wireless API. These wireless devices use the Low Power Wide Area Networking (LPWAN) communication protocol to communicate with AWS IoT. Using the API, you can perform create, read, update, and delete operations for your wireless devices, gateways, destinations, and profiles. After onboarding your devices, you can use the API operations to set log levels and monitor your devices with CloudWatch.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS IoT Wireless where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS IoT Wireless by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for AWS IoT Wireless resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS IoT Wireless workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Update`, `Delete`, `Create` operation families, including `GetDestination`, `GetDeviceProfile`, `GetEventConfigurationByResourceTypes`, `GetFuotaTask`, `ListDestinations`, `ListDeviceProfiles`.

## Service Identity and Protocol

- AWS model slug: `iot-wireless`
- AWS SDK for Rust slug: `iotwireless`
- Model version: `2020-11-22`
- Model file: `vendor/api-models-aws/models/iot-wireless/service/2020-11-22/iot-wireless-2020-11-22.json`
- SDK ID: `IoT Wireless`
- Endpoint prefix: `api.iotwireless`
- ARN namespace: `iotwireless`
- CloudFormation name: `IoTWireless`
- CloudTrail event source: `iotwireless.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (28), `List` (17), `Update` (14), `Delete` (12), `Create` (10), `Associate` (7), `Disassociate` (7), `Start` (6).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateAwsAccountWithPartnerAccount`, `AssociateMulticastGroupWithFuotaTask`, `AssociateWirelessDeviceWithFuotaTask`, `AssociateWirelessDeviceWithMulticastGroup`, `AssociateWirelessDeviceWithThing`, `AssociateWirelessGatewayWithCertificate`, `AssociateWirelessGatewayWithThing`, `CancelMulticastGroupSession`, `CreateDestination`, `CreateDeviceProfile`, `CreateFuotaTask`, `CreateMulticastGroup`, `CreateNetworkAnalyzerConfiguration`, `CreateServiceProfile`, `CreateWirelessDevice`, `CreateWirelessGateway`, `CreateWirelessGatewayTask`, `CreateWirelessGatewayTaskDefinition`, `DeleteDestination`, `DeleteDeviceProfile`, ... (+42).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetDestination`, `GetDeviceProfile`, `GetEventConfigurationByResourceTypes`, `GetFuotaTask`, `GetLogLevelsByResourceTypes`, `GetMetricConfiguration`, `GetMetrics`, `GetMulticastGroup`, `GetMulticastGroupSession`, `GetNetworkAnalyzerConfiguration`, `GetPartnerAccount`, `GetPosition`, `GetPositionConfiguration`, `GetPositionEstimate`, `GetResourceEventConfiguration`, `GetResourceLogLevel`, `GetResourcePosition`, `GetServiceEndpoint`, `GetServiceProfile`, `GetWirelessDevice`, ... (+25).
- Pagination is modelled for 11 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 13 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `AssociateMulticastGroupWithFuotaTask`, `AssociateWirelessDeviceWithFuotaTask`, `CancelMulticastGroupSession`, `CreateFuotaTask`, `CreateWirelessGatewayTask`, `CreateWirelessGatewayTaskDefinition`, `DeleteFuotaTask`, `DeleteWirelessDeviceImportTask`, `DeleteWirelessGatewayTask`, `DeleteWirelessGatewayTaskDefinition`, `DisassociateMulticastGroupFromFuotaTask`, `DisassociateWirelessDeviceFromFuotaTask`, `GetFuotaTask`, `GetWirelessDeviceImportTask`, `GetWirelessGatewayTask`, `GetWirelessGatewayTaskDefinition`, `ListDevicesForWirelessDeviceImportTask`, `ListFuotaTasks`, `ListMulticastGroupsByFuotaTask`, `ListWirelessDeviceImportTasks`, ... (+9).
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 112 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `SNS`, `SQS`, `ECS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetDestination`, `GetDeviceProfile`, `GetEventConfigurationByResourceTypes`, `GetFuotaTask`, `GetLogLevelsByResourceTypes`, `GetMetricConfiguration`, `GetMetrics`, `GetMulticastGroup`, `GetMulticastGroupSession`, `GetNetworkAnalyzerConfiguration`, `GetPartnerAccount`, `GetPosition`, `GetPositionConfiguration`, `GetPositionEstimate`, `GetResourceEventConfiguration`, `GetResourceLogLevel`, `GetResourcePosition`, `GetServiceEndpoint`, `GetServiceProfile`, `GetWirelessDevice`, `GetWirelessDeviceImportTask`, `GetWirelessDeviceStatistics`, `GetWirelessGateway`, `GetWirelessGatewayCertificate`, `GetWirelessGatewayFirmwareInformation`, `GetWirelessGatewayStatistics`, `GetWirelessGatewayTask`, `GetWirelessGatewayTaskDefinition`
- Common required input members in this group: `ConfigurationName`, `Id`, `Identifier`, `IdentifierType`, `Name`, `PartnerAccountId`, `PartnerType`, `ResourceIdentifier`, `ResourceType`, `WirelessDeviceId`, `WirelessGatewayId`

### List

- Operations: `ListDestinations`, `ListDeviceProfiles`, `ListDevicesForWirelessDeviceImportTask`, `ListEventConfigurations`, `ListFuotaTasks`, `ListMulticastGroups`, `ListMulticastGroupsByFuotaTask`, `ListNetworkAnalyzerConfigurations`, `ListPartnerAccounts`, `ListPositionConfigurations`, `ListQueuedMessages`, `ListServiceProfiles`, `ListTagsForResource`, `ListWirelessDeviceImportTasks`, `ListWirelessDevices`, `ListWirelessGatewayTaskDefinitions`, `ListWirelessGateways`
- Traits: `paginated` (11)
- Common required input members in this group: `Id`, `ResourceArn`, `ResourceType`

### Update

- Operations: `UpdateDestination`, `UpdateEventConfigurationByResourceTypes`, `UpdateFuotaTask`, `UpdateLogLevelsByResourceTypes`, `UpdateMetricConfiguration`, `UpdateMulticastGroup`, `UpdateNetworkAnalyzerConfiguration`, `UpdatePartnerAccount`, `UpdatePosition`, `UpdateResourceEventConfiguration`, `UpdateResourcePosition`, `UpdateWirelessDevice`, `UpdateWirelessDeviceImportTask`, `UpdateWirelessGateway`
- Traits: `idempotent` (1)
- Common required input members in this group: `ConfigurationName`, `Id`, `Identifier`, `IdentifierType`, `Name`, `PartnerAccountId`, `PartnerType`, `Position`, `ResourceIdentifier`, `ResourceType`, `Sidewalk`

### Delete

- Operations: `DeleteDestination`, `DeleteDeviceProfile`, `DeleteFuotaTask`, `DeleteMulticastGroup`, `DeleteNetworkAnalyzerConfiguration`, `DeleteQueuedMessages`, `DeleteServiceProfile`, `DeleteWirelessDevice`, `DeleteWirelessDeviceImportTask`, `DeleteWirelessGateway`, `DeleteWirelessGatewayTask`, `DeleteWirelessGatewayTaskDefinition`
- Common required input members in this group: `ConfigurationName`, `Id`, `MessageId`, `Name`

### Create

- Operations: `CreateDestination`, `CreateDeviceProfile`, `CreateFuotaTask`, `CreateMulticastGroup`, `CreateNetworkAnalyzerConfiguration`, `CreateServiceProfile`, `CreateWirelessDevice`, `CreateWirelessGateway`, `CreateWirelessGatewayTask`, `CreateWirelessGatewayTaskDefinition`
- Traits: `idempotency-token` (9)
- Common required input members in this group: `AutoCreateTasks`, `DestinationName`, `Expression`, `ExpressionType`, `FirmwareUpdateImage`, `FirmwareUpdateRole`, `Id`, `LoRaWAN`, `Name`, `RoleArn`, `Type`, `WirelessGatewayTaskDefinitionId`

### Associate

- Operations: `AssociateAwsAccountWithPartnerAccount`, `AssociateMulticastGroupWithFuotaTask`, `AssociateWirelessDeviceWithFuotaTask`, `AssociateWirelessDeviceWithMulticastGroup`, `AssociateWirelessDeviceWithThing`, `AssociateWirelessGatewayWithCertificate`, `AssociateWirelessGatewayWithThing`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Id`, `IotCertificateId`, `MulticastGroupId`, `Sidewalk`, `ThingArn`, `WirelessDeviceId`

### Disassociate

- Operations: `DisassociateAwsAccountFromPartnerAccount`, `DisassociateMulticastGroupFromFuotaTask`, `DisassociateWirelessDeviceFromFuotaTask`, `DisassociateWirelessDeviceFromMulticastGroup`, `DisassociateWirelessDeviceFromThing`, `DisassociateWirelessGatewayFromCertificate`, `DisassociateWirelessGatewayFromThing`
- Common required input members in this group: `Id`, `MulticastGroupId`, `PartnerAccountId`, `PartnerType`, `WirelessDeviceId`

### Start

- Operations: `StartBulkAssociateWirelessDeviceWithMulticastGroup`, `StartBulkDisassociateWirelessDeviceFromMulticastGroup`, `StartFuotaTask`, `StartMulticastGroupSession`, `StartSingleWirelessDeviceImportTask`, `StartWirelessDeviceImportTask`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `DestinationName`, `Id`, `LoRaWAN`, `Sidewalk`

### Put

- Operations: `PutPositionConfiguration`, `PutResourceLogLevel`
- Common required input members in this group: `LogLevel`, `ResourceIdentifier`, `ResourceType`

### Reset

- Operations: `ResetAllResourceLogLevels`, `ResetResourceLogLevel`
- Common required input members in this group: `ResourceIdentifier`, `ResourceType`

### Send

- Operations: `SendDataToMulticastGroup`, `SendDataToWirelessDevice`
- Common required input members in this group: `Id`, `PayloadData`, `TransmitMode`, `WirelessMetadata`

### Cancel

- Operations: `CancelMulticastGroupSession`
- Common required input members in this group: `Id`

### Deregister

- Operations: `DeregisterWirelessDevice`
- Common required input members in this group: `Identifier`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Test

- Operations: `TestWirelessDevice`
- Common required input members in this group: `Id`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateAwsAccountWithPartnerAccount` | `POST /partner-accounts` | `idempotency-token` | `Sidewalk` | `ClientRequestToken` | `AssociateAwsAccountWithPartnerAccountResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates a partner account with your AWS account. |
| `AssociateMulticastGroupWithFuotaTask` | `PUT /fuota-tasks/{Id}/multicast-group` | - | `Id`, `MulticastGroupId` | - | `AssociateMulticastGroupWithFuotaTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associate a multicast group with a FUOTA task. |
| `AssociateWirelessDeviceWithFuotaTask` | `PUT /fuota-tasks/{Id}/wireless-device` | - | `Id`, `WirelessDeviceId` | - | `AssociateWirelessDeviceWithFuotaTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associate a wireless device with a FUOTA task. |
| `AssociateWirelessDeviceWithMulticastGroup` | `PUT /multicast-groups/{Id}/wireless-device` | - | `Id`, `WirelessDeviceId` | - | `AssociateWirelessDeviceWithMulticastGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates a wireless device with a multicast group. |
| `AssociateWirelessDeviceWithThing` | `PUT /wireless-devices/{Id}/thing` | - | `Id`, `ThingArn` | - | `AssociateWirelessDeviceWithThingResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates a wireless device with a thing. |
| `AssociateWirelessGatewayWithCertificate` | `PUT /wireless-gateways/{Id}/certificate` | - | `Id`, `IotCertificateId` | - | `AssociateWirelessGatewayWithCertificateResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates a wireless gateway with a certificate. |
| `AssociateWirelessGatewayWithThing` | `PUT /wireless-gateways/{Id}/thing` | - | `Id`, `ThingArn` | - | `AssociateWirelessGatewayWithThingResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates a wireless gateway with a thing. |
| `CancelMulticastGroupSession` | `DELETE /multicast-groups/{Id}/session` | - | `Id` | - | `CancelMulticastGroupSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels an existing multicast group session. |
| `CreateDestination` | `POST /destinations` | `idempotency-token` | `Expression`, `ExpressionType`, `Name`, `RoleArn` | `ClientRequestToken` | `CreateDestinationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a new destination that maps a device message to an AWS IoT rule. |
| `CreateDeviceProfile` | `POST /device-profiles` | `idempotency-token` | - | `ClientRequestToken` | `CreateDeviceProfileResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a new device profile. |
| `CreateFuotaTask` | `POST /fuota-tasks` | `idempotency-token` | `FirmwareUpdateImage`, `FirmwareUpdateRole` | `ClientRequestToken` | `CreateFuotaTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a FUOTA task. |
| `CreateMulticastGroup` | `POST /multicast-groups` | `idempotency-token` | `LoRaWAN` | `ClientRequestToken` | `CreateMulticastGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a multicast group. |
| `CreateNetworkAnalyzerConfiguration` | `POST /network-analyzer-configurations` | `idempotency-token` | `Name` | `ClientRequestToken` | `CreateNetworkAnalyzerConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a new network analyzer configuration. |
| `CreateServiceProfile` | `POST /service-profiles` | `idempotency-token` | - | `ClientRequestToken` | `CreateServiceProfileResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a new service profile. |
| `CreateWirelessDevice` | `POST /wireless-devices` | `idempotency-token` | `DestinationName`, `Type` | `ClientRequestToken` | `CreateWirelessDeviceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provisions a wireless device. |
| `CreateWirelessGateway` | `POST /wireless-gateways` | `idempotency-token` | `LoRaWAN` | `ClientRequestToken` | `CreateWirelessGatewayResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Provisions a wireless gateway. When provisioning a wireless gateway, you might run into duplication errors for the following reasons. |
| `CreateWirelessGatewayTask` | `POST /wireless-gateways/{Id}/tasks` | - | `Id`, `WirelessGatewayTaskDefinitionId` | - | `CreateWirelessGatewayTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a task for a wireless gateway. |
| `CreateWirelessGatewayTaskDefinition` | `POST /wireless-gateway-task-definitions` | `idempotency-token` | `AutoCreateTasks` | `ClientRequestToken` | `CreateWirelessGatewayTaskDefinitionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a gateway task definition. |
| `DeleteDestination` | `DELETE /destinations/{Name}` | - | `Name` | - | `DeleteDestinationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a destination. |
| `DeleteDeviceProfile` | `DELETE /device-profiles/{Id}` | - | `Id` | - | `DeleteDeviceProfileResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a device profile. |
| `DeleteFuotaTask` | `DELETE /fuota-tasks/{Id}` | - | `Id` | - | `DeleteFuotaTaskResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a FUOTA task. |
| `DeleteMulticastGroup` | `DELETE /multicast-groups/{Id}` | - | `Id` | - | `DeleteMulticastGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a multicast group if it is not in use by a FUOTA task. |
| `DeleteNetworkAnalyzerConfiguration` | `DELETE /network-analyzer-configurations/{ConfigurationName}` | - | `ConfigurationName` | - | `DeleteNetworkAnalyzerConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a network analyzer configuration. |
| `DeleteQueuedMessages` | `DELETE /wireless-devices/{Id}/data` | - | `Id`, `MessageId` | - | `DeleteQueuedMessagesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Remove queued messages from the downlink queue. |
| `DeleteServiceProfile` | `DELETE /service-profiles/{Id}` | - | `Id` | - | `DeleteServiceProfileResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a service profile. |
| `DeleteWirelessDevice` | `DELETE /wireless-devices/{Id}` | - | `Id` | - | `DeleteWirelessDeviceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a wireless device. |
| `DeleteWirelessDeviceImportTask` | `DELETE /wireless_device_import_task/{Id}` | - | `Id` | - | `DeleteWirelessDeviceImportTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete an import task. |
| `DeleteWirelessGateway` | `DELETE /wireless-gateways/{Id}` | - | `Id` | - | `DeleteWirelessGatewayResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a wireless gateway. When deleting a wireless gateway, you might run into duplication errors for the following reasons. |
| `DeleteWirelessGatewayTask` | `DELETE /wireless-gateways/{Id}/tasks` | - | `Id` | - | `DeleteWirelessGatewayTaskResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a wireless gateway task. |
| `DeleteWirelessGatewayTaskDefinition` | `DELETE /wireless-gateway-task-definitions/{Id}` | - | `Id` | - | `DeleteWirelessGatewayTaskDefinitionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a wireless gateway task definition. Deleting this task definition does not affect tasks that are currently in progress. |
| `DeregisterWirelessDevice` | `PATCH /wireless-devices/{Identifier}/deregister` | - | `Identifier` | - | `DeregisterWirelessDeviceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deregister a wireless device from AWS IoT Wireless. |
| `DisassociateAwsAccountFromPartnerAccount` | `DELETE /partner-accounts/{PartnerAccountId}` | - | `PartnerAccountId`, `PartnerType` | - | `DisassociateAwsAccountFromPartnerAccountResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates your AWS account from a partner account. If `PartnerAccountId` and `PartnerType` are `null`, disassociates your AWS account from all partner accounts. |
| `DisassociateMulticastGroupFromFuotaTask` | `DELETE /fuota-tasks/{Id}/multicast-groups/{MulticastGroupId}` | - | `Id`, `MulticastGroupId` | - | `DisassociateMulticastGroupFromFuotaTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Disassociates a multicast group from a FUOTA task. |
| `DisassociateWirelessDeviceFromFuotaTask` | `DELETE /fuota-tasks/{Id}/wireless-devices/{WirelessDeviceId}` | - | `Id`, `WirelessDeviceId` | - | `DisassociateWirelessDeviceFromFuotaTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a wireless device from a FUOTA task. |
| `DisassociateWirelessDeviceFromMulticastGroup` | `DELETE /multicast-groups/{Id}/wireless-devices/{WirelessDeviceId}` | - | `Id`, `WirelessDeviceId` | - | `DisassociateWirelessDeviceFromMulticastGroupResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a wireless device from a multicast group. |
| `DisassociateWirelessDeviceFromThing` | `DELETE /wireless-devices/{Id}/thing` | - | `Id` | - | `DisassociateWirelessDeviceFromThingResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a wireless device from its currently associated thing. |
| `DisassociateWirelessGatewayFromCertificate` | `DELETE /wireless-gateways/{Id}/certificate` | - | `Id` | - | `DisassociateWirelessGatewayFromCertificateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a wireless gateway from its currently associated certificate. |
| `DisassociateWirelessGatewayFromThing` | `DELETE /wireless-gateways/{Id}/thing` | - | `Id` | - | `DisassociateWirelessGatewayFromThingResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates a wireless gateway from its currently associated thing. |
| `GetDestination` | `GET /destinations/{Name}` | - | `Name` | - | `GetDestinationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a destination. |
| `GetDeviceProfile` | `GET /device-profiles/{Id}` | - | `Id` | - | `GetDeviceProfileResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a device profile. |
| `GetEventConfigurationByResourceTypes` | `GET /event-configurations-resource-types` | - | - | - | `GetEventConfigurationByResourceTypesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException` | Get the event configuration based on resource types. |
| `GetFuotaTask` | `GET /fuota-tasks/{Id}` | - | `Id` | - | `GetFuotaTaskResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a FUOTA task. |
| `GetLogLevelsByResourceTypes` | `GET /log-levels` | - | - | - | `GetLogLevelsByResourceTypesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns current default log levels or log levels by resource types. Based on the resource type, log levels can be returned for wireless device, wireless gateway, or FUOTA task log options. |
| `GetMetricConfiguration` | `GET /metric-configuration` | - | - | - | `GetMetricConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the metric configuration status for this AWS account. |
| `GetMetrics` | `POST /metrics` | - | - | - | `GetMetricsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the summary metrics for this AWS account. |
| `GetMulticastGroup` | `GET /multicast-groups/{Id}` | - | `Id` | - | `GetMulticastGroupResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a multicast group. |
| `GetMulticastGroupSession` | `GET /multicast-groups/{Id}/session` | - | `Id` | - | `GetMulticastGroupSessionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a multicast group session. |
| `GetNetworkAnalyzerConfiguration` | `GET /network-analyzer-configurations/{ConfigurationName}` | - | `ConfigurationName` | - | `GetNetworkAnalyzerConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get network analyzer configuration. |
| `GetPartnerAccount` | `GET /partner-accounts/{PartnerAccountId}` | - | `PartnerAccountId`, `PartnerType` | - | `GetPartnerAccountResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a partner account. If `PartnerAccountId` and `PartnerType` are `null`, returns all partner accounts. |
| `GetPosition` | `GET /positions/{ResourceIdentifier}` | - | `ResourceIdentifier`, `ResourceType` | - | `GetPositionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the position information for a given resource. This action is no longer supported. |
| `GetPositionConfiguration` | `GET /position-configurations/{ResourceIdentifier}` | - | `ResourceIdentifier`, `ResourceType` | - | `GetPositionConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get position configuration for a given resource. This action is no longer supported. |
| `GetPositionEstimate` | `POST /position-estimate` | - | - | - | `GetPositionEstimateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get estimated position information as a payload in GeoJSON format. The payload measurement data is resolved using solvers that are provided by third-party vendors. |
| `GetResourceEventConfiguration` | `GET /event-configurations/{Identifier}` | - | `Identifier`, `IdentifierType` | - | `GetResourceEventConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the event configuration for a particular resource identifier. |
| `GetResourceLogLevel` | `GET /log-levels/{ResourceIdentifier}` | - | `ResourceIdentifier`, `ResourceType` | - | `GetResourceLogLevelResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Fetches the log-level override, if any, for a given resource ID and resource type.. |
| `GetResourcePosition` | `GET /resource-positions/{ResourceIdentifier}` | - | `ResourceIdentifier`, `ResourceType` | - | `GetResourcePositionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the position information for a given wireless device or a wireless gateway resource. The position information uses the World Geodetic System (WGS84). |
| `GetServiceEndpoint` | `GET /service-endpoint` | - | - | - | `GetServiceEndpointResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Gets the account-specific endpoint for Configuration and Update Server (CUPS) protocol or LoRaWAN Network Server (LNS) connections. |
| `GetServiceProfile` | `GET /service-profiles/{Id}` | - | `Id` | - | `GetServiceProfileResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a service profile. |
| `GetWirelessDevice` | `GET /wireless-devices/{Identifier}` | - | `Identifier`, `IdentifierType` | - | `GetWirelessDeviceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a wireless device. |
| `GetWirelessDeviceImportTask` | `GET /wireless_device_import_task/{Id}` | - | `Id` | - | `GetWirelessDeviceImportTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get information about an import task and count of device onboarding summary information for the import task. |
| `GetWirelessDeviceStatistics` | `GET /wireless-devices/{WirelessDeviceId}/statistics` | - | `WirelessDeviceId` | - | `GetWirelessDeviceStatisticsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets operating information about a wireless device. |
| `GetWirelessGateway` | `GET /wireless-gateways/{Identifier}` | - | `Identifier`, `IdentifierType` | - | `GetWirelessGatewayResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a wireless gateway. |
| `GetWirelessGatewayCertificate` | `GET /wireless-gateways/{Id}/certificate` | - | `Id` | - | `GetWirelessGatewayCertificateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the ID of the certificate that is currently associated with a wireless gateway. |
| `GetWirelessGatewayFirmwareInformation` | `GET /wireless-gateways/{Id}/firmware-information` | - | `Id` | - | `GetWirelessGatewayFirmwareInformationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the firmware version and other information about a wireless gateway. |
| `GetWirelessGatewayStatistics` | `GET /wireless-gateways/{WirelessGatewayId}/statistics` | - | `WirelessGatewayId` | - | `GetWirelessGatewayStatisticsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets operating information about a wireless gateway. |
| `GetWirelessGatewayTask` | `GET /wireless-gateways/{Id}/tasks` | - | `Id` | - | `GetWirelessGatewayTaskResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a wireless gateway task. |
| `GetWirelessGatewayTaskDefinition` | `GET /wireless-gateway-task-definitions/{Id}` | - | `Id` | - | `GetWirelessGatewayTaskDefinitionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets information about a wireless gateway task definition. |
| `ListDestinations` | `GET /destinations` | `paginated` | - | - | `ListDestinationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the destinations registered to your AWS account. |
| `ListDeviceProfiles` | `GET /device-profiles` | `paginated` | - | - | `ListDeviceProfilesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the device profiles registered to your AWS account. |
| `ListDevicesForWirelessDeviceImportTask` | `GET /wireless_device_import_task` | - | `Id` | - | `ListDevicesForWirelessDeviceImportTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List the Sidewalk devices in an import task and their onboarding status. |
| `ListEventConfigurations` | `GET /event-configurations` | - | `ResourceType` | - | `ListEventConfigurationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List event configurations where at least one event topic has been enabled. |
| `ListFuotaTasks` | `GET /fuota-tasks` | `paginated` | - | - | `ListFuotaTasksResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the FUOTA tasks registered to your AWS account. |
| `ListMulticastGroups` | `GET /multicast-groups` | `paginated` | - | - | `ListMulticastGroupsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the multicast groups registered to your AWS account. |
| `ListMulticastGroupsByFuotaTask` | `GET /fuota-tasks/{Id}/multicast-groups` | `paginated` | `Id` | - | `ListMulticastGroupsByFuotaTaskResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List all multicast groups associated with a FUOTA task. |
| `ListNetworkAnalyzerConfigurations` | `GET /network-analyzer-configurations` | `paginated` | - | - | `ListNetworkAnalyzerConfigurationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the network analyzer configurations. |
| `ListPartnerAccounts` | `GET /partner-accounts` | - | - | - | `ListPartnerAccountsResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the partner accounts associated with your AWS account. |
| `ListPositionConfigurations` | `GET /position-configurations` | `paginated` | - | - | `ListPositionConfigurationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List position configurations for a given resource, such as positioning solvers. This action is no longer supported. |
| `ListQueuedMessages` | `GET /wireless-devices/{Id}/data` | `paginated` | `Id` | - | `ListQueuedMessagesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List queued messages in the downlink queue. |
| `ListServiceProfiles` | `GET /service-profiles` | `paginated` | - | - | `ListServiceProfilesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the service profiles registered to your AWS account. |
| `ListTagsForResource` | `GET /tags` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags (metadata) you have assigned to the resource. |
| `ListWirelessDeviceImportTasks` | `GET /wireless_device_import_tasks` | - | - | - | `ListWirelessDeviceImportTasksResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List of import tasks and summary information of onboarding status of devices in each import task. |
| `ListWirelessDevices` | `GET /wireless-devices` | `paginated` | - | - | `ListWirelessDevicesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the wireless devices registered to your AWS account. |
| `ListWirelessGatewayTaskDefinitions` | `GET /wireless-gateway-task-definitions` | - | - | - | `ListWirelessGatewayTaskDefinitionsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List the wireless gateway tasks definitions registered to your AWS account. |
| `ListWirelessGateways` | `GET /wireless-gateways` | `paginated` | - | - | `ListWirelessGatewaysResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the wireless gateways registered to your AWS account. |
| `PutPositionConfiguration` | `PUT /position-configurations/{ResourceIdentifier}` | - | `ResourceIdentifier`, `ResourceType` | - | `PutPositionConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Put position configuration for a given resource. This action is no longer supported. |
| `PutResourceLogLevel` | `PUT /log-levels/{ResourceIdentifier}` | - | `LogLevel`, `ResourceIdentifier`, `ResourceType` | - | `PutResourceLogLevelResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sets the log-level override for a resource ID and resource type. A limit of 200 log level override can be set per account. |
| `ResetAllResourceLogLevels` | `DELETE /log-levels` | - | - | - | `ResetAllResourceLogLevelsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the log-level overrides for all resources; wireless devices, wireless gateways, and FUOTA tasks. |
| `ResetResourceLogLevel` | `DELETE /log-levels/{ResourceIdentifier}` | - | `ResourceIdentifier`, `ResourceType` | - | `ResetResourceLogLevelResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the log-level override, if any, for a specific resource ID and resource type. It can be used for a wireless device, a wireless gateway, or a FUOTA task. |
| `SendDataToMulticastGroup` | `POST /multicast-groups/{Id}/data` | - | `Id`, `PayloadData`, `WirelessMetadata` | - | `SendDataToMulticastGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sends the specified data to a multicast group. |
| `SendDataToWirelessDevice` | `POST /wireless-devices/{Id}/data` | - | `Id`, `PayloadData`, `TransmitMode` | - | `SendDataToWirelessDeviceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Sends a decrypted application data frame to a device. |
| `StartBulkAssociateWirelessDeviceWithMulticastGroup` | `PATCH /multicast-groups/{Id}/bulk` | - | `Id` | - | `StartBulkAssociateWirelessDeviceWithMulticastGroupResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts a bulk association of all qualifying wireless devices with a multicast group. |
| `StartBulkDisassociateWirelessDeviceFromMulticastGroup` | `POST /multicast-groups/{Id}/bulk` | - | `Id` | - | `StartBulkDisassociateWirelessDeviceFromMulticastGroupResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts a bulk disassociatin of all qualifying wireless devices from a multicast group. |
| `StartFuotaTask` | `PUT /fuota-tasks/{Id}` | - | `Id` | - | `StartFuotaTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts a FUOTA task. |
| `StartMulticastGroupSession` | `PUT /multicast-groups/{Id}/session` | - | `Id`, `LoRaWAN` | - | `StartMulticastGroupSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts a multicast group session. |
| `StartSingleWirelessDeviceImportTask` | `POST /wireless_single_device_import_task` | `idempotency-token` | `DestinationName`, `Sidewalk` | `ClientRequestToken` | `StartSingleWirelessDeviceImportTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Start import task for a single wireless device. |
| `StartWirelessDeviceImportTask` | `POST /wireless_device_import_task` | `idempotency-token` | `DestinationName`, `Sidewalk` | `ClientRequestToken` | `StartWirelessDeviceImportTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Start import task for provisioning Sidewalk devices in bulk using an S3 CSV file. |
| `TagResource` | `POST /tags` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `TooManyTagsException`, `ValidationException` | Adds a tag to a resource. |
| `TestWirelessDevice` | `POST /wireless-devices/{Id}/test` | - | `Id` | - | `TestWirelessDeviceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Simulates a provisioned device by sending an uplink data payload of `Hello`. |
| `UntagResource` | `DELETE /tags` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes one or more tags from a resource. |
| `UpdateDestination` | `PATCH /destinations/{Name}` | - | `Name` | - | `UpdateDestinationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates properties of a destination. |
| `UpdateEventConfigurationByResourceTypes` | `PATCH /event-configurations-resource-types` | - | - | - | `UpdateEventConfigurationByResourceTypesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Update the event configuration based on resource types. |
| `UpdateFuotaTask` | `PATCH /fuota-tasks/{Id}` | - | `Id` | - | `UpdateFuotaTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates properties of a FUOTA task. |
| `UpdateLogLevelsByResourceTypes` | `POST /log-levels` | - | - | - | `UpdateLogLevelsByResourceTypesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Set default log level, or log levels by resource types. This can be for wireless device, wireless gateway, or FUOTA task log options, and is used to control the log messages that'll be displayed in CloudWatch. |
| `UpdateMetricConfiguration` | `PUT /metric-configuration` | `idempotent` | - | - | `UpdateMetricConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update the summary metric configuration. |
| `UpdateMulticastGroup` | `PATCH /multicast-groups/{Id}` | - | `Id` | - | `UpdateMulticastGroupResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates properties of a multicast group session. |
| `UpdateNetworkAnalyzerConfiguration` | `PATCH /network-analyzer-configurations/{ConfigurationName}` | - | `ConfigurationName` | - | `UpdateNetworkAnalyzerConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update network analyzer configuration. |
| `UpdatePartnerAccount` | `PATCH /partner-accounts/{PartnerAccountId}` | - | `PartnerAccountId`, `PartnerType`, `Sidewalk` | - | `UpdatePartnerAccountResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates properties of a partner account. |
| `UpdatePosition` | `PATCH /positions/{ResourceIdentifier}` | - | `Position`, `ResourceIdentifier`, `ResourceType` | - | `UpdatePositionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update the position information of a resource. This action is no longer supported. |
| `UpdateResourceEventConfiguration` | `PATCH /event-configurations/{Identifier}` | - | `Identifier`, `IdentifierType` | - | `UpdateResourceEventConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update the event configuration for a particular resource identifier. |
| `UpdateResourcePosition` | `PATCH /resource-positions/{ResourceIdentifier}` | - | `ResourceIdentifier`, `ResourceType` | - | `UpdateResourcePositionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update the position information of a given wireless device or a wireless gateway resource. The position coordinates are based on the World Geodetic System (WGS84). |
| `UpdateWirelessDevice` | `PATCH /wireless-devices/{Id}` | - | `Id` | - | `UpdateWirelessDeviceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates properties of a wireless device. |
| `UpdateWirelessDeviceImportTask` | `PATCH /wireless_device_import_task/{Id}` | - | `Id`, `Sidewalk` | - | `UpdateWirelessDeviceImportTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update an import task to add more devices to the task. |
| `UpdateWirelessGateway` | `PATCH /wireless-gateways/{Id}` | - | `Id` | - | `UpdateWirelessGatewayResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates properties of a wireless gateway. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `DeleteQueuedMessages` | - | `MessageId -> messageId`, `WirelessDeviceType -> WirelessDeviceType` | - | - |
| `DeregisterWirelessDevice` | - | `WirelessDeviceType -> WirelessDeviceType` | - | - |
| `DisassociateAwsAccountFromPartnerAccount` | - | `PartnerType -> partnerType` | - | - |
| `GetPartnerAccount` | - | `PartnerType -> partnerType` | - | - |
| `GetPosition` | - | `ResourceType -> resourceType` | - | - |
| `GetPositionConfiguration` | - | `ResourceType -> resourceType` | - | - |
| `GetResourceEventConfiguration` | - | `IdentifierType -> identifierType`, `PartnerType -> partnerType` | - | - |
| `GetResourceLogLevel` | - | `ResourceType -> resourceType` | - | - |
| `GetResourcePosition` | - | `ResourceType -> resourceType` | - | - |
| `GetServiceEndpoint` | - | `ServiceType -> serviceType` | - | - |
| `GetWirelessDevice` | - | `IdentifierType -> identifierType` | - | - |
| `GetWirelessGateway` | - | `IdentifierType -> identifierType` | - | - |
| `ListDestinations` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListDeviceProfiles` | - | `NextToken -> nextToken`, `MaxResults -> maxResults`, `DeviceProfileType -> deviceProfileType` | - | - |
| `ListDevicesForWirelessDeviceImportTask` | - | `Id -> id`, `MaxResults -> maxResults`, `NextToken -> nextToken`, `Status -> status` | - | - |
| `ListEventConfigurations` | - | `ResourceType -> resourceType`, `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListFuotaTasks` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListMulticastGroups` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListMulticastGroupsByFuotaTask` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListNetworkAnalyzerConfigurations` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListPartnerAccounts` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListPositionConfigurations` | - | `ResourceType -> resourceType`, `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListQueuedMessages` | - | `NextToken -> nextToken`, `MaxResults -> maxResults`, `WirelessDeviceType -> WirelessDeviceType` | - | - |
| `ListServiceProfiles` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListTagsForResource` | - | `ResourceArn -> resourceArn` | - | - |
| `ListWirelessDeviceImportTasks` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `ListWirelessDevices` | - | `MaxResults -> maxResults`, `NextToken -> nextToken`, `DestinationName -> destinationName`, `DeviceProfileId -> deviceProfileId`, `ServiceProfileId -> serviceProfileId`, `WirelessDeviceType -> wirelessDeviceType`, `FuotaTaskId -> fuotaTaskId`, `MulticastGroupId -> multicastGroupId` | - | - |
| `ListWirelessGateways` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListWirelessGatewayTaskDefinitions` | - | `MaxResults -> maxResults`, `NextToken -> nextToken`, `TaskDefinitionType -> taskDefinitionType` | - | - |
| `PutPositionConfiguration` | - | `ResourceType -> resourceType` | - | - |
| `PutResourceLogLevel` | - | `ResourceType -> resourceType` | - | - |
| `ResetResourceLogLevel` | - | `ResourceType -> resourceType` | - | - |
| `TagResource` | - | `ResourceArn -> resourceArn` | - | - |
| `UntagResource` | - | `ResourceArn -> resourceArn`, `TagKeys -> tagKeys` | - | - |
| `UpdatePartnerAccount` | - | `PartnerType -> partnerType` | - | - |
| `UpdatePosition` | - | `ResourceType -> resourceType` | - | - |
| `UpdateResourceEventConfiguration` | - | `IdentifierType -> identifierType`, `PartnerType -> partnerType` | - | - |
| `UpdateResourcePosition` | - | `ResourceType -> resourceType` | - | `GeoJsonPayload` |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InternalServerException` | `structure` | `Message` | An unexpected error occurred while processing a request. |
| `ThrottlingException` | `structure` | `Message` | The request was denied because it exceeded the allowed API request rate. |
| `ValidationException` | `structure` | `Message` | The input did not meet the specified constraints. |
| `AccessDeniedException` | `structure` | `Message` | User does not have permission to perform this action. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId`, `ResourceType` | Resource does not exist. |
| `ConflictException` | `structure` | `Message`, `ResourceId`, `ResourceType` | Adding, updating, or deleting the resource can cause an inconsistent state. |
| `AssociateAwsAccountWithPartnerAccountRequest` | `structure` | `ClientRequestToken`, `Sidewalk`, `Tags` | - |
| `AssociateAwsAccountWithPartnerAccountResponse` | `structure` | `Arn`, `Sidewalk` | - |
| `AssociateMulticastGroupWithFuotaTaskRequest` | `structure` | `Id`, `MulticastGroupId` | - |
| `AssociateMulticastGroupWithFuotaTaskResponse` | `structure` | - | - |
| `AssociateWirelessDeviceWithFuotaTaskRequest` | `structure` | `Id`, `WirelessDeviceId` | - |
| `AssociateWirelessDeviceWithFuotaTaskResponse` | `structure` | - | - |
| `AssociateWirelessDeviceWithMulticastGroupRequest` | `structure` | `Id`, `WirelessDeviceId` | - |
| `AssociateWirelessDeviceWithMulticastGroupResponse` | `structure` | - | - |
| `AssociateWirelessDeviceWithThingRequest` | `structure` | `Id`, `ThingArn` | - |
| `AssociateWirelessDeviceWithThingResponse` | `structure` | - | - |
| `AssociateWirelessGatewayWithCertificateRequest` | `structure` | `Id`, `IotCertificateId` | - |
| `AssociateWirelessGatewayWithCertificateResponse` | `structure` | `IotCertificateId` | - |
| `AssociateWirelessGatewayWithThingRequest` | `structure` | `Id`, `ThingArn` | - |
| `AssociateWirelessGatewayWithThingResponse` | `structure` | - | - |
| `CancelMulticastGroupSessionRequest` | `structure` | `Id` | - |
| `CancelMulticastGroupSessionResponse` | `structure` | - | - |
| `CreateDestinationRequest` | `structure` | `ClientRequestToken`, `Description`, `Expression`, `ExpressionType`, `Name`, `RoleArn`, `Tags` | - |
| `CreateDestinationResponse` | `structure` | `Arn`, `Name` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
