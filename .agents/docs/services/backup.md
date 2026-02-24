# AWS Backup

Source: AWS Smithy API model vendored in `vendor/api-models-aws`.

## Service Overview

Backup Backup is a unified backup service designed to protect Amazon Web Services services and their associated data. Backup simplifies the creation, migration, restoration, and deletion of backups, while also providing reporting and auditing.

## Possible Usage Scenarios
- Scenario insight from EC2: include mutable binding failover for AWS Backup where a stable endpoint, target, subscription, route, or association moves between backing resources and read APIs must show only the new binding.
- Scenario insight from EC2: cover association replacement for AWS Backup by verifying the old parent no longer lists the child, the new parent does, and returned association identifiers match subsequent reads.
- Scenario insight from EC2: add full state-machine walks for AWS Backup resources that start, stop, cancel, complete, disable, or otherwise transition through observable lifecycle states.
- From the AWS documentation and model: create backup plans, backup vaults, recovery points, restore jobs, copy jobs, frameworks, reports, and vault locks.
- From the operation surface: model data-protection policy rollout, cross-Region/account backup copy, restore testing, legal hold, audit reporting, and tag-based backup selection.

## Service Identity and Protocol

- AWS model slug: `backup`
- AWS SDK for Rust slug: `backup`
- Model version: `2018-11-15`
- Model file: `vendor/api-models-aws/models/backup/service/2018-11-15/backup-2018-11-15.json`
- SDK ID: `Backup`
- Endpoint prefix: `backup`
- ARN namespace: `backup`
- CloudFormation name: `Backup`
- CloudTrail event source: `backup.amazonaws.com`
- Protocols: `restJson1`
- Auth schemes: `sigv4`
- Endpoint rule parameters: `Endpoint`, `Region`, `UseDualStack`, `UseFIPS`

## Behavioural Model Notes

- Operation surface is concentrated in these families: `List` (29), `Get` (15), `Delete` (12), `Describe` (12), `Create` (11), `Update` (10), `Start` (5), `Put` (4).
- State-changing operations should define resource existence, duplicate, conflict, and deletion semantics: `AssociateBackupVaultMpaApprovalTeam`, `CancelLegalHold`, `CreateBackupPlan`, `CreateBackupSelection`, `CreateBackupVault`, `CreateFramework`, `CreateLegalHold`, `CreateLogicallyAirGappedBackupVault`, `CreateReportPlan`, `CreateRestoreAccessBackupVault`, `CreateRestoreTestingPlan`, `CreateRestoreTestingSelection`, `CreateTieringConfiguration`, `DeleteBackupPlan`, `DeleteBackupSelection`, `DeleteBackupVault`, `DeleteBackupVaultAccessPolicy`, `DeleteBackupVaultLockConfiguration`, `DeleteBackupVaultNotifications`, `DeleteFramework`, ... (+31).
- Read/list operations should define not-found behaviour, filtering, ordering, and empty-result shapes: `DescribeBackupJob`, `DescribeBackupVault`, `DescribeCopyJob`, `DescribeFramework`, `DescribeGlobalSettings`, `DescribeProtectedResource`, `DescribeRecoveryPoint`, `DescribeRegionSettings`, `DescribeReportJob`, `DescribeReportPlan`, `DescribeRestoreJob`, `DescribeScanJob`, `GetBackupPlan`, `GetBackupPlanFromJSON`, `GetBackupPlanFromTemplate`, `GetBackupSelection`, `GetBackupVaultAccessPolicy`, `GetBackupVaultNotifications`, `GetLegalHold`, `GetRecoveryPointIndexDetails`, ... (+36).
- Pagination is modelled for 29 operations; token stability and page boundaries are observable API behaviour.
- Idempotency is explicit for 70 operations; repeated calls with the same token should preserve AWS-compatible outcomes.
- Asynchronous or job-like operations need lifecycle states, polling semantics, and terminal failure modelling: `CancelLegalHold`, `CreateReportPlan`, `DeleteReportPlan`, `DescribeBackupJob`, `DescribeCopyJob`, `DescribeReportJob`, `DescribeReportPlan`, `DescribeRestoreJob`, `DescribeScanJob`, `ExportBackupPlanTemplate`, `GetRestoreJobMetadata`, `ListBackupJobSummaries`, `ListBackupJobs`, `ListCopyJobSummaries`, `ListCopyJobs`, `ListReportJobs`, `ListReportPlans`, `ListRestoreJobSummaries`, `ListRestoreJobs`, `ListRestoreJobsByProtectedResource`, ... (+9).
- Tagging is part of the service contract; preserve tag key uniqueness, merge/replace semantics, and list-tags ARN validation.
- 108 operations declare modelled service errors; parity work should map exact error names and retryability where documented.
- Documentation and model terms indicate cross-service dependencies or identifiers: `IAM`, `S3`, `KMS`, `SNS`, `ECS`, `RDS`.
- Some responses appear to be derived from telemetry, managed inventories, recommendations, or findings; seedable mock state may be required because real AWS derives these from external systems.

## Official AWS Documentation Research

Sources:
- https://docs.aws.amazon.com/aws-backup/latest/devguide/whatisbackup.html
- https://docs.aws.amazon.com/aws-backup/latest/devguide/creating-a-backup-plan.html
- https://docs.aws.amazon.com/aws-backup/latest/devguide/create-cross-account-backup.html

Research outcomes:
- AWS Backup uses policy-based backup plans to define backup rules and assign protected resources directly or by tag.
- Backup vaults store immutable recovery points separately from their source resources and can have resource-based access policies.
- Backup lifecycle policies can transition supported backups from warm storage to cold storage according to schedule.
- Backups can be copied across Regions or accounts, including scheduled plan copies and on-demand copies.
- Cross-account backup depends on AWS Organizations structure and destination-account/vault access configuration.
- AWS Backup performs incremental backups for supported resources: the first backup is full, and subsequent backups store only changes.
- Some resource types support full AWS Backup management, including independent vault KMS encryption and `arn:aws:backup` recovery point ARNs.
- Backup Vault Lock can enforce a write-once-read-many model that prevents deletion or retention alteration.

Parity implications:
- Model backup plans, selections, rules, protected resources, vaults, recovery points, jobs, copy jobs, restore jobs, lifecycle state, and vault lock.
- Backup job creation should resolve resource assignments, schedule windows, lifecycle settings, copy actions, KMS/vault policies, and supported-resource capabilities.
- Recovery points should survive source deletion according to vault/lifecycle rules and expose immutable retention constraints.

## Operation Groups

### List

- Operations: `ListBackupJobSummaries`, `ListBackupJobs`, `ListBackupPlanTemplates`, `ListBackupPlanVersions`, `ListBackupPlans`, `ListBackupSelections`, `ListBackupVaults`, `ListCopyJobSummaries`, `ListCopyJobs`, `ListFrameworks`, `ListIndexedRecoveryPoints`, `ListLegalHolds`, `ListProtectedResources`, `ListProtectedResourcesByBackupVault`, `ListRecoveryPointsByBackupVault`, `ListRecoveryPointsByLegalHold`, `ListRecoveryPointsByResource`, `ListReportJobs`, `ListReportPlans`, `ListRestoreAccessBackupVaults`, `ListRestoreJobSummaries`, `ListRestoreJobs`, `ListRestoreJobsByProtectedResource`, `ListRestoreTestingPlans`, `ListRestoreTestingSelections`, `ListScanJobSummaries`, `ListScanJobs`, `ListTags`, `ListTieringConfigurations`
- Traits: `idempotent` (15), `paginated` (29)
- Common required input members in this group: `BackupPlanId`, `BackupVaultName`, `LegalHoldId`, `ResourceArn`, `RestoreTestingPlanName`

### Get

- Operations: `GetBackupPlan`, `GetBackupPlanFromJSON`, `GetBackupPlanFromTemplate`, `GetBackupSelection`, `GetBackupVaultAccessPolicy`, `GetBackupVaultNotifications`, `GetLegalHold`, `GetRecoveryPointIndexDetails`, `GetRecoveryPointRestoreMetadata`, `GetRestoreJobMetadata`, `GetRestoreTestingInferredMetadata`, `GetRestoreTestingPlan`, `GetRestoreTestingSelection`, `GetSupportedResourceTypes`, `GetTieringConfiguration`
- Traits: `idempotent` (8)
- Common required input members in this group: `BackupPlanId`, `BackupPlanTemplateId`, `BackupPlanTemplateJson`, `BackupVaultName`, `LegalHoldId`, `RecoveryPointArn`, `RestoreJobId`, `RestoreTestingPlanName`, `RestoreTestingSelectionName`, `SelectionId`, `TieringConfigurationName`

### Delete

- Operations: `DeleteBackupPlan`, `DeleteBackupSelection`, `DeleteBackupVault`, `DeleteBackupVaultAccessPolicy`, `DeleteBackupVaultLockConfiguration`, `DeleteBackupVaultNotifications`, `DeleteFramework`, `DeleteRecoveryPoint`, `DeleteReportPlan`, `DeleteRestoreTestingPlan`, `DeleteRestoreTestingSelection`, `DeleteTieringConfiguration`
- Traits: `idempotent` (9)
- Common required input members in this group: `BackupPlanId`, `BackupVaultName`, `FrameworkName`, `RecoveryPointArn`, `ReportPlanName`, `RestoreTestingPlanName`, `RestoreTestingSelectionName`, `SelectionId`, `TieringConfigurationName`

### Describe

- Operations: `DescribeBackupJob`, `DescribeBackupVault`, `DescribeCopyJob`, `DescribeFramework`, `DescribeGlobalSettings`, `DescribeProtectedResource`, `DescribeRecoveryPoint`, `DescribeRegionSettings`, `DescribeReportJob`, `DescribeReportPlan`, `DescribeRestoreJob`, `DescribeScanJob`
- Traits: `idempotent` (7)
- Common required input members in this group: `BackupJobId`, `BackupVaultName`, `CopyJobId`, `FrameworkName`, `RecoveryPointArn`, `ReportJobId`, `ReportPlanName`, `ResourceArn`, `RestoreJobId`, `ScanJobId`

### Create

- Operations: `CreateBackupPlan`, `CreateBackupSelection`, `CreateBackupVault`, `CreateFramework`, `CreateLegalHold`, `CreateLogicallyAirGappedBackupVault`, `CreateReportPlan`, `CreateRestoreAccessBackupVault`, `CreateRestoreTestingPlan`, `CreateRestoreTestingSelection`, `CreateTieringConfiguration`
- Traits: `idempotency-token` (9), `idempotent` (11)
- Common required input members in this group: `BackupPlan`, `BackupPlanId`, `BackupSelection`, `BackupVaultName`, `Description`, `FrameworkControls`, `FrameworkName`, `MaxRetentionDays`, `MinRetentionDays`, `ReportDeliveryChannel`, `ReportPlanName`, `ReportSetting`, `RestoreTestingPlan`, `RestoreTestingPlanName`, `RestoreTestingSelection`, `SourceBackupVaultArn`, `TieringConfiguration`, `Title`

### Update

- Operations: `UpdateBackupPlan`, `UpdateFramework`, `UpdateGlobalSettings`, `UpdateRecoveryPointIndexSettings`, `UpdateRecoveryPointLifecycle`, `UpdateRegionSettings`, `UpdateReportPlan`, `UpdateRestoreTestingPlan`, `UpdateRestoreTestingSelection`, `UpdateTieringConfiguration`
- Traits: `idempotency-token` (2), `idempotent` (8)
- Common required input members in this group: `BackupPlan`, `BackupPlanId`, `BackupVaultName`, `FrameworkName`, `Index`, `RecoveryPointArn`, `ReportPlanName`, `RestoreTestingPlan`, `RestoreTestingPlanName`, `RestoreTestingSelection`, `RestoreTestingSelectionName`, `TieringConfiguration`, `TieringConfigurationName`

### Start

- Operations: `StartBackupJob`, `StartCopyJob`, `StartReportJob`, `StartRestoreJob`, `StartScanJob`
- Traits: `idempotency-token` (4), `idempotent` (5)
- Common required input members in this group: `BackupVaultName`, `DestinationBackupVaultArn`, `IamRoleArn`, `MalwareScanner`, `Metadata`, `RecoveryPointArn`, `ReportPlanName`, `ResourceArn`, `ScanMode`, `ScannerRoleArn`, `SourceBackupVaultName`

### Put

- Operations: `PutBackupVaultAccessPolicy`, `PutBackupVaultLockConfiguration`, `PutBackupVaultNotifications`, `PutRestoreValidationResult`
- Traits: `idempotent` (4)
- Common required input members in this group: `BackupVaultEvents`, `BackupVaultName`, `RestoreJobId`, `SNSTopicArn`, `ValidationStatus`

### Disassociate

- Operations: `DisassociateBackupVaultMpaApprovalTeam`, `DisassociateRecoveryPoint`, `DisassociateRecoveryPointFromParent`
- Common required input members in this group: `BackupVaultName`, `RecoveryPointArn`

### Associate

- Operations: `AssociateBackupVaultMpaApprovalTeam`
- Common required input members in this group: `BackupVaultName`, `MpaApprovalTeamArn`

### Cancel

- Operations: `CancelLegalHold`
- Traits: `idempotent` (1)
- Common required input members in this group: `CancelDescription`, `LegalHoldId`

### Export

- Operations: `ExportBackupPlanTemplate`
- Common required input members in this group: `BackupPlanId`

### Revoke

- Operations: `RevokeRestoreAccessBackupVault`
- Common required input members in this group: `BackupVaultName`, `RestoreAccessBackupVaultArn`

### Stop

- Operations: `StopBackupJob`
- Common required input members in this group: `BackupJobId`

### Tag

- Operations: `TagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `Tags`

### Untag

- Operations: `UntagResource`
- Traits: `idempotent` (1)
- Common required input members in this group: `ResourceArn`, `TagKeyList`

## Operation Detail Matrix

| Operation | HTTP | Traits | Required input | Idempotency tokens | Output | Errors | AWS documentation summary |
|---|---|---|---|---|---|---|---|
| `AssociateBackupVaultMpaApprovalTeam` | `PUT /backup-vaults/{BackupVaultName}/mpaApprovalTeam` | - | `BackupVaultName`, `MpaApprovalTeamArn` | - | `Unit` | `InvalidParameterValueException`, `InvalidRequestException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Associates an MPA approval team with a backup vault. |
| `CancelLegalHold` | `DELETE /legal-holds/{LegalHoldId}` | `idempotent` | `CancelDescription`, `LegalHoldId` | - | `CancelLegalHoldOutput` | `InvalidParameterValueException`, `InvalidResourceStateException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Removes the specified legal hold on a recovery point. This action can only be performed by a user with sufficient permissions. |
| `CreateBackupPlan` | `PUT /backup/plans` | `idempotent`, `idempotency-token` | `BackupPlan` | `CreatorRequestId` | `CreateBackupPlanOutput` | `AlreadyExistsException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingParameterValueException`, `ServiceUnavailableException` | Creates a backup plan using a backup plan name and backup rules. A backup plan is a document that contains information that Backup uses to schedule tasks that create recovery points for resources. |
| `CreateBackupSelection` | `PUT /backup/plans/{BackupPlanId}/selections` | `idempotent`, `idempotency-token` | `BackupPlanId`, `BackupSelection` | `CreatorRequestId` | `CreateBackupSelectionOutput` | `AlreadyExistsException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingParameterValueException`, `ServiceUnavailableException` | Creates a JSON document that specifies a set of resources to assign to a backup plan. For examples, see Assigning resources programmatically. |
| `CreateBackupVault` | `PUT /backup-vaults/{BackupVaultName}` | `idempotent`, `idempotency-token` | `BackupVaultName` | `CreatorRequestId` | `CreateBackupVaultOutput` | `AlreadyExistsException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingParameterValueException`, `ServiceUnavailableException` | Creates a logical container where backups are stored. A `CreateBackupVault` request includes a name, optionally one or more resource tags, an encryption key, and a request ID. |
| `CreateFramework` | `POST /audit/frameworks` | `idempotent`, `idempotency-token` | `FrameworkControls`, `FrameworkName` | `IdempotencyToken` | `CreateFrameworkOutput` | `AlreadyExistsException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingParameterValueException`, `ServiceUnavailableException` | Creates a framework with one or more controls. A framework is a collection of controls that you can use to evaluate your backup practices. |
| `CreateLegalHold` | `POST /legal-holds` | `idempotent`, `idempotency-token` | `Description`, `Title` | `IdempotencyToken` | `CreateLegalHoldOutput` | `InvalidParameterValueException`, `LimitExceededException`, `MissingParameterValueException`, `ServiceUnavailableException` | Creates a legal hold on a recovery point (backup). A legal hold is a restraint on altering or deleting a backup until an authorized user cancels the legal hold. |
| `CreateLogicallyAirGappedBackupVault` | `PUT /logically-air-gapped-backup-vaults/{BackupVaultName}` | `idempotent`, `idempotency-token` | `BackupVaultName`, `MaxRetentionDays`, `MinRetentionDays` | `CreatorRequestId` | `CreateLogicallyAirGappedBackupVaultOutput` | `AlreadyExistsException`, `InvalidParameterValueException`, `InvalidRequestException`, `LimitExceededException`, `MissingParameterValueException`, `ServiceUnavailableException` | Creates a logical container to where backups may be copied. This request includes a name, the Region, the maximum number of retention days, the minimum number of retention days, and optionally can include tags and a creator request ID. |
| `CreateReportPlan` | `POST /audit/report-plans` | `idempotent`, `idempotency-token` | `ReportDeliveryChannel`, `ReportPlanName`, `ReportSetting` | `IdempotencyToken` | `CreateReportPlanOutput` | `AlreadyExistsException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingParameterValueException`, `ServiceUnavailableException` | Creates a report plan. A report plan is a document that contains information about the contents of the report and where Backup will deliver it. |
| `CreateRestoreAccessBackupVault` | `PUT /restore-access-backup-vaults` | `idempotent`, `idempotency-token` | `SourceBackupVaultArn` | `CreatorRequestId` | `CreateRestoreAccessBackupVaultOutput` | `AlreadyExistsException`, `InvalidParameterValueException`, `InvalidRequestException`, `LimitExceededException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Creates a restore access backup vault that provides temporary access to recovery points in a logically air-gapped backup vault, subject to MPA approval. |
| `CreateRestoreTestingPlan` | `PUT /restore-testing/plans` | `idempotent` | `RestoreTestingPlan` | - | `CreateRestoreTestingPlanOutput` | `AlreadyExistsException`, `ConflictException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingParameterValueException`, `ServiceUnavailableException` | Creates a restore testing plan. The first of two steps to create a restore testing plan. |
| `CreateRestoreTestingSelection` | `PUT /restore-testing/plans/{RestoreTestingPlanName}/selections` | `idempotent` | `RestoreTestingPlanName`, `RestoreTestingSelection` | - | `CreateRestoreTestingSelectionOutput` | `AlreadyExistsException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This request can be sent after CreateRestoreTestingPlan request returns successfully. This is the second part of creating a resource testing plan, and it must be completed sequentially. |
| `CreateTieringConfiguration` | `PUT /tiering-configurations` | `idempotent`, `idempotency-token` | `TieringConfiguration` | `CreatorRequestId` | `CreateTieringConfigurationOutput` | `AlreadyExistsException`, `ConflictException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingParameterValueException`, `ServiceUnavailableException` | Creates a tiering configuration. A tiering configuration enables automatic movement of backup data to a lower-cost storage tier based on the age of backed-up objects in the backup vault. |
| `DeleteBackupPlan` | `DELETE /backup/plans/{BackupPlanId}` | - | `BackupPlanId` | - | `DeleteBackupPlanOutput` | `InvalidParameterValueException`, `InvalidRequestException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes a backup plan. A backup plan can only be deleted after all associated selections of resources have been deleted. |
| `DeleteBackupSelection` | `DELETE /backup/plans/{BackupPlanId}/selections/{SelectionId}` | - | `BackupPlanId`, `SelectionId` | - | `Unit` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the resource selection associated with a backup plan that is specified by the `SelectionId`. |
| `DeleteBackupVault` | `DELETE /backup-vaults/{BackupVaultName}` | `idempotent` | `BackupVaultName` | - | `Unit` | `InvalidParameterValueException`, `InvalidRequestException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the backup vault identified by its name. A vault can be deleted only if it is empty. |
| `DeleteBackupVaultAccessPolicy` | `DELETE /backup-vaults/{BackupVaultName}/access-policy` | `idempotent` | `BackupVaultName` | - | `Unit` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the policy document that manages permissions on a backup vault. |
| `DeleteBackupVaultLockConfiguration` | `DELETE /backup-vaults/{BackupVaultName}/vault-lock` | `idempotent` | `BackupVaultName` | - | `Unit` | `InvalidParameterValueException`, `InvalidRequestException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes Backup Vault Lock from a backup vault specified by a backup vault name. If the Vault Lock configuration is immutable, then you cannot delete Vault Lock using API operations, and you will receive an `InvalidRequestException` if you attempt to do so. |
| `DeleteBackupVaultNotifications` | `DELETE /backup-vaults/{BackupVaultName}/notification-configuration` | `idempotent` | `BackupVaultName` | - | `Unit` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes event notifications for the specified backup vault. |
| `DeleteFramework` | `DELETE /audit/frameworks/{FrameworkName}` | - | `FrameworkName` | - | `Unit` | `ConflictException`, `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the framework specified by a framework name. |
| `DeleteRecoveryPoint` | `DELETE /backup-vaults/{BackupVaultName}/recovery-points/{RecoveryPointArn}` | `idempotent` | `BackupVaultName`, `RecoveryPointArn` | - | `Unit` | `InvalidParameterValueException`, `InvalidRequestException`, `InvalidResourceStateException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the recovery point specified by a recovery point ID. If the recovery point ID belongs to a continuous backup, calling this endpoint deletes the existing continuous backup and stops future continuous backup. |
| `DeleteReportPlan` | `DELETE /audit/report-plans/{ReportPlanName}` | `idempotent` | `ReportPlanName` | - | `Unit` | `ConflictException`, `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the report plan specified by a report plan name. |
| `DeleteRestoreTestingPlan` | `DELETE /restore-testing/plans/{RestoreTestingPlanName}` | `idempotent` | `RestoreTestingPlanName` | - | `Unit` | `InvalidRequestException`, `ServiceUnavailableException` | This request deletes the specified restore testing plan. Deletion can only successfully occur if all associated restore testing selections are deleted first. |
| `DeleteRestoreTestingSelection` | `DELETE /restore-testing/plans/{RestoreTestingPlanName}/selections/{RestoreTestingSelectionName}` | `idempotent` | `RestoreTestingPlanName`, `RestoreTestingSelectionName` | - | `Unit` | `ResourceNotFoundException`, `ServiceUnavailableException` | Input the Restore Testing Plan name and Restore Testing Selection name. All testing selections associated with a restore testing plan must be deleted before the restore testing plan can be deleted. |
| `DeleteTieringConfiguration` | `DELETE /tiering-configurations/{TieringConfigurationName}` | `idempotent` | `TieringConfigurationName` | - | `DeleteTieringConfigurationOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the tiering configuration specified by a tiering configuration name. |
| `DescribeBackupJob` | `GET /backup-jobs/{BackupJobId}` | `idempotent` | `BackupJobId` | - | `DescribeBackupJobOutput` | `DependencyFailureException`, `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns backup job details for the specified `BackupJobId`. |
| `DescribeBackupVault` | `GET /backup-vaults/{BackupVaultName}` | `idempotent` | `BackupVaultName` | - | `DescribeBackupVaultOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns metadata about a backup vault specified by its name. |
| `DescribeCopyJob` | `GET /copy-jobs/{CopyJobId}` | `idempotent` | `CopyJobId` | - | `DescribeCopyJobOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns metadata associated with creating a copy of a resource. |
| `DescribeFramework` | `GET /audit/frameworks/{FrameworkName}` | - | `FrameworkName` | - | `DescribeFrameworkOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns the framework details for the specified `FrameworkName`. |
| `DescribeGlobalSettings` | `GET /global-settings` | - | - | - | `DescribeGlobalSettingsOutput` | `InvalidRequestException`, `ServiceUnavailableException` | Describes whether the Amazon Web Services account is opted in to cross-account backup. Returns an error if the account is not a member of an Organizations organization. |
| `DescribeProtectedResource` | `GET /resources/{ResourceArn}` | `idempotent` | `ResourceArn` | - | `DescribeProtectedResourceOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns information about a saved resource, including the last time it was backed up, its Amazon Resource Name (ARN), and the Amazon Web Services service type of the saved resource. |
| `DescribeRecoveryPoint` | `GET /backup-vaults/{BackupVaultName}/recovery-points/{RecoveryPointArn}` | `idempotent` | `BackupVaultName`, `RecoveryPointArn` | - | `DescribeRecoveryPointOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns metadata associated with a recovery point, including ID, status, encryption, and lifecycle. |
| `DescribeRegionSettings` | `GET /account-settings` | - | - | - | `DescribeRegionSettingsOutput` | `ServiceUnavailableException` | Returns the current service opt-in settings for the Region. If service opt-in is enabled for a service, Backup tries to protect that service's resources in this Region, when the resource is included in an on-demand backup or scheduled backup plan. |
| `DescribeReportJob` | `GET /audit/report-jobs/{ReportJobId}` | - | `ReportJobId` | - | `DescribeReportJobOutput` | `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns the details associated with creating a report as specified by its `ReportJobId`. |
| `DescribeReportPlan` | `GET /audit/report-plans/{ReportPlanName}` | - | `ReportPlanName` | - | `DescribeReportPlanOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns a list of all report plans for an Amazon Web Services account and Amazon Web Services Region. |
| `DescribeRestoreJob` | `GET /restore-jobs/{RestoreJobId}` | `idempotent` | `RestoreJobId` | - | `DescribeRestoreJobOutput` | `DependencyFailureException`, `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns metadata associated with a restore job that is specified by a job ID. |
| `DescribeScanJob` | `GET /scan/jobs/{ScanJobId}` | `idempotent` | `ScanJobId` | - | `DescribeScanJobOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns scan job details for the specified ScanJobID. |
| `DisassociateBackupVaultMpaApprovalTeam` | `POST /backup-vaults/{BackupVaultName}/mpaApprovalTeam?delete` | - | `BackupVaultName` | - | `Unit` | `InvalidParameterValueException`, `InvalidRequestException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Removes the association between an MPA approval team and a backup vault, disabling the MPA approval workflow for restore operations. |
| `DisassociateRecoveryPoint` | `POST /backup-vaults/{BackupVaultName}/recovery-points/{RecoveryPointArn}/disassociate` | - | `BackupVaultName`, `RecoveryPointArn` | - | `Unit` | `InvalidParameterValueException`, `InvalidRequestException`, `InvalidResourceStateException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Deletes the specified continuous backup recovery point from Backup and releases control of that continuous backup to the source service, such as Amazon RDS. The source service will continue to create and retain continuous backups using the lifecycle that you... |
| `DisassociateRecoveryPointFromParent` | `DELETE /backup-vaults/{BackupVaultName}/recovery-points/{RecoveryPointArn}/parentAssociation` | - | `BackupVaultName`, `RecoveryPointArn` | - | `Unit` | `InvalidParameterValueException`, `InvalidRequestException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This action to a specific child (nested) recovery point removes the relationship between the specified recovery point and its parent (composite) recovery point. |
| `ExportBackupPlanTemplate` | `GET /backup/plans/{BackupPlanId}/toTemplate` | - | `BackupPlanId` | - | `ExportBackupPlanTemplateOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns the backup plan that is specified by the plan ID as a backup template. |
| `GetBackupPlan` | `GET /backup/plans/{BackupPlanId}` | `idempotent` | `BackupPlanId` | - | `GetBackupPlanOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns `BackupPlan` details for the specified `BackupPlanId`. The details are the body of a backup plan in JSON format, in addition to plan metadata. |
| `GetBackupPlanFromJSON` | `POST /backup/template/json/toPlan` | - | `BackupPlanTemplateJson` | - | `GetBackupPlanFromJSONOutput` | `InvalidParameterValueException`, `InvalidRequestException`, `LimitExceededException`, `MissingParameterValueException`, `ServiceUnavailableException` | Returns a valid JSON document specifying a backup plan or an error. |
| `GetBackupPlanFromTemplate` | `GET /backup/template/plans/{BackupPlanTemplateId}/toPlan` | - | `BackupPlanTemplateId` | - | `GetBackupPlanFromTemplateOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns the template specified by its `templateId` as a backup plan. |
| `GetBackupSelection` | `GET /backup/plans/{BackupPlanId}/selections/{SelectionId}` | `idempotent` | `BackupPlanId`, `SelectionId` | - | `GetBackupSelectionOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns selection metadata and a document in JSON format that specifies a list of resources that are associated with a backup plan. |
| `GetBackupVaultAccessPolicy` | `GET /backup-vaults/{BackupVaultName}/access-policy` | `idempotent` | `BackupVaultName` | - | `GetBackupVaultAccessPolicyOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns the access policy document that is associated with the named backup vault. |
| `GetBackupVaultNotifications` | `GET /backup-vaults/{BackupVaultName}/notification-configuration` | `idempotent` | `BackupVaultName` | - | `GetBackupVaultNotificationsOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns event notifications for the specified backup vault. |
| `GetLegalHold` | `GET /legal-holds/{LegalHoldId}` | `idempotent` | `LegalHoldId` | - | `GetLegalHoldOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This action returns details for a specified legal hold. The details are the body of a legal hold in JSON format, in addition to metadata. |
| `GetRecoveryPointIndexDetails` | `GET /backup-vaults/{BackupVaultName}/recovery-points/{RecoveryPointArn}/index` | `idempotent` | `BackupVaultName`, `RecoveryPointArn` | - | `GetRecoveryPointIndexDetailsOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation returns the metadata and details specific to the backup index associated with the specified recovery point. |
| `GetRecoveryPointRestoreMetadata` | `GET /backup-vaults/{BackupVaultName}/recovery-points/{RecoveryPointArn}/restore-metadata` | `idempotent` | `BackupVaultName`, `RecoveryPointArn` | - | `GetRecoveryPointRestoreMetadataOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns a set of metadata key-value pairs that were used to create the backup. |
| `GetRestoreJobMetadata` | `GET /restore-jobs/{RestoreJobId}/metadata` | - | `RestoreJobId` | - | `GetRestoreJobMetadataOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This request returns the metadata for the specified restore job. |
| `GetRestoreTestingInferredMetadata` | `GET /restore-testing/inferred-metadata` | - | `BackupVaultName`, `RecoveryPointArn` | - | `GetRestoreTestingInferredMetadataOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This request returns the minimal required set of metadata needed to start a restore job with secure default settings. `BackupVaultName` and `RecoveryPointArn` are required parameters. |
| `GetRestoreTestingPlan` | `GET /restore-testing/plans/{RestoreTestingPlanName}` | - | `RestoreTestingPlanName` | - | `GetRestoreTestingPlanOutput` | `ResourceNotFoundException`, `ServiceUnavailableException` | Returns `RestoreTestingPlan` details for the specified `RestoreTestingPlanName`. The details are the body of a restore testing plan in JSON format, in addition to plan metadata. |
| `GetRestoreTestingSelection` | `GET /restore-testing/plans/{RestoreTestingPlanName}/selections/{RestoreTestingSelectionName}` | - | `RestoreTestingPlanName`, `RestoreTestingSelectionName` | - | `GetRestoreTestingSelectionOutput` | `ResourceNotFoundException`, `ServiceUnavailableException` | Returns RestoreTestingSelection, which displays resources and elements of the restore testing plan. |
| `GetSupportedResourceTypes` | `GET /supported-resource-types` | - | - | - | `GetSupportedResourceTypesOutput` | `ServiceUnavailableException` | Returns the Amazon Web Services resource types supported by Backup. |
| `GetTieringConfiguration` | `GET /tiering-configurations/{TieringConfigurationName}` | `idempotent` | `TieringConfigurationName` | - | `GetTieringConfigurationOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns `TieringConfiguration` details for the specified `TieringConfigurationName`. The details are the body of a tiering configuration in JSON format, in addition to configuration metadata. |
| `ListBackupJobSummaries` | `GET /audit/backup-job-summaries` | `paginated` | - | - | `ListBackupJobSummariesOutput` | `InvalidParameterValueException`, `ServiceUnavailableException` | This is a request for a summary of backup jobs created or running within the most recent 30 days. You can include parameters AccountID, State, ResourceType, MessageCategory, AggregationPeriod, MaxResults, or NextToken to filter results. |
| `ListBackupJobs` | `GET /backup-jobs` | `idempotent`, `paginated` | - | - | `ListBackupJobsOutput` | `InvalidParameterValueException`, `ServiceUnavailableException` | Returns a list of existing backup jobs for an authenticated account for the last 30 days. For a longer period of time, consider using these monitoring tools. |
| `ListBackupPlanTemplates` | `GET /backup/template/plans` | `paginated` | - | - | `ListBackupPlanTemplatesOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Lists the backup plan templates. |
| `ListBackupPlanVersions` | `GET /backup/plans/{BackupPlanId}/versions` | `idempotent`, `paginated` | `BackupPlanId` | - | `ListBackupPlanVersionsOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns version metadata of your backup plans, including Amazon Resource Names (ARNs), backup plan IDs, creation and deletion dates, plan names, and version IDs. |
| `ListBackupPlans` | `GET /backup/plans` | `idempotent`, `paginated` | - | - | `ListBackupPlansOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Lists the active backup plans for the account. |
| `ListBackupSelections` | `GET /backup/plans/{BackupPlanId}/selections` | `idempotent`, `paginated` | `BackupPlanId` | - | `ListBackupSelectionsOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns an array containing metadata of the resources associated with the target backup plan. |
| `ListBackupVaults` | `GET /backup-vaults` | `idempotent`, `paginated` | - | - | `ListBackupVaultsOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns a list of recovery point storage containers along with information about them. |
| `ListCopyJobSummaries` | `GET /audit/copy-job-summaries` | `paginated` | - | - | `ListCopyJobSummariesOutput` | `InvalidParameterValueException`, `ServiceUnavailableException` | This request obtains a list of copy jobs created or running within the the most recent 30 days. You can include parameters AccountID, State, ResourceType, MessageCategory, AggregationPeriod, MaxResults, or NextToken to filter results. |
| `ListCopyJobs` | `GET /copy-jobs` | `paginated` | - | - | `ListCopyJobsOutput` | `InvalidParameterValueException`, `ServiceUnavailableException` | Returns metadata about your copy jobs. |
| `ListFrameworks` | `GET /audit/frameworks` | `paginated` | - | - | `ListFrameworksOutput` | `InvalidParameterValueException`, `ServiceUnavailableException` | Returns a list of all frameworks for an Amazon Web Services account and Amazon Web Services Region. |
| `ListIndexedRecoveryPoints` | `GET /indexes/recovery-point` | `idempotent`, `paginated` | - | - | `ListIndexedRecoveryPointsOutput` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation returns a list of recovery points that have an associated index, belonging to the specified account. Optional parameters you can include are: MaxResults; NextToken; SourceResourceArns; CreatedBefore; CreatedAfter; and ResourceType. |
| `ListLegalHolds` | `GET /legal-holds` | `idempotent`, `paginated` | - | - | `ListLegalHoldsOutput` | `InvalidParameterValueException`, `ServiceUnavailableException` | This action returns metadata about active and previous legal holds. |
| `ListProtectedResources` | `GET /resources` | `idempotent`, `paginated` | - | - | `ListProtectedResourcesOutput` | `InvalidParameterValueException`, `ServiceUnavailableException` | Returns an array of resources successfully backed up by Backup, including the time the resource was saved, an Amazon Resource Name (ARN) of the resource, and a resource type. |
| `ListProtectedResourcesByBackupVault` | `GET /backup-vaults/{BackupVaultName}/resources` | `paginated` | `BackupVaultName` | - | `ListProtectedResourcesByBackupVaultOutput` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This request lists the protected resources corresponding to each backup vault. |
| `ListRecoveryPointsByBackupVault` | `GET /backup-vaults/{BackupVaultName}/recovery-points` | `idempotent`, `paginated` | `BackupVaultName` | - | `ListRecoveryPointsByBackupVaultOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns detailed information about the recovery points stored in a backup vault. |
| `ListRecoveryPointsByLegalHold` | `GET /legal-holds/{LegalHoldId}/recovery-points` | `idempotent`, `paginated` | `LegalHoldId` | - | `ListRecoveryPointsByLegalHoldOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ServiceUnavailableException` | This action returns recovery point ARNs (Amazon Resource Names) of the specified legal hold. |
| `ListRecoveryPointsByResource` | `GET /resources/{ResourceArn}/recovery-points` | `idempotent`, `paginated` | `ResourceArn` | - | `ListRecoveryPointsByResourceOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | The information about the recovery points of the type specified by a resource Amazon Resource Name (ARN). For Amazon EFS and Amazon EC2, this action only lists recovery points created by Backup. |
| `ListReportJobs` | `GET /audit/report-jobs` | `paginated` | - | - | `ListReportJobsOutput` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns details about your report jobs. |
| `ListReportPlans` | `GET /audit/report-plans` | `paginated` | - | - | `ListReportPlansOutput` | `InvalidParameterValueException`, `ServiceUnavailableException` | Returns a list of your report plans. For detailed information about a single report plan, use `DescribeReportPlan`. |
| `ListRestoreAccessBackupVaults` | `GET /logically-air-gapped-backup-vaults/{BackupVaultName}/restore-access-backup-vaults` | `paginated` | `BackupVaultName` | - | `ListRestoreAccessBackupVaultsOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns a list of restore access backup vaults associated with a specified backup vault. |
| `ListRestoreJobSummaries` | `GET /audit/restore-job-summaries` | `paginated` | - | - | `ListRestoreJobSummariesOutput` | `InvalidParameterValueException`, `ServiceUnavailableException` | This request obtains a summary of restore jobs created or running within the the most recent 30 days. You can include parameters AccountID, State, ResourceType, AggregationPeriod, MaxResults, or NextToken to filter results. |
| `ListRestoreJobs` | `GET /restore-jobs` | `idempotent`, `paginated` | - | - | `ListRestoreJobsOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns a list of jobs that Backup initiated to restore a saved resource, including details about the recovery process. |
| `ListRestoreJobsByProtectedResource` | `GET /resources/{ResourceArn}/restore-jobs` | `paginated` | `ResourceArn` | - | `ListRestoreJobsByProtectedResourceOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This returns restore jobs that contain the specified protected resource. You must include `ResourceArn`. |
| `ListRestoreTestingPlans` | `GET /restore-testing/plans` | `paginated` | - | - | `ListRestoreTestingPlansOutput` | `InvalidParameterValueException`, `ServiceUnavailableException` | Returns a list of restore testing plans. |
| `ListRestoreTestingSelections` | `GET /restore-testing/plans/{RestoreTestingPlanName}/selections` | `paginated` | `RestoreTestingPlanName` | - | `ListRestoreTestingSelectionsOutput` | `InvalidParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns a list of restore testing selections. Can be filtered by `MaxResults` and `RestoreTestingPlanName`. |
| `ListScanJobSummaries` | `GET /audit/scan-job-summaries` | `paginated` | - | - | `ListScanJobSummariesOutput` | `InvalidParameterValueException`, `ServiceUnavailableException` | This is a request for a summary of scan jobs created or running within the most recent 30 days. |
| `ListScanJobs` | `GET /scan/jobs` | `idempotent`, `paginated` | - | - | `ListScanJobsOutput` | `InvalidParameterValueException`, `ServiceUnavailableException` | Returns a list of existing scan jobs for an authenticated account for the last 30 days. |
| `ListTags` | `GET /tags/{ResourceArn}` | `idempotent`, `paginated` | `ResourceArn` | - | `ListTagsOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Returns the tags assigned to the resource, such as a target recovery point, backup plan, or backup vault. This operation returns results depending on the resource type used in the value for `resourceArn`. |
| `ListTieringConfigurations` | `GET /tiering-configurations` | `idempotent`, `paginated` | - | - | `ListTieringConfigurationsOutput` | `InvalidParameterValueException`, `ServiceUnavailableException` | Returns a list of tiering configurations. |
| `PutBackupVaultAccessPolicy` | `PUT /backup-vaults/{BackupVaultName}/access-policy` | `idempotent` | `BackupVaultName` | - | `Unit` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Sets a resource-based policy that is used to manage access permissions on the target backup vault. Requires a backup vault name and an access policy document in JSON format. |
| `PutBackupVaultLockConfiguration` | `PUT /backup-vaults/{BackupVaultName}/vault-lock` | `idempotent` | `BackupVaultName` | - | `Unit` | `InvalidParameterValueException`, `InvalidRequestException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Applies Backup Vault Lock to a backup vault, preventing attempts to delete any recovery point stored in or created in a backup vault. Vault Lock also prevents attempts to update the lifecycle policy that controls the retention period of any recovery point... |
| `PutBackupVaultNotifications` | `PUT /backup-vaults/{BackupVaultName}/notification-configuration` | `idempotent` | `BackupVaultEvents`, `BackupVaultName`, `SNSTopicArn` | - | `Unit` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Turns on notifications on a backup vault for the specified topic and events. |
| `PutRestoreValidationResult` | `PUT /restore-jobs/{RestoreJobId}/validations` | `idempotent` | `RestoreJobId`, `ValidationStatus` | - | `Unit` | `InvalidParameterValueException`, `InvalidRequestException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This request allows you to send your independent self-run restore test validation results. `RestoreJobId` and `ValidationStatus` are required. |
| `RevokeRestoreAccessBackupVault` | `DELETE /logically-air-gapped-backup-vaults/{BackupVaultName}/restore-access-backup-vaults/{RestoreAccessBackupVaultArn}` | - | `BackupVaultName`, `RestoreAccessBackupVaultArn` | - | `Unit` | `InvalidParameterValueException`, `InvalidRequestException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Revokes access to a restore access backup vault, removing the ability to restore from its recovery points and permanently deleting the vault. |
| `StartBackupJob` | `PUT /backup-jobs` | `idempotent`, `idempotency-token` | `BackupVaultName`, `IamRoleArn`, `ResourceArn` | `IdempotencyToken` | `StartBackupJobOutput` | `InvalidParameterValueException`, `InvalidRequestException`, `LimitExceededException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Starts an on-demand backup job for the specified resource. |
| `StartCopyJob` | `PUT /copy-jobs` | `idempotent`, `idempotency-token` | `DestinationBackupVaultArn`, `IamRoleArn`, `RecoveryPointArn`, `SourceBackupVaultName` | `IdempotencyToken` | `StartCopyJobOutput` | `InvalidParameterValueException`, `InvalidRequestException`, `LimitExceededException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Starts a job to create a one-time copy of the specified resource. Does not support continuous backups. |
| `StartReportJob` | `POST /audit/report-jobs/{ReportPlanName}` | `idempotent`, `idempotency-token` | `ReportPlanName` | `IdempotencyToken` | `StartReportJobOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Starts an on-demand report job for the specified report plan. |
| `StartRestoreJob` | `PUT /restore-jobs` | `idempotent`, `idempotency-token` | `Metadata`, `RecoveryPointArn` | `IdempotencyToken` | `StartRestoreJobOutput` | `InvalidParameterValueException`, `InvalidRequestException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Recovers the saved resource identified by an Amazon Resource Name (ARN). |
| `StartScanJob` | `PUT /scan/job` | `idempotent` | `BackupVaultName`, `IamRoleArn`, `MalwareScanner`, `RecoveryPointArn`, `ScanMode`, `ScannerRoleArn` | - | `StartScanJobOutput` | `InvalidParameterValueException`, `InvalidRequestException`, `LimitExceededException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Starts scanning jobs for specific resources. |
| `StopBackupJob` | `POST /backup-jobs/{BackupJobId}` | - | `BackupJobId` | - | `Unit` | `InvalidParameterValueException`, `InvalidRequestException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Attempts to cancel a job to create a one-time backup of a resource. This action is not supported for the following services: Amazon Aurora Amazon DocumentDB (with MongoDB compatibility) Amazon FSx for Lustre Amazon FSx for NetApp ONTAP Amazon FSx for OpenZFS... |
| `TagResource` | `POST /tags/{ResourceArn}` | `idempotent` | `ResourceArn`, `Tags` | - | `Unit` | `InvalidParameterValueException`, `LimitExceededException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Assigns a set of key-value pairs to a resource. |
| `UntagResource` | `POST /untag/{ResourceArn}` | `idempotent` | `ResourceArn`, `TagKeyList` | - | `Unit` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Removes a set of key-value pairs from a recovery point, backup plan, or backup vault identified by an Amazon Resource Name (ARN) This API is not supported for recovery points for resource types including Aurora, Amazon DocumentDB. Amazon EBS, Amazon FSx... |
| `UpdateBackupPlan` | `POST /backup/plans/{BackupPlanId}` | `idempotent` | `BackupPlan`, `BackupPlanId` | - | `UpdateBackupPlanOutput` | `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Updates the specified backup plan. The new version is uniquely identified by its ID. |
| `UpdateFramework` | `PUT /audit/frameworks/{FrameworkName}` | `idempotent`, `idempotency-token` | `FrameworkName` | `IdempotencyToken` | `UpdateFrameworkOutput` | `AlreadyExistsException`, `ConflictException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Updates the specified framework. |
| `UpdateGlobalSettings` | `PUT /global-settings` | - | - | - | `Unit` | `InvalidParameterValueException`, `InvalidRequestException`, `MissingParameterValueException`, `ServiceUnavailableException` | Updates whether the Amazon Web Services account is opted in to cross-account backup. Returns an error if the account is not an Organizations management account. |
| `UpdateRecoveryPointIndexSettings` | `POST /backup-vaults/{BackupVaultName}/recovery-points/{RecoveryPointArn}/index` | `idempotent` | `BackupVaultName`, `Index`, `RecoveryPointArn` | - | `UpdateRecoveryPointIndexSettingsOutput` | `InvalidParameterValueException`, `InvalidRequestException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This operation updates the settings of a recovery point index. Required: BackupVaultName, RecoveryPointArn, and IAMRoleArn |
| `UpdateRecoveryPointLifecycle` | `POST /backup-vaults/{BackupVaultName}/recovery-points/{RecoveryPointArn}` | `idempotent` | `BackupVaultName`, `RecoveryPointArn` | - | `UpdateRecoveryPointLifecycleOutput` | `InvalidParameterValueException`, `InvalidRequestException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Sets the transition lifecycle of a recovery point. The lifecycle defines when a protected resource is transitioned to cold storage and when it expires. |
| `UpdateRegionSettings` | `PUT /account-settings` | - | - | - | `Unit` | `InvalidParameterValueException`, `MissingParameterValueException`, `ServiceUnavailableException` | Updates the current service opt-in settings for the Region. Use the `DescribeRegionSettings` API to determine the resource types that are supported. |
| `UpdateReportPlan` | `PUT /audit/report-plans/{ReportPlanName}` | `idempotent`, `idempotency-token` | `ReportPlanName` | `IdempotencyToken` | `UpdateReportPlanOutput` | `ConflictException`, `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Updates the specified report plan. |
| `UpdateRestoreTestingPlan` | `PUT /restore-testing/plans/{RestoreTestingPlanName}` | `idempotent` | `RestoreTestingPlan`, `RestoreTestingPlanName` | - | `UpdateRestoreTestingPlanOutput` | `ConflictException`, `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This request will send changes to your specified restore testing plan. `RestoreTestingPlanName` cannot be updated after it is created. |
| `UpdateRestoreTestingSelection` | `PUT /restore-testing/plans/{RestoreTestingPlanName}/selections/{RestoreTestingSelectionName}` | `idempotent` | `RestoreTestingPlanName`, `RestoreTestingSelection`, `RestoreTestingSelectionName` | - | `UpdateRestoreTestingSelectionOutput` | `ConflictException`, `InvalidParameterValueException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | Updates the specified restore testing selection. Most elements except the `RestoreTestingSelectionName` can be updated with this request. |
| `UpdateTieringConfiguration` | `PUT /tiering-configurations/{TieringConfigurationName}` | `idempotent` | `TieringConfiguration`, `TieringConfigurationName` | - | `UpdateTieringConfigurationOutput` | `AlreadyExistsException`, `ConflictException`, `InvalidParameterValueException`, `LimitExceededException`, `MissingParameterValueException`, `ResourceNotFoundException`, `ServiceUnavailableException` | This request will send changes to your specified tiering configuration. `TieringConfigurationName` cannot be updated after it is created. |

## Important Shapes

| Shape | Type | Members | Documentation cue |
|---|---|---|---|
| `ServiceUnavailableException` | `structure` | `Code`, `Context`, `Message`, `Type` | The request failed due to a temporary failure of the server. |
| `InvalidParameterValueException` | `structure` | `Code`, `Context`, `Message`, `Type` | Indicates that something is wrong with a parameter's value. |
| `MissingParameterValueException` | `structure` | `Code`, `Context`, `Message`, `Type` | Indicates that a required parameter is missing. |
| `ResourceNotFoundException` | `structure` | `Code`, `Context`, `Message`, `Type` | A resource that is required for the action doesn't exist. |
| `InvalidRequestException` | `structure` | `Code`, `Context`, `Message`, `Type` | Indicates that something is wrong with the input to the request. |
| `LimitExceededException` | `structure` | `Code`, `Context`, `Message`, `Type` | A limit in the request has been exceeded; for example, a maximum number of items allowed in a request. |
| `AlreadyExistsException` | `structure` | `Arn`, `Code`, `Context`, `CreatorRequestId`, `Message`, `Type` | The required resource already exists. |
| `ConflictException` | `structure` | `Code`, `Context`, `Message`, `Type` | Backup can't perform the action that you requested until it finishes performing a previous action. |
| `InvalidResourceStateException` | `structure` | `Code`, `Context`, `Message`, `Type` | Backup is already performing an action on this recovery point. |
| `DependencyFailureException` | `structure` | `Code`, `Context`, `Message`, `Type` | A dependent Amazon Web Services service or resource returned an error to the Backup service, and the action cannot be completed. |
| `AssociateBackupVaultMpaApprovalTeamInput` | `structure` | `BackupVaultName`, `MpaApprovalTeamArn`, `RequesterComment` | - |
| `CancelLegalHoldInput` | `structure` | `CancelDescription`, `LegalHoldId`, `RetainRecordInDays` | - |
| `CancelLegalHoldOutput` | `structure` | - | - |
| `CreateBackupPlanInput` | `structure` | `BackupPlan`, `BackupPlanTags`, `CreatorRequestId` | - |
| `CreateBackupPlanOutput` | `structure` | `AdvancedBackupSettings`, `BackupPlanArn`, `BackupPlanId`, `CreationDate`, `VersionId` | - |
| `CreateBackupSelectionInput` | `structure` | `BackupPlanId`, `BackupSelection`, `CreatorRequestId` | - |
| `CreateBackupSelectionOutput` | `structure` | `BackupPlanId`, `CreationDate`, `SelectionId` | - |
| `CreateBackupVaultInput` | `structure` | `BackupVaultName`, `BackupVaultTags`, `CreatorRequestId`, `EncryptionKeyArn` | - |
| `CreateBackupVaultOutput` | `structure` | `BackupVaultArn`, `BackupVaultName`, `CreationDate` | - |
| `CreateFrameworkInput` | `structure` | `FrameworkControls`, `FrameworkDescription`, `FrameworkName`, `FrameworkTags`, `IdempotencyToken` | - |
| `CreateFrameworkOutput` | `structure` | `FrameworkArn`, `FrameworkName` | - |
| `CreateLegalHoldInput` | `structure` | `Description`, `IdempotencyToken`, `RecoveryPointSelection`, `Tags`, `Title` | - |
| `CreateLegalHoldOutput` | `structure` | `CreationDate`, `Description`, `LegalHoldArn`, `LegalHoldId`, `RecoveryPointSelection`, `Status`, `Title` | - |

## Research Checklist for Parity Work

- Confirm lifecycle transitions for every create/update/delete/start/stop operation.
- Confirm exact not-found, already-exists, conflict, validation, throttling, and access-denied error names.
- Confirm pagination token format, result ordering, default limits, and empty collection shape.
- Confirm idempotency-token behaviour, especially mismatched replay parameters.
- Confirm cross-service identifiers such as ARNs, IAM roles, KMS keys, S3 buckets, VPC resources, and logging destinations.
- Confirm whether read APIs are derived from customer-managed state, AWS-managed catalogues, telemetry, recommendations, or asynchronous jobs.
