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

- Operations: `ListSearchJobBackups`, `ListSearchJobResults`, `ListSearchJobs`, `ListSearchResultExportJobs`, `ListTagsForResource`
- Traits: `paginated` (4), `readonly` (5)
- Common required input members in this group: `ResourceArn`, `SearchJobIdentifier`

### Get

- Operations: `GetSearchJob`, `GetSearchResultExportJob`
- Traits: `readonly` (2)
- Common required input members in this group: `ExportJobIdentifier`, `SearchJobIdentifier`

### Start

- Operations: `StartSearchJob`, `StartSearchResultExportJob`
- Traits: `idempotent` (2)
- Common required input members in this group: `ExportSpecification`, `SearchJobIdentifier`, `SearchScope`

### Stop

- Operations: `StopSearchJob`
- Traits: `idempotent` (1)
- Common required input members in this group: `SearchJobIdentifier`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `GetSearchJob` | `GET /search-jobs/{SearchJobIdentifier}` | `readonly` | `SearchJobIdentifier` | - | `GetSearchJobOutput` | `ResourceNotFoundException` | This operation retrieves metadata of a search job, including its progress. |
| `GetSearchResultExportJob` | `GET /export-search-jobs/{ExportJobIdentifier}` | `readonly` | `ExportJobIdentifier` | - | `GetSearchResultExportJobOutput` | `ResourceNotFoundException` | This operation retrieves the metadata of an export job. An export job is an operation that transmits the results of a search job to a specified S3 bucket in a .csv file. |
| `ListSearchJobBackups` | `GET /search-jobs/{SearchJobIdentifier}/backups` | `readonly`, `paginated` | `SearchJobIdentifier` | - | `ListSearchJobBackupsOutput` | `ResourceNotFoundException` | This operation returns a list of all backups (recovery points) in a paginated format that were included in the search job. If a search does not display an expected backup in the results, you can call this operation to display each backup included in the... |
| `ListSearchJobResults` | `GET /search-jobs/{SearchJobIdentifier}/search-results` | `readonly`, `paginated` | `SearchJobIdentifier` | - | `ListSearchJobResultsOutput` | `ResourceNotFoundException` | This operation returns a list of a specified search job. |
| `ListSearchJobs` | `GET /search-jobs` | `readonly`, `paginated` | - | - | `ListSearchJobsOutput` | - | This operation returns a list of search jobs belonging to an account. |
| `ListSearchResultExportJobs` | `GET /export-search-jobs` | `readonly`, `paginated` | - | - | `ListSearchResultExportJobsOutput` | `ResourceNotFoundException`, `ServiceQuotaExceededException` | This operation exports search results of a search job to a specified destination S3 bucket. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException` | This operation returns the tags for a resource type. |
| `StartSearchJob` | `PUT /search-jobs` | `idempotent` | `SearchScope` | - | `StartSearchJobOutput` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | This operation creates a search job which returns recovery points filtered by SearchScope and items filtered by ItemFilters. You can optionally include ClientToken, EncryptionKeyArn, Name, and/or Tags. |
| `StartSearchResultExportJob` | `PUT /export-search-jobs` | `idempotent` | `ExportSpecification`, `SearchJobIdentifier` | - | `StartSearchResultExportJobOutput` | `ConflictException`, `ResourceNotFoundException`, `ServiceQuotaExceededException` | This operations starts a job to export the results of search job to a designated S3 bucket. |
| `StopSearchJob` | `PUT /search-jobs/{SearchJobIdentifier}/actions/cancel` | `idempotent` | `SearchJobIdentifier` | - | `StopSearchJobOutput` | `ConflictException`, `ResourceNotFoundException` | This operations ends a search job. Only a search job with a status of `RUNNING` can be stopped. |
| `TagResource` | `POST /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `ResourceNotFoundException` | This operation puts tags on the resource you indicate. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException` | This operation removes tags from the specified resource. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ResourceNotFoundException` | `structure` | `message`, `resourceId`, `resourceType` | The resource was not found for this request. |
| `ServiceQuotaExceededException` | `structure` | `message`, `quotaCode`, `resourceId`, `resourceType`, `serviceCode` | The request denied due to exceeding the quota limits permitted. |
| `ConflictException` | `structure` | `message`, `resourceId`, `resourceType` | This exception occurs when a conflict with a previous successful operation is detected. |
| `GetSearchJobInput` | `structure` | `SearchJobIdentifier` | - |
| `GetSearchJobOutput` | `structure` | `CompletionTime`, `CreationTime`, `CurrentSearchProgress`, `EncryptionKeyArn`, `ItemFilters`, `Name`, `SearchJobArn`, `SearchJobIdentifier`, `SearchScope`, `SearchScopeSummary`, `Status`, `StatusMessage` | - |
| `GetSearchResultExportJobInput` | `structure` | `ExportJobIdentifier` | - |
| `GetSearchResultExportJobOutput` | `structure` | `CompletionTime`, `CreationTime`, `ExportJobArn`, `ExportJobIdentifier`, `ExportSpecification`, `SearchJobArn`, `Status`, `StatusMessage` | - |
| `ListSearchJobBackupsInput` | `structure` | `MaxResults`, `NextToken`, `SearchJobIdentifier` | - |
| `ListSearchJobBackupsOutput` | `structure` | `NextToken`, `Results` | - |
| `ListSearchJobResultsInput` | `structure` | `MaxResults`, `NextToken`, `SearchJobIdentifier` | - |
| `ListSearchJobResultsOutput` | `structure` | `NextToken`, `Results` | - |
| `ListSearchJobsInput` | `structure` | `ByStatus`, `MaxResults`, `NextToken` | - |
| `ListSearchJobsOutput` | `structure` | `NextToken`, `SearchJobs` | - |
| `ListSearchResultExportJobsInput` | `structure` | `MaxResults`, `NextToken`, `SearchJobIdentifier`, `Status` | - |
| `ListSearchResultExportJobsOutput` | `structure` | `ExportJobs`, `NextToken` | - |
| `ListTagsForResourceRequest` | `structure` | `ResourceArn` | - |
| `ListTagsForResourceResponse` | `structure` | `Tags` | - |
| `StartSearchJobInput` | `structure` | `ClientToken`, `EncryptionKeyArn`, `ItemFilters`, `Name`, `SearchScope`, `Tags` | - |
| `StartSearchJobOutput` | `structure` | `CreationTime`, `SearchJobArn`, `SearchJobIdentifier` | - |
| `StartSearchResultExportJobInput` | `structure` | `ClientToken`, `ExportSpecification`, `RoleArn`, `SearchJobIdentifier`, `Tags` | - |
| `StartSearchResultExportJobOutput` | `structure` | `ExportJobArn`, `ExportJobIdentifier` | - |
| `StopSearchJobInput` | `structure` | `SearchJobIdentifier` | - |
| `StopSearchJobOutput` | `structure` | - | - |
| `TagResourceRequest` | `structure` | `ResourceArn`, `Tags` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
