# Inspector2

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Amazon Inspector is a vulnerability discovery service that automates continuous scanning for security vulnerabilities within your Amazon EC2, Amazon ECR, and Amazon Web Services Lambda environments.

## Possible Usage Scenarios
- Backported from `crates/winterbaume-inspector2/tests/scenario_test.rs`: enable and disable scanning resources and verify the scanning lifecycle.
- Backported from `scenario_test.rs`: create, update, list, and delete finding filters.
- Backported from `scenario_test.rs`: associate and disassociate member accounts, and rotate encryption key configuration.
- Scenario insight from EC2: include mutable binding failover for Inspector2 where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: exercise account or service defaults for Inspector2 by toggling configuration, creating later resources without explicit overrides, and confirming the default propagates into those resources.
- Scenario insight from EC2: cover association replacement for Inspector2 by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- From the AWS documentation and model: model vulnerability management across account membership, scanning configuration, findings, filters, coverage, delegated administration, encryption, and reporting/export operations.

## Service Identity and Protocol

- AWS model slug: `inspector2`
- AWS SDK for Rust slug: `inspector2`
- Model version: `2020-06-08`
- Model file: `vendor/api-models-aws/models/inspector2/service/2020-06-08/inspector2-2020-06-08.json`
- SDK ID: `Inspector2`
- Endpoint prefix: `-`
- ARN namespace: `inspector2`
- CloudFormation name: `Inspector2`
- CloudTrail event source: `inspector2.amazon.aws`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (17), `Get` (13), `Update` (9), `Batch` (8), `Create` (6), `Delete` (4), `Cancel` (2), `Disable` (2).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateMember`, `BatchAssociateCodeSecurityScanConfiguration`, `BatchDisassociateCodeSecurityScanConfiguration`, `BatchGetAccountStatus`, `BatchGetCodeSnippet`, `BatchGetFindingDetails`, `BatchGetFreeTrialInfo`, `BatchGetMemberEc2DeepInspectionStatus`, `BatchUpdateMemberEc2DeepInspectionStatus`, `CancelFindingsReport`, `CancelSbomExport`, `CreateCisScanConfiguration`, `CreateCodeSecurityIntegration`, `CreateCodeSecurityScanConfiguration`, `CreateFilter`, `CreateFindingsReport`, `CreateSbomExport`, `DeleteCisScanConfiguration`, `DeleteCodeSecurityIntegration`, `DeleteCodeSecurityScanConfiguration`, ... (+20).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeOrganizationConfiguration`, `GetCisScanReport`, `GetCisScanResultDetails`, `GetClustersForImage`, `GetCodeSecurityIntegration`, `GetCodeSecurityScan`, `GetCodeSecurityScanConfiguration`, `GetConfiguration`, `GetDelegatedAdminAccount`, `GetEc2DeepInspectionConfiguration`, `GetEncryptionKey`, `GetFindingsReportStatus`, `GetMember`, `GetSbomExport`, `ListAccountPermissions`, `ListCisScanConfigurations`, `ListCisScanResultsAggregatedByChecks`, `ListCisScanResultsAggregatedByTargetResource`, `ListCisScans`, `ListCodeSecurityIntegrations`, ... (+12).
- Pagination is modelled for 16 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 13 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `BatchAssociateCodeSecurityScanConfiguration`, `BatchDisassociateCodeSecurityScanConfiguration`, `CancelFindingsReport`, `CancelSbomExport`, `CreateCisScanConfiguration`, `CreateCodeSecurityScanConfiguration`, `CreateFindingsReport`, `CreateSbomExport`, `DeleteCisScanConfiguration`, `DeleteCodeSecurityScanConfiguration`, `GetCisScanReport`, `GetCisScanResultDetails`, `GetCodeSecurityScan`, `GetCodeSecurityScanConfiguration`, `GetFindingsReportStatus`, `GetSbomExport`, `ListCisScanConfigurations`, `ListCisScanResultsAggregatedByChecks`, `ListCisScanResultsAggregatedByTargetResource`, `ListCisScans`, ... (+7).
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 75 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `Lambda`, `EC2/VPC`, `ECR`, `ECS`, `EKS`, `STS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/inspector/latest/user/what-is-inspector.html
- https://docs.aws.amazon.com/inspector/latest/user/admin-member-relationship.html
- https://docs.aws.amazon.com/inspector/latest/user/scanning_resources_lambda.html

Research outcomes:
- Amazon Inspector is an automated vulnerability management service for resources such as EC2 instances, ECR container images, and Lambda functions.
- Inspector produces findings with vulnerability, severity, affected resource, package, remediation, and score information.
- Lambda standard scanning analyses package dependencies. Lambda code scanning can detect code vulnerabilities such as injection flaws, weak cryptography, and hardcoded credentials.
- Delegated administrator accounts can enable and govern Inspector scanning for member accounts in an organisation.
- Member accounts retain local visibility and some control while the delegated administrator manages organisation-wide scanning configuration.
- Findings reports can be exported to S3 with KMS encryption and filtering.
- Coverage and dashboard views aggregate scanning coverage, critical findings, and resource risk.

Parity implications:
- Model organisation configuration, delegated administrator, member accounts, scan configuration, covered resources, findings, filters, suppression, SBOM/report export, and coverage state separately.
- Finding lifecycle should depend on scanner state, resource changes, and vulnerability database updates.
- Export jobs should be asynchronous and validate S3 and KMS policies.

## Operation Groups

### List

- Operations: `ListAccountPermissions`, `ListCisScanConfigurations`, `ListCisScanResultsAggregatedByChecks`, `ListCisScanResultsAggregatedByTargetResource`, `ListCisScans`, `ListCodeSecurityIntegrations`, `ListCodeSecurityScanConfigurationAssociations`, `ListCodeSecurityScanConfigurations`, `ListCoverage`, `ListCoverageStatistics`, `ListDelegatedAdminAccounts`, `ListFilters`, `ListFindingAggregations`, `ListFindings`, `ListMembers`, `ListTagsForResource`, `ListUsageTotals`
- Traits: `paginated` (13), `readonly` (1)
- Common required input members in this group: `scanArn`

### Get

- Operations: `GetCisScanReport`, `GetCisScanResultDetails`, `GetClustersForImage`, `GetCodeSecurityIntegration`, `GetCodeSecurityScan`, `GetCodeSecurityScanConfiguration`, `GetConfiguration`, `GetDelegatedAdminAccount`, `GetEc2DeepInspectionConfiguration`, `GetEncryptionKey`, `GetFindingsReportStatus`, `GetMember`, `GetSbomExport`
- Traits: `paginated` (2), `readonly` (2), `idempotent` (1)
- Common required input members in this group: `scanArn`, `accountId`

### Update

- Operations: `UpdateCisScanConfiguration`, `UpdateCodeSecurityIntegration`, `UpdateCodeSecurityScanConfiguration`, `UpdateConfiguration`, `UpdateEc2DeepInspectionConfiguration`, `UpdateEncryptionKey`, `UpdateFilter`, `UpdateOrganizationConfiguration`, `UpdateOrgEc2DeepInspectionConfiguration`
- Traits: `idempotent` (1)
- Common required input members in this group: `scanConfigurationArn`

### Batch

- Operations: `BatchAssociateCodeSecurityScanConfiguration`, `BatchDisassociateCodeSecurityScanConfiguration`, `BatchGetAccountStatus`, `BatchGetCodeSnippet`, `BatchGetFindingDetails`, `BatchGetFreeTrialInfo`, `BatchGetMemberEc2DeepInspectionStatus`, `BatchUpdateMemberEc2DeepInspectionStatus`
- Common required input members in this group: `findingArns`, `accountIds`

### Create

- Operations: `CreateCisScanConfiguration`, `CreateCodeSecurityIntegration`, `CreateCodeSecurityScanConfiguration`, `CreateFilter`, `CreateFindingsReport`, `CreateSbomExport`
- Traits: `idempotent` (1)
- Common required input members in this group: `name`, `reportFormat`, `s3Destination`

### Delete

- Operations: `DeleteCisScanConfiguration`, `DeleteCodeSecurityIntegration`, `DeleteCodeSecurityScanConfiguration`, `DeleteFilter`
- Common required input members in this group: `scanConfigurationArn`

### Cancel

- Operations: `CancelFindingsReport`, `CancelSbomExport`
- Traits: `idempotent` (1)
- Common required input members in this group: `reportId`

### Disable

- Operations: `Disable`, `DisableDelegatedAdminAccount`
- Common required input members in this group: -

### Enable

- Operations: `Enable`, `EnableDelegatedAdminAccount`
- Traits: `idempotency-token` (2)
- Common required input members in this group: -

### Send

- Operations: `SendCisSessionHealth`, `SendCisSessionTelemetry`
- Traits: `idempotent` (2)
- Common required input members in this group: `scanJobId`, `sessionToken`

### Start

- Operations: `StartCisSession`, `StartCodeSecurityScan`
- Traits: `idempotent` (1), `idempotency-token` (1)
- Common required input members in this group: -

### Associate

- Operations: `AssociateMember`
- Common required input members in this group: -

### Describe

- Operations: `DescribeOrganizationConfiguration`
- Common required input members in this group: -

### Disassociate

- Operations: `DisassociateMember`
- Common required input members in this group: -

### Reset

- Operations: `ResetEncryptionKey`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Search

- Operations: `SearchVulnerabilities`
- Traits: `paginated` (1)
- Common required input members in this group: -

### Stop

- Operations: `StopCisSession`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: -

### Untag

- Operations: `UntagResource`
- Common required input members in this group: -

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateMember` | `POST /members/associate` | - | `accountId` | - | `AssociateMemberResponse` | `AccessDeniedException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Associates an Amazon Web Services account with an Amazon Inspector delegated administrator. An HTTP 200 response indicates the association was successfully started, but doesn’t indicate whether it was completed. You ... |
| `BatchAssociateCodeSecurityScanConfiguration` | `POST /codesecurity/scan-configuration/batch/associate` | - | `associateConfigurationRequests` | - | `BatchAssociateCodeSecurityScanConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Associates multiple code repositories with an Amazon Inspector code security scan configuration. |
| `BatchDisassociateCodeSecurityScanConfiguration` | `POST /codesecurity/scan-configuration/batch/disassociate` | - | `disassociateConfigurationRequests` | - | `BatchDisassociateCodeSecurityScanConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disassociates multiple code repositories from an Amazon Inspector code security scan configuration. |
| `BatchGetAccountStatus` | `POST /status/batch/get` | - | - | - | `BatchGetAccountStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves the Amazon Inspector status of multiple Amazon Web Services accounts within your environment. |
| `BatchGetCodeSnippet` | `POST /codesnippet/batchget` | - | `findingArns` | - | `BatchGetCodeSnippetResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves code snippets from findings that Amazon Inspector detected code vulnerabilities in. |
| `BatchGetFindingDetails` | `POST /findings/details/batch/get` | - | `findingArns` | - | `BatchGetFindingDetailsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Gets vulnerability details for findings. |
| `BatchGetFreeTrialInfo` | `POST /freetrialinfo/batchget` | - | `accountIds` | - | `BatchGetFreeTrialInfoResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Gets free trial status for multiple Amazon Web Services accounts. |
| `BatchGetMemberEc2DeepInspectionStatus` | `POST /ec2deepinspectionstatus/member/batch/get` | - | - | - | `BatchGetMemberEc2DeepInspectionStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves Amazon Inspector deep inspection activation status of multiple member accounts within your organization. You must be the delegated administrator of an organization in Amazon Inspector to use this API. |
| `BatchUpdateMemberEc2DeepInspectionStatus` | `POST /ec2deepinspectionstatus/member/batch/update` | - | `accountIds` | - | `BatchUpdateMemberEc2DeepInspectionStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Activates or deactivates Amazon Inspector deep inspection for the provided member accounts in your organization. You must be the delegated administrator of an organization in Amazon Inspector to use this API. |
| `CancelFindingsReport` | `POST /reporting/cancel` | - | `reportId` | - | `CancelFindingsReportResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels the given findings report. |
| `CancelSbomExport` | `POST /sbomexport/cancel` | `idempotent` | `reportId` | - | `CancelSbomExportResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Cancels a software bill of materials (SBOM) report. |
| `CreateCisScanConfiguration` | `POST /cis/scan-configuration/create` | - | `scanName`, `securityLevel`, `schedule`, `targets` | - | `CreateCisScanConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Creates a CIS scan configuration. |
| `CreateCodeSecurityIntegration` | `POST /codesecurity/integration/create` | - | `name`, `type` | - | `CreateCodeSecurityIntegrationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a code security integration with a source code repository provider. After calling the CreateCodeSecurityIntegration operation, you complete authentication and authorization with your provider. Next you call t ... |
| `CreateCodeSecurityScanConfiguration` | `POST /codesecurity/scan-configuration/create` | - | `name`, `level`, `configuration` | - | `CreateCodeSecurityScanConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a scan configuration for code security scanning. |
| `CreateFilter` | `POST /filters/create` | - | `action`, `filterCriteria`, `name` | - | `CreateFilterResponse` | `AccessDeniedException`, `BadRequestException`, `InternalServerException`, `ServiceQuotaExceededException`, `ThrottlingException`, `ValidationException` | Creates a filter resource using specified filter criteria. When the filter action is set to SUPPRESS this action creates a suppression rule. |
| `CreateFindingsReport` | `POST /reporting/create` | - | `reportFormat`, `s3Destination` | - | `CreateFindingsReportResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a finding report. By default only ACTIVE findings are returned in the report. To see SUPRESSED or CLOSED findings you must specify a value for the findingStatus filter criteria. |
| `CreateSbomExport` | `POST /sbomexport/create` | `idempotent` | `reportFormat`, `s3Destination` | - | `CreateSbomExportResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Creates a software bill of materials (SBOM) report. |
| `DeleteCisScanConfiguration` | `POST /cis/scan-configuration/delete` | - | `scanConfigurationArn` | - | `DeleteCisScanConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a CIS scan configuration. |
| `DeleteCodeSecurityIntegration` | `POST /codesecurity/integration/delete` | - | `integrationArn` | - | `DeleteCodeSecurityIntegrationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a code security integration. |
| `DeleteCodeSecurityScanConfiguration` | `POST /codesecurity/scan-configuration/delete` | - | `scanConfigurationArn` | - | `DeleteCodeSecurityScanConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a code security scan configuration. |
| `DeleteFilter` | `POST /filters/delete` | - | `arn` | - | `DeleteFilterResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Deletes a filter resource. |
| `DescribeOrganizationConfiguration` | `POST /organizationconfiguration/describe` | - | - | - | `DescribeOrganizationConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Describe Amazon Inspector configuration settings for an Amazon Web Services organization. |
| `Disable` | `POST /disable` | - | - | - | `DisableResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disables Amazon Inspector scans for one or more Amazon Web Services accounts. Disabling all scan types in an account disables the Amazon Inspector service. |
| `DisableDelegatedAdminAccount` | `POST /delegatedadminaccounts/disable` | - | `delegatedAdminAccountId` | - | `DisableDelegatedAdminAccountResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Disables the Amazon Inspector delegated administrator for your organization. |
| `DisassociateMember` | `POST /members/disassociate` | - | `accountId` | - | `DisassociateMemberResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Disassociates a member account from an Amazon Inspector delegated administrator. |
| `Enable` | `POST /enable` | `idempotency-token` | `resourceTypes` | `clientToken` | `EnableResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables Amazon Inspector scans for one or more Amazon Web Services accounts. |
| `EnableDelegatedAdminAccount` | `POST /delegatedadminaccounts/enable` | `idempotency-token` | `delegatedAdminAccountId` | `clientToken` | `EnableDelegatedAdminAccountResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Enables the Amazon Inspector delegated administrator for your Organizations organization. |
| `GetCisScanReport` | `POST /cis/scan/report/get` | - | `scanArn` | - | `GetCisScanReportResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves a CIS scan report. |
| `GetCisScanResultDetails` | `POST /cis/scan-result/details/get` | `paginated` | `scanArn`, `targetResourceId`, `accountId` | - | `GetCisScanResultDetailsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Retrieves CIS scan result details. |
| `GetClustersForImage` | `POST /cluster/get` | `paginated` | `filter` | - | `GetClustersForImageResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a list of clusters and metadata associated with an image. |
| `GetCodeSecurityIntegration` | `POST /codesecurity/integration/get` | - | `integrationArn` | - | `GetCodeSecurityIntegrationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a code security integration. |
| `GetCodeSecurityScan` | `POST /codesecurity/scan/get` | `readonly` | `resource`, `scanId` | - | `GetCodeSecurityScanResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a specific code security scan. |
| `GetCodeSecurityScanConfiguration` | `POST /codesecurity/scan-configuration/get` | - | `scanConfigurationArn` | - | `GetCodeSecurityScanConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about a code security scan configuration. |
| `GetConfiguration` | `POST /configuration/get` | - | - | - | `GetConfigurationResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves setting configurations for Inspector scans. |
| `GetDelegatedAdminAccount` | `POST /delegatedadminaccounts/get` | - | - | - | `GetDelegatedAdminAccountResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Retrieves information about the Amazon Inspector delegated administrator for your organization. |
| `GetEc2DeepInspectionConfiguration` | `POST /ec2deepinspectionconfiguration/get` | - | - | - | `GetEc2DeepInspectionConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException` | Retrieves the activation status of Amazon Inspector deep inspection and custom paths associated with your account. |
| `GetEncryptionKey` | `GET /encryptionkey/get` | `readonly` | `scanType`, `resourceType` | - | `GetEncryptionKeyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets an encryption key. |
| `GetFindingsReportStatus` | `POST /reporting/status/get` | - | - | - | `GetFindingsReportStatusResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets the status of a findings report. |
| `GetMember` | `POST /members/get` | - | `accountId` | - | `GetMemberResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets member information for your organization. |
| `GetSbomExport` | `POST /sbomexport/get` | `idempotent` | `reportId` | - | `GetSbomExportResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Gets details of a software bill of materials (SBOM) report. |
| `ListAccountPermissions` | `POST /accountpermissions/list` | `paginated` | - | - | `ListAccountPermissionsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the permissions an account has to configure Amazon Inspector. If the account is a member account or standalone account with resources managed by an Organizations policy, the operation returns fewer permissions. |
| `ListCisScanConfigurations` | `POST /cis/scan-configuration/list` | `paginated` | - | - | `ListCisScanConfigurationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists CIS scan configurations. |
| `ListCisScanResultsAggregatedByChecks` | `POST /cis/scan-result/check/list` | `paginated` | `scanArn` | - | `ListCisScanResultsAggregatedByChecksResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists scan results aggregated by checks. |
| `ListCisScanResultsAggregatedByTargetResource` | `POST /cis/scan-result/resource/list` | `paginated` | `scanArn` | - | `ListCisScanResultsAggregatedByTargetResourceResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists scan results aggregated by a target resource. |
| `ListCisScans` | `POST /cis/scan/list` | `paginated` | - | - | `ListCisScansResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Returns a CIS scan list. |
| `ListCodeSecurityIntegrations` | `POST /codesecurity/integration/list` | - | - | - | `ListCodeSecurityIntegrationsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists all code security integrations in your account. |
| `ListCodeSecurityScanConfigurationAssociations` | `POST /codesecurity/scan-configuration/associations/list` | - | `scanConfigurationArn` | - | `ListCodeSecurityScanConfigurationAssociationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists the associations between code repositories and Amazon Inspector code security scan configurations. |
| `ListCodeSecurityScanConfigurations` | `POST /codesecurity/scan-configuration/list` | - | - | - | `ListCodeSecurityScanConfigurationsResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all code security scan configurations in your account. |
| `ListCoverage` | `POST /coverage/list` | `paginated` | - | - | `ListCoverageResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists coverage details for your environment. |
| `ListCoverageStatistics` | `POST /coverage/statistics/list` | `paginated` | - | - | `ListCoverageStatisticsResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists Amazon Inspector coverage statistics for your environment. |
| `ListDelegatedAdminAccounts` | `POST /delegatedadminaccounts/list` | `paginated` | - | - | `ListDelegatedAdminAccountsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists information about the Amazon Inspector delegated administrator of your organization. |
| `ListFilters` | `POST /filters/list` | `paginated` | - | - | `ListFiltersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the filters associated with your account. |
| `ListFindingAggregations` | `POST /findings/aggregation/list` | `paginated` | `aggregationType` | - | `ListFindingAggregationsResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists aggregated finding data for your environment based on specific criteria. |
| `ListFindings` | `POST /findings/list` | `paginated` | - | - | `ListFindingsResponse` | `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists findings for your environment. |
| `ListMembers` | `POST /members/list` | `paginated` | - | - | `ListMembersResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | List members associated with the Amazon Inspector delegated administrator for your organization. |
| `ListTagsForResource` | `GET /tags/{resourceArn}` | `readonly` | `resourceArn` | - | `ListTagsForResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Lists all tags attached to a given resource. |
| `ListUsageTotals` | `POST /usage/list` | `paginated` | - | - | `ListUsageTotalsResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists the Amazon Inspector usage totals over the last 30 days. |
| `ResetEncryptionKey` | `PUT /encryptionkey/reset` | `idempotent` | `scanType`, `resourceType` | - | `ResetEncryptionKeyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Resets an encryption key. After the key is reset your resources will be encrypted by an Amazon Web Services owned key. |
| `SearchVulnerabilities` | `POST /vulnerabilities/search` | `paginated` | `filterCriteria` | - | `SearchVulnerabilitiesResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Lists Amazon Inspector coverage details for a specific vulnerability. |
| `SendCisSessionHealth` | `PUT /cissession/health/send` | `idempotent` | `scanJobId`, `sessionToken` | - | `SendCisSessionHealthResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Sends a CIS session health. This API is used by the Amazon Inspector SSM plugin to communicate with the Amazon Inspector service. The Amazon Inspector SSM plugin calls this API to start a CIS scan session for the sca ... |
| `SendCisSessionTelemetry` | `PUT /cissession/telemetry/send` | `idempotent` | `scanJobId`, `sessionToken`, `messages` | - | `SendCisSessionTelemetryResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Sends a CIS session telemetry. This API is used by the Amazon Inspector SSM plugin to communicate with the Amazon Inspector service. The Amazon Inspector SSM plugin calls this API to start a CIS scan session for the ... |
| `StartCisSession` | `PUT /cissession/start` | `idempotent` | `scanJobId`, `message` | - | `StartCisSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Starts a CIS session. This API is used by the Amazon Inspector SSM plugin to communicate with the Amazon Inspector service. The Amazon Inspector SSM plugin calls this API to start a CIS scan session for the scan ID s ... |
| `StartCodeSecurityScan` | `POST /codesecurity/scan/start` | `idempotency-token` | `resource` | `clientToken` | `StartCodeSecurityScanResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Initiates a code security scan on a specified repository. |
| `StopCisSession` | `PUT /cissession/stop` | `idempotent` | `scanJobId`, `sessionToken`, `message` | - | `StopCisSessionResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Stops a CIS session. This API is used by the Amazon Inspector SSM plugin to communicate with the Amazon Inspector service. The Amazon Inspector SSM plugin calls this API to stop a CIS scan session for the scan ID sup ... |
| `TagResource` | `POST /tags/{resourceArn}` | `idempotent` | `resourceArn`, `tags` | - | `TagResourceResponse` | `BadRequestException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Adds tags to a resource. |
| `UntagResource` | `DELETE /tags/{resourceArn}` | - | `resourceArn`, `tagKeys` | - | `UntagResourceResponse` | `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Removes tags from a resource. |
| `UpdateCisScanConfiguration` | `POST /cis/scan-configuration/update` | - | `scanConfigurationArn` | - | `UpdateCisScanConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates a CIS scan configuration. |
| `UpdateCodeSecurityIntegration` | `POST /codesecurity/integration/update` | - | `integrationArn`, `details` | - | `UpdateCodeSecurityIntegrationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing code security integration. After calling the CreateCodeSecurityIntegration operation, you complete authentication and authorization with your provider. Next you call the UpdateCodeSecurityIntegrat ... |
| `UpdateCodeSecurityScanConfiguration` | `POST /codesecurity/scan-configuration/update` | - | `scanConfigurationArn`, `configuration` | - | `UpdateCodeSecurityScanConfigurationResponse` | `AccessDeniedException`, `ConflictException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an existing code security scan configuration. |
| `UpdateConfiguration` | `POST /configuration/update` | - | - | - | `UpdateConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Updates setting configurations for your Amazon Inspector account. When you use this API as an Amazon Inspector delegated administrator this updates the setting for all accounts you manage. Member accounts in an organ ... |
| `UpdateEc2DeepInspectionConfiguration` | `POST /ec2deepinspectionconfiguration/update` | - | - | - | `UpdateEc2DeepInspectionConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Activates, deactivates Amazon Inspector deep inspection, or updates custom paths for your account. |
| `UpdateEncryptionKey` | `PUT /encryptionkey/update` | `idempotent` | `kmsKeyId`, `scanType`, `resourceType` | - | `UpdateEncryptionKeyResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Updates an encryption key. A ResourceNotFoundException means that an Amazon Web Services owned key is being used for encryption. |
| `UpdateFilter` | `POST /filters/update` | - | `filterArn` | - | `UpdateFilterResponse` | `AccessDeniedException`, `InternalServerException`, `ResourceNotFoundException`, `ThrottlingException`, `ValidationException` | Specifies the action that is to be applied to the findings that match the filter. |
| `UpdateOrganizationConfiguration` | `POST /organizationconfiguration/update` | - | `autoEnable` | - | `UpdateOrganizationConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Updates the configurations for your Amazon Inspector organization. |
| `UpdateOrgEc2DeepInspectionConfiguration` | `POST /ec2deepinspectionconfiguration/org/update` | - | `orgPackagePaths` | - | `UpdateOrgEc2DeepInspectionConfigurationResponse` | `AccessDeniedException`, `InternalServerException`, `ThrottlingException`, `ValidationException` | Updates the Amazon Inspector deep inspection custom paths for your organization. You must be an Amazon Inspector delegated administrator to use this API. |

## HTTP Bindings

Per-operation input members that bind to HTTP transport surfaces. Optional members are easy to miss because they do not appear in the operation matrix's Required input column. RFC 7232 conditional headers (`If-Match`, `If-None-Match`, `If-Modified-Since`, `If-Unmodified-Since`) and service-specific modifier headers (`x-amz-*`, `x-amzn-*`) surface here. Every handler must list each binding as honoured, intentionally unsupported, or ignored-with-rationale.

| Operation | Header inputs | Query inputs | Prefix headers | Payload |
|---|---|---|---|---|
| `GetEncryptionKey` | - | `scanType -> scanType`, `resourceType -> resourceType` | - | - |
| `ListCodeSecurityIntegrations` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListCodeSecurityScanConfigurationAssociations` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `ListCodeSecurityScanConfigurations` | - | `nextToken -> nextToken`, `maxResults -> maxResults` | - | - |
| `UntagResource` | - | `tagKeys -> tagKeys` | - | - |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `AccessDeniedException` | `structure` | message | You do not have sufficient access to perform this action. For Enable , you receive this error if you attempt to use a feature in an unsupported Amazon Web S ... |
| `BadRequestException` | `structure` | message | One or more tags submitted as part of the request is not valid. |
| `ConflictException` | `structure` | message, resourceId, resourceType | A conflict occurred. This exception occurs when the same resource is being modified by concurrent requests. |
| `InternalServerException` | `structure` | message, retryAfterSeconds | The request has failed due to an internal failure of the Amazon Inspector service. |
| `ResourceNotFoundException` | `structure` | message | The operation tried to access an invalid resource. Make sure the resource is specified correctly. |
| `ServiceQuotaExceededException` | `structure` | message, resourceId | You have exceeded your service quota. To perform the requested action, remove some of the relevant resources, or use Service Quotas to request a service quo ... |
| `ThrottlingException` | `structure` | message, retryAfterSeconds | The limit on the number of requests per second was exceeded. |
| `ValidationException` | `structure` | message, reason, fields | The request has failed validation due to missing required fields or having invalid inputs. |
| `AssociateMemberRequest` | `structure` | accountId | - |
| `AssociateMemberResponse` | `structure` | accountId | - |
| `BatchAssociateCodeSecurityScanConfigurationRequest` | `structure` | associateConfigurationRequests | - |
| `BatchAssociateCodeSecurityScanConfigurationResponse` | `structure` | failedAssociations, successfulAssociations | - |
| `BatchDisassociateCodeSecurityScanConfigurationRequest` | `structure` | disassociateConfigurationRequests | - |
| `BatchDisassociateCodeSecurityScanConfigurationResponse` | `structure` | failedAssociations, successfulAssociations | - |
| `BatchGetAccountStatusRequest` | `structure` | accountIds | - |
| `BatchGetAccountStatusResponse` | `structure` | accounts, failedAccounts | - |
| `BatchGetCodeSnippetRequest` | `structure` | findingArns | - |
| `BatchGetCodeSnippetResponse` | `structure` | codeSnippetResults, errors | - |
| `BatchGetFindingDetailsRequest` | `structure` | findingArns | - |
| `BatchGetFindingDetailsResponse` | `structure` | findingDetails, errors | - |
| `BatchGetFreeTrialInfoRequest` | `structure` | accountIds | - |
| `BatchGetFreeTrialInfoResponse` | `structure` | accounts, failedAccounts | - |
| `BatchGetMemberEc2DeepInspectionStatusRequest` | `structure` | accountIds | - |
| `BatchGetMemberEc2DeepInspectionStatusResponse` | `structure` | accountIds, failedAccountIds | - |
| `BatchUpdateMemberEc2DeepInspectionStatusRequest` | `structure` | accountIds | - |
| `BatchUpdateMemberEc2DeepInspectionStatusResponse` | `structure` | accountIds, failedAccountIds | - |
| `CancelFindingsReportRequest` | `structure` | reportId | - |
| `CancelFindingsReportResponse` | `structure` | reportId | - |
| `CancelSbomExportRequest` | `structure` | reportId | - |
| `CancelSbomExportResponse` | `structure` | reportId | - |
| `CreateCisScanConfigurationRequest` | `structure` | scanName, securityLevel, schedule, targets, tags | - |
| `CreateCisScanConfigurationResponse` | `structure` | scanConfigurationArn | - |
| `CreateCodeSecurityIntegrationRequest` | `structure` | name, type, details, tags | - |
| `CreateCodeSecurityIntegrationResponse` | `structure` | integrationArn, status, authorizationUrl | - |
| `CreateCodeSecurityScanConfigurationRequest` | `structure` | name, level, configuration, scopeSettings, tags | - |
| `CreateCodeSecurityScanConfigurationResponse` | `structure` | scanConfigurationArn | - |
| `CreateFilterRequest` | `structure` | action, description, filterCriteria, name, tags, reason | - |
| `CreateFilterResponse` | `structure` | arn | - |
| `CreateFindingsReportRequest` | `structure` | filterCriteria, reportFormat, s3Destination | - |
| `CreateFindingsReportResponse` | `structure` | reportId | - |
| `AssociationResultStatusCode` | `enum` | INTERNAL_ERROR, ACCESS_DENIED, SCAN_CONFIGURATION_NOT_FOUND, INVALID_INPUT, RESOURCE_NOT_FOUND, QUOTA_EXCEEDED | - |
| `CisFindingStatus` | `enum` | PASSED, FAILED, SKIPPED | - |
| `CisFindingStatusComparison` | `enum` | EQUALS | - |
| `CisReportFormat` | `enum` | PDF, CSV | - |
| `CisReportStatus` | `enum` | SUCCEEDED, FAILED, IN_PROGRESS | - |
| `CisResultStatus` | `enum` | PASSED, FAILED, SKIPPED | - |
| `CisResultStatusComparison` | `enum` | EQUALS | - |
| `CisRuleStatus` | `enum` | FAILED, PASSED, NOT_EVALUATED, INFORMATIONAL, UNKNOWN, NOT_APPLICABLE, ERROR | - |
| `CisScanConfigurationsSortBy` | `enum` | SCAN_NAME, SCAN_CONFIGURATION_ARN | - |
| `CisScanResultDetailsSortBy` | `enum` | CHECK_ID, STATUS | - |
## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
