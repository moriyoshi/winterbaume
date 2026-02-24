# Amazon Macie 2

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Macie

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for Amazon Macie 2 where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Amazon Macie 2 by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Amazon Macie 2 by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: represent documented Amazon Macie 2 workflows in the local mock. Include service-managed state, documented errors, pagination, and asynchronous job state where the model exposes them.
- From the operation surface: model lifecycle workflows that provision, inspect, update, and clean up service resources across the `Get`, `List`, `Update`, `Create`, `Delete` operation families, including `GetAdministratorAccount`, `GetAllowList`, `GetAutomatedDiscoveryConfiguration`, `GetBucketStatistics`, `ListAllowLists`, `ListAutomatedDiscoveryAccounts`.

## Service Identity and Protocol

- AWS model slug: `macie2`
- AWS SDK for Rust slug: `macie2`
- Model version: `2020-01-01`
- Model file: `vendor/api-models-aws/models/macie2/service/2020-01-01/macie2-2020-01-01.json`
- SDK ID: `Macie2`
- Endpoint prefix: `macie2`
- ARN namespace: `macie2`
- CloudFormation name: `Macie2`
- CloudTrail event source: `macie2.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `Get` (22), `List` (15), `Update` (12), `Create` (7), `Delete` (5), `Describe` (3), `Disassociate` (3), `Batch` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AcceptInvitation`, `BatchGetCustomDataIdentifiers`, `BatchUpdateAutomatedDiscoveryAccounts`, `CreateAllowList`, `CreateClassificationJob`, `CreateCustomDataIdentifier`, `CreateFindingsFilter`, `CreateInvitations`, `CreateMember`, `CreateSampleFindings`, `DeleteAllowList`, `DeleteCustomDataIdentifier`, `DeleteFindingsFilter`, `DeleteInvitations`, `DeleteMember`, `DisableMacie`, `DisableOrganizationAdminAccount`, `DisassociateFromAdministratorAccount`, `DisassociateFromMasterAccount`, `DisassociateMember`, ... (+18).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeBuckets`, `DescribeClassificationJob`, `DescribeOrganizationConfiguration`, `GetAdministratorAccount`, `GetAllowList`, `GetAutomatedDiscoveryConfiguration`, `GetBucketStatistics`, `GetClassificationExportConfiguration`, `GetClassificationScope`, `GetCustomDataIdentifier`, `GetFindingStatistics`, `GetFindings`, `GetFindingsFilter`, `GetFindingsPublicationConfiguration`, `GetInvitationsCount`, `GetMacieSession`, `GetMasterAccount`, `GetMember`, `GetResourceProfile`, `GetRevealConfiguration`, ... (+21).
- Pagination is modelled for 17 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 8 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CreateClassificationJob`, `DescribeClassificationJob`, `GetClassificationExportConfiguration`, `ListClassificationJobs`, `PutClassificationExportConfiguration`, `UpdateClassificationJob`.
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 77 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `CloudWatch`, `EventBridge`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/macie/latest/user/what-is-macie.html
- https://docs.aws.amazon.com/macie/latest/user/discovery-jobs.html
- https://docs.aws.amazon.com/macie/latest/user/findings-types.html

Research outcomes:
- Macie discovers and protects sensitive data in Amazon S3 using automated discovery, classification jobs, and policy analysis.
- Sensitive data discovery jobs inspect selected S3 buckets using managed data identifiers, custom identifiers, allow lists, sampling, and scheduling.
- Macie generates policy findings for S3 bucket security and sensitive data findings for detected credentials, financial data, personal information, and related categories.
- Automated sensitive data discovery provides bucket-level coverage and scoring across S3.
- Findings can be filtered by common fields, affected resource fields, policy fields, and sensitive data fields.
- Macie is enabled per account/Region and supports organisation administration.

Parity implications:
- Model account enablement, member accounts, classification jobs, job scope, managed/custom identifiers, allow lists, findings, buckets, and automated discovery state separately.
- Job execution should be asynchronous and should create findings based on selected bucket/object scope.
- Sensitive-data findings and policy findings should have distinct schemas and lifecycle.

## Operation Groups

### Get

- Operations: `GetAdministratorAccount`, `GetAllowList`, `GetAutomatedDiscoveryConfiguration`, `GetBucketStatistics`, `GetClassificationExportConfiguration`, `GetClassificationScope`, `GetCustomDataIdentifier`, `GetFindingStatistics`, `GetFindings`, `GetFindingsFilter`, `GetFindingsPublicationConfiguration`, `GetInvitationsCount`, `GetMacieSession`, `GetMasterAccount`, `GetMember`, `GetResourceProfile`, `GetRevealConfiguration`, `GetSensitiveDataOccurrences`, `GetSensitiveDataOccurrencesAvailability`, `GetSensitivityInspectionTemplate`, `GetUsageStatistics`, `GetUsageTotals`
- Traits: `paginated` (1)
- Common required input members in this group: `findingId`, `findingIds`, `groupBy`, `id`, `resourceArn`

### List

- Operations: `ListAllowLists`, `ListAutomatedDiscoveryAccounts`, `ListClassificationJobs`, `ListClassificationScopes`, `ListCustomDataIdentifiers`, `ListFindings`, `ListFindingsFilters`, `ListInvitations`, `ListManagedDataIdentifiers`, `ListMembers`, `ListOrganizationAdminAccounts`, `ListResourceProfileArtifacts`, `ListResourceProfileDetections`, `ListSensitivityInspectionTemplates`, `ListTagsForResource`
- Traits: `paginated` (14)
- Common required input members in this group: `resourceArn`

### Update

- Operations: `UpdateAllowList`, `UpdateAutomatedDiscoveryConfiguration`, `UpdateClassificationJob`, `UpdateClassificationScope`, `UpdateFindingsFilter`, `UpdateMacieSession`, `UpdateMemberSession`, `UpdateOrganizationConfiguration`, `UpdateResourceProfile`, `UpdateResourceProfileDetections`, `UpdateRevealConfiguration`, `UpdateSensitivityInspectionTemplate`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `autoEnable`, `configuration`, `criteria`, `id`, `jobId`, `jobStatus`, `name`, `resourceArn`, `status`

### Create

- Operations: `CreateAllowList`, `CreateClassificationJob`, `CreateCustomDataIdentifier`, `CreateFindingsFilter`, `CreateInvitations`, `CreateMember`, `CreateSampleFindings`
- Traits: `idempotency-token` (4)
- Common required input members in this group: `account`, `accountIds`, `action`, `clientToken`, `criteria`, `findingCriteria`, `jobType`, `name`, `regex`, `s3JobDefinition`

### Delete

- Operations: `DeleteAllowList`, `DeleteCustomDataIdentifier`, `DeleteFindingsFilter`, `DeleteInvitations`, `DeleteMember`
- Common required input members in this group: `accountIds`, `id`

### Describe

- Operations: `DescribeBuckets`, `DescribeClassificationJob`, `DescribeOrganizationConfiguration`
- Traits: `paginated` (1)
- Common required input members in this group: `jobId`

### Disassociate

- Operations: `DisassociateFromAdministratorAccount`, `DisassociateFromMasterAccount`, `DisassociateMember`
- Common required input members in this group: `id`

### Batch

- Operations: `BatchGetCustomDataIdentifiers`, `BatchUpdateAutomatedDiscoveryAccounts`

### Disable

- Operations: `DisableMacie`, `DisableOrganizationAdminAccount`
- Common required input members in this group: `adminAccountId`

### Enable

- Operations: `EnableMacie`, `EnableOrganizationAdminAccount`
- Traits: `idempotency-token` (2)
- Common required input members in this group: `adminAccountId`

### Put

- Operations: `PutClassificationExportConfiguration`, `PutFindingsPublicationConfiguration`
- Traits: `idempotency-token` (1)
- Common required input members in this group: `configuration`

### Accept

- Operations: `AcceptInvitation`
- Common required input members in this group: `invitationId`

### Decline

- Operations: `DeclineInvitations`
- Common required input members in this group: `accountIds`

### Search

- Operations: `SearchResources`
- Traits: `paginated` (1)

### Tag

- Operations: `TagResource`
- Common required input members in this group: `resourceArn`, `tags`

### Test

- Operations: `TestCustomDataIdentifier`
- Common required input members in this group: `regex`, `sampleText`

### Untag

- Operations: `UntagResource`
- Common required input members in this group: `resourceArn`, `tagKeys`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AcceptInvitation` | `POST /invitations/accept` | - | `invitationId` | - | `AcceptInvitationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Accepts an Amazon Macie membership invitation that was received from a specific account. |
| `BatchGetCustomDataIdentifiers` | `POST /custom-data-identifiers/get` | - | - | - | `BatchGetCustomDataIdentifiersResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves information about one or more custom data identifiers. |
| `BatchUpdateAutomatedDiscoveryAccounts` | `PATCH /automated-discovery/accounts` | - | - | - | `BatchUpdateAutomatedDiscoveryAccountsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Changes the status of automated sensitive data discovery for one or more accounts. |
| `CreateAllowList` | `POST /allow-lists` | `idempotency-token` | `clientToken`, `criteria`, `name` | `clientToken` | `CreateAllowListResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates and defines the settings for an allow list. |
| `CreateClassificationJob` | `POST /jobs` | `idempotency-token` | `clientToken`, `jobType`, `name`, `s3JobDefinition` | `clientToken` | `CreateClassificationJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates and defines the settings for a classification job. |
| `CreateCustomDataIdentifier` | `POST /custom-data-identifiers` | `idempotency-token` | `name`, `regex` | `clientToken` | `CreateCustomDataIdentifierResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates and defines the criteria and other settings for a custom data identifier. |
| `CreateFindingsFilter` | `POST /findingsfilters` | `idempotency-token` | `action`, `findingCriteria`, `name` | `clientToken` | `CreateFindingsFilterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates and defines the criteria and other settings for a findings filter. |
| `CreateInvitations` | `POST /invitations` | - | `accountIds` | - | `CreateInvitationsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Sends an Amazon Macie membership invitation to one or more accounts. |
| `CreateMember` | `POST /members` | - | `account` | - | `CreateMemberResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates an account with an Amazon Macie administrator account. |
| `CreateSampleFindings` | `POST /findings/sample` | - | - | - | `CreateSampleFindingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates sample findings. |
| `DeclineInvitations` | `POST /invitations/decline` | - | `accountIds` | - | `DeclineInvitationsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Declines Amazon Macie membership invitations that were received from specific accounts. |
| `DeleteAllowList` | `DELETE /allow-lists/{id}` | - | `id` | - | `DeleteAllowListResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes an allow list. |
| `DeleteCustomDataIdentifier` | `DELETE /custom-data-identifiers/{id}` | - | `id` | - | `DeleteCustomDataIdentifierResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Soft deletes a custom data identifier. |
| `DeleteFindingsFilter` | `DELETE /findingsfilters/{id}` | - | `id` | - | `DeleteFindingsFilterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes a findings filter. |
| `DeleteInvitations` | `POST /invitations/delete` | - | `accountIds` | - | `DeleteInvitationsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes Amazon Macie membership invitations that were received from specific accounts. |
| `DeleteMember` | `DELETE /members/{id}` | - | `id` | - | `DeleteMemberResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Deletes the association between an Amazon Macie administrator account and an account. |
| `DescribeBuckets` | `POST /datasources/s3` | `paginated` | - | - | `DescribeBucketsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves (queries) statistical data and other information about one or more S3 buckets that Amazon Macie monitors and analyzes for an account. |
| `DescribeClassificationJob` | `GET /jobs/{jobId}` | - | `jobId` | - | `DescribeClassificationJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves the status and settings for a classification job. |
| `DescribeOrganizationConfiguration` | `GET /admin/configuration` | - | - | - | `DescribeOrganizationConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves the Amazon Macie configuration settings for an organization in Organizations. |
| `DisableMacie` | `DELETE /macie` | - | - | - | `DisableMacieResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Disables Amazon Macie and deletes all settings and resources for a Macie account. |
| `DisableOrganizationAdminAccount` | `DELETE /admin` | - | `adminAccountId` | - | `DisableOrganizationAdminAccountResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Disables an account as the delegated Amazon Macie administrator account for an organization in Organizations. |
| `DisassociateFromAdministratorAccount` | `POST /administrator/disassociate` | - | - | - | `DisassociateFromAdministratorAccountResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Disassociates a member account from its Amazon Macie administrator account. |
| `DisassociateFromMasterAccount` | `POST /master/disassociate` | - | - | - | `DisassociateFromMasterAccountResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | (Deprecated) Disassociates a member account from its Amazon Macie administrator account. This operation has been replaced by the DisassociateFromAdministratorAccount operation. |
| `DisassociateMember` | `POST /members/disassociate/{id}` | - | `id` | - | `DisassociateMemberResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Disassociates an Amazon Macie administrator account from a member account. |
| `EnableMacie` | `POST /macie` | `idempotency-token` | - | `clientToken` | `EnableMacieResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Enables Amazon Macie and specifies the configuration settings for a Macie account. |
| `EnableOrganizationAdminAccount` | `POST /admin` | `idempotency-token` | `adminAccountId` | `clientToken` | `EnableOrganizationAdminAccountResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Designates an account as the delegated Amazon Macie administrator account for an organization in Organizations. |
| `GetAdministratorAccount` | `GET /administrator` | - | - | - | `GetAdministratorAccountResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves information about the Amazon Macie administrator account for an account. |
| `GetAllowList` | `GET /allow-lists/{id}` | - | `id` | - | `GetAllowListResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the settings and status of an allow list. |
| `GetAutomatedDiscoveryConfiguration` | `GET /automated-discovery/configuration` | - | - | - | `GetAutomatedDiscoveryConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves the configuration settings and status of automated sensitive data discovery for an organization or standalone account. |
| `GetBucketStatistics` | `POST /datasources/s3/statistics` | - | - | - | `GetBucketStatisticsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves (queries) aggregated statistical data about all the S3 buckets that Amazon Macie monitors and analyzes for an account. |
| `GetClassificationExportConfiguration` | `GET /classification-export-configuration` | - | - | - | `GetClassificationExportConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves the configuration settings for storing data classification results. |
| `GetClassificationScope` | `GET /classification-scopes/{id}` | - | `id` | - | `GetClassificationScopeResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the classification scope settings for an account. |
| `GetCustomDataIdentifier` | `GET /custom-data-identifiers/{id}` | - | `id` | - | `GetCustomDataIdentifierResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves the criteria and other settings for a custom data identifier. |
| `GetFindingStatistics` | `POST /findings/statistics` | - | `groupBy` | - | `GetFindingStatisticsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves (queries) aggregated statistical data about findings. |
| `GetFindings` | `POST /findings/describe` | - | `findingIds` | - | `GetFindingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves the details of one or more findings. |
| `GetFindingsFilter` | `GET /findingsfilters/{id}` | - | `id` | - | `GetFindingsFilterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves the criteria and other settings for a findings filter. |
| `GetFindingsPublicationConfiguration` | `GET /findings-publication-configuration` | - | - | - | `GetFindingsPublicationConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves the configuration settings for publishing findings to Security Hub. |
| `GetInvitationsCount` | `GET /invitations/count` | - | - | - | `GetInvitationsCountResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves the count of Amazon Macie membership invitations that were received by an account. |
| `GetMacieSession` | `GET /macie` | - | - | - | `GetMacieSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves the status and configuration settings for an Amazon Macie account. |
| `GetMasterAccount` | `GET /master` | - | - | - | `GetMasterAccountResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | (Deprecated) Retrieves information about the Amazon Macie administrator account for an account. This operation has been replaced by the GetAdministratorAccount operation. |
| `GetMember` | `GET /members/{id}` | - | `id` | - | `GetMemberResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves information about an account that's associated with an Amazon Macie administrator account. |
| `GetResourceProfile` | `GET /resource-profiles` | - | `resourceArn` | - | `GetResourceProfileResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves (queries) sensitive data discovery statistics and the sensitivity score for an S3 bucket. |
| `GetRevealConfiguration` | `GET /reveal-configuration` | - | - | - | `GetRevealConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves the status and configuration settings for retrieving occurrences of sensitive data reported by findings. |
| `GetSensitiveDataOccurrences` | `GET /findings/{findingId}/reveal` | - | `findingId` | - | `GetSensitiveDataOccurrencesResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `UnprocessableEntityException` | Retrieves occurrences of sensitive data reported by a finding. |
| `GetSensitiveDataOccurrencesAvailability` | `GET /findings/{findingId}/reveal/availability` | - | `findingId` | - | `GetSensitiveDataOccurrencesAvailabilityResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Checks whether occurrences of sensitive data can be retrieved for a finding. |
| `GetSensitivityInspectionTemplate` | `GET /templates/sensitivity-inspections/{id}` | - | `id` | - | `GetSensitivityInspectionTemplateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the settings for the sensitivity inspection template for an account. |
| `GetUsageStatistics` | `POST /usage/statistics` | `paginated` | - | - | `GetUsageStatisticsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves (queries) quotas and aggregated usage data for one or more accounts. |
| `GetUsageTotals` | `GET /usage` | - | - | - | `GetUsageTotalsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves (queries) aggregated usage data for an account. |
| `ListAllowLists` | `GET /allow-lists` | `paginated` | - | - | `ListAllowListsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a subset of information about all the allow lists for an account. |
| `ListAutomatedDiscoveryAccounts` | `GET /automated-discovery/accounts` | `paginated` | - | - | `ListAutomatedDiscoveryAccountsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the status of automated sensitive data discovery for one or more accounts. |
| `ListClassificationJobs` | `POST /jobs/list` | `paginated` | - | - | `ListClassificationJobsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves a subset of information about one or more classification jobs. |
| `ListClassificationScopes` | `GET /classification-scopes` | `paginated` | - | - | `ListClassificationScopesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves a subset of information about the classification scope for an account. |
| `ListCustomDataIdentifiers` | `POST /custom-data-identifiers/list` | `paginated` | - | - | `ListCustomDataIdentifiersResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves a subset of information about the custom data identifiers for an account. |
| `ListFindings` | `POST /findings` | `paginated` | - | - | `ListFindingsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves a subset of information about one or more findings. |
| `ListFindingsFilters` | `GET /findingsfilters` | `paginated` | - | - | `ListFindingsFiltersResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves a subset of information about all the findings filters for an account. |
| `ListInvitations` | `GET /invitations` | `paginated` | - | - | `ListInvitationsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves information about Amazon Macie membership invitations that were received by an account. |
| `ListManagedDataIdentifiers` | `POST /managed-data-identifiers/list` | `paginated` | - | - | `ListManagedDataIdentifiersResponse` | - | Retrieves information about all the managed data identifiers that Amazon Macie currently provides. |
| `ListMembers` | `GET /members` | `paginated` | - | - | `ListMembersResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves information about the accounts that are associated with an Amazon Macie administrator account. |
| `ListOrganizationAdminAccounts` | `GET /admin` | `paginated` | - | - | `ListOrganizationAdminAccountsResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves information about the delegated Amazon Macie administrator account for an organization in Organizations. |
| `ListResourceProfileArtifacts` | `GET /resource-profiles/artifacts` | `paginated` | `resourceArn` | - | `ListResourceProfileArtifactsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about objects that Amazon Macie selected from an S3 bucket for automated sensitive data discovery. |
| `ListResourceProfileDetections` | `GET /resource-profiles/detections` | `paginated` | `resourceArn` | - | `ListResourceProfileDetectionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves information about the types and amount of sensitive data that Amazon Macie found in an S3 bucket. |
| `ListSensitivityInspectionTemplates` | `GET /templates/sensitivity-inspections` | `paginated` | - | - | `ListSensitivityInspectionTemplatesResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves a subset of information about the sensitivity inspection template for an account. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | - | `resourceArn` | - | `ListTagsForResourceResponse` | - | Retrieves the tags (keys and values) that are associated with an Amazon Macie resource. |
| `PutClassificationExportConfiguration` | `PUT /classification-export-configuration` | - | `configuration` | - | `PutClassificationExportConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Adds or updates the configuration settings for storing data classification results. |
| `PutFindingsPublicationConfiguration` | `PUT /findings-publication-configuration` | `idempotency-token` | - | `clientToken` | `PutFindingsPublicationConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the configuration settings for publishing findings to Security Hub. |
| `SearchResources` | `POST /datasources/search-resources` | `paginated` | - | - | `SearchResourcesResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Retrieves (queries) statistical data and other information about Amazon Web Services resources that Amazon Macie monitors and analyzes for an account. |
| `TagResource` | `POST /tags/{resourceArn}` | - | `resourceArn`, `tags` | - | `TagResourceResponse` | - | Adds or updates one or more tags (keys and values) that are associated with an Amazon Macie resource. |
| `TestCustomDataIdentifier` | `POST /custom-data-identifiers/test` | - | `regex`, `sampleText` | - | `TestCustomDataIdentifierResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Tests criteria for a custom data identifier. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | - | Removes one or more tags (keys and values) from an Amazon Macie resource. |
| `UpdateAllowList` | `PUT /allow-lists/{id}` | - | `criteria`, `id`, `name` | - | `UpdateAllowListResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the settings for an allow list. |
| `UpdateAutomatedDiscoveryConfiguration` | `PUT /automated-discovery/configuration` | - | `status` | - | `UpdateAutomatedDiscoveryConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Changes the configuration settings and status of automated sensitive data discovery for an organization or standalone account. |
| `UpdateClassificationJob` | `PATCH /jobs/{jobId}` | - | `jobId`, `jobStatus` | - | `UpdateClassificationJobResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Changes the status of a classification job. |
| `UpdateClassificationScope` | `PATCH /classification-scopes/{id}` | - | `id` | - | `UpdateClassificationScopeResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the classification scope settings for an account. |
| `UpdateFindingsFilter` | `PATCH /findingsfilters/{id}` | `idempotency-token` | `id` | `clientToken` | `UpdateFindingsFilterResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the criteria and other settings for a findings filter. |
| `UpdateMacieSession` | `PATCH /macie` | - | - | - | `UpdateMacieSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Suspends or re-enables Amazon Macie, or updates the configuration settings for a Macie account. |
| `UpdateMemberSession` | `PATCH /macie/members/{id}` | - | `id`, `status` | - | `UpdateMemberSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Enables an Amazon Macie administrator to suspend or re-enable Macie for a member account. |
| `UpdateOrganizationConfiguration` | `PATCH /admin/configuration` | - | `autoEnable` | - | `UpdateOrganizationConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the Amazon Macie configuration settings for an organization in Organizations. |
| `UpdateResourceProfile` | `PATCH /resource-profiles` | - | `resourceArn` | - | `UpdateResourceProfileResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the sensitivity score for an S3 bucket. |
| `UpdateResourceProfileDetections` | `PATCH /resource-profiles/detections` | - | `resourceArn` | - | `UpdateResourceProfileDetectionsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Updates the sensitivity scoring settings for an S3 bucket. |
| `UpdateRevealConfiguration` | `PUT /reveal-configuration` | - | `configuration` | - | `UpdateRevealConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Updates the status and configuration settings for retrieving occurrences of sensitive data reported by findings. |
| `UpdateSensitivityInspectionTemplate` | `PUT /templates/sensitivity-inspections/{id}` | - | `id` | - | `UpdateSensitivityInspectionTemplateResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates the settings for the sensitivity inspection template for an account. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | `message` | Provides information about an error that occurred due to insufficient access to a specified resource. |
| `InternalServerException` | `structure` | `message` | Provides information about an error that occurred due to an unknown internal server error, exception, or failure. |
| `ThrottlingException` | `structure` | `message` | Provides information about an error that occurred because too many requests were sent during a certain amount of time. |
| `ValidationException` | `structure` | `message` | Provides information about an error that occurred due to a syntax error in a request. |
| `ResourceNotFoundException` | `structure` | `message` | Provides information about an error that occurred because a specified resource wasn't found. |
| `ServiceQuotaExceededException` | `structure` | `message` | Provides information about an error that occurred due to one or more service quotas for an account. |
| `ConflictException` | `structure` | `message` | Provides information about an error that occurred due to a versioning conflict for a specified resource. |
| `AcceptInvitationRequest` | `structure` | `administratorAccountId`, `invitationId`, `masterAccount` | - |
| `AcceptInvitationResponse` | `structure` | - | - |
| `BatchGetCustomDataIdentifiersRequest` | `structure` | `ids` | - |
| `BatchGetCustomDataIdentifiersResponse` | `structure` | `customDataIdentifiers`, `notFoundIdentifierIds` | - |
| `BatchUpdateAutomatedDiscoveryAccountsRequest` | `structure` | `accounts` | - |
| `BatchUpdateAutomatedDiscoveryAccountsResponse` | `structure` | `errors` | - |
| `CreateAllowListRequest` | `structure` | `clientToken`, `criteria`, `description`, `name`, `tags` | - |
| `CreateAllowListResponse` | `structure` | `arn`, `id` | - |
| `CreateClassificationJobRequest` | `structure` | `allowListIds`, `clientToken`, `customDataIdentifierIds`, `description`, `initialRun`, `jobType`, `managedDataIdentifierIds`, `managedDataIdentifierSelector`, `name`, `s3JobDefinition`, `samplingPercentage`, `scheduleFrequency`, ... (+1) | - |
| `CreateClassificationJobResponse` | `structure` | `jobArn`, `jobId` | - |
| `CreateCustomDataIdentifierRequest` | `structure` | `clientToken`, `description`, `ignoreWords`, `keywords`, `maximumMatchDistance`, `name`, `regex`, `severityLevels`, `tags` | - |
| `CreateCustomDataIdentifierResponse` | `structure` | `customDataIdentifierId` | - |
| `CreateFindingsFilterRequest` | `structure` | `action`, `clientToken`, `description`, `findingCriteria`, `name`, `position`, `tags` | - |
| `CreateFindingsFilterResponse` | `structure` | `arn`, `id` | - |
| `CreateInvitationsRequest` | `structure` | `accountIds`, `disableEmailNotification`, `message` | - |
| `CreateInvitationsResponse` | `structure` | `unprocessedAccounts` | - |
| `CreateMemberRequest` | `structure` | `account`, `tags` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
