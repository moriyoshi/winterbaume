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

- Operations: `GetAccountAssociation`, `GetCloudConnector`, `GetConnectorDestination`, `GetCredentialLocker`, `GetCustomEndpoint`, `GetDefaultEncryptionConfiguration`, `GetDestination`, `GetDeviceDiscovery`, `GetEventLogConfiguration`, `GetHubConfiguration`, `GetManagedThing`, `GetManagedThingCapabilities`, `GetManagedThingCertificate`, `GetManagedThingConnectivityData`, `GetManagedThingMetaData`, `GetManagedThingState`, `GetNotificationConfiguration`, `GetOtaTask`, `GetOtaTaskConfiguration`, `GetProvisioningProfile`, `GetRuntimeLogConfiguration`, `GetSchemaVersion`
- Traits: `readonly` (22)
- Common required input members in this group: `AccountAssociationId`, `EventType`, `Id`, `Identifier`, `ManagedThingId`, `Name`, `SchemaVersionedId`, `Type`

### List

- Operations: `ListAccountAssociations`, `ListCloudConnectors`, `ListConnectorDestinations`, `ListCredentialLockers`, `ListDestinations`, `ListDeviceDiscoveries`, `ListDiscoveredDevices`, `ListEventLogConfigurations`, `ListManagedThingAccountAssociations`, `ListManagedThingSchemas`, `ListManagedThings`, `ListNotificationConfigurations`, `ListOtaTaskConfigurations`, `ListOtaTaskExecutions`, `ListOtaTasks`, `ListProvisioningProfiles`, `ListSchemaVersions`, `ListTagsForResource`
- Traits: `paginated` (17), `readonly` (18)
- Common required input members in this group: `Identifier`, `ResourceArn`, `Type`

### Create

- Operations: `CreateAccountAssociation`, `CreateCloudConnector`, `CreateConnectorDestination`, `CreateCredentialLocker`, `CreateDestination`, `CreateEventLogConfiguration`, `CreateManagedThing`, `CreateNotificationConfiguration`, `CreateOtaTask`, `CreateOtaTaskConfiguration`, `CreateProvisioningProfile`
- Traits: `idempotency-token` (11)
- Common required input members in this group: `AuthConfig`, `AuthenticationMaterial`, `AuthenticationMaterialType`, `CloudConnectorId`, `ConnectorDestinationId`, `DeliveryDestinationArn`, `DeliveryDestinationType`, `DestinationName`, `EndpointConfig`, `EventLogLevel`, `EventType`, `Name`, `OtaType`, `ProvisioningType`, `ResourceType`, `Role`, `RoleArn`, `S3Url`

### Delete

- Operations: `DeleteAccountAssociation`, `DeleteCloudConnector`, `DeleteConnectorDestination`, `DeleteCredentialLocker`, `DeleteDestination`, `DeleteEventLogConfiguration`, `DeleteManagedThing`, `DeleteNotificationConfiguration`, `DeleteOtaTask`, `DeleteOtaTaskConfiguration`, `DeleteProvisioningProfile`
- Traits: `idempotent` (11)
- Common required input members in this group: `AccountAssociationId`, `EventType`, `Id`, `Identifier`, `Name`

### Update

- Operations: `UpdateAccountAssociation`, `UpdateCloudConnector`, `UpdateConnectorDestination`, `UpdateDestination`, `UpdateEventLogConfiguration`, `UpdateManagedThing`, `UpdateNotificationConfiguration`, `UpdateOtaTask`
- Traits: `idempotent` (8)
- Common required input members in this group: `AccountAssociationId`, `DestinationName`, `EventLogLevel`, `EventType`, `Id`, `Identifier`, `Name`

### Put

- Operations: `PutDefaultEncryptionConfiguration`, `PutHubConfiguration`, `PutRuntimeLogConfiguration`
- Traits: `idempotent` (2)
- Common required input members in this group: `HubTokenTimerExpirySettingInSeconds`, `ManagedThingId`, `RuntimeLogConfigurations`, `encryptionType`

### Register

- Operations: `RegisterAccountAssociation`, `RegisterCustomEndpoint`
- Traits: `idempotent` (1)
- Common required input members in this group: `AccountAssociationId`, `DeviceDiscoveryId`, `ManagedThingId`

### Send

- Operations: `SendConnectorEvent`, `SendManagedThingCommand`
- Traits: `idempotent` (1)
- Common required input members in this group: `ConnectorId`, `Endpoints`, `ManagedThingId`, `Operation`

### Start

- Operations: `StartAccountAssociationRefresh`, `StartDeviceDiscovery`
- Common required input members in this group: `AccountAssociationId`, `DiscoveryType`

### Deregister

- Operations: `DeregisterAccountAssociation`
- Traits: `idempotent` (1)
- Common required input members in this group: `AccountAssociationId`, `ManagedThingId`

### Reset

- Operations: `ResetRuntimeLogConfiguration`
- Traits: `idempotent` (1)
- Common required input members in this group: `ManagedThingId`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateAccountAssociation` | `POST /account-associations` | `idempotency-token` | `ConnectorDestinationId` | `ClientToken` | `CreateAccountAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Creates a new account association via the destination id. |
| `CreateCloudConnector` | `POST /cloud-connectors` | `idempotency-token` | `EndpointConfig`, `Name` | `ClientToken` | `CreateCloudConnectorResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a C2C (cloud-to-cloud) connector. |
| `CreateConnectorDestination` | `POST /connector-destinations` | `idempotency-token` | `AuthConfig`, `CloudConnectorId` | `ClientToken` | `CreateConnectorDestinationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Create a connector destination for connecting a cloud-to-cloud (C2C) connector to the customer's Amazon Web Services account. |
| `CreateCredentialLocker` | `POST /credential-lockers` | `idempotency-token` | - | `ClientToken` | `CreateCredentialLockerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Create a credential locker. This operation will not trigger the creation of all the manufacturing resources. |
| `CreateDestination` | `POST /destinations` | `idempotency-token` | `DeliveryDestinationArn`, `DeliveryDestinationType`, `Name`, `RoleArn` | `ClientToken` | `CreateDestinationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Create a notification destination such as Kinesis Data Streams that receive events and notifications from Managed integrations. Managed integrations uses the destination to determine where to deliver notifications. |
| `CreateEventLogConfiguration` | `POST /event-log-configurations` | `idempotency-token` | `EventLogLevel`, `ResourceType` | `ClientToken` | `CreateEventLogConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Set the event log configuration for the account, resource type, or specific resource. |
| `CreateManagedThing` | `POST /managed-things` | `idempotency-token` | `AuthenticationMaterial`, `AuthenticationMaterialType`, `Role` | `ClientToken` | `CreateManagedThingResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Creates a managed thing. A managed thing contains the device identifier, protocol supported, and capabilities of the device in a data model format defined by Managed integrations. |
| `CreateNotificationConfiguration` | `POST /notification-configurations` | `idempotency-token` | `DestinationName`, `EventType` | `ClientToken` | `CreateNotificationConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a notification configuration. A configuration is a connection between an event type and a destination that you have already created. |
| `CreateOtaTask` | `POST /ota-tasks` | `idempotency-token` | `OtaType`, `S3Url` | `ClientToken` | `CreateOtaTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Create an over-the-air (OTA) task to target a device. |
| `CreateOtaTaskConfiguration` | `POST /ota-task-configurations` | `idempotency-token` | - | `ClientToken` | `CreateOtaTaskConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Create a configuraiton for the over-the-air (OTA) task. |
| `CreateProvisioningProfile` | `POST /provisioning-profiles` | `idempotency-token` | `ProvisioningType` | `ClientToken` | `CreateProvisioningProfileResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Create a provisioning profile for a device to execute the provisioning flows using a provisioning template. The provisioning template is a document that defines the set of resources and policies applied to a device during the provisioning process. |
| `DeleteAccountAssociation` | `DELETE /account-associations/{AccountAssociationId}` | `idempotent` | `AccountAssociationId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Remove a third-party account association for an end user. You must first call the `DeregisterAccountAssociation` to remove the connection between the managed thing and the third-party account before calling the `DeleteAccountAssociation` API. |
| `DeleteCloudConnector` | `DELETE /cloud-connectors/{Identifier}` | `idempotent` | `Identifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Delete a cloud connector. |
| `DeleteConnectorDestination` | `DELETE /connector-destinations/{Identifier}` | `idempotent` | `Identifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a connector destination linked to a cloud-to-cloud (C2C) connector. Deletion can't be done if the account association has used this connector destination. |
| `DeleteCredentialLocker` | `DELETE /credential-lockers/{Identifier}` | `idempotent` | `Identifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Delete a credential locker. This operation can't be undone and any existing device won't be able to use IoT managed integrations. |
| `DeleteDestination` | `DELETE /destinations/{Name}` | `idempotent` | `Name` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a notification destination specified by name. |
| `DeleteEventLogConfiguration` | `DELETE /event-log-configurations/{Id}` | `idempotent` | `Id` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete an event log configuration. |
| `DeleteManagedThing` | `DELETE /managed-things/{Identifier}` | `idempotent` | `Identifier` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Delete a managed thing. For direct-connected and hub-connected devices connecting with Managed integrations via a controller, all of the devices connected to it will have their status changed to `PENDING`. |
| `DeleteNotificationConfiguration` | `DELETE /notification-configurations/{EventType}` | `idempotent` | `EventType` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a notification configuration. |
| `DeleteOtaTask` | `DELETE /ota-tasks/{Identifier}` | `idempotent` | `Identifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete the over-the-air (OTA) task. |
| `DeleteOtaTaskConfiguration` | `DELETE /ota-task-configurations/{Identifier}` | `idempotent` | `Identifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete the over-the-air (OTA) task configuration. |
| `DeleteProvisioningProfile` | `DELETE /provisioning-profiles/{Identifier}` | `idempotent` | `Identifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Delete a provisioning profile. |
| `DeregisterAccountAssociation` | `PUT /managed-thing-associations/deregister` | `idempotent` | `AccountAssociationId`, `ManagedThingId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deregister an account association from a managed thing. |
| `GetAccountAssociation` | `GET /account-associations/{AccountAssociationId}` | `readonly` | `AccountAssociationId` | - | `GetAccountAssociationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Get an account association for an Amazon Web Services account linked to a customer-managed destination. |
| `GetCloudConnector` | `GET /cloud-connectors/{Identifier}` | `readonly` | `Identifier` | - | `GetCloudConnectorResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get configuration details for a cloud connector. |
| `GetConnectorDestination` | `GET /connector-destinations/{Identifier}` | `readonly` | `Identifier` | - | `GetConnectorDestinationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get connector destination details linked to a cloud-to-cloud (C2C) connector. |
| `GetCredentialLocker` | `GET /credential-lockers/{Identifier}` | `readonly` | `Identifier` | - | `GetCredentialLockerResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Get information on an existing credential locker |
| `GetCustomEndpoint` | `GET /custom-endpoint` | `readonly` | - | - | `GetCustomEndpointResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Returns the IoT managed integrations custom endpoint. |
| `GetDefaultEncryptionConfiguration` | `GET /configuration/account/encryption` | `readonly` | - | - | `GetDefaultEncryptionConfigurationResponse` | `AccessDeniedException`, `InternalFailureException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Retrieves information about the default encryption configuration for the Amazon Web Services account in the default or specified region. For more information, see Key management in the AWS IoT SiteWise User Guide . |
| `GetDestination` | `GET /destinations/{Name}` | `readonly` | `Name` | - | `GetDestinationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets a destination by name. |
| `GetDeviceDiscovery` | `GET /device-discoveries/{Identifier}` | `readonly` | `Identifier` | - | `GetDeviceDiscoveryResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Get the current state of a device discovery. |
| `GetEventLogConfiguration` | `GET /event-log-configurations/{Id}` | `readonly` | `Id` | - | `GetEventLogConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get an event log configuration. |
| `GetHubConfiguration` | `GET /hub-configuration` | `readonly` | - | - | `GetHubConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Get a hub configuration. |
| `GetManagedThing` | `GET /managed-things/{Identifier}` | `readonly` | `Identifier` | - | `GetManagedThingResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Get details of a managed thing including its attributes and capabilities. |
| `GetManagedThingCapabilities` | `GET /managed-things-capabilities/{Identifier}` | `readonly` | `Identifier` | - | `GetManagedThingCapabilitiesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Get the capabilities for a managed thing using the device ID. |
| `GetManagedThingCertificate` | `GET /managed-things-certificate/{Identifier}` | `readonly` | `Identifier` | - | `GetManagedThingCertificateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Retrieves the certificate PEM for a managed IoT thing. |
| `GetManagedThingConnectivityData` | `POST /managed-things-connectivity-data/{Identifier}` | `readonly` | `Identifier` | - | `GetManagedThingConnectivityDataResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Get the connectivity status of a managed thing. |
| `GetManagedThingMetaData` | `GET /managed-things-metadata/{Identifier}` | `readonly` | `Identifier` | - | `GetManagedThingMetaDataResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Get the metadata information for a managed thing. The `managedThing` `metadata` parameter is used for associating attributes with a `managedThing` that can be used for grouping over-the-air (OTA) tasks. |
| `GetManagedThingState` | `GET /managed-thing-states/{ManagedThingId}` | `readonly` | `ManagedThingId` | - | `GetManagedThingStateResponse` | `AccessDeniedException`, `InternalFailureException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Returns the managed thing state for the given device Id. |
| `GetNotificationConfiguration` | `GET /notification-configurations/{EventType}` | `readonly` | `EventType` | - | `GetNotificationConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get a notification configuration for a specified event type. |
| `GetOtaTask` | `GET /ota-tasks/{Identifier}` | `readonly` | `Identifier` | - | `GetOtaTaskResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get details of the over-the-air (OTA) task by its task id. |
| `GetOtaTaskConfiguration` | `GET /ota-task-configurations/{Identifier}` | `readonly` | `Identifier` | - | `GetOtaTaskConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get a configuraiton for the over-the-air (OTA) task. |
| `GetProvisioningProfile` | `GET /provisioning-profiles/{Identifier}` | `readonly` | `Identifier` | - | `GetProvisioningProfileResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Get a provisioning profile by template name. |
| `GetRuntimeLogConfiguration` | `GET /runtime-log-configurations/{ManagedThingId}` | `readonly` | `ManagedThingId` | - | `GetRuntimeLogConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the runtime log configuration for a specific managed thing. |
| `GetSchemaVersion` | `GET /schema-versions/{Type}/{SchemaVersionedId}` | `readonly` | `SchemaVersionedId`, `Type` | - | `GetSchemaVersionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Gets a schema version with the provided information. |
| `ListAccountAssociations` | `GET /account-associations` | `readonly`, `paginated` | - | - | `ListAccountAssociationsResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Lists all account associations, with optional filtering by connector destination ID. |
| `ListCloudConnectors` | `GET /cloud-connectors` | `readonly`, `paginated` | - | - | `ListCloudConnectorsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of connectors filtered by its Lambda Amazon Resource Name (ARN) and `type`. |
| `ListConnectorDestinations` | `GET /connector-destinations` | `readonly`, `paginated` | - | - | `ListConnectorDestinationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all connector destinations, with optional filtering by cloud connector ID. |
| `ListCredentialLockers` | `GET /credential-lockers` | `readonly`, `paginated` | - | - | `ListCredentialLockersResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | List information on an existing credential locker. |
| `ListDestinations` | `GET /destinations` | `readonly`, `paginated` | - | - | `ListDestinationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List all notification destinations. |
| `ListDeviceDiscoveries` | `GET /device-discoveries` | `readonly`, `paginated` | - | - | `ListDeviceDiscoveriesResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Lists all device discovery tasks, with optional filtering by type and status. |
| `ListDiscoveredDevices` | `GET /device-discoveries/{Identifier}/devices` | `readonly`, `paginated` | `Identifier` | - | `ListDiscoveredDevicesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Lists all devices discovered during a specific device discovery task. |
| `ListEventLogConfigurations` | `GET /event-log-configurations` | `readonly`, `paginated` | - | - | `ListEventLogConfigurationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List all event log configurations for an account. |
| `ListManagedThingAccountAssociations` | `GET /managed-thing-associations` | `readonly`, `paginated` | - | - | `ListManagedThingAccountAssociationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all account associations for a specific managed thing. |
| `ListManagedThingSchemas` | `GET /managed-thing-schemas/{Identifier}` | `readonly`, `paginated` | `Identifier` | - | `ListManagedThingSchemasResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | List schemas associated with a managed thing. |
| `ListManagedThings` | `GET /managed-things` | `readonly`, `paginated` | - | - | `ListManagedThingsResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Listing all managed things with provision for filters. |
| `ListNotificationConfigurations` | `GET /notification-configurations` | `readonly`, `paginated` | - | - | `ListNotificationConfigurationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List all notification configurations. |
| `ListOtaTaskConfigurations` | `GET /ota-task-configurations` | `readonly`, `paginated` | - | - | `ListOtaTaskConfigurationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List all of the over-the-air (OTA) task configurations. |
| `ListOtaTaskExecutions` | `GET /ota-tasks/{Identifier}/devices` | `readonly`, `paginated` | `Identifier` | - | `ListOtaTaskExecutionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List all of the over-the-air (OTA) task executions. |
| `ListOtaTasks` | `GET /ota-tasks` | `readonly`, `paginated` | - | - | `ListOtaTasksResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List all of the over-the-air (OTA) tasks. |
| `ListProvisioningProfiles` | `GET /provisioning-profiles` | `readonly`, `paginated` | - | - | `ListProvisioningProfilesResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | List the provisioning profiles within the Amazon Web Services account. |
| `ListSchemaVersions` | `GET /schema-versions/{Type}` | `readonly`, `paginated` | `Type` | - | `ListSchemaVersionsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Lists schema versions with the provided information. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException` | Lists the tags for a specified resource. |
| `PutDefaultEncryptionConfiguration` | `POST /configuration/account/encryption` | - | `encryptionType` | - | `PutDefaultEncryptionConfigurationResponse` | `AccessDeniedException`, `InternalFailureException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Sets the default encryption configuration for the Amazon Web Services account. For more information, see Key management in the AWS IoT SiteWise User Guide. |
| `PutHubConfiguration` | `PUT /hub-configuration` | `idempotent` | `HubTokenTimerExpirySettingInSeconds` | - | `PutHubConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Update a hub configuration. |
| `PutRuntimeLogConfiguration` | `PUT /runtime-log-configurations/{ManagedThingId}` | `idempotent` | `ManagedThingId`, `RuntimeLogConfigurations` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Set the runtime log configuration for a specific managed thing. |
| `RegisterAccountAssociation` | `PUT /managed-thing-associations/register` | `idempotent` | `AccountAssociationId`, `DeviceDiscoveryId`, `ManagedThingId` | - | `RegisterAccountAssociationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Registers an account association with a managed thing, establishing a connection between a device and a third-party account. |
| `RegisterCustomEndpoint` | `POST /custom-endpoint` | - | - | - | `RegisterCustomEndpointResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Customers can request IoT managed integrations to manage the server trust for them or bring their own external server trusts for the custom domain. Returns an IoT managed integrations endpoint. |
| `ResetRuntimeLogConfiguration` | `DELETE /runtime-log-configurations/{ManagedThingId}` | `idempotent` | `ManagedThingId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Reset a runtime log configuration for a specific managed thing. |
| `SendConnectorEvent` | `POST /connector-event/{ConnectorId}` | `idempotent` | `ConnectorId`, `Operation` | - | `SendConnectorEventResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Relays third-party device events for a connector such as a new device or a device state change event. |
| `SendManagedThingCommand` | `POST /managed-things-command/{ManagedThingId}` | - | `Endpoints`, `ManagedThingId` | - | `SendManagedThingCommandResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Send the command to the device represented by the managed thing. |
| `StartAccountAssociationRefresh` | `POST /account-associations/{AccountAssociationId}/refresh` | - | `AccountAssociationId` | - | `StartAccountAssociationRefreshResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Initiates a refresh of an existing account association to update its authorization and connection status. |
| `StartDeviceDiscovery` | `POST /device-discoveries` | - | `DiscoveryType` | - | `StartDeviceDiscoveryResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | This API is used to start device discovery for hub-connected and third-party-connected devices. The authentication material (install code) is delivered as a message to the controller instructing it to start the discovery. |
| `TagResource` | `POST /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `ConflictException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException` | Adds tags to a specified resource. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `ConflictException`, `InvalidRequestException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException` | Removes tags from a specified resource. |
| `UpdateAccountAssociation` | `PUT /account-associations/{AccountAssociationId}` | `idempotent` | `AccountAssociationId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `ValidationException` | Updates the properties of an existing account association. |
| `UpdateCloudConnector` | `PUT /cloud-connectors/{Identifier}` | `idempotent` | `Identifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Update an existing cloud connector. |
| `UpdateConnectorDestination` | `PUT /connector-destinations/{Identifier}` | `idempotent` | `Identifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the properties of an existing connector destination. |
| `UpdateDestination` | `PUT /destinations/{Name}` | `idempotent` | `Name` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a destination specified by name. |
| `UpdateEventLogConfiguration` | `PATCH /event-log-configurations/{Id}` | `idempotent` | `EventLogLevel`, `Id` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update an event log configuration by log configuration ID. |
| `UpdateManagedThing` | `PUT /managed-things/{Identifier}` | `idempotent` | `Identifier` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceUnavailableException`, `ThrottlingException`, `UnauthorizedException`, `ValidationException` | Update the attributes and capabilities associated with a managed thing. |
| `UpdateNotificationConfiguration` | `PUT /notification-configurations/{EventType}` | `idempotent` | `DestinationName`, `EventType` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update a notification configuration. |
| `UpdateOtaTask` | `PUT /ota-tasks/{Identifier}` | `idempotent` | `Identifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Update an over-the-air (OTA) task. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ThrottlingException` | `structure` | `Message` | The rate exceeds the limit. |
| `AccessDeniedException` | `structure` | `Message` | User is not authorized. |
| `ValidationException` | `structure` | `Message` | A validation error occurred when performing the API request. |
| `InternalServerException` | `structure` | `Message` | Internal error from the service that indicates an unexpected error or that the service is unavailable. |
| `ResourceNotFoundException` | `structure` | `Message`, `ResourceId`, `ResourceType` | The specified resource does not exist. |
| `ServiceUnavailableException` | `structure` | `Message` | The service is temporarily unavailable. |
| `UnauthorizedException` | `structure` | `Message` | You are not authorized to perform this operation. |
| `ConflictException` | `structure` | `Message` | There is a conflict with the request. |
| `InternalFailureException` | `structure` | `Message` | An unexpected error has occurred. |
| `InvalidRequestException` | `structure` | `Message` | The request is not valid. |
| `ServiceQuotaExceededException` | `structure` | `Message` | The service quota has been exceeded for this request. |
| `CreateAccountAssociationRequest` | `structure` | `ClientToken`, `ConnectorDestinationId`, `Description`, `GeneralAuthorization`, `Name`, `Tags` | - |
| `CreateAccountAssociationResponse` | `structure` | `AccountAssociationId`, `Arn`, `AssociationState`, `OAuthAuthorizationUrl` | - |
| `CreateCloudConnectorRequest` | `structure` | `ClientToken`, `Description`, `EndpointConfig`, `EndpointType`, `Name` | - |
| `CreateCloudConnectorResponse` | `structure` | `Id` | - |
| `CreateConnectorDestinationRequest` | `structure` | `AuthConfig`, `AuthType`, `ClientToken`, `CloudConnectorId`, `Description`, `Name`, `SecretsManager` | - |
| `CreateConnectorDestinationResponse` | `structure` | `Id` | - |
| `CreateCredentialLockerRequest` | `structure` | `ClientToken`, `Name`, `Tags` | - |
| `CreateCredentialLockerResponse` | `structure` | `Arn`, `CreatedAt`, `Id` | - |
| `CreateDestinationRequest` | `structure` | `ClientToken`, `DeliveryDestinationArn`, `DeliveryDestinationType`, `Description`, `Name`, `RoleArn`, `Tags` | - |
| `CreateDestinationResponse` | `structure` | `Name` | - |
| `CreateEventLogConfigurationRequest` | `structure` | `ClientToken`, `EventLogLevel`, `ResourceId`, `ResourceType` | - |
| `CreateEventLogConfigurationResponse` | `structure` | `Id` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
