# AWS IoT FleetWise

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services IoT FleetWise is a fully managed service that you can use to collect, model, and transfer vehicle data to the Amazon Web Services cloud at scale. With Amazon Web Services IoT FleetWise, you can standardize all of your vehicle data models, independent of the in-vehicle communication architecture, and define data collection rules to transfer only high-value data to the cloud. For more information, see What is Amazon Web Services IoT FleetWise? in the Amazon Web Services IoT FleetWise Developer Guide . Access to certain Amazon Web Services IoT FleetWise features is currently gated.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS IoT FleetWise where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for AWS IoT FleetWise by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for AWS IoT FleetWise by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented AWS IoT FleetWise workflows in the local mock. Key resources include `CampaignResource`, `DecoderManifestResource`, `FleetAssociationResource`, `FleetResource`, `ModelManifestResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Create`, `Delete`, `Update` operation families, including `ListCampaigns`, `ListDecoderManifestNetworkInterfaces`, `ListDecoderManifestSignals`, `ListDecoderManifests`, `GetCampaign`, `GetDecoderManifest`.

## Service Identity and Protocol

- AWS model slug: `iotfleetwise`
- AWS SDK for Rust slug: `iotfleetwise`
- Model version: `2021-06-17`
- Model file: `vendor/api-models-aws/models/iotfleetwise/service/2021-06-17/iotfleetwise-2021-06-17.json`
- SDK ID: `IoTFleetWise`
- Endpoint prefix: `-`
- ARN namespace: `iotfleetwise`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (14), `Get` (11), `Create` (7), `Delete` (7), `Update` (7), `Batch` (2), `Import` (2), `Put` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateVehicleFleet`, `BatchCreateVehicle`, `BatchUpdateVehicle`, `CreateCampaign`, `CreateDecoderManifest`, `CreateFleet`, `CreateModelManifest`, `CreateSignalCatalog`, `CreateStateTemplate`, `CreateVehicle`, `DeleteCampaign`, `DeleteDecoderManifest`, `DeleteFleet`, `DeleteModelManifest`, `DeleteSignalCatalog`, `DeleteStateTemplate`, `DeleteVehicle`, `DisassociateVehicleFleet`, `ImportDecoderManifest`, `ImportSignalCatalog`, ... (+12).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetCampaign`, `GetDecoderManifest`, `GetEncryptionConfiguration`, `GetFleet`, `GetLoggingOptions`, `GetModelManifest`, `GetRegisterAccountStatus`, `GetSignalCatalog`, `GetStateTemplate`, `GetVehicle`, `GetVehicleStatus`, `ListCampaigns`, `ListDecoderManifestNetworkInterfaces`, `ListDecoderManifestSignals`, `ListDecoderManifests`, `ListFleets`, `ListFleetsForVehicle`, `ListModelManifestNodes`, `ListModelManifests`, `ListSignalCatalogNodes`, ... (+5).
- Pagination is modelled for 14 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 22 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `ImportDecoderManifest`, `ImportSignalCatalog`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 57 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `CloudWatch Logs`, `SNS`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `CampaignResource` | `name` | put: `CreateCampaign`; read: `GetCampaign`; update: `UpdateCampaign`; delete: `DeleteCampaign`; list: `ListCampaigns` | - | - |
| `DecoderManifestResource` | `name` | put: `CreateDecoderManifest`; read: `GetDecoderManifest`; update: `UpdateDecoderManifest`; delete: `DeleteDecoderManifest`; list: `ListDecoderManifests` | `ImportDecoderManifest`, `ListDecoderManifestNetworkInterfaces`, `ListDecoderManifestSignals` | - |
| `FleetAssociationResource` | `fleetId`, `vehicleName` | list: `ListFleetsForVehicle` | - | - |
| `FleetResource` | `fleetId` | put: `CreateFleet`; read: `GetFleet`; update: `UpdateFleet`; delete: `DeleteFleet`; list: `ListFleets` | - | - |
| `ModelManifestResource` | `name` | put: `CreateModelManifest`; read: `GetModelManifest`; update: `UpdateModelManifest`; delete: `DeleteModelManifest`; list: `ListModelManifests` | `ListModelManifestNodes` | - |
| `SignalCatalogResource` | `name` | put: `CreateSignalCatalog`; read: `GetSignalCatalog`; update: `UpdateSignalCatalog`; delete: `DeleteSignalCatalog`; list: `ListSignalCatalogs` | `ImportSignalCatalog`, `ListSignalCatalogNodes` | - |
| `StateTemplateResource` | `identifier` | create: `CreateStateTemplate`; read: `GetStateTemplate`; update: `UpdateStateTemplate`; delete: `DeleteStateTemplate`; list: `ListStateTemplates` | - | - |
| `VehicleAssociationResource` | `fleetId`, `vehicleName` | list: `ListVehiclesInFleet` | - | - |
| `VehicleResource` | `vehicleName` | put: `CreateVehicle`; read: `GetVehicle`; update: `UpdateVehicle`; delete: `DeleteVehicle`; list: `ListVehicles` | `AssociateVehicleFleet`, `DisassociateVehicleFleet` | - |
## Operation Groups

### Get

- Operations: `GetEncryptionConfiguration`, `GetLoggingOptions`, `GetRegisterAccountStatus`, `GetVehicleStatus`
- Traits: `readonly` (4), `paginated` (1)
- Common required input members in this group: -

### Batch

- Operations: `BatchCreateVehicle`, `BatchUpdateVehicle`
- Common required input members in this group: `vehicles`

### Put

- Operations: `PutEncryptionConfiguration`, `PutLoggingOptions`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Register

- Operations: `RegisterAccount`
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
| `BatchCreateVehicle` | `POST /vehicles` | - | `vehicles` | - | `BatchCreateVehicleResponse` | `AccessDeniedException`, `InternalServerException`, `LimitExceededException`, `ThrottlingException`, `ValidationException` | Creates a group, or batch, of vehicles. You must specify a decoder manifest and a vehicle model (model manifest) for each vehicle. For more information, see Create multiple vehicles (AWS CLI) in the Amazon Web Servic ... |
| `BatchUpdateVehicle` | `PUT /vehicles` | - | `vehicles` | - | `BatchUpdateVehicleResponse` | `AccessDeniedException`, `InternalServerException`, `LimitExceededException`, `ThrottlingException`, `ValidationException` | Updates a group, or batch, of vehicles. You must specify a decoder manifest and a vehicle model (model manifest) for each vehicle. For more information, see Update multiple vehicles (AWS CLI) in the Amazon Web Servic ... |
| `GetEncryptionConfiguration` | `GET /encryptionConfiguration` | `readonly` | - | - | `GetEncryptionConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the encryption configuration for resources and data in Amazon Web Services IoT FleetWise. |
| `GetLoggingOptions` | `GET /loggingOptions` | `readonly` | - | - | `GetLoggingOptionsResponse` | `AccessDeniedException`, `ThrottlingException` | Retrieves the logging options. |
| `GetRegisterAccountStatus` | `GET /account/registration_status` | `readonly` | - | - | `GetRegisterAccountStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the status of registering your Amazon Web Services account, IAM, and Amazon Timestream resources so that Amazon Web Services IoT FleetWise can transfer your vehicle data to the Amazon Web ... |
| `GetVehicleStatus` | `GET /vehicles/{vehicleName}/status` | `readonly`, `paginated` | `vehicleName` | - | `GetVehicleStatusResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the status of campaigns, decoder manifests, or state templates associated with a vehicle. |
| `ListTagsForResource` | `GET /tags` | `readonly` | `ResourceARN` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags (metadata) you have assigned to the resource. |
| `PutEncryptionConfiguration` | `POST /encryptionConfiguration` | - | `encryptionType` | - | `PutEncryptionConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates or updates the encryption configuration. Amazon Web Services IoT FleetWise can encrypt your data and resources using an Amazon Web Services managed key. Or, you can use a KMS key that you own and manage. For ... |
| `PutLoggingOptions` | `PUT /loggingOptions` | `idempotent` | `cloudWatchLogDelivery` | - | `PutLoggingOptionsResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates or updates the logging option. |
| `RegisterAccount` | `POST /account/registration` | - | - | - | `RegisterAccountResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This API operation contains deprecated parameters. Register your account again without the Timestream resources parameter so that Amazon Web Services IoT FleetWise can remove the Timestream metadata stored. You shoul ... |
| `TagResource` | `POST /tags` | `idempotent` | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds to or modifies the tags of the given resource. Tags are metadata which can be used to manage a resource. |
| `UntagResource` | `DELETE /tags` | `idempotent` | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the given tags (metadata) from the resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetVehicleStatus` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListTagsForResource` | - | `ResourceARN -> resourceArn` | - | - |
| `TagResource` | - | `ResourceARN -> resourceArn` | - | - |
| `UntagResource` | - | `ResourceARN -> resourceArn`, `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You don't have sufficient permission to perform this action. |
| `ConflictException` | `structure` | message, resource, resourceType | The request has conflicting operations. This can occur if you're trying to perform more than one operation on the same resource at the same time. |
| `DecoderManifestValidationException` | `structure` | invalidSignals, invalidNetworkInterfaces, message | The request couldn't be completed because it contains signal decoders with one or more validation errors. |
| `InternalServerException` | `structure` | message, retryAfterSeconds | The request couldn't be completed because the server temporarily failed. |
| `InvalidNodeException` | `structure` | invalidNodes, reason, message | The specified node type doesn't match the expected node type for a node. You can specify the node type as branch, sensor, actuator, or attribute. |
| `InvalidSignalsException` | `structure` | message, invalidSignals | The request couldn't be completed because it contains signals that aren't valid. |
| `LimitExceededException` | `structure` | message, resourceId, resourceType | A service quota was exceeded. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | The resource wasn't found. |
| `ThrottlingException` | `structure` | message, quotaCode, serviceCode, retryAfterSeconds | The request couldn't be completed due to throttling. |
| `ValidationException` | `structure` | message, reason, fieldList | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `BatchCreateVehicleRequest` | `structure` | vehicles | - |
| `BatchCreateVehicleResponse` | `structure` | vehicles, errors | - |
| `BatchUpdateVehicleRequest` | `structure` | vehicles | - |
| `BatchUpdateVehicleResponse` | `structure` | vehicles, errors | - |
| `GetEncryptionConfigurationRequest` | `structure` | **empty (no members)** | - |
| `GetEncryptionConfigurationResponse` | `structure` | kmsKeyId, encryptionStatus, encryptionType, errorMessage, creationTime, lastModificationTime | - |
| `GetLoggingOptionsRequest` | `structure` | **empty (no members)** | - |
| `GetLoggingOptionsResponse` | `structure` | cloudWatchLogDelivery | - |
| `GetRegisterAccountStatusRequest` | `structure` | **empty (no members)** | - |
| `GetRegisterAccountStatusResponse` | `structure` | customerAccountId, accountStatus, timestreamRegistrationResponse, iamRegistrationResponse, creationTime, lastModificationTime | - |
| `GetVehicleStatusRequest` | `structure` | nextToken, maxResults, vehicleName | - |
| `GetVehicleStatusResponse` | `structure` | campaigns, nextToken | - |
| `ListTagsForResourceRequest` | `structure` | ResourceARN | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `PutEncryptionConfigurationRequest` | `structure` | kmsKeyId, encryptionType | - |
| `PutEncryptionConfigurationResponse` | `structure` | kmsKeyId, encryptionStatus, encryptionType | - |
| `PutLoggingOptionsRequest` | `structure` | cloudWatchLogDelivery | - |
| `PutLoggingOptionsResponse` | `structure` | **empty (no members)** | - |
| `RegisterAccountRequest` | `structure` | timestreamResources, iamResources | - |
| `RegisterAccountResponse` | `structure` | registerAccountStatus, timestreamResources, iamResources, creationTime, lastModificationTime | - |
| `TagResourceRequest` | `structure` | ResourceARN, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceARN, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `CampaignStatus` | `enum` | CREATING, WAITING_FOR_APPROVAL, RUNNING, SUSPENDED | - |
| `Compression` | `enum` | OFF, SNAPPY | - |
| `DataFormat` | `enum` | JSON, PARQUET | - |
| `DefaultForUnmappedSignalsType` | `enum` | CUSTOM_DECODING | - |
| `DiagnosticsMode` | `enum` | OFF, SEND_ACTIVE_DTCS | - |
| `EncryptionStatus` | `enum` | PENDING, SUCCESS, FAILURE | - |
| `EncryptionType` | `enum` | KMS_BASED_ENCRYPTION, FLEETWISE_DEFAULT_ENCRYPTION | - |
| `ListResponseScope` | `enum` | METADATA_ONLY | - |
| `LogType` | `enum` | OFF, ERROR | - |
| `ManifestStatus` | `enum` | ACTIVE, DRAFT, INVALID, VALIDATING | - |
| `NetworkInterfaceFailureReason` | `enum` | DUPLICATE_INTERFACE, CONFLICTING_NETWORK_INTERFACE, NETWORK_INTERFACE_TO_ADD_ALREADY_EXISTS, CAN_NETWORK_INTERFACE_INFO_IS_NULL, OBD_NETWORK_INTERFACE_INFO_IS_NULL, NETWORK_INTERFACE_TO_REMOVE_ASSOCIATED_WITH_SIGNALS, VEHICLE_MIDDLEWARE_NETWORK_INTERFACE_INFO_IS_NULL, CUSTOM_DECODING_SIGNAL_NETWORK_INTERFACE_INFO_IS_NULL | - |
| `NetworkInterfaceType` | `enum` | CAN_INTERFACE, OBD_INTERFACE, VEHICLE_MIDDLEWARE, CUSTOM_DECODING_INTERFACE | - |
| `NodeDataEncoding` | `enum` | BINARY, TYPED | - |
| `NodeDataType` | `enum` | INT8, UINT8, INT16, UINT16, INT32, UINT32, INT64, UINT64, BOOLEAN, FLOAT, DOUBLE, STRING, ... (+17) | - |
| `ROS2PrimitiveType` | `enum` | BOOL, BYTE, CHAR, FLOAT32, FLOAT64, INT8, UINT8, INT16, UINT16, INT32, UINT32, INT64, ... (+3) | - |
| `RegistrationStatus` | `enum` | REGISTRATION_PENDING, REGISTRATION_SUCCESS, REGISTRATION_FAILURE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
