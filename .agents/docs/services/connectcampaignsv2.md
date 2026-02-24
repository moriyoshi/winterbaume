# AmazonConnectCampaignServiceV2

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Provide APIs to create and manage Amazon Connect Campaigns.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AmazonConnectCampaignServiceV2 where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AmazonConnectCampaignServiceV2 by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for AmazonConnectCampaignServiceV2 resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented AmazonConnectCampaignServiceV2 workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Delete`, `Update`, `Get`, `Put`, `List` operation families, including `DeleteCampaign`, `DeleteCampaignChannelSubtypeConfig`, `DeleteCampaignCommunicationLimits`, `DeleteCampaignCommunicationTime`, `UpdateCampaignChannelSubtypeConfig`, `UpdateCampaignCommunicationLimits`.

## Service Identity and Protocol

- AWS model slug: `connectcampaignsv2`
- AWS SDK for Rust slug: `connectcampaigns`
- Model version: `2024-04-23`
- Model file: `vendor/api-models-aws/models/connectcampaignsv2/service/2024-04-23/connectcampaignsv2-2024-04-23.json`
- SDK ID: `ConnectCampaignsV2`
- Endpoint prefix: `-`
- ARN namespace: `connect-campaigns`
- CloudFormation name: `-`
- CloudTrail event source: `connect-campaigns.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Delete` (7), `Update` (7), `Get` (5), `Put` (4), `List` (3), `Start` (2), `Create` (1), `Describe` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateCampaign`, `DeleteCampaign`, `DeleteCampaignChannelSubtypeConfig`, `DeleteCampaignCommunicationLimits`, `DeleteCampaignCommunicationTime`, `DeleteConnectInstanceConfig`, `DeleteConnectInstanceIntegration`, `DeleteInstanceOnboardingJob`, `PutConnectInstanceIntegration`, `PutInstanceCommunicationLimits`, `PutOutboundRequestBatch`, `PutProfileOutboundRequestBatch`, `StartCampaign`, `StartInstanceOnboardingJob`, `StopCampaign`, `TagResource`, `UntagResource`, `UpdateCampaignChannelSubtypeConfig`, `UpdateCampaignCommunicationLimits`, `UpdateCampaignCommunicationTime`, ... (+4).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeCampaign`, `GetCampaignState`, `GetCampaignStateBatch`, `GetConnectInstanceConfig`, `GetInstanceCommunicationLimits`, `GetInstanceOnboardingJobStatus`, `ListCampaigns`, `ListConnectInstanceIntegrations`, `ListTagsForResource`.
- Pagination is modelled for 2 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 21 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DeleteInstanceOnboardingJob`, `GetInstanceOnboardingJobStatus`, `StartCampaign`, `StartInstanceOnboardingJob`, `StopCampaign`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 35 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `SQS`, `Lambda`, `EC2/VPC`.

## Operation Groups

### Delete

- Operations: `DeleteCampaign`, `DeleteCampaignChannelSubtypeConfig`, `DeleteCampaignCommunicationLimits`, `DeleteCampaignCommunicationTime`, `DeleteConnectInstanceConfig`, `DeleteConnectInstanceIntegration`, `DeleteInstanceOnboardingJob`
- Traits: `idempotent` (6)
- Common required input members in this group: `channelSubtype`, `config`, `connectInstanceId`, `id`, `integrationIdentifier`

### Update

- Operations: `UpdateCampaignChannelSubtypeConfig`, `UpdateCampaignCommunicationLimits`, `UpdateCampaignCommunicationTime`, `UpdateCampaignFlowAssociation`, `UpdateCampaignName`, `UpdateCampaignSchedule`, `UpdateCampaignSource`
- Traits: `idempotent` (7)
- Common required input members in this group: `channelSubtypeConfig`, `communicationLimitsOverride`, `communicationTimeConfig`, `connectCampaignFlowArn`, `id`, `name`, `schedule`, `source`

### Get

- Operations: `GetCampaignState`, `GetCampaignStateBatch`, `GetConnectInstanceConfig`, `GetInstanceCommunicationLimits`, `GetInstanceOnboardingJobStatus`
- Traits: `readonly` (3)
- Common required input members in this group: `campaignIds`, `connectInstanceId`, `id`

### Put

- Operations: `PutConnectInstanceIntegration`, `PutInstanceCommunicationLimits`, `PutOutboundRequestBatch`, `PutProfileOutboundRequestBatch`
- Traits: `idempotent` (4)
- Common required input members in this group: `communicationLimitsConfig`, `connectInstanceId`, `id`, `integrationConfig`, `outboundRequests`, `profileOutboundRequests`

### List

- Operations: `ListCampaigns`, `ListConnectInstanceIntegrations`, `ListTagsForResource`
- Traits: `paginated` (2), `readonly` (2)
- Common required input members in this group: `arn`, `connectInstanceId`

### Start

- Operations: `StartCampaign`, `StartInstanceOnboardingJob`
- Traits: `idempotent` (1)
- Common required input members in this group: `connectInstanceId`, `encryptionConfig`, `id`

### Create

- Operations: `CreateCampaign`
- Traits: `idempotent` (1)
- Common required input members in this group: `connectInstanceId`, `name`

### Describe

- Operations: `DescribeCampaign`
- Traits: `readonly` (1)
- Common required input members in this group: `id`

### Pause

- Operations: `PauseCampaign`
- Common required input members in this group: `id`

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
| `CreateCampaign` | `PUT /v2/campaigns` | `idempotent` | `connectInstanceId`, `name` | - | `CreateCampaignResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a campaign for the specified Amazon Connect account. This API is idempotent. |
| `DeleteCampaign` | `DELETE /v2/campaigns/{id}` | `idempotent` | `id` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes a campaign from the specified Amazon Connect account. |
| `DeleteCampaignChannelSubtypeConfig` | `DELETE /v2/campaigns/{id}/channel-subtype-config` | `idempotent` | `channelSubtype`, `id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Deletes the channel subtype config of a campaign. This API is idempotent. |
| `DeleteCampaignCommunicationLimits` | `DELETE /v2/campaigns/{id}/communication-limits` | `idempotent` | `config`, `id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidCampaignStateException`, `ResourceNotFoundException`, `ValidationException` | Deletes the communication limits config for a campaign. This API is idempotent. |
| `DeleteCampaignCommunicationTime` | `DELETE /v2/campaigns/{id}/communication-time` | `idempotent` | `config`, `id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidCampaignStateException`, `ResourceNotFoundException`, `ValidationException` | Deletes the communication time config for a campaign. This API is idempotent. |
| `DeleteConnectInstanceConfig` | `DELETE /v2/connect-instance/{connectInstanceId}/config` | `idempotent` | `connectInstanceId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `InvalidStateException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a connect instance config from the specified AWS account. |
| `DeleteConnectInstanceIntegration` | `POST /v2/connect-instance/{connectInstanceId}/integrations/delete` | - | `connectInstanceId`, `integrationIdentifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Delete the integration for the specified Amazon Connect instance. |
| `DeleteInstanceOnboardingJob` | `DELETE /v2/connect-instance/{connectInstanceId}/onboarding` | `idempotent` | `connectInstanceId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `InvalidStateException`, `ResourceNotFoundException`, `ValidationException` | Delete the Connect Campaigns onboarding job for the specified Amazon Connect instance. |
| `DescribeCampaign` | `GET /v2/campaigns/{id}` | `readonly` | `id` | - | `DescribeCampaignResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Describes the specific campaign. |
| `GetCampaignState` | `GET /v2/campaigns/{id}/state` | - | `id` | - | `GetCampaignStateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Get state of a campaign for the specified Amazon Connect account. |
| `GetCampaignStateBatch` | `POST /v2/campaigns-state` | - | `campaignIds` | - | `GetCampaignStateBatchResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Get state of campaigns for the specified Amazon Connect account. |
| `GetConnectInstanceConfig` | `GET /v2/connect-instance/{connectInstanceId}/config` | `readonly` | `connectInstanceId` | - | `GetConnectInstanceConfigResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Get the specific Connect instance config. |
| `GetInstanceCommunicationLimits` | `GET /v2/connect-instance/{connectInstanceId}/communication-limits` | `readonly` | `connectInstanceId` | - | `GetInstanceCommunicationLimitsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Get the instance communication limits. |
| `GetInstanceOnboardingJobStatus` | `GET /v2/connect-instance/{connectInstanceId}/onboarding` | `readonly` | `connectInstanceId` | - | `GetInstanceOnboardingJobStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Get the specific instance onboarding job status. |
| `ListCampaigns` | `POST /v2/campaigns-summary` | `readonly`, `paginated` | - | - | `ListCampaignsResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Provides summary information about the campaigns under the specified Amazon Connect account. |
| `ListConnectInstanceIntegrations` | `GET /v2/connect-instance/{connectInstanceId}/integrations` | `readonly`, `paginated` | `connectInstanceId` | - | `ListConnectInstanceIntegrationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Provides summary information about the integration under the specified Connect instance. |
| `ListTagsForResource` | `GET /v2/tags/{arn}` | - | `arn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | List tags for a resource. |
| `PauseCampaign` | `POST /v2/campaigns/{id}/pause` | - | `id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidCampaignStateException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Pauses a campaign for the specified Amazon Connect account. |
| `PutConnectInstanceIntegration` | `PUT /v2/connect-instance/{connectInstanceId}/integrations` | `idempotent` | `connectInstanceId`, `integrationConfig` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Put or update the integration for the specified Amazon Connect instance. |
| `PutInstanceCommunicationLimits` | `PUT /v2/connect-instance/{connectInstanceId}/communication-limits` | `idempotent` | `communicationLimitsConfig`, `connectInstanceId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Put the instance communication limits. This API is idempotent. |
| `PutOutboundRequestBatch` | `PUT /v2/campaigns/{id}/outbound-requests` | `idempotent` | `id`, `outboundRequests` | - | `PutOutboundRequestBatchResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidCampaignStateException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates outbound requests for the specified campaign Amazon Connect account. This API is idempotent. |
| `PutProfileOutboundRequestBatch` | `PUT /v2/campaigns/{id}/profile-outbound-requests` | `idempotent` | `id`, `profileOutboundRequests` | - | `PutProfileOutboundRequestBatchResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidCampaignStateException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Takes in a list of profile outbound requests to be placed as part of an outbound campaign. This API is idempotent. |
| `ResumeCampaign` | `POST /v2/campaigns/{id}/resume` | - | `id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidCampaignStateException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops a campaign for the specified Amazon Connect account. |
| `StartCampaign` | `POST /v2/campaigns/{id}/start` | - | `id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidCampaignStateException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts a campaign for the specified Amazon Connect account. |
| `StartInstanceOnboardingJob` | `PUT /v2/connect-instance/{connectInstanceId}/onboarding` | `idempotent` | `connectInstanceId`, `encryptionConfig` | - | `StartInstanceOnboardingJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Onboard the specific Amazon Connect instance to Connect Campaigns. |
| `StopCampaign` | `POST /v2/campaigns/{id}/stop` | - | `id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidCampaignStateException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops a campaign for the specified Amazon Connect account. |
| `TagResource` | `POST /v2/tags/{arn}` | `idempotent` | `arn`, `tags` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Tag a resource. |
| `UntagResource` | `DELETE /v2/tags/{arn}` | `idempotent` | `arn`, `tagKeys` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Untag a resource. |
| `UpdateCampaignChannelSubtypeConfig` | `POST /v2/campaigns/{id}/channel-subtype-config` | `idempotent` | `channelSubtypeConfig`, `id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates the channel subtype config of a campaign. This API is idempotent. |
| `UpdateCampaignCommunicationLimits` | `POST /v2/campaigns/{id}/communication-limits` | `idempotent` | `communicationLimitsOverride`, `id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidCampaignStateException`, `ResourceNotFoundException`, `ValidationException` | Updates the communication limits config for a campaign. This API is idempotent. |
| `UpdateCampaignCommunicationTime` | `POST /v2/campaigns/{id}/communication-time` | `idempotent` | `communicationTimeConfig`, `id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidCampaignStateException`, `ResourceNotFoundException`, `ValidationException` | Updates the communication time config for a campaign. This API is idempotent. |
| `UpdateCampaignFlowAssociation` | `POST /v2/campaigns/{id}/flow` | `idempotent` | `connectCampaignFlowArn`, `id` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidCampaignStateException`, `ResourceNotFoundException`, `ValidationException` | Updates the campaign flow associated with a campaign. This API is idempotent. |
| `UpdateCampaignName` | `POST /v2/campaigns/{id}/name` | `idempotent` | `id`, `name` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Updates the name of a campaign. This API is idempotent. |
| `UpdateCampaignSchedule` | `POST /v2/campaigns/{id}/schedule` | `idempotent` | `id`, `schedule` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidCampaignStateException`, `ResourceNotFoundException`, `ValidationException` | Updates the schedule for a campaign. This API is idempotent. |
| `UpdateCampaignSource` | `POST /v2/campaigns/{id}/source` | `idempotent` | `id`, `source` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `InvalidCampaignStateException`, `ResourceNotFoundException`, `ValidationException` | Updates the campaign source with a campaign. This API is idempotent. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message`, `xAmzErrorType` | You do not have sufficient access to perform this action. |
| `InternalServerException` | `structure` | `message`, `xAmzErrorType` | Request processing failed because of an error or failure with the service. |
| `ValidationException` | `structure` | `message`, `xAmzErrorType` | The input fails to satisfy the constraints specified by an AWS service. |
| `ResourceNotFoundException` | `structure` | `message`, `xAmzErrorType` | The specified resource was not found. |
| `ConflictException` | `structure` | `message`, `xAmzErrorType` | The request could not be processed because of conflict in the current state of the resource. |
| `ThrottlingException` | `structure` | `message`, `xAmzErrorType` | The request was denied due to request throttling. |
| `InvalidCampaignStateException` | `structure` | `message`, `state`, `xAmzErrorType` | The request could not be processed because of conflict in the current state of the campaign. |
| `InvalidStateException` | `structure` | `message`, `xAmzErrorType` | The request could not be processed because of conflict in the current state. |
| `CreateCampaignRequest` | `structure` | `channelSubtypeConfig`, `communicationLimitsOverride`, `communicationTimeConfig`, `connectCampaignFlowArn`, `connectInstanceId`, `name`, `schedule`, `source`, `tags`, `type` | The request for CreateCampaign API. |
| `CreateCampaignResponse` | `structure` | `arn`, `id`, `tags` | The response for Create Campaign API |
| `ServiceQuotaExceededException` | `structure` | `message`, `xAmzErrorType` | Request would cause a service quota to be exceeded. |
| `DeleteCampaignRequest` | `structure` | `id` | The request for DeleteCampaign API. |
| `DeleteCampaignChannelSubtypeConfigRequest` | `structure` | `channelSubtype`, `id` | The request for DeleteCampaignChannelSubtypeConfig API. |
| `DeleteCampaignCommunicationLimitsRequest` | `structure` | `config`, `id` | The request for DeleteCampaignCommunicationLimits API. |
| `DeleteCampaignCommunicationTimeRequest` | `structure` | `config`, `id` | The request for DeleteCampaignCommunicationTime API. |
| `DeleteConnectInstanceConfigRequest` | `structure` | `campaignDeletionPolicy`, `connectInstanceId` | The request for DeleteConnectInstanceConfig API. |
| `DeleteConnectInstanceIntegrationRequest` | `structure` | `connectInstanceId`, `integrationIdentifier` | The request for DeleteConnectInstanceIntegration API. |
| `DeleteInstanceOnboardingJobRequest` | `structure` | `connectInstanceId` | The request for DeleteInstanceOnboardingJob API. |
| `DescribeCampaignRequest` | `structure` | `id` | The request for DescribeCampaign API. |
| `DescribeCampaignResponse` | `structure` | `campaign` | The response for DescribeCampaign API. |
| `GetCampaignStateRequest` | `structure` | `id` | The request for GetCampaignState API. |
| `GetCampaignStateResponse` | `structure` | `state` | The response for GetCampaignState API. |
| `GetCampaignStateBatchRequest` | `structure` | `campaignIds` | The request for GetCampaignStateBatch API. |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
