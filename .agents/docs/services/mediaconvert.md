# AWS Elemental MediaConvert

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Elemental MediaConvert

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Elemental MediaConvert where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Elemental MediaConvert by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for AWS Elemental MediaConvert resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AWS Elemental MediaConvert workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Create`, `Delete`, `Update` operation families, including `GetJob`, `GetJobTemplate`, `GetJobsQueryResults`, `GetPolicy`, `ListJobTemplates`, `ListJobs`.

## Service Identity and Protocol

- AWS model slug: `mediaconvert`
- AWS SDK for Rust slug: `mediaconvert`
- Model version: `2017-08-29`
- Model file: `vendor/api-models-aws/models/mediaconvert/service/2017-08-29/mediaconvert-2017-08-29.json`
- SDK ID: `MediaConvert`
- Endpoint prefix: `mediaconvert`
- ARN namespace: `mediaconvert`
- CloudFormation name: `MediaConvert`
- CloudTrail event source: `mediaconvert.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (6), `List` (6), `Create` (5), `Delete` (4), `Update` (3), `Associate` (1), `Cancel` (1), `Describe` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateCertificate`, `CancelJob`, `CreateJob`, `CreateJobTemplate`, `CreatePreset`, `CreateQueue`, `CreateResourceShare`, `DeleteJobTemplate`, `DeletePolicy`, `DeletePreset`, `DeleteQueue`, `DisassociateCertificate`, `PutPolicy`, `StartJobsQuery`, `TagResource`, `UntagResource`, `UpdateJobTemplate`, `UpdatePreset`, `UpdateQueue`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeEndpoints`, `GetJob`, `GetJobTemplate`, `GetJobsQueryResults`, `GetPolicy`, `GetPreset`, `GetQueue`, `ListJobTemplates`, `ListJobs`, `ListPresets`, `ListQueues`, `ListTagsForResource`, `ListVersions`, `SearchJobs`.
- Pagination is modelled for 7 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 1 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelJob`, `CreateJob`, `CreateJobTemplate`, `DeleteJobTemplate`, `GetJob`, `GetJobTemplate`, `GetJobsQueryResults`, `ListJobTemplates`, `ListJobs`, `SearchJobs`, `StartJobsQuery`, `UpdateJobTemplate`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 34 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `SQS`, `EC2/VPC`, `Secrets Manager`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetJob`, `GetJobTemplate`, `GetJobsQueryResults`, `GetPolicy`, `GetPreset`, `GetQueue`
- Common required input members in this group: `Id`, `Name`

### List

- Operations: `ListJobTemplates`, `ListJobs`, `ListPresets`, `ListQueues`, `ListTagsForResource`, `ListVersions`
- Traits: `paginated` (5)
- Common required input members in this group: `Arn`

### Create

- Operations: `CreateJob`, `CreateJobTemplate`, `CreatePreset`, `CreateQueue`, `CreateResourceShare`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `JobId`, `Name`, `Role`, `Settings`, `SupportCaseId`

### Delete

- Operations: `DeleteJobTemplate`, `DeletePolicy`, `DeletePreset`, `DeleteQueue`
- Common required input members in this group: `Name`

### Update

- Operations: `UpdateJobTemplate`, `UpdatePreset`, `UpdateQueue`
- Common required input members in this group: `Name`

### Associate

- Operations: `AssociateCertificate`
- Common required input members in this group: `Arn`

### Cancel

- Operations: `CancelJob`
- Common required input members in this group: `Id`

### Describe

- Operations: `DescribeEndpoints`
- Traits: `paginated` (1)

### Disassociate

- Operations: `DisassociateCertificate`
- Common required input members in this group: `Arn`

### Probe

- Operations: `Probe`

### Put

- Operations: `PutPolicy`
- Common required input members in this group: `Policy`

### Search

- Operations: `SearchJobs`
- Traits: `paginated` (1)

### Start

- Operations: `StartJobsQuery`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `Arn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `Arn`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateCertificate` | `POST /2017-08-29/certificates` | - | `Arn` | - | `AssociateCertificateResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Associates an AWS Certificate Manager (ACM) Amazon Resource Name (ARN) with AWS Elemental MediaConvert. |
| `CancelJob` | `DELETE /2017-08-29/jobs/{Id}` | - | `Id` | - | `CancelJobResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Permanently cancel a job. Once you have canceled a job, you can't start it again. |
| `CreateJob` | `POST /2017-08-29/jobs` | `idempotency-token` | `Role`, `Settings` | `ClientRequestToken` | `CreateJobResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Create a new transcoding job. For information about jobs and job settings, see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html |
| `CreateJobTemplate` | `POST /2017-08-29/jobTemplates` | - | `Name`, `Settings` | - | `CreateJobTemplateResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Create a new job template. For information about job templates see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html |
| `CreatePreset` | `POST /2017-08-29/presets` | - | `Name`, `Settings` | - | `CreatePresetResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Create a new preset. For information about job templates see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html |
| `CreateQueue` | `POST /2017-08-29/queues` | - | `Name` | - | `CreateQueueResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Create a new transcoding queue. For information about queues, see Working With Queues in the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/working-with-queues.html |
| `CreateResourceShare` | `POST /2017-08-29/resourceShares` | - | `JobId`, `SupportCaseId` | - | `CreateResourceShareResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Create a new resource share request for MediaConvert resources with AWS Support. |
| `DeleteJobTemplate` | `DELETE /2017-08-29/jobTemplates/{Name}` | - | `Name` | - | `DeleteJobTemplateResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Permanently delete a job template you have created. |
| `DeletePolicy` | `DELETE /2017-08-29/policy` | - | - | - | `DeletePolicyResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Permanently delete a policy that you created. |
| `DeletePreset` | `DELETE /2017-08-29/presets/{Name}` | - | `Name` | - | `DeletePresetResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Permanently delete a preset you have created. |
| `DeleteQueue` | `DELETE /2017-08-29/queues/{Name}` | - | `Name` | - | `DeleteQueueResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Permanently delete a queue you have created. |
| `DescribeEndpoints` | `POST /2017-08-29/endpoints` | `paginated` | - | - | `DescribeEndpointsResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Send a request with an empty body to the regional API endpoint to get your account API endpoint. Note that DescribeEndpoints is no longer required. |
| `DisassociateCertificate` | `DELETE /2017-08-29/certificates/{Arn}` | - | `Arn` | - | `DisassociateCertificateResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Removes an association between the Amazon Resource Name (ARN) of an AWS Certificate Manager (ACM) certificate and an AWS Elemental MediaConvert resource. |
| `GetJob` | `GET /2017-08-29/jobs/{Id}` | - | `Id` | - | `GetJobResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve the JSON for a specific transcoding job. |
| `GetJobTemplate` | `GET /2017-08-29/jobTemplates/{Name}` | - | `Name` | - | `GetJobTemplateResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve the JSON for a specific job template. |
| `GetJobsQueryResults` | `GET /2017-08-29/jobsQueries/{Id}` | - | `Id` | - | `GetJobsQueryResultsResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve a JSON array of up to twenty of your most recent jobs matched by a jobs query. |
| `GetPolicy` | `GET /2017-08-29/policy` | - | - | - | `GetPolicyResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve the JSON for your policy. |
| `GetPreset` | `GET /2017-08-29/presets/{Name}` | - | `Name` | - | `GetPresetResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve the JSON for a specific preset. |
| `GetQueue` | `GET /2017-08-29/queues/{Name}` | - | `Name` | - | `GetQueueResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve the JSON for a specific queue. |
| `ListJobTemplates` | `GET /2017-08-29/jobTemplates` | `paginated` | - | - | `ListJobTemplatesResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve a JSON array of up to twenty of your job templates. This will return the templates themselves, not just a list of them. |
| `ListJobs` | `GET /2017-08-29/jobs` | `paginated` | - | - | `ListJobsResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve a JSON array of up to twenty of your most recently created jobs. This array includes in-process, completed, and errored jobs. |
| `ListPresets` | `GET /2017-08-29/presets` | `paginated` | - | - | `ListPresetsResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve a JSON array of up to twenty of your presets. This will return the presets themselves, not just a list of them. |
| `ListQueues` | `GET /2017-08-29/queues` | `paginated` | - | - | `ListQueuesResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve a JSON array of up to twenty of your queues. This will return the queues themselves, not just a list of them. |
| `ListTagsForResource` | `GET /2017-08-29/tags/{Arn}` | - | `Arn` | - | `ListTagsForResourceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve the tags for a MediaConvert resource. |
| `ListVersions` | `GET /2017-08-29/versions` | `paginated` | - | - | `ListVersionsResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve a JSON array of all available Job engine versions and the date they expire. |
| `Probe` | `POST /2017-08-29/probe` | - | - | - | `ProbeResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Use Probe to obtain detailed information about your input media files. Probe returns a JSON that includes container, codec, frame rate, resolution, track count, audio layout, captions, and more. |
| `PutPolicy` | `PUT /2017-08-29/policy` | - | `Policy` | - | `PutPolicyResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Create or change your policy. For more information about policies, see the user guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html |
| `SearchJobs` | `GET /2017-08-29/search` | `paginated` | - | - | `SearchJobsResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve a JSON array that includes job details for up to twenty of your most recent jobs. Optionally filter results further according to input file, queue, or status. |
| `StartJobsQuery` | `POST /2017-08-29/jobsQueries` | - | - | - | `StartJobsQueryResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Start an asynchronous jobs query using the provided filters. To receive the list of jobs that match your query, call the GetJobsQueryResults API using the query ID returned by this API. |
| `TagResource` | `POST /2017-08-29/tags` | - | `Arn`, `Tags` | - | `TagResourceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Add tags to a MediaConvert queue, preset, or job template. For information about tagging, see the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/tagging-resources.html |
| `UntagResource` | `PUT /2017-08-29/tags/{Arn}` | - | `Arn` | - | `UntagResourceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Remove tags from a MediaConvert queue, preset, or job template. For information about tagging, see the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/tagging-resources.html |
| `UpdateJobTemplate` | `PUT /2017-08-29/jobTemplates/{Name}` | - | `Name` | - | `UpdateJobTemplateResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Modify one of your existing job templates. |
| `UpdatePreset` | `PUT /2017-08-29/presets/{Name}` | - | `Name` | - | `UpdatePresetResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Modify one of your existing presets. |
| `UpdateQueue` | `PUT /2017-08-29/queues/{Name}` | - | `Name` | - | `UpdateQueueResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Modify one of your existing queues. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `Message` | The service can't process your request because of a problem in the request. |
| `ConflictException` | `structure` | `Message` | The service couldn't complete your request because there is a conflict with the current state of the resource. |
| `ForbiddenException` | `structure` | `Message` | You don't have permissions for this action with the credentials you sent. |
| `InternalServerErrorException` | `structure` | `Message` | The service encountered an unexpected condition and can't fulfill your request. |
| `NotFoundException` | `structure` | `Message` | The resource you requested doesn't exist. |
| `ServiceQuotaExceededException` | `structure` | `Message` | You attempted to create more resources than the service allows based on service quotas. |
| `TooManyRequestsException` | `structure` | `Message` | Too many requests have been sent in too short of a time. |
| `AssociateCertificateRequest` | `structure` | `Arn` | - |
| `AssociateCertificateResponse` | `structure` | - | - |
| `CancelJobRequest` | `structure` | `Id` | - |
| `CancelJobResponse` | `structure` | - | - |
| `CreateJobRequest` | `structure` | `AccelerationSettings`, `BillingTagsSource`, `ClientRequestToken`, `HopDestinations`, `JobEngineVersion`, `JobTemplate`, `Priority`, `Queue`, `Role`, `Settings`, `SimulateReservedQueue`, `StatusUpdateInterval`, ... (+2) | - |
| `CreateJobResponse` | `structure` | `Job` | - |
| `CreateJobTemplateRequest` | `structure` | `AccelerationSettings`, `Category`, `Description`, `HopDestinations`, `Name`, `Priority`, `Queue`, `Settings`, `StatusUpdateInterval`, `Tags` | - |
| `CreateJobTemplateResponse` | `structure` | `JobTemplate` | - |
| `CreatePresetRequest` | `structure` | `Category`, `Description`, `Name`, `Settings`, `Tags` | - |
| `CreatePresetResponse` | `structure` | `Preset` | - |
| `CreateQueueRequest` | `structure` | `ConcurrentJobs`, `Description`, `Name`, `PricingPlan`, `ReservationPlanSettings`, `Status`, `Tags` | - |
| `CreateQueueResponse` | `structure` | `Queue` | - |
| `CreateResourceShareRequest` | `structure` | `JobId`, `SupportCaseId` | - |
| `CreateResourceShareResponse` | `structure` | - | - |
| `DeleteJobTemplateRequest` | `structure` | `Name` | - |
| `DeleteJobTemplateResponse` | `structure` | - | - |
| `DeletePolicyRequest` | `structure` | - | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
