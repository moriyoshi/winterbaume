# AWS Control Catalog

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Welcome to the Control Catalog API reference. This guide is for developers who need detailed information about how to programmatically identify and filter the common controls and related metadata that are available to Amazon Web Services customers. This API reference provides descriptions, syntax, and usage examples for each of the actions and data types that are supported by Control Catalog. Use the following links to get started with the Control Catalog API: Actions: An alphabetical list of all Control Catalog API operations. Data types: An alphabetical list of all Control Catalog data types.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Control Catalog workflows in the local mock. Key resources include `CommonControlResource`, `ControlResource`, `DomainResource`, `ObjectiveResource`.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `List`, `Get` operation families, including `ListCommonControls`, `ListControlMappings`, `ListControls`, `ListDomains`, `GetControl`.

## Service Identity and Protocol

- AWS model slug: `controlcatalog`
- AWS SDK for Rust slug: `controlcatalog`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/controlcatalog/service/2018-05-10/controlcatalog-2018-05-10.json`
- SDK ID: `ControlCatalog`
- Endpoint prefix: `-`
- ARN namespace: `controlcatalog`
- CloudFormation name: `-`
- CloudTrail event source: `controlcatalog.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (5), `Get` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetControl`, `ListCommonControls`, `ListControlMappings`, `ListControls`, `ListDomains`, `ListObjectives`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- 6 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `CommonControlResource` | `CommonControlArn` | list: `ListCommonControls` | - | - |
| `ControlResource` | `ControlArn` | read: `GetControl`; list: `ListControls` | - | - |
| `DomainResource` | `DomainArn` | list: `ListDomains` | - | - |
| `ObjectiveResource` | `ObjectiveArn` | list: `ListObjectives` | - | - |
## Operation Groups

### List

- Operations: `ListCommonControls`, `ListControlMappings`, `ListControls`, `ListDomains`, `ListObjectives`
- Traits: `paginated` (5), `readonly` (5)

### Get

- Operations: `GetControl`
- Traits: `readonly` (1)
- Common required input members in this group: `ControlArn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetControl` | `POST /get-control` | `readonly` | `ControlArn` | - | `GetControlResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns details about a specific control, most notably a list of Amazon Web Services Regions where this control is supported. Input a value for the ControlArn parameter, in ARN form. |
| `ListCommonControls` | `POST /common-controls` | `readonly`, `paginated` | - | - | `ListCommonControlsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a paginated list of common controls from the Amazon Web Services Control Catalog. You can apply an optional filter to see common controls that have a specific objective. |
| `ListControlMappings` | `POST /list-control-mappings` | `readonly`, `paginated` | - | - | `ListControlMappingsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a paginated list of control mappings from the Control Catalog. Control mappings show relationships between controls and other entities, such as common controls or compliance frameworks. |
| `ListControls` | `POST /list-controls` | `readonly`, `paginated` | - | - | `ListControlsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a paginated list of all available controls in the Control Catalog library. Allows you to discover available controls. |
| `ListDomains` | `POST /domains` | `readonly`, `paginated` | - | - | `ListDomainsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a paginated list of domains from the Control Catalog. |
| `ListObjectives` | `POST /objectives` | `readonly`, `paginated` | - | - | `ListObjectivesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a paginated list of objectives from the Control Catalog. You can apply an optional filter to see the objectives that belong to a specific domain. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `Message` | An internal service error occurred during the processing of your request. |
| `ThrottlingException` | `structure` | `Message` | The request was denied due to request throttling. |
| `ValidationException` | `structure` | `Message` | The request has invalid or missing parameters. |
| `GetControlRequest` | `structure` | `ControlArn` | - |
| `GetControlResponse` | `structure` | `Aliases`, `Arn`, `Behavior`, `CreateTime`, `Description`, `GovernedResources`, `Implementation`, `Name`, `Parameters`, `RegionConfiguration`, `Severity` | - |
| `ResourceNotFoundException` | `structure` | `Message` | The requested resource does not exist. |
| `ListCommonControlsRequest` | `structure` | `CommonControlFilter`, `MaxResults`, `NextToken` | - |
| `ListCommonControlsResponse` | `structure` | `CommonControls`, `NextToken` | - |
| `ListControlMappingsRequest` | `structure` | `Filter`, `MaxResults`, `NextToken` | - |
| `ListControlMappingsResponse` | `structure` | `ControlMappings`, `NextToken` | - |
| `ListControlsRequest` | `structure` | `Filter`, `MaxResults`, `NextToken` | - |
| `ListControlsResponse` | `structure` | `Controls`, `NextToken` | - |
| `ListDomainsRequest` | `structure` | `MaxResults`, `NextToken` | - |
| `ListDomainsResponse` | `structure` | `Domains`, `NextToken` | - |
| `ListObjectivesRequest` | `structure` | `MaxResults`, `NextToken`, `ObjectiveFilter` | - |
| `ListObjectivesResponse` | `structure` | `NextToken`, `Objectives` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
