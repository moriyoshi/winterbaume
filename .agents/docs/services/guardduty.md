# Amazon GuardDuty

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon GuardDuty is a continuous security monitoring service that analyzes and processes the following foundational data sources - VPC flow logs, Amazon Web Services CloudTrail management event logs, CloudTrail S3 data event logs, EKS audit logs, DNS logs, Amazon EBS volume data, runtime activity belonging to container workloads, such as Amazon EKS, Amazon ECS (including Amazon Web Services Fargate), and Amazon EC2 instances. It uses threat intelligence feeds, such as lists of malicious IPs and domains, and machine learning to identify unexpected, potentially unauthorized, and malicious activity within your Amazon Web Services environment. This can include issues like escalations of privileges, uses of exposed credentials, or communication with malicious IPs, domains, or presence of malware on your Amazon EC2 instances and container workloads. For example, GuardDuty can detect compromised EC2 instances and container workloads serving malware, or mining bitcoin. GuardDuty also monitors Amazon Web Services account access behavior for signs of compromise, such as unauthorized infrastructure deployments like EC2 instances deployed in a Region that has never been used, or unusual API calls like a password policy change to reduce password strength.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-guardduty/tests/scenario_test.rs`: create a detector, manage filters, seed/list findings, and clean up.
- Backported from `scenario_test.rs`: manage IP sets and threat intel sets inside a detector.
- Scenario insight from EC2: include mutable binding failover for Amazon GuardDuty where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon GuardDuty by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon GuardDuty by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: support threat-detection enablement, detectors, findings, filters, trusted/threat intelligence lists, organisation/member workflows, publishing destinations, and detector feature configuration.

## Service Identity and Protocol

- AWS model slug: `guardduty`
- AWS SDK for Rust slug: `guardduty`
- Model version: `2017-11-28`
- Model file: `vendor/api-models-aws/models/guardduty/service/2017-11-28/guardduty-2017-11-28.json`
- SDK ID: `GuardDuty`
- Endpoint prefix: `guardduty`
- ARN namespace: `guardduty`
- CloudFormation name: `GuardDuty`
- CloudTrail event source: `guardduty.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (20), `List` (15), `Update` (12), `Create` (10), `Delete` (10), `Describe` (3), `Disassociate` (3), `Accept` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptAdministratorInvitation`, `AcceptInvitation`, `CreateDetector`, `CreateFilter`, `CreateIPSet`, `CreateMalwareProtectionPlan`, `CreateMembers`, `CreatePublishingDestination`, `CreateSampleFindings`, `CreateThreatEntitySet`, `CreateThreatIntelSet`, `CreateTrustedEntitySet`, `DeleteDetector`, `DeleteFilter`, `DeleteIPSet`, `DeleteInvitations`, `DeleteMalwareProtectionPlan`, `DeleteMembers`, `DeletePublishingDestination`, `DeleteThreatEntitySet`, ... (+24).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeMalwareScans`, `DescribeOrganizationConfiguration`, `DescribePublishingDestination`, `GetAdministratorAccount`, `GetCoverageStatistics`, `GetDetector`, `GetFilter`, `GetFindings`, `GetFindingsStatistics`, `GetIPSet`, `GetInvitationsCount`, `GetMalwareProtectionPlan`, `GetMalwareScan`, `GetMalwareScanSettings`, `GetMasterAccount`, `GetMemberDetectors`, `GetMembers`, `GetOrganizationStatistics`, `GetRemainingFreeTrialDays`, `GetThreatEntitySet`, ... (+18).
- Pagination is modelled for 16 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 9 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `DescribeMalwareScans`, `GetMalwareScan`, `GetMalwareScanSettings`, `ListMalwareScans`, `SendObjectMalwareScan`, `StartMalwareScan`, `StartMonitoringMembers`, `StopMonitoringMembers`, `UpdateMalwareScanSettings`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 87 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `Lambda`, `EC2/VPC`, `ECS`, `EKS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/guardduty/latest/ug/what-is-guardduty.html
- https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_findings.html
- https://docs.aws.amazon.com/guardduty/latest/ug/guardduty_upload-lists.html

Research outcomes:
- GuardDuty detects threats by analysing AWS data sources and producing findings for suspicious activity across supported services.
- Enabling GuardDuty creates detector state for an account and Region.
- Findings have types, severities, resources, service metadata, and lifecycle state, and sample findings can be generated for testing.
- GuardDuty supports multi-account administration with administrator and member accounts.
- Threat intelligence sets and trusted IP lists customise detection. Lists are stored externally, such as in S3, and activated in GuardDuty.
- GuardDuty can export findings to S3 and send finding events through EventBridge for alerting.
- Updated active findings can be exported at a configured frequency.

Parity implications:
- Model detectors, findings, finding filters, members, invitations, administrators, threat intel sets, trusted IP sets, publishing destinations, and sample findings separately.
- Finding generation should be Region/account scoped and detector-state dependent.
- List activation should affect future detection semantics without deleting historical findings.

## Operation Groups

### Get

- Operations: `GetAdministratorAccount`, `GetCoverageStatistics`, `GetDetector`, `GetFilter`, `GetFindings`, `GetFindingsStatistics`, `GetIPSet`, `GetInvitationsCount`, `GetMalwareProtectionPlan`, `GetMalwareScan`, `GetMalwareScanSettings`, `GetMasterAccount`, `GetMemberDetectors`, `GetMembers`, `GetOrganizationStatistics`, `GetRemainingFreeTrialDays`, `GetThreatEntitySet`, `GetThreatIntelSet`, `GetTrustedEntitySet`, `GetUsageStatistics`
- Traits: `paginated` (1)
- Common required input members in this group: `AccountIds`, `DetectorId`, `FilterName`, `FindingIds`, `IpSetId`, `MalwareProtectionPlanId`, `ScanId`, `StatisticsType`, `ThreatEntitySetId`, `ThreatIntelSetId`, `TrustedEntitySetId`, `UsageCriteria`, `UsageStatisticType`

### List

- Operations: `ListCoverage`, `ListDetectors`, `ListFilters`, `ListFindings`, `ListIPSets`, `ListInvitations`, `ListMalwareProtectionPlans`, `ListMalwareScans`, `ListMembers`, `ListOrganizationAdminAccounts`, `ListPublishingDestinations`, `ListTagsForResource`, `ListThreatEntitySets`, `ListThreatIntelSets`, `ListTrustedEntitySets`
- Traits: `paginated` (13)
- Common required input members in this group: `DetectorId`, `ResourceArn`

### Update

- Operations: `UpdateDetector`, `UpdateFilter`, `UpdateFindingsFeedback`, `UpdateIPSet`, `UpdateMalwareProtectionPlan`, `UpdateMalwareScanSettings`, `UpdateMemberDetectors`, `UpdateOrganizationConfiguration`, `UpdatePublishingDestination`, `UpdateThreatEntitySet`, `UpdateThreatIntelSet`, `UpdateTrustedEntitySet`
- Common required input members in this group: `AccountIds`, `DestinationId`, `DetectorId`, `Feedback`, `FilterName`, `FindingIds`, `IpSetId`, `MalwareProtectionPlanId`, `ThreatEntitySetId`, `ThreatIntelSetId`, `TrustedEntitySetId`

### Create

- Operations: `CreateDetector`, `CreateFilter`, `CreateIPSet`, `CreateMalwareProtectionPlan`, `CreateMembers`, `CreatePublishingDestination`, `CreateSampleFindings`, `CreateThreatEntitySet`, `CreateThreatIntelSet`, `CreateTrustedEntitySet`
- Traits: `idempotency-token` (8)
- Common required input members in this group: `AccountDetails`, `Activate`, `DestinationProperties`, `DestinationType`, `DetectorId`, `Enable`, `FindingCriteria`, `Format`, `Location`, `Name`, `ProtectedResource`, `Role`

### Delete

- Operations: `DeleteDetector`, `DeleteFilter`, `DeleteIPSet`, `DeleteInvitations`, `DeleteMalwareProtectionPlan`, `DeleteMembers`, `DeletePublishingDestination`, `DeleteThreatEntitySet`, `DeleteThreatIntelSet`, `DeleteTrustedEntitySet`
- Common required input members in this group: `AccountIds`, `DestinationId`, `DetectorId`, `FilterName`, `IpSetId`, `MalwareProtectionPlanId`, `ThreatEntitySetId`, `ThreatIntelSetId`, `TrustedEntitySetId`

### Describe

- Operations: `DescribeMalwareScans`, `DescribeOrganizationConfiguration`, `DescribePublishingDestination`
- Traits: `paginated` (2)
- Common required input members in this group: `DestinationId`, `DetectorId`

### Disassociate

- Operations: `DisassociateFromAdministratorAccount`, `DisassociateFromMasterAccount`, `DisassociateMembers`
- Common required input members in this group: `AccountIds`, `DetectorId`

### Accept

- Operations: `AcceptAdministratorInvitation`, `AcceptInvitation`
- Common required input members in this group: `AdministratorId`, `DetectorId`, `InvitationId`, `MasterId`

### Start

- Operations: `StartMalwareScan`, `StartMonitoringMembers`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `AccountIds`, `DetectorId`, `ResourceArn`

### Archive

- Operations: `ArchiveFindings`
- Common required input members in this group: `DetectorId`, `FindingIds`

### Decline

- Operations: `DeclineInvitations`
- Common required input members in this group: `AccountIds`

### Disable

- Operations: `DisableOrganizationAdminAccount`
- Common required input members in this group: `AdminAccountId`

### Enable

- Operations: `EnableOrganizationAdminAccount`
- Common required input members in this group: `AdminAccountId`

### Invite

- Operations: `InviteMembers`
- Common required input members in this group: `AccountIds`, `DetectorId`

### Send

- Operations: `SendObjectMalwareScan`

### Stop

- Operations: `StopMonitoringMembers`
- Common required input members in this group: `AccountIds`, `DetectorId`

### Tag

- Operations: `TagResource`
- Common required input members in this group: `ResourceArn`, `Tags`

### Unarchive

- Operations: `UnarchiveFindings`
- Common required input members in this group: `DetectorId`, `FindingIds`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `ResourceArn`, `TagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptAdministratorInvitation` | `POST /detector/{DetectorId}/administrator` | - | `AdministratorId`, `DetectorId`, `InvitationId` | - | `AcceptAdministratorInvitationResponse` | `BadRequestException`, `InternalServerErrorException` | Accepts the invitation to be a member account and get monitored by a GuardDuty administrator account that sent the invitation. |
| `AcceptInvitation` | `POST /detector/{DetectorId}/master` | - | `DetectorId`, `InvitationId`, `MasterId` | - | `AcceptInvitationResponse` | `BadRequestException`, `InternalServerErrorException` | Accepts the invitation to be monitored by a GuardDuty administrator account. |
| `ArchiveFindings` | `POST /detector/{DetectorId}/findings/archive` | - | `DetectorId`, `FindingIds` | - | `ArchiveFindingsResponse` | `BadRequestException`, `InternalServerErrorException` | Archives GuardDuty findings that are specified by the list of finding IDs. Only the administrator account can archive findings. |
| `CreateDetector` | `POST /detector` | `idempotency-token` | `Enable` | `ClientToken` | `CreateDetectorResponse` | `BadRequestException`, `InternalServerErrorException` | Creates a single GuardDuty detector. A detector is a resource that represents the GuardDuty service. |
| `CreateFilter` | `POST /detector/{DetectorId}/filter` | `idempotency-token` | `DetectorId`, `FindingCriteria`, `Name` | `ClientToken` | `CreateFilterResponse` | `BadRequestException`, `InternalServerErrorException` | Creates a filter using the specified finding criteria. The maximum number of saved filters per Amazon Web Services account per Region is 100. |
| `CreateIPSet` | `POST /detector/{DetectorId}/ipset` | `idempotency-token` | `Activate`, `DetectorId`, `Format`, `Location`, `Name` | `ClientToken` | `CreateIPSetResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerErrorException` | Creates a new IPSet, which is called a trusted IP list in the console user interface. An IPSet is a list of IP addresses that are trusted for secure communication with Amazon Web Services infrastructure and applications. |
| `CreateMalwareProtectionPlan` | `POST /malware-protection-plan` | `idempotency-token` | `ProtectedResource`, `Role` | `ClientToken` | `CreateMalwareProtectionPlanResponse` | `AccessDeniedException`, `BadRequestException`, `ConflictException`, `InternalServerErrorException` | Creates a new Malware Protection plan for the protected resource. When you create a Malware Protection plan, the Amazon Web Services service terms for GuardDuty Malware Protection apply. |
| `CreateMembers` | `POST /detector/{DetectorId}/member` | - | `AccountDetails`, `DetectorId` | - | `CreateMembersResponse` | `BadRequestException`, `InternalServerErrorException` | Creates member accounts of the current Amazon Web Services account by specifying a list of Amazon Web Services account IDs. This step is a prerequisite for managing the associated member accounts either by invitation or through an organization. |
| `CreatePublishingDestination` | `POST /detector/{DetectorId}/publishingDestination` | `idempotency-token` | `DestinationProperties`, `DestinationType`, `DetectorId` | `ClientToken` | `CreatePublishingDestinationResponse` | `BadRequestException`, `InternalServerErrorException` | Creates a publishing destination where you can export your GuardDuty findings. Before you start exporting the findings, the destination resource must exist. |
| `CreateSampleFindings` | `POST /detector/{DetectorId}/findings/create` | - | `DetectorId` | - | `CreateSampleFindingsResponse` | `BadRequestException`, `InternalServerErrorException` | Generates sample findings of types specified by the list of finding types. If 'NULL' is specified for `findingTypes`, the API generates sample findings of all supported finding types. |
| `CreateThreatEntitySet` | `POST /detector/{DetectorId}/threatentityset` | `idempotency-token` | `Activate`, `DetectorId`, `Format`, `Location`, `Name` | `ClientToken` | `CreateThreatEntitySetResponse` | `BadRequestException`, `InternalServerErrorException` | Creates a new threat entity set. In a threat entity set, you can provide known malicious IP addresses and domains for your Amazon Web Services environment. |
| `CreateThreatIntelSet` | `POST /detector/{DetectorId}/threatintelset` | `idempotency-token` | `Activate`, `DetectorId`, `Format`, `Location`, `Name` | `ClientToken` | `CreateThreatIntelSetResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerErrorException` | Creates a new ThreatIntelSet. ThreatIntelSets consist of known malicious IP addresses. |
| `CreateTrustedEntitySet` | `POST /detector/{DetectorId}/trustedentityset` | `idempotency-token` | `Activate`, `DetectorId`, `Format`, `Location`, `Name` | `ClientToken` | `CreateTrustedEntitySetResponse` | `BadRequestException`, `InternalServerErrorException` | Creates a new trusted entity set. In the trusted entity set, you can provide IP addresses and domains that you believe are secure for communication in your Amazon Web Services environment. |
| `DeclineInvitations` | `POST /invitation/decline` | - | `AccountIds` | - | `DeclineInvitationsResponse` | `BadRequestException`, `InternalServerErrorException` | Declines invitations sent to the current member account by Amazon Web Services accounts specified by their account IDs. |
| `DeleteDetector` | `DELETE /detector/{DetectorId}` | - | `DetectorId` | - | `DeleteDetectorResponse` | `BadRequestException`, `InternalServerErrorException` | Deletes an Amazon GuardDuty detector that is specified by the detector ID. |
| `DeleteFilter` | `DELETE /detector/{DetectorId}/filter/{FilterName}` | - | `DetectorId`, `FilterName` | - | `DeleteFilterResponse` | `BadRequestException`, `InternalServerErrorException` | Deletes the filter specified by the filter name. |
| `DeleteIPSet` | `DELETE /detector/{DetectorId}/ipset/{IpSetId}` | - | `DetectorId`, `IpSetId` | - | `DeleteIPSetResponse` | `BadRequestException`, `InternalServerErrorException` | Deletes the IPSet specified by the `ipSetId`. IPSets are called trusted IP lists in the console user interface. |
| `DeleteInvitations` | `POST /invitation/delete` | - | `AccountIds` | - | `DeleteInvitationsResponse` | `BadRequestException`, `InternalServerErrorException` | Deletes invitations sent to the current member account by Amazon Web Services accounts specified by their account IDs. |
| `DeleteMalwareProtectionPlan` | `DELETE /malware-protection-plan/{MalwareProtectionPlanId}` | - | `MalwareProtectionPlanId` | - | `Unit` | `AccessDeniedException`, `BadRequestException`, `InternalServerErrorException`, `ResourceNotFoundException` | Deletes the Malware Protection plan ID associated with the Malware Protection plan resource. Use this API only when you no longer want to protect the resource associated with this Malware Protection plan ID. |
| `DeleteMembers` | `POST /detector/{DetectorId}/member/delete` | - | `AccountIds`, `DetectorId` | - | `DeleteMembersResponse` | `BadRequestException`, `InternalServerErrorException` | Deletes GuardDuty member accounts (to the current GuardDuty administrator account) specified by the account IDs. With `autoEnableOrganizationMembers` configuration for your organization set to `ALL`, you'll receive an error if you attempt to disable GuardDuty... |
| `DeletePublishingDestination` | `DELETE /detector/{DetectorId}/publishingDestination/{DestinationId}` | - | `DestinationId`, `DetectorId` | - | `DeletePublishingDestinationResponse` | `BadRequestException`, `InternalServerErrorException` | Deletes the publishing definition with the specified `destinationId`. |
| `DeleteThreatEntitySet` | `DELETE /detector/{DetectorId}/threatentityset/{ThreatEntitySetId}` | - | `DetectorId`, `ThreatEntitySetId` | - | `DeleteThreatEntitySetResponse` | `BadRequestException`, `InternalServerErrorException` | Deletes the threat entity set that is associated with the specified `threatEntitySetId`. |
| `DeleteThreatIntelSet` | `DELETE /detector/{DetectorId}/threatintelset/{ThreatIntelSetId}` | - | `DetectorId`, `ThreatIntelSetId` | - | `DeleteThreatIntelSetResponse` | `BadRequestException`, `InternalServerErrorException` | Deletes the ThreatIntelSet specified by the ThreatIntelSet ID. |
| `DeleteTrustedEntitySet` | `DELETE /detector/{DetectorId}/trustedentityset/{TrustedEntitySetId}` | - | `DetectorId`, `TrustedEntitySetId` | - | `DeleteTrustedEntitySetResponse` | `BadRequestException`, `InternalServerErrorException` | Deletes the trusted entity set that is associated with the specified `trustedEntitySetId`. |
| `DescribeMalwareScans` | `POST /detector/{DetectorId}/malware-scans` | `paginated` | `DetectorId` | - | `DescribeMalwareScansResponse` | `BadRequestException`, `InternalServerErrorException` | Returns a list of malware scans. Each member account can view the malware scans for their own accounts. |
| `DescribeOrganizationConfiguration` | `GET /detector/{DetectorId}/admin` | `paginated` | `DetectorId` | - | `DescribeOrganizationConfigurationResponse` | `BadRequestException`, `InternalServerErrorException` | Returns information about the account selected as the delegated administrator for GuardDuty. There might be regional differences because some data sources might not be available in all the Amazon Web Services Regions where GuardDuty is presently supported. |
| `DescribePublishingDestination` | `GET /detector/{DetectorId}/publishingDestination/{DestinationId}` | - | `DestinationId`, `DetectorId` | - | `DescribePublishingDestinationResponse` | `BadRequestException`, `InternalServerErrorException` | Returns information about the publishing destination specified by the provided `destinationId`. |
| `DisableOrganizationAdminAccount` | `POST /admin/disable` | - | `AdminAccountId` | - | `DisableOrganizationAdminAccountResponse` | `BadRequestException`, `InternalServerErrorException` | Removes the existing GuardDuty delegated administrator of the organization. Only the organization's management account can run this API operation. |
| `DisassociateFromAdministratorAccount` | `POST /detector/{DetectorId}/administrator/disassociate` | - | `DetectorId` | - | `DisassociateFromAdministratorAccountResponse` | `BadRequestException`, `InternalServerErrorException` | Disassociates the current GuardDuty member account from its administrator account. When you disassociate an invited member from a GuardDuty delegated administrator, the member account details obtained from the CreateMembers API, including the associated email... |
| `DisassociateFromMasterAccount` | `POST /detector/{DetectorId}/master/disassociate` | - | `DetectorId` | - | `DisassociateFromMasterAccountResponse` | `BadRequestException`, `InternalServerErrorException` | Disassociates the current GuardDuty member account from its administrator account. When you disassociate an invited member from a GuardDuty delegated administrator, the member account details obtained from the CreateMembers API, including the associated email... |
| `DisassociateMembers` | `POST /detector/{DetectorId}/member/disassociate` | - | `AccountIds`, `DetectorId` | - | `DisassociateMembersResponse` | `BadRequestException`, `InternalServerErrorException` | Disassociates GuardDuty member accounts (from the current administrator account) specified by the account IDs. When you disassociate an invited member from a GuardDuty delegated administrator, the member account details obtained from the CreateMembers API... |
| `EnableOrganizationAdminAccount` | `POST /admin/enable` | - | `AdminAccountId` | - | `EnableOrganizationAdminAccountResponse` | `BadRequestException`, `InternalServerErrorException` | Designates an Amazon Web Services account within the organization as your GuardDuty delegated administrator. Only the organization's management account can run this API operation. |
| `GetAdministratorAccount` | `GET /detector/{DetectorId}/administrator` | - | `DetectorId` | - | `GetAdministratorAccountResponse` | `BadRequestException`, `InternalServerErrorException` | Provides the details of the GuardDuty administrator account associated with the current GuardDuty member account. Based on the type of account that runs this API, the following list shows how the API behavior varies: When the GuardDuty administrator account... |
| `GetCoverageStatistics` | `POST /detector/{DetectorId}/coverage/statistics` | - | `DetectorId`, `StatisticsType` | - | `GetCoverageStatisticsResponse` | `BadRequestException`, `InternalServerErrorException` | Retrieves aggregated statistics for your account. If you are a GuardDuty administrator, you can retrieve the statistics for all the resources associated with the active member accounts in your organization who have enabled Runtime Monitoring and have the... |
| `GetDetector` | `GET /detector/{DetectorId}` | - | `DetectorId` | - | `GetDetectorResponse` | `BadRequestException`, `InternalServerErrorException` | Retrieves a GuardDuty detector specified by the detectorId. There might be regional differences because some data sources might not be available in all the Amazon Web Services Regions where GuardDuty is presently supported. |
| `GetFilter` | `GET /detector/{DetectorId}/filter/{FilterName}` | - | `DetectorId`, `FilterName` | - | `GetFilterResponse` | `BadRequestException`, `InternalServerErrorException` | Returns the details of the filter specified by the filter name. |
| `GetFindings` | `POST /detector/{DetectorId}/findings/get` | - | `DetectorId`, `FindingIds` | - | `GetFindingsResponse` | `BadRequestException`, `InternalServerErrorException` | Describes Amazon GuardDuty findings specified by finding IDs. |
| `GetFindingsStatistics` | `POST /detector/{DetectorId}/findings/statistics` | - | `DetectorId` | - | `GetFindingsStatisticsResponse` | `BadRequestException`, `InternalServerErrorException` | Lists GuardDuty findings statistics for the specified detector ID. You must provide either `findingStatisticTypes` or `groupBy` parameter, and not both. |
| `GetIPSet` | `GET /detector/{DetectorId}/ipset/{IpSetId}` | - | `DetectorId`, `IpSetId` | - | `GetIPSetResponse` | `BadRequestException`, `InternalServerErrorException` | Retrieves the IPSet specified by the `ipSetId`. |
| `GetInvitationsCount` | `GET /invitation/count` | - | - | - | `GetInvitationsCountResponse` | `BadRequestException`, `InternalServerErrorException` | Returns the count of all GuardDuty membership invitations that were sent to the current member account except the currently accepted invitation. |
| `GetMalwareProtectionPlan` | `GET /malware-protection-plan/{MalwareProtectionPlanId}` | - | `MalwareProtectionPlanId` | - | `GetMalwareProtectionPlanResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerErrorException`, `ResourceNotFoundException` | Retrieves the Malware Protection plan details associated with a Malware Protection plan ID. |
| `GetMalwareScan` | `GET /malware-scan/{ScanId}` | - | `ScanId` | - | `GetMalwareScanResponse` | `BadRequestException`, `InternalServerErrorException`, `ResourceNotFoundException` | Retrieves the detailed information for a specific malware scan. Each member account can view the malware scan details for their own account. |
| `GetMalwareScanSettings` | `GET /detector/{DetectorId}/malware-scan-settings` | - | `DetectorId` | - | `GetMalwareScanSettingsResponse` | `BadRequestException`, `InternalServerErrorException` | Returns the details of the malware scan settings. There might be regional differences because some data sources might not be available in all the Amazon Web Services Regions where GuardDuty is presently supported. |
| `GetMasterAccount` | `GET /detector/{DetectorId}/master` | - | `DetectorId` | - | `GetMasterAccountResponse` | `BadRequestException`, `InternalServerErrorException` | Provides the details for the GuardDuty administrator account associated with the current GuardDuty member account. |
| `GetMemberDetectors` | `POST /detector/{DetectorId}/member/detector/get` | - | `AccountIds`, `DetectorId` | - | `GetMemberDetectorsResponse` | `BadRequestException`, `InternalServerErrorException` | Describes which data sources are enabled for the member account's detector. There might be regional differences because some data sources might not be available in all the Amazon Web Services Regions where GuardDuty is presently supported. |
| `GetMembers` | `POST /detector/{DetectorId}/member/get` | - | `AccountIds`, `DetectorId` | - | `GetMembersResponse` | `BadRequestException`, `InternalServerErrorException` | Retrieves GuardDuty member accounts (of the current GuardDuty administrator account) specified by the account IDs. |
| `GetOrganizationStatistics` | `GET /organization/statistics` | - | - | - | `GetOrganizationStatisticsResponse` | `BadRequestException`, `InternalServerErrorException` | Retrieves how many active member accounts have each feature enabled within GuardDuty. Only a delegated GuardDuty administrator of an organization can run this API. |
| `GetRemainingFreeTrialDays` | `POST /detector/{DetectorId}/freeTrial/daysRemaining` | - | `AccountIds`, `DetectorId` | - | `GetRemainingFreeTrialDaysResponse` | `BadRequestException`, `InternalServerErrorException` | Provides the number of days left for each data source used in the free trial period. |
| `GetThreatEntitySet` | `GET /detector/{DetectorId}/threatentityset/{ThreatEntitySetId}` | - | `DetectorId`, `ThreatEntitySetId` | - | `GetThreatEntitySetResponse` | `BadRequestException`, `InternalServerErrorException` | Retrieves the threat entity set associated with the specified `threatEntitySetId`. |
| `GetThreatIntelSet` | `GET /detector/{DetectorId}/threatintelset/{ThreatIntelSetId}` | - | `DetectorId`, `ThreatIntelSetId` | - | `GetThreatIntelSetResponse` | `BadRequestException`, `InternalServerErrorException` | Retrieves the ThreatIntelSet that is specified by the ThreatIntelSet ID. |
| `GetTrustedEntitySet` | `GET /detector/{DetectorId}/trustedentityset/{TrustedEntitySetId}` | - | `DetectorId`, `TrustedEntitySetId` | - | `GetTrustedEntitySetResponse` | `BadRequestException`, `InternalServerErrorException` | Retrieves the trusted entity set associated with the specified `trustedEntitySetId`. |
| `GetUsageStatistics` | `POST /detector/{DetectorId}/usage/statistics` | `paginated` | `DetectorId`, `UsageCriteria`, `UsageStatisticType` | - | `GetUsageStatisticsResponse` | `BadRequestException`, `InternalServerErrorException` | Lists Amazon GuardDuty usage statistics over the last 30 days for the specified detector ID. For newly enabled detectors or data sources, the cost returned will include only the usage so far under 30 days. |
| `InviteMembers` | `POST /detector/{DetectorId}/member/invite` | - | `AccountIds`, `DetectorId` | - | `InviteMembersResponse` | `BadRequestException`, `InternalServerErrorException` | Invites Amazon Web Services accounts to become members of an organization administered by the Amazon Web Services account that invokes this API. If you are using Amazon Web Services Organizations to manage your GuardDuty environment, this step is not needed. |
| `ListCoverage` | `POST /detector/{DetectorId}/coverage` | `paginated` | `DetectorId` | - | `ListCoverageResponse` | `BadRequestException`, `InternalServerErrorException` | Lists coverage details for your GuardDuty account. If you're a GuardDuty administrator, you can retrieve all resources associated with the active member accounts in your organization. |
| `ListDetectors` | `GET /detector` | `paginated` | - | - | `ListDetectorsResponse` | `BadRequestException`, `InternalServerErrorException` | Lists detectorIds of all the existing Amazon GuardDuty detector resources. |
| `ListFilters` | `GET /detector/{DetectorId}/filter` | `paginated` | `DetectorId` | - | `ListFiltersResponse` | `BadRequestException`, `InternalServerErrorException` | Returns a paginated list of the current filters. |
| `ListFindings` | `POST /detector/{DetectorId}/findings` | `paginated` | `DetectorId` | - | `ListFindingsResponse` | `BadRequestException`, `InternalServerErrorException` | Lists GuardDuty findings for the specified detector ID. There might be regional differences because some flags might not be available in all the Regions where GuardDuty is currently supported. |
| `ListIPSets` | `GET /detector/{DetectorId}/ipset` | `paginated` | `DetectorId` | - | `ListIPSetsResponse` | `BadRequestException`, `InternalServerErrorException` | Lists the IPSets of the GuardDuty service specified by the detector ID. If you use this operation from a member account, the IPSets returned are the IPSets from the associated administrator account. |
| `ListInvitations` | `GET /invitation` | `paginated` | - | - | `ListInvitationsResponse` | `BadRequestException`, `InternalServerErrorException` | Lists all GuardDuty membership invitations that were sent to the current Amazon Web Services account. |
| `ListMalwareProtectionPlans` | `GET /malware-protection-plan` | - | - | - | `ListMalwareProtectionPlansResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerErrorException` | Lists the Malware Protection plan IDs associated with the protected resources in your Amazon Web Services account. |
| `ListMalwareScans` | `POST /malware-scan` | `paginated` | - | - | `ListMalwareScansResponse` | `BadRequestException`, `InternalServerErrorException` | Returns a list of malware scans. Each member account can view the malware scans for their own accounts. |
| `ListMembers` | `GET /detector/{DetectorId}/member` | `paginated` | `DetectorId` | - | `ListMembersResponse` | `BadRequestException`, `InternalServerErrorException` | Lists details about all member accounts for the current GuardDuty administrator account. |
| `ListOrganizationAdminAccounts` | `GET /admin` | `paginated` | - | - | `ListOrganizationAdminAccountsResponse` | `BadRequestException`, `InternalServerErrorException` | Lists the accounts designated as GuardDuty delegated administrators. Only the organization's management account can run this API operation. |
| `ListPublishingDestinations` | `GET /detector/{DetectorId}/publishingDestination` | `paginated` | `DetectorId` | - | `ListPublishingDestinationsResponse` | `BadRequestException`, `InternalServerErrorException` | Returns a list of publishing destinations associated with the specified `detectorId`. |
| `ListTagsForResource` | `GET /tags/{ResourceArn}` | - | `ResourceArn` | - | `ListTagsForResourceResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerErrorException` | Lists tags for a resource. Tagging is currently supported for detectors, finding filters, IP sets, threat intel sets, and publishing destination, with a limit of 50 tags per resource. |
| `ListThreatEntitySets` | `GET /detector/{DetectorId}/threatentityset` | `paginated` | `DetectorId` | - | `ListThreatEntitySetsResponse` | `BadRequestException`, `InternalServerErrorException` | Lists the threat entity sets associated with the specified GuardDuty detector ID. If you use this operation from a member account, the threat entity sets that are returned as a response, belong to the administrator account. |
| `ListThreatIntelSets` | `GET /detector/{DetectorId}/threatintelset` | `paginated` | `DetectorId` | - | `ListThreatIntelSetsResponse` | `BadRequestException`, `InternalServerErrorException` | Lists the ThreatIntelSets of the GuardDuty service specified by the detector ID. If you use this operation from a member account, the ThreatIntelSets associated with the administrator account are returned. |
| `ListTrustedEntitySets` | `GET /detector/{DetectorId}/trustedentityset` | `paginated` | `DetectorId` | - | `ListTrustedEntitySetsResponse` | `BadRequestException`, `InternalServerErrorException` | Lists the trusted entity sets associated with the specified GuardDuty detector ID. If you use this operation from a member account, the trusted entity sets that are returned as a response, belong to the administrator account. |
| `SendObjectMalwareScan` | `POST /object-malware-scan/send` | - | - | - | `SendObjectMalwareScanResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerErrorException` | Initiates a malware scan for a specific S3 object. This API allows you to perform on-demand malware scanning of individual objects in S3 buckets that have Malware Protection for S3 enabled. |
| `StartMalwareScan` | `POST /malware-scan/start` | `idempotency-token` | `ResourceArn` | `ClientToken` | `StartMalwareScanResponse` | `BadRequestException`, `ConflictException`, `InternalServerErrorException` | Initiates the malware scan. Invoking this API will automatically create the Service-linked role in the corresponding account if the resourceArn belongs to an EC2 instance. |
| `StartMonitoringMembers` | `POST /detector/{DetectorId}/member/start` | - | `AccountIds`, `DetectorId` | - | `StartMonitoringMembersResponse` | `BadRequestException`, `InternalServerErrorException` | Turns on GuardDuty monitoring of the specified member accounts. Use this operation to restart monitoring of accounts that you stopped monitoring with the StopMonitoringMembers operation. |
| `StopMonitoringMembers` | `POST /detector/{DetectorId}/member/stop` | - | `AccountIds`, `DetectorId` | - | `StopMonitoringMembersResponse` | `BadRequestException`, `InternalServerErrorException` | Stops GuardDuty monitoring for the specified member accounts. Use the `StartMonitoringMembers` operation to restart monitoring for those accounts. |
| `TagResource` | `POST /tags/{ResourceArn}` | - | `ResourceArn`, `Tags` | - | `TagResourceResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerErrorException` | Adds tags to a resource. |
| `UnarchiveFindings` | `POST /detector/{DetectorId}/findings/unarchive` | - | `DetectorId`, `FindingIds` | - | `UnarchiveFindingsResponse` | `BadRequestException`, `InternalServerErrorException` | Unarchives GuardDuty findings specified by the `findingIds`. |
| `UntagResource` | `DELETE /tags/{ResourceArn}` | - | `ResourceArn`, `TagKeys` | - | `UntagResourceResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerErrorException` | Removes tags from a resource. |
| `UpdateDetector` | `POST /detector/{DetectorId}` | - | `DetectorId` | - | `UpdateDetectorResponse` | `BadRequestException`, `InternalServerErrorException` | Updates the GuardDuty detector specified by the detector ID. Specifying both EKS Runtime Monitoring (`EKS_RUNTIME_MONITORING`) and Runtime Monitoring (`RUNTIME_MONITORING`) will cause an error. |
| `UpdateFilter` | `POST /detector/{DetectorId}/filter/{FilterName}` | - | `DetectorId`, `FilterName` | - | `UpdateFilterResponse` | `BadRequestException`, `InternalServerErrorException` | Updates the filter specified by the filter name. |
| `UpdateFindingsFeedback` | `POST /detector/{DetectorId}/findings/feedback` | - | `DetectorId`, `Feedback`, `FindingIds` | - | `UpdateFindingsFeedbackResponse` | `BadRequestException`, `InternalServerErrorException` | Marks the specified GuardDuty findings as useful or not useful. |
| `UpdateIPSet` | `POST /detector/{DetectorId}/ipset/{IpSetId}` | - | `DetectorId`, `IpSetId` | - | `UpdateIPSetResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerErrorException` | Updates the IPSet specified by the IPSet ID. |
| `UpdateMalwareProtectionPlan` | `PATCH /malware-protection-plan/{MalwareProtectionPlanId}` | - | `MalwareProtectionPlanId` | - | `Unit` | `AccessDeniedException`, `BadRequestException`, `InternalServerErrorException`, `ResourceNotFoundException` | Updates an existing Malware Protection plan resource. |
| `UpdateMalwareScanSettings` | `POST /detector/{DetectorId}/malware-scan-settings` | - | `DetectorId` | - | `UpdateMalwareScanSettingsResponse` | `BadRequestException`, `InternalServerErrorException` | Updates the malware scan settings. There might be regional differences because some data sources might not be available in all the Amazon Web Services Regions where GuardDuty is presently supported. |
| `UpdateMemberDetectors` | `POST /detector/{DetectorId}/member/detector/update` | - | `AccountIds`, `DetectorId` | - | `UpdateMemberDetectorsResponse` | `BadRequestException`, `InternalServerErrorException` | Contains information on member accounts to be updated. Specifying both EKS Runtime Monitoring (`EKS_RUNTIME_MONITORING`) and Runtime Monitoring (`RUNTIME_MONITORING`) will cause an error. |
| `UpdateOrganizationConfiguration` | `POST /detector/{DetectorId}/admin` | - | `DetectorId` | - | `UpdateOrganizationConfigurationResponse` | `BadRequestException`, `InternalServerErrorException` | Configures the delegated administrator account with the provided values. You must provide a value for either `autoEnableOrganizationMembers` or `autoEnable`, but not both. |
| `UpdatePublishingDestination` | `POST /detector/{DetectorId}/publishingDestination/{DestinationId}` | - | `DestinationId`, `DetectorId` | - | `UpdatePublishingDestinationResponse` | `BadRequestException`, `InternalServerErrorException` | Updates information about the publishing destination specified by the `destinationId`. |
| `UpdateThreatEntitySet` | `POST /detector/{DetectorId}/threatentityset/{ThreatEntitySetId}` | - | `DetectorId`, `ThreatEntitySetId` | - | `UpdateThreatEntitySetResponse` | `BadRequestException`, `InternalServerErrorException` | Updates the threat entity set associated with the specified `threatEntitySetId`. |
| `UpdateThreatIntelSet` | `POST /detector/{DetectorId}/threatintelset/{ThreatIntelSetId}` | - | `DetectorId`, `ThreatIntelSetId` | - | `UpdateThreatIntelSetResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerErrorException` | Updates the ThreatIntelSet specified by the ThreatIntelSet ID. |
| `UpdateTrustedEntitySet` | `POST /detector/{DetectorId}/trustedentityset/{TrustedEntitySetId}` | - | `DetectorId`, `TrustedEntitySetId` | - | `UpdateTrustedEntitySetResponse` | `BadRequestException`, `InternalServerErrorException` | Updates the trusted entity set associated with the specified `trustedEntitySetId`. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `BadRequestException` | `structure` | `Message`, `Type` | A bad request exception object. |
| `InternalServerErrorException` | `structure` | `Message`, `Type` | An internal server error exception object. |
| `AccessDeniedException` | `structure` | `Message`, `Type` | An access denied exception object. |
| `ResourceNotFoundException` | `structure` | `Message`, `Type` | The requested resource can't be found. |
| `ConflictException` | `structure` | `Message`, `Type` | A request conflict exception object. |
| `AcceptAdministratorInvitationRequest` | `structure` | `AdministratorId`, `DetectorId`, `InvitationId` | - |
| `AcceptAdministratorInvitationResponse` | `structure` | - | - |
| `AcceptInvitationRequest` | `structure` | `DetectorId`, `InvitationId`, `MasterId` | - |
| `AcceptInvitationResponse` | `structure` | - | - |
| `ArchiveFindingsRequest` | `structure` | `DetectorId`, `FindingIds` | - |
| `ArchiveFindingsResponse` | `structure` | - | - |
| `CreateDetectorRequest` | `structure` | `ClientToken`, `DataSources`, `Enable`, `Features`, `FindingPublishingFrequency`, `Tags` | - |
| `CreateDetectorResponse` | `structure` | `DetectorId`, `UnprocessedDataSources` | - |
| `CreateFilterRequest` | `structure` | `Action`, `ClientToken`, `Description`, `DetectorId`, `FindingCriteria`, `Name`, `Rank`, `Tags` | - |
| `CreateFilterResponse` | `structure` | `Name` | - |
| `CreateIPSetRequest` | `structure` | `Activate`, `ClientToken`, `DetectorId`, `ExpectedBucketOwner`, `Format`, `Location`, `Name`, `Tags` | - |
| `CreateIPSetResponse` | `structure` | `IpSetId` | - |
| `CreateMalwareProtectionPlanRequest` | `structure` | `Actions`, `ClientToken`, `ProtectedResource`, `Role`, `Tags` | - |
| `CreateMalwareProtectionPlanResponse` | `structure` | `MalwareProtectionPlanId` | - |
| `CreateMembersRequest` | `structure` | `AccountDetails`, `DetectorId` | - |
| `CreateMembersResponse` | `structure` | `UnprocessedAccounts` | - |
| `CreatePublishingDestinationRequest` | `structure` | `ClientToken`, `DestinationProperties`, `DestinationType`, `DetectorId`, `Tags` | - |
| `CreatePublishingDestinationResponse` | `structure` | `DestinationId` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
