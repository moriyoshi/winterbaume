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

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | You don't have sufficient access to perform this action. |
| `InternalServerException` | `structure` | Message | The request processing has failed because of an unknown error, exception or failure. |
| `ThrottlingException` | `structure` | Message | The request was denied due to request throttling. |
| `ValidationException` | `structure` | Message, Reason, FieldList | The input fails to satisfy the constraints specified by an AWS service. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
