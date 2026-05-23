# Amazon Pinpoint

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Doc Engage API - Amazon Pinpoint API

## Possible Usage Scenarios
- Backported from `crates/winterbaume-pinpoint/tests/scenario_test.rs`: manage an app with an event stream and tags, then verify describe/list behaviour.
- Backported from `scenario_test.rs`: use application settings to drive a campaign lifecycle.
- From the AWS documentation and model: represent customer engagement applications, endpoints, campaigns, segments, journeys, event streams, channels, templates, analytics exports, and tag-based administration.

## Service Identity and Protocol

- AWS model slug: `pinpoint`
- AWS SDK for Rust slug: `pinpoint`
- Model version: `2016-12-01`
- Model file: `vendor/api-models-aws/models/pinpoint/service/2016-12-01/pinpoint-2016-12-01.json`
- SDK ID: `Pinpoint`
- Endpoint prefix: `pinpoint`
- ARN namespace: `mobiletargeting`
- CloudFormation name: `Pinpoint`
- CloudTrail event source: `pinpoint.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (49), `Update` (24), `Delete` (23), `Create` (12), `List` (4), `Send` (3), `Put` (2), `Phone` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateApp`, `CreateCampaign`, `CreateEmailTemplate`, `CreateExportJob`, `CreateImportJob`, `CreateInAppTemplate`, `CreateJourney`, `CreatePushTemplate`, `CreateRecommenderConfiguration`, `CreateSegment`, `CreateSmsTemplate`, `CreateVoiceTemplate`, `DeleteAdmChannel`, `DeleteApnsChannel`, `DeleteApnsSandboxChannel`, `DeleteApnsVoipChannel`, `DeleteApnsVoipSandboxChannel`, `DeleteApp`, `DeleteBaiduChannel`, `DeleteCampaign`, ... (+44).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAdmChannel`, `GetApnsChannel`, `GetApnsSandboxChannel`, `GetApnsVoipChannel`, `GetApnsVoipSandboxChannel`, `GetApp`, `GetApplicationDateRangeKpi`, `GetApplicationSettings`, `GetApps`, `GetBaiduChannel`, `GetCampaign`, `GetCampaignActivities`, `GetCampaignDateRangeKpi`, `GetCampaignVersion`, `GetCampaignVersions`, `GetCampaigns`, `GetChannels`, `GetEmailChannel`, `GetEmailTemplate`, `GetEndpoint`, ... (+33).
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateExportJob`, `CreateImportJob`, `GetExportJob`, `GetExportJobs`, `GetImportJob`, `GetImportJobs`, `GetJourneyExecutionActivityMetrics`, `GetJourneyExecutionMetrics`, `GetJourneyRunExecutionActivityMetrics`, `GetJourneyRunExecutionMetrics`, `GetSegmentExportJobs`, `GetSegmentImportJobs`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 119 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `CloudWatch`, `Lambda`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Get

- Operations: `GetAdmChannel`, `GetApnsChannel`, `GetApnsSandboxChannel`, `GetApnsVoipChannel`, `GetApnsVoipSandboxChannel`, `GetApp`, `GetApplicationDateRangeKpi`, `GetApplicationSettings`, `GetApps`, `GetBaiduChannel`, `GetCampaign`, `GetCampaignActivities`, `GetCampaignDateRangeKpi`, `GetCampaigns`, `GetCampaignVersion`, `GetCampaignVersions`, `GetChannels`, `GetEmailChannel`, `GetEmailTemplate`, `GetEndpoint`, `GetEventStream`, `GetExportJob`, `GetExportJobs`, `GetGcmChannel`, `GetImportJob`, `GetImportJobs`, `GetInAppMessages`, `GetInAppTemplate`, `GetJourney`, `GetJourneyDateRangeKpi`, `GetJourneyExecutionActivityMetrics`, `GetJourneyExecutionMetrics`, `GetJourneyRunExecutionActivityMetrics`, `GetJourneyRunExecutionMetrics`, `GetJourneyRuns`, `GetPushTemplate`, `GetRecommenderConfiguration`, `GetRecommenderConfigurations`, `GetSegment`, `GetSegmentExportJobs`, `GetSegmentImportJobs`, `GetSegments`, `GetSegmentVersion`, `GetSegmentVersions`, `GetSmsChannel`, `GetSmsTemplate`, `GetUserEndpoints`, `GetVoiceChannel`, `GetVoiceTemplate`
- Common required input members in this group: `ApplicationId`, `KpiName`, `CampaignId`, `Version`, `TemplateName`, `EndpointId`, `JobId`, `JourneyId`, `JourneyActivityId`, `RunId`, `SegmentId`

### Update

- Operations: `UpdateAdmChannel`, `UpdateApnsChannel`, `UpdateApnsSandboxChannel`, `UpdateApnsVoipChannel`, `UpdateApnsVoipSandboxChannel`, `UpdateApplicationSettings`, `UpdateBaiduChannel`, `UpdateCampaign`, `UpdateEmailChannel`, `UpdateEmailTemplate`, `UpdateEndpoint`, `UpdateEndpointsBatch`, `UpdateGcmChannel`, `UpdateInAppTemplate`, `UpdateJourney`, `UpdateJourneyState`, `UpdatePushTemplate`, `UpdateRecommenderConfiguration`, `UpdateSegment`, `UpdateSmsChannel`, `UpdateSmsTemplate`, `UpdateTemplateActiveVersion`, `UpdateVoiceChannel`, `UpdateVoiceTemplate`
- Common required input members in this group: `ApplicationId`, `TemplateName`, `JourneyId`

### Delete

- Operations: `DeleteAdmChannel`, `DeleteApnsChannel`, `DeleteApnsSandboxChannel`, `DeleteApnsVoipChannel`, `DeleteApnsVoipSandboxChannel`, `DeleteApp`, `DeleteBaiduChannel`, `DeleteCampaign`, `DeleteEmailChannel`, `DeleteEmailTemplate`, `DeleteEndpoint`, `DeleteEventStream`, `DeleteGcmChannel`, `DeleteInAppTemplate`, `DeleteJourney`, `DeletePushTemplate`, `DeleteRecommenderConfiguration`, `DeleteSegment`, `DeleteSmsChannel`, `DeleteSmsTemplate`, `DeleteUserEndpoints`, `DeleteVoiceChannel`, `DeleteVoiceTemplate`
- Common required input members in this group: `ApplicationId`, `TemplateName`

### Create

- Operations: `CreateApp`, `CreateCampaign`, `CreateEmailTemplate`, `CreateExportJob`, `CreateImportJob`, `CreateInAppTemplate`, `CreateJourney`, `CreatePushTemplate`, `CreateRecommenderConfiguration`, `CreateSegment`, `CreateSmsTemplate`, `CreateVoiceTemplate`
- Common required input members in this group: `ApplicationId`, `TemplateName`

### List

- Operations: `ListJourneys`, `ListTagsForResource`, `ListTemplates`, `ListTemplateVersions`
- Common required input members in this group: -

### Send

- Operations: `SendMessages`, `SendOTPMessage`, `SendUsersMessages`
- Common required input members in this group: `ApplicationId`

### Put

- Operations: `PutEvents`, `PutEventStream`
- Common required input members in this group: `ApplicationId`

### Phone

- Operations: `PhoneNumberValidate`
- Common required input members in this group: -

### Remove

- Operations: `RemoveAttributes`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Verify

- Operations: `VerifyOTPMessage`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateApp` | `POST /v1/apps` | - | `CreateApplicationRequest` | - | `CreateAppResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Creates an application. |
| `CreateCampaign` | `POST /v1/apps/{ApplicationId}/campaigns` | - | `ApplicationId`, `WriteCampaignRequest` | - | `CreateCampaignResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Creates a new campaign for an application or updates the settings of an existing campaign for an application. |
| `CreateEmailTemplate` | `POST /v1/templates/{TemplateName}/email` | - | `EmailTemplateRequest`, `TemplateName` | - | `CreateEmailTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `TooManyRequestsException` | Creates a message template for messages that are sent through the email channel. |
| `CreateExportJob` | `POST /v1/apps/{ApplicationId}/jobs/export` | - | `ApplicationId`, `ExportJobRequest` | - | `CreateExportJobResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Creates an export job for an application. |
| `CreateImportJob` | `POST /v1/apps/{ApplicationId}/jobs/import` | - | `ApplicationId`, `ImportJobRequest` | - | `CreateImportJobResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Creates an import job for an application. |
| `CreateInAppTemplate` | `POST /v1/templates/{TemplateName}/inapp` | - | `InAppTemplateRequest`, `TemplateName` | - | `CreateInAppTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `TooManyRequestsException` | Creates a new message template for messages using the in-app message channel. |
| `CreateJourney` | `POST /v1/apps/{ApplicationId}/journeys` | - | `ApplicationId`, `WriteJourneyRequest` | - | `CreateJourneyResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Creates a journey for an application. |
| `CreatePushTemplate` | `POST /v1/templates/{TemplateName}/push` | - | `PushNotificationTemplateRequest`, `TemplateName` | - | `CreatePushTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `TooManyRequestsException` | Creates a message template for messages that are sent through a push notification channel. |
| `CreateRecommenderConfiguration` | `POST /v1/recommenders` | - | `CreateRecommenderConfiguration` | - | `CreateRecommenderConfigurationResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Creates an Amazon Pinpoint configuration for a recommender model. |
| `CreateSegment` | `POST /v1/apps/{ApplicationId}/segments` | - | `ApplicationId`, `WriteSegmentRequest` | - | `CreateSegmentResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Creates a new segment for an application or updates the configuration, dimension, and other settings for an existing segment that's associated with an application. |
| `CreateSmsTemplate` | `POST /v1/templates/{TemplateName}/sms` | - | `SMSTemplateRequest`, `TemplateName` | - | `CreateSmsTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `TooManyRequestsException` | Creates a message template for messages that are sent through the SMS channel. |
| `CreateVoiceTemplate` | `POST /v1/templates/{TemplateName}/voice` | - | `TemplateName`, `VoiceTemplateRequest` | - | `CreateVoiceTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `TooManyRequestsException` | Creates a message template for messages that are sent through the voice channel. |
| `DeleteAdmChannel` | `DELETE /v1/apps/{ApplicationId}/channels/adm` | - | `ApplicationId` | - | `DeleteAdmChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Disables the ADM channel for an application and deletes any existing settings for the channel. |
| `DeleteApnsChannel` | `DELETE /v1/apps/{ApplicationId}/channels/apns` | - | `ApplicationId` | - | `DeleteApnsChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Disables the APNs channel for an application and deletes any existing settings for the channel. |
| `DeleteApnsSandboxChannel` | `DELETE /v1/apps/{ApplicationId}/channels/apns_sandbox` | - | `ApplicationId` | - | `DeleteApnsSandboxChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Disables the APNs sandbox channel for an application and deletes any existing settings for the channel. |
| `DeleteApnsVoipChannel` | `DELETE /v1/apps/{ApplicationId}/channels/apns_voip` | - | `ApplicationId` | - | `DeleteApnsVoipChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Disables the APNs VoIP channel for an application and deletes any existing settings for the channel. |
| `DeleteApnsVoipSandboxChannel` | `DELETE /v1/apps/{ApplicationId}/channels/apns_voip_sandbox` | - | `ApplicationId` | - | `DeleteApnsVoipSandboxChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Disables the APNs VoIP sandbox channel for an application and deletes any existing settings for the channel. |
| `DeleteApp` | `DELETE /v1/apps/{ApplicationId}` | - | `ApplicationId` | - | `DeleteAppResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Deletes an application. |
| `DeleteBaiduChannel` | `DELETE /v1/apps/{ApplicationId}/channels/baidu` | - | `ApplicationId` | - | `DeleteBaiduChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Disables the Baidu channel for an application and deletes any existing settings for the channel. |
| `DeleteCampaign` | `DELETE /v1/apps/{ApplicationId}/campaigns/{CampaignId}` | - | `ApplicationId`, `CampaignId` | - | `DeleteCampaignResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Deletes a campaign from an application. |
| `DeleteEmailChannel` | `DELETE /v1/apps/{ApplicationId}/channels/email` | - | `ApplicationId` | - | `DeleteEmailChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Disables the email channel for an application and deletes any existing settings for the channel. |
| `DeleteEmailTemplate` | `DELETE /v1/templates/{TemplateName}/email` | - | `TemplateName` | - | `DeleteEmailTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Deletes a message template for messages that were sent through the email channel. |
| `DeleteEndpoint` | `DELETE /v1/apps/{ApplicationId}/endpoints/{EndpointId}` | - | `ApplicationId`, `EndpointId` | - | `DeleteEndpointResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Deletes an endpoint from an application. |
| `DeleteEventStream` | `DELETE /v1/apps/{ApplicationId}/eventstream` | - | `ApplicationId` | - | `DeleteEventStreamResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Deletes the event stream for an application. |
| `DeleteGcmChannel` | `DELETE /v1/apps/{ApplicationId}/channels/gcm` | - | `ApplicationId` | - | `DeleteGcmChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Disables the GCM channel for an application and deletes any existing settings for the channel. |
| `DeleteInAppTemplate` | `DELETE /v1/templates/{TemplateName}/inapp` | - | `TemplateName` | - | `DeleteInAppTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Deletes a message template for messages sent using the in-app message channel. |
| `DeleteJourney` | `DELETE /v1/apps/{ApplicationId}/journeys/{JourneyId}` | - | `ApplicationId`, `JourneyId` | - | `DeleteJourneyResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Deletes a journey from an application. |
| `DeletePushTemplate` | `DELETE /v1/templates/{TemplateName}/push` | - | `TemplateName` | - | `DeletePushTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Deletes a message template for messages that were sent through a push notification channel. |
| `DeleteRecommenderConfiguration` | `DELETE /v1/recommenders/{RecommenderId}` | - | `RecommenderId` | - | `DeleteRecommenderConfigurationResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Deletes an Amazon Pinpoint configuration for a recommender model. |
| `DeleteSegment` | `DELETE /v1/apps/{ApplicationId}/segments/{SegmentId}` | - | `ApplicationId`, `SegmentId` | - | `DeleteSegmentResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Deletes a segment from an application. |
| `DeleteSmsChannel` | `DELETE /v1/apps/{ApplicationId}/channels/sms` | - | `ApplicationId` | - | `DeleteSmsChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Disables the SMS channel for an application and deletes any existing settings for the channel. |
| `DeleteSmsTemplate` | `DELETE /v1/templates/{TemplateName}/sms` | - | `TemplateName` | - | `DeleteSmsTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Deletes a message template for messages that were sent through the SMS channel. |
| `DeleteUserEndpoints` | `DELETE /v1/apps/{ApplicationId}/users/{UserId}` | - | `ApplicationId`, `UserId` | - | `DeleteUserEndpointsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Deletes all the endpoints that are associated with a specific user ID. |
| `DeleteVoiceChannel` | `DELETE /v1/apps/{ApplicationId}/channels/voice` | - | `ApplicationId` | - | `DeleteVoiceChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Disables the voice channel for an application and deletes any existing settings for the channel. |
| `DeleteVoiceTemplate` | `DELETE /v1/templates/{TemplateName}/voice` | - | `TemplateName` | - | `DeleteVoiceTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Deletes a message template for messages that were sent through the voice channel. |
| `GetAdmChannel` | `GET /v1/apps/{ApplicationId}/channels/adm` | - | `ApplicationId` | - | `GetAdmChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status and settings of the ADM channel for an application. |
| `GetApnsChannel` | `GET /v1/apps/{ApplicationId}/channels/apns` | - | `ApplicationId` | - | `GetApnsChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status and settings of the APNs channel for an application. |
| `GetApnsSandboxChannel` | `GET /v1/apps/{ApplicationId}/channels/apns_sandbox` | - | `ApplicationId` | - | `GetApnsSandboxChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status and settings of the APNs sandbox channel for an application. |
| `GetApnsVoipChannel` | `GET /v1/apps/{ApplicationId}/channels/apns_voip` | - | `ApplicationId` | - | `GetApnsVoipChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status and settings of the APNs VoIP channel for an application. |
| `GetApnsVoipSandboxChannel` | `GET /v1/apps/{ApplicationId}/channels/apns_voip_sandbox` | - | `ApplicationId` | - | `GetApnsVoipSandboxChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status and settings of the APNs VoIP sandbox channel for an application. |
| `GetApp` | `GET /v1/apps/{ApplicationId}` | - | `ApplicationId` | - | `GetAppResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about an application. |
| `GetApplicationDateRangeKpi` | `GET /v1/apps/{ApplicationId}/kpis/daterange/{KpiName}` | - | `ApplicationId`, `KpiName` | - | `GetApplicationDateRangeKpiResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves (queries) pre-aggregated data for a standard metric that applies to an application. |
| `GetApplicationSettings` | `GET /v1/apps/{ApplicationId}/settings` | - | `ApplicationId` | - | `GetApplicationSettingsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the settings for an application. |
| `GetApps` | `GET /v1/apps` | - | - | - | `GetAppsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about all the applications that are associated with your Amazon Pinpoint account. |
| `GetBaiduChannel` | `GET /v1/apps/{ApplicationId}/channels/baidu` | - | `ApplicationId` | - | `GetBaiduChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status and settings of the Baidu channel for an application. |
| `GetCampaign` | `GET /v1/apps/{ApplicationId}/campaigns/{CampaignId}` | - | `ApplicationId`, `CampaignId` | - | `GetCampaignResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status, configuration, and other settings for a campaign. |
| `GetCampaignActivities` | `GET /v1/apps/{ApplicationId}/campaigns/{CampaignId}/activities` | - | `ApplicationId`, `CampaignId` | - | `GetCampaignActivitiesResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about all the activities for a campaign. |
| `GetCampaignDateRangeKpi` | `GET /v1/apps/{ApplicationId}/campaigns/{CampaignId}/kpis/daterange/{KpiName}` | - | `ApplicationId`, `CampaignId`, `KpiName` | - | `GetCampaignDateRangeKpiResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves (queries) pre-aggregated data for a standard metric that applies to a campaign. |
| `GetCampaigns` | `GET /v1/apps/{ApplicationId}/campaigns` | - | `ApplicationId` | - | `GetCampaignsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status, configuration, and other settings for all the campaigns that are associated with an application. |
| `GetCampaignVersion` | `GET /v1/apps/{ApplicationId}/campaigns/{CampaignId}/versions/{Version}` | - | `ApplicationId`, `CampaignId`, `Version` | - | `GetCampaignVersionResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status, configuration, and other settings for a specific version of a campaign. |
| `GetCampaignVersions` | `GET /v1/apps/{ApplicationId}/campaigns/{CampaignId}/versions` | - | `ApplicationId`, `CampaignId` | - | `GetCampaignVersionsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status, configuration, and other settings for all versions of a campaign. |
| `GetChannels` | `GET /v1/apps/{ApplicationId}/channels` | - | `ApplicationId` | - | `GetChannelsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the history and status of each channel for an application. |
| `GetEmailChannel` | `GET /v1/apps/{ApplicationId}/channels/email` | - | `ApplicationId` | - | `GetEmailChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status and settings of the email channel for an application. |
| `GetEmailTemplate` | `GET /v1/templates/{TemplateName}/email` | - | `TemplateName` | - | `GetEmailTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves the content and settings of a message template for messages that are sent through the email channel. |
| `GetEndpoint` | `GET /v1/apps/{ApplicationId}/endpoints/{EndpointId}` | - | `ApplicationId`, `EndpointId` | - | `GetEndpointResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the settings and attributes of a specific endpoint for an application. |
| `GetEventStream` | `GET /v1/apps/{ApplicationId}/eventstream` | - | `ApplicationId` | - | `GetEventStreamResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the event stream settings for an application. |
| `GetExportJob` | `GET /v1/apps/{ApplicationId}/jobs/export/{JobId}` | - | `ApplicationId`, `JobId` | - | `GetExportJobResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status and settings of a specific export job for an application. |
| `GetExportJobs` | `GET /v1/apps/{ApplicationId}/jobs/export` | - | `ApplicationId` | - | `GetExportJobsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status and settings of all the export jobs for an application. |
| `GetGcmChannel` | `GET /v1/apps/{ApplicationId}/channels/gcm` | - | `ApplicationId` | - | `GetGcmChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status and settings of the GCM channel for an application. |
| `GetImportJob` | `GET /v1/apps/{ApplicationId}/jobs/import/{JobId}` | - | `ApplicationId`, `JobId` | - | `GetImportJobResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status and settings of a specific import job for an application. |
| `GetImportJobs` | `GET /v1/apps/{ApplicationId}/jobs/import` | - | `ApplicationId` | - | `GetImportJobsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status and settings of all the import jobs for an application. |
| `GetInAppMessages` | `GET /v1/apps/{ApplicationId}/endpoints/{EndpointId}/inappmessages` | - | `ApplicationId`, `EndpointId` | - | `GetInAppMessagesResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves the in-app messages targeted for the provided endpoint ID. |
| `GetInAppTemplate` | `GET /v1/templates/{TemplateName}/inapp` | - | `TemplateName` | - | `GetInAppTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves the content and settings of a message template for messages sent through the in-app channel. |
| `GetJourney` | `GET /v1/apps/{ApplicationId}/journeys/{JourneyId}` | - | `ApplicationId`, `JourneyId` | - | `GetJourneyResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status, configuration, and other settings for a journey. |
| `GetJourneyDateRangeKpi` | `GET /v1/apps/{ApplicationId}/journeys/{JourneyId}/kpis/daterange/{KpiName}` | - | `ApplicationId`, `JourneyId`, `KpiName` | - | `GetJourneyDateRangeKpiResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves (queries) pre-aggregated data for a standard engagement metric that applies to a journey. |
| `GetJourneyExecutionActivityMetrics` | `GET /v1/apps/{ApplicationId}/journeys/{JourneyId}/activities/{JourneyActivityId}/execution-metrics` | - | `ApplicationId`, `JourneyActivityId`, `JourneyId` | - | `GetJourneyExecutionActivityMetricsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves (queries) pre-aggregated data for a standard execution metric that applies to a journey activity. |
| `GetJourneyExecutionMetrics` | `GET /v1/apps/{ApplicationId}/journeys/{JourneyId}/execution-metrics` | - | `ApplicationId`, `JourneyId` | - | `GetJourneyExecutionMetricsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves (queries) pre-aggregated data for a standard execution metric that applies to a journey. |
| `GetJourneyRunExecutionActivityMetrics` | `GET /v1/apps/{ApplicationId}/journeys/{JourneyId}/runs/{RunId}/activities/{JourneyActivityId}/execution-metrics` | - | `ApplicationId`, `JourneyActivityId`, `JourneyId`, `RunId` | - | `GetJourneyRunExecutionActivityMetricsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves (queries) pre-aggregated data for a standard run execution metric that applies to a journey activity. |
| `GetJourneyRunExecutionMetrics` | `GET /v1/apps/{ApplicationId}/journeys/{JourneyId}/runs/{RunId}/execution-metrics` | - | `ApplicationId`, `JourneyId`, `RunId` | - | `GetJourneyRunExecutionMetricsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves (queries) pre-aggregated data for a standard run execution metric that applies to a journey. |
| `GetJourneyRuns` | `GET /v1/apps/{ApplicationId}/journeys/{JourneyId}/runs` | - | `ApplicationId`, `JourneyId` | - | `GetJourneyRunsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Provides information about the runs of a journey. |
| `GetPushTemplate` | `GET /v1/templates/{TemplateName}/push` | - | `TemplateName` | - | `GetPushTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves the content and settings of a message template for messages that are sent through a push notification channel. |
| `GetRecommenderConfiguration` | `GET /v1/recommenders/{RecommenderId}` | - | `RecommenderId` | - | `GetRecommenderConfigurationResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about an Amazon Pinpoint configuration for a recommender model. |
| `GetRecommenderConfigurations` | `GET /v1/recommenders` | - | - | - | `GetRecommenderConfigurationsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about all the recommender model configurations that are associated with your Amazon Pinpoint account. |
| `GetSegment` | `GET /v1/apps/{ApplicationId}/segments/{SegmentId}` | - | `ApplicationId`, `SegmentId` | - | `GetSegmentResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the configuration, dimension, and other settings for a specific segment that's associated with an application. |
| `GetSegmentExportJobs` | `GET /v1/apps/{ApplicationId}/segments/{SegmentId}/jobs/export` | - | `ApplicationId`, `SegmentId` | - | `GetSegmentExportJobsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status and settings of the export jobs for a segment. |
| `GetSegmentImportJobs` | `GET /v1/apps/{ApplicationId}/segments/{SegmentId}/jobs/import` | - | `ApplicationId`, `SegmentId` | - | `GetSegmentImportJobsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status and settings of the import jobs for a segment. |
| `GetSegments` | `GET /v1/apps/{ApplicationId}/segments` | - | `ApplicationId` | - | `GetSegmentsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the configuration, dimension, and other settings for all the segments that are associated with an application. |
| `GetSegmentVersion` | `GET /v1/apps/{ApplicationId}/segments/{SegmentId}/versions/{Version}` | - | `ApplicationId`, `SegmentId`, `Version` | - | `GetSegmentVersionResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the configuration, dimension, and other settings for a specific version of a segment that's associated with an application. |
| `GetSegmentVersions` | `GET /v1/apps/{ApplicationId}/segments/{SegmentId}/versions` | - | `ApplicationId`, `SegmentId` | - | `GetSegmentVersionsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the configuration, dimension, and other settings for all the versions of a specific segment that's associated with an application. |
| `GetSmsChannel` | `GET /v1/apps/{ApplicationId}/channels/sms` | - | `ApplicationId` | - | `GetSmsChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status and settings of the SMS channel for an application. |
| `GetSmsTemplate` | `GET /v1/templates/{TemplateName}/sms` | - | `TemplateName` | - | `GetSmsTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves the content and settings of a message template for messages that are sent through the SMS channel. |
| `GetUserEndpoints` | `GET /v1/apps/{ApplicationId}/users/{UserId}` | - | `ApplicationId`, `UserId` | - | `GetUserEndpointsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about all the endpoints that are associated with a specific user ID. |
| `GetVoiceChannel` | `GET /v1/apps/{ApplicationId}/channels/voice` | - | `ApplicationId` | - | `GetVoiceChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status and settings of the voice channel for an application. |
| `GetVoiceTemplate` | `GET /v1/templates/{TemplateName}/voice` | - | `TemplateName` | - | `GetVoiceTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves the content and settings of a message template for messages that are sent through the voice channel. |
| `ListJourneys` | `GET /v1/apps/{ApplicationId}/journeys` | - | `ApplicationId` | - | `ListJourneysResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about the status, configuration, and other settings for all the journeys that are associated with an application. |
| `ListTagsForResource` | `GET /v1/tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | - | Retrieves all the tags (keys and values) that are associated with an application, campaign, message template, or segment. |
| `ListTemplates` | `GET /v1/templates` | - | - | - | `ListTemplatesResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `TooManyRequestsException` | Retrieves information about all the message templates that are associated with your Amazon Pinpoint account. |
| `ListTemplateVersions` | `GET /v1/templates/{TemplateName}/{TemplateType}/versions` | - | `TemplateName`, `TemplateType` | - | `ListTemplateVersionsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about all the versions of a specific message template. |
| `PhoneNumberValidate` | `POST /v1/phone/number/validate` | - | `NumberValidateRequest` | - | `PhoneNumberValidateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Retrieves information about a phone number. |
| `PutEvents` | `POST /v1/apps/{ApplicationId}/events` | - | `ApplicationId`, `EventsRequest` | - | `PutEventsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Creates a new event to record for endpoints, or creates or updates endpoint data that existing events are associated with. |
| `PutEventStream` | `POST /v1/apps/{ApplicationId}/eventstream` | - | `ApplicationId`, `WriteEventStream` | - | `PutEventStreamResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Creates a new event stream for an application or updates the settings of an existing event stream for an application. |
| `RemoveAttributes` | `PUT /v1/apps/{ApplicationId}/attributes/{AttributeType}` | - | `ApplicationId`, `AttributeType`, `UpdateAttributesRequest` | - | `RemoveAttributesResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Removes one or more custom attributes, of the same attribute type, from the application. Existing endpoints still have the attributes but Amazon Pinpoint will stop capturing new or changed values for these attributes. |
| `SendMessages` | `POST /v1/apps/{ApplicationId}/messages` | - | `ApplicationId`, `MessageRequest` | - | `SendMessagesResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Creates and sends a direct message. |
| `SendOTPMessage` | `POST /v1/apps/{ApplicationId}/otp` | - | `ApplicationId`, `SendOTPMessageRequestParameters` | - | `SendOTPMessageResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Send an OTP message |
| `SendUsersMessages` | `POST /v1/apps/{ApplicationId}/users-messages` | - | `ApplicationId`, `SendUsersMessageRequest` | - | `SendUsersMessagesResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Creates and sends a message to a list of users. |
| `TagResource` | `POST /v1/tags/{ResourceArn}` | - | `ResourceArn`, `TagsModel` | - | `Unit` | - | Adds one or more tags (keys and values) to an application, campaign, message template, or segment. |
| `UntagResource` | `DELETE /v1/tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `Unit` | - | Removes one or more tags (keys and values) from an application, campaign, message template, or segment. |
| `UpdateAdmChannel` | `PUT /v1/apps/{ApplicationId}/channels/adm` | - | `ADMChannelRequest`, `ApplicationId` | - | `UpdateAdmChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Enables the ADM channel for an application or updates the status and settings of the ADM channel for an application. |
| `UpdateApnsChannel` | `PUT /v1/apps/{ApplicationId}/channels/apns` | - | `APNSChannelRequest`, `ApplicationId` | - | `UpdateApnsChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Enables the APNs channel for an application or updates the status and settings of the APNs channel for an application. |
| `UpdateApnsSandboxChannel` | `PUT /v1/apps/{ApplicationId}/channels/apns_sandbox` | - | `APNSSandboxChannelRequest`, `ApplicationId` | - | `UpdateApnsSandboxChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Enables the APNs sandbox channel for an application or updates the status and settings of the APNs sandbox channel for an application. |
| `UpdateApnsVoipChannel` | `PUT /v1/apps/{ApplicationId}/channels/apns_voip` | - | `APNSVoipChannelRequest`, `ApplicationId` | - | `UpdateApnsVoipChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Enables the APNs VoIP channel for an application or updates the status and settings of the APNs VoIP channel for an application. |
| `UpdateApnsVoipSandboxChannel` | `PUT /v1/apps/{ApplicationId}/channels/apns_voip_sandbox` | - | `APNSVoipSandboxChannelRequest`, `ApplicationId` | - | `UpdateApnsVoipSandboxChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Enables the APNs VoIP sandbox channel for an application or updates the status and settings of the APNs VoIP sandbox channel for an application. |
| `UpdateApplicationSettings` | `PUT /v1/apps/{ApplicationId}/settings` | - | `ApplicationId`, `WriteApplicationSettingsRequest` | - | `UpdateApplicationSettingsResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Updates the settings for an application. |
| `UpdateBaiduChannel` | `PUT /v1/apps/{ApplicationId}/channels/baidu` | - | `ApplicationId`, `BaiduChannelRequest` | - | `UpdateBaiduChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Enables the Baidu channel for an application or updates the status and settings of the Baidu channel for an application. |
| `UpdateCampaign` | `PUT /v1/apps/{ApplicationId}/campaigns/{CampaignId}` | - | `ApplicationId`, `CampaignId`, `WriteCampaignRequest` | - | `UpdateCampaignResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Updates the configuration and other settings for a campaign. |
| `UpdateEmailChannel` | `PUT /v1/apps/{ApplicationId}/channels/email` | - | `ApplicationId`, `EmailChannelRequest` | - | `UpdateEmailChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Enables the email channel for an application or updates the status and settings of the email channel for an application. |
| `UpdateEmailTemplate` | `PUT /v1/templates/{TemplateName}/email` | - | `EmailTemplateRequest`, `TemplateName` | - | `UpdateEmailTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Updates an existing message template for messages that are sent through the email channel. |
| `UpdateEndpoint` | `PUT /v1/apps/{ApplicationId}/endpoints/{EndpointId}` | - | `ApplicationId`, `EndpointId`, `EndpointRequest` | - | `UpdateEndpointResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Creates a new endpoint for an application or updates the settings and attributes of an existing endpoint for an application. You can also use this operation to define custom attributes for an endpoint. If an update i ... |
| `UpdateEndpointsBatch` | `PUT /v1/apps/{ApplicationId}/endpoints` | - | `ApplicationId`, `EndpointBatchRequest` | - | `UpdateEndpointsBatchResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Creates a new batch of endpoints for an application or updates the settings and attributes of a batch of existing endpoints for an application. You can also use this operation to define custom attributes for a batch ... |
| `UpdateGcmChannel` | `PUT /v1/apps/{ApplicationId}/channels/gcm` | - | `ApplicationId`, `GCMChannelRequest` | - | `UpdateGcmChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Enables the GCM channel for an application or updates the status and settings of the GCM channel for an application. |
| `UpdateInAppTemplate` | `PUT /v1/templates/{TemplateName}/inapp` | - | `InAppTemplateRequest`, `TemplateName` | - | `UpdateInAppTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Updates an existing message template for messages sent through the in-app message channel. |
| `UpdateJourney` | `PUT /v1/apps/{ApplicationId}/journeys/{JourneyId}` | - | `ApplicationId`, `JourneyId`, `WriteJourneyRequest` | - | `UpdateJourneyResponse` | `BadRequestException`, `ConflictException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Updates the configuration and other settings for a journey. |
| `UpdateJourneyState` | `PUT /v1/apps/{ApplicationId}/journeys/{JourneyId}/state` | - | `ApplicationId`, `JourneyId`, `JourneyStateRequest` | - | `UpdateJourneyStateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Cancels (stops) an active journey. |
| `UpdatePushTemplate` | `PUT /v1/templates/{TemplateName}/push` | - | `PushNotificationTemplateRequest`, `TemplateName` | - | `UpdatePushTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Updates an existing message template for messages that are sent through a push notification channel. |
| `UpdateRecommenderConfiguration` | `PUT /v1/recommenders/{RecommenderId}` | - | `RecommenderId`, `UpdateRecommenderConfiguration` | - | `UpdateRecommenderConfigurationResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Updates an Amazon Pinpoint configuration for a recommender model. |
| `UpdateSegment` | `PUT /v1/apps/{ApplicationId}/segments/{SegmentId}` | - | `ApplicationId`, `SegmentId`, `WriteSegmentRequest` | - | `UpdateSegmentResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Creates a new segment for an application or updates the configuration, dimension, and other settings for an existing segment that's associated with an application. |
| `UpdateSmsChannel` | `PUT /v1/apps/{ApplicationId}/channels/sms` | - | `ApplicationId`, `SMSChannelRequest` | - | `UpdateSmsChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Enables the SMS channel for an application or updates the status and settings of the SMS channel for an application. |
| `UpdateSmsTemplate` | `PUT /v1/templates/{TemplateName}/sms` | - | `SMSTemplateRequest`, `TemplateName` | - | `UpdateSmsTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Updates an existing message template for messages that are sent through the SMS channel. |
| `UpdateTemplateActiveVersion` | `PUT /v1/templates/{TemplateName}/{TemplateType}/active-version` | - | `TemplateActiveVersionRequest`, `TemplateName`, `TemplateType` | - | `UpdateTemplateActiveVersionResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Changes the status of a specific version of a message template to active . |
| `UpdateVoiceChannel` | `PUT /v1/apps/{ApplicationId}/channels/voice` | - | `ApplicationId`, `VoiceChannelRequest` | - | `UpdateVoiceChannelResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Enables the voice channel for an application or updates the status and settings of the voice channel for an application. |
| `UpdateVoiceTemplate` | `PUT /v1/templates/{TemplateName}/voice` | - | `TemplateName`, `VoiceTemplateRequest` | - | `UpdateVoiceTemplateResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Updates an existing message template for messages that are sent through the voice channel. |
| `VerifyOTPMessage` | `POST /v1/apps/{ApplicationId}/verify-otp` | - | `ApplicationId`, `VerifyOTPMessageRequestParameters` | - | `VerifyOTPMessageResponse` | `BadRequestException`, `ForbiddenException`, `InternalServerErrorException`, `MethodNotAllowedException`, `NotFoundException`, `PayloadTooLargeException`, `TooManyRequestsException` | Verify an OTP |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `CreateApp` | - | - | - | `CreateApplicationRequest` |
| `CreateCampaign` | - | - | - | `WriteCampaignRequest` |
| `CreateEmailTemplate` | - | - | - | `EmailTemplateRequest` |
| `CreateExportJob` | - | - | - | `ExportJobRequest` |
| `CreateImportJob` | - | - | - | `ImportJobRequest` |
| `CreateInAppTemplate` | - | - | - | `InAppTemplateRequest` |
| `CreateJourney` | - | - | - | `WriteJourneyRequest` |
| `CreatePushTemplate` | - | - | - | `PushNotificationTemplateRequest` |
| `CreateRecommenderConfiguration` | - | - | - | `CreateRecommenderConfiguration` |
| `CreateSegment` | - | - | - | `WriteSegmentRequest` |
| `CreateSmsTemplate` | - | - | - | `SMSTemplateRequest` |
| `CreateVoiceTemplate` | - | - | - | `VoiceTemplateRequest` |
| `DeleteEmailTemplate` | - | `Version -> version` | - | - |
| `DeleteInAppTemplate` | - | `Version -> version` | - | - |
| `DeletePushTemplate` | - | `Version -> version` | - | - |
| `DeleteSmsTemplate` | - | `Version -> version` | - | - |
| `DeleteVoiceTemplate` | - | `Version -> version` | - | - |
| `GetApplicationDateRangeKpi` | - | `EndTime -> end-time`, `NextToken -> next-token`, `PageSize -> page-size`, `StartTime -> start-time` | - | - |
| `GetApps` | - | `PageSize -> page-size`, `Token -> token` | - | - |
| `GetCampaignActivities` | - | `PageSize -> page-size`, `Token -> token` | - | - |
| `GetCampaignDateRangeKpi` | - | `EndTime -> end-time`, `NextToken -> next-token`, `PageSize -> page-size`, `StartTime -> start-time` | - | - |
| `GetCampaigns` | - | `PageSize -> page-size`, `Token -> token` | - | - |
| `GetCampaignVersions` | - | `PageSize -> page-size`, `Token -> token` | - | - |
| `GetEmailTemplate` | - | `Version -> version` | - | - |
| `GetExportJobs` | - | `PageSize -> page-size`, `Token -> token` | - | - |
| `GetImportJobs` | - | `PageSize -> page-size`, `Token -> token` | - | - |
| `GetInAppTemplate` | - | `Version -> version` | - | - |
| `GetJourneyDateRangeKpi` | - | `EndTime -> end-time`, `NextToken -> next-token`, `PageSize -> page-size`, `StartTime -> start-time` | - | - |
| `GetJourneyExecutionActivityMetrics` | - | `NextToken -> next-token`, `PageSize -> page-size` | - | - |
| `GetJourneyExecutionMetrics` | - | `NextToken -> next-token`, `PageSize -> page-size` | - | - |
| `GetJourneyRunExecutionActivityMetrics` | - | `NextToken -> next-token`, `PageSize -> page-size` | - | - |
| `GetJourneyRunExecutionMetrics` | - | `NextToken -> next-token`, `PageSize -> page-size` | - | - |
| `GetJourneyRuns` | - | `PageSize -> page-size`, `Token -> token` | - | - |
| `GetPushTemplate` | - | `Version -> version` | - | - |
| `GetRecommenderConfigurations` | - | `PageSize -> page-size`, `Token -> token` | - | - |
| `GetSegmentExportJobs` | - | `PageSize -> page-size`, `Token -> token` | - | - |
| `GetSegmentImportJobs` | - | `PageSize -> page-size`, `Token -> token` | - | - |
| `GetSegments` | - | `PageSize -> page-size`, `Token -> token` | - | - |
| `GetSegmentVersions` | - | `PageSize -> page-size`, `Token -> token` | - | - |
| `GetSmsTemplate` | - | `Version -> version` | - | - |
| `GetVoiceTemplate` | - | `Version -> version` | - | - |
| `ListJourneys` | - | `PageSize -> page-size`, `Token -> token` | - | - |
| `ListTemplates` | - | `NextToken -> next-token`, `PageSize -> page-size`, `Prefix -> prefix`, `TemplateType -> template-type` | - | - |
| `ListTemplateVersions` | - | `NextToken -> next-token`, `PageSize -> page-size` | - | - |
| `PhoneNumberValidate` | - | - | - | `NumberValidateRequest` |
| `PutEvents` | - | - | - | `EventsRequest` |
| `PutEventStream` | - | - | - | `WriteEventStream` |
| `RemoveAttributes` | - | - | - | `UpdateAttributesRequest` |
| `SendMessages` | - | - | - | `MessageRequest` |
| `SendOTPMessage` | - | - | - | `SendOTPMessageRequestParameters` |
| `SendUsersMessages` | - | - | - | `SendUsersMessageRequest` |
| `TagResource` | - | - | - | `TagsModel` |
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |
| `UpdateAdmChannel` | - | - | - | `ADMChannelRequest` |
| `UpdateApnsChannel` | - | - | - | `APNSChannelRequest` |
| `UpdateApnsSandboxChannel` | - | - | - | `APNSSandboxChannelRequest` |
| `UpdateApnsVoipChannel` | - | - | - | `APNSVoipChannelRequest` |
| `UpdateApnsVoipSandboxChannel` | - | - | - | `APNSVoipSandboxChannelRequest` |
| `UpdateApplicationSettings` | - | - | - | `WriteApplicationSettingsRequest` |
| `UpdateBaiduChannel` | - | - | - | `BaiduChannelRequest` |
| `UpdateCampaign` | - | - | - | `WriteCampaignRequest` |
| `UpdateEmailChannel` | - | - | - | `EmailChannelRequest` |
| `UpdateEmailTemplate` | - | `CreateNewVersion -> create-new-version`, `Version -> version` | - | `EmailTemplateRequest` |
| `UpdateEndpoint` | - | - | - | `EndpointRequest` |
| `UpdateEndpointsBatch` | - | - | - | `EndpointBatchRequest` |
| `UpdateGcmChannel` | - | - | - | `GCMChannelRequest` |
| `UpdateInAppTemplate` | - | `CreateNewVersion -> create-new-version`, `Version -> version` | - | `InAppTemplateRequest` |
| `UpdateJourney` | - | - | - | `WriteJourneyRequest` |
| `UpdateJourneyState` | - | - | - | `JourneyStateRequest` |
| `UpdatePushTemplate` | - | `CreateNewVersion -> create-new-version`, `Version -> version` | - | `PushNotificationTemplateRequest` |
| `UpdateRecommenderConfiguration` | - | - | - | `UpdateRecommenderConfiguration` |
| `UpdateSegment` | - | - | - | `WriteSegmentRequest` |
| `UpdateSmsChannel` | - | - | - | `SMSChannelRequest` |
| `UpdateSmsTemplate` | - | `CreateNewVersion -> create-new-version`, `Version -> version` | - | `SMSTemplateRequest` |
| `UpdateTemplateActiveVersion` | - | - | - | `TemplateActiveVersionRequest` |
| `UpdateVoiceChannel` | - | - | - | `VoiceChannelRequest` |
| `UpdateVoiceTemplate` | - | `CreateNewVersion -> create-new-version`, `Version -> version` | - | `VoiceTemplateRequest` |
| `VerifyOTPMessage` | - | - | - | `VerifyOTPMessageRequestParameters` |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | Message, RequestID | Provides information about an API request or response. |
| `ConflictException` | `structure` | Message, RequestID | Provides information about an API request or response. |
| `ForbiddenException` | `structure` | Message, RequestID | Provides information about an API request or response. |
| `InternalServerErrorException` | `structure` | Message, RequestID | Provides information about an API request or response. |
| `MethodNotAllowedException` | `structure` | Message, RequestID | Provides information about an API request or response. |
| `NotFoundException` | `structure` | Message, RequestID | Provides information about an API request or response. |
| `PayloadTooLargeException` | `structure` | Message, RequestID | Provides information about an API request or response. |
| `TooManyRequestsException` | `structure` | Message, RequestID | Provides information about an API request or response. |
| `CreateAppRequest` | `structure` | CreateApplicationRequest | - |
| `CreateAppResponse` | `structure` | ApplicationResponse | - |
| `CreateCampaignRequest` | `structure` | ApplicationId, WriteCampaignRequest | - |
| `CreateCampaignResponse` | `structure` | CampaignResponse | - |
| `CreateEmailTemplateRequest` | `structure` | EmailTemplateRequest, TemplateName | - |
| `CreateEmailTemplateResponse` | `structure` | CreateTemplateMessageBody | - |
| `CreateExportJobRequest` | `structure` | ApplicationId, ExportJobRequest | - |
| `CreateExportJobResponse` | `structure` | ExportJobResponse | - |
| `CreateImportJobRequest` | `structure` | ApplicationId, ImportJobRequest | - |
| `CreateImportJobResponse` | `structure` | ImportJobResponse | - |
| `CreateInAppTemplateRequest` | `structure` | InAppTemplateRequest, TemplateName | - |
| `CreateInAppTemplateResponse` | `structure` | TemplateCreateMessageBody | - |
| `CreateJourneyRequest` | `structure` | ApplicationId, WriteJourneyRequest | - |
| `CreateJourneyResponse` | `structure` | JourneyResponse | - |
| `CreatePushTemplateRequest` | `structure` | PushNotificationTemplateRequest, TemplateName | - |
| `CreatePushTemplateResponse` | `structure` | CreateTemplateMessageBody | - |
| `CreateRecommenderConfigurationRequest` | `structure` | CreateRecommenderConfiguration | - |
| `CreateRecommenderConfigurationResponse` | `structure` | RecommenderConfigurationResponse | - |
| `CreateSegmentRequest` | `structure` | ApplicationId, WriteSegmentRequest | - |
| `CreateSegmentResponse` | `structure` | SegmentResponse | - |
| `CreateSmsTemplateRequest` | `structure` | SMSTemplateRequest, TemplateName | - |
| `CreateSmsTemplateResponse` | `structure` | CreateTemplateMessageBody | - |
| `CreateVoiceTemplateRequest` | `structure` | TemplateName, VoiceTemplateRequest | - |
| `CreateVoiceTemplateResponse` | `structure` | CreateTemplateMessageBody | - |
| `DeleteAdmChannelRequest` | `structure` | ApplicationId | - |
| `DeleteAdmChannelResponse` | `structure` | ADMChannelResponse | - |
| `DeleteApnsChannelRequest` | `structure` | ApplicationId | - |
| `DeleteApnsChannelResponse` | `structure` | APNSChannelResponse | - |
| `DeleteApnsSandboxChannelRequest` | `structure` | ApplicationId | - |
| `DeleteApnsSandboxChannelResponse` | `structure` | APNSSandboxChannelResponse | - |
| `DeleteApnsVoipChannelRequest` | `structure` | ApplicationId | - |
| `DeleteApnsVoipChannelResponse` | `structure` | APNSVoipChannelResponse | - |
| `Action` | `enum` | OPEN_APP, DEEP_LINK, URL | - |
| `Alignment` | `enum` | LEFT, CENTER, RIGHT | - |
| `AttributeType` | `enum` | INCLUSIVE, EXCLUSIVE, CONTAINS, BEFORE, AFTER, ON, BETWEEN | - |
| `ButtonAction` | `enum` | LINK, DEEP_LINK, CLOSE | - |
| `CampaignStatus` | `enum` | SCHEDULED, EXECUTING, PENDING_NEXT_RUN, COMPLETED, PAUSED, DELETED, INVALID | - |
| `ChannelType` | `enum` | PUSH, GCM, APNS, APNS_SANDBOX, APNS_VOIP, APNS_VOIP_SANDBOX, ADM, SMS, VOICE, EMAIL, BAIDU, CUSTOM, ... (+1) | - |
| `DayOfWeek` | `enum` | MONDAY, TUESDAY, WEDNESDAY, THURSDAY, FRIDAY, SATURDAY, SUNDAY | - |
| `DeliveryStatus` | `enum` | SUCCESSFUL, THROTTLED, TEMPORARY_FAILURE, PERMANENT_FAILURE, UNKNOWN_FAILURE, OPT_OUT, DUPLICATE | - |
| `DimensionType` | `enum` | INCLUSIVE, EXCLUSIVE | - |
| `Duration` | `enum` | HR_24, DAY_7, DAY_14, DAY_30 | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
