# AWS Marketplace Commerce Analytics

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Provides AWS Marketplace business intelligence data on-demand.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS Marketplace Commerce Analytics resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Marketplace Commerce Analytics workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model setup and mutation workflows that create or update service resources across the `Generate`, `Start` operation families, including `GenerateDataSet`, `StartSupportDataExport`.

## Service Identity and Protocol

- AWS model slug: `marketplace-commerce-analytics`
- AWS SDK for Rust slug: `marketplacecommerceanalytics`
- Model version: `2015-07-01`
- Model file: `vendor/api-models-aws/models/marketplace-commerce-analytics/service/2015-07-01/marketplace-commerce-analytics-2015-07-01.json`
- SDK ID: `Marketplace Commerce Analytics`
- Endpoint prefix: `marketplacecommerceanalytics`
- ARN namespace: `marketplacecommerceanalytics`
- CloudFormation name: `MarketplaceCommerceAnalytics`
- CloudTrail event source: `marketplacecommerceanalytics.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Generate` (1), `Start` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `StartSupportDataExport`.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartSupportDataExport`.
- 2 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `SNS`.

## Operation Groups

### Generate

- Operations: `GenerateDataSet`
- Common required input members in this group: `dataSetPublicationDate`, `dataSetType`, `destinationS3BucketName`, `roleNameArn`, `snsTopicArn`

### Start

- Operations: `StartSupportDataExport`
- Common required input members in this group: `dataSetType`, `destinationS3BucketName`, `fromDate`, `roleNameArn`, `snsTopicArn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GenerateDataSet` | - | - | `dataSetPublicationDate`, `dataSetType`, `destinationS3BucketName`, `roleNameArn`, `snsTopicArn` | - | `GenerateDataSetResult` | `MarketplaceCommerceAnalyticsException` | Given a data set type and data set publication date, asynchronously publishes the requested data set to the specified S3 bucket and notifies the specified SNS topic once the data is available. Returns a unique request identifier that can be used to correlate... |
| `StartSupportDataExport` | - | - | `dataSetType`, `destinationS3BucketName`, `fromDate`, `roleNameArn`, `snsTopicArn` | - | `StartSupportDataExportResult` | `MarketplaceCommerceAnalyticsException` | This target has been deprecated. Given a data set type and a from date, asynchronously publishes the requested customer support data to the specified S3 bucket and notifies the specified SNS topic once the data is available. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `MarketplaceCommerceAnalyticsException` | `structure` | `message` | This exception is thrown when an internal service error occurs. |
| `GenerateDataSetRequest` | `structure` | `customerDefinedValues`, `dataSetPublicationDate`, `dataSetType`, `destinationS3BucketName`, `destinationS3Prefix`, `roleNameArn`, `snsTopicArn` | Container for the parameters to the GenerateDataSet operation. |
| `GenerateDataSetResult` | `structure` | `dataSetRequestId` | Container for the result of the GenerateDataSet operation. |
| `StartSupportDataExportRequest` | `structure` | `customerDefinedValues`, `dataSetType`, `destinationS3BucketName`, `destinationS3Prefix`, `fromDate`, `roleNameArn`, `snsTopicArn` | This target has been deprecated. |
| `StartSupportDataExportResult` | `structure` | `dataSetRequestId` | This target has been deprecated. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
