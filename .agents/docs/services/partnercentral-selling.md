# Partner Central Selling API

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

AWS Partner Central API for Selling AWS Partner Central API for Selling Reference Guide This Amazon Web Services (AWS) Partner Central API reference is designed to help AWS Partners integrate Customer Relationship Management (CRM) systems with AWS Partner Central. Partners can automate interactions with AWS Partner Central, which helps to ensure effective engagements in joint business activities. The API provides standard AWS API functionality. Access it by either using API Actions or by using an AWS SDK that's tailored to your programming language or platform. For more information, see Getting Started with AWS and Tools to Build on AWS.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Partner Central Selling API where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for Partner Central Selling API by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for Partner Central Selling API resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: represent documented Partner Central Selling API workflows in the local mock. Key resources include `Engagement`, `EngagementByAcceptingInvitationTask`, `EngagementFromOpportunityTask`, `EngagementInvitation`, `Opportunity`.
- From the operation surface: model setup and mutation workflows that create or update service resources across the `List`, `Get`, `Create`, `Start`, `Update` operation families, including `ListEngagementByAcceptingInvitationTasks`, `ListEngagementFromOpportunityTasks`, `ListEngagementInvitations`, `ListEngagementMembers`, `GetAwsOpportunitySummary`, `GetEngagement`.

## Service Identity and Protocol

- AWS model slug: `partnercentral-selling`
- AWS SDK for Rust slug: `partnercentralselling`
- Model version: `2022-07-26`
- Model file: `vendor/api-models-aws/models/partnercentral-selling/service/2022-07-26/partnercentral-selling-2022-07-26.json`
- SDK ID: `PartnerCentral Selling`
- Endpoint prefix: `partnercentral-selling`
- ARN namespace: `partnercentral`
- CloudFormation name: `-`
- CloudTrail event source: `partnercentral-selling.amazonaws.com`
- Protocols: `awsJson1_0`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (12), `Get` (7), `Create` (6), `Start` (4), `Update` (2), `Accept` (1), `Assign` (1), `Associate` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptEngagementInvitation`, `AssociateOpportunity`, `CreateEngagement`, `CreateEngagementContext`, `CreateEngagementInvitation`, `CreateOpportunity`, `CreateResourceSnapshot`, `CreateResourceSnapshotJob`, `DeleteResourceSnapshotJob`, `DisassociateOpportunity`, `PutSellingSystemSettings`, `RejectEngagementInvitation`, `StartEngagementByAcceptingInvitationTask`, `StartEngagementFromOpportunityTask`, `StartOpportunityFromEngagementTask`, `StartResourceSnapshotJob`, `StopResourceSnapshotJob`, `SubmitOpportunity`, `TagResource`, `UntagResource`, ... (+2).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `GetAwsOpportunitySummary`, `GetEngagement`, `GetEngagementInvitation`, `GetOpportunity`, `GetResourceSnapshot`, `GetResourceSnapshotJob`, `GetSellingSystemSettings`, `ListEngagementByAcceptingInvitationTasks`, `ListEngagementFromOpportunityTasks`, `ListEngagementInvitations`, `ListEngagementMembers`, `ListEngagementResourceAssociations`, `ListEngagements`, `ListOpportunities`, `ListOpportunityFromEngagementTasks`, `ListResourceSnapshotJobs`, `ListResourceSnapshots`, `ListSolutions`, `ListTagsForResource`.
- Pagination is modelled for 11 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 15 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateResourceSnapshotJob`, `DeleteResourceSnapshotJob`, `GetResourceSnapshotJob`, `ListEngagementByAcceptingInvitationTasks`, `ListEngagementFromOpportunityTasks`, `ListOpportunityFromEngagementTasks`, `ListResourceSnapshotJobs`, `StartEngagementByAcceptingInvitationTask`, `StartEngagementFromOpportunityTask`, `StartOpportunityFromEngagementTask`, `StartResourceSnapshotJob`, `StopResourceSnapshotJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 42 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `EventBridge`, `ECS`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.


## Resource Model

| Resource | Identifiers | Lifecycle operations | Other operations | Documentation cue |
|---|---|---|---|---|
| `Engagement` | `Catalog`, `Identifier` | create: `CreateEngagement`; read: `GetEngagement`; list: `ListEngagements` | `ListEngagementMembers` | Represents an engagement that is in AWS Partner Central |
| `EngagementByAcceptingInvitationTask` | `Catalog`, `TaskId` | create: `StartEngagementByAcceptingInvitationTask`; list: `ListEngagementByAcceptingInvitationTasks` | - | Represents a task that starts an Engagement by accepting an Engagement Invitation in AWS Partner Central |
| `EngagementFromOpportunityTask` | `Catalog`, `TaskId` | create: `StartEngagementFromOpportunityTask`; list: `ListEngagementFromOpportunityTasks` | - | Represents a task that starts an Engagement from an Opportunity in AWS Partner Central |
| `EngagementInvitation` | `Catalog`, `Identifier` | create: `CreateEngagementInvitation`; read: `GetEngagementInvitation`; list: `ListEngagementInvitations` | `AcceptEngagementInvitation`, `RejectEngagementInvitation` | Represents an Engagement Invitation that allows to join an Engagement on AWS Partner Central |
| `Opportunity` | `Catalog`, `Identifier` | create: `CreateOpportunity`; read: `GetOpportunity`; update: `UpdateOpportunity`; list: `ListOpportunities` | `AssignOpportunity`, `AssociateOpportunity`, `DisassociateOpportunity`, `GetAwsOpportunitySummary`, `SubmitOpportunity` | Represents an Opportunity that captures deal details in AWS Partner Central |
| `OpportunityFromEngagementTask` | `Catalog`, `TaskId` | create: `StartOpportunityFromEngagementTask`; list: `ListOpportunityFromEngagementTasks` | - | Represents a task that starts an Opportunity from an Engagement in AWS Partner Central |
| `ResourceSnapshot` | - | - | `CreateResourceSnapshot`, `GetResourceSnapshot`, `ListEngagementResourceAssociations`, `ListResourceSnapshots` | Represents a snapshot of a resource that is in AWS Partner Central |
| `ResourceSnapshotJob` | `Catalog`, `Identifier` | create: `CreateResourceSnapshotJob`; read: `GetResourceSnapshotJob`; delete: `DeleteResourceSnapshotJob`; list: `ListResourceSnapshotJobs` | `StartResourceSnapshotJob`, `StopResourceSnapshotJob` | Represents a job that captures snapshot resource in AWS Partner Central |
| `Solution` | `Catalog`, `Identifier` | list: `ListSolutions` | - | Represents a Solution that is listed on AWS Partner Central |
## Operation Groups

### Create

- Operations: `CreateEngagementContext`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Get

- Operations: `GetSellingSystemSettings`
- Traits: `readonly` (1)
- Common required input members in this group: -

### List

- Operations: `ListTagsForResource`
- Traits: `readonly` (1)
- Common required input members in this group: -

### Put

- Operations: `PutSellingSystemSettings`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Update

- Operations: `UpdateEngagementContext`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `CreateEngagementContext` | `POST /CreateEngagementContext` | `idempotent`, `idempotency-token` | `Catalog`, `EngagementIdentifier`, `ClientToken`, `Type`, `Payload` | `ClientToken` | `CreateEngagementContextResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new context within an existing engagement. This action allows you to add contextual information such as customer projects or documents to an engagement, providing additional details that help facilitate col ... |
| `GetSellingSystemSettings` | `POST /GetSellingSystemSettings` | `readonly` | `Catalog` | - | `GetSellingSystemSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the currently set system settings, which include the IAM Role used for resource snapshot jobs. |
| `ListTagsForResource` | `POST /ListTagsForResource` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of tags for a resource. |
| `PutSellingSystemSettings` | `POST /PutSellingSystemSettings` | `idempotent` | `Catalog` | - | `PutSellingSystemSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the currently set system settings, which include the IAM Role used for resource snapshot jobs. |
| `TagResource` | `POST /TagResource` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Assigns one or more tags (key-value pairs) to the specified resource. |
| `UntagResource` | `POST /UntagResource` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a tag or tags from a resource. |
| `UpdateEngagementContext` | `POST /UpdateEngagementContext` | - | `Catalog`, `EngagementIdentifier`, `ContextIdentifier`, `EngagementLastModifiedAt`, `Type`, `Payload` | - | `UpdateEngagementContextResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the context information for an existing engagement with new or modified data. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

_No `@httpHeader`, `@httpQuery`, `@httpPrefixHeaders`, or `@httpPayload` input members are modelled for this service (typical for `awsJson1_*` protocols, where all input flows through the JSON body)._

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message, Reason | This error occurs when you don't have permission to perform the requested action. You don’t have access to this action or resource. Review IAM policies or c ... |
| `ConflictException` | `structure` | Message | This error occurs when the request can’t be processed due to a conflict with the target resource's current state, which could result from updating or deleti ... |
| `InternalServerException` | `structure` | Message | This error occurs when the specified resource can’t be found or doesn't exist. Resource ID and type might be incorrect. Suggested action: This is usually a ... |
| `ResourceNotFoundException` | `structure` | Message | This error occurs when the specified resource can't be found. The resource might not exist, or isn't visible with the current credentials. Suggested action: ... |
| `ServiceQuotaExceededException` | `structure` | Message | This error occurs when the request would cause a service quota to be exceeded. Service quotas represent the maximum allowed use of a specific resource, and ... |
| `ThrottlingException` | `structure` | Message | This error occurs when there are too many requests sent. Review the provided quotas and adapt your usage to avoid throttling. This error occurs when there a ... |
| `ValidationException` | `structure` | Message, Reason, ErrorList | The input fails to satisfy the constraints specified by the service or business validation rules. Suggested action: Review the error message, including the ... |
| `CreateEngagementContextRequest` | `structure` | Catalog, EngagementIdentifier, ClientToken, Type, Payload | - |
| `CreateEngagementContextResponse` | `structure` | EngagementId, EngagementArn, EngagementLastModifiedAt, ContextId | - |
| `GetSellingSystemSettingsRequest` | `structure` | Catalog | - |
| `GetSellingSystemSettingsResponse` | `structure` | Catalog, ResourceSnapshotJobRoleArn | - |
| `ListTagsForResourceRequest` | `structure` | ResourceArn | - |
| `ListTagsForResourceResponse` | `structure` | Tags | - |
| `PutSellingSystemSettingsRequest` | `structure` | Catalog, ResourceSnapshotJobRoleIdentifier | - |
| `PutSellingSystemSettingsResponse` | `structure` | Catalog, ResourceSnapshotJobRoleArn | - |
| `TagResourceRequest` | `structure` | ResourceArn, Tags | - |
| `TagResourceResponse` | `structure` | **empty (no members)** | - |
| `UntagResourceRequest` | `structure` | ResourceArn, TagKeys | - |
| `UntagResourceResponse` | `structure` | **empty (no members)** | - |
| `UpdateEngagementContextRequest` | `structure` | Catalog, EngagementIdentifier, ContextIdentifier, EngagementLastModifiedAt, Type, Payload | - |
| `UpdateEngagementContextResponse` | `structure` | EngagementId, EngagementArn, EngagementLastModifiedAt, ContextId | - |
| `AccessDeniedExceptionErrorCode` | `enum` | INCOMPATIBLE_BENEFIT_AWS_PARTNER_STATE | - |
| `AwsClosedLostReason` | `enum` | ADMINISTRATIVE, BUSINESS_ASSOCIATE_AGREEMENT, COMPANY_ACQUIRED_DISSOLVED, COMPETITIVE_OFFERING, CUSTOMER_DATA_REQUIREMENT, CUSTOMER_DEFICIENCY, CUSTOMER_EXPERIENCE, DELAY_CANCELLATION_OF_PROJECT, DUPLICATE, DUPLICATE_OPPORTUNITY, EXECUTIVE_BLOCKER, FAILED_VETTING, ... (+36) | - |
| `AwsFundingUsed` | `enum` | YES, NO | - |
| `AwsMemberBusinessTitle` | `enum` | AWS_SALES_REP, AWS_ACCOUNT_OWNER, WWPSPDM, PDM, PSM, ISVSM | - |
| `AwsOpportunityStage` | `enum` | NOT_STARTED, IN_PROGRESS, PROSPECT, ENGAGED, IDENTIFIED, QUALIFY, RESEARCH, SELLER_ENGAGED, EVALUATING, SELLER_REGISTERED, TERM_SHEET_NEGOTIATION, CONTRACT_NEGOTIATION, ... (+12) | - |
| `AwsPartition` | `enum` | AWS_EUSC | - |
| `Channel` | `enum` | AWS_MARKETING_CENTRAL, CONTENT_SYNDICATION, DISPLAY, EMAIL, LIVE_EVENT, OUT_OF_HOME, PRINT, SEARCH, SOCIAL, TELEMARKETING, TV, VIDEO, ... (+1) | - |
| `ClosedLostReason` | `enum` | CUSTOMER_DEFICIENCY, DELAY_CANCELLATION_OF_PROJECT, LEGAL_TAX_REGULATORY, LOST_TO_COMPETITOR_GOOGLE, LOST_TO_COMPETITOR_MICROSOFT, LOST_TO_COMPETITOR_SOFTLAYER, LOST_TO_COMPETITOR_VMWARE, LOST_TO_COMPETITOR_OTHER, NO_OPPORTUNITY, ON_PREMISES_DEPLOYMENT, PARTNER_GAP, PRICE, ... (+7) | - |
| `CompetitorName` | `enum` | ORACLE_CLOUD, ON_PREM, CO_LOCATION, AKAMAI, ALICLOUD, GOOGLE_CLOUD_PLATFORM, IBM_SOFTLAYER, MICROSOFT_AZURE, OTHER_COST_OPTIMIZATION, NO_COMPETITION, OTHER | - |
| `CountryCode` | `enum` | US, AF, AX, AL, DZ, AS, AD, AO, AI, AQ, AG, AR, ... (+237) | - |
| `CurrencyCode` | `enum` | USD, EUR, GBP, AUD, CAD, CNY, NZD, INR, JPY, CHF, SEK, AED, ... (+156) | - |
| `DeliveryModel` | `enum` | SAAS_OR_PAAS, BYOL_OR_AMI, MANAGED_SERVICES, PROFESSIONAL_SERVICES, RESELL, OTHER | - |
| `EngagementContextType` | `enum` | CUSTOMER_PROJECT, LEAD | - |
| `EngagementInvitationPayloadType` | `enum` | OPPORTUNITY_INVITATION, LEAD_INVITATION | - |
| `EngagementScore` | `enum` | HIGH, MEDIUM, LOW | - |
| `EngagementSortName` | `enum` | CreatedDate | - |
| `Industry` | `enum` | AEROSPACE_SATELLITE, AGRICULTURE, AUTOMOTIVE, COMPUTERS_ELECTRONICS, CONSUMER_GOODS, EDUCATION, ENERGY_OIL_GAS, ENERGY_POWER_UTILITIES, FINANCIAL_SERVICES, GAMING, GOVERNMENT, HEALTHCARE, ... (+16) | - |
| `InvitationStatus` | `enum` | ACCEPTED, PENDING, REJECTED, EXPIRED | - |
| `InvolvementTypeChangeReason` | `enum` | EXPANSION_OPPORTUNITY, CHANGE_IN_DEAL_INFORMATION, CUSTOMER_REQUESTED, TECHNICAL_COMPLEXITY, RISK_MITIGATION | - |
| `ListTasksSortName` | `enum` | START_TIME | - |
| `MarketSegment` | `enum` | ENTERPRISE, LARGE, MEDIUM, SMALL, MICRO | - |
| `MarketingSource` | `enum` | MARKETING_ACTIVITY, NONE | - |
| `NationalSecurity` | `enum` | YES, NO | - |
| `OpportunityEngagementInvitationSortName` | `enum` | INVITATION_DATE | - |
| `OpportunityOrigin` | `enum` | AWS_REFERRAL, PARTNER_REFERRAL | - |
| `OpportunitySortName` | `enum` | LAST_MODIFIEDDATE, IDENTIFIER, CUSTOMER_COMPANY_NAME, CREATED_DATE, TARGET_CLOSE_DATE | - |
| `OpportunityType` | `enum` | NET_NEW_BUSINESS, FLAT_RENEWAL, EXPANSION | - |
| `ParticipantType` | `enum` | SENDER, RECEIVER | - |
| `PaymentFrequency` | `enum` | MONTHLY | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
