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

### List

- Operations: `ListCampaigns`, `ListDecoderManifestNetworkInterfaces`, `ListDecoderManifestSignals`, `ListDecoderManifests`, `ListFleets`, `ListFleetsForVehicle`, `ListModelManifestNodes`, `ListModelManifests`, `ListSignalCatalogNodes`, `ListSignalCatalogs`, `ListStateTemplates`, `ListTagsForResource`, `ListVehicles`, `ListVehiclesInFleet`
- Traits: `paginated` (13), `readonly` (14)
- Common required input members in this group: `ResourceARN`, `fleetId`, `name`, `vehicleName`

### Get

- Operations: `GetCampaign`, `GetDecoderManifest`, `GetEncryptionConfiguration`, `GetFleet`, `GetLoggingOptions`, `GetModelManifest`, `GetRegisterAccountStatus`, `GetSignalCatalog`, `GetStateTemplate`, `GetVehicle`, `GetVehicleStatus`
- Traits: `paginated` (1), `readonly` (11)
- Common required input members in this group: `fleetId`, `identifier`, `name`, `vehicleName`

### Create

- Operations: `CreateCampaign`, `CreateDecoderManifest`, `CreateFleet`, `CreateModelManifest`, `CreateSignalCatalog`, `CreateStateTemplate`, `CreateVehicle`
- Traits: `idempotent` (7)
- Common required input members in this group: `collectionScheme`, `decoderManifestArn`, `fleetId`, `modelManifestArn`, `name`, `nodes`, `signalCatalogArn`, `stateTemplateProperties`, `targetArn`, `vehicleName`

### Delete

- Operations: `DeleteCampaign`, `DeleteDecoderManifest`, `DeleteFleet`, `DeleteModelManifest`, `DeleteSignalCatalog`, `DeleteStateTemplate`, `DeleteVehicle`
- Traits: `idempotent` (7)
- Common required input members in this group: `fleetId`, `identifier`, `name`, `vehicleName`

### Update

- Operations: `UpdateCampaign`, `UpdateDecoderManifest`, `UpdateFleet`, `UpdateModelManifest`, `UpdateSignalCatalog`, `UpdateStateTemplate`, `UpdateVehicle`
- Traits: `idempotent` (4)
- Common required input members in this group: `action`, `fleetId`, `identifier`, `name`, `vehicleName`

### Batch

- Operations: `BatchCreateVehicle`, `BatchUpdateVehicle`
- Common required input members in this group: `vehicles`

### Import

- Operations: `ImportDecoderManifest`, `ImportSignalCatalog`
- Traits: `idempotent` (1)
- Common required input members in this group: `name`, `networkFileDefinitions`

### Put

- Operations: `PutEncryptionConfiguration`, `PutLoggingOptions`
- Traits: `idempotent` (1)
- Common required input members in this group: `cloudWatchLogDelivery`, `encryptionType`

### Associate

- Operations: `AssociateVehicleFleet`
- Common required input members in this group: `fleetId`, `vehicleName`

### Disassociate

- Operations: `DisassociateVehicleFleet`
- Common required input members in this group: `fleetId`, `vehicleName`

### Register

- Operations: `RegisterAccount`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateVehicleFleet` | `PUT /vehicles/{vehicleName}/associate` | - | `fleetId`, `vehicleName` | - | `AssociateVehicleFleetResponse` | `AccessDeniedException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds, or associates, a vehicle with a fleet. |
| `BatchCreateVehicle` | `POST /vehicles` | - | `vehicles` | - | `BatchCreateVehicleResponse` | `AccessDeniedException`, `InternalServerException`, `LimitExceededException`, `ThrottlingException`, `ValidationException` | Creates a group, or batch, of vehicles. You must specify a decoder manifest and a vehicle model (model manifest) for each vehicle. |
| `BatchUpdateVehicle` | `PUT /vehicles` | - | `vehicles` | - | `BatchUpdateVehicleResponse` | `AccessDeniedException`, `InternalServerException`, `LimitExceededException`, `ThrottlingException`, `ValidationException` | Updates a group, or batch, of vehicles. You must specify a decoder manifest and a vehicle model (model manifest) for each vehicle. |
| `CreateCampaign` | `POST /campaigns/{name}` | `idempotent` | `collectionScheme`, `name`, `signalCatalogArn`, `targetArn` | - | `CreateCampaignResponse` | `AccessDeniedException`, `ConflictException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates an orchestration of data collection rules. The Amazon Web Services IoT FleetWise Edge Agent software running in vehicles uses campaigns to decide how to collect and transfer data to the cloud. |
| `CreateDecoderManifest` | `POST /decoder-manifests/{name}` | `idempotent` | `modelManifestArn`, `name` | - | `CreateDecoderManifestResponse` | `AccessDeniedException`, `ConflictException`, `DecoderManifestValidationException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates the decoder manifest associated with a model manifest. To create a decoder manifest, the following must be true: Every signal decoder has a unique name. |
| `CreateFleet` | `POST /fleets/{fleetId}` | `idempotent` | `fleetId`, `signalCatalogArn` | - | `CreateFleetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a fleet that represents a group of vehicles. You must create both a signal catalog and vehicles before you can create a fleet. |
| `CreateModelManifest` | `POST /model-manifests/{name}` | `idempotent` | `name`, `nodes`, `signalCatalogArn` | - | `CreateModelManifestResponse` | `AccessDeniedException`, `ConflictException`, `InvalidSignalsException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a vehicle model (model manifest) that specifies signals (attributes, branches, sensors, and actuators). For more information, see Vehicle models in the Amazon Web Services IoT FleetWise Developer Guide . |
| `CreateSignalCatalog` | `POST /signal-catalogs/{name}` | `idempotent` | `name` | - | `CreateSignalCatalogResponse` | `AccessDeniedException`, `ConflictException`, `InvalidNodeException`, `InvalidSignalsException`, `LimitExceededException`, `ThrottlingException`, `ValidationException` | Creates a collection of standardized signals that can be reused to create vehicle models. |
| `CreateStateTemplate` | `POST /state-templates/{name}` | `idempotent` | `name`, `signalCatalogArn`, `stateTemplateProperties` | - | `CreateStateTemplateResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidSignalsException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a state template. State templates contain state properties, which are signals that belong to a signal catalog that is synchronized between the Amazon Web Services IoT FleetWise Edge and the Amazon Web Services Cloud. |
| `CreateVehicle` | `POST /vehicles/{vehicleName}` | `idempotent` | `decoderManifestArn`, `modelManifestArn`, `vehicleName` | - | `CreateVehicleResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a vehicle, which is an instance of a vehicle model (model manifest). Vehicles created from the same vehicle model consist of the same signals inherited from the vehicle model. |
| `DeleteCampaign` | `DELETE /campaigns/{name}` | `idempotent` | `name` | - | `DeleteCampaignResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a data collection campaign. Deleting a campaign suspends all data collection and removes it from any vehicles. |
| `DeleteDecoderManifest` | `DELETE /decoder-manifests/{name}` | `idempotent` | `name` | - | `DeleteDecoderManifestResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a decoder manifest. You can't delete a decoder manifest if it has vehicles associated with it. |
| `DeleteFleet` | `DELETE /fleets/{fleetId}` | `idempotent` | `fleetId` | - | `DeleteFleetResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a fleet. Before you delete a fleet, all vehicles must be dissociated from the fleet. |
| `DeleteModelManifest` | `DELETE /model-manifests/{name}` | `idempotent` | `name` | - | `DeleteModelManifestResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a vehicle model (model manifest). |
| `DeleteSignalCatalog` | `DELETE /signal-catalogs/{name}` | `idempotent` | `name` | - | `DeleteSignalCatalogResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a signal catalog. |
| `DeleteStateTemplate` | `DELETE /state-templates/{identifier}` | `idempotent` | `identifier` | - | `DeleteStateTemplateResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a state template. |
| `DeleteVehicle` | `DELETE /vehicles/{vehicleName}` | `idempotent` | `vehicleName` | - | `DeleteVehicleResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes a vehicle and removes it from any campaigns. |
| `DisassociateVehicleFleet` | `PUT /vehicles/{vehicleName}/disassociate` | - | `fleetId`, `vehicleName` | - | `DisassociateVehicleFleetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes, or disassociates, a vehicle from a fleet. Disassociating a vehicle from a fleet doesn't delete the vehicle. |
| `GetCampaign` | `GET /campaigns/{name}` | `readonly` | `name` | - | `GetCampaignResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a campaign. Access to certain Amazon Web Services IoT FleetWise features is currently gated. |
| `GetDecoderManifest` | `GET /decoder-manifests/{name}` | `readonly` | `name` | - | `GetDecoderManifestResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a created decoder manifest. |
| `GetEncryptionConfiguration` | `GET /encryptionConfiguration` | `readonly` | - | - | `GetEncryptionConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the encryption configuration for resources and data in Amazon Web Services IoT FleetWise. |
| `GetFleet` | `GET /fleets/{fleetId}` | `readonly` | `fleetId` | - | `GetFleetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a fleet. |
| `GetLoggingOptions` | `GET /loggingOptions` | `readonly` | - | - | `GetLoggingOptionsResponse` | `AccessDeniedException`, `ThrottlingException` | Retrieves the logging options. |
| `GetModelManifest` | `GET /model-manifests/{name}` | `readonly` | `name` | - | `GetModelManifestResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a vehicle model (model manifest). |
| `GetRegisterAccountStatus` | `GET /account/registration_status` | `readonly` | - | - | `GetRegisterAccountStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the status of registering your Amazon Web Services account, IAM, and Amazon Timestream resources so that Amazon Web Services IoT FleetWise can transfer your vehicle data to the Amazon Web Services Cloud. For more information... |
| `GetSignalCatalog` | `GET /signal-catalogs/{name}` | `readonly` | `name` | - | `GetSignalCatalogResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a signal catalog. |
| `GetStateTemplate` | `GET /state-templates/{identifier}` | `readonly` | `identifier` | - | `GetStateTemplateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a state template. Access to certain Amazon Web Services IoT FleetWise features is currently gated. |
| `GetVehicle` | `GET /vehicles/{vehicleName}` | `readonly` | `vehicleName` | - | `GetVehicleResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a vehicle. |
| `GetVehicleStatus` | `GET /vehicles/{vehicleName}/status` | `readonly`, `paginated` | `vehicleName` | - | `GetVehicleStatusResponse` | `AccessDeniedException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the status of campaigns, decoder manifests, or state templates associated with a vehicle. |
| `ImportDecoderManifest` | `PUT /decoder-manifests/{name}` | - | `name`, `networkFileDefinitions` | - | `ImportDecoderManifestResponse` | `AccessDeniedException`, `ConflictException`, `DecoderManifestValidationException`, `InvalidSignalsException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a decoder manifest using your existing CAN DBC file from your local device. The CAN signal name must be unique and not repeated across CAN message definitions in a .dbc file. |
| `ImportSignalCatalog` | `PUT /signal-catalogs/{name}/nodes` | `idempotent` | `name` | - | `ImportSignalCatalogResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidSignalsException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a signal catalog using your existing VSS formatted content from your local device. |
| `ListCampaigns` | `GET /campaigns` | `readonly`, `paginated` | - | - | `ListCampaignsResponse` | `AccessDeniedException`, `ThrottlingException`, `ValidationException` | Lists information about created campaigns. This API operation uses pagination. |
| `ListDecoderManifestNetworkInterfaces` | `GET /decoder-manifests/{name}/network-interfaces` | `readonly`, `paginated` | `name` | - | `ListDecoderManifestNetworkInterfacesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the network interfaces specified in a decoder manifest. This API operation uses pagination. |
| `ListDecoderManifestSignals` | `GET /decoder-manifests/{name}/signals` | `readonly`, `paginated` | `name` | - | `ListDecoderManifestSignalsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | A list of information about signal decoders specified in a decoder manifest. This API operation uses pagination. |
| `ListDecoderManifests` | `GET /decoder-manifests` | `readonly`, `paginated` | - | - | `ListDecoderManifestsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists decoder manifests. This API operation uses pagination. |
| `ListFleets` | `GET /fleets` | `readonly`, `paginated` | - | - | `ListFleetsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information for each created fleet in an Amazon Web Services account. This API operation uses pagination. |
| `ListFleetsForVehicle` | `GET /vehicles/{vehicleName}/fleets` | `readonly`, `paginated` | `vehicleName` | - | `ListFleetsForVehicleResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of IDs for all fleets that the vehicle is associated with. This API operation uses pagination. |
| `ListModelManifestNodes` | `GET /model-manifests/{name}/nodes` | `readonly`, `paginated` | `name` | - | `ListModelManifestNodesResponse` | `AccessDeniedException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists information about nodes specified in a vehicle model (model manifest). This API operation uses pagination. |
| `ListModelManifests` | `GET /model-manifests` | `readonly`, `paginated` | - | - | `ListModelManifestsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of vehicle models (model manifests). This API operation uses pagination. |
| `ListSignalCatalogNodes` | `GET /signal-catalogs/{name}/nodes` | `readonly`, `paginated` | `name` | - | `ListSignalCatalogNodesResponse` | `AccessDeniedException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists of information about the signals (nodes) specified in a signal catalog. This API operation uses pagination. |
| `ListSignalCatalogs` | `GET /signal-catalogs` | `readonly`, `paginated` | - | - | `ListSignalCatalogsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all the created signal catalogs in an Amazon Web Services account. You can use to list information about each signal (node) specified in a signal catalog. |
| `ListStateTemplates` | `GET /state-templates` | `readonly`, `paginated` | - | - | `ListStateTemplatesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists information about created state templates. Access to certain Amazon Web Services IoT FleetWise features is currently gated. |
| `ListTagsForResource` | `GET /tags` | `readonly` | `ResourceARN` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the tags (metadata) you have assigned to the resource. |
| `ListVehicles` | `GET /vehicles` | `readonly`, `paginated` | - | - | `ListVehiclesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of summaries of created vehicles. This API operation uses pagination. |
| `ListVehiclesInFleet` | `GET /fleets/{fleetId}/vehicles` | `readonly`, `paginated` | `fleetId` | - | `ListVehiclesInFleetResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of summaries of all vehicles associated with a fleet. This API operation uses pagination. |
| `PutEncryptionConfiguration` | `POST /encryptionConfiguration` | - | `encryptionType` | - | `PutEncryptionConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates or updates the encryption configuration. Amazon Web Services IoT FleetWise can encrypt your data and resources using an Amazon Web Services managed key. |
| `PutLoggingOptions` | `PUT /loggingOptions` | `idempotent` | `cloudWatchLogDelivery` | - | `PutLoggingOptionsResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates or updates the logging option. |
| `RegisterAccount` | `POST /account/registration` | - | - | - | `RegisterAccountResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This API operation contains deprecated parameters. Register your account again without the Timestream resources parameter so that Amazon Web Services IoT FleetWise can remove the Timestream metadata stored. |
| `TagResource` | `POST /tags` | `idempotent` | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds to or modifies the tags of the given resource. Tags are metadata which can be used to manage a resource. |
| `UntagResource` | `DELETE /tags` | `idempotent` | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the given tags (metadata) from the resource. |
| `UpdateCampaign` | `PUT /campaigns/{name}` | - | `action`, `name` | - | `UpdateCampaignResponse` | `AccessDeniedException`, `ConflictException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a campaign. |
| `UpdateDecoderManifest` | `PATCH /decoder-manifests/{name}` | `idempotent` | `name` | - | `UpdateDecoderManifestResponse` | `AccessDeniedException`, `ConflictException`, `DecoderManifestValidationException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a decoder manifest. A decoder manifest can only be updated when the status is `DRAFT`. |
| `UpdateFleet` | `PATCH /fleets/{fleetId}` | - | `fleetId` | - | `UpdateFleetResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the description of an existing fleet. |
| `UpdateModelManifest` | `PATCH /model-manifests/{name}` | `idempotent` | `name` | - | `UpdateModelManifestResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidSignalsException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a vehicle model (model manifest). If created vehicles are associated with a vehicle model, it can't be updated. |
| `UpdateSignalCatalog` | `PATCH /signal-catalogs/{name}` | `idempotent` | `name` | - | `UpdateSignalCatalogResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidNodeException`, `InvalidSignalsException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, ... (+1) | Updates a signal catalog. |
| `UpdateStateTemplate` | `PATCH /state-templates/{identifier}` | `idempotent` | `identifier` | - | `UpdateStateTemplateResponse` | `AccessDeniedException`, `InternalServerException`, `InvalidSignalsException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a state template. Access to certain Amazon Web Services IoT FleetWise features is currently gated. |
| `UpdateVehicle` | `PATCH /vehicles/{vehicleName}` | - | `vehicleName` | - | `UpdateVehicleResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `LimitExceededException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a vehicle. Access to certain Amazon Web Services IoT FleetWise features is currently gated. |

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
| `AccessDeniedException` | `structure` | `message` | You don't have sufficient permission to perform this action. |
| `ThrottlingException` | `structure` | `message`, `quotaCode`, `retryAfterSeconds`, `serviceCode` | The request couldn't be completed due to throttling. |
| `ValidationException` | `structure` | `fieldList`, `message`, `reason` | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `InternalServerException` | `structure` | `message`, `retryAfterSeconds` | The request couldn't be completed because the server temporarily failed. |
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The resource wasn't found. |
| `ConflictException` | `structure` | `message`, `resource`, `resourceType` | The request has conflicting operations. |
| `LimitExceededException` | `structure` | `message`, `resourceId`, `resourceType` | A service quota was exceeded. |
| `InvalidSignalsException` | `structure` | `invalidSignals`, `message` | The request couldn't be completed because it contains signals that aren't valid. |
| `DecoderManifestValidationException` | `structure` | `invalidNetworkInterfaces`, `invalidSignals`, `message` | The request couldn't be completed because it contains signal decoders with one or more validation errors. |
| `InvalidNodeException` | `structure` | `invalidNodes`, `message`, `reason` | The specified node type doesn't match the expected node type for a node. |
| `AssociateVehicleFleetRequest` | `structure` | `fleetId`, `vehicleName` | - |
| `AssociateVehicleFleetResponse` | `structure` | - | - |
| `BatchCreateVehicleRequest` | `structure` | `vehicles` | - |
| `BatchCreateVehicleResponse` | `structure` | `errors`, `vehicles` | - |
| `BatchUpdateVehicleRequest` | `structure` | `vehicles` | - |
| `BatchUpdateVehicleResponse` | `structure` | `errors`, `vehicles` | - |
| `CreateCampaignRequest` | `structure` | `collectionScheme`, `compression`, `dataDestinationConfigs`, `dataExtraDimensions`, `dataPartitions`, `description`, `diagnosticsMode`, `expiryTime`, `name`, `postTriggerCollectionDuration`, `priority`, `signalCatalogArn`, ... (+6) | - |
| `CreateCampaignResponse` | `structure` | `arn`, `name` | - |
| `CreateDecoderManifestRequest` | `structure` | `defaultForUnmappedSignals`, `description`, `modelManifestArn`, `name`, `networkInterfaces`, `signalDecoders`, `tags` | - |
| `CreateDecoderManifestResponse` | `structure` | `arn`, `name` | - |
| `CreateFleetRequest` | `structure` | `description`, `fleetId`, `signalCatalogArn`, `tags` | - |
| `CreateFleetResponse` | `structure` | `arn`, `id` | - |
| `CreateModelManifestRequest` | `structure` | `description`, `name`, `nodes`, `signalCatalogArn`, `tags` | - |
| `CreateModelManifestResponse` | `structure` | `arn`, `name` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
