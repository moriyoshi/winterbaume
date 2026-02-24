# Amazon Forecast Query Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Provides APIs for creating and managing Amazon Forecast resources.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented Amazon Forecast Query Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Query` operation families, including `QueryForecast`, `QueryWhatIfForecast`.

## Service Identity and Protocol

- AWS model slug: `forecastquery`
- AWS SDK for Rust slug: `forecast`
- Model version: `2018-06-26`
- Model file: `vendor/api-models-aws/models/forecastquery/service/2018-06-26/forecastquery-2018-06-26.json`
- SDK ID: `forecastquery`
- Endpoint prefix: `forecastquery`
- ARN namespace: `forecast`
- CloudFormation name: `Forecastquery`
- CloudTrail event source: `forecastquery.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Query` (2).
- 2 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.

## Operation Groups

### Query

- Operations: `QueryForecast`, `QueryWhatIfForecast`
- Common required input members in this group: `Filters`, `ForecastArn`, `WhatIfForecastArn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `QueryForecast` | - | - | `Filters`, `ForecastArn` | - | `QueryForecastResponse` | `InvalidInputException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Retrieves a forecast for a single item, filtered by the supplied criteria. The criteria is a key-value pair. |
| `QueryWhatIfForecast` | - | - | `Filters`, `WhatIfForecastArn` | - | `QueryWhatIfForecastResponse` | `InvalidInputException`, `InvalidNextTokenException`, `LimitExceededException`, `ResourceInUseException`, `ResourceNotFoundException` | Retrieves a what-if forecast. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `InvalidInputException` | `structure` | `Message` | The value is invalid or is too long. |
| `InvalidNextTokenException` | `structure` | `Message` | The token is not valid. |
| `LimitExceededException` | `structure` | `Message` | The limit on the number of requests per second has been exceeded. |
| `ResourceInUseException` | `structure` | `Message` | The specified resource is in use. |
| `ResourceNotFoundException` | `structure` | `Message` | We can't find that resource. |
| `QueryForecastRequest` | `structure` | `EndDate`, `Filters`, `ForecastArn`, `NextToken`, `StartDate` | - |
| `QueryForecastResponse` | `structure` | `Forecast` | - |
| `QueryWhatIfForecastRequest` | `structure` | `EndDate`, `Filters`, `NextToken`, `StartDate`, `WhatIfForecastArn` | - |
| `QueryWhatIfForecastResponse` | `structure` | `Forecast` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
