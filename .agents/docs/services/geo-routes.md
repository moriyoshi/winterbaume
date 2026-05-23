# Amazon Location Service Routes V2

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

With the Amazon Location Routes API you can calculate routes and estimate travel time based on up-to-date road network and live traffic information. Calculate optimal travel routes and estimate travel times using up-to-date road network and traffic data. Key features include: Point-to-point routing with estimated travel time, distance, and turn-by-turn directions Multi-point route optimization to minimize travel time or distance Route matrices for efficient multi-destination planning Isoline calculations to determine reachable areas within specified time or distance thresholds Map-matching to align GPS traces with the road network

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Location Service Routes V2 workflows in the local mock. Key resources include `ProviderResource`.
- From the operation surface: model workflows exposed by the modelled operation families across the `Calculate`, `Optimize`, `Snap` operation families, including `CalculateIsolines`, `CalculateRouteMatrix`, `CalculateRoutes`, `OptimizeWaypoints`, `SnapToRoads`.

## Service Identity and Protocol

- AWS model slug: `geo-routes`
- AWS SDK for Rust slug: `georoutes`
- Model version: `2020-11-19`
- Model file: `vendor/api-models-aws/models/geo-routes/service/2020-11-19/geo-routes-2020-11-19.json`
- SDK ID: `Geo Routes`
- Endpoint prefix: `-`
- ARN namespace: `geo-routes`
- CloudFormation name: `-`
- CloudTrail event source: `geo-routes.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Calculate` (3), `Optimize` (1), `Snap` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `CalculateIsolines`, `CalculateRouteMatrix`, `CalculateRoutes`, `OptimizeWaypoints`, `SnapToRoads`.
- 5 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ProviderResource` | - | - | `CalculateIsolines`, `CalculateRouteMatrix`, `CalculateRoutes`, `OptimizeWaypoints`, `SnapToRoads` | - |
## Operation Groups

### Calculate

- Operations: `CalculateIsolines`, `CalculateRouteMatrix`, `CalculateRoutes`
- Traits: `readonly` (3)
- Common required input members in this group: `Destination`, `Destinations`, `Origin`, `Origins`, `RoutingBoundary`, `Thresholds`

### Optimize

- Operations: `OptimizeWaypoints`
- Traits: `readonly` (1)
- Common required input members in this group: `Origin`

### Snap

- Operations: `SnapToRoads`
- Traits: `readonly` (1)
- Common required input members in this group: `TracePoints`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CalculateIsolines` | `POST /isolines` | `readonly` | `Thresholds` | - | `CalculateIsolinesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Use the `CalculateIsolines` action to find service areas that can be reached in a given threshold of time, distance. |
| `CalculateRouteMatrix` | `POST /route-matrix` | `readonly` | `Destinations`, `Origins`, `RoutingBoundary` | - | `CalculateRouteMatrixResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Use `CalculateRouteMatrix` to compute results for all pairs of Origins to Destinations. Each row corresponds to one entry in Origins. |
| `CalculateRoutes` | `POST /routes` | `readonly` | `Destination`, `Origin` | - | `CalculateRoutesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | `CalculateRoutes` computes routes given the following required parameters: `Origin` and `Destination`. |
| `OptimizeWaypoints` | `POST /optimize-waypoints` | `readonly` | `Origin` | - | `OptimizeWaypointsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | `OptimizeWaypoints` calculates the optimal order to travel between a set of waypoints to minimize either the travel time or the distance travelled during the journey, based on road network restrictions and the traffic pattern data. |
| `SnapToRoads` | `POST /snap-to-roads` | `readonly` | `TracePoints` | - | `SnapToRoadsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | `SnapToRoads` matches GPS trace to roads most likely traveled on. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You don't have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `Message` | The request processing has failed because of an unknown error, exception or failure. |
| `ThrottlingException` | `structure` | `Message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `FieldList`, `Message`, `Reason` | The input fails to satisfy the constraints specified by an AWS service. |
| `CalculateIsolinesRequest` | `structure` | `Allow`, `ArrivalTime`, `Avoid`, `DepartNow`, `DepartureTime`, `Destination`, `DestinationOptions`, `IsolineGeometryFormat`, `IsolineGranularity`, `Key`, `OptimizeIsolineFor`, `OptimizeRoutingFor`, ... (+6) | - |
| `CalculateIsolinesResponse` | `structure` | `ArrivalTime`, `DepartureTime`, `IsolineGeometryFormat`, `Isolines`, `PricingBucket`, `SnappedDestination`, `SnappedOrigin` | - |
| `CalculateRouteMatrixRequest` | `structure` | `Allow`, `Avoid`, `DepartNow`, `DepartureTime`, `Destinations`, `Exclude`, `Key`, `OptimizeRoutingFor`, `Origins`, `RoutingBoundary`, `Traffic`, `TravelMode`, ... (+1) | - |
| `CalculateRouteMatrixResponse` | `structure` | `ErrorCount`, `PricingBucket`, `RouteMatrix`, `RoutingBoundary` | - |
| `CalculateRoutesRequest` | `structure` | `Allow`, `ArrivalTime`, `Avoid`, `DepartNow`, `DepartureTime`, `Destination`, `DestinationOptions`, `Driver`, `Exclude`, `InstructionsMeasurementSystem`, `Key`, `Languages`, ... (+13) | - |
| `CalculateRoutesResponse` | `structure` | `LegGeometryFormat`, `Notices`, `PricingBucket`, `Routes` | - |
| `OptimizeWaypointsRequest` | `structure` | `Avoid`, `Clustering`, `DepartureTime`, `Destination`, `DestinationOptions`, `Driver`, `Exclude`, `Key`, `OptimizeSequencingFor`, `Origin`, `OriginOptions`, `Traffic`, ... (+3) | - |
| `OptimizeWaypointsResponse` | `structure` | `Connections`, `Distance`, `Duration`, `ImpedingWaypoints`, `OptimizedWaypoints`, `PricingBucket`, `TimeBreakdown` | - |
| `SnapToRoadsRequest` | `structure` | `Key`, `SnapRadius`, `SnappedGeometryFormat`, `TracePoints`, `TravelMode`, `TravelModeOptions` | - |
| `SnapToRoadsResponse` | `structure` | `Notices`, `PricingBucket`, `SnappedGeometry`, `SnappedGeometryFormat`, `SnappedTracePoints` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
