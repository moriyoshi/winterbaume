# Managed integrations for AWS IoT Device Management

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Managed integrations is a feature of AWS IoT Device Management that enables developers to quickly build innovative IoT solutions. Customers can use managed integrations to automate device setup workflows and support interoperability across many devices, regardless of device vendor or connectivity protocol. This allows developers to use a single user-interface to control, manage, and operate a range of devices.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Managed integrations for AWS IoT Device Management where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Managed integrations for AWS IoT Device Management by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Managed integrations for AWS IoT Device Management by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Managed integrations for AWS IoT Device Management workflows in the local mock. Key resources include `AccountAssociationResource`, `CloudConnectorResource`, `ConnectorDestinationResource`, `CredentialLockerResource`, `DestinationResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Update` operation families, including `GetAccountAssociation`, `GetCloudConnector`, `GetConnectorDestination`, `GetCredentialLocker`, `ListAccountAssociations`, `ListCloudConnectors`.

## Service Identity and Protocol

- AWS model slug: `iot-managed-integrations`
- AWS SDK for Rust slug: `iotmanagedintegrations`
- Model version: `2025-03-03`
- Model file: `vendor/api-models-aws/models/iot-managed-integrations/service/2025-03-03/iot-managed-integrations-2025-03-03.json`
- SDK ID: `IoT Managed Integrations`
- Endpoint prefix: `api.iotmanagedintegrations`
- ARN namespace: `iotmanagedintegrations`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (22), `List` (18), `Create` (11), `Delete` (11), `Update` (8), `Put` (3), `Register` (2), `Send` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateAccountAssociation`, `CreateCloudConnector`, `CreateConnectorDestination`, `CreateCredentialLocker`, `CreateDestination`, `CreateEventLogConfiguration`, `CreateManagedThing`, `CreateNotificationConfiguration`, `CreateOtaTask`, `CreateOtaTaskConfiguration`, `CreateProvisioningProfile`, `DeleteAccountAssociation`, `DeleteCloudConnector`, `DeleteConnectorDestination`, `DeleteCredentialLocker`, `DeleteDestination`, `DeleteEventLogConfiguration`, `DeleteManagedThing`, `DeleteNotificationConfiguration`, `DeleteOtaTask`, ... (+20).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccountAssociation`, `GetCloudConnector`, `GetConnectorDestination`, `GetCredentialLocker`, `GetCustomEndpoint`, `GetDefaultEncryptionConfiguration`, `GetDestination`, `GetDeviceDiscovery`, `GetEventLogConfiguration`, `GetHubConfiguration`, `GetManagedThing`, `GetManagedThingCapabilities`, `GetManagedThingCertificate`, `GetManagedThingConnectivityData`, `GetManagedThingMetaData`, `GetManagedThingState`, `GetNotificationConfiguration`, `GetOtaTask`, `GetOtaTaskConfiguration`, `GetProvisioningProfile`, ... (+20).
- Pagination is modelled for 17 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 38 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateOtaTask`, `CreateOtaTaskConfiguration`, `DeleteOtaTask`, `DeleteOtaTaskConfiguration`, `GetOtaTask`, `GetOtaTaskConfiguration`, `ListOtaTaskConfigurations`, `ListOtaTaskExecutions`, `ListOtaTasks`, `StartAccountAssociationRefresh`, `StartDeviceDiscovery`, `UpdateOtaTask`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 83 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `Lambda`, `ECS`, `Secrets Manager`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `AccountAssociationResource` | `AccountAssociationId` | create: `CreateAccountAssociation`; read: `GetAccountAssociation`; update: `UpdateAccountAssociation`; delete: `DeleteAccountAssociation`; list: `ListAccountAssociations` | `StartAccountAssociationRefresh` | - |
| `CloudConnectorResource` | `Identifier` | create: `CreateCloudConnector`; read: `GetCloudConnector`; update: `UpdateCloudConnector`; delete: `DeleteCloudConnector`; list: `ListCloudConnectors` | - | - |
| `ConnectorDestinationResource` | `Identifier` | create: `CreateConnectorDestination`; read: `GetConnectorDestination`; update: `UpdateConnectorDestination`; delete: `DeleteConnectorDestination`; list: `ListConnectorDestinations` | - | - |
| `CredentialLockerResource` | `Identifier` | create: `CreateCredentialLocker`; read: `GetCredentialLocker`; delete: `DeleteCredentialLocker`; list: `ListCredentialLockers` | - | - |
| `DestinationResource` | - | - | `CreateDestination`, `DeleteDestination`, `GetDestination`, `ListDestinations`, `UpdateDestination` | - |
| `DeviceDiscoveryResource` | `Identifier` | create: `StartDeviceDiscovery`; read: `GetDeviceDiscovery`; list: `ListDeviceDiscoveries` | `ListDiscoveredDevices` | - |
| `EventLogConfigurationResource` | - | - | `CreateEventLogConfiguration`, `DeleteEventLogConfiguration`, `GetEventLogConfiguration`, `ListEventLogConfigurations`, `UpdateEventLogConfiguration` | - |
| `HubConfigurationResource` | - | - | `GetHubConfiguration`, `PutHubConfiguration` | - |
| `KmsKeyAssociationResource` | - | - | `GetDefaultEncryptionConfiguration`, `PutDefaultEncryptionConfiguration` | - |
| `ManagedThingAssociationResource` | - | - | `DeregisterAccountAssociation`, `ListManagedThingAccountAssociations`, `RegisterAccountAssociation` | - |
| `ManagedThingCommandResource` | `ManagedThingId` | - | `SendManagedThingCommand` | - |
| `ManagedThingResource` | `Identifier` | create: `CreateManagedThing`; read: `GetManagedThing`; update: `UpdateManagedThing`; delete: `DeleteManagedThing`; list: `ListManagedThings` | `GetManagedThingCapabilities`, `GetManagedThingCertificate`, `GetManagedThingConnectivityData`, `GetManagedThingMetaData`, `ListManagedThingSchemas` | - |
| `ManagedThingStateResource` | `ManagedThingId` | - | `GetManagedThingState` | - |
| `NotificationConfigurationResource` | - | - | `CreateNotificationConfiguration`, `DeleteNotificationConfiguration`, `GetNotificationConfiguration`, `ListNotificationConfigurations`, `UpdateNotificationConfiguration` | - |
| `OtaTaskConfigurationResource` | - | - | `CreateOtaTaskConfiguration`, `DeleteOtaTaskConfiguration`, `GetOtaTaskConfiguration`, `ListOtaTaskConfigurations` | - |
| `OtaTaskResource` | `Identifier` | create: `CreateOtaTask`; read: `GetOtaTask`; update: `UpdateOtaTask`; delete: `DeleteOtaTask`; list: `ListOtaTasks` | `ListOtaTaskExecutions` | - |
| `ProvisioningProfileResource` | `Identifier` | create: `CreateProvisioningProfile`; read: `GetProvisioningProfile`; delete: `DeleteProvisioningProfile`; list: `ListProvisioningProfiles` | - | - |
| `RuntimeLogConfigurationResource` | - | - | `GetRuntimeLogConfiguration`, `PutRuntimeLogConfiguration`, `ResetRuntimeLogConfiguration` | - |
| `SchemaVersionResource` | `SchemaVersionedId`, `Type` | read: `GetSchemaVersion`; list: `ListSchemaVersions` | - | - |
## Operation Groups

### Get

- Operations: `GetCustomEndpoint`
- Traits: `readonly` (1)
- Common required input members in this group: -

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Register

- Operations: `RegisterCustomEndpoint`
- Common required input members in this group: -

### Send

- Operations: `SendConnectorEvent`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetCustomEndpoint` | `GET /custom-endpoint` | `readonly` | - | - | `GetCustomEndpointResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Returns the IoT managed integrations custom endpoint. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException` | Lists the tags for a specified resource. |
| `RegisterCustomEndpoint` | `POST /custom-endpoint` | - | - | - | `RegisterCustomEndpointResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Customers can request IoT managed integrations to manage the server trust for them or bring their own external server trusts for the custom domain. Returns an IoT managed integrations endpoint. |
| `SendConnectorEvent` | `POST /connector-event/{ConnectorId}` | `idempotent` | `ConnectorId`, `Operation` | - | `SendConnectorEventResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Relays third-party device events for a connector such as a new device or a device state change event. |
| `TagResource` | `POST /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `ConflictException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException` | Adds tags to a specified resource. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `ConflictException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException` | Removes tags from a specified resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | User is not authorized. |
| `ConflictException` | `structure` | Message | There is a conflict with the request. |
| `InternalFailureException` | `structure` | Message | An unexpected error has occurred. |
| `InternalServerException` | `structure` | Message | Internal error from the service that indicates an unexpected error or that the service is unavailable. |
| `InvalidRequestException` | `structure` | Message | The request is not valid. |
| `LimitExceededException` | `structure` | Message | The request exceeds a service limit or quota. Adjust your request parameters and try again. |
| `ResourceNotFoundException` | `structure` | Message, ResourceId, ResourceType | The specified resource does not exist. |
| `ServiceQuotaExceededException` | `structure` | Message | The service quota has been exceeded for this request. |
| `ServiceUnavailableException` | `structure` | Message | The service is temporarily unavailable. |
| `ThrottlingException` | `structure` | Message | The rate exceeds the limit. |
| `UnauthorizedException` | `structure` | Message | You are not authorized to perform this operation. |
| `ValidationException` | `structure` | Message | A validation error occurred when performing the API request. |
| `GetCustomEndpointRequest` | `structure` | **empty (no members)** | - |
| `GetCustomEndpointResponse` | `structure` | EndpointAddress | - |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | tags | - |
| `RegisterCustomEndpointRequest` | `structure` | **empty (no members)** | - |
| `RegisterCustomEndpointResponse` | `structure` | EndpointAddress | - |
| `SendConnectorEventRequest` | `structure` | ConnectorId, UserId, Operation, OperationVersion, StatusCode, Message, DeviceDiscoveryId, ConnectorDeviceId, TraceId, Devices, MatterEndpoint | - |
| `SendConnectorEventResponse` | `structure` | ConnectorId | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `AbortCriteriaAction` | `enum` | CANCEL | - |
| `AbortCriteriaFailureType` | `enum` | FAILED, REJECTED, TIMED_OUT, ALL | - |
| `AssociationState` | `enum` | ASSOCIATION_IN_PROGRESS, ASSOCIATION_FAILED, ASSOCIATION_SUCCEEDED, ASSOCIATION_DELETING, REFRESH_TOKEN_EXPIRED | - |
| `AuthMaterialType` | `enum` | CUSTOM_PROTOCOL_QR_BAR_CODE, WIFI_SETUP_QR_BAR_CODE, ZWAVE_QR_BAR_CODE, ZIGBEE_QR_BAR_CODE, DISCOVERED_DEVICE, PRE_ONBOARDED_CLOUD | - |
| `AuthType` | `enum` | OAUTH | - |
| `CloudConnectorType` | `enum` | LISTED, UNLISTED | - |
| `ConfigurationState` | `enum` | ENABLED, UPDATE_IN_PROGRESS, UPDATE_FAILED | - |
| `ConnectorEventOperation` | `enum` | DEVICE_COMMAND_RESPONSE, DEVICE_DISCOVERY, DEVICE_EVENT, DEVICE_COMMAND_REQUEST | - |
| `DeliveryDestinationType` | `enum` | KINESIS | - |
| `DeviceDiscoveryStatus` | `enum` | RUNNING, SUCCEEDED, FAILED, TIMED_OUT | - |
| `DisconnectReasonValue` | `enum` | AUTH_ERROR, CLIENT_INITIATED_DISCONNECT, CLIENT_ERROR, CONNECTION_LOST, DUPLICATE_CLIENTID, FORBIDDEN_ACCESS, MQTT_KEEP_ALIVE_TIMEOUT, SERVER_ERROR, SERVER_INITIATED_DISCONNECT, THROTTLED, WEBSOCKET_TTL_EXPIRATION, CUSTOMAUTH_TTL_EXPIRATION, ... (+2) | - |
| `DiscoveryAuthMaterialType` | `enum` | ZWAVE_INSTALL_CODE | - |
| `DiscoveryModification` | `enum` | DISCOVERED, UPDATED, NO_CHANGE | - |
| `DiscoveryType` | `enum` | ZWAVE, ZIGBEE, CLOUD, CUSTOM, CONTROLLER_CAPABILITY_REDISCOVERY | - |
| `EncryptionType` | `enum` | MANAGED_INTEGRATIONS_DEFAULT_ENCRYPTION, CUSTOMER_KEY_ENCRYPTION | - |
| `EndpointType` | `enum` | LAMBDA | - |
| `EventType` | `enum` | DEVICE_COMMAND, DEVICE_COMMAND_REQUEST, DEVICE_DISCOVERY_STATUS, DEVICE_EVENT, DEVICE_LIFE_CYCLE, DEVICE_STATE, DEVICE_OTA, DEVICE_WSS, CONNECTOR_ASSOCIATION, ACCOUNT_ASSOCIATION, CONNECTOR_ERROR_REPORT | - |
| `HubNetworkMode` | `enum` | STANDARD, NETWORK_WIDE_EXCLUSION | - |
| `LogLevel` | `enum` | DEBUG, ERROR, INFO, WARN | - |
| `ManagedThingAssociationStatus` | `enum` | PRE_ASSOCIATED, ASSOCIATED | - |
| `OtaMechanism` | `enum` | PUSH | - |
| `OtaProtocol` | `enum` | HTTP | - |
| `OtaStatus` | `enum` | IN_PROGRESS, CANCELED, COMPLETED, DELETION_IN_PROGRESS, SCHEDULED | - |
| `OtaTaskExecutionStatus` | `enum` | QUEUED, IN_PROGRESS, SUCCEEDED, FAILED, TIMED_OUT, REJECTED, REMOVED, CANCELED | - |
| `OtaType` | `enum` | ONE_TIME, CONTINUOUS | - |
| `ProtocolType` | `enum` | ZWAVE, ZIGBEE, CUSTOM | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
