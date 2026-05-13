# winterbaume-backup

AWS Backup service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Backup |
| AWS model | `backup` |
| Protocol | restJson1 |
| winterbaume coverage | 105/108 operations (97.2%) |
| stubs (routed, returns empty/default) | 3/108 operations (2.8%) |
| moto coverage | 17/108 operations (15.7%) |
| floci coverage | 0/108 operations (0.0%) |
| kumo coverage | 12/108 operations (11.1%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws backup list-backup-plans
```

## Example

```rust
use aws_sdk_backup::config::BehaviorVersion;
use winterbaume_backup::BackupService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BackupService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_backup::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_backup::Client::new(&config);

    let resp = client
        .list_backup_vaults()
        .send()
        .await
        .expect("list_backup_vaults should succeed");
    println!("Backup vaults: {}", resp.backup_vault_list().len());
}
```

## Implemented APIs (105)

- `AssociateBackupVaultMpaApprovalTeam`
- `CancelLegalHold`
- `CreateBackupPlan`
- `CreateBackupSelection`
- `CreateBackupVault`
- `CreateFramework`
- `CreateLegalHold`
- `CreateLogicallyAirGappedBackupVault`
- `CreateReportPlan`
- `CreateRestoreAccessBackupVault`
- `CreateRestoreTestingPlan`
- `CreateRestoreTestingSelection`
- `CreateTieringConfiguration`
- `DeleteBackupPlan`
- `DeleteBackupSelection`
- `DeleteBackupVault`
- `DeleteBackupVaultAccessPolicy`
- `DeleteBackupVaultLockConfiguration`
- `DeleteBackupVaultNotifications`
- `DeleteFramework`
- `DeleteRecoveryPoint`
- `DeleteReportPlan`
- `DeleteRestoreTestingPlan`
- `DeleteRestoreTestingSelection`
- `DeleteTieringConfiguration`
- `DescribeBackupJob`
- `DescribeBackupVault`
- `DescribeCopyJob`
- `DescribeFramework`
- `DescribeGlobalSettings`
- `DescribeProtectedResource`
- `DescribeRecoveryPoint`
- `DescribeRegionSettings`
- `DescribeReportJob`
- `DescribeReportPlan`
- `DescribeRestoreJob`
- `DescribeScanJob`
- `DisassociateBackupVaultMpaApprovalTeam`
- `DisassociateRecoveryPoint`
- `DisassociateRecoveryPointFromParent`
- `ExportBackupPlanTemplate`
- `GetBackupPlan`
- `GetBackupPlanFromJSON`
- `GetBackupSelection`
- `GetBackupVaultAccessPolicy`
- `GetBackupVaultNotifications`
- `GetLegalHold`
- `GetRecoveryPointIndexDetails`
- `GetRecoveryPointRestoreMetadata`
- `GetRestoreJobMetadata`
- `GetRestoreTestingInferredMetadata`
- `GetRestoreTestingPlan`
- `GetRestoreTestingSelection`
- `GetSupportedResourceTypes`
- `GetTieringConfiguration`
- `ListBackupJobSummaries`
- `ListBackupJobs`
- `ListBackupPlanVersions`
- `ListBackupPlans`
- `ListBackupSelections`
- `ListBackupVaults`
- `ListCopyJobSummaries`
- `ListCopyJobs`
- `ListFrameworks`
- `ListIndexedRecoveryPoints`
- `ListLegalHolds`
- `ListProtectedResources`
- `ListProtectedResourcesByBackupVault`
- `ListRecoveryPointsByBackupVault`
- `ListRecoveryPointsByLegalHold`
- `ListRecoveryPointsByResource`
- `ListReportJobs`
- `ListReportPlans`
- `ListRestoreJobSummaries`
- `ListRestoreJobs`
- `ListRestoreJobsByProtectedResource`
- `ListRestoreTestingPlans`
- `ListRestoreTestingSelections`
- `ListScanJobSummaries`
- `ListScanJobs`
- `ListTags`
- `ListTieringConfigurations`
- `PutBackupVaultAccessPolicy`
- `PutBackupVaultLockConfiguration`
- `PutBackupVaultNotifications`
- `PutRestoreValidationResult`
- `RevokeRestoreAccessBackupVault`
- `StartBackupJob`
- `StartCopyJob`
- `StartReportJob`
- `StartRestoreJob`
- `StartScanJob`
- `StopBackupJob`
- `TagResource`
- `UntagResource`
- `UpdateBackupPlan`
- `UpdateFramework`
- `UpdateGlobalSettings`
- `UpdateRecoveryPointIndexSettings`
- `UpdateRecoveryPointLifecycle`
- `UpdateRegionSettings`
- `UpdateReportPlan`
- `UpdateRestoreTestingPlan`
- `UpdateRestoreTestingSelection`
- `UpdateTieringConfiguration`

<details><summary>Stubbed APIs (3) &mdash; routed but return an empty/default response</summary>

- `GetBackupPlanFromTemplate`
- `ListBackupPlanTemplates`
- `ListRestoreAccessBackupVaults`

</details>
