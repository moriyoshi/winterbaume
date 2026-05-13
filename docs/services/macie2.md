# winterbaume-macie2

Macie2 service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Macie2 |
| AWS model | `macie2` |
| Protocol | restJson1 |
| winterbaume coverage | 67/81 operations (82.7%) |
| stubs (routed, returns empty/default) | 14/81 operations (17.3%) |
| moto coverage | 13/81 operations (16.0%) |
| floci coverage | 0/81 operations (0.0%) |
| kumo coverage | 24/81 operations (29.6%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws macie2 list-findings
```

## Example

```rust
use aws_sdk_macie2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_macie2::Macie2Service;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(Macie2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_macie2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_macie2::Client::new(&config);

    let resp = client
        .list_findings()
        .send()
        .await
        .expect("list_findings should succeed");
    println!("Macie2 findings: {}", resp.finding_ids().len());
}
```

## Implemented APIs (67)

- `AcceptInvitation`
- `BatchGetCustomDataIdentifiers`
- `BatchUpdateAutomatedDiscoveryAccounts`
- `CreateAllowList`
- `CreateClassificationJob`
- `CreateCustomDataIdentifier`
- `CreateFindingsFilter`
- `CreateInvitations`
- `CreateMember`
- `CreateSampleFindings`
- `DeclineInvitations`
- `DeleteAllowList`
- `DeleteCustomDataIdentifier`
- `DeleteFindingsFilter`
- `DeleteInvitations`
- `DeleteMember`
- `DescribeClassificationJob`
- `DescribeOrganizationConfiguration`
- `DisableMacie`
- `DisableOrganizationAdminAccount`
- `DisassociateFromAdministratorAccount`
- `DisassociateFromMasterAccount`
- `DisassociateMember`
- `EnableMacie`
- `EnableOrganizationAdminAccount`
- `GetAdministratorAccount`
- `GetAllowList`
- `GetAutomatedDiscoveryConfiguration`
- `GetClassificationExportConfiguration`
- `GetCustomDataIdentifier`
- `GetFindingStatistics`
- `GetFindings`
- `GetFindingsFilter`
- `GetFindingsPublicationConfiguration`
- `GetInvitationsCount`
- `GetMacieSession`
- `GetMasterAccount`
- `GetMember`
- `GetRevealConfiguration`
- `GetSensitivityInspectionTemplate`
- `GetUsageStatistics`
- `GetUsageTotals`
- `ListAllowLists`
- `ListAutomatedDiscoveryAccounts`
- `ListClassificationJobs`
- `ListCustomDataIdentifiers`
- `ListFindings`
- `ListFindingsFilters`
- `ListInvitations`
- `ListManagedDataIdentifiers`
- `ListMembers`
- `ListOrganizationAdminAccounts`
- `ListSensitivityInspectionTemplates`
- `ListTagsForResource`
- `PutClassificationExportConfiguration`
- `PutFindingsPublicationConfiguration`
- `TagResource`
- `UntagResource`
- `UpdateAllowList`
- `UpdateAutomatedDiscoveryConfiguration`
- `UpdateClassificationJob`
- `UpdateFindingsFilter`
- `UpdateMacieSession`
- `UpdateMemberSession`
- `UpdateOrganizationConfiguration`
- `UpdateRevealConfiguration`
- `UpdateSensitivityInspectionTemplate`

<details><summary>Stubbed APIs (14) &mdash; routed but return an empty/default response</summary>

- `DescribeBuckets`
- `GetBucketStatistics`
- `GetClassificationScope`
- `GetResourceProfile`
- `GetSensitiveDataOccurrences`
- `GetSensitiveDataOccurrencesAvailability`
- `ListClassificationScopes`
- `ListResourceProfileArtifacts`
- `ListResourceProfileDetections`
- `SearchResources`
- `TestCustomDataIdentifier`
- `UpdateClassificationScope`
- `UpdateResourceProfile`
- `UpdateResourceProfileDetections`

</details>
