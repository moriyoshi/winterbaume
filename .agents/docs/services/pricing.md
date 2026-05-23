# AWS Price List Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

The Amazon Web Services Price List API is a centralized and convenient way to programmatically query Amazon Web Services for services, products, and pricing information. The Amazon Web Services Price List uses standardized product attributes such as `Location`, `Storage Class`, and `Operating System`, and provides prices at the SKU level. You can use the Amazon Web Services Price List to do the following: Build cost control and scenario planning tools Reconcile billing data Forecast future spend for budgeting purposes Provide cost benefit analysis that compare your internal workloads with Amazon Web Services Use `GetServices` without a service code to retrieve the service codes for all Amazon Web Services services, then `GetServices` with a service code to retrieve the attribute names for that service. After you have the service code and attribute names, you can use `GetAttributeValues` to see what values are available for an attribute. With the service code and an attribute name and value, you can use `GetProducts` to find specific products that you're interested in, such as an `AmazonEC2` instance, with a `Provisioned IOPS` `volumeType`.

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Price List Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model discovery and reporting workflows that retrieve, list, filter, or query service-managed state across the `Get`, `Describe`, `List` operation families, including `GetAttributeValues`, `GetPriceListFileUrl`, `GetProducts`, `DescribeServices`, `ListPriceLists`.

## Service Identity and Protocol

- AWS model slug: `pricing`
- AWS SDK for Rust slug: `pricing`
- Model version: `2017-10-15`
- Model file: `vendor/api-models-aws/models/pricing/service/2017-10-15/pricing-2017-10-15.json`
- SDK ID: `Pricing`
- Endpoint prefix: `api.pricing`
- ARN namespace: `pricing`
- CloudFormation name: `Pricing`
- CloudTrail event source: `pricelist.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (3), `Describe` (1), `List` (1).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeServices`, `GetAttributeValues`, `GetPriceListFileUrl`, `GetProducts`, `ListPriceLists`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- 5 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetAttributeValues`, `GetPriceListFileUrl`, `GetProducts`
- Traits: `paginated` (2)
- Common required input members in this group: `ServiceCode`

### Describe

- Operations: `DescribeServices`
- Traits: `paginated` (1)
- Common required input members in this group: -

### List

- Operations: `ListPriceLists`
- Traits: `paginated` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DescribeServices` | `-` | `paginated` | - | - | `DescribeServicesResponse` | `AccessDeniedException`, `ExpiredNextTokenException`, `InternalErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Returns the metadata for one service or a list of the metadata for all services. Use this without a service code to get the service codes for all services. Use it with a service code, such as AmazonEC2 , to get infor ... |
| `GetAttributeValues` | `-` | `paginated` | `ServiceCode`, `AttributeName` | - | `GetAttributeValuesResponse` | `AccessDeniedException`, `ExpiredNextTokenException`, `InternalErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Returns a list of attribute values. Attributes are similar to the details in a Price List API offer file. For a list of available attributes, see Offer File Definitions in the Billing and Cost Management User Guide . |
| `GetPriceListFileUrl` | `-` | - | `PriceListArn`, `FileFormat` | - | `GetPriceListFileUrlResponse` | `AccessDeniedException`, `InternalErrorException`, `InvalidParameterException`, `NotFoundException`, `ResourceNotFoundException`, `ThrottlingException` | This feature is in preview release and is subject to change. Your use of Amazon Web Services Price List API is subject to the Beta Service Participation terms of the Amazon Web Services Service Terms (Section 1.10). ... |
| `GetProducts` | `-` | `paginated` | `ServiceCode` | - | `GetProductsResponse` | `AccessDeniedException`, `ExpiredNextTokenException`, `InternalErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `NotFoundException`, `ThrottlingException` | Returns a list of all products that match the filter criteria. |
| `ListPriceLists` | `-` | `paginated` | `ServiceCode`, `EffectiveDate`, `CurrencyCode` | - | `ListPriceListsResponse` | `AccessDeniedException`, `ExpiredNextTokenException`, `InternalErrorException`, `InvalidNextTokenException`, `InvalidParameterException`, `NotFoundException`, `ResourceNotFoundException`, `ThrottlingException` | This feature is in preview release and is subject to change. Your use of Amazon Web Services Price List API is subject to the Beta Service Participation terms of the Amazon Web Services Service Terms (Section 1.10). ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message | General authentication failure. The request wasn't signed correctly. |
| `ExpiredNextTokenException` | `structure` | Message | The pagination token expired. Try again without a pagination token. |
| `InternalErrorException` | `structure` | Message | An error on the server occurred during the processing of your request. Try again later. |
| `InvalidNextTokenException` | `structure` | Message | The pagination token is invalid. Try again without a pagination token. |
| `InvalidParameterException` | `structure` | Message | One or more parameters had an invalid value. |
| `NotFoundException` | `structure` | Message | The requested resource can't be found. |
| `ResourceNotFoundException` | `structure` | Message | The requested resource can't be found. |
| `ThrottlingException` | `structure` | Message | You've made too many requests exceeding service quotas. |
| `DescribeServicesRequest` | `structure` | ServiceCode, FormatVersion, NextToken, MaxResults | - |
| `DescribeServicesResponse` | `structure` | Services, FormatVersion, NextToken | - |
| `GetAttributeValuesRequest` | `structure` | ServiceCode, AttributeName, NextToken, MaxResults | - |
| `GetAttributeValuesResponse` | `structure` | AttributeValues, NextToken | - |
| `GetPriceListFileUrlRequest` | `structure` | PriceListArn, FileFormat | - |
| `GetPriceListFileUrlResponse` | `structure` | Url | - |
| `GetProductsRequest` | `structure` | ServiceCode, Filters, FormatVersion, NextToken, MaxResults | - |
| `GetProductsResponse` | `structure` | FormatVersion, PriceList, NextToken | - |
| `ListPriceListsRequest` | `structure` | ServiceCode, EffectiveDate, RegionCode, CurrencyCode, NextToken, MaxResults | - |
| `ListPriceListsResponse` | `structure` | PriceLists, NextToken | - |
| `FilterType` | `enum` | TERM_MATCH, EQUALS, CONTAINS, ANY_OF, NONE_OF | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
