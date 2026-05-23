# AWS Application Cost Profiler

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This reference provides descriptions of the AWS Application Cost Profiler API. The AWS Application Cost Profiler API provides programmatic access to view, create, update, and delete application cost report definitions, as well as to import your usage data into the Application Cost Profiler service. For more information about using this service, see the AWS Application Cost Profiler User Guide.

## Possible Usage Scenarios
- From the AWS documentation and model: configure reports that allocate application usage and costs by tenant or business dimension.
- From the operation surface: manage report definitions, ingest usage data, list generated reports, and delete reporting configuration.

## Service Identity and Protocol

- AWS model slug: `applicationcostprofiler`
- AWS SDK for Rust slug: `applicationcostprofiler`
- Model version: `2020-09-10`
- Model file: `vendor/api-models-aws/models/applicationcostprofiler/service/2020-09-10/applicationcostprofiler-2020-09-10.json`
- SDK ID: `ApplicationCostProfiler`
- Endpoint prefix: `application-cost-profiler`
- ARN namespace: `application-cost-profiler`
- CloudFormation name: `ApplicationCostProfiler`
- CloudTrail event source: `applicationcostprofiler.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (1), `Get` (1), `Import` (1), `List` (1), `Put` (1), `Update` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `DeleteReportDefinition`, `ImportApplicationUsage`, `PutReportDefinition`, `UpdateReportDefinition`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetReportDefinition`, `ListReportDefinitions`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DeleteReportDefinition`, `GetReportDefinition`, `ImportApplicationUsage`, `ListReportDefinitions`, `PutReportDefinition`, `UpdateReportDefinition`.
- 6 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Delete

- Operations: `DeleteReportDefinition`
- Common required input members in this group: `reportId`

### Get

- Operations: `GetReportDefinition`
- Common required input members in this group: `reportId`

### Import

- Operations: `ImportApplicationUsage`
- Common required input members in this group: `sourceS3Location`

### List

- Operations: `ListReportDefinitions`
- Traits: `paginated` (1)

### Put

- Operations: `PutReportDefinition`
- Common required input members in this group: `destinationS3Location`, `format`, `reportDescription`, `reportFrequency`, `reportId`

### Update

- Operations: `UpdateReportDefinition`
- Common required input members in this group: `destinationS3Location`, `format`, `reportDescription`, `reportFrequency`, `reportId`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `DeleteReportDefinition` | `DELETE /reportDefinition/{reportId}` | - | `reportId` | - | `DeleteReportDefinitionResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Deletes the specified report definition in AWS Application Cost Profiler. This stops the report from being generated. |
| `GetReportDefinition` | `GET /reportDefinition/{reportId}` | - | `reportId` | - | `GetReportDefinitionResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves the definition of a report already configured in AWS Application Cost Profiler. |
| `ImportApplicationUsage` | `POST /importApplicationUsage` | - | `sourceS3Location` | - | `ImportApplicationUsageResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Ingests application usage data from Amazon Simple Storage Service (Amazon S3). The data must already exist in the S3 location. |
| `ListReportDefinitions` | `GET /reportDefinition` | `paginated` | - | - | `ListReportDefinitionsResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a list of all reports and their configurations for your AWS account. The maximum number of reports is one. |
| `PutReportDefinition` | `POST /reportDefinition` | - | `destinationS3Location`, `format`, `reportDescription`, `reportFrequency`, `reportId` | - | `PutReportDefinitionResult` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates the report definition for a report in Application Cost Profiler. |
| `UpdateReportDefinition` | `PUT /reportDefinition/{reportId}` | - | `destinationS3Location`, `format`, `reportDescription`, `reportFrequency`, `reportId` | - | `UpdateReportDefinitionResult` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Updates existing report in AWS Application Cost Profiler. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListReportDefinitions` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | You do not have permission to perform this action. |
| `InternalServerException` | `structure` | `message` | An internal server error occurred. |
| `ThrottlingException` | `structure` | `message` | The calls to AWS Application Cost Profiler API are throttled. |
| `ValidationException` | `structure` | `message` | The input fails to satisfy the constraints for the API. |
| `DeleteReportDefinitionRequest` | `structure` | `reportId` | - |
| `DeleteReportDefinitionResult` | `structure` | `reportId` | - |
| `GetReportDefinitionRequest` | `structure` | `reportId` | - |
| `GetReportDefinitionResult` | `structure` | `createdAt`, `destinationS3Location`, `format`, `lastUpdated`, `reportDescription`, `reportFrequency`, `reportId` | - |
| `ImportApplicationUsageRequest` | `structure` | `sourceS3Location` | - |
| `ImportApplicationUsageResult` | `structure` | `importId` | - |
| `ListReportDefinitionsRequest` | `structure` | `maxResults`, `nextToken` | - |
| `ListReportDefinitionsResult` | `structure` | `nextToken`, `reportDefinitions` | - |
| `PutReportDefinitionRequest` | `structure` | `destinationS3Location`, `format`, `reportDescription`, `reportFrequency`, `reportId` | - |
| `PutReportDefinitionResult` | `structure` | `reportId` | - |
| `ServiceQuotaExceededException` | `structure` | `message` | Your request exceeds one or more of the service quotas. |
| `UpdateReportDefinitionRequest` | `structure` | `destinationS3Location`, `format`, `reportDescription`, `reportFrequency`, `reportId` | - |
| `UpdateReportDefinitionResult` | `structure` | `reportId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
