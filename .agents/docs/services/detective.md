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
- Common required input members in this group: `GraphArn`

### Update

- Operations: `UpdateDatasourcePackages`, `UpdateInvestigationState`, `UpdateOrganizationConfiguration`
- Common required input members in this group: `GraphArn`

### Batch

- Operations: `BatchGetGraphMemberDatasources`, `BatchGetMembershipDatasources`
- Common required input members in this group: -

### Create

- Operations: `CreateGraph`, `CreateMembers`
- Common required input members in this group: -

### Delete

- Operations: `DeleteGraph`, `DeleteMembers`
- Common required input members in this group: `GraphArn`

### Get

- Operations: `GetInvestigation`, `GetMembers`
- Common required input members in this group: `GraphArn`

### Start

- Operations: `StartInvestigation`, `StartMonitoringMember`
- Common required input members in this group: `GraphArn`

### Accept

- Operations: `AcceptInvitation`
- Common required input members in this group: -

### Describe

- Operations: `DescribeOrganizationConfiguration`
- Common required input members in this group: -

### Disable

- Operations: `DisableOrganizationAdminAccount`
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateMembership`
- Common required input members in this group: -

### Enable

- Operations: `EnableOrganizationAdminAccount`
- Common required input members in this group: -

### Reject

- Operations: `RejectInvitation`
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
| `AcceptInvitation` | `PUT /invitation` | - | `GraphArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Accepts an invitation for the member account to contribute data to a behavior graph. This operation can only be called by an invited member account. The request provides the ARN of behavior graph. The member account ... |
| `BatchGetGraphMemberDatasources` | `POST /graph/datasources/get` | - | `GraphArn`, `AccountIds` | - | `BatchGetGraphMemberDatasourcesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets data source package information for the behavior graph. |
| `BatchGetMembershipDatasources` | `POST /membership/datasources/get` | - | `GraphArns` | - | `BatchGetMembershipDatasourcesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Gets information on the data source package history for an account. |
| `CreateGraph` | `POST /graph` | - | - | - | `CreateGraphResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException` | Creates a new behavior graph for the calling account, and sets that account as the administrator account. This operation is called by the account that is enabling Detective. The operation also enables Detective for t ... |
| `CreateMembers` | `POST /graph/members` | - | `GraphArn`, `Accounts` | - | `CreateMembersResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | CreateMembers is used to send invitations to accounts. For the organization behavior graph, the Detective administrator account uses CreateMembers to enable organization accounts as member accounts. For invited accou ... |
| `DeleteGraph` | `POST /graph/removal` | - | `GraphArn` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Disables the specified behavior graph and queues it to be deleted. This operation removes the behavior graph from each member account's list of behavior graphs. DeleteGraph can only be called by the administrator acc ... |
| `DeleteMembers` | `POST /graph/members/removal` | - | `GraphArn`, `AccountIds` | - | `DeleteMembersResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes the specified member accounts from the behavior graph. The removed accounts no longer contribute data to the behavior graph. This operation can only be called by the administrator account for the behavior gra ... |
| `DescribeOrganizationConfiguration` | `POST /orgs/describeOrganizationConfiguration` | - | `GraphArn` | - | `DescribeOrganizationConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | Returns information about the configuration for the organization behavior graph. Currently indicates whether to automatically enable new organization accounts as member accounts. Can only be called by the Detective a ... |
| `DisableOrganizationAdminAccount` | `POST /orgs/disableAdminAccount` | - | - | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | Removes the Detective administrator account in the current Region. Deletes the organization behavior graph. Can only be called by the organization management account. Removing the Detective administrator account does ... |
| `DisassociateMembership` | `POST /membership/removal` | - | `GraphArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes the member account from the specified behavior graph. This operation can only be called by an invited member account that has the ENABLED status. DisassociateMembership cannot be called by an organization acc ... |
| `EnableOrganizationAdminAccount` | `POST /orgs/enableAdminAccount` | - | `AccountId` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | Designates the Detective administrator account for the organization in the current Region. If the account does not have Detective enabled, then enables Detective for that account and creates a new behavior graph. Can ... |
| `GetInvestigation` | `POST /investigations/getInvestigation` | - | `GraphArn`, `InvestigationId` | - | `GetInvestigationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Detective investigations lets you investigate IAM users and IAM roles using indicators of compromise. An indicator of compromise (IOC) is an artifact observed in or on a network, system, or environment that can (with ... |
| `GetMembers` | `POST /graph/members/get` | - | `GraphArn`, `AccountIds` | - | `GetMembersResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns the membership details for specified member accounts for a behavior graph. |
| `ListDatasourcePackages` | `POST /graph/datasources/list` | `paginated` | `GraphArn` | - | `ListDatasourcePackagesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Lists data source packages in the behavior graph. |
| `ListGraphs` | `POST /graphs/list` | `paginated` | - | - | `ListGraphsResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Returns the list of behavior graphs that the calling account is an administrator account of. This operation can only be called by an administrator account. Because an account can currently only be the administrator o ... |
| `ListIndicators` | `POST /investigations/listIndicators` | - | `GraphArn`, `InvestigationId` | - | `ListIndicatorsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Gets the indicators from an investigation. You can use the information from the indicators to determine if an IAM user and/or IAM role is involved in an unusual activity that could indicate malicious behavior and its ... |
| `ListInvestigations` | `POST /investigations/listInvestigations` | - | `GraphArn` | - | `ListInvestigationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Detective investigations lets you investigate IAM users and IAM roles using indicators of compromise. An indicator of compromise (IOC) is an artifact observed in or on a network, system, or environment that can (with ... |
| `ListInvitations` | `POST /invitations/list` | `paginated` | - | - | `ListInvitationsResponse` | `AccessDeniedException`, `InternalServerException`, `ValidationException` | Retrieves the list of open and accepted behavior graph invitations for the member account. This operation can only be called by an invited member account. Open invitations are invitations that the member account has ... |
| `ListMembers` | `POST /graph/members/list` | `paginated` | `GraphArn` | - | `ListMembersResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Retrieves the list of member accounts for a behavior graph. For invited accounts, the results do not include member accounts that were removed from the behavior graph. For the organization behavior graph, the results ... |
| `ListOrganizationAdminAccounts` | `POST /orgs/adminAccountslist` | `paginated` | - | - | `ListOrganizationAdminAccountsResponse` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | Returns information about the Detective administrator account for an organization. Can only be called by the organization management account. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Returns the tag values that are assigned to a behavior graph. |
| `RejectInvitation` | `POST /invitation/removal` | - | `GraphArn` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Rejects an invitation to contribute the account data to a behavior graph. This operation must be called by an invited member account that has the INVITED status. RejectInvitation cannot be called by an organization a ... |
| `StartInvestigation` | `POST /investigations/startInvestigation` | - | `GraphArn`, `EntityArn`, `ScopeStartTime`, `ScopeEndTime` | - | `StartInvestigationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Detective investigations lets you investigate IAM users and IAM roles using indicators of compromise. An indicator of compromise (IOC) is an artifact observed in or on a network, system, or environment that can (with ... |
| `StartMonitoringMember` | `POST /graph/member/monitoringstate` | - | `GraphArn`, `AccountId` | - | `Unit` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Sends a request to enable data ingest for a member account that has a status of ACCEPTED_BUT_DISABLED . For valid member accounts, the status is updated as follows. If Detective enabled the member account, then the n ... |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Applies tag values to a behavior graph. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ValidationException` | Removes tags from a behavior graph. |
| `UpdateDatasourcePackages` | `POST /graph/datasources/update` | - | `GraphArn`, `DatasourcePackages` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ValidationException` | Starts a data source package for the Detective behavior graph. |
| `UpdateInvestigationState` | `POST /investigations/updateInvestigationState` | - | `GraphArn`, `InvestigationId`, `State` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `TooManyRequestsException`, `ValidationException` | Updates the state of an investigation. |
| `UpdateOrganizationConfiguration` | `POST /orgs/updateOrganizationConfiguration` | - | `GraphArn` | - | `Unit` | `AccessDeniedException`, `InternalServerException`, `TooManyRequestsException`, `ValidationException` | Updates the configuration for the Organizations integration in the current Region. Can only be called by the Detective administrator account for the organization. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `UntagResource` | - | `TagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | Message, ErrorCode, ErrorCodeReason, SubErrorCode, SubErrorCodeReason | The request issuer does not have permission to access this resource or perform this operation. |
| `ConflictException` | `structure` | Message | The request attempted an invalid action. |
| `InternalServerException` | `structure` | Message | The request was valid but failed because of a problem with the service. |
| `ResourceNotFoundException` | `structure` | Message | The request refers to a nonexistent resource. |
| `ServiceQuotaExceededException` | `structure` | Message, Resources | This request cannot be completed for one of the following reasons. This request cannot be completed if it would cause the number of member accounts in the b ... |
| `TooManyRequestsException` | `structure` | Message | The request cannot be completed because too many other requests are occurring at the same time. |
| `ValidationException` | `structure` | Message, ErrorCode, ErrorCodeReason | The request parameters are invalid. |
| `AcceptInvitationRequest` | `structure` | GraphArn | - |
| `BatchGetGraphMemberDatasourcesRequest` | `structure` | GraphArn, AccountIds | - |
| `BatchGetGraphMemberDatasourcesResponse` | `structure` | MemberDatasources, UnprocessedAccounts | - |
| `BatchGetMembershipDatasourcesRequest` | `structure` | GraphArns | - |
| `BatchGetMembershipDatasourcesResponse` | `structure` | MembershipDatasources, UnprocessedGraphs | - |
| `CreateGraphRequest` | `structure` | Tags | - |
| `CreateGraphResponse` | `structure` | GraphArn | - |
| `CreateMembersRequest` | `structure` | GraphArn, Message, DisableEmailNotification, Accounts | - |
| `CreateMembersResponse` | `structure` | Members, UnprocessedAccounts | - |
| `DeleteGraphRequest` | `structure` | GraphArn | - |
| `DeleteMembersRequest` | `structure` | GraphArn, AccountIds | - |
| `DeleteMembersResponse` | `structure` | AccountIds, UnprocessedAccounts | - |
| `DescribeOrganizationConfigurationRequest` | `structure` | GraphArn | - |
| `DescribeOrganizationConfigurationResponse` | `structure` | AutoEnable | - |
| `DisassociateMembershipRequest` | `structure` | GraphArn | - |
| `EnableOrganizationAdminAccountRequest` | `structure` | AccountId | - |
| `GetInvestigationRequest` | `structure` | GraphArn, InvestigationId | - |
| `GetInvestigationResponse` | `structure` | GraphArn, InvestigationId, EntityArn, EntityType, CreatedTime, ScopeStartTime, ScopeEndTime, Status, Severity, State | - |
| `GetMembersRequest` | `structure` | GraphArn, AccountIds | - |
| `GetMembersResponse` | `structure` | MemberDetails, UnprocessedAccounts | - |
| `ListDatasourcePackagesRequest` | `structure` | GraphArn, NextToken, MaxResults | - |
| `ListDatasourcePackagesResponse` | `structure` | DatasourcePackages, NextToken | - |
| `ListGraphsRequest` | `structure` | NextToken, MaxResults | - |
| `ListGraphsResponse` | `structure` | GraphList, NextToken | - |
| `ListIndicatorsRequest` | `structure` | GraphArn, InvestigationId, IndicatorType, NextToken, MaxResults | - |
| `ListIndicatorsResponse` | `structure` | GraphArn, InvestigationId, NextToken, Indicators | - |
| `ListInvestigationsRequest` | `structure` | GraphArn, NextToken, MaxResults, FilterCriteria, SortCriteria | - |
| `ListInvestigationsResponse` | `structure` | InvestigationDetails, NextToken | - |
| `ListInvitationsRequest` | `structure` | NextToken, MaxResults | - |
| `ListInvitationsResponse` | `structure` | Invitations, NextToken | - |
| `ListMembersRequest` | `structure` | GraphArn, NextToken, MaxResults | - |
| `ListMembersResponse` | `structure` | MemberDetails, NextToken | - |
| `ListOrganizationAdminAccountsRequest` | `structure` | NextToken, MaxResults | - |
| `DatasourcePackage` | `enum` | DETECTIVE_CORE, EKS_AUDIT, ASFF_SECURITYHUB_FINDING | - |
| `DatasourcePackageIngestState` | `enum` | STARTED, STOPPED, DISABLED | - |
| `EntityType` | `enum` | IAM_ROLE, IAM_USER | - |
| `ErrorCode` | `enum` | InvalidGraphArn, InvalidRequestBody, InternalError | - |
| `Field` | `enum` | SEVERITY, STATUS, CREATED_TIME | - |
| `IndicatorType` | `enum` | TTP_OBSERVED, IMPOSSIBLE_TRAVEL, FLAGGED_IP_ADDRESS, NEW_GEOLOCATION, NEW_ASO, NEW_USER_AGENT, RELATED_FINDING, RELATED_FINDING_GROUP | - |
| `InvitationType` | `enum` | INVITATION, ORGANIZATION | - |
| `MemberDisabledReason` | `enum` | VOLUME_TOO_HIGH, VOLUME_UNKNOWN | - |
| `MemberStatus` | `enum` | INVITED, VERIFICATION_IN_PROGRESS, VERIFICATION_FAILED, ENABLED, ACCEPTED_BUT_DISABLED | - |
| `Reason` | `enum` | AWS_THREAT_INTELLIGENCE | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
