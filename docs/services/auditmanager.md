# winterbaume-auditmanager

Audit Manager service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Audit Manager |
| AWS model | `auditmanager` |
| Protocol | restJson1 |
| winterbaume coverage | 15/62 operations (24.2%) |
| stubs (routed, returns empty/default) | 0/62 operations (0.0%) |
| moto coverage | 0/62 operations (0.0%) |
| floci coverage | 0/62 operations (0.0%) |
| kumo coverage | 0/62 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws auditmanager help
```

## Example

```rust
use aws_sdk_auditmanager::config::BehaviorVersion;
use winterbaume_auditmanager::AuditManagerService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AuditManagerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_auditmanager::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_auditmanager::Client::new(&config);

    let resp = client
        .list_controls()
        .control_type(aws_sdk_auditmanager::types::ControlType::Custom)
        .send()
        .await
        .expect("list_controls should succeed");
    println!("Audit Manager controls: {:?}", resp.control_metadata_list());
}
```

## Implemented APIs (15)

- `CreateAssessment`
- `CreateAssessmentFramework`
- `CreateControl`
- `DeleteAssessment`
- `DeleteAssessmentFramework`
- `DeleteControl`
- `DeregisterAccount`
- `GetAccountStatus`
- `GetAssessment`
- `GetAssessmentFramework`
- `GetControl`
- `ListAssessmentFrameworks`
- `ListAssessments`
- `ListControls`
- `RegisterAccount`

<details><summary>Not yet implemented APIs (47)</summary>

- `AssociateAssessmentReportEvidenceFolder`
- `BatchAssociateAssessmentReportEvidence`
- `BatchCreateDelegationByAssessment`
- `BatchDeleteDelegationByAssessment`
- `BatchDisassociateAssessmentReportEvidence`
- `BatchImportEvidenceToAssessmentControl`
- `CreateAssessmentReport`
- `DeleteAssessmentFrameworkShare`
- `DeleteAssessmentReport`
- `DeregisterOrganizationAdminAccount`
- `DisassociateAssessmentReportEvidenceFolder`
- `GetAssessmentReportUrl`
- `GetChangeLogs`
- `GetDelegations`
- `GetEvidence`
- `GetEvidenceByEvidenceFolder`
- `GetEvidenceFileUploadUrl`
- `GetEvidenceFolder`
- `GetEvidenceFoldersByAssessment`
- `GetEvidenceFoldersByAssessmentControl`
- `GetInsights`
- `GetInsightsByAssessment`
- `GetOrganizationAdminAccount`
- `GetServicesInScope`
- `GetSettings`
- `ListAssessmentControlInsightsByControlDomain`
- `ListAssessmentFrameworkShareRequests`
- `ListAssessmentReports`
- `ListControlDomainInsights`
- `ListControlDomainInsightsByAssessment`
- `ListControlInsightsByControlDomain`
- `ListKeywordsForDataSource`
- `ListNotifications`
- `ListTagsForResource`
- `RegisterOrganizationAdminAccount`
- `StartAssessmentFrameworkShare`
- `TagResource`
- `UntagResource`
- `UpdateAssessment`
- `UpdateAssessmentControl`
- `UpdateAssessmentControlSetStatus`
- `UpdateAssessmentFramework`
- `UpdateAssessmentFrameworkShare`
- `UpdateAssessmentStatus`
- `UpdateControl`
- `UpdateSettings`
- `ValidateAssessmentReportIntegrity`

</details>
