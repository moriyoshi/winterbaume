# AWS Migration Hub Config

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The AWS Migration Hub home region APIs are available specifically for working with your Migration Hub home region. You can use these APIs to determine a home region, as well as to create and work with controls that describe the home region. You must make API calls for write actions (create, notify, associate, disassociate, import, or put) while in your home region, or a `HomeRegionNotSetException` error is returned. API calls for read actions (list, describe, stop, and delete) are permitted outside of your home region. If you call a write API outside the home region, an `InvalidInputException` is returned.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Migration Hub Config workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Create`, `Delete`, `Describe`, `Get` operation families, including `CreateHomeRegionControl`, `DeleteHomeRegionControl`, `DescribeHomeRegionControls`, `GetHomeRegion`.

## Service Identity and Protocol

- AWS model slug: `migrationhub-config`
- AWS SDK for Rust slug: `migrationhubconfig`
- Model version: `2019-06-30`
- Model file: `vendor/api-models-aws/models/migrationhub-config/service/2019-06-30/migrationhub-config-2019-06-30.json`
- SDK ID: `MigrationHub Config`
- Endpoint prefix: `migrationhub-config`
- ARN namespace: `mgh`
- CloudFormation name: `MigrationHubConfig`
- CloudTrail event source: `migrationhubconfig.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Create` (1), `Delete` (1), `Describe` (1), `Get` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateHomeRegionControl`, `DeleteHomeRegionControl`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeHomeRegionControls`, `GetHomeRegion`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- 4 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`.

## Operation Groups

### Create

- Operations: `CreateHomeRegionControl`
- Common required input members in this group: `HomeRegion`, `Target`

### Delete

- Operations: `DeleteHomeRegionControl`
- Common required input members in this group: `ControlId`

### Describe

- Operations: `DescribeHomeRegionControls`
- Traits: `paginated` (1)

### Get

- Operations: `GetHomeRegion`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateHomeRegionControl` | - | - | `HomeRegion`, `Target` | - | `CreateHomeRegionControlResult` | `AccessDeniedException`, `DryRunOperation`, `InternalServerError`, `InvalidInputException`, `ServiceUnavailableException`, `ThrottlingException` | This API sets up the home region for the calling account only. |
| `DeleteHomeRegionControl` | - | - | `ControlId` | - | `DeleteHomeRegionControlResult` | `AccessDeniedException`, `InternalServerError`, `InvalidInputException`, `ServiceUnavailableException`, `ThrottlingException` | This operation deletes the home region configuration for the calling account. The operation does not delete discovery or migration tracking data in the home region. |
| `DescribeHomeRegionControls` | - | `paginated` | - | - | `DescribeHomeRegionControlsResult` | `AccessDeniedException`, `InternalServerError`, `InvalidInputException`, `ServiceUnavailableException`, `ThrottlingException` | This API permits filtering on the `ControlId` and `HomeRegion` fields. |
| `GetHomeRegion` | - | - | - | - | `GetHomeRegionResult` | `AccessDeniedException`, `InternalServerError`, `InvalidInputException`, `ServiceUnavailableException`, `ThrottlingException` | Returns the calling account’s home region, if configured. This API is used by other AWS services to determine the regional endpoint for calling AWS Application Discovery Service and Migration Hub. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You do not have sufficient access to perform this action. |
| `InternalServerError` | `structure` | `Message` | Exception raised when an internal, configuration, or dependency error is encountered. |
| `InvalidInputException` | `structure` | `Message` | Exception raised when the provided input violates a policy constraint or is entered in the wrong format or data type. |
| `ServiceUnavailableException` | `structure` | `Message` | Exception raised when a request fails due to temporary unavailability of the service. |
| `ThrottlingException` | `structure` | `Message`, `RetryAfterSeconds` | The request was denied due to request throttling. |
| `CreateHomeRegionControlRequest` | `structure` | `DryRun`, `HomeRegion`, `Target` | - |
| `CreateHomeRegionControlResult` | `structure` | `HomeRegionControl` | - |
| `DryRunOperation` | `structure` | `Message` | Exception raised to indicate that authorization of an action was successful, when the `DryRun` flag is set to true. |
| `DeleteHomeRegionControlRequest` | `structure` | `ControlId` | - |
| `DeleteHomeRegionControlResult` | `structure` | - | - |
| `DescribeHomeRegionControlsRequest` | `structure` | `ControlId`, `HomeRegion`, `MaxResults`, `NextToken`, `Target` | - |
| `DescribeHomeRegionControlsResult` | `structure` | `HomeRegionControls`, `NextToken` | - |
| `GetHomeRegionRequest` | `structure` | - | - |
| `GetHomeRegionResult` | `structure` | `HomeRegion` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
