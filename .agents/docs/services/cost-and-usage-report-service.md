# AWS Cost and Usage Report Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

You can use the Amazon Web Services Cost and Usage Report API to programmatically create, query, and delete Amazon Web Services Cost and Usage Report definitions. Amazon Web Services Cost and Usage Report track the monthly Amazon Web Services costs and usage associated with your Amazon Web Services account. The report contains line items for each unique combination of Amazon Web Services product, usage type, and operation that your Amazon Web Services account uses. You can configure the Amazon Web Services Cost and Usage Report to show only the data that you want, using the Amazon Web Services Cost and Usage Report API. Service Endpoint The Amazon Web Services Cost and Usage Report API provides the following endpoint: cur.us-east-1.amazonaws.com

## Possible Usage Scenarios
- From the AWS documentation and model: represent documented AWS Cost and Usage Report Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Delete`, `Describe`, `List`, `Modify`, `Put` operation families, including `DeleteReportDefinition`, `DescribeReportDefinitions`, `ListTagsForResource`, `ModifyReportDefinition`, `PutReportDefinition`, `TagResource`.

## Service Identity and Protocol

- AWS model slug: `cost-and-usage-report-service`
- AWS SDK for Rust slug: `costandusagereport`
- Model version: `2017-01-06`
- Model file: `vendor/api-models-aws/models/cost-and-usage-report-service/service/2017-01-06/cost-and-usage-report-service-2017-01-06.json`
- SDK ID: `Cost and Usage Report Service`
- Endpoint prefix: `cur`
- ARN namespace: `cur`
- CloudFormation name: `CostandUsageReportService`
- CloudTrail event source: `costandusagereportservice.amazonaws.com`
- Protocols: `awsJson1_1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (1), `Describe` (1), `List` (1), `Modify` (1), `Put` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DeleteReportDefinition`, `ModifyReportDefinition`, `PutReportDefinition`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeReportDefinitions`, `ListTagsForResource`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DeleteReportDefinition`, `DescribeReportDefinitions`, `ModifyReportDefinition`, `PutReportDefinition`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 7 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/cur/latest/userguide/what-is-cur.html
- https://docs.aws.amazon.com/cur/latest/userguide/cur-create.html
- https://docs.aws.amazon.com/cur/latest/userguide/cur-consolidated-billing.html

Research outcomes:
- AWS Cost and Usage Reports deliver detailed billing and usage data to an Amazon S3 bucket.
- Reports can include hourly, daily, or monthly granularity depending on report configuration.
- Reports can include resource IDs when supported and enabled.
- Report files are delivered as billing data becomes available and are finalised after invoices close for the month.
- Report definitions specify S3 bucket, prefix, time unit, format, compression, schema elements, additional artefacts, and refresh behaviour.
- Management accounts can create reports for consolidated billing; member-account behaviour depends on organisation configuration and permissions.
- CUR data can be queried through Athena or loaded into analytics tools depending on output format and artefacts.

Parity implications:
- Model report definitions, S3 delivery configuration, schema elements, refresh behaviour, report files, billing periods, and organisation scope separately.
- Report delivery should be asynchronous and billing-period aware, with open-period files distinct from finalised invoice-period data.
- Include-resource-id and schema changes should affect generated report columns.

## Operation Groups

### Delete

- Operations: `DeleteReportDefinition`
- Common required input members in this group: -

### Describe

- Operations: `DescribeReportDefinitions`
- Traits: `paginated` (1)
- Common required input members in this group: -

### List

- Operations: `ListTagsForResource`
- Common required input members in this group: -

### Modify

- Operations: `ModifyReportDefinition`
- Common required input members in this group: -

### Put

- Operations: `PutReportDefinition`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DeleteReportDefinition` | `-` | - | `ReportName` | - | `DeleteReportDefinitionResponse` | `InternalErrorException`, `ValidationException` | Deletes the specified report. Any tags associated with the report are also deleted. |
| `DescribeReportDefinitions` | `-` | `paginated` | - | - | `DescribeReportDefinitionsResponse` | `InternalErrorException` | Lists the Amazon Web Services Cost and Usage Report available to this account. |
| `ListTagsForResource` | `-` | - | `ReportName` | - | `ListTagsForResourceResponse` | `InternalErrorException`, `ResourceNotFoundException`, `ValidationException` | Lists the tags associated with the specified report definition. |
| `ModifyReportDefinition` | `-` | - | `ReportName`, `ReportDefinition` | - | `ModifyReportDefinitionResponse` | `InternalErrorException`, `ValidationException` | Allows you to programmatically update your report preferences. |
| `PutReportDefinition` | `-` | - | `ReportDefinition` | - | `PutReportDefinitionResponse` | `DuplicateReportNameException`, `InternalErrorException`, `ReportLimitReachedException`, `ResourceNotFoundException`, `ValidationException` | Creates a new report using the description that you provide. |
| `TagResource` | `-` | - | `ReportName`, `Tags` | - | `TagResourceResponse` | `InternalErrorException`, `ResourceNotFoundException`, `ValidationException` | Associates a set of tags with a report definition. |
| `UntagResource` | `-` | - | `ReportName`, `TagKeys` | - | `UntagResourceResponse` | `InternalErrorException`, `ResourceNotFoundException`, `ValidationException` | Disassociates a set of tags from a report definition. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `DuplicateReportNameException` | `structure` | Message | A report with the specified name already exists in the account. Specify a different report name. |
| `InternalErrorException` | `structure` | Message | An error on the server occurred during the processing of your request. Try again later. |
| `ReportLimitReachedException` | `structure` | Message | This account already has five reports defined. To define a new report, you must delete an existing report. |
| `ResourceNotFoundException` | `structure` | Message | The specified report ( ReportName ) in the request doesn't exist. |
| `ValidationException` | `structure` | Message | The input fails to satisfy the constraints specified by an Amazon Web Services service. |
| `DeleteReportDefinitionRequest` | `structure` | ReportName | Deletes the specified report. |
| `DeleteReportDefinitionResponse` | `structure` | ResponseMessage | If the action is successful, the service sends back an HTTP 200 response. |
| `DescribeReportDefinitionsRequest` | `structure` | MaxResults, NextToken | Requests a Amazon Web Services Cost and Usage Report list owned by the account. |
| `DescribeReportDefinitionsResponse` | `structure` | ReportDefinitions, NextToken | If the action is successful, the service sends back an HTTP 200 response. |
| `ListTagsForResourceRequest` | `structure` | ReportName | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `ModifyReportDefinitionRequest` | `structure` | ReportName, ReportDefinition | - |
| `ModifyReportDefinitionResponse` | `structure` | **empty (no members)** | - |
| `PutReportDefinitionRequest` | `structure` | ReportDefinition, Tags | Creates a Cost and Usage Report. |
| `PutReportDefinitionResponse` | `structure` | **empty (no members)** | If the action is successful, the service sends back an HTTP 200 response with an empty HTTP body. |
| `TagResourceRequest` | `structure` | ReportName, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ReportName, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `AWSRegion` | `enum` | CAPE_TOWN, HONG_KONG, MUMBAI, HYDERABAD, SINGAPORE, SYDNEY, JAKARTA, TOKYO, SEOUL, OSAKA, CANADA_CENTRAL, FRANKFURT, ... (+16) | The region of the S3 bucket that Amazon Web Services delivers the report into. |
| `AdditionalArtifact` | `enum` | REDSHIFT, QUICKSIGHT, ATHENA | The types of manifest that you want Amazon Web Services to create for this report. |
| `CompressionFormat` | `enum` | ZIP, GZIP, Parquet | The compression format that Amazon Web Services uses for the report. |
| `LastStatus` | `enum` | SUCCESS, ERROR_PERMISSIONS, ERROR_NO_BUCKET | - |
| `ReportFormat` | `enum` | CSV, Parquet | The format that Amazon Web Services saves the report in. |
| `ReportVersioning` | `enum` | CREATE_NEW_REPORT, OVERWRITE_REPORT | - |
| `SchemaElement` | `enum` | RESOURCES, SPLIT_COST_ALLOCATION_DATA, MANUAL_DISCOUNT_COMPATIBILITY | Whether or not Amazon Web Services includes resource IDs in the report. |
| `TimeUnit` | `enum` | HOURLY, DAILY, MONTHLY | The length of time covered by the report. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
