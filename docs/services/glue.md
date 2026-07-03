# winterbaume-glue

AWS Glue service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Glue |
| AWS model | `glue` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 132/265 operations (49.8%) |
| stubs (routed, returns empty/default) | 0/265 operations (0.0%) |
| moto coverage | 96/265 operations (36.2%) |
| floci coverage | 0/265 operations (0.0%) |
| kumo coverage | 14/265 operations (5.3%) |
| fakecloud coverage | 265/265 operations (100.0%) |
| Coverage report date | 2026-07-03 |

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
aws glue list-registries
```

## Current Network Resource Stub Semantics

Glue currently stores connection networking data as Glue connection properties.

- Connection records can include physical connection requirements such as subnet ID, security group ID lists, and availability zone.
- Jobs, crawlers, and sessions can refer to Glue connections without network reachability checks.
- Glue does not create ENIs or validate subnet and security group combinations.
- The implementation does not consult `winterbaume-ec2` state for these identifiers, so it does not check that referenced VPCs, subnets, security groups, VPC endpoints, network interfaces, or load balancers exist, belong to the same VPC, or are in a usable lifecycle state.

## Example

```rust
use aws_sdk_glue::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_glue::GlueService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(GlueService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_glue::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_glue::Client::new(&config);

    let resp = client
        .get_databases()
        .send()
        .await
        .expect("get_databases should succeed");
    println!("Glue databases: {}", resp.database_list().len());
}
```

## Implemented APIs (132)

- `BatchCreatePartition`
- `BatchDeleteConnection`
- `BatchDeletePartition`
- `BatchDeleteTable`
- `BatchGetCrawlers`
- `BatchGetDevEndpoints`
- `BatchGetJobs`
- `BatchGetPartition`
- `BatchGetTriggers`
- `BatchGetWorkflows`
- `BatchStopJobRun`
- `BatchUpdatePartition`
- `CheckSchemaVersionValidity`
- `CreateConnection`
- `CreateCrawler`
- `CreateDatabase`
- `CreateDevEndpoint`
- `CreateJob`
- `CreateMLTransform`
- `CreatePartition`
- `CreateRegistry`
- `CreateSchema`
- `CreateSecurityConfiguration`
- `CreateSession`
- `CreateTable`
- `CreateTrigger`
- `CreateWorkflow`
- `DeleteConnection`
- `DeleteCrawler`
- `DeleteDatabase`
- `DeleteDevEndpoint`
- `DeleteJob`
- `DeleteMLTransform`
- `DeletePartition`
- `DeleteRegistry`
- `DeleteResourcePolicy`
- `DeleteSchema`
- `DeleteSchemaVersions`
- `DeleteSecurityConfiguration`
- `DeleteSession`
- `DeleteTable`
- `DeleteTableVersion`
- `DeleteTrigger`
- `DeleteWorkflow`
- `GetConnection`
- `GetConnections`
- `GetCrawler`
- `GetCrawlerMetrics`
- `GetCrawlers`
- `GetDataCatalogEncryptionSettings`
- `GetDatabase`
- `GetDatabases`
- `GetDevEndpoint`
- `GetDevEndpoints`
- `GetJob`
- `GetJobBookmark`
- `GetJobRun`
- `GetJobRuns`
- `GetJobs`
- `GetMLTransform`
- `GetMLTransforms`
- `GetPartition`
- `GetPartitions`
- `GetRegistry`
- `GetResourcePolicy`
- `GetSchema`
- `GetSchemaByDefinition`
- `GetSchemaVersion`
- `GetSchemaVersionsDiff`
- `GetSecurityConfiguration`
- `GetSecurityConfigurations`
- `GetSession`
- `GetTable`
- `GetTableVersion`
- `GetTableVersions`
- `GetTables`
- `GetTags`
- `GetTrigger`
- `GetTriggers`
- `GetWorkflow`
- `GetWorkflowRun`
- `GetWorkflowRunProperties`
- `GetWorkflowRuns`
- `ImportCatalogToGlue`
- `ListCrawlers`
- `ListCrawls`
- `ListDevEndpoints`
- `ListJobs`
- `ListMLTransforms`
- `ListRegistries`
- `ListSchemaVersions`
- `ListSchemas`
- `ListSessions`
- `ListTriggers`
- `ListWorkflows`
- `PutDataCatalogEncryptionSettings`
- `PutResourcePolicy`
- `PutSchemaVersionMetadata`
- `PutWorkflowRunProperties`
- `QuerySchemaVersionMetadata`
- `RegisterSchemaVersion`
- `RemoveSchemaVersionMetadata`
- `ResetJobBookmark`
- `ResumeWorkflowRun`
- `SearchTables`
- `StartCrawler`
- `StartCrawlerSchedule`
- `StartJobRun`
- `StartTrigger`
- `StartWorkflowRun`
- `StopCrawler`
- `StopCrawlerSchedule`
- `StopSession`
- `StopTrigger`
- `StopWorkflowRun`
- `TagResource`
- `UntagResource`
- `UpdateConnection`
- `UpdateCrawler`
- `UpdateCrawlerSchedule`
- `UpdateDatabase`
- `UpdateDevEndpoint`
- `UpdateJob`
- `UpdateJobFromSourceControl`
- `UpdateMLTransform`
- `UpdatePartition`
- `UpdateRegistry`
- `UpdateSchema`
- `UpdateSourceControlFromJob`
- `UpdateTable`
- `UpdateTrigger`
- `UpdateWorkflow`

<details><summary>Not yet implemented APIs (133)</summary>

- `BatchDeleteTableVersion` (implemented by fakecloud)
- `BatchGetBlueprints` (implemented by fakecloud)
- `BatchGetCustomEntityTypes` (implemented by fakecloud)
- `BatchGetDataQualityResult` (implemented by fakecloud)
- `BatchGetTableOptimizer` (implemented by fakecloud)
- `BatchPutDataQualityStatisticAnnotation` (implemented by fakecloud)
- `CancelDataQualityRuleRecommendationRun` (implemented by fakecloud)
- `CancelDataQualityRulesetEvaluationRun` (implemented by fakecloud)
- `CancelMLTaskRun` (implemented by fakecloud)
- `CancelStatement` (implemented by fakecloud)
- `CreateBlueprint` (implemented by fakecloud)
- `CreateCatalog` (implemented by fakecloud)
- `CreateClassifier` (implemented by fakecloud)
- `CreateColumnStatisticsTaskSettings` (implemented by fakecloud)
- `CreateCustomEntityType` (implemented by fakecloud)
- `CreateDataQualityRuleset` (implemented by fakecloud)
- `CreateGlueIdentityCenterConfiguration` (implemented by fakecloud)
- `CreateIntegration` (implemented by fakecloud)
- `CreateIntegrationResourceProperty` (implemented by fakecloud)
- `CreateIntegrationTableProperties` (implemented by fakecloud)
- `CreatePartitionIndex` (implemented by fakecloud)
- `CreateScript` (implemented by fakecloud)
- `CreateTableOptimizer` (implemented by fakecloud)
- `CreateUsageProfile` (implemented by fakecloud)
- `CreateUserDefinedFunction` (implemented by fakecloud)
- `DeleteBlueprint` (implemented by fakecloud)
- `DeleteCatalog` (implemented by fakecloud)
- `DeleteClassifier` (implemented by fakecloud)
- `DeleteColumnStatisticsForPartition` (implemented by fakecloud)
- `DeleteColumnStatisticsForTable` (implemented by fakecloud)
- `DeleteColumnStatisticsTaskSettings` (implemented by fakecloud)
- `DeleteConnectionType` (implemented by fakecloud)
- `DeleteCustomEntityType` (implemented by fakecloud)
- `DeleteDataQualityRuleset` (implemented by fakecloud)
- `DeleteGlueIdentityCenterConfiguration` (implemented by fakecloud)
- `DeleteIntegration` (implemented by fakecloud)
- `DeleteIntegrationResourceProperty` (implemented by fakecloud)
- `DeleteIntegrationTableProperties` (implemented by fakecloud)
- `DeletePartitionIndex` (implemented by fakecloud)
- `DeleteTableOptimizer` (implemented by fakecloud)
- `DeleteUsageProfile` (implemented by fakecloud)
- `DeleteUserDefinedFunction` (implemented by fakecloud)
- `DescribeConnectionType` (implemented by fakecloud)
- `DescribeEntity` (implemented by fakecloud)
- `DescribeInboundIntegrations` (implemented by fakecloud)
- `DescribeIntegrations` (implemented by fakecloud)
- `GetBlueprint` (implemented by fakecloud)
- `GetBlueprintRun` (implemented by fakecloud)
- `GetBlueprintRuns` (implemented by fakecloud)
- `GetCatalog` (implemented by fakecloud)
- `GetCatalogImportStatus` (implemented by fakecloud)
- `GetCatalogs` (implemented by fakecloud)
- `GetClassifier` (implemented by fakecloud)
- `GetClassifiers` (implemented by fakecloud)
- `GetColumnStatisticsForPartition` (implemented by fakecloud)
- `GetColumnStatisticsForTable` (implemented by fakecloud)
- `GetColumnStatisticsTaskRun` (implemented by fakecloud)
- `GetColumnStatisticsTaskRuns` (implemented by fakecloud)
- `GetColumnStatisticsTaskSettings` (implemented by fakecloud)
- `GetCustomEntityType` (implemented by fakecloud)
- `GetDataQualityModel` (implemented by fakecloud)
- `GetDataQualityModelResult` (implemented by fakecloud)
- `GetDataQualityResult` (implemented by fakecloud)
- `GetDataQualityRuleRecommendationRun` (implemented by fakecloud)
- `GetDataQualityRuleset` (implemented by fakecloud)
- `GetDataQualityRulesetEvaluationRun` (implemented by fakecloud)
- `GetDataflowGraph` (implemented by fakecloud)
- `GetEntityRecords` (implemented by fakecloud)
- `GetGlueIdentityCenterConfiguration` (implemented by fakecloud)
- `GetIntegrationResourceProperty` (implemented by fakecloud)
- `GetIntegrationTableProperties` (implemented by fakecloud)
- `GetMLTaskRun` (implemented by fakecloud)
- `GetMLTaskRuns` (implemented by fakecloud)
- `GetMapping` (implemented by fakecloud)
- `GetMaterializedViewRefreshTaskRun` (implemented by fakecloud)
- `GetPartitionIndexes` (implemented by fakecloud)
- `GetPlan` (implemented by fakecloud)
- `GetResourcePolicies` (implemented by fakecloud)
- `GetStatement` (implemented by fakecloud)
- `GetTableOptimizer` (implemented by fakecloud)
- `GetUnfilteredPartitionMetadata` (implemented by fakecloud)
- `GetUnfilteredPartitionsMetadata` (implemented by fakecloud)
- `GetUnfilteredTableMetadata` (implemented by fakecloud)
- `GetUsageProfile` (implemented by fakecloud)
- `GetUserDefinedFunction` (implemented by fakecloud)
- `GetUserDefinedFunctions` (implemented by fakecloud)
- `ListBlueprints` (implemented by fakecloud)
- `ListColumnStatisticsTaskRuns` (implemented by fakecloud)
- `ListConnectionTypes` (implemented by fakecloud)
- `ListCustomEntityTypes` (implemented by fakecloud)
- `ListDataQualityResults` (implemented by fakecloud)
- `ListDataQualityRuleRecommendationRuns` (implemented by fakecloud)
- `ListDataQualityRulesetEvaluationRuns` (implemented by fakecloud)
- `ListDataQualityRulesets` (implemented by fakecloud)
- `ListDataQualityStatisticAnnotations` (implemented by fakecloud)
- `ListDataQualityStatistics` (implemented by fakecloud)
- `ListEntities` (implemented by fakecloud)
- `ListIntegrationResourceProperties` (implemented by fakecloud)
- `ListMaterializedViewRefreshTaskRuns` (implemented by fakecloud)
- `ListStatements` (implemented by fakecloud)
- `ListTableOptimizerRuns` (implemented by fakecloud)
- `ListUsageProfiles` (implemented by fakecloud)
- `ModifyIntegration` (implemented by fakecloud)
- `PutDataQualityProfileAnnotation` (implemented by fakecloud)
- `RegisterConnectionType` (implemented by fakecloud)
- `RunStatement` (implemented by fakecloud)
- `StartBlueprintRun` (implemented by fakecloud)
- `StartColumnStatisticsTaskRun` (implemented by fakecloud)
- `StartColumnStatisticsTaskRunSchedule` (implemented by fakecloud)
- `StartDataQualityRuleRecommendationRun` (implemented by fakecloud)
- `StartDataQualityRulesetEvaluationRun` (implemented by fakecloud)
- `StartExportLabelsTaskRun` (implemented by fakecloud)
- `StartImportLabelsTaskRun` (implemented by fakecloud)
- `StartMLEvaluationTaskRun` (implemented by fakecloud)
- `StartMLLabelingSetGenerationTaskRun` (implemented by fakecloud)
- `StartMaterializedViewRefreshTaskRun` (implemented by fakecloud)
- `StopColumnStatisticsTaskRun` (implemented by fakecloud)
- `StopColumnStatisticsTaskRunSchedule` (implemented by fakecloud)
- `StopMaterializedViewRefreshTaskRun` (implemented by fakecloud)
- `TestConnection` (implemented by fakecloud)
- `UpdateBlueprint` (implemented by fakecloud)
- `UpdateCatalog` (implemented by fakecloud)
- `UpdateClassifier` (implemented by fakecloud)
- `UpdateColumnStatisticsForPartition` (implemented by fakecloud)
- `UpdateColumnStatisticsForTable` (implemented by fakecloud)
- `UpdateColumnStatisticsTaskSettings` (implemented by fakecloud)
- `UpdateDataQualityRuleset` (implemented by fakecloud)
- `UpdateGlueIdentityCenterConfiguration` (implemented by fakecloud)
- `UpdateIntegrationResourceProperty` (implemented by fakecloud)
- `UpdateIntegrationTableProperties` (implemented by fakecloud)
- `UpdateTableOptimizer` (implemented by fakecloud)
- `UpdateUsageProfile` (implemented by fakecloud)
- `UpdateUserDefinedFunction` (implemented by fakecloud)

</details>
