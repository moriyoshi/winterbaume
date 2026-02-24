# Amazon Location Service Places V2

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Places API enables powerful location search and geocoding capabilities for your applications, offering global coverage with rich, detailed information. Key features include: Forward and reverse geocoding for addresses and coordinates Comprehensive place searches with detailed information, including: Business names and addresses Contact information Hours of operation POI (Points of Interest) categories Food types for restaurants Chain affiliation for relevant businesses Global data coverage with a wide range of POI categories Regular data updates to ensure accuracy and relevance

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Location Service Places V2 workflows in the local mock. Key resources include `ProviderResource`.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Search`, `Autocomplete`, `Geocode`, `Get`, `Reverse` operation families, including `SearchNearby`, `SearchText`, `Autocomplete`, `Geocode`, `GetPlace`, `ReverseGeocode`.

## Service Identity and Protocol

- AWS model slug: `geo-places`
- AWS SDK for Rust slug: `geoplaces`
- Model version: `2020-11-19`
- Model file: `vendor/api-models-aws/models/geo-places/service/2020-11-19/geo-places-2020-11-19.json`
- SDK ID: `Geo Places`
- Endpoint prefix: `-`
- ARN namespace: `geo-places`
- CloudFormation name: `-`
- CloudTrail event source: `geo-places.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Search` (2), `Autocomplete` (1), `Geocode` (1), `Get` (1), `Reverse` (1), `Suggest` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `Autocomplete`, `Geocode`, `GetPlace`, `ReverseGeocode`, `SearchNearby`, `SearchText`, `Suggest`.
- 7 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `ProviderResource` | - | - | `Autocomplete`, `Geocode`, `GetPlace`, `ReverseGeocode`, `SearchNearby`, `SearchText`, `Suggest` | - |
## Operation Groups

### Search

- Operations: `SearchNearby`, `SearchText`
- Traits: `readonly` (2)
- Common required input members in this group: `QueryPosition`

### Autocomplete

- Operations: `Autocomplete`
- Traits: `readonly` (1)
- Common required input members in this group: `QueryText`

### Geocode

- Operations: `Geocode`
- Traits: `readonly` (1)

### Get

- Operations: `GetPlace`
- Traits: `readonly` (1)
- Common required input members in this group: `PlaceId`

### Reverse

- Operations: `ReverseGeocode`
- Traits: `readonly` (1)
- Common required input members in this group: `QueryPosition`

### Suggest

- Operations: `Suggest`
- Traits: `readonly` (1)
- Common required input members in this group: `QueryText`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `Autocomplete` | `POST /autocomplete` | `readonly` | `QueryText` | - | `AutocompleteResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | `Autocomplete` completes potential places and addresses as the user types, based on the partial input. The API enhances the efficiency and accuracy of address by completing query based on a few entered keystrokes. |
| `Geocode` | `POST /geocode` | `readonly` | - | - | `GeocodeResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | `Geocode` converts a textual address or place into geographic coordinates. You can obtain geographic coordinates, address component, and other related information. |
| `GetPlace` | `GET /place/{PlaceId}` | `readonly` | `PlaceId` | - | `GetPlaceResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | `GetPlace` finds a place by its unique ID. A `PlaceId` is returned by other place operations. |
| `ReverseGeocode` | `POST /reverse-geocode` | `readonly` | `QueryPosition` | - | `ReverseGeocodeResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | `ReverseGeocode` converts geographic coordinates into a human-readable address or place. You can obtain address component, and other related information such as place type, category, street information. |
| `SearchNearby` | `POST /search-nearby` | `readonly` | `QueryPosition` | - | `SearchNearbyResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | `SearchNearby` queries for points of interest within a radius from a central coordinates, returning place results with optional filters such as categories, business chains, food types and more. The API returns details such as a place name, address, phone... |
| `SearchText` | `POST /search-text` | `readonly` | - | - | `SearchTextResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | `SearchText` searches for geocode and place information. You can then complete a follow-up query suggested from the `Suggest` API via a query id. |
| `Suggest` | `POST /suggest` | `readonly` | `QueryText` | - | `SuggestResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | `Suggest` provides intelligent predictions or recommendations based on the user's input or context, such as relevant places, points of interest, query terms or search category. It is designed to help users find places or point of interests candidates or... |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You don't have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `Message` | The request processing has failed because of an unknown error, exception or failure. |
| `ThrottlingException` | `structure` | `Message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `FieldList`, `Message`, `Reason` | The input fails to satisfy the constraints specified by an AWS service. |
| `AutocompleteRequest` | `structure` | `AdditionalFeatures`, `BiasPosition`, `Filter`, `IntendedUse`, `Key`, `Language`, `MaxResults`, `PoliticalView`, `PostalCodeMode`, `QueryText` | - |
| `AutocompleteResponse` | `structure` | `PricingBucket`, `ResultItems` | - |
| `GeocodeRequest` | `structure` | `AdditionalFeatures`, `BiasPosition`, `Filter`, `IntendedUse`, `Key`, `Language`, `MaxResults`, `PoliticalView`, `QueryComponents`, `QueryText` | - |
| `GeocodeResponse` | `structure` | `PricingBucket`, `ResultItems` | - |
| `GetPlaceRequest` | `structure` | `AdditionalFeatures`, `IntendedUse`, `Key`, `Language`, `PlaceId`, `PoliticalView` | - |
| `GetPlaceResponse` | `structure` | `AccessPoints`, `AccessRestrictions`, `Address`, `AddressNumberCorrected`, `BusinessChains`, `Categories`, `Contacts`, `FoodTypes`, `MainAddress`, `MapView`, `OpeningHours`, `Phonemes`, ... (+9) | - |
| `ReverseGeocodeRequest` | `structure` | `AdditionalFeatures`, `Filter`, `Heading`, `IntendedUse`, `Key`, `Language`, `MaxResults`, `PoliticalView`, `QueryPosition`, `QueryRadius` | - |
| `ReverseGeocodeResponse` | `structure` | `PricingBucket`, `ResultItems` | - |
| `SearchNearbyRequest` | `structure` | `AdditionalFeatures`, `Filter`, `IntendedUse`, `Key`, `Language`, `MaxResults`, `NextToken`, `PoliticalView`, `QueryPosition`, `QueryRadius` | - |
| `SearchNearbyResponse` | `structure` | `NextToken`, `PricingBucket`, `ResultItems` | - |
| `SearchTextRequest` | `structure` | `AdditionalFeatures`, `BiasPosition`, `Filter`, `IntendedUse`, `Key`, `Language`, `MaxResults`, `NextToken`, `PoliticalView`, `QueryId`, `QueryText` | - |
| `SearchTextResponse` | `structure` | `NextToken`, `PricingBucket`, `ResultItems` | - |
| `SuggestRequest` | `structure` | `AdditionalFeatures`, `BiasPosition`, `Filter`, `IntendedUse`, `Key`, `Language`, `MaxQueryRefinements`, `MaxResults`, `PoliticalView`, `QueryText` | - |
| `SuggestResponse` | `structure` | `PricingBucket`, `QueryRefinements`, `ResultItems` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
