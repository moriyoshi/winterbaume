# AWS Marketplace Entitlement Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Marketplace Entitlement Service This reference provides descriptions of the AWS Marketplace Entitlement Service API. AWS Marketplace Entitlement Service is used to determine the entitlement of a customer to a given product. An entitlement represents capacity in a product owned by the customer. For example, a customer might own some number of users or seats in an SaaS application or some amount of data capacity in a multi-tenant database. Getting Entitlement Records GetEntitlements - Gets the entitlements for a Marketplace product.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Marketplace Entitlement Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Get` operation families, including `GetEntitlements`.

## Service Identity and Protocol

- AWS model slug: `marketplace-entitlement-service`
- AWS SDK for Rust slug: `marketplaceentitlementservice`
- Model version: `2017-01-11`
- Model file: `vendor/api-models-aws/models/marketplace-entitlement-service/service/2017-01-11/marketplace-entitlement-service-2017-01-11.json`
- SDK ID: `Marketplace Entitlement Service`
- Endpoint prefix: `entitlement.marketplace`
- ARN namespace: `aws-marketplace`
- CloudFormation name: `MarketplaceEntitlementService`
- CloudTrail event source: `marketplaceentitlementservice.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetEntitlements`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- 1 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `STS`.

## Operation Groups

### Get

- Operations: `GetEntitlements`
- Traits: `paginated` (1)
- Common required input members in this group: `ProductCode`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetEntitlements` | - | `paginated` | `ProductCode` | - | `GetEntitlementsResult` | `InternalServiceErrorException`, `InvalidParameterException`, `ThrottlingException` | GetEntitlements retrieves entitlement values for a given product. The results can be filtered based on customer identifier, AWS account ID, license ARN, or product dimensions. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `GetEntitlementsRequest` | `structure` | `Filter`, `MaxResults`, `NextToken`, `ProductCode` | The GetEntitlementsRequest contains parameters for the GetEntitlements operation. |
| `GetEntitlementsResult` | `structure` | `Entitlements`, `NextToken` | The GetEntitlementsRequest contains results from the GetEntitlements operation. |
| `InternalServiceErrorException` | `structure` | `message` | An internal error has occurred. |
| `InvalidParameterException` | `structure` | `message` | One or more parameters in your request was invalid. |
| `ThrottlingException` | `structure` | `message` | The calls to the GetEntitlements API are throttled. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
