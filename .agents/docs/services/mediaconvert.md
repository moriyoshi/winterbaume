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

- Operations: `GetJob`, `GetJobsQueryResults`, `GetJobTemplate`, `GetPolicy`, `GetPreset`, `GetQueue`
- Common required input members in this group: `Id`, `Name`

### List

- Operations: `ListJobs`, `ListJobTemplates`, `ListPresets`, `ListQueues`, `ListTagsForResource`, `ListVersions`
- Traits: `paginated` (5)
- Common required input members in this group: -

### Create

- Operations: `CreateJob`, `CreateJobTemplate`, `CreatePreset`, `CreateQueue`, `CreateResourceShare`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `Settings`, `Name`

### Delete

- Operations: `DeleteJobTemplate`, `DeletePolicy`, `DeletePreset`, `DeleteQueue`
- Common required input members in this group: `Name`

### Update

- Operations: `UpdateJobTemplate`, `UpdatePreset`, `UpdateQueue`
- Common required input members in this group: `Name`

### Associate

- Operations: `AssociateCertificate`
- Common required input members in this group: -

### Cancel

- Operations: `CancelJob`
- Common required input members in this group: -

### Describe

- Operations: `DescribeEndpoints`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateCertificate`
- Common required input members in this group: -

### Probe

- Operations: `Probe`
- Common required input members in this group: -

### Put

- Operations: `PutPolicy`
- Common required input members in this group: -

### Search

- Operations: `SearchJobs`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Start

- Operations: `StartJobsQuery`
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
| `DescribeEndpoints` | `POST /2017-08-29/endpoints` | `paginated` | - | - | `DescribeEndpointsResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Send a request with an empty body to the regional API endpoint to get your account API endpoint. Note that DescribeEndpoints is no longer required. We recommend that you send your requests directly to the regional en ... |
| `DisassociateCertificate` | `DELETE /2017-08-29/certificates/{Arn}` | - | `Arn` | - | `DisassociateCertificateResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Removes an association between the Amazon Resource Name (ARN) of an AWS Certificate Manager (ACM) certificate and an AWS Elemental MediaConvert resource. |
| `GetJob` | `GET /2017-08-29/jobs/{Id}` | - | `Id` | - | `GetJobResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve the JSON for a specific transcoding job. |
| `GetJobsQueryResults` | `GET /2017-08-29/jobsQueries/{Id}` | - | `Id` | - | `GetJobsQueryResultsResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve a JSON array of up to twenty of your most recent jobs matched by a jobs query. |
| `GetJobTemplate` | `GET /2017-08-29/jobTemplates/{Name}` | - | `Name` | - | `GetJobTemplateResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve the JSON for a specific job template. |
| `GetPolicy` | `GET /2017-08-29/policy` | - | - | - | `GetPolicyResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve the JSON for your policy. |
| `GetPreset` | `GET /2017-08-29/presets/{Name}` | - | `Name` | - | `GetPresetResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve the JSON for a specific preset. |
| `GetQueue` | `GET /2017-08-29/queues/{Name}` | - | `Name` | - | `GetQueueResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve the JSON for a specific queue. |
| `ListJobs` | `GET /2017-08-29/jobs` | `paginated` | - | - | `ListJobsResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve a JSON array of up to twenty of your most recently created jobs. This array includes in-process, completed, and errored jobs. This will return the jobs themselves, not just a list of the jobs. To retrieve th ... |
| `ListJobTemplates` | `GET /2017-08-29/jobTemplates` | `paginated` | - | - | `ListJobTemplatesResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve a JSON array of up to twenty of your job templates. This will return the templates themselves, not just a list of them. To retrieve the next twenty templates, use the nextToken string returned with the array |
| `ListPresets` | `GET /2017-08-29/presets` | `paginated` | - | - | `ListPresetsResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve a JSON array of up to twenty of your presets. This will return the presets themselves, not just a list of them. To retrieve the next twenty presets, use the nextToken string returned with the array. |
| `ListQueues` | `GET /2017-08-29/queues` | `paginated` | - | - | `ListQueuesResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve a JSON array of up to twenty of your queues. This will return the queues themselves, not just a list of them. To retrieve the next twenty queues, use the nextToken string returned with the array. |
| `ListTagsForResource` | `GET /2017-08-29/tags/{Arn}` | - | `Arn` | - | `ListTagsForResourceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve the tags for a MediaConvert resource. |
| `ListVersions` | `GET /2017-08-29/versions` | `paginated` | - | - | `ListVersionsResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve a JSON array of all available Job engine versions and the date they expire. |
| `Probe` | `POST /2017-08-29/probe` | - | - | - | `ProbeResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Use Probe to obtain detailed information about your input media files. Probe returns a JSON that includes container, codec, frame rate, resolution, track count, audio layout, captions, and more. You can use this info ... |
| `PutPolicy` | `PUT /2017-08-29/policy` | - | `Policy` | - | `PutPolicyResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Create or change your policy. For more information about policies, see the user guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html |
| `SearchJobs` | `GET /2017-08-29/search` | `paginated` | - | - | `SearchJobsResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Retrieve a JSON array that includes job details for up to twenty of your most recent jobs. Optionally filter results further according to input file, queue, or status. To retrieve the twenty next most recent jobs, us ... |
| `StartJobsQuery` | `POST /2017-08-29/jobsQueries` | - | - | - | `StartJobsQueryResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Start an asynchronous jobs query using the provided filters. To receive the list of jobs that match your query, call the GetJobsQueryResults API using the query ID returned by this API. |
| `TagResource` | `POST /2017-08-29/tags` | - | `Arn`, `Tags` | - | `TagResourceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Add tags to a MediaConvert queue, preset, or job template. For information about tagging, see the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/tagging-resources.html |
| `UntagResource` | `PUT /2017-08-29/tags/{Arn}` | - | `Arn` | - | `UntagResourceResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Remove tags from a MediaConvert queue, preset, or job template. For information about tagging, see the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/tagging-resources.html |
| `UpdateJobTemplate` | `PUT /2017-08-29/jobTemplates/{Name}` | - | `Name` | - | `UpdateJobTemplateResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Modify one of your existing job templates. |
| `UpdatePreset` | `PUT /2017-08-29/presets/{Name}` | - | `Name` | - | `UpdatePresetResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Modify one of your existing presets. |
| `UpdateQueue` | `PUT /2017-08-29/queues/{Name}` | - | `Name` | - | `UpdateQueueResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `NotFoundException`, `ServiceQuotaExceededException`, `TooManyRequestsException` | Modify one of your existing queues. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `ListJobs` | - | `MaxResults -> maxResults`, `NextToken -> nextToken`, `Order -> order`, `Queue -> queue`, `Status -> status` | - | - |
| `ListJobTemplates` | - | `Category -> category`, `ListBy -> listBy`, `MaxResults -> maxResults`, `NextToken -> nextToken`, `Order -> order` | - | - |
| `ListPresets` | - | `Category -> category`, `ListBy -> listBy`, `MaxResults -> maxResults`, `NextToken -> nextToken`, `Order -> order` | - | - |
| `ListQueues` | - | `ListBy -> listBy`, `MaxResults -> maxResults`, `NextToken -> nextToken`, `Order -> order` | - | - |
| `ListVersions` | - | `MaxResults -> maxResults`, `NextToken -> nextToken` | - | - |
| `SearchJobs` | - | `InputFile -> inputFile`, `MaxResults -> maxResults`, `NextToken -> nextToken`, `Order -> order`, `Queue -> queue`, `Status -> status` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | Message | The service can't process your request because of a problem in the request. Please check your request form and syntax. |
| `ConflictException` | `structure` | Message | The service couldn't complete your request because there is a conflict with the current state of the resource. |
| `ForbiddenException` | `structure` | Message | You don't have permissions for this action with the credentials you sent. |
| `InternalServerErrorException` | `structure` | Message | The service encountered an unexpected condition and can't fulfill your request. |
| `NotFoundException` | `structure` | Message | The resource you requested doesn't exist. |
| `ServiceQuotaExceededException` | `structure` | Message | You attempted to create more resources than the service allows based on service quotas. |
| `TooManyRequestsException` | `structure` | Message | Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests. |
| `AssociateCertificateRequest` | `structure` | Arn | - |
| `AssociateCertificateResponse` | `structure` | **empty (no members)** | - |
| `CancelJobRequest` | `structure` | Id | - |
| `CancelJobResponse` | `structure` | **empty (no members)** | - |
| `CreateJobRequest` | `structure` | AccelerationSettings, BillingTagsSource, ClientRequestToken, HopDestinations, JobEngineVersion, JobTemplate, Priority, Queue, Role, Settings, SimulateReservedQueue, StatusUpdateInterval, ... (+2) | - |
| `CreateJobResponse` | `structure` | Job | - |
| `CreateJobTemplateRequest` | `structure` | AccelerationSettings, Category, Description, HopDestinations, Name, Priority, Queue, Settings, StatusUpdateInterval, Tags | - |
| `CreateJobTemplateResponse` | `structure` | JobTemplate | - |
| `CreatePresetRequest` | `structure` | Category, Description, Name, Settings, Tags | - |
| `CreatePresetResponse` | `structure` | Preset | - |
| `CreateQueueRequest` | `structure` | ConcurrentJobs, Description, MaximumConcurrentFeeds, Name, PricingPlan, ReservationPlanSettings, Status, Tags | - |
| `CreateQueueResponse` | `structure` | Queue | - |
| `CreateResourceShareRequest` | `structure` | JobId, SupportCaseId | - |
| `CreateResourceShareResponse` | `structure` | **empty (no members)** | - |
| `DeleteJobTemplateRequest` | `structure` | Name | - |
| `DeleteJobTemplateResponse` | `structure` | **empty (no members)** | - |
| `DeletePolicyRequest` | `structure` | **empty (no members)** | - |
| `DeletePolicyResponse` | `structure` | **empty (no members)** | - |
| `DeletePresetRequest` | `structure` | Name | - |
| `DeletePresetResponse` | `structure` | **empty (no members)** | - |
| `DeleteQueueRequest` | `structure` | Name | - |
| `DeleteQueueResponse` | `structure` | **empty (no members)** | - |
| `DescribeEndpointsRequest` | `structure` | MaxResults, Mode, NextToken | - |
| `DescribeEndpointsResponse` | `structure` | Endpoints, NextToken | - |
| `DisassociateCertificateRequest` | `structure` | Arn | - |
| `DisassociateCertificateResponse` | `structure` | **empty (no members)** | - |
| `GetJobRequest` | `structure` | Id | - |
| `GetJobResponse` | `structure` | Job | - |
| `GetJobsQueryResultsRequest` | `structure` | Id | - |
| `GetJobsQueryResultsResponse` | `structure` | Jobs, NextToken, Status | - |
| `GetJobTemplateRequest` | `structure` | Name | - |
| `GetJobTemplateResponse` | `structure` | JobTemplate | - |
| `GetPolicyRequest` | `structure` | **empty (no members)** | - |
| `AacAudioDescriptionBroadcasterMix` | `enum` | BROADCASTER_MIXED_AD, NORMAL | Choose BROADCASTER_MIXED_AD when the input contains pre-mixed main audio + audio description (AD) as a stereo pair. The value for AudioType will be set to 3 ... |
| `AacCodecProfile` | `enum` | LC, HEV1, HEV2, XHE | Specify the AAC profile. For the widest player compatibility and where higher bitrates are acceptable: Keep the default profile, LC (AAC-LC) For improved au ... |
| `AacCodingMode` | `enum` | AD_RECEIVER_MIX, CODING_MODE_1_0, CODING_MODE_1_1, CODING_MODE_2_0, CODING_MODE_5_1, CODING_MODE_AUTO | The Coding mode that you specify determines the number of audio channels and the audio channel layout metadata in your AAC output. Valid coding modes depend ... |
| `AacLoudnessMeasurementMode` | `enum` | PROGRAM, ANCHOR | Choose the loudness measurement mode for your audio content. For music or advertisements: We recommend that you keep the default value, Program. For speech ... |
| `AacRateControlMode` | `enum` | CBR, VBR | Specify the AAC rate control mode. For a constant bitrate: Choose CBR. Your AAC output bitrate will be equal to the value that you choose for Bitrate. For a ... |
| `AacRawFormat` | `enum` | LATM_LOAS, NONE | Enables LATM/LOAS AAC output. Note that if you use LATM/LOAS AAC in an output, you must choose "No container" for the output container. |
| `AacSpecification` | `enum` | MPEG2, MPEG4 | Use MPEG-2 AAC instead of MPEG-4 AAC audio for raw or MPEG-2 Transport Stream containers. |
| `AacVbrQuality` | `enum` | LOW, MEDIUM_LOW, MEDIUM_HIGH, HIGH | Specify the quality of your variable bitrate (VBR) AAC audio. For a list of approximate VBR bitrates, see: https://docs.aws.amazon.com/mediaconvert/latest/u ... |
| `Ac3BitstreamMode` | `enum` | COMPLETE_MAIN, COMMENTARY, DIALOGUE, EMERGENCY, HEARING_IMPAIRED, MUSIC_AND_EFFECTS, VISUALLY_IMPAIRED, VOICE_OVER | Specify the bitstream mode for the AC-3 stream that the encoder emits. For more information about the AC3 bitstream mode, see ATSC A/52-2012 (Annex E). |
| `Ac3CodingMode` | `enum` | CODING_MODE_1_0, CODING_MODE_1_1, CODING_MODE_2_0, CODING_MODE_3_2_LFE, CODING_MODE_AUTO | Dolby Digital coding mode. Determines number of channels. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
