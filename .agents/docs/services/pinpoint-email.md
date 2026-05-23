# Amazon Pinpoint Email Service

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Pinpoint Email Service Welcome to the Amazon Pinpoint Email API Reference . This guide provides information about the Amazon Pinpoint Email API (version 1.0), including supported operations, data types, parameters, and schemas. Amazon Pinpoint is an AWS service that you can use to engage with your customers across multiple messaging channels. You can use Amazon Pinpoint to send email, SMS text messages, voice messages, and push notifications. The Amazon Pinpoint Email API provides programmatic access to options that are unique to the email channel and supplement the options provided by the Amazon Pinpoint API.

## Possible Usage Scenarios
- Scenario insight from EC2: exercise account or service defaults for Amazon Pinpoint Email Service by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- From the AWS documentation and model: represent documented Amazon Pinpoint Email Service workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Put`, `Get`, `List`, `Create`, `Delete` operation families, including `PutAccountDedicatedIpWarmupAttributes`, `PutAccountSendingAttributes`, `PutConfigurationSetDeliveryOptions`, `PutConfigurationSetReputationOptions`, `GetAccount`, `GetBlacklistReports`.

## Service Identity and Protocol

- AWS model slug: `pinpoint-email`
- AWS SDK for Rust slug: `ses`
- Model version: `2018-07-26`
- Model file: `vendor/api-models-aws/models/pinpoint-email/service/2018-07-26/pinpoint-email-2018-07-26.json`
- SDK ID: `Pinpoint Email`
- Endpoint prefix: `email`
- ARN namespace: `ses`
- CloudFormation name: `PinpointEmail`
- CloudTrail event source: `pinpointemail.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Put` (12), `Get` (11), `List` (6), `Create` (5), `Delete` (4), `Send` (1), `Tag` (1), `Untag` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `CreateConfigurationSet`, `CreateConfigurationSetEventDestination`, `CreateDedicatedIpPool`, `CreateDeliverabilityTestReport`, `CreateEmailIdentity`, `DeleteConfigurationSet`, `DeleteConfigurationSetEventDestination`, `DeleteDedicatedIpPool`, `DeleteEmailIdentity`, `PutAccountDedicatedIpWarmupAttributes`, `PutAccountSendingAttributes`, `PutConfigurationSetDeliveryOptions`, `PutConfigurationSetReputationOptions`, `PutConfigurationSetSendingOptions`, `PutConfigurationSetTrackingOptions`, `PutDedicatedIpInPool`, `PutDedicatedIpWarmupAttributes`, `PutDeliverabilityDashboardOption`, `PutEmailIdentityDkimAttributes`, `PutEmailIdentityFeedbackAttributes`, ... (+4).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAccount`, `GetBlacklistReports`, `GetConfigurationSet`, `GetConfigurationSetEventDestinations`, `GetDedicatedIp`, `GetDedicatedIps`, `GetDeliverabilityDashboardOptions`, `GetDeliverabilityTestReport`, `GetDomainDeliverabilityCampaign`, `GetDomainStatisticsReport`, `GetEmailIdentity`, `ListConfigurationSets`, `ListDedicatedIpPools`, `ListDeliverabilityTestReports`, `ListDomainDeliverabilityCampaigns`, `ListEmailIdentities`, `ListTagsForResource`.
- Pagination is modelled for 6 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateDeliverabilityTestReport`, `GetBlacklistReports`, `GetDeliverabilityTestReport`, `GetDomainStatisticsReport`, `ListDeliverabilityTestReports`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 42 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `S3`, `CloudWatch`, `SNS`, `Redshift`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Operation Groups

### Put

- Operations: `PutAccountDedicatedIpWarmupAttributes`, `PutAccountSendingAttributes`, `PutConfigurationSetDeliveryOptions`, `PutConfigurationSetReputationOptions`, `PutConfigurationSetSendingOptions`, `PutConfigurationSetTrackingOptions`, `PutDedicatedIpInPool`, `PutDedicatedIpWarmupAttributes`, `PutDeliverabilityDashboardOption`, `PutEmailIdentityDkimAttributes`, `PutEmailIdentityFeedbackAttributes`, `PutEmailIdentityMailFromAttributes`
- Common required input members in this group: `ConfigurationSetName`, `Ip`, `EmailIdentity`

### Get

- Operations: `GetAccount`, `GetBlacklistReports`, `GetConfigurationSet`, `GetConfigurationSetEventDestinations`, `GetDedicatedIp`, `GetDedicatedIps`, `GetDeliverabilityDashboardOptions`, `GetDeliverabilityTestReport`, `GetDomainDeliverabilityCampaign`, `GetDomainStatisticsReport`, `GetEmailIdentity`
- Traits: `paginated` (1)
- Common required input members in this group: `ConfigurationSetName`

### List

- Operations: `ListConfigurationSets`, `ListDedicatedIpPools`, `ListDeliverabilityTestReports`, `ListDomainDeliverabilityCampaigns`, `ListEmailIdentities`, `ListTagsForResource`
- Traits: `paginated` (5)
- Common required input members in this group: -

### Create

- Operations: `CreateConfigurationSet`, `CreateConfigurationSetEventDestination`, `CreateDedicatedIpPool`, `CreateDeliverabilityTestReport`, `CreateEmailIdentity`
- Common required input members in this group: `ConfigurationSetName`

### Delete

- Operations: `DeleteConfigurationSet`, `DeleteConfigurationSetEventDestination`, `DeleteDedicatedIpPool`, `DeleteEmailIdentity`
- Common required input members in this group: `ConfigurationSetName`

### Send

- Operations: `SendEmail`
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

### Update

- Operations: `UpdateConfigurationSetEventDestination`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateConfigurationSet` | `POST /v1/email/configuration-sets` | - | `ConfigurationSetName` | - | `CreateConfigurationSetResponse` | `AlreadyExistsException`, `BadRequestException`, `ConcurrentModificationException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Create a configuration set. Configuration sets are groups of rules that you can apply to the emails you send using Amazon Pinpoint. You apply a configuration set to an email by including a reference to the configurat ... |
| `CreateConfigurationSetEventDestination` | `POST /v1/email/configuration-sets/{ConfigurationSetName}/event-destinations` | - | `ConfigurationSetName`, `EventDestinationName`, `EventDestination` | - | `CreateConfigurationSetEventDestinationResponse` | `AlreadyExistsException`, `BadRequestException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Create an event destination. In Amazon Pinpoint, events include message sends, deliveries, opens, clicks, bounces, and complaints. Event destinations are places that you can send information about these events to. Fo ... |
| `CreateDedicatedIpPool` | `POST /v1/email/dedicated-ip-pools` | - | `PoolName` | - | `CreateDedicatedIpPoolResponse` | `AlreadyExistsException`, `BadRequestException`, `ConcurrentModificationException`, `LimitExceededException`, `TooManyRequestsException` | Create a new pool of dedicated IP addresses. A pool can include one or more dedicated IP addresses that are associated with your Amazon Pinpoint account. You can associate a pool with a configuration set. When you se ... |
| `CreateDeliverabilityTestReport` | `POST /v1/email/deliverability-dashboard/test` | - | `FromEmailAddress`, `Content` | - | `CreateDeliverabilityTestReportResponse` | `AccountSuspendedException`, `BadRequestException`, `ConcurrentModificationException`, `LimitExceededException`, `MailFromDomainNotVerifiedException`, `MessageRejected`, `NotFoundException`, `SendingPausedException`, `TooManyRequestsException` | Create a new predictive inbox placement test. Predictive inbox placement tests can help you predict how your messages will be handled by various email providers around the world. When you perform a predictive inbox p ... |
| `CreateEmailIdentity` | `POST /v1/email/identities` | - | `EmailIdentity` | - | `CreateEmailIdentityResponse` | `BadRequestException`, `ConcurrentModificationException`, `LimitExceededException`, `TooManyRequestsException` | Verifies an email identity for use with Amazon Pinpoint. In Amazon Pinpoint, an identity is an email address or domain that you use when you send email. Before you can use an identity to send email with Amazon Pinpoi ... |
| `DeleteConfigurationSet` | `DELETE /v1/email/configuration-sets/{ConfigurationSetName}` | - | `ConfigurationSetName` | - | `DeleteConfigurationSetResponse` | `BadRequestException`, `ConcurrentModificationException`, `NotFoundException`, `TooManyRequestsException` | Delete an existing configuration set. In Amazon Pinpoint, configuration sets are groups of rules that you can apply to the emails you send. You apply a configuration set to an email by including a reference to the co ... |
| `DeleteConfigurationSetEventDestination` | `DELETE /v1/email/configuration-sets/{ConfigurationSetName}/event-destinations/{EventDestinationName}` | - | `ConfigurationSetName`, `EventDestinationName` | - | `DeleteConfigurationSetEventDestinationResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Delete an event destination. In Amazon Pinpoint, events include message sends, deliveries, opens, clicks, bounces, and complaints. Event destinations are places that you can send information about these events to. Fo ... |
| `DeleteDedicatedIpPool` | `DELETE /v1/email/dedicated-ip-pools/{PoolName}` | - | `PoolName` | - | `DeleteDedicatedIpPoolResponse` | `BadRequestException`, `ConcurrentModificationException`, `NotFoundException`, `TooManyRequestsException` | Delete a dedicated IP pool. |
| `DeleteEmailIdentity` | `DELETE /v1/email/identities/{EmailIdentity}` | - | `EmailIdentity` | - | `DeleteEmailIdentityResponse` | `BadRequestException`, `ConcurrentModificationException`, `NotFoundException`, `TooManyRequestsException` | Deletes an email identity that you previously verified for use with Amazon Pinpoint. An identity can be either an email address or a domain name. |
| `GetAccount` | `GET /v1/email/account` | - | - | - | `GetAccountResponse` | `BadRequestException`, `TooManyRequestsException` | Obtain information about the email-sending status and capabilities of your Amazon Pinpoint account in the current AWS Region. |
| `GetBlacklistReports` | `GET /v1/email/deliverability-dashboard/blacklist-report` | - | `BlacklistItemNames` | - | `GetBlacklistReportsResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Retrieve a list of the blacklists that your dedicated IP addresses appear on. |
| `GetConfigurationSet` | `GET /v1/email/configuration-sets/{ConfigurationSetName}` | - | `ConfigurationSetName` | - | `GetConfigurationSetResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Get information about an existing configuration set, including the dedicated IP pool that it's associated with, whether or not it's enabled for sending email, and more. In Amazon Pinpoint, configuration sets are grou ... |
| `GetConfigurationSetEventDestinations` | `GET /v1/email/configuration-sets/{ConfigurationSetName}/event-destinations` | - | `ConfigurationSetName` | - | `GetConfigurationSetEventDestinationsResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Retrieve a list of event destinations that are associated with a configuration set. In Amazon Pinpoint, events include message sends, deliveries, opens, clicks, bounces, and complaints. Event destinations are places ... |
| `GetDedicatedIp` | `GET /v1/email/dedicated-ips/{Ip}` | - | `Ip` | - | `GetDedicatedIpResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Get information about a dedicated IP address, including the name of the dedicated IP pool that it's associated with, as well information about the automatic warm-up process for the address. |
| `GetDedicatedIps` | `GET /v1/email/dedicated-ips` | `paginated` | - | - | `GetDedicatedIpsResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | List the dedicated IP addresses that are associated with your Amazon Pinpoint account. |
| `GetDeliverabilityDashboardOptions` | `GET /v1/email/deliverability-dashboard` | - | - | - | `GetDeliverabilityDashboardOptionsResponse` | `BadRequestException`, `LimitExceededException`, `TooManyRequestsException` | Retrieve information about the status of the Deliverability dashboard for your Amazon Pinpoint account. When the Deliverability dashboard is enabled, you gain access to reputation, deliverability, and other metrics f ... |
| `GetDeliverabilityTestReport` | `GET /v1/email/deliverability-dashboard/test-reports/{ReportId}` | - | `ReportId` | - | `GetDeliverabilityTestReportResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Retrieve the results of a predictive inbox placement test. |
| `GetDomainDeliverabilityCampaign` | `GET /v1/email/deliverability-dashboard/campaigns/{CampaignId}` | - | `CampaignId` | - | `GetDomainDeliverabilityCampaignResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Retrieve all the deliverability data for a specific campaign. This data is available for a campaign only if the campaign sent email by using a domain that the Deliverability dashboard is enabled for ( PutDeliverabili ... |
| `GetDomainStatisticsReport` | `GET /v1/email/deliverability-dashboard/statistics-report/{Domain}` | - | `Domain`, `StartDate`, `EndDate` | - | `GetDomainStatisticsReportResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Retrieve inbox placement and engagement rates for the domains that you use to send email. |
| `GetEmailIdentity` | `GET /v1/email/identities/{EmailIdentity}` | - | `EmailIdentity` | - | `GetEmailIdentityResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Provides information about a specific identity associated with your Amazon Pinpoint account, including the identity's verification status, its DKIM authentication status, and its custom Mail-From settings. |
| `ListConfigurationSets` | `GET /v1/email/configuration-sets` | `paginated` | - | - | `ListConfigurationSetsResponse` | `BadRequestException`, `TooManyRequestsException` | List all of the configuration sets associated with your Amazon Pinpoint account in the current region. In Amazon Pinpoint, configuration sets are groups of rules that you can apply to the emails you send. You apply a ... |
| `ListDedicatedIpPools` | `GET /v1/email/dedicated-ip-pools` | `paginated` | - | - | `ListDedicatedIpPoolsResponse` | `BadRequestException`, `TooManyRequestsException` | List all of the dedicated IP pools that exist in your Amazon Pinpoint account in the current AWS Region. |
| `ListDeliverabilityTestReports` | `GET /v1/email/deliverability-dashboard/test-reports` | `paginated` | - | - | `ListDeliverabilityTestReportsResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Show a list of the predictive inbox placement tests that you've performed, regardless of their statuses. For predictive inbox placement tests that are complete, you can use the GetDeliverabilityTestReport operation t ... |
| `ListDomainDeliverabilityCampaigns` | `GET /v1/email/deliverability-dashboard/domains/{SubscribedDomain}/campaigns` | `paginated` | `StartDate`, `EndDate`, `SubscribedDomain` | - | `ListDomainDeliverabilityCampaignsResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Retrieve deliverability data for all the campaigns that used a specific domain to send email during a specified time range. This data is available for a domain only if you enabled the Deliverability dashboard ( PutDe ... |
| `ListEmailIdentities` | `GET /v1/email/identities` | `paginated` | - | - | `ListEmailIdentitiesResponse` | `BadRequestException`, `TooManyRequestsException` | Returns a list of all of the email identities that are associated with your Amazon Pinpoint account. An identity can be either an email address or a domain. This operation returns identities that are verified as well ... |
| `ListTagsForResource` | `GET /v1/email/tags` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Retrieve a list of the tags (keys and values) that are associated with a specified resource. A tag is a label that you optionally define and associate with a resource in Amazon Pinpoint. Each tag consists of a requir ... |
| `PutAccountDedicatedIpWarmupAttributes` | `PUT /v1/email/account/dedicated-ips/warmup` | - | - | - | `PutAccountDedicatedIpWarmupAttributesResponse` | `BadRequestException`, `TooManyRequestsException` | Enable or disable the automatic warm-up feature for dedicated IP addresses. |
| `PutAccountSendingAttributes` | `PUT /v1/email/account/sending` | - | - | - | `PutAccountSendingAttributesResponse` | `BadRequestException`, `TooManyRequestsException` | Enable or disable the ability of your account to send email. |
| `PutConfigurationSetDeliveryOptions` | `PUT /v1/email/configuration-sets/{ConfigurationSetName}/delivery-options` | - | `ConfigurationSetName` | - | `PutConfigurationSetDeliveryOptionsResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Associate a configuration set with a dedicated IP pool. You can use dedicated IP pools to create groups of dedicated IP addresses for sending specific types of email. |
| `PutConfigurationSetReputationOptions` | `PUT /v1/email/configuration-sets/{ConfigurationSetName}/reputation-options` | - | `ConfigurationSetName` | - | `PutConfigurationSetReputationOptionsResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Enable or disable collection of reputation metrics for emails that you send using a particular configuration set in a specific AWS Region. |
| `PutConfigurationSetSendingOptions` | `PUT /v1/email/configuration-sets/{ConfigurationSetName}/sending` | - | `ConfigurationSetName` | - | `PutConfigurationSetSendingOptionsResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Enable or disable email sending for messages that use a particular configuration set in a specific AWS Region. |
| `PutConfigurationSetTrackingOptions` | `PUT /v1/email/configuration-sets/{ConfigurationSetName}/tracking-options` | - | `ConfigurationSetName` | - | `PutConfigurationSetTrackingOptionsResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Specify a custom domain to use for open and click tracking elements in email that you send using Amazon Pinpoint. |
| `PutDedicatedIpInPool` | `PUT /v1/email/dedicated-ips/{Ip}/pool` | - | `Ip`, `DestinationPoolName` | - | `PutDedicatedIpInPoolResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Move a dedicated IP address to an existing dedicated IP pool. The dedicated IP address that you specify must already exist, and must be associated with your Amazon Pinpoint account. The dedicated IP pool you specify ... |
| `PutDedicatedIpWarmupAttributes` | `PUT /v1/email/dedicated-ips/{Ip}/warmup` | - | `Ip`, `WarmupPercentage` | - | `PutDedicatedIpWarmupAttributesResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | - |
| `PutDeliverabilityDashboardOption` | `PUT /v1/email/deliverability-dashboard` | - | `DashboardEnabled` | - | `PutDeliverabilityDashboardOptionResponse` | `AlreadyExistsException`, `BadRequestException`, `LimitExceededException`, `NotFoundException`, `TooManyRequestsException` | Enable or disable the Deliverability dashboard for your Amazon Pinpoint account. When you enable the Deliverability dashboard, you gain access to reputation, deliverability, and other metrics for the domains that you ... |
| `PutEmailIdentityDkimAttributes` | `PUT /v1/email/identities/{EmailIdentity}/dkim` | - | `EmailIdentity` | - | `PutEmailIdentityDkimAttributesResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Used to enable or disable DKIM authentication for an email identity. |
| `PutEmailIdentityFeedbackAttributes` | `PUT /v1/email/identities/{EmailIdentity}/feedback` | - | `EmailIdentity` | - | `PutEmailIdentityFeedbackAttributesResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Used to enable or disable feedback forwarding for an identity. This setting determines what happens when an identity is used to send an email that results in a bounce or complaint event. When you enable feedback forw ... |
| `PutEmailIdentityMailFromAttributes` | `PUT /v1/email/identities/{EmailIdentity}/mail-from` | - | `EmailIdentity` | - | `PutEmailIdentityMailFromAttributesResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Used to enable or disable the custom Mail-From domain configuration for an email identity. |
| `SendEmail` | `POST /v1/email/outbound-emails` | - | `Destination`, `Content` | - | `SendEmailResponse` | `AccountSuspendedException`, `BadRequestException`, `LimitExceededException`, `MailFromDomainNotVerifiedException`, `MessageRejected`, `NotFoundException`, `SendingPausedException`, `TooManyRequestsException` | Sends an email message. You can use the Amazon Pinpoint Email API to send two types of messages: Simple – A standard email message. When you create this type of message, you specify the sender, the recipient, and the ... |
| `TagResource` | `POST /v1/email/tags` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `BadRequestException`, `ConcurrentModificationException`, `NotFoundException`, `TooManyRequestsException` | Add one or more tags (keys and values) to a specified resource. A tag is a label that you optionally define and associate with a resource in Amazon Pinpoint. Tags can help you categorize and manage resources in diffe ... |
| `UntagResource` | `DELETE /v1/email/tags` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `BadRequestException`, `ConcurrentModificationException`, `NotFoundException`, `TooManyRequestsException` | Remove one or more tags (keys and values) from a specified resource. |
| `UpdateConfigurationSetEventDestination` | `PUT /v1/email/configuration-sets/{ConfigurationSetName}/event-destinations/{EventDestinationName}` | - | `ConfigurationSetName`, `EventDestinationName`, `EventDestination` | - | `UpdateConfigurationSetEventDestinationResponse` | `BadRequestException`, `NotFoundException`, `TooManyRequestsException` | Update the configuration of an event destination for a configuration set. In Amazon Pinpoint, events include message sends, deliveries, opens, clicks, bounces, and complaints. Event destinations are places that you c ... |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetBlacklistReports` | - | `BlacklistItemNames -> BlacklistItemNames` | - | - |
| `GetDedicatedIps` | - | `PoolName -> PoolName`, `NextToken -> NextToken`, `PageSize -> PageSize` | - | - |
| `GetDomainStatisticsReport` | - | `StartDate -> StartDate`, `EndDate -> EndDate` | - | - |
| `ListConfigurationSets` | - | `NextToken -> NextToken`, `PageSize -> PageSize` | - | - |
| `ListDedicatedIpPools` | - | `NextToken -> NextToken`, `PageSize -> PageSize` | - | - |
| `ListDeliverabilityTestReports` | - | `NextToken -> NextToken`, `PageSize -> PageSize` | - | - |
| `ListDomainDeliverabilityCampaigns` | - | `StartDate -> StartDate`, `EndDate -> EndDate`, `NextToken -> NextToken`, `PageSize -> PageSize` | - | - |
| `ListEmailIdentities` | - | `NextToken -> NextToken`, `PageSize -> PageSize` | - | - |
| `ListTagsForResource` | - | `ResourceArn -> ResourceArn` | - | - |
| `UntagResource` | - | `ResourceArn -> ResourceArn`, `TagKeys -> TagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccountSuspendedException` | `structure` | message | The message can't be sent because the account's ability to send email has been permanently restricted. |
| `AlreadyExistsException` | `structure` | message | The resource specified in your request already exists. |
| `BadRequestException` | `structure` | message | The input you provided is invalid. |
| `ConcurrentModificationException` | `structure` | message | The resource is being modified by another operation or thread. |
| `LimitExceededException` | `structure` | message | There are too many instances of the specified resource type. |
| `MailFromDomainNotVerifiedException` | `structure` | message | The message can't be sent because the sending domain isn't verified. |
| `MessageRejected` | `structure` | message | The message can't be sent because it contains invalid content. |
| `NotFoundException` | `structure` | message | The resource you attempted to access doesn't exist. |
| `SendingPausedException` | `structure` | message | The message can't be sent because the account's ability to send email is currently paused. |
| `TooManyRequestsException` | `structure` | message | Too many requests have been made to the operation. |
| `CreateConfigurationSetRequest` | `structure` | ConfigurationSetName, TrackingOptions, DeliveryOptions, ReputationOptions, SendingOptions, Tags | A request to create a configuration set. |
| `CreateConfigurationSetResponse` | `structure` | **empty (no members)** | An HTTP 200 response if the request succeeds, or an error message if the request fails. |
| `CreateConfigurationSetEventDestinationRequest` | `structure` | ConfigurationSetName, EventDestinationName, EventDestination | A request to add an event destination to a configuration set. |
| `CreateConfigurationSetEventDestinationResponse` | `structure` | **empty (no members)** | An HTTP 200 response if the request succeeds, or an error message if the request fails. |
| `CreateDedicatedIpPoolRequest` | `structure` | PoolName, Tags | A request to create a new dedicated IP pool. |
| `CreateDedicatedIpPoolResponse` | `structure` | **empty (no members)** | An HTTP 200 response if the request succeeds, or an error message if the request fails. |
| `CreateDeliverabilityTestReportRequest` | `structure` | ReportName, FromEmailAddress, Content, Tags | A request to perform a predictive inbox placement test. Predictive inbox placement tests can help you predict how your messages will be handled by various e ... |
| `CreateDeliverabilityTestReportResponse` | `structure` | ReportId, DeliverabilityTestStatus | Information about the predictive inbox placement test that you created. |
| `CreateEmailIdentityRequest` | `structure` | EmailIdentity, Tags | A request to begin the verification process for an email identity (an email address or domain). |
| `CreateEmailIdentityResponse` | `structure` | IdentityType, VerifiedForSendingStatus, DkimAttributes | If the email identity is a domain, this object contains tokens that you can use to create a set of CNAME records. To sucessfully verify your domain, you hav ... |
| `DeleteConfigurationSetRequest` | `structure` | ConfigurationSetName | A request to delete a configuration set. |
| `DeleteConfigurationSetResponse` | `structure` | **empty (no members)** | An HTTP 200 response if the request succeeds, or an error message if the request fails. |
| `DeleteConfigurationSetEventDestinationRequest` | `structure` | ConfigurationSetName, EventDestinationName | A request to delete an event destination from a configuration set. |
| `DeleteConfigurationSetEventDestinationResponse` | `structure` | **empty (no members)** | An HTTP 200 response if the request succeeds, or an error message if the request fails. |
| `DeleteDedicatedIpPoolRequest` | `structure` | PoolName | A request to delete a dedicated IP pool. |
| `DeleteDedicatedIpPoolResponse` | `structure` | **empty (no members)** | An HTTP 200 response if the request succeeds, or an error message if the request fails. |
| `DeleteEmailIdentityRequest` | `structure` | EmailIdentity | A request to delete an existing email identity. When you delete an identity, you lose the ability to use Amazon Pinpoint to send email from that identity. Y ... |
| `DeleteEmailIdentityResponse` | `structure` | **empty (no members)** | An HTTP 200 response if the request succeeds, or an error message if the request fails. |
| `GetAccountRequest` | `structure` | **empty (no members)** | A request to obtain information about the email-sending capabilities of your Amazon Pinpoint account. |
| `GetAccountResponse` | `structure` | SendQuota, SendingEnabled, DedicatedIpAutoWarmupEnabled, EnforcementStatus, ProductionAccessEnabled | A list of details about the email-sending capabilities of your Amazon Pinpoint account in the current AWS Region. |
| `GetBlacklistReportsRequest` | `structure` | BlacklistItemNames | A request to retrieve a list of the blacklists that your dedicated IP addresses appear on. |
| `GetBlacklistReportsResponse` | `structure` | BlacklistReport | An object that contains information about blacklist events. |
| `GetConfigurationSetRequest` | `structure` | ConfigurationSetName | A request to obtain information about a configuration set. |
| `GetConfigurationSetResponse` | `structure` | ConfigurationSetName, TrackingOptions, DeliveryOptions, ReputationOptions, SendingOptions, Tags | Information about a configuration set. |
| `GetConfigurationSetEventDestinationsRequest` | `structure` | ConfigurationSetName | A request to obtain information about the event destinations for a configuration set. |
| `GetConfigurationSetEventDestinationsResponse` | `structure` | EventDestinations | Information about an event destination for a configuration set. |
| `GetDedicatedIpRequest` | `structure` | Ip | A request to obtain more information about a dedicated IP address. |
| `GetDedicatedIpResponse` | `structure` | DedicatedIp | Information about a dedicated IP address. |
| `GetDedicatedIpsRequest` | `structure` | PoolName, NextToken, PageSize | A request to obtain more information about dedicated IP pools. |
| `GetDedicatedIpsResponse` | `structure` | DedicatedIps, NextToken | Information about the dedicated IP addresses that are associated with your Amazon Pinpoint account. |
| `BehaviorOnMxFailure` | `enum` | USE_DEFAULT_VALUE, REJECT_MESSAGE | The action that you want Amazon Pinpoint to take if it can't read the required MX record for a custom MAIL FROM domain. When you set this value to UseDefaul ... |
| `DeliverabilityDashboardAccountStatus` | `enum` | ACTIVE, PENDING_EXPIRATION, DISABLED | The current status of your Deliverability dashboard subscription. If this value is PENDING_EXPIRATION , your subscription is scheduled to expire at the end ... |
| `DeliverabilityTestStatus` | `enum` | IN_PROGRESS, COMPLETED | The status of a predictive inbox placement test. If the status is IN_PROGRESS , then the predictive inbox placement test is currently running. Predictive in ... |
| `DimensionValueSource` | `enum` | MESSAGE_TAG, EMAIL_HEADER, LINK_TAG | The location where Amazon Pinpoint finds the value of a dimension to publish to Amazon CloudWatch. If you want Amazon Pinpoint to use the message tags that ... |
| `DkimStatus` | `enum` | PENDING, SUCCESS, FAILED, TEMPORARY_FAILURE, NOT_STARTED | The DKIM authentication status of the identity. The status can be one of the following: PENDING – The DKIM verification process was initiated, and Amazon Pi ... |
| `EventType` | `enum` | SEND, REJECT, BOUNCE, COMPLAINT, DELIVERY, OPEN, CLICK, RENDERING_FAILURE | An email sending event type. For example, email sends, opens, and bounces are all email events. |
| `IdentityType` | `enum` | EMAIL_ADDRESS, DOMAIN, MANAGED_DOMAIN | The email identity type. The identity type can be one of the following: EMAIL_ADDRESS – The identity is an email address. DOMAIN – The identity is a domain. |
| `MailFromDomainStatus` | `enum` | PENDING, SUCCESS, FAILED, TEMPORARY_FAILURE | The status of the MAIL FROM domain. This status can have the following values: PENDING – Amazon Pinpoint hasn't started searching for the MX record yet. SUC ... |
| `TlsPolicy` | `enum` | REQUIRE, OPTIONAL | Specifies whether messages that use the configuration set are required to use Transport Layer Security (TLS). If the value is Require , messages are only de ... |
| `WarmupStatus` | `enum` | IN_PROGRESS, DONE | The warmup status of a dedicated IP. |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
