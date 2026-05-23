# AWS Backup Search

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Backup Search Backup Search is the recovery point and item level search for Backup. For additional information, see: Backup API Reference Backup Developer Guide

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AWS Backup Search resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: create and inspect backup search jobs that locate items inside protected recovery points.
- From the operation surface: support indexed backup search, result pagination, job lifecycle state, and retrieval of matching backup items.

## Service Identity and Protocol

- AWS model slug: `backupsearch`
- AWS SDK for Rust slug: `backupsearch`
- Model version: `2018-05-10`
- Model file: `vendor/api-models-aws/models/backupsearch/service/2018-05-10/backupsearch-2018-05-10.json`
- SDK ID: `BackupSearch`
- Endpoint prefix: `backup-search`
- ARN namespace: `backup-search`
- CloudFormation name: `-`
- CloudTrail event source: `backup.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (5), `Get` (2), `Start` (2), `Stop` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `StartSearchJob`, `StartSearchResultExportJob`, `StopSearchJob`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetSearchJob`, `GetSearchResultExportJob`, `ListSearchJobBackups`, `ListSearchJobResults`, `ListSearchJobs`, `ListSearchResultExportJobs`, `ListTagsForResource`.
- Pagination is modelled for 4 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 5 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `GetSearchJob`, `GetSearchResultExportJob`, `ListSearchJobBackups`, `ListSearchJobResults`, `ListSearchJobs`, `ListSearchResultExportJobs`, `StartSearchJob`, `StartSearchResultExportJob`, `StopSearchJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 11 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `SearchJob` | `SearchJobIdentifier` | create: `StartSearchJob`; read: `GetSearchJob`; update: `StopSearchJob`; list: `ListSearchJobs` | - | - |
| `SearchResultExportJob` | `ExportJobIdentifier` | create: `StartSearchResultExportJob`; read: `GetSearchResultExportJob`; list: `ListSearchResultExportJobs` | - | - |
## Operation Groups

### List

- Operations: `ListSearchJobBackups`, `ListSearchJobResults`, `ListTagsForResource`
- Traits: `readonly` (3), `paginated` (2)
- Common required input members in this group: `SearchJobIdentifier`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `ListSearchJobBackups` | `GET /search-jobs/{SearchJobIdentifier}/backups` | `readonly`, `paginated` | `SearchJobIdentifier` | - | `ListSearchJobBackupsOutput` | `ResourceNotFoundException` | This operation returns a list of all backups (recovery points) in a paginated format that were included in the search job. If a search does not display an expected backup in the results, you can call this operation t ... |
| `ListSearchJobResults` | `GET /search-jobs/{SearchJobIdentifier}/search-results` | `readonly`, `paginated` | `SearchJobIdentifier` | - | `ListSearchJobResultsOutput` | `ResourceNotFoundException` | This operation returns a list of a specified search job. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | This operation returns the tags for a resource type. |
| `TagResource` | `POST /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `ResourceNotFoundException` | This operation puts tags on the resource you indicate. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException` | This operation removes tags from the specified resource. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListSearchJobBackups` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `ListSearchJobResults` | - | `NextToken -> nextToken`, `MaxResults -> maxResults` | - | - |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. |
| `ConflictException` | `structure` | message, resourceId, resourceType | This exception occurs when a conflict with a previous successful operation is detected. This generally occurs when the previous operation did not have time ... |
| `InternalServerException` | `structure` | message, retryAfterSeconds | An internal server error occurred. Retry your request. |
| `ResourceNotFoundException` | `structure` | message, resourceId, resourceType | The resource was not found for this request. Confirm the resource information, such as the ARN or type is correct and exists, then retry the request. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId, resourceType, serviceCode, quotaCode | The request denied due to exceeding the quota limits permitted. |
| `ThrottlingException` | `structure` | message, serviceCode, quotaCode, retryAfterSeconds | The request was denied due to request throttling. |
| `ValidationException` | `structure` | message | The input fails to satisfy the constraints specified by a service. |
| `ListSearchJobBackupsInput` | `structure` | SearchJobIdentifier, NextToken, MaxResults | - |
| `ListSearchJobBackupsOutput` | `structure` | Results, NextToken | - |
| `ListSearchJobResultsInput` | `structure` | SearchJobIdentifier, NextToken, MaxResults | - |
| `ListSearchJobResultsOutput` | `structure` | Results, NextToken | - |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `ExportJobStatus` | `enum` | RUNNING, FAILED, COMPLETED | - |
| `LongConditionOperator` | `enum` | EQUALS_TO, NOT_EQUALS_TO, LESS_THAN_EQUAL_TO, GREATER_THAN_EQUAL_TO | - |
| `ResourceType` | `enum` | S3, EBS | - |
| `SearchJobState` | `enum` | RUNNING, COMPLETED, STOPPING, STOPPED, FAILED | - |
| `StringConditionOperator` | `enum` | EQUALS_TO, NOT_EQUALS_TO, CONTAINS, DOES_NOT_CONTAIN, BEGINS_WITH, ENDS_WITH, DOES_NOT_BEGIN_WITH, DOES_NOT_END_WITH | - |
| `TimeConditionOperator` | `enum` | EQUALS_TO, NOT_EQUALS_TO, LESS_THAN_EQUAL_TO, GREATER_THAN_EQUAL_TO | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
