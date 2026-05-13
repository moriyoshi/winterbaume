# winterbaume-guardduty

GuardDuty service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | GuardDuty |
| AWS model | `guardduty` |
| Protocol | restJson1 |
| winterbaume coverage | 85/87 operations (97.7%) |
| stubs (routed, returns empty/default) | 2/87 operations (2.3%) |
| moto coverage | 12/87 operations (13.8%) |
| floci coverage | 0/87 operations (0.0%) |
| kumo coverage | 0/87 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws guardduty list-detectors
```

## Example

```rust
use aws_sdk_guardduty::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_guardduty::GuardDutyService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(GuardDutyService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_guardduty::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_guardduty::Client::new(&config);

    let resp = client
        .list_detectors()
        .send()
        .await
        .expect("list_detectors should succeed");
    println!("GuardDuty detectors: {}", resp.detector_ids().len());
}
```

## Implemented APIs (85)

- `AcceptAdministratorInvitation`
- `AcceptInvitation`
- `ArchiveFindings`
- `CreateDetector`
- `CreateFilter`
- `CreateIPSet`
- `CreateMalwareProtectionPlan`
- `CreateMembers`
- `CreatePublishingDestination`
- `CreateSampleFindings`
- `CreateThreatEntitySet`
- `CreateThreatIntelSet`
- `CreateTrustedEntitySet`
- `DeclineInvitations`
- `DeleteDetector`
- `DeleteFilter`
- `DeleteIPSet`
- `DeleteInvitations`
- `DeleteMalwareProtectionPlan`
- `DeleteMembers`
- `DeletePublishingDestination`
- `DeleteThreatEntitySet`
- `DeleteThreatIntelSet`
- `DeleteTrustedEntitySet`
- `DescribeMalwareScans`
- `DescribeOrganizationConfiguration`
- `DescribePublishingDestination`
- `DisableOrganizationAdminAccount`
- `DisassociateFromAdministratorAccount`
- `DisassociateFromMasterAccount`
- `DisassociateMembers`
- `EnableOrganizationAdminAccount`
- `GetAdministratorAccount`
- `GetCoverageStatistics`
- `GetDetector`
- `GetFilter`
- `GetFindings`
- `GetFindingsStatistics`
- `GetIPSet`
- `GetInvitationsCount`
- `GetMalwareProtectionPlan`
- `GetMalwareScan`
- `GetMalwareScanSettings`
- `GetMasterAccount`
- `GetMemberDetectors`
- `GetMembers`
- `GetOrganizationStatistics`
- `GetThreatEntitySet`
- `GetThreatIntelSet`
- `GetTrustedEntitySet`
- `InviteMembers`
- `ListCoverage`
- `ListDetectors`
- `ListFilters`
- `ListFindings`
- `ListIPSets`
- `ListInvitations`
- `ListMalwareProtectionPlans`
- `ListMalwareScans`
- `ListMembers`
- `ListOrganizationAdminAccounts`
- `ListPublishingDestinations`
- `ListTagsForResource`
- `ListThreatEntitySets`
- `ListThreatIntelSets`
- `ListTrustedEntitySets`
- `SendObjectMalwareScan`
- `StartMalwareScan`
- `StartMonitoringMembers`
- `StopMonitoringMembers`
- `TagResource`
- `UnarchiveFindings`
- `UntagResource`
- `UpdateDetector`
- `UpdateFilter`
- `UpdateFindingsFeedback`
- `UpdateIPSet`
- `UpdateMalwareProtectionPlan`
- `UpdateMalwareScanSettings`
- `UpdateMemberDetectors`
- `UpdateOrganizationConfiguration`
- `UpdatePublishingDestination`
- `UpdateThreatEntitySet`
- `UpdateThreatIntelSet`
- `UpdateTrustedEntitySet`

<details><summary>Stubbed APIs (2) &mdash; routed but return an empty/default response</summary>

- `GetRemainingFreeTrialDays`
- `GetUsageStatistics`

</details>
