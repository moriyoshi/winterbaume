# AWS Marketplace Reporting Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Amazon Web Services Marketplace `GetBuyerDashboard` API enables you to get a procurement insights dashboard programmatically. The API gets the agreement and cost analysis dashboards with data for all of the Amazon Web Services accounts in your Amazon Web Services Organization. To use the Amazon Web Services Marketplace Reporting API, you must complete the following prerequisites: Enable all features for your organization. For more information, see Enabling all features for an organization with Organizations, in the Organizations User Guide . Call the service as the Organizations management account or an account registered as a delegated administrator for the procurement insights service.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Marketplace Reporting Service workflows in the local mock. Key resources include `Dashboard`.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Get` operation families, including `GetBuyerDashboard`.

## Service Identity and Protocol

- AWS model slug: `marketplace-reporting`
- AWS SDK for Rust slug: `marketplacereporting`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/marketplace-reporting/service/2018-05-10/marketplace-reporting-2018-05-10.json`
- SDK ID: `Marketplace Reporting`
- Endpoint prefix: `reporting-marketplace`
- ARN namespace: `aws-marketplace`
- CloudFormation name: `-`
- CloudTrail event source: `reporting-marketplace.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetBuyerDashboard`.
- 1 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Dashboard` | `dashboardIdentifier` | - | `GetBuyerDashboard` | - |
## Operation Groups

### Get

- Operations: `GetBuyerDashboard`
- Traits: `readonly` (1)
- Common required input members in this group: `dashboardIdentifier`, `embeddingDomains`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetBuyerDashboard` | `POST /getBuyerDashboard` | `readonly` | `dashboardIdentifier`, `embeddingDomains` | - | `GetBuyerDashboardOutput` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `UnauthorizedException` | Generates an embedding URL for an Amazon QuickSight dashboard for an anonymous user. This API is available only to Amazon Web Services Organization management accounts or delegated administrators registered for the procurement insights... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `GetBuyerDashboardInput` | `structure` | `dashboardIdentifier`, `embeddingDomains` | - |
| `GetBuyerDashboardOutput` | `structure` | `dashboardIdentifier`, `embedUrl`, `embeddingDomains` | - |
| `AccessDeniedException` | `structure` | `message` | You do not have sufficient access to perform this action. |
| `BadRequestException` | `structure` | `message` | The request is malformed, or it contains an error such as an invalid parameter. |
| `InternalServerException` | `structure` | `message` | The operation failed due to a server error. |
| `UnauthorizedException` | `structure` | `message` | You do not have permission to perform this action. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
