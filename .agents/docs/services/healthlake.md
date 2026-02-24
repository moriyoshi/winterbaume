# Amazon HealthLake

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

This is the AWS HealthLake API Reference . For an introduction to the service, see What is AWS HealthLake? in the AWS HealthLake Developer Guide .

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for Amazon HealthLake resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Amazon HealthLake workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Describe`, `Start`, `Create`, `Delete` operation families, including `ListFHIRDatastores`, `ListFHIRExportJobs`, `ListFHIRImportJobs`, `ListTagsForResource`, `DescribeFHIRDatastore`, `DescribeFHIRExportJob`.

## Service Identity and Protocol

- AWS model slug: `healthlake`
- AWS SDK for Rust slug: `healthlake`
- Model version: `2017-07-01`
- Model file: `vendor/api-models-aws/models/healthlake/service/2017-07-01/healthlake-2017-07-01.json`
- SDK ID: `HealthLake`
- Endpoint prefix: `healthlake`
- ARN namespace: `healthlake`
- CloudFormation name: `HealthLake`
- CloudTrail event source: `healthlake.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (4), `Describe` (3), `Start` (2), `Create` (1), `Delete` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateFHIRDatastore`, `DeleteFHIRDatastore`, `StartFHIRExportJob`, `StartFHIRImportJob`, `TagResource`, `UntagResource`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeFHIRDatastore`, `DescribeFHIRExportJob`, `DescribeFHIRImportJob`, `ListFHIRDatastores`, `ListFHIRExportJobs`, `ListFHIRImportJobs`, `ListTagsForResource`.
- Pagination is modelled for 3 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 3 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeFHIRExportJob`, `DescribeFHIRImportJob`, `ListFHIRExportJobs`, `ListFHIRImportJobs`, `StartFHIRExportJob`, `StartFHIRImportJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 13 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `KMS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### List

- Operations: `ListFHIRDatastores`, `ListFHIRExportJobs`, `ListFHIRImportJobs`, `ListTagsForResource`
- Traits: `paginated` (3)
- Common required input members in this group: `DatastoreId`, `ResourceARN`

### Describe

- Operations: `DescribeFHIRDatastore`, `DescribeFHIRExportJob`, `DescribeFHIRImportJob`
- Common required input members in this group: `DatastoreId`, `JobId`

### Start

- Operations: `StartFHIRExportJob`, `StartFHIRImportJob`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `DataAccessRoleArn`, `DatastoreId`, `InputDataConfig`, `JobOutputDataConfig`, `OutputDataConfig`

### Create

- Operations: `CreateFHIRDatastore`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `DatastoreTypeVersion`

### Delete

- Operations: `DeleteFHIRDatastore`
- Common required input members in this group: `DatastoreId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceARN`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceARN`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateFHIRDatastore` | - | `idempotency-token` | `DatastoreTypeVersion` | `ClientToken` | `CreateFHIRDatastoreResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Create a FHIR-enabled data store. |
| `DeleteFHIRDatastore` | - | - | `DatastoreId` | - | `DeleteFHIRDatastoreResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete a FHIR-enabled data store. |
| `DescribeFHIRDatastore` | - | - | `DatastoreId` | - | `DescribeFHIRDatastoreResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get properties for a FHIR-enabled data store. |
| `DescribeFHIRExportJob` | - | - | `DatastoreId`, `JobId` | - | `DescribeFHIRExportJobResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get FHIR export job properties. |
| `DescribeFHIRImportJob` | - | - | `DatastoreId`, `JobId` | - | `DescribeFHIRImportJobResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get the import job properties to learn more about the job or job progress. |
| `ListFHIRDatastores` | - | `paginated` | - | - | `ListFHIRDatastoresResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | List all FHIR-enabled data stores in a user’s account, regardless of data store status. |
| `ListFHIRExportJobs` | - | `paginated` | `DatastoreId` | - | `ListFHIRExportJobsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all FHIR export jobs associated with an account and their statuses. |
| `ListFHIRImportJobs` | - | `paginated` | `DatastoreId` | - | `ListFHIRImportJobsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List all FHIR import jobs associated with an account and their statuses. |
| `ListTagsForResource` | - | - | `ResourceARN` | - | `ListTagsForResourceResponse` | `ResourceNotFoundException`, `ValidationException` | Returns a list of all existing tags associated with a data store. |
| `StartFHIRExportJob` | - | `idempotency-token` | `DataAccessRoleArn`, `DatastoreId`, `OutputDataConfig` | `ClientToken` | `StartFHIRExportJobResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Start a FHIR export job. |
| `StartFHIRImportJob` | - | `idempotency-token` | `DataAccessRoleArn`, `DatastoreId`, `InputDataConfig`, `JobOutputDataConfig` | `ClientToken` | `StartFHIRImportJobResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Start importing bulk FHIR data into an ACTIVE data store. The import job imports FHIR data found in the `InputDataConfig` object and stores processing results in the `JobOutputDataConfig` object. |
| `TagResource` | - | - | `ResourceARN`, `Tags` | - | `TagResourceResponse` | `ResourceNotFoundException`, `ValidationException` | Add a user-specifed key and value tag to a data store. |
| `UntagResource` | - | - | `ResourceARN`, `TagKeys` | - | `UntagResourceResponse` | `ResourceNotFoundException`, `ValidationException` | Remove a user-specifed key and value tag from a data store. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ValidationException` | `structure` | `Message` | The user input parameter was invalid. |
| `ResourceNotFoundException` | `structure` | `Message` | The requested data store was not found. |
| `InternalServerException` | `structure` | `Message` | An unknown internal error occurred in the service. |
| `ThrottlingException` | `structure` | `Message` | The user has exceeded their maximum number of allowed calls to the given API. |
| `AccessDeniedException` | `structure` | `Message` | Access is denied. |
| `CreateFHIRDatastoreRequest` | `structure` | `ClientToken`, `DatastoreName`, `DatastoreTypeVersion`, `IdentityProviderConfiguration`, `PreloadDataConfig`, `SseConfiguration`, `Tags` | - |
| `CreateFHIRDatastoreResponse` | `structure` | `DatastoreArn`, `DatastoreEndpoint`, `DatastoreId`, `DatastoreStatus` | - |
| `DeleteFHIRDatastoreRequest` | `structure` | `DatastoreId` | - |
| `DeleteFHIRDatastoreResponse` | `structure` | `DatastoreArn`, `DatastoreEndpoint`, `DatastoreId`, `DatastoreStatus` | - |
| `ConflictException` | `structure` | `Message` | The data store is in a transition state and the user requested action cannot be performed. |
| `DescribeFHIRDatastoreRequest` | `structure` | `DatastoreId` | - |
| `DescribeFHIRDatastoreResponse` | `structure` | `DatastoreProperties` | - |
| `DescribeFHIRExportJobRequest` | `structure` | `DatastoreId`, `JobId` | - |
| `DescribeFHIRExportJobResponse` | `structure` | `ExportJobProperties` | - |
| `DescribeFHIRImportJobRequest` | `structure` | `DatastoreId`, `JobId` | - |
| `DescribeFHIRImportJobResponse` | `structure` | `ImportJobProperties` | - |
| `ListFHIRDatastoresRequest` | `structure` | `Filter`, `MaxResults`, `NextToken` | - |
| `ListFHIRDatastoresResponse` | `structure` | `DatastorePropertiesList`, `NextToken` | - |
| `ListFHIRExportJobsRequest` | `structure` | `DatastoreId`, `JobName`, `JobStatus`, `MaxResults`, `NextToken`, `SubmittedAfter`, `SubmittedBefore` | - |
| `ListFHIRExportJobsResponse` | `structure` | `ExportJobPropertiesList`, `NextToken` | - |
| `ListFHIRImportJobsRequest` | `structure` | `DatastoreId`, `JobName`, `JobStatus`, `MaxResults`, `NextToken`, `SubmittedAfter`, `SubmittedBefore` | - |
| `ListFHIRImportJobsResponse` | `structure` | `ImportJobPropertiesList`, `NextToken` | - |
| `ListTagsForResourceRequest` | `structure` | `ResourceARN` | - |
| `ListTagsForResourceResponse` | `structure` | `Tags` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
