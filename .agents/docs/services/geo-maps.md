# Amazon Location Service Maps V2

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Integrate high-quality base map data into your applications using MapLibre. Capabilities include: Access to comprehensive base map data, allowing you to tailor the map display to your specific needs. Multiple pre-designed map styles suited for various application types, such as navigation, logistics, or data visualization. Generation of static map images for scenarios where interactive maps aren't suitable, such as: Embedding in emails or documents Displaying in low-bandwidth environments Creating printable maps Enhancing application performance by reducing client-side rendering

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Location Service Maps V2 workflows in the local mock. Key resources include `ProviderResource`.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Get` operation families, including `GetGlyphs`, `GetSprites`, `GetStaticMap`, `GetStyleDescriptor`.

## Service Identity and Protocol

- AWS model slug: `geo-maps`
- AWS SDK for Rust slug: `geomaps`
- Model version: `2020-11-19`
- Model file: `vendor/api-models-aws/models/geo-maps/service/2020-11-19/geo-maps-2020-11-19.json`
- SDK ID: `Geo Maps`
- Endpoint prefix: `-`
- ARN namespace: `geo-maps`
- CloudFormation name: `-`
- CloudTrail event source: `geo-maps.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (5).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetGlyphs`, `GetSprites`, `GetStaticMap`, `GetStyleDescriptor`, `GetTile`.
- 2 operations declare modelled service errors; parity work should map exact error names and retryability where documented.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ProviderResource` | - | - | `GetGlyphs`, `GetSprites`, `GetStaticMap`, `GetStyleDescriptor`, `GetTile` | - |
## Operation Groups

### Get

- Operations: `GetGlyphs`, `GetSprites`, `GetStaticMap`, `GetStyleDescriptor`, `GetTile`
- Traits: `readonly` (5)
- Common required input members in this group: `ColorScheme`, `FileName`, `FontStack`, `FontUnicodeRange`, `Height`, `Style`, `Tileset`, `Variant`, `Width`, `X`, `Y`, `Z`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetGlyphs` | `GET /glyphs/{FontStack}/{FontUnicodeRange}` | `readonly` | `FontStack`, `FontUnicodeRange` | - | `GetGlyphsResponse` | - | `GetGlyphs` returns the map's glyphs. For more information, see Style labels with glyphs in the Amazon Location Service Developer Guide . |
| `GetSprites` | `GET /styles/{Style}/{ColorScheme}/{Variant}/sprites/{FileName}` | `readonly` | `ColorScheme`, `FileName`, `Style`, `Variant` | - | `GetSpritesResponse` | - | `GetSprites` returns the map's sprites. For more information, see Style iconography with sprites in the Amazon Location Service Developer Guide . |
| `GetStaticMap` | `GET /static/{FileName}` | `readonly` | `FileName`, `Height`, `Width` | - | `GetStaticMapResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | `GetStaticMap` provides high-quality static map images with customizable options. You can modify the map's appearance and overlay additional information. |
| `GetStyleDescriptor` | `GET /styles/{Style}/descriptor` | `readonly` | `Style` | - | `GetStyleDescriptorResponse` | - | `GetStyleDescriptor` returns information about the style. For more information, see Style dynamic maps in the Amazon Location Service Developer Guide . |
| `GetTile` | `GET /tiles/{Tileset}/{Z}/{X}/{Y}` | `readonly` | `Tileset`, `X`, `Y`, `Z` | - | `GetTileResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | `GetTile` returns a tile. Map tiles are used by clients to render a map. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | The request was denied because of insufficient access or permissions. |
| `InternalServerException` | `structure` | `Message` | The request processing has failed because of an unknown error, exception or failure. |
| `ThrottlingException` | `structure` | `Message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `FieldList`, `Message`, `Reason` | The input fails to satisfy the constraints specified by an AWS service. |
| `GetGlyphsRequest` | `structure` | `FontStack`, `FontUnicodeRange` | - |
| `GetGlyphsResponse` | `structure` | `Blob`, `CacheControl`, `ContentType`, `ETag` | - |
| `GetSpritesRequest` | `structure` | `ColorScheme`, `FileName`, `Style`, `Variant` | - |
| `GetSpritesResponse` | `structure` | `Blob`, `CacheControl`, `ContentType`, `ETag` | - |
| `GetStaticMapRequest` | `structure` | `BoundedPositions`, `BoundingBox`, `Center`, `ColorScheme`, `CompactOverlay`, `CropLabels`, `FileName`, `GeoJsonOverlay`, `Height`, `Key`, `LabelSize`, `Language`, ... (+8) | - |
| `GetStaticMapResponse` | `structure` | `Blob`, `CacheControl`, `ContentType`, `ETag`, `PricingBucket` | - |
| `GetStyleDescriptorRequest` | `structure` | `Buildings`, `ColorScheme`, `ContourDensity`, `Key`, `PoliticalView`, `Style`, `Terrain`, `Traffic`, `TravelModes` | - |
| `GetStyleDescriptorResponse` | `structure` | `Blob`, `CacheControl`, `ContentType`, `ETag` | - |
| `GetTileRequest` | `structure` | `AdditionalFeatures`, `Key`, `Tileset`, `X`, `Y`, `Z` | - |
| `GetTileResponse` | `structure` | `Blob`, `CacheControl`, `ContentType`, `ETag`, `PricingBucket` | - |
| `ResourceNotFoundException` | `structure` | `Message` | Exception thrown when the associated resource could not be found. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
