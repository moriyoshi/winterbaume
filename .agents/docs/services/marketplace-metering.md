# AWSMarketplace Metering

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Web Services Marketplace Metering Service This reference provides descriptions of the low-level Marketplace Metering Service API. Amazon Web Services Marketplace sellers can use this API to submit usage data for custom usage dimensions. For information about the permissions that you need to use this API, see Amazon Web Services Marketplace metering and entitlement API permissions in the Amazon Web Services Marketplace Seller Guide. Submitting metering records MeterUsage Submits the metering record for an Amazon Web Services Marketplace product. Called from: Amazon Elastic Compute Cloud (Amazon EC2) instance or a container running on either Amazon Elastic Kubernetes Service (Amazon EKS) or Amazon Elastic Container Service (Amazon ECS) Supported product types: Amazon Machine Images (AMIs) and containers Vendor-metered tagging: Supported allocation tagging BatchMeterUsage Submits the metering record for a set of customers.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWSMarketplace Metering workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model setup and mutation workflows that create or update service resources across the `Batch`, `Meter`, `Register`, `Resolve` operation families, including `BatchMeterUsage`, `MeterUsage`, `RegisterUsage`, `ResolveCustomer`.

## Service Identity and Protocol

- AWS model slug: `marketplace-metering`
- AWS SDK for Rust slug: `marketplacemetering`
- Model version: `2016-01-14`
- Model file: `vendor/api-models-aws/models/marketplace-metering/service/2016-01-14/marketplace-metering-2016-01-14.json`
- SDK ID: `Marketplace Metering`
- Endpoint prefix: `metering.marketplace`
- ARN namespace: `aws-marketplace`
- CloudFormation name: `MarketplaceMetering`
- CloudTrail event source: `marketplacemetering.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Batch` (1), `Meter` (1), `Register` (1), `Resolve` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `BatchMeterUsage`, `RegisterUsage`.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- 4 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `EC2/VPC`, `ECS`, `EKS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Batch

- Operations: `BatchMeterUsage`
- Common required input members in this group: -

### Meter

- Operations: `MeterUsage`
- Traits: `idempotency-token` (1)
- Common required input members in this group: -

### Register

- Operations: `RegisterUsage`
- Common required input members in this group: -

### Resolve

- Operations: `ResolveCustomer`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `BatchMeterUsage` | `-` | - | `UsageRecords` | - | `BatchMeterUsageResult` | `DisabledApiException`, `InternalServiceErrorException`, `InvalidCustomerIdentifierException`, `InvalidLicenseException`, `InvalidProductCodeException`, `InvalidTagException`, `InvalidUsageAllocationsException`, `InvalidUsageDimensionException`, `ThrottlingException`, `TimestampOutOfBoundsException` | Amazon Web Services Marketplace is introducing Concurrent Agreements, enabling buyers to make multiple purchases per Amazon Web Services account. Starting June 1, 2026, new SaaS products must use CustomerAWSAccountId ... |
| `MeterUsage` | `-` | `idempotency-token` | `ProductCode`, `Timestamp`, `UsageDimension` | `ClientToken` | `MeterUsageResult` | `CustomerNotEntitledException`, `DuplicateRequestException`, `IdempotencyConflictException`, `InternalServiceErrorException`, `InvalidEndpointRegionException`, `InvalidProductCodeException`, `InvalidTagException`, `InvalidUsageAllocationsException`, `InvalidUsageDimensionException`, `ThrottlingException`, `TimestampOutOfBoundsException` | As a seller, your software hosted in the buyer's Amazon Web Services account uses this API action to emit metering records directly to Amazon Web Services Marketplace. You must use the following buyer Amazon Web Serv ... |
| `RegisterUsage` | `-` | - | `ProductCode`, `PublicKeyVersion` | - | `RegisterUsageResult` | `CustomerNotEntitledException`, `DisabledApiException`, `InternalServiceErrorException`, `InvalidProductCodeException`, `InvalidPublicKeyVersionException`, `InvalidRegionException`, `PlatformNotSupportedException`, `ThrottlingException` | Paid container software products sold through Amazon Web Services Marketplace must integrate with the Amazon Web Services Marketplace Metering Service and call the RegisterUsage operation for software entitlement and ... |
| `ResolveCustomer` | `-` | - | `RegistrationToken` | - | `ResolveCustomerResult` | `DisabledApiException`, `ExpiredTokenException`, `InternalServiceErrorException`, `InvalidTokenException`, `ThrottlingException` | ResolveCustomer is called by a SaaS application during the registration process. When a buyer visits your website during the registration process, the buyer submits a registration token through their browser. The reg ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `CustomerNotEntitledException` | `structure` | message | Exception thrown when the customer does not have a valid subscription for the product. |
| `DisabledApiException` | `structure` | message | The API is disabled in the Region. |
| `DuplicateRequestException` | `structure` | message | A metering record has already been emitted by the same EC2 instance, ECS task, or EKS pod for the given { usageDimension , timestamp } with a different usag ... |
| `ExpiredTokenException` | `structure` | message | The submitted registration token has expired. This can happen if the buyer's browser takes too long to redirect to your page, the buyer has resubmitted the ... |
| `IdempotencyConflictException` | `structure` | message | The ClientToken is being used for multiple requests. |
| `InternalServiceErrorException` | `structure` | message | An internal error has occurred. Retry your request. If the problem persists, post a message with details on the Amazon Web Services forums. |
| `InvalidCustomerIdentifierException` | `structure` | message | You have metered usage for a CustomerIdentifier that does not exist. |
| `InvalidEndpointRegionException` | `structure` | message | The endpoint being called is in a Amazon Web Services Region different from your EC2 instance, ECS task, or EKS pod. The Region of the Metering Service endp ... |
| `InvalidLicenseException` | `structure` | message | Ensure the LicenseArn is valid, matches the customer, and usage is within the license activation period. |
| `InvalidProductCodeException` | `structure` | message | The product code passed does not match the product code used for publishing the product. |
| `InvalidPublicKeyVersionException` | `structure` | message | Public Key version is invalid. |
| `InvalidRegionException` | `structure` | message | RegisterUsage must be called in the same Amazon Web Services Region the ECS task was launched in. This prevents a container from hardcoding a Region (e.g. w ... |
| `InvalidTagException` | `structure` | message | The tag is invalid, or the number of tags is greater than 5. |
| `InvalidTokenException` | `structure` | message | Registration token is invalid. |
| `InvalidUsageAllocationsException` | `structure` | message | Sum of allocated usage quantities is not equal to the usage quantity. |
| `InvalidUsageDimensionException` | `structure` | message | The usage dimension does not match one of the UsageDimensions associated with products. |
| `PlatformNotSupportedException` | `structure` | message | Amazon Web Services Marketplace does not support metering usage from the underlying platform. Currently, Amazon ECS, Amazon EKS, and Fargate are supported. |
| `ThrottlingException` | `structure` | message | The calls to the API are throttled. |
| `TimestampOutOfBoundsException` | `structure` | message | The timestamp value passed in the UsageRecord is out of allowed range. For BatchMeterUsage , if any of the records are outside of the allowed range, the ent ... |
| `BatchMeterUsageRequest` | `structure` | UsageRecords, ProductCode | A BatchMeterUsageRequest contains UsageRecords , which indicate quantities of usage within your application. |
| `BatchMeterUsageResult` | `structure` | Results, UnprocessedRecords | Contains the UsageRecords processed by BatchMeterUsage and any records that have failed due to transient error. |
| `MeterUsageRequest` | `structure` | ProductCode, Timestamp, UsageDimension, UsageQuantity, DryRun, UsageAllocations, ClientToken | - |
| `MeterUsageResult` | `structure` | MeteringRecordId | - |
| `RegisterUsageRequest` | `structure` | ProductCode, PublicKeyVersion, Nonce | - |
| `RegisterUsageResult` | `structure` | PublicKeyRotationTimestamp, Signature | - |
| `ResolveCustomerRequest` | `structure` | RegistrationToken | Contains input to the ResolveCustomer operation. |
| `ResolveCustomerResult` | `structure` | CustomerIdentifier, ProductCode, CustomerAWSAccountId, LicenseArn | The result of the ResolveCustomer operation. Contains the CustomerIdentifier along with the CustomerAWSAccountId , ProductCode , and LicenseArn . |
| `UsageRecordResultStatus` | `enum` | SUCCESS, CUSTOMER_NOT_SUBSCRIBED, DUPLICATE_RECORD | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
