# AmazonConnectCampaignService

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Provide APIs to create and manage Amazon Connect Campaigns.

## Possible Usage Scenarios
- Scenario insight from EC2: add full state-machine walks for AmazonConnectCampaignService resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AmazonConnectCampaignService workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `Delete`, `Update`, `List`, `Start` operation families, including `GetCampaignState`, `GetCampaignStateBatch`, `GetConnectInstanceConfig`, `GetInstanceOnboardingJobStatus`, `DeleteCampaign`, `DeleteConnectInstanceConfig`.

## Service Identity and Protocol

- AWS model slug: `connectcampaigns`
- AWS SDK for Rust slug: `connectcampaigns`
- Model version: `2021-01-30`
- Model file: `vendor/api-models-aws/models/connectcampaigns/service/2021-01-30/connectcampaigns-2021-01-30.json`
- SDK ID: `ConnectCampaigns`
- Endpoint prefix: `-`
- ARN namespace: `connect-campaigns`
- CloudFormation name: `-`
- CloudTrail event source: `connect-campaigns.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (4), `Delete` (3), `Update` (3), `List` (2), `Start` (2), `Create` (1), `Describe` (1), `Pause` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateCampaign`, `DeleteCampaign`, `DeleteConnectInstanceConfig`, `DeleteInstanceOnboardingJob`, `PutDialRequestBatch`, `StartCampaign`, `StartInstanceOnboardingJob`, `StopCampaign`, `TagResource`, `UntagResource`, `UpdateCampaignDialerConfig`, `UpdateCampaignName`, `UpdateCampaignOutboundCallConfig`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeCampaign`, `GetCampaignState`, `GetCampaignStateBatch`, `GetConnectInstanceConfig`, `GetInstanceOnboardingJobStatus`, `ListCampaigns`, `ListTagsForResource`.
- Pagination is modelled for 1 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 12 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DeleteInstanceOnboardingJob`, `GetInstanceOnboardingJobStatus`, `StartCampaign`, `StartInstanceOnboardingJob`, `StopCampaign`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 22 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `SQS`, `EC2/VPC`.

## Operation Groups

### Get

- Operations: `GetCampaignState`, `GetCampaignStateBatch`, `GetConnectInstanceConfig`, `GetInstanceOnboardingJobStatus`
- Traits: `readonly` (2)
- Common required input members in this group: `campaignIds`, `connectInstanceId`, `id`

### Delete

- Operations: `DeleteCampaign`, `DeleteConnectInstanceConfig`, `DeleteInstanceOnboardingJob`
- Traits: `idempotent` (3)
- Common required input members in this group: `connectInstanceId`, `id`

### Update

- Operations: `UpdateCampaignDialerConfig`, `UpdateCampaignName`, `UpdateCampaignOutboundCallConfig`
- Traits: `idempotent` (3)
- Common required input members in this group: `dialerConfig`, `id`, `name`

### List

- Operations: `ListCampaigns`, `ListTagsForResource`
- Traits: `idempotent` (1), `paginated` (1), `readonly` (1)
- Common required input members in this group: `arn`

### Start

- Operations: `StartCampaign`, `StartInstanceOnboardingJob`
- Traits: `idempotent` (1)
- Common required input members in this group: `connectInstanceId`, `encryptionConfig`, `id`

### Create

- Operations: `CreateCampaign`
- Traits: `idempotent` (1)
- Common required input members in this group: `connectInstanceId`, `dialerConfig`, `name`, `outboundCallConfig`

### Describe

- Operations: `DescribeCampaign`
- Traits: `readonly` (1)
- Common required input members in this group: `id`

### Pause

- Operations: `PauseCampaign`
- Common required input members in this group: `id`

### Put

- Operations: `PutDialRequestBatch`
- Traits: `idempotent` (1)
- Common required input members in this group: `dialRequests`, `id`

### Resume

- Operations: `ResumeCampaign`
- Common required input members in this group: `id`

### Stop

- Operations: `StopCampaign`
- Common required input members in this group: `id`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `arn`, `tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `arn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateCampaign` | `PUT /campaigns` | `idempotent` | `connectInstanceId`, `dialerConfig`, `name`, `outboundCallConfig` | - | `CreateCampaignResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a campaign for the specified Amazon Connect account. This API is idempotent. |
| `DeleteCampaign` | `DELETE /campaigns/{id}` | `idempotent` | `id` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a campaign from the specified Amazon Connect account. |
| `DeleteConnectInstanceConfig` | `DELETE /connect-instance/{connectInstanceId}/config` | `idempotent` | `connectInstanceId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `InvalidStateException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a connect instance config from the specified AWS account. |
| `DeleteInstanceOnboardingJob` | `DELETE /connect-instance/{connectInstanceId}/onboarding` | `idempotent` | `connectInstanceId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `InvalidStateException`, `ResourceNotFoundException`, `ValidationException` | Delete the Connect Campaigns onboarding job for the specified Amazon Connect instance. |
| `DescribeCampaign` | `GET /campaigns/{id}` | `readonly` | `id` | - | `DescribeCampaignResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes the specific campaign. |
| `GetCampaignState` | `GET /campaigns/{id}/state` | - | `id` | - | `GetCampaignStateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get state of a campaign for the specified Amazon Connect account. |
| `GetCampaignStateBatch` | `POST /campaigns-state` | - | `campaignIds` | - | `GetCampaignStateBatchResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Get state of campaigns for the specified Amazon Connect account. |
| `GetConnectInstanceConfig` | `GET /connect-instance/{connectInstanceId}/config` | `readonly` | `connectInstanceId` | - | `GetConnectInstanceConfigResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Get the specific Connect instance config. |
| `GetInstanceOnboardingJobStatus` | `GET /connect-instance/{connectInstanceId}/onboarding` | `readonly` | `connectInstanceId` | - | `GetInstanceOnboardingJobStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Get the specific instance onboarding job status. |
| `ListCampaigns` | `POST /campaigns-summary` | `readonly`, `paginated` | - | - | `ListCampaignsResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Provides summary information about the campaigns under the specified Amazon Connect account. |
| `ListTagsForResource` | `GET /tags/{arn}` | `idempotent` | `arn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List tags for a resource. |
| `PauseCampaign` | `POST /campaigns/{id}/pause` | - | `id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidCampaignStateException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Pauses a campaign for the specified Amazon Connect account. |
| `PutDialRequestBatch` | `PUT /campaigns/{id}/dial-requests` | `idempotent` | `dialRequests`, `id` | - | `PutDialRequestBatchResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidCampaignStateException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates dials requests for the specified campaign Amazon Connect account. This API is idempotent. |
| `ResumeCampaign` | `POST /campaigns/{id}/resume` | - | `id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidCampaignStateException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops a campaign for the specified Amazon Connect account. |
| `StartCampaign` | `POST /campaigns/{id}/start` | - | `id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidCampaignStateException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts a campaign for the specified Amazon Connect account. |
| `StartInstanceOnboardingJob` | `PUT /connect-instance/{connectInstanceId}/onboarding` | `idempotent` | `connectInstanceId`, `encryptionConfig` | - | `StartInstanceOnboardingJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Onboard the specific Amazon Connect instance to Connect Campaigns. |
| `StopCampaign` | `POST /campaigns/{id}/stop` | - | `id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidCampaignStateException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops a campaign for the specified Amazon Connect account. |
| `TagResource` | `POST /tags/{arn}` | `idempotent` | `arn`, `tags` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Tag a resource. |
| `UntagResource` | `DELETE /tags/{arn}` | `idempotent` | `arn`, `tagKeys` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Untag a resource. |
| `UpdateCampaignDialerConfig` | `POST /campaigns/{id}/dialer-config` | `idempotent` | `dialerConfig`, `id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates the dialer config of a campaign. This API is idempotent. |
| `UpdateCampaignName` | `POST /campaigns/{id}/name` | `idempotent` | `id`, `name` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates the name of a campaign. This API is idempotent. |
| `UpdateCampaignOutboundCallConfig` | `POST /campaigns/{id}/outbound-call-config` | `idempotent` | `id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the outbound call config of a campaign. This API is idempotent. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message`, `xAmzErrorType` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message`, `xAmzErrorType` | Request processing failed because of an error or failure with the service. |
| `ValidationException` | `structure` | `message`, `xAmzErrorType` | The input fails to satisfy the constraints specified by an AWS service. |
| `ResourceNotFoundException` | `structure` | `message`, `xAmzErrorType` | The specified resource was not found. |
| `ThrottlingException` | `structure` | `message`, `xAmzErrorType` | The request was denied due to request throttling. |
| `ConflictException` | `structure` | `message`, `xAmzErrorType` | The request could not be processed because of conflict in the current state of the resource. |
| `InvalidCampaignStateException` | `structure` | `message`, `state`, `xAmzErrorType` | The request could not be processed because of conflict in the current state of the campaign. |
| `InvalidStateException` | `structure` | `message`, `xAmzErrorType` | The request could not be processed because of conflict in the current state. |
| `CreateCampaignRequest` | `structure` | `connectInstanceId`, `dialerConfig`, `name`, `outboundCallConfig`, `tags` | The request for Create Campaign API. |
| `CreateCampaignResponse` | `structure` | `arn`, `id`, `tags` | The response for Create Campaign API |
| `ServiceQuotaExceededException` | `structure` | `message`, `xAmzErrorType` | Request would cause a service quota to be exceeded. |
| `DeleteCampaignRequest` | `structure` | `id` | DeleteCampaignRequest |
| `DeleteConnectInstanceConfigRequest` | `structure` | `connectInstanceId` | DeleteCampaignRequest |
| `DeleteInstanceOnboardingJobRequest` | `structure` | `connectInstanceId` | The request for DeleteInstanceOnboardingJob API. |
| `DescribeCampaignRequest` | `structure` | `id` | DescribeCampaignRequests |
| `DescribeCampaignResponse` | `structure` | `campaign` | DescribeCampaignResponse |
| `GetCampaignStateRequest` | `structure` | `id` | GetCampaignStateRequest |
| `GetCampaignStateResponse` | `structure` | `state` | GetCampaignStateResponse |
| `GetCampaignStateBatchRequest` | `structure` | `campaignIds` | GetCampaignStateBatchRequest |
| `GetCampaignStateBatchResponse` | `structure` | `failedRequests`, `successfulRequests` | GetCampaignStateBatchResponse |
| `GetConnectInstanceConfigRequest` | `structure` | `connectInstanceId` | GetConnectInstanceConfigRequest |
| `GetConnectInstanceConfigResponse` | `structure` | `connectInstanceConfig` | GetConnectInstanceConfigResponse |
| `GetInstanceOnboardingJobStatusRequest` | `structure` | `connectInstanceId` | GetInstanceOnboardingJobStatusRequest |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
