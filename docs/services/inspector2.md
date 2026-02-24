# winterbaume-inspector2

Inspector2 service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Inspector2 |
| AWS model | `inspector2` |
| Protocol | restJson1 |
| winterbaume coverage | 49/75 operations (65.3%) |
| stubs (routed, returns empty/default) | 21/75 operations (28.0%) |
| moto coverage | 19/75 operations (25.3%) |
| floci coverage | 0/75 operations (0.0%) |
| kumo coverage | 0/75 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws inspector2 list-findings
```

## Example

```rust
use aws_sdk_inspector2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_inspector2::Inspector2Service;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(Inspector2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_inspector2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_inspector2::Client::new(&config);

    let resp = client
        .list_coverage()
        .send()
        .await
        .expect("list_coverage should succeed");
    println!(
        "Inspector2 covered resources: {}",
        resp.covered_resources().len()
    );
}
```

## Implemented APIs (49)

- `AssociateMember`
- `BatchAssociateCodeSecurityScanConfiguration`
- `BatchDisassociateCodeSecurityScanConfiguration`
- `BatchGetAccountStatus`
- `CancelFindingsReport`
- `CancelSbomExport`
- `CreateCisScanConfiguration`
- `CreateCodeSecurityIntegration`
- `CreateCodeSecurityScanConfiguration`
- `CreateFilter`
- `CreateFindingsReport`
- `CreateSbomExport`
- `DeleteCisScanConfiguration`
- `DeleteCodeSecurityIntegration`
- `DeleteCodeSecurityScanConfiguration`
- `DeleteFilter`
- `DescribeOrganizationConfiguration`
- `Disable`
- `DisableDelegatedAdminAccount`
- `DisassociateMember`
- `Enable`
- `EnableDelegatedAdminAccount`
- `GetCodeSecurityIntegration`
- `GetCodeSecurityScanConfiguration`
- `GetConfiguration`
- `GetDelegatedAdminAccount`
- `GetEncryptionKey`
- `GetFindingsReportStatus`
- `GetMember`
- `GetSbomExport`
- `ListCisScanConfigurations`
- `ListCodeSecurityIntegrations`
- `ListCodeSecurityScanConfigurationAssociations`
- `ListCodeSecurityScanConfigurations`
- `ListDelegatedAdminAccounts`
- `ListFilters`
- `ListFindings`
- `ListMembers`
- `ListTagsForResource`
- `ResetEncryptionKey`
- `TagResource`
- `UntagResource`
- `UpdateCisScanConfiguration`
- `UpdateCodeSecurityIntegration`
- `UpdateCodeSecurityScanConfiguration`
- `UpdateConfiguration`
- `UpdateEncryptionKey`
- `UpdateFilter`
- `UpdateOrganizationConfiguration`

<details><summary>Stubbed APIs (21) &mdash; routed but return an empty/default response</summary>

- `BatchGetCodeSnippet`
- `BatchGetFindingDetails`
- `BatchGetFreeTrialInfo`
- `GetCisScanReport`
- `GetCisScanResultDetails`
- `GetClustersForImage`
- `GetCodeSecurityScan`
- `ListAccountPermissions`
- `ListCisScanResultsAggregatedByChecks`
- `ListCisScanResultsAggregatedByTargetResource`
- `ListCisScans`
- `ListCoverage`
- `ListCoverageStatistics`
- `ListFindingAggregations`
- `ListUsageTotals`
- `SearchVulnerabilities`
- `SendCisSessionHealth`
- `SendCisSessionTelemetry`
- `StartCisSession`
- `StartCodeSecurityScan`
- `StopCisSession`

</details>

<details><summary>Not yet implemented APIs (5)</summary>

- `BatchGetMemberEc2DeepInspectionStatus`
- `BatchUpdateMemberEc2DeepInspectionStatus`
- `GetEc2DeepInspectionConfiguration`
- `UpdateEc2DeepInspectionConfiguration`
- `UpdateOrgEc2DeepInspectionConfiguration`

</details>
