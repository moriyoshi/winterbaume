# Amazon Location Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

"Suite of geospatial services including Maps, Places, Routes, Tracking, and Geofencing"

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Location Service where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Amazon Location Service by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon Location Service workflows in the local mock. Key resources include `ApiKeyResource`, `GenericResource`, `GeofenceCollectionResource`, `JobResource`, `MapResource`.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Get`, `Batch`, `Create`, `Delete` operation families, including `ListDevicePositions`, `ListGeofenceCollections`, `ListGeofences`, `ListKeys`, `GetDevicePosition`, `GetDevicePositionHistory`.

## Service Identity and Protocol

- AWS model slug: `location`
- AWS SDK for Rust slug: `location`
- Model version: `2020-11-19`
- Model file: `vendor/api-models-aws/models/location/service/2020-11-19/location-2020-11-19.json`
- SDK ID: `Location`
- Endpoint prefix: `-`
- ARN namespace: `geo`
- CloudFormation name: `-`
- CloudTrail event source: `-`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (10), `Get` (8), `Batch` (6), `Create` (6), `Delete` (6), `Describe` (6), `Update` (6), `Search` (3).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateTrackerConsumer`, `BatchDeleteDevicePositionHistory`, `BatchDeleteGeofence`, `BatchEvaluateGeofences`, `BatchGetDevicePosition`, `BatchPutGeofence`, `BatchUpdateDevicePosition`, `CreateGeofenceCollection`, `CreateKey`, `CreateMap`, `CreatePlaceIndex`, `CreateRouteCalculator`, `CreateTracker`, `DeleteGeofenceCollection`, `DeleteKey`, `DeleteMap`, `DeletePlaceIndex`, `DeleteRouteCalculator`, `DeleteTracker`, `DisassociateTrackerConsumer`, ... (+9).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `BatchGetDevicePosition`, `CalculateRoute`, `CalculateRouteMatrix`, `DescribeGeofenceCollection`, `DescribeKey`, `DescribeMap`, `DescribePlaceIndex`, `DescribeRouteCalculator`, `DescribeTracker`, `ForecastGeofenceEvents`, `GetDevicePosition`, `GetDevicePositionHistory`, `GetGeofence`, `GetMapGlyphs`, `GetMapSprites`, `GetMapStyleDescriptor`, `GetMapTile`, `GetPlace`, `ListDevicePositions`, `ListGeofenceCollections`, ... (+11).
- Pagination is modelled for 11 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 19 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 60 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `EventBridge`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ApiKeyResource` | `KeyName` | put: `CreateKey`; read: `DescribeKey`; update: `UpdateKey`; delete: `DeleteKey`; list: `ListKeys` | - | - |
| `GenericResource` | `ResourceArn` | - | `ListTagsForResource`, `TagResource`, `UntagResource` | - |
| `GeofenceCollectionResource` | `CollectionName` | put: `CreateGeofenceCollection`; read: `DescribeGeofenceCollection`; update: `UpdateGeofenceCollection`; delete: `DeleteGeofenceCollection`; list: `ListGeofenceCollections` | `BatchDeleteGeofence`, `BatchEvaluateGeofences`, `BatchPutGeofence`, `ForecastGeofenceEvents`, `GetGeofence`, `ListGeofences`, `PutGeofence` | - |
| `JobResource` | `JobId` | - | - | - |
| `MapResource` | `MapName` | put: `CreateMap`; read: `DescribeMap`; update: `UpdateMap`; delete: `DeleteMap`; list: `ListMaps` | `GetMapGlyphs`, `GetMapSprites`, `GetMapStyleDescriptor`, `GetMapTile` | - |
| `PlaceIndexResource` | `IndexName` | put: `CreatePlaceIndex`; read: `DescribePlaceIndex`; update: `UpdatePlaceIndex`; delete: `DeletePlaceIndex`; list: `ListPlaceIndexes` | `GetPlace`, `SearchPlaceIndexForPosition`, `SearchPlaceIndexForSuggestions`, `SearchPlaceIndexForText` | - |
| `RouteCalculatorResource` | `CalculatorName` | put: `CreateRouteCalculator`; read: `DescribeRouteCalculator`; update: `UpdateRouteCalculator`; delete: `DeleteRouteCalculator`; list: `ListRouteCalculators` | `CalculateRoute`, `CalculateRouteMatrix` | - |
| `TrackerResource` | `TrackerName` | put: `CreateTracker`; read: `DescribeTracker`; update: `UpdateTracker`; delete: `DeleteTracker`; list: `ListTrackers` | `AssociateTrackerConsumer`, `BatchDeleteDevicePositionHistory`, `BatchGetDevicePosition`, `BatchUpdateDevicePosition`, `DisassociateTrackerConsumer`, `GetDevicePosition`, `GetDevicePositionHistory`, `ListDevicePositions`, `ListTrackerConsumers`, `VerifyDevicePosition` | - |

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/location/latest/developerguide/features.html
- https://docs.aws.amazon.com/location/previous/developerguide/geofence-tracker-concepts.html

Research outcomes:
- Amazon Location Service provides Maps, Places, Routes, Geofences, Trackers, and Jobs for location-aware applications.
- Map resources define map style and provider-backed tile rendering.
- Place indexes support geocoding, reverse geocoding, and place search.
- Route calculators calculate routes and travel estimates.
- Geofence collections store polygon boundaries and can emit entry and exit events when tracked positions cross boundaries.
- Trackers store device positions and can be linked to geofence collections for automatic evaluation.
- API keys and other auth methods control frontend access to selected resources and actions.

Parity implications:
- Model maps, place indexes, route calculators, geofence collections, geofences, trackers, device positions, API keys, and geofence events separately.
- Position updates should evaluate linked geofence collections and produce enter/exit state transitions.
- Search/route APIs should validate resource/provider capabilities and auth restrictions.

## Operation Groups

### List

- Operations: `ListDevicePositions`, `ListGeofenceCollections`, `ListGeofences`, `ListKeys`, `ListMaps`, `ListPlaceIndexes`, `ListRouteCalculators`, `ListTagsForResource`, `ListTrackerConsumers`, `ListTrackers`
- Traits: `endpoint-bound` (10), `paginated` (9), `readonly` (10)
- Common required input members in this group: `CollectionName`, `ResourceArn`, `TrackerName`

### Get

- Operations: `GetDevicePosition`, `GetDevicePositionHistory`, `GetGeofence`, `GetMapGlyphs`, `GetMapSprites`, `GetMapStyleDescriptor`, `GetMapTile`, `GetPlace`
- Traits: `endpoint-bound` (8), `paginated` (1), `readonly` (8)
- Common required input members in this group: `CollectionName`, `DeviceId`, `FileName`, `FontStack`, `FontUnicodeRange`, `GeofenceId`, `IndexName`, `MapName`, `PlaceId`, `TrackerName`, `X`, `Y`, `Z`

### Batch

- Operations: `BatchDeleteDevicePositionHistory`, `BatchDeleteGeofence`, `BatchEvaluateGeofences`, `BatchGetDevicePosition`, `BatchPutGeofence`, `BatchUpdateDevicePosition`
- Traits: `endpoint-bound` (6), `readonly` (1)
- Common required input members in this group: `CollectionName`, `DeviceIds`, `DevicePositionUpdates`, `Entries`, `GeofenceIds`, `TrackerName`, `Updates`

### Create

- Operations: `CreateGeofenceCollection`, `CreateKey`, `CreateMap`, `CreatePlaceIndex`, `CreateRouteCalculator`, `CreateTracker`
- Traits: `endpoint-bound` (6), `idempotent` (6)
- Common required input members in this group: `CalculatorName`, `CollectionName`, `Configuration`, `DataSource`, `IndexName`, `KeyName`, `MapName`, `Restrictions`, `TrackerName`

### Delete

- Operations: `DeleteGeofenceCollection`, `DeleteKey`, `DeleteMap`, `DeletePlaceIndex`, `DeleteRouteCalculator`, `DeleteTracker`
- Traits: `endpoint-bound` (6), `idempotent` (6)
- Common required input members in this group: `CalculatorName`, `CollectionName`, `IndexName`, `KeyName`, `MapName`, `TrackerName`

### Describe

- Operations: `DescribeGeofenceCollection`, `DescribeKey`, `DescribeMap`, `DescribePlaceIndex`, `DescribeRouteCalculator`, `DescribeTracker`
- Traits: `endpoint-bound` (6), `readonly` (6)
- Common required input members in this group: `CalculatorName`, `CollectionName`, `IndexName`, `KeyName`, `MapName`, `TrackerName`

### Update

- Operations: `UpdateGeofenceCollection`, `UpdateKey`, `UpdateMap`, `UpdatePlaceIndex`, `UpdateRouteCalculator`, `UpdateTracker`
- Traits: `endpoint-bound` (6), `idempotent` (6)
- Common required input members in this group: `CalculatorName`, `CollectionName`, `IndexName`, `KeyName`, `MapName`, `TrackerName`

### Search

- Operations: `SearchPlaceIndexForPosition`, `SearchPlaceIndexForSuggestions`, `SearchPlaceIndexForText`
- Traits: `endpoint-bound` (3), `readonly` (3)
- Common required input members in this group: `IndexName`, `Position`, `Text`

### Calculate

- Operations: `CalculateRoute`, `CalculateRouteMatrix`
- Traits: `endpoint-bound` (2), `readonly` (2)
- Common required input members in this group: `CalculatorName`, `DeparturePosition`, `DeparturePositions`, `DestinationPosition`, `DestinationPositions`

### Associate

- Operations: `AssociateTrackerConsumer`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `ConsumerArn`, `TrackerName`

### Disassociate

- Operations: `DisassociateTrackerConsumer`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `ConsumerArn`, `TrackerName`

### Forecast

- Operations: `ForecastGeofenceEvents`
- Traits: `endpoint-bound` (1), `paginated` (1), `readonly` (1)
- Common required input members in this group: `CollectionName`, `DeviceState`

### Put

- Operations: `PutGeofence`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `CollectionName`, `GeofenceId`, `Geometry`

### Tag

- Operations: `TagResource`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Traits: `endpoint-bound` (1), `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `TagKeys`

### Verify

- Operations: `VerifyDevicePosition`
- Traits: `endpoint-bound` (1)
- Common required input members in this group: `DeviceState`, `TrackerName`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateTrackerConsumer` | `POST /tracking/v0/trackers/{TrackerName}/consumers` | `endpoint-bound` | `ConsumerArn`, `TrackerName` | - | `AssociateTrackerConsumerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an association between a geofence collection and a tracker resource. This allows the tracker resource to communicate location data to the linked geofence collection. |
| `BatchDeleteDevicePositionHistory` | `POST /tracking/v0/trackers/{TrackerName}/delete-positions` | `endpoint-bound` | `DeviceIds`, `TrackerName` | - | `BatchDeleteDevicePositionHistoryResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the position history of one or more devices from a tracker resource. |
| `BatchDeleteGeofence` | `POST /geofencing/v0/collections/{CollectionName}/delete-geofences` | `endpoint-bound` | `CollectionName`, `GeofenceIds` | - | `BatchDeleteGeofenceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a batch of geofences from a geofence collection. This operation deletes the resource permanently. |
| `BatchEvaluateGeofences` | `POST /geofencing/v0/collections/{CollectionName}/positions` | `endpoint-bound` | `CollectionName`, `DevicePositionUpdates` | - | `BatchEvaluateGeofencesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Evaluates device positions against the geofence geometries from a given geofence collection. This operation always returns an empty response because geofences are asynchronously evaluated. |
| `BatchGetDevicePosition` | `POST /tracking/v0/trackers/{TrackerName}/get-positions` | `readonly`, `endpoint-bound` | `DeviceIds`, `TrackerName` | - | `BatchGetDevicePositionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the latest device positions for requested devices. |
| `BatchPutGeofence` | `POST /geofencing/v0/collections/{CollectionName}/put-geofences` | `endpoint-bound` | `CollectionName`, `Entries` | - | `BatchPutGeofenceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | A batch request for storing geofence geometries into a given geofence collection, or updates the geometry of an existing geofence if a geofence ID is included in the request. |
| `BatchUpdateDevicePosition` | `POST /tracking/v0/trackers/{TrackerName}/positions` | `endpoint-bound` | `TrackerName`, `Updates` | - | `BatchUpdateDevicePositionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Uploads position update data for one or more devices to a tracker resource (up to 10 devices per batch). Amazon Location uses the data when it reports the last known device position and position history. |
| `CalculateRoute` | `POST /routes/v0/calculators/{CalculatorName}/calculate/route` | `readonly`, `endpoint-bound` | `CalculatorName`, `DeparturePosition`, `DestinationPosition` | - | `CalculateRouteResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend you upgrade to `CalculateRoutes` or `CalculateIsolines` unless you require Grab data. |
| `CalculateRouteMatrix` | `POST /routes/v0/calculators/{CalculatorName}/calculate/route-matrix` | `readonly`, `endpoint-bound` | `CalculatorName`, `DeparturePositions`, `DestinationPositions` | - | `CalculateRouteMatrixResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend you upgrade to the V2 `CalculateRouteMatrix` unless you require Grab data. |
| `CreateGeofenceCollection` | `POST /geofencing/v0/collections` | `idempotent`, `endpoint-bound` | `CollectionName` | - | `CreateGeofenceCollectionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a geofence collection, which manages and stores geofences. |
| `CreateKey` | `POST /metadata/v0/keys` | `idempotent`, `endpoint-bound` | `KeyName`, `Restrictions` | - | `CreateKeyResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates an API key resource in your Amazon Web Services account, which lets you grant actions for Amazon Location resources to the API key bearer. For more information, see Use API keys to authenticate in the Amazon Location Service Developer Guide . |
| `CreateMap` | `POST /maps/v0/maps` | `idempotent`, `endpoint-bound` | `Configuration`, `MapName` | - | `CreateMapResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend upgrading to the Maps API V2 unless you require `Grab` data. |
| `CreatePlaceIndex` | `POST /places/v0/indexes` | `idempotent`, `endpoint-bound` | `DataSource`, `IndexName` | - | `CreatePlaceIndexResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend you upgrade to the Places API V2 unless you require Grab data. |
| `CreateRouteCalculator` | `POST /routes/v0/calculators` | `idempotent`, `endpoint-bound` | `CalculatorName`, `DataSource` | - | `CreateRouteCalculatorResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend you upgrade to the Routes API V2 unless you require Grab data. |
| `CreateTracker` | `POST /tracking/v0/trackers` | `idempotent`, `endpoint-bound` | `TrackerName` | - | `CreateTrackerResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a tracker resource in your Amazon Web Services account, which lets you retrieve current and historical location of devices. |
| `DeleteGeofenceCollection` | `DELETE /geofencing/v0/collections/{CollectionName}` | `idempotent`, `endpoint-bound` | `CollectionName` | - | `DeleteGeofenceCollectionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a geofence collection from your Amazon Web Services account. This operation deletes the resource permanently. |
| `DeleteKey` | `DELETE /metadata/v0/keys/{KeyName}` | `idempotent`, `endpoint-bound` | `KeyName` | - | `DeleteKeyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes the specified API key. The API key must have been deactivated more than 90 days previously. |
| `DeleteMap` | `DELETE /maps/v0/maps/{MapName}` | `idempotent`, `endpoint-bound` | `MapName` | - | `DeleteMapResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend upgrading to the Maps API V2 unless you require `Grab` data. |
| `DeletePlaceIndex` | `DELETE /places/v0/indexes/{IndexName}` | `idempotent`, `endpoint-bound` | `IndexName` | - | `DeletePlaceIndexResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend you upgrade to the Places API V2 unless you require Grab data. |
| `DeleteRouteCalculator` | `DELETE /routes/v0/calculators/{CalculatorName}` | `idempotent`, `endpoint-bound` | `CalculatorName` | - | `DeleteRouteCalculatorResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend you upgrade to the Routes API V2 unless you require Grab data. |
| `DeleteTracker` | `DELETE /tracking/v0/trackers/{TrackerName}` | `idempotent`, `endpoint-bound` | `TrackerName` | - | `DeleteTrackerResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a tracker resource from your Amazon Web Services account. This operation deletes the resource permanently. |
| `DescribeGeofenceCollection` | `GET /geofencing/v0/collections/{CollectionName}` | `readonly`, `endpoint-bound` | `CollectionName` | - | `DescribeGeofenceCollectionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the geofence collection details. |
| `DescribeKey` | `GET /metadata/v0/keys/{KeyName}` | `readonly`, `endpoint-bound` | `KeyName` | - | `DescribeKeyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the API key resource details. For more information, see Use API keys to authenticate in the Amazon Location Service Developer Guide . |
| `DescribeMap` | `GET /maps/v0/maps/{MapName}` | `readonly`, `endpoint-bound` | `MapName` | - | `DescribeMapResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend upgrading to the Maps API V2 unless you require `Grab` data. |
| `DescribePlaceIndex` | `GET /places/v0/indexes/{IndexName}` | `readonly`, `endpoint-bound` | `IndexName` | - | `DescribePlaceIndexResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend you upgrade to the Places API V2 unless you require Grab data. |
| `DescribeRouteCalculator` | `GET /routes/v0/calculators/{CalculatorName}` | `readonly`, `endpoint-bound` | `CalculatorName` | - | `DescribeRouteCalculatorResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend you upgrade to the Routes API V2 unless you require Grab data. |
| `DescribeTracker` | `GET /tracking/v0/trackers/{TrackerName}` | `readonly`, `endpoint-bound` | `TrackerName` | - | `DescribeTrackerResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the tracker resource details. |
| `DisassociateTrackerConsumer` | `DELETE /tracking/v0/trackers/{TrackerName}/consumers/{ConsumerArn}` | `endpoint-bound` | `ConsumerArn`, `TrackerName` | - | `DisassociateTrackerConsumerResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes the association between a tracker resource and a geofence collection. Once you unlink a tracker resource from a geofence collection, the tracker positions will no longer be automatically evaluated against geofences. |
| `ForecastGeofenceEvents` | `POST /geofencing/v0/collections/{CollectionName}/forecast-geofence-events` | `readonly`, `paginated`, `endpoint-bound` | `CollectionName`, `DeviceState` | - | `ForecastGeofenceEventsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This action forecasts future geofence events that are likely to occur within a specified time horizon if a device continues moving at its current speed. Each forecasted event is associated with a geofence from a provided geofence collection. |
| `GetDevicePosition` | `GET /tracking/v0/trackers/{TrackerName}/devices/{DeviceId}/positions/latest` | `readonly`, `endpoint-bound` | `DeviceId`, `TrackerName` | - | `GetDevicePositionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a device's most recent position according to its sample time. Device positions are deleted after 30 days. |
| `GetDevicePositionHistory` | `POST /tracking/v0/trackers/{TrackerName}/devices/{DeviceId}/list-positions` | `readonly`, `paginated`, `endpoint-bound` | `DeviceId`, `TrackerName` | - | `GetDevicePositionHistoryResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the device position history from a tracker resource within a specified range of time. Device positions are deleted after 30 days. |
| `GetGeofence` | `GET /geofencing/v0/collections/{CollectionName}/geofences/{GeofenceId}` | `readonly`, `endpoint-bound` | `CollectionName`, `GeofenceId` | - | `GetGeofenceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the geofence details from a geofence collection. The returned geometry will always match the geometry format used when the geofence was created. |
| `GetMapGlyphs` | `GET /maps/v0/maps/{MapName}/glyphs/{FontStack}/{FontUnicodeRange}` | `readonly`, `endpoint-bound` | `FontStack`, `FontUnicodeRange`, `MapName` | - | `GetMapGlyphsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend upgrading to `GetGlyphs` unless you require `Grab` data. |
| `GetMapSprites` | `GET /maps/v0/maps/{MapName}/sprites/{FileName}` | `readonly`, `endpoint-bound` | `FileName`, `MapName` | - | `GetMapSpritesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend upgrading to `GetSprites` unless you require `Grab` data. |
| `GetMapStyleDescriptor` | `GET /maps/v0/maps/{MapName}/style-descriptor` | `readonly`, `endpoint-bound` | `MapName` | - | `GetMapStyleDescriptorResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend upgrading to `GetStyleDescriptor` unless you require `Grab` data. |
| `GetMapTile` | `GET /maps/v0/maps/{MapName}/tiles/{Z}/{X}/{Y}` | `readonly`, `endpoint-bound` | `MapName`, `X`, `Y`, `Z` | - | `GetMapTileResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend upgrading to `GetTile` unless you require `Grab` data. |
| `GetPlace` | `GET /places/v0/indexes/{IndexName}/places/{PlaceId}` | `readonly`, `endpoint-bound` | `IndexName`, `PlaceId` | - | `GetPlaceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend you upgrade to the V2 `GetPlace` operation unless you require Grab data. |
| `ListDevicePositions` | `POST /tracking/v0/trackers/{TrackerName}/list-positions` | `readonly`, `paginated`, `endpoint-bound` | `TrackerName` | - | `ListDevicePositionsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | A batch request to retrieve all device positions. |
| `ListGeofenceCollections` | `POST /geofencing/v0/list-collections` | `readonly`, `paginated`, `endpoint-bound` | - | - | `ListGeofenceCollectionsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists geofence collections in your Amazon Web Services account. |
| `ListGeofences` | `POST /geofencing/v0/collections/{CollectionName}/list-geofences` | `readonly`, `paginated`, `endpoint-bound` | `CollectionName` | - | `ListGeofencesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists geofences stored in a given geofence collection. |
| `ListKeys` | `POST /metadata/v0/list-keys` | `readonly`, `paginated`, `endpoint-bound` | - | - | `ListKeysResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists API key resources in your Amazon Web Services account. For more information, see Use API keys to authenticate in the Amazon Location Service Developer Guide . |
| `ListMaps` | `POST /maps/v0/list-maps` | `readonly`, `paginated`, `endpoint-bound` | - | - | `ListMapsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend upgrading to the Maps API V2 unless you require `Grab` data. |
| `ListPlaceIndexes` | `POST /places/v0/list-indexes` | `readonly`, `paginated`, `endpoint-bound` | - | - | `ListPlaceIndexesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend you upgrade to the Places API V2 unless you require Grab data. |
| `ListRouteCalculators` | `POST /routes/v0/list-calculators` | `readonly`, `paginated`, `endpoint-bound` | - | - | `ListRouteCalculatorsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend you upgrade to the Routes API V2 unless you require Grab data. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly`, `endpoint-bound` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of tags that are applied to the specified Amazon Location resource. |
| `ListTrackerConsumers` | `POST /tracking/v0/trackers/{TrackerName}/list-consumers` | `readonly`, `paginated`, `endpoint-bound` | `TrackerName` | - | `ListTrackerConsumersResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists geofence collections currently associated to the given tracker resource. |
| `ListTrackers` | `POST /tracking/v0/list-trackers` | `readonly`, `paginated`, `endpoint-bound` | - | - | `ListTrackersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists tracker resources in your Amazon Web Services account. |
| `PutGeofence` | `PUT /geofencing/v0/collections/{CollectionName}/geofences/{GeofenceId}` | `endpoint-bound` | `CollectionName`, `GeofenceId`, `Geometry` | - | `PutGeofenceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stores a geofence geometry in a given geofence collection, or updates the geometry of an existing geofence if a geofence ID is included in the request. |
| `SearchPlaceIndexForPosition` | `POST /places/v0/indexes/{IndexName}/search/position` | `readonly`, `endpoint-bound` | `IndexName`, `Position` | - | `SearchPlaceIndexForPositionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend you upgrade to `ReverseGeocode` or `SearchNearby` unless you require Grab data. |
| `SearchPlaceIndexForSuggestions` | `POST /places/v0/indexes/{IndexName}/search/suggestions` | `readonly`, `endpoint-bound` | `IndexName`, `Text` | - | `SearchPlaceIndexForSuggestionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend you upgrade to `Suggest` or `Autocomplete` unless you require Grab data. |
| `SearchPlaceIndexForText` | `POST /places/v0/indexes/{IndexName}/search/text` | `readonly`, `endpoint-bound` | `IndexName`, `Text` | - | `SearchPlaceIndexForTextResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend you upgrade to `Geocode` or `SearchText` unless you require Grab data. |
| `TagResource` | `POST /tags/{ResourceArn}` | `endpoint-bound` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Assigns one or more tags (key-value pairs) to the specified Amazon Location Service resource. Tags can help you organize and categorize your resources. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent`, `endpoint-bound` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes one or more tags from the specified Amazon Location resource. |
| `UpdateGeofenceCollection` | `PATCH /geofencing/v0/collections/{CollectionName}` | `idempotent`, `endpoint-bound` | `CollectionName` | - | `UpdateGeofenceCollectionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the specified properties of a given geofence collection. |
| `UpdateKey` | `PATCH /metadata/v0/keys/{KeyName}` | `idempotent`, `endpoint-bound` | `KeyName` | - | `UpdateKeyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the specified properties of a given API key resource. |
| `UpdateMap` | `PATCH /maps/v0/maps/{MapName}` | `idempotent`, `endpoint-bound` | `MapName` | - | `UpdateMapResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend upgrading to the Maps API V2 unless you require `Grab` data. |
| `UpdatePlaceIndex` | `PATCH /places/v0/indexes/{IndexName}` | `idempotent`, `endpoint-bound` | `IndexName` | - | `UpdatePlaceIndexResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend you upgrade to the Places API V2 unless you require Grab data. |
| `UpdateRouteCalculator` | `PATCH /routes/v0/calculators/{CalculatorName}` | `idempotent`, `endpoint-bound` | `CalculatorName` | - | `UpdateRouteCalculatorResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This operation is no longer current and may be deprecated in the future. We recommend you upgrade to the Routes API V2 unless you require Grab data. |
| `UpdateTracker` | `PATCH /tracking/v0/trackers/{TrackerName}` | `idempotent`, `endpoint-bound` | `TrackerName` | - | `UpdateTrackerResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the specified properties of a given tracker resource. |
| `VerifyDevicePosition` | `POST /tracking/v0/trackers/{TrackerName}/positions/verify` | `endpoint-bound` | `DeviceState`, `TrackerName` | - | `VerifyDevicePositionResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Verifies the integrity of the device's position by determining if it was reported behind a proxy, and by comparing it to an inferred position estimated based on the device's state. The Location Integrity SDK provides enhanced features related to device... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | The request was denied because of insufficient access or permissions. |
| `InternalServerException` | `structure` | `Message` | The request has failed to process because of an unknown server error, exception, or failure. |
| `ThrottlingException` | `structure` | `Message` | The request was denied because of request throttling. |
| `ValidationException` | `structure` | `FieldList`, `Message`, `Reason` | The input failed to meet the constraints specified by the AWS service. |
| `ResourceNotFoundException` | `structure` | `Message` | The resource that you've entered was not found in your AWS account. |
| `ConflictException` | `structure` | `Message` | The request was unsuccessful because of a conflict. |
| `ServiceQuotaExceededException` | `structure` | `Message` | The operation was denied because the request would exceed the maximum quota set for Amazon Location Service. |
| `AssociateTrackerConsumerRequest` | `structure` | `ConsumerArn`, `TrackerName` | - |
| `AssociateTrackerConsumerResponse` | `structure` | - | - |
| `BatchDeleteDevicePositionHistoryRequest` | `structure` | `DeviceIds`, `TrackerName` | - |
| `BatchDeleteDevicePositionHistoryResponse` | `structure` | `Errors` | - |
| `BatchDeleteGeofenceRequest` | `structure` | `CollectionName`, `GeofenceIds` | - |
| `BatchDeleteGeofenceResponse` | `structure` | `Errors` | - |
| `BatchEvaluateGeofencesRequest` | `structure` | `CollectionName`, `DevicePositionUpdates` | - |
| `BatchEvaluateGeofencesResponse` | `structure` | `Errors` | - |
| `BatchGetDevicePositionRequest` | `structure` | `DeviceIds`, `TrackerName` | - |
| `BatchGetDevicePositionResponse` | `structure` | `DevicePositions`, `Errors` | - |
| `BatchPutGeofenceRequest` | `structure` | `CollectionName`, `Entries` | - |
| `BatchPutGeofenceResponse` | `structure` | `Errors`, `Successes` | - |
| `BatchUpdateDevicePositionRequest` | `structure` | `TrackerName`, `Updates` | - |
| `BatchUpdateDevicePositionResponse` | `structure` | `Errors` | - |
| `CalculateRouteRequest` | `structure` | `ArrivalTime`, `CalculatorName`, `CarModeOptions`, `DepartNow`, `DeparturePosition`, `DepartureTime`, `DestinationPosition`, `DistanceUnit`, `IncludeLegGeometry`, `Key`, `OptimizeFor`, `TravelMode`, ... (+2) | - |
| `CalculateRouteResponse` | `structure` | `Legs`, `Summary` | Returns the result of the route calculation. |
| `CalculateRouteMatrixRequest` | `structure` | `CalculatorName`, `CarModeOptions`, `DepartNow`, `DeparturePositions`, `DepartureTime`, `DestinationPositions`, `DistanceUnit`, `Key`, `TravelMode`, `TruckModeOptions` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
