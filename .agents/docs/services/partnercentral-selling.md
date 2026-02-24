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

### List

- Operations: `ListEngagementByAcceptingInvitationTasks`, `ListEngagementFromOpportunityTasks`, `ListEngagementInvitations`, `ListEngagementMembers`, `ListEngagementResourceAssociations`, `ListEngagements`, `ListOpportunities`, `ListOpportunityFromEngagementTasks`, `ListResourceSnapshotJobs`, `ListResourceSnapshots`, `ListSolutions`, `ListTagsForResource`
- Traits: `paginated` (11), `readonly` (12)
- Common required input members in this group: `Catalog`, `EngagementIdentifier`, `Identifier`, `ParticipantType`, `ResourceArn`

### Get

- Operations: `GetAwsOpportunitySummary`, `GetEngagement`, `GetEngagementInvitation`, `GetOpportunity`, `GetResourceSnapshot`, `GetResourceSnapshotJob`, `GetSellingSystemSettings`
- Traits: `readonly` (7)
- Common required input members in this group: `Catalog`, `EngagementIdentifier`, `Identifier`, `RelatedOpportunityIdentifier`, `ResourceIdentifier`, `ResourceSnapshotJobIdentifier`, `ResourceSnapshotTemplateIdentifier`, `ResourceType`

### Create

- Operations: `CreateEngagement`, `CreateEngagementContext`, `CreateEngagementInvitation`, `CreateOpportunity`, `CreateResourceSnapshot`, `CreateResourceSnapshotJob`
- Traits: `idempotency-token` (6), `idempotent` (6)
- Common required input members in this group: `Catalog`, `ClientToken`, `Description`, `EngagementIdentifier`, `Invitation`, `Payload`, `ResourceIdentifier`, `ResourceSnapshotTemplateIdentifier`, `ResourceType`, `Title`, `Type`

### Start

- Operations: `StartEngagementByAcceptingInvitationTask`, `StartEngagementFromOpportunityTask`, `StartOpportunityFromEngagementTask`, `StartResourceSnapshotJob`
- Traits: `idempotency-token` (3), `idempotent` (1)
- Common required input members in this group: `AwsSubmission`, `Catalog`, `ClientToken`, `ContextIdentifier`, `Identifier`, `ResourceSnapshotJobIdentifier`

### Update

- Operations: `UpdateEngagementContext`, `UpdateOpportunity`
- Common required input members in this group: `Catalog`, `ContextIdentifier`, `EngagementIdentifier`, `EngagementLastModifiedAt`, `Identifier`, `LastModifiedDate`, `Payload`, `Type`

### Accept

- Operations: `AcceptEngagementInvitation`
- Common required input members in this group: `Catalog`, `Identifier`

### Assign

- Operations: `AssignOpportunity`
- Common required input members in this group: `Assignee`, `Catalog`, `Identifier`

### Associate

- Operations: `AssociateOpportunity`
- Common required input members in this group: `Catalog`, `OpportunityIdentifier`, `RelatedEntityIdentifier`, `RelatedEntityType`

### Delete

- Operations: `DeleteResourceSnapshotJob`
- Traits: `idempotent` (1)
- Common required input members in this group: `Catalog`, `ResourceSnapshotJobIdentifier`

### Disassociate

- Operations: `DisassociateOpportunity`
- Common required input members in this group: `Catalog`, `OpportunityIdentifier`, `RelatedEntityIdentifier`, `RelatedEntityType`

### Put

- Operations: `PutSellingSystemSettings`
- Traits: `idempotent` (1)
- Common required input members in this group: `Catalog`

### Reject

- Operations: `RejectEngagementInvitation`
- Common required input members in this group: `Catalog`, `Identifier`

### Stop

- Operations: `StopResourceSnapshotJob`
- Traits: `idempotent` (1)
- Common required input members in this group: `Catalog`, `ResourceSnapshotJobIdentifier`

### Submit

- Operations: `SubmitOpportunity`
- Common required input members in this group: `Catalog`, `Identifier`, `InvolvementType`

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
| `AcceptEngagementInvitation` | `POST /AcceptEngagementInvitation` | - | `Catalog`, `Identifier` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use the `AcceptEngagementInvitation` action to accept an engagement invitation shared by AWS. Accepting the invitation indicates your willingness to participate in the engagement, granting you access to all engagement-related data. |
| `AssignOpportunity` | `POST /AssignOpportunity` | - | `Assignee`, `Catalog`, `Identifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables you to reassign an existing `Opportunity` to another user within your Partner Central account. The specified user receives the opportunity, and it appears on their Partner Central dashboard, allowing them to take necessary actions or proceed with the... |
| `AssociateOpportunity` | `POST /AssociateOpportunity` | - | `Catalog`, `OpportunityIdentifier`, `RelatedEntityIdentifier`, `RelatedEntityType` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables you to create a formal association between an `Opportunity` and various related entities, enriching the context and details of the opportunity for better collaboration and decision making. You can associate an opportunity with the following entity... |
| `CreateEngagement` | `POST /CreateEngagement` | `idempotent`, `idempotency-token` | `Catalog`, `ClientToken`, `Description`, `Title` | `ClientToken` | `CreateEngagementResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | The `CreateEngagement` action allows you to create an `Engagement`, which serves as a collaborative space between different parties such as AWS Partners and AWS Sellers. This action automatically adds the caller's AWS account as an active member of the newly... |
| `CreateEngagementContext` | `POST /CreateEngagementContext` | `idempotent`, `idempotency-token` | `Catalog`, `ClientToken`, `EngagementIdentifier`, `Payload`, `Type` | `ClientToken` | `CreateEngagementContextResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a new context within an existing engagement. This action allows you to add contextual information such as customer projects or documents to an engagement, providing additional details that help facilitate collaboration between engagement members. |
| `CreateEngagementInvitation` | `POST /CreateEngagementInvitation` | `idempotent`, `idempotency-token` | `Catalog`, `ClientToken`, `EngagementIdentifier`, `Invitation` | `ClientToken` | `CreateEngagementInvitationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | This action creates an invitation from a sender to a single receiver to join an engagement. |
| `CreateOpportunity` | `POST /CreateOpportunity` | `idempotent`, `idempotency-token` | `Catalog`, `ClientToken` | `ClientToken` | `CreateOpportunityResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates an `Opportunity` record in Partner Central. Use this operation to create a potential business opportunity for submission to Amazon Web Services. |
| `CreateResourceSnapshot` | `POST /CreateResourceSnapshot` | `idempotent`, `idempotency-token` | `Catalog`, `ClientToken`, `EngagementIdentifier`, `ResourceIdentifier`, `ResourceSnapshotTemplateIdentifier`, `ResourceType` | `ClientToken` | `CreateResourceSnapshotResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | This action allows you to create an immutable snapshot of a specific resource, such as an opportunity, within the context of an engagement. The snapshot captures a subset of the resource's data based on the schema defined by the provided template. |
| `CreateResourceSnapshotJob` | `POST /CreateResourceSnapshotJob` | `idempotent`, `idempotency-token` | `Catalog`, `ClientToken`, `EngagementIdentifier`, `ResourceIdentifier`, `ResourceSnapshotTemplateIdentifier`, `ResourceType` | `ClientToken` | `CreateResourceSnapshotJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Use this action to create a job to generate a snapshot of the specified resource within an engagement. It initiates an asynchronous process to create a resource snapshot. |
| `DeleteResourceSnapshotJob` | `POST /DeleteResourceSnapshotJob` | `idempotent` | `Catalog`, `ResourceSnapshotJobIdentifier` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use this action to deletes a previously created resource snapshot job. The job must be in a stopped state before it can be deleted. |
| `DisassociateOpportunity` | `POST /DisassociateOpportunity` | - | `Catalog`, `OpportunityIdentifier`, `RelatedEntityIdentifier`, `RelatedEntityType` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Allows you to remove an existing association between an `Opportunity` and related entities, such as a Partner Solution, Amazon Web Services product, or an Amazon Web Services Marketplace offer. This operation is the counterpart to `AssociateOpportunity`, and... |
| `GetAwsOpportunitySummary` | `POST /GetAwsOpportunitySummary` | `readonly` | `Catalog`, `RelatedOpportunityIdentifier` | - | `GetAwsOpportunitySummaryResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a summary of an AWS Opportunity. This summary includes high-level details about the opportunity sourced from AWS, such as lifecycle information, customer details, and involvement type. |
| `GetEngagement` | `POST /GetEngagement` | `readonly` | `Catalog`, `Identifier` | - | `GetEngagementResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use this action to retrieve the engagement record for a given `EngagementIdentifier`. |
| `GetEngagementInvitation` | `POST /GetEngagementInvitation` | `readonly` | `Catalog`, `Identifier` | - | `GetEngagementInvitationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the details of an engagement invitation shared by AWS with a partner. The information includes aspects such as customer, project details, and lifecycle information. |
| `GetOpportunity` | `POST /GetOpportunity` | `readonly` | `Catalog`, `Identifier` | - | `GetOpportunityResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Fetches the `Opportunity` record from Partner Central by a given `Identifier`. Use the `ListOpportunities` action or the event notification (from Amazon EventBridge) to obtain this identifier. |
| `GetResourceSnapshot` | `POST /GetResourceSnapshot` | `readonly` | `Catalog`, `EngagementIdentifier`, `ResourceIdentifier`, `ResourceSnapshotTemplateIdentifier`, `ResourceType` | - | `GetResourceSnapshotResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use this action to retrieve a specific snapshot record. |
| `GetResourceSnapshotJob` | `POST /GetResourceSnapshotJob` | `readonly` | `Catalog`, `ResourceSnapshotJobIdentifier` | - | `GetResourceSnapshotJobResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use this action to retrieves information about a specific resource snapshot job. |
| `GetSellingSystemSettings` | `POST /GetSellingSystemSettings` | `readonly` | `Catalog` | - | `GetSellingSystemSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the currently set system settings, which include the IAM Role used for resource snapshot jobs. |
| `ListEngagementByAcceptingInvitationTasks` | `POST /ListEngagementByAcceptingInvitationTasks` | `readonly`, `paginated` | `Catalog` | - | `ListEngagementByAcceptingInvitationTasksResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all in-progress, completed, or failed StartEngagementByAcceptingInvitationTask tasks that were initiated by the caller's account. |
| `ListEngagementFromOpportunityTasks` | `POST /ListEngagementFromOpportunityTasks` | `readonly`, `paginated` | `Catalog` | - | `ListEngagementFromOpportunityTasksResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all in-progress, completed, or failed `EngagementFromOpportunity` tasks that were initiated by the caller's account. |
| `ListEngagementInvitations` | `POST /ListEngagementInvitations` | `readonly`, `paginated` | `Catalog`, `ParticipantType` | - | `ListEngagementInvitationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of engagement invitations sent to the partner. This allows partners to view all pending or past engagement invitations, helping them track opportunities shared by AWS. |
| `ListEngagementMembers` | `POST /ListEngagementMembers` | `readonly`, `paginated` | `Catalog`, `Identifier` | - | `ListEngagementMembersResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the details of member partners in an Engagement. This operation can only be invoked by members of the Engagement. |
| `ListEngagementResourceAssociations` | `POST /ListEngagementResourceAssociations` | `readonly`, `paginated` | `Catalog` | - | `ListEngagementResourceAssociationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the associations between resources and engagements where the caller is a member and has at least one snapshot in the engagement. |
| `ListEngagements` | `POST /ListEngagements` | `readonly`, `paginated` | `Catalog` | - | `ListEngagementsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This action allows users to retrieve a list of Engagement records from Partner Central. This action can be used to manage and track various engagements across different stages of the partner selling process. |
| `ListOpportunities` | `POST /ListOpportunities` | `readonly`, `paginated` | `Catalog` | - | `ListOpportunitiesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This request accepts a list of filters that retrieve opportunity subsets as well as sort options. This feature is available to partners from Partner Central using the `ListOpportunities` API action. |
| `ListOpportunityFromEngagementTasks` | `POST /ListOpportunityFromEngagementTasks` | `readonly`, `paginated` | `Catalog` | - | `ListOpportunityFromEngagementTasksResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all in-progress, completed, or failed opportunity creation tasks from engagements that were initiated by the caller's account. |
| `ListResourceSnapshotJobs` | `POST /ListResourceSnapshotJobs` | `readonly`, `paginated` | `Catalog` | - | `ListResourceSnapshotJobsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists resource snapshot jobs owned by the customer. This operation supports various filtering scenarios, including listing all jobs owned by the caller, jobs for a specific engagement, jobs with a specific status, or any combination of these filters. |
| `ListResourceSnapshots` | `POST /ListResourceSnapshots` | `readonly`, `paginated` | `Catalog`, `EngagementIdentifier` | - | `ListResourceSnapshotsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of resource view snapshots based on specified criteria. This operation supports various use cases, including: Fetching all snapshots associated with an engagement. |
| `ListSolutions` | `POST /ListSolutions` | `readonly`, `paginated` | `Catalog` | - | `ListSolutionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a list of Partner Solutions that the partner registered on Partner Central. This API is used to generate a list of solutions that an end user selects from for association with an opportunity. |
| `ListTagsForResource` | `POST /ListTagsForResource` | `readonly` | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Returns a list of tags for a resource. |
| `PutSellingSystemSettings` | `POST /PutSellingSystemSettings` | `idempotent` | `Catalog` | - | `PutSellingSystemSettingsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the currently set system settings, which include the IAM Role used for resource snapshot jobs. |
| `RejectEngagementInvitation` | `POST /RejectEngagementInvitation` | - | `Catalog`, `Identifier` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | This action rejects an `EngagementInvitation` that AWS shared. Rejecting an invitation indicates that the partner doesn't want to pursue the opportunity, and all related data will become inaccessible thereafter. |
| `StartEngagementByAcceptingInvitationTask` | `POST /StartEngagementByAcceptingInvitationTask` | `idempotency-token` | `Catalog`, `ClientToken`, `Identifier` | `ClientToken` | `StartEngagementByAcceptingInvitationTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | This action starts the engagement by accepting an `EngagementInvitation`. The task is asynchronous and involves the following steps: accepting the invitation, creating an opportunity in the partner’s account from the AWS opportunity, and copying details for... |
| `StartEngagementFromOpportunityTask` | `POST /StartEngagementFromOpportunityTask` | `idempotency-token` | `AwsSubmission`, `Catalog`, `ClientToken`, `Identifier` | `ClientToken` | `StartEngagementFromOpportunityTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Similar to `StartEngagementByAcceptingInvitationTask`, this action is asynchronous and performs multiple steps before completion. This action orchestrates a comprehensive workflow that combines multiple API operations into a single task to create and initiate... |
| `StartOpportunityFromEngagementTask` | `POST /StartOpportunityFromEngagementTask` | `idempotency-token` | `Catalog`, `ClientToken`, `ContextIdentifier`, `Identifier` | `ClientToken` | `StartOpportunityFromEngagementTaskResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | This action creates an opportunity from an existing engagement context. The task is asynchronous and orchestrates the process of converting engagement contextual information into a structured opportunity record within the partner's account. |
| `StartResourceSnapshotJob` | `POST /StartResourceSnapshotJob` | `idempotent` | `Catalog`, `ResourceSnapshotJobIdentifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Starts a resource snapshot job that has been previously created. |
| `StopResourceSnapshotJob` | `POST /StopResourceSnapshotJob` | `idempotent` | `Catalog`, `ResourceSnapshotJobIdentifier` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Stops a resource snapshot job. The job must be started prior to being stopped. |
| `SubmitOpportunity` | `POST /SubmitOpportunity` | - | `Catalog`, `Identifier`, `InvolvementType` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Use this action to submit an Opportunity that was previously created by partner for AWS review. After you perform this action, the Opportunity becomes non-editable until it is reviewed by AWS and has ` LifeCycle.ReviewStatus ` as either `Approved` or `Action... |
| `TagResource` | `POST /TagResource` | `idempotent` | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Assigns one or more tags (key-value pairs) to the specified resource. |
| `UntagResource` | `POST /UntagResource` | `idempotent` | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes a tag or tags from a resource. |
| `UpdateEngagementContext` | `POST /UpdateEngagementContext` | - | `Catalog`, `ContextIdentifier`, `EngagementIdentifier`, `EngagementLastModifiedAt`, `Payload`, `Type` | - | `UpdateEngagementContextResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the context information for an existing engagement with new or modified data. |
| `UpdateOpportunity` | `POST /UpdateOpportunity` | - | `Catalog`, `Identifier`, `LastModifiedDate` | - | `UpdateOpportunityResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the `Opportunity` record identified by a given `Identifier`. This operation allows you to modify the details of an existing opportunity to reflect the latest information and progress. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `Message`, `Reason` | This error occurs when you don't have permission to perform the requested action. |
| `InternalServerException` | `structure` | `Message` | This error occurs when the specified resource can’t be found or doesn't exist. |
| `ResourceNotFoundException` | `structure` | `Message` | This error occurs when the specified resource can't be found. |
| `ThrottlingException` | `structure` | `Message` | This error occurs when there are too many requests sent. |
| `ValidationException` | `structure` | `ErrorList`, `Message`, `Reason` | The input fails to satisfy the constraints specified by the service or business validation rules. |
| `ConflictException` | `structure` | `Message` | This error occurs when the request can’t be processed due to a conflict with the target resource's current state, which could result from updating or deleting the resource. |
| `ServiceQuotaExceededException` | `structure` | `Message` | This error occurs when the request would cause a service quota to be exceeded. |
| `AcceptEngagementInvitationRequest` | `structure` | `Catalog`, `Identifier` | - |
| `AssignOpportunityRequest` | `structure` | `Assignee`, `Catalog`, `Identifier` | - |
| `AssociateOpportunityRequest` | `structure` | `Catalog`, `OpportunityIdentifier`, `RelatedEntityIdentifier`, `RelatedEntityType` | - |
| `CreateEngagementRequest` | `structure` | `Catalog`, `ClientToken`, `Contexts`, `Description`, `Title` | - |
| `CreateEngagementResponse` | `structure` | `Arn`, `Id`, `ModifiedAt` | - |
| `CreateEngagementContextRequest` | `structure` | `Catalog`, `ClientToken`, `EngagementIdentifier`, `Payload`, `Type` | - |
| `CreateEngagementContextResponse` | `structure` | `ContextId`, `EngagementArn`, `EngagementId`, `EngagementLastModifiedAt` | - |
| `CreateEngagementInvitationRequest` | `structure` | `Catalog`, `ClientToken`, `EngagementIdentifier`, `Invitation` | - |
| `CreateEngagementInvitationResponse` | `structure` | `Arn`, `Id` | - |
| `CreateOpportunityRequest` | `structure` | `Catalog`, `ClientToken`, `Customer`, `LifeCycle`, `Marketing`, `NationalSecurity`, `OpportunityTeam`, `OpportunityType`, `Origin`, `PartnerOpportunityIdentifier`, `PrimaryNeedsFromAws`, `Project`, ... (+2) | - |
| `CreateOpportunityResponse` | `structure` | `Id`, `LastModifiedDate`, `PartnerOpportunityIdentifier` | - |
| `CreateResourceSnapshotRequest` | `structure` | `Catalog`, `ClientToken`, `EngagementIdentifier`, `ResourceIdentifier`, `ResourceSnapshotTemplateIdentifier`, `ResourceType` | - |
| `CreateResourceSnapshotResponse` | `structure` | `Arn`, `Revision` | - |
| `CreateResourceSnapshotJobRequest` | `structure` | `Catalog`, `ClientToken`, `EngagementIdentifier`, `ResourceIdentifier`, `ResourceSnapshotTemplateIdentifier`, `ResourceType`, `Tags` | - |
| `CreateResourceSnapshotJobResponse` | `structure` | `Arn`, `Id` | - |
| `DeleteResourceSnapshotJobRequest` | `structure` | `Catalog`, `ResourceSnapshotJobIdentifier` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
