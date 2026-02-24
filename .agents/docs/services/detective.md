# Amazon Detective

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Detective uses machine learning and purpose-built visualizations to help you to analyze and investigate security issues across your Amazon Web Services (Amazon Web Services) workloads. Detective automatically extracts time-based events such as login attempts, API calls, and network traffic from CloudTrail and Amazon Virtual Private Cloud (Amazon VPC) flow logs. It also extracts findings detected by Amazon GuardDuty. The Detective API primarily supports the creation and management of behavior graphs. A behavior graph contains the extracted data from a set of member accounts, and is created and managed by an administrator account.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Detective where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon Detective by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon Detective by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon Detective workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `List`, `Update`, `Batch`, `Create`, `Delete` operation families, including `ListDatasourcePackages`, `ListGraphs`, `ListIndicators`, `ListInvestigations`, `UpdateDatasourcePackages`, `UpdateInvestigationState`.

## Service Identity and Protocol

- AWS model slug: `detective`
- AWS SDK for Rust slug: `detective`
- Model version: `2018-10-26`
- Model file: `vendor/api-models-aws/models/detective/service/2018-10-26/detective-2018-10-26.json`
- SDK ID: `Detective`
- Endpoint prefix: `api.detective`
- ARN namespace: `detective`
- CloudFormation name: `Detective`
- CloudTrail event source: `detective.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (8), `Update` (3), `Batch` (2), `Create` (2), `Delete` (2), `Get` (2), `Start` (2), `Accept` (1).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptInvitation`, `BatchGetGraphMemberDatasources`, `BatchGetMembershipDatasources`, `CreateGraph`, `CreateMembers`, `DeleteGraph`, `DeleteMembers`, `DisableOrganizationAdminAccount`, `DisassociateMembership`, `EnableOrganizationAdminAccount`, `RejectInvitation`, `StartInvestigation`, `StartMonitoringMember`, `TagResource`, `UntagResource`, `UpdateDatasourcePackages`, `UpdateInvestigationState`, `UpdateOrganizationConfiguration`.
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeOrganizationConfiguration`, `GetInvestigation`, `GetMembers`, `ListDatasourcePackages`, `ListGraphs`, `ListIndicators`, `ListInvestigations`, `ListInvitations`, `ListMembers`, `ListOrganizationAdminAccounts`, `ListTagsForResource`.
- Pagination is modelled for 5 operations; token stability and page boundaries are observable API behaviour.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `StartInvestigation`, `StartMonitoringMember`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 29 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `EC2/VPC`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/detective/latest/userguide/what-is-detective.html
- https://docs.aws.amazon.com/detective/latest/userguide/behavior-graph-data-about.html
- https://docs.aws.amazon.com/detective/latest/userguide/accounts-effects.html

Research outcomes:
- Detective builds behaviour graphs that aggregate account activity, entities, findings, and evidence for security investigations.
- Behaviour graphs ingest data from AWS accounts and correlate entities such as accounts, roles, IP addresses, and resources.
- Detective investigations support triage, scoping, and root-cause analysis for findings such as GuardDuty findings.
- Administrator and member accounts participate in behaviour graph membership.
- Removing a member, disabling Detective, leaving an organisation, suspending an account, or closing an account affects data ingestion and graph membership.
- Detective data retention and finding archival considerations are separate from original GuardDuty or Security Hub data.

Parity implications:
- Model behaviour graphs, administrator/member accounts, invitations, entities, findings, investigations, evidence, and ingestion state separately.
- Account membership actions should affect future ingestion without rewriting historical graph data unless documented.
- Investigation APIs should operate on graph-correlated entities rather than raw event logs only.

## Operation Groups

### List

- Operations: `ListDatasourcePackages`, `ListGraphs`, `ListIndicators`, `ListInvestigations`, `ListInvitations`, `ListMembers`, `ListOrganizationAdminAccounts`, `ListTagsForResource`
- Traits: `paginated` (5)
- Common required input members in this group: `GraphArn`, `InvestigationId`, `ResourceArn`

### Update

- Operations: `UpdateDatasourcePackages`, `UpdateInvestigationState`, `UpdateOrganizationConfiguration`
- Common required input members in this group: `DatasourcePackages`, `GraphArn`, `InvestigationId`, `State`

### Batch

- Operations: `BatchGetGraphMemberDatasources`, `BatchGetMembershipDatasources`
- Common required input members in this group: `AccountIds`, `GraphArn`, `GraphArns`

### Create

- Operations: `CreateGraph`, `CreateMembers`
- Common required input members in this group: `Accounts`, `GraphArn`

### Delete

- Operations: `DeleteGraph`, `DeleteMembers`
- Common required input members in this group: `AccountIds`, `GraphArn`

### Get

- Operations: `GetInvestigation`, `GetMembers`
- Common required input members in this group: `AccountIds`, `GraphArn`, `InvestigationId`

### Start

- Operations: `StartInvestigation`, `StartMonitoringMember`
- Common required input members in this group: `AccountId`, `EntityArn`, `GraphArn`, `ScopeEndTime`, `ScopeStartTime`

### Accept

- Operations: `AcceptInvitation`
- Common required input members in this group: `GraphArn`

### Describe

- Operations: `DescribeOrganizationConfiguration`
- Common required input members in this group: `GraphArn`

### Disable

- Operations: `DisableOrganizationAdminAccount`

### Disassociate

- Operations: `DisassociateMembership`
- Common required input members in this group: `GraphArn`

### Enable

- Operations: `EnableOrganizationAdminAccount`
- Common required input members in this group: `AccountId`

### Reject

- Operations: `RejectInvitation`
- Common required input members in this group: `GraphArn`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptInvitation` | `PUT /invitation` | - | `GraphArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Accepts an invitation for the member account to contribute data to a behavior graph. This operation can only be called by an invited member account. |
| `BatchGetGraphMemberDatasources` | `POST /graph/datasources/get` | - | `AccountIds`, `GraphArn` | - | `BatchGetGraphMemberDatasourcesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets data source package information for the behavior graph. |
| `BatchGetMembershipDatasources` | `POST /membership/datasources/get` | - | `GraphArns` | - | `BatchGetMembershipDatasourcesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets information on the data source package history for an account. |
| `CreateGraph` | `POST /graph` | - | - | - | `CreateGraphResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException` | Creates a new behavior graph for the calling account, and sets that account as the administrator account. This operation is called by the account that is enabling Detective. |
| `CreateMembers` | `POST /graph/members` | - | `Accounts`, `GraphArn` | - | `CreateMembersResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | `CreateMembers` is used to send invitations to accounts. For the organization behavior graph, the Detective administrator account uses `CreateMembers` to enable organization accounts as member accounts. |
| `DeleteGraph` | `POST /graph/removal` | - | `GraphArn` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Disables the specified behavior graph and queues it to be deleted. This operation removes the behavior graph from each member account's list of behavior graphs. |
| `DeleteMembers` | `POST /graph/members/removal` | - | `AccountIds`, `GraphArn` | - | `DeleteMembersResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes the specified member accounts from the behavior graph. The removed accounts no longer contribute data to the behavior graph. |
| `DescribeOrganizationConfiguration` | `POST /orgs/describeOrganizationConfiguration` | - | `GraphArn` | - | `DescribeOrganizationConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | Returns information about the configuration for the organization behavior graph. Currently indicates whether to automatically enable new organization accounts as member accounts. |
| `DisableOrganizationAdminAccount` | `POST /orgs/disableAdminAccount` | - | - | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | Removes the Detective administrator account in the current Region. Deletes the organization behavior graph. |
| `DisassociateMembership` | `POST /membership/removal` | - | `GraphArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes the member account from the specified behavior graph. This operation can only be called by an invited member account that has the `ENABLED` status. |
| `EnableOrganizationAdminAccount` | `POST /orgs/enableAdminAccount` | - | `AccountId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | Designates the Detective administrator account for the organization in the current Region. If the account does not have Detective enabled, then enables Detective for that account and creates a new behavior graph. |
| `GetInvestigation` | `POST /investigations/getInvestigation` | - | `GraphArn`, `InvestigationId` | - | `GetInvestigationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Detective investigations lets you investigate IAM users and IAM roles using indicators of compromise. An indicator of compromise (IOC) is an artifact observed in or on a network, system, or environment that can (with a high level of confidence) identify... |
| `GetMembers` | `POST /graph/members/get` | - | `AccountIds`, `GraphArn` | - | `GetMembersResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns the membership details for specified member accounts for a behavior graph. |
| `ListDatasourcePackages` | `POST /graph/datasources/list` | `paginated` | `GraphArn` | - | `ListDatasourcePackagesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists data source packages in the behavior graph. |
| `ListGraphs` | `POST /graphs/list` | `paginated` | - | - | `ListGraphsResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Returns the list of behavior graphs that the calling account is an administrator account of. This operation can only be called by an administrator account. |
| `ListIndicators` | `POST /investigations/listIndicators` | - | `GraphArn`, `InvestigationId` | - | `ListIndicatorsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Gets the indicators from an investigation. You can use the information from the indicators to determine if an IAM user and/or IAM role is involved in an unusual activity that could indicate malicious behavior and its impact. |
| `ListInvestigations` | `POST /investigations/listInvestigations` | - | `GraphArn` | - | `ListInvestigationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Detective investigations lets you investigate IAM users and IAM roles using indicators of compromise. An indicator of compromise (IOC) is an artifact observed in or on a network, system, or environment that can (with a high level of confidence) identify... |
| `ListInvitations` | `POST /invitations/list` | `paginated` | - | - | `ListInvitationsResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Retrieves the list of open and accepted behavior graph invitations for the member account. This operation can only be called by an invited member account. |
| `ListMembers` | `POST /graph/members/list` | `paginated` | `GraphArn` | - | `ListMembersResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Retrieves the list of member accounts for a behavior graph. For invited accounts, the results do not include member accounts that were removed from the behavior graph. |
| `ListOrganizationAdminAccounts` | `POST /orgs/adminAccountslist` | `paginated` | - | - | `ListOrganizationAdminAccountsResponse` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | Returns information about the Detective administrator account for an organization. Can only be called by the organization management account. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns the tag values that are assigned to a behavior graph. |
| `RejectInvitation` | `POST /invitation/removal` | - | `GraphArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Rejects an invitation to contribute the account data to a behavior graph. This operation must be called by an invited member account that has the `INVITED` status. |
| `StartInvestigation` | `POST /investigations/startInvestigation` | - | `EntityArn`, `GraphArn`, `ScopeEndTime`, `ScopeStartTime` | - | `StartInvestigationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Detective investigations lets you investigate IAM users and IAM roles using indicators of compromise. An indicator of compromise (IOC) is an artifact observed in or on a network, system, or environment that can (with a high level of confidence) identify... |
| `StartMonitoringMember` | `POST /graph/member/monitoringstate` | - | `AccountId`, `GraphArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Sends a request to enable data ingest for a member account that has a status of `ACCEPTED_BUT_DISABLED`. For valid member accounts, the status is updated as follows. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Applies tag values to a behavior graph. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes tags from a behavior graph. |
| `UpdateDatasourcePackages` | `POST /graph/datasources/update` | - | `DatasourcePackages`, `GraphArn` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Starts a data source package for the Detective behavior graph. |
| `UpdateInvestigationState` | `POST /investigations/updateInvestigationState` | - | `GraphArn`, `InvestigationId`, `State` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Updates the state of an investigation. |
| `UpdateOrganizationConfiguration` | `POST /orgs/updateOrganizationConfiguration` | - | `GraphArn` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | Updates the configuration for the Organizations integration in the current Region. Can only be called by the Detective administrator account for the organization. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `ErrorCode`, `ErrorCodeReason`, `Message`, `SubErrorCode`, `SubErrorCodeReason` | The request issuer does not have permission to access this resource or perform this operation. |
| `InternalServerException` | `structure` | `Message` | The request was valid but failed because of a problem with the service. |
| `ValidationException` | `structure` | `ErrorCode`, `ErrorCodeReason`, `Message` | The request parameters are invalid. |
| `ResourceNotFoundException` | `structure` | `Message` | The request refers to a nonexistent resource. |
| `TooManyRequestsException` | `structure` | `Message` | The request cannot be completed because too many other requests are occurring at the same time. |
| `ConflictException` | `structure` | `Message` | The request attempted an invalid action. |
| `ServiceQuotaExceededException` | `structure` | `Message`, `Resources` | This request cannot be completed for one of the following reasons. |
| `AcceptInvitationRequest` | `structure` | `GraphArn` | - |
| `BatchGetGraphMemberDatasourcesRequest` | `structure` | `AccountIds`, `GraphArn` | - |
| `BatchGetGraphMemberDatasourcesResponse` | `structure` | `MemberDatasources`, `UnprocessedAccounts` | - |
| `BatchGetMembershipDatasourcesRequest` | `structure` | `GraphArns` | - |
| `BatchGetMembershipDatasourcesResponse` | `structure` | `MembershipDatasources`, `UnprocessedGraphs` | - |
| `CreateGraphRequest` | `structure` | `Tags` | - |
| `CreateGraphResponse` | `structure` | `GraphArn` | - |
| `CreateMembersRequest` | `structure` | `Accounts`, `DisableEmailNotification`, `GraphArn`, `Message` | - |
| `CreateMembersResponse` | `structure` | `Members`, `UnprocessedAccounts` | - |
| `DeleteGraphRequest` | `structure` | `GraphArn` | - |
| `DeleteMembersRequest` | `structure` | `AccountIds`, `GraphArn` | - |
| `DeleteMembersResponse` | `structure` | `AccountIds`, `UnprocessedAccounts` | - |
| `DescribeOrganizationConfigurationRequest` | `structure` | `GraphArn` | - |
| `DescribeOrganizationConfigurationResponse` | `structure` | `AutoEnable` | - |
| `DisassociateMembershipRequest` | `structure` | `GraphArn` | - |
| `EnableOrganizationAdminAccountRequest` | `structure` | `AccountId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
