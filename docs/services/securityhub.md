# winterbaume-securityhub

Security Hub service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Security Hub |
| AWS model | `securityhub` |
| Protocol | restJson1 |
| winterbaume coverage | 97/107 operations (90.7%) |
| stubs (routed, returns empty/default) | 10/107 operations (9.3%) |
| moto coverage | 13/107 operations (12.1%) |
| floci coverage | 0/107 operations (0.0%) |
| kumo coverage | 0/107 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws securityhub list-hubs
```

## Example

```rust
use aws_sdk_securityhub::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_securityhub::SecurityHubService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SecurityHubService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_securityhub::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_securityhub::Client::new(&config);

    let resp = client
        .describe_hub()
        .send()
        .await
        .expect("describe_hub should succeed");
    println!("Security Hub ARN: {}", resp.hub_arn().unwrap_or("(none)"));
}
```

## Implemented APIs (97)

- `BatchDeleteAutomationRules`
- `BatchDisableStandards`
- `BatchEnableStandards`
- `BatchGetAutomationRules`
- `BatchGetConfigurationPolicyAssociations`
- `BatchGetSecurityControls`
- `BatchGetStandardsControlAssociations`
- `BatchImportFindings`
- `BatchUpdateAutomationRules`
- `BatchUpdateFindings`
- `BatchUpdateFindingsV2`
- `BatchUpdateStandardsControlAssociations`
- `CreateActionTarget`
- `CreateAggregatorV2`
- `CreateAutomationRule`
- `CreateAutomationRuleV2`
- `CreateConfigurationPolicy`
- `CreateConnectorV2`
- `CreateFindingAggregator`
- `CreateInsight`
- `CreateMembers`
- `CreateTicketV2`
- `DeleteActionTarget`
- `DeleteAggregatorV2`
- `DeleteAutomationRuleV2`
- `DeleteConfigurationPolicy`
- `DeleteConnectorV2`
- `DeleteFindingAggregator`
- `DeleteInsight`
- `DeleteMembers`
- `DescribeActionTargets`
- `DescribeHub`
- `DescribeOrganizationConfiguration`
- `DescribeProducts`
- `DescribeProductsV2`
- `DescribeSecurityHubV2`
- `DescribeStandards`
- `DescribeStandardsControls`
- `DisableImportFindingsForProduct`
- `DisableOrganizationAdminAccount`
- `DisableSecurityHub`
- `DisableSecurityHubV2`
- `EnableImportFindingsForProduct`
- `EnableOrganizationAdminAccount`
- `EnableSecurityHub`
- `EnableSecurityHubV2`
- `GetAdministratorAccount`
- `GetAggregatorV2`
- `GetAutomationRuleV2`
- `GetConfigurationPolicy`
- `GetConfigurationPolicyAssociation`
- `GetConnectorV2`
- `GetEnabledStandards`
- `GetFindingAggregator`
- `GetFindingHistory`
- `GetFindingStatisticsV2`
- `GetFindings`
- `GetFindingsTrendsV2`
- `GetFindingsV2`
- `GetInsightResults`
- `GetInsights`
- `GetMasterAccount`
- `GetMembers`
- `GetResourcesStatisticsV2`
- `GetResourcesTrendsV2`
- `GetResourcesV2`
- `GetSecurityControlDefinition`
- `ListAggregatorsV2`
- `ListAutomationRules`
- `ListAutomationRulesV2`
- `ListConfigurationPolicies`
- `ListConfigurationPolicyAssociations`
- `ListConnectorsV2`
- `ListEnabledProductsForImport`
- `ListFindingAggregators`
- `ListMembers`
- `ListOrganizationAdminAccounts`
- `ListSecurityControlDefinitions`
- `ListStandardsControlAssociations`
- `ListTagsForResource`
- `RegisterConnectorV2`
- `StartConfigurationPolicyAssociation`
- `StartConfigurationPolicyDisassociation`
- `TagResource`
- `UntagResource`
- `UpdateActionTarget`
- `UpdateAggregatorV2`
- `UpdateAutomationRuleV2`
- `UpdateConfigurationPolicy`
- `UpdateConnectorV2`
- `UpdateFindingAggregator`
- `UpdateFindings`
- `UpdateInsight`
- `UpdateOrganizationConfiguration`
- `UpdateSecurityControl`
- `UpdateSecurityHubConfiguration`
- `UpdateStandardsControl`

<details><summary>Stubbed APIs (10) &mdash; routed but return an empty/default response</summary>

- `AcceptAdministratorInvitation`
- `AcceptInvitation`
- `DeclineInvitations`
- `DeleteInvitations`
- `DisassociateFromAdministratorAccount`
- `DisassociateFromMasterAccount`
- `DisassociateMembers`
- `GetInvitationsCount`
- `InviteMembers`
- `ListInvitations`

</details>
