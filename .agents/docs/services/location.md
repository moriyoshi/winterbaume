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

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | The request was denied because of insufficient access or permissions. Check with an administrator to verify your permissions. |
| `ConflictException` | `structure` | Message | The request was unsuccessful because of a conflict. |
| `InternalServerException` | `structure` | Message | The request has failed to process because of an unknown server error, exception, or failure. |
| `ResourceNotFoundException` | `structure` | Message | The resource that you've entered was not found in your AWS account. |
| `ServiceQuotaExceededException` | `structure` | Message | The operation was denied because the request would exceed the maximum quota set for Amazon Location Service. |
| `ThrottlingException` | `structure` | Message | The request was denied because of request throttling. |
| `ValidationException` | `structure` | Message, Reason, FieldList | The input failed to meet the constraints specified by the AWS service. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
