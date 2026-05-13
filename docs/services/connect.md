# winterbaume-connect

Amazon Connect service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Connect |
| AWS model | `connect` |
| Protocol | restJson1 |
| winterbaume coverage | 10/370 operations (2.7%) |
| stubs (routed, returns empty/default) | 0/370 operations (0.0%) |
| moto coverage | 10/370 operations (2.7%) |
| floci coverage | 0/370 operations (0.0%) |
| kumo coverage | 0/370 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws connect list-instances
```

## Example

```rust
use aws_sdk_connect::config::BehaviorVersion;
use winterbaume_connect::ConnectService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ConnectService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_connect::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_connect::Client::new(&config);

    let resp = client
        .list_instances()
        .send()
        .await
        .expect("list_instances should succeed");
    println!("Connect instances: {}", resp.instance_summary_list().len());
}
```

## Implemented APIs (10)

- `AssociateAnalyticsDataSet`
- `CreateInstance`
- `DeleteInstance`
- `DescribeInstance`
- `DisassociateAnalyticsDataSet`
- `ListAnalyticsDataAssociations`
- `ListInstances`
- `ListTagsForResource`
- `TagResource`
- `UntagResource`

<details><summary>Not yet implemented APIs (360)</summary>

- `ActivateEvaluationForm`
- `AssociateApprovedOrigin`
- `AssociateBot`
- `AssociateContactWithUser`
- `AssociateDefaultVocabulary`
- `AssociateEmailAddressAlias`
- `AssociateFlow`
- `AssociateHoursOfOperations`
- `AssociateInstanceStorageConfig`
- `AssociateLambdaFunction`
- `AssociateLexBot`
- `AssociatePhoneNumberContactFlow`
- `AssociateQueueEmailAddresses`
- `AssociateQueueQuickConnects`
- `AssociateRoutingProfileQueues`
- `AssociateSecurityKey`
- `AssociateSecurityProfiles`
- `AssociateTrafficDistributionGroupUser`
- `AssociateUserProficiencies`
- `AssociateWorkspace`
- `BatchAssociateAnalyticsDataSet`
- `BatchCreateDataTableValue`
- `BatchDeleteDataTableValue`
- `BatchDescribeDataTableValue`
- `BatchDisassociateAnalyticsDataSet`
- `BatchGetAttachedFileMetadata`
- `BatchGetFlowAssociation`
- `BatchPutContact`
- `BatchUpdateDataTableValue`
- `ClaimPhoneNumber`
- `CompleteAttachedFileUpload`
- `CreateAgentStatus`
- `CreateContact`
- `CreateContactFlow`
- `CreateContactFlowModule`
- `CreateContactFlowModuleAlias`
- `CreateContactFlowModuleVersion`
- `CreateContactFlowVersion`
- `CreateDataTable`
- `CreateDataTableAttribute`
- `CreateEmailAddress`
- `CreateEvaluationForm`
- `CreateHoursOfOperation`
- `CreateHoursOfOperationOverride`
- `CreateIntegrationAssociation`
- `CreateNotification`
- `CreateParticipant`
- `CreatePersistentContactAssociation`
- `CreatePredefinedAttribute`
- `CreatePrompt`
- `CreatePushNotificationRegistration`
- `CreateQueue`
- `CreateQuickConnect`
- `CreateRoutingProfile`
- `CreateRule`
- `CreateSecurityProfile`
- `CreateTaskTemplate`
- `CreateTestCase`
- `CreateTrafficDistributionGroup`
- `CreateUseCase`
- `CreateUser`
- `CreateUserHierarchyGroup`
- `CreateView`
- `CreateViewVersion`
- `CreateVocabulary`
- `CreateWorkspace`
- `CreateWorkspacePage`
- `DeactivateEvaluationForm`
- `DeleteAttachedFile`
- `DeleteContactEvaluation`
- `DeleteContactFlow`
- `DeleteContactFlowModule`
- `DeleteContactFlowModuleAlias`
- `DeleteContactFlowModuleVersion`
- `DeleteContactFlowVersion`
- `DeleteDataTable`
- `DeleteDataTableAttribute`
- `DeleteEmailAddress`
- `DeleteEvaluationForm`
- `DeleteHoursOfOperation`
- `DeleteHoursOfOperationOverride`
- `DeleteIntegrationAssociation`
- `DeleteNotification`
- `DeletePredefinedAttribute`
- `DeletePrompt`
- `DeletePushNotificationRegistration`
- `DeleteQueue`
- `DeleteQuickConnect`
- `DeleteRoutingProfile`
- `DeleteRule`
- `DeleteSecurityProfile`
- `DeleteTaskTemplate`
- `DeleteTestCase`
- `DeleteTrafficDistributionGroup`
- `DeleteUseCase`
- `DeleteUser`
- `DeleteUserHierarchyGroup`
- `DeleteView`
- `DeleteViewVersion`
- `DeleteVocabulary`
- `DeleteWorkspace`
- `DeleteWorkspaceMedia`
- `DeleteWorkspacePage`
- `DescribeAgentStatus`
- `DescribeAttachedFilesConfiguration`
- `DescribeAuthenticationProfile`
- `DescribeContact`
- `DescribeContactEvaluation`
- `DescribeContactFlow`
- `DescribeContactFlowModule`
- `DescribeContactFlowModuleAlias`
- `DescribeDataTable`
- `DescribeDataTableAttribute`
- `DescribeEmailAddress`
- `DescribeEvaluationForm`
- `DescribeHoursOfOperation`
- `DescribeHoursOfOperationOverride`
- `DescribeInstanceAttribute`
- `DescribeInstanceStorageConfig`
- `DescribeNotification`
- `DescribePhoneNumber`
- `DescribePredefinedAttribute`
- `DescribePrompt`
- `DescribeQueue`
- `DescribeQuickConnect`
- `DescribeRoutingProfile`
- `DescribeRule`
- `DescribeSecurityProfile`
- `DescribeTestCase`
- `DescribeTrafficDistributionGroup`
- `DescribeUser`
- `DescribeUserHierarchyGroup`
- `DescribeUserHierarchyStructure`
- `DescribeView`
- `DescribeVocabulary`
- `DescribeWorkspace`
- `DisassociateApprovedOrigin`
- `DisassociateBot`
- `DisassociateEmailAddressAlias`
- `DisassociateFlow`
- `DisassociateHoursOfOperations`
- `DisassociateInstanceStorageConfig`
- `DisassociateLambdaFunction`
- `DisassociateLexBot`
- `DisassociatePhoneNumberContactFlow`
- `DisassociateQueueEmailAddresses`
- `DisassociateQueueQuickConnects`
- `DisassociateRoutingProfileQueues`
- `DisassociateSecurityKey`
- `DisassociateSecurityProfiles`
- `DisassociateTrafficDistributionGroupUser`
- `DisassociateUserProficiencies`
- `DisassociateWorkspace`
- `DismissUserContact`
- `EvaluateDataTableValues`
- `GetAttachedFile`
- `GetContactAttributes`
- `GetContactMetrics`
- `GetCurrentMetricData`
- `GetCurrentUserData`
- `GetEffectiveHoursOfOperations`
- `GetFederationToken`
- `GetFlowAssociation`
- `GetMetricData`
- `GetMetricDataV2`
- `GetPromptFile`
- `GetTaskTemplate`
- `GetTestCaseExecutionSummary`
- `GetTrafficDistribution`
- `ImportPhoneNumber`
- `ImportWorkspaceMedia`
- `ListAgentStatuses`
- `ListAnalyticsDataLakeDataSets`
- `ListApprovedOrigins`
- `ListAssociatedContacts`
- `ListAttachedFilesConfigurations`
- `ListAuthenticationProfiles`
- `ListBots`
- `ListChildHoursOfOperations`
- `ListContactEvaluations`
- `ListContactFlowModuleAliases`
- `ListContactFlowModuleVersions`
- `ListContactFlowModules`
- `ListContactFlowVersions`
- `ListContactFlows`
- `ListContactReferences`
- `ListDataTableAttributes`
- `ListDataTablePrimaryValues`
- `ListDataTableValues`
- `ListDataTables`
- `ListDefaultVocabularies`
- `ListEntitySecurityProfiles`
- `ListEvaluationFormVersions`
- `ListEvaluationForms`
- `ListFlowAssociations`
- `ListHoursOfOperationOverrides`
- `ListHoursOfOperations`
- `ListInstanceAttributes`
- `ListInstanceStorageConfigs`
- `ListIntegrationAssociations`
- `ListLambdaFunctions`
- `ListLexBots`
- `ListNotifications`
- `ListPhoneNumbers`
- `ListPhoneNumbersV2`
- `ListPredefinedAttributes`
- `ListPrompts`
- `ListQueueEmailAddresses`
- `ListQueueQuickConnects`
- `ListQueues`
- `ListQuickConnects`
- `ListRealtimeContactAnalysisSegmentsV2`
- `ListRoutingProfileManualAssignmentQueues`
- `ListRoutingProfileQueues`
- `ListRoutingProfiles`
- `ListRules`
- `ListSecurityKeys`
- `ListSecurityProfileApplications`
- `ListSecurityProfileFlowModules`
- `ListSecurityProfilePermissions`
- `ListSecurityProfiles`
- `ListTaskTemplates`
- `ListTestCaseExecutionRecords`
- `ListTestCaseExecutions`
- `ListTestCases`
- `ListTrafficDistributionGroupUsers`
- `ListTrafficDistributionGroups`
- `ListUseCases`
- `ListUserHierarchyGroups`
- `ListUserNotifications`
- `ListUserProficiencies`
- `ListUsers`
- `ListViewVersions`
- `ListViews`
- `ListWorkspaceMedia`
- `ListWorkspacePages`
- `ListWorkspaces`
- `MonitorContact`
- `PauseContact`
- `PutUserStatus`
- `ReleasePhoneNumber`
- `ReplicateInstance`
- `ResumeContact`
- `ResumeContactRecording`
- `SearchAgentStatuses`
- `SearchAvailablePhoneNumbers`
- `SearchContactEvaluations`
- `SearchContactFlowModules`
- `SearchContactFlows`
- `SearchContacts`
- `SearchDataTables`
- `SearchEmailAddresses`
- `SearchEvaluationForms`
- `SearchHoursOfOperationOverrides`
- `SearchHoursOfOperations`
- `SearchNotifications`
- `SearchPredefinedAttributes`
- `SearchPrompts`
- `SearchQueues`
- `SearchQuickConnects`
- `SearchResourceTags`
- `SearchRoutingProfiles`
- `SearchSecurityProfiles`
- `SearchTestCases`
- `SearchUserHierarchyGroups`
- `SearchUsers`
- `SearchViews`
- `SearchVocabularies`
- `SearchWorkspaceAssociations`
- `SearchWorkspaces`
- `SendChatIntegrationEvent`
- `SendOutboundEmail`
- `StartAttachedFileUpload`
- `StartChatContact`
- `StartContactEvaluation`
- `StartContactMediaProcessing`
- `StartContactRecording`
- `StartContactStreaming`
- `StartEmailContact`
- `StartOutboundChatContact`
- `StartOutboundEmailContact`
- `StartOutboundVoiceContact`
- `StartScreenSharing`
- `StartTaskContact`
- `StartTestCaseExecution`
- `StartWebRTCContact`
- `StopContact`
- `StopContactMediaProcessing`
- `StopContactRecording`
- `StopContactStreaming`
- `StopTestCaseExecution`
- `SubmitContactEvaluation`
- `SuspendContactRecording`
- `TagContact`
- `TransferContact`
- `UntagContact`
- `UpdateAgentStatus`
- `UpdateAttachedFilesConfiguration`
- `UpdateAuthenticationProfile`
- `UpdateContact`
- `UpdateContactAttributes`
- `UpdateContactEvaluation`
- `UpdateContactFlowContent`
- `UpdateContactFlowMetadata`
- `UpdateContactFlowModuleAlias`
- `UpdateContactFlowModuleContent`
- `UpdateContactFlowModuleMetadata`
- `UpdateContactFlowName`
- `UpdateContactRoutingData`
- `UpdateContactSchedule`
- `UpdateDataTableAttribute`
- `UpdateDataTableMetadata`
- `UpdateDataTablePrimaryValues`
- `UpdateEmailAddressMetadata`
- `UpdateEvaluationForm`
- `UpdateHoursOfOperation`
- `UpdateHoursOfOperationOverride`
- `UpdateInstanceAttribute`
- `UpdateInstanceStorageConfig`
- `UpdateNotificationContent`
- `UpdateParticipantAuthentication`
- `UpdateParticipantRoleConfig`
- `UpdatePhoneNumber`
- `UpdatePhoneNumberMetadata`
- `UpdatePredefinedAttribute`
- `UpdatePrompt`
- `UpdateQueueHoursOfOperation`
- `UpdateQueueMaxContacts`
- `UpdateQueueName`
- `UpdateQueueOutboundCallerConfig`
- `UpdateQueueOutboundEmailConfig`
- `UpdateQueueStatus`
- `UpdateQuickConnectConfig`
- `UpdateQuickConnectName`
- `UpdateRoutingProfileAgentAvailabilityTimer`
- `UpdateRoutingProfileConcurrency`
- `UpdateRoutingProfileDefaultOutboundQueue`
- `UpdateRoutingProfileName`
- `UpdateRoutingProfileQueues`
- `UpdateRule`
- `UpdateSecurityProfile`
- `UpdateTaskTemplate`
- `UpdateTestCase`
- `UpdateTrafficDistribution`
- `UpdateUserConfig`
- `UpdateUserHierarchy`
- `UpdateUserHierarchyGroupName`
- `UpdateUserHierarchyStructure`
- `UpdateUserIdentityInfo`
- `UpdateUserNotificationStatus`
- `UpdateUserPhoneConfig`
- `UpdateUserProficiencies`
- `UpdateUserRoutingProfile`
- `UpdateUserSecurityProfiles`
- `UpdateViewContent`
- `UpdateViewMetadata`
- `UpdateWorkspaceMetadata`
- `UpdateWorkspacePage`
- `UpdateWorkspaceTheme`
- `UpdateWorkspaceVisibility`

</details>
