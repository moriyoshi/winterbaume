# winterbaume-quicksight

QuickSight service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | QuickSight |
| AWS model | `quicksight` |
| Protocol | restJson1 |
| winterbaume coverage | 68/232 operations (29.3%) |
| stubs (routed, returns empty/default) | 0/232 operations (0.0%) |
| moto coverage | 31/232 operations (13.4%) |
| floci coverage | 0/232 operations (0.0%) |
| kumo coverage | 0/232 operations (0.0%) |
| Coverage report date | 2026-05-16 |

## Server-mode usage

Install `winterbaume-server` from crates.io or run it from a workspace checkout, then point the AWS CLI at it:

```sh
# Installed binary ( from crates.io ):
cargo install winterbaume-server
winterbaume-server --host 127.0.0.1 --port 5555

# Or, from a workspace checkout:
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws quicksight list-dashboards --aws-account-id 123456789012
```

## Current Network Resource Stub Semantics

QuickSight currently has a placeholder for VPC connection properties in dashboard/data-source snapshots.

- Views include a `vpc_connection_properties` JSON slot, and current snapshot construction sets it to `None`.
- Data source and asset state does not create or track QuickSight VPC connections.
- The service does not validate subnet, security group, or VPC connection identifiers.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_quicksight::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_quicksight::QuickSightService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(QuickSightService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_quicksight::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_quicksight::Client::new(&config);

    let resp = client
        .list_users()
        .aws_account_id("123456789012")
        .namespace("default")
        .send()
        .await
        .expect("list_users should succeed");
    println!("QuickSight users: {}", resp.user_list().len());
}
```

## Implemented APIs (68)

- `CreateAnalysis`
- `CreateDashboard`
- `CreateDataSet`
- `CreateDataSource`
- `CreateFolder`
- `CreateFolderMembership`
- `CreateGroup`
- `CreateGroupMembership`
- `CreateIngestion`
- `CreateNamespace`
- `CreateTemplate`
- `CreateTheme`
- `DeleteAnalysis`
- `DeleteDashboard`
- `DeleteDataSet`
- `DeleteDataSource`
- `DeleteFolder`
- `DeleteFolderMembership`
- `DeleteGroup`
- `DeleteGroupMembership`
- `DeleteNamespace`
- `DeleteTemplate`
- `DeleteTheme`
- `DeleteUser`
- `DescribeAccountSettings`
- `DescribeAnalysis`
- `DescribeDashboard`
- `DescribeDataSet`
- `DescribeDataSource`
- `DescribeDataSourcePermissions`
- `DescribeFolder`
- `DescribeGroup`
- `DescribeGroupMembership`
- `DescribeIngestion`
- `DescribeNamespace`
- `DescribeTemplate`
- `DescribeTheme`
- `DescribeUser`
- `ListAnalyses`
- `ListDashboards`
- `ListDataSets`
- `ListDataSources`
- `ListFolderMembers`
- `ListFolders`
- `ListGroupMemberships`
- `ListGroups`
- `ListIngestions`
- `ListNamespaces`
- `ListTagsForResource`
- `ListTemplates`
- `ListThemes`
- `ListUserGroups`
- `ListUsers`
- `RegisterUser`
- `SearchGroups`
- `TagResource`
- `UntagResource`
- `UpdateAccountSettings`
- `UpdateAnalysis`
- `UpdateDashboard`
- `UpdateDataSet`
- `UpdateDataSource`
- `UpdateDataSourcePermissions`
- `UpdateGroup`
- `UpdatePublicSharingSettings`
- `UpdateTemplate`
- `UpdateTheme`
- `UpdateUser`

<details><summary>Not yet implemented APIs (164)</summary>

- `BatchCreateTopicReviewedAnswer`
- `BatchDeleteTopicReviewedAnswer`
- `CancelIngestion`
- `CreateAccountCustomization`
- `CreateAccountSubscription`
- `CreateActionConnector`
- `CreateBrand`
- `CreateCustomPermissions`
- `CreateIAMPolicyAssignment`
- `CreateRefreshSchedule`
- `CreateRoleMembership`
- `CreateTemplateAlias`
- `CreateThemeAlias`
- `CreateTopic`
- `CreateTopicRefreshSchedule`
- `CreateVPCConnection`
- `DeleteAccountCustomPermission`
- `DeleteAccountCustomization`
- `DeleteAccountSubscription`
- `DeleteActionConnector`
- `DeleteBrand`
- `DeleteBrandAssignment`
- `DeleteCustomPermissions`
- `DeleteDataSetRefreshProperties`
- `DeleteDefaultQBusinessApplication`
- `DeleteIAMPolicyAssignment`
- `DeleteIdentityPropagationConfig`
- `DeleteRefreshSchedule`
- `DeleteRoleCustomPermission`
- `DeleteRoleMembership`
- `DeleteTemplateAlias`
- `DeleteThemeAlias`
- `DeleteTopic`
- `DeleteTopicRefreshSchedule`
- `DeleteUserByPrincipalId`
- `DeleteUserCustomPermission`
- `DeleteVPCConnection`
- `DescribeAccountCustomPermission`
- `DescribeAccountCustomization`
- `DescribeAccountSubscription`
- `DescribeActionConnector`
- `DescribeActionConnectorPermissions`
- `DescribeAnalysisDefinition`
- `DescribeAnalysisPermissions`
- `DescribeAssetBundleExportJob`
- `DescribeAssetBundleImportJob`
- `DescribeAutomationJob`
- `DescribeBrand`
- `DescribeBrandAssignment`
- `DescribeBrandPublishedVersion`
- `DescribeCustomPermissions`
- `DescribeDashboardDefinition`
- `DescribeDashboardPermissions`
- `DescribeDashboardSnapshotJob`
- `DescribeDashboardSnapshotJobResult`
- `DescribeDashboardsQAConfiguration`
- `DescribeDataSetPermissions`
- `DescribeDataSetRefreshProperties`
- `DescribeDefaultQBusinessApplication`
- `DescribeFolderPermissions`
- `DescribeFolderResolvedPermissions`
- `DescribeIAMPolicyAssignment`
- `DescribeIpRestriction`
- `DescribeKeyRegistration`
- `DescribeQPersonalizationConfiguration`
- `DescribeQuickSightQSearchConfiguration`
- `DescribeRefreshSchedule`
- `DescribeRoleCustomPermission`
- `DescribeSelfUpgradeConfiguration`
- `DescribeTemplateAlias`
- `DescribeTemplateDefinition`
- `DescribeTemplatePermissions`
- `DescribeThemeAlias`
- `DescribeThemePermissions`
- `DescribeTopic`
- `DescribeTopicPermissions`
- `DescribeTopicRefresh`
- `DescribeTopicRefreshSchedule`
- `DescribeVPCConnection`
- `GenerateEmbedUrlForAnonymousUser`
- `GenerateEmbedUrlForRegisteredUser`
- `GenerateEmbedUrlForRegisteredUserWithIdentity`
- `GetDashboardEmbedUrl`
- `GetFlowMetadata`
- `GetFlowPermissions`
- `GetIdentityContext`
- `GetSessionEmbedUrl`
- `ListActionConnectors`
- `ListAssetBundleExportJobs`
- `ListAssetBundleImportJobs`
- `ListBrands`
- `ListCustomPermissions`
- `ListDashboardVersions`
- `ListFlows`
- `ListFoldersForResource`
- `ListIAMPolicyAssignments`
- `ListIAMPolicyAssignmentsForUser`
- `ListIdentityPropagationConfigs`
- `ListRefreshSchedules`
- `ListRoleMemberships`
- `ListSelfUpgrades`
- `ListTemplateAliases`
- `ListTemplateVersions`
- `ListThemeAliases`
- `ListThemeVersions`
- `ListTopicRefreshSchedules`
- `ListTopicReviewedAnswers`
- `ListTopics`
- `ListVPCConnections`
- `PredictQAResults`
- `PutDataSetRefreshProperties`
- `RestoreAnalysis`
- `SearchActionConnectors`
- `SearchAnalyses`
- `SearchDashboards`
- `SearchDataSets`
- `SearchDataSources`
- `SearchFlows`
- `SearchFolders`
- `SearchTopics`
- `StartAssetBundleExportJob`
- `StartAssetBundleImportJob`
- `StartAutomationJob`
- `StartDashboardSnapshotJob`
- `StartDashboardSnapshotJobSchedule`
- `UpdateAccountCustomPermission`
- `UpdateAccountCustomization`
- `UpdateActionConnector`
- `UpdateActionConnectorPermissions`
- `UpdateAnalysisPermissions`
- `UpdateApplicationWithTokenExchangeGrant`
- `UpdateBrand`
- `UpdateBrandAssignment`
- `UpdateBrandPublishedVersion`
- `UpdateCustomPermissions`
- `UpdateDashboardLinks`
- `UpdateDashboardPermissions`
- `UpdateDashboardPublishedVersion`
- `UpdateDashboardsQAConfiguration`
- `UpdateDataSetPermissions`
- `UpdateDefaultQBusinessApplication`
- `UpdateFlowPermissions`
- `UpdateFolder`
- `UpdateFolderPermissions`
- `UpdateIAMPolicyAssignment`
- `UpdateIdentityPropagationConfig`
- `UpdateIpRestriction`
- `UpdateKeyRegistration`
- `UpdateQPersonalizationConfiguration`
- `UpdateQuickSightQSearchConfiguration`
- `UpdateRefreshSchedule`
- `UpdateRoleCustomPermission`
- `UpdateSPICECapacityConfiguration`
- `UpdateSelfUpgrade`
- `UpdateSelfUpgradeConfiguration`
- `UpdateTemplateAlias`
- `UpdateTemplatePermissions`
- `UpdateThemeAlias`
- `UpdateThemePermissions`
- `UpdateTopic`
- `UpdateTopicPermissions`
- `UpdateTopicRefreshSchedule`
- `UpdateUserCustomPermission`
- `UpdateVPCConnection`

</details>
