# winterbaume-lexmodelsv2

Amazon Lex V2 Models service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Lex Models V2 |
| AWS model | `lex-models-v2` |
| Protocol | restJson1 |
| winterbaume coverage | 58/107 operations (54.2%) |
| stubs (routed, returns empty/default) | 2/107 operations (1.9%) |
| moto coverage | 17/107 operations (15.9%) |
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
aws lexmodelsv2 help
```

## Example

```rust
use aws_sdk_lexmodelsv2::config::BehaviorVersion;
use aws_sdk_lexmodelsv2::types::DataPrivacy;
use winterbaume_core::MockAws;
use winterbaume_lexmodelsv2::LexModelsV2Service;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(LexModelsV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_lexmodelsv2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_lexmodelsv2::Client::new(&config);

    // Create a bot
    let create_resp = client
        .create_bot()
        .bot_name("ExampleBot")
        .role_arn("arn:aws:iam::123456789012:role/LexBotRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");
    println!("Created bot: {:?}", create_resp.bot_id());

    // List bots
    let list_resp = client
        .list_bots()
        .send()
        .await
        .expect("list_bots should succeed");
    println!("Bots: {:?}", list_resp.bot_summaries());
}
```

## Implemented APIs (58)

- `BatchCreateCustomVocabularyItem`
- `BatchDeleteCustomVocabularyItem`
- `BatchUpdateCustomVocabularyItem`
- `BuildBotLocale`
- `CreateBot`
- `CreateBotAlias`
- `CreateBotLocale`
- `CreateBotVersion`
- `CreateExport`
- `CreateIntent`
- `CreateResourcePolicy`
- `CreateSlot`
- `CreateSlotType`
- `DeleteBot`
- `DeleteBotAlias`
- `DeleteBotLocale`
- `DeleteBotVersion`
- `DeleteCustomVocabulary`
- `DeleteExport`
- `DeleteImport`
- `DeleteIntent`
- `DeleteResourcePolicy`
- `DeleteSlot`
- `DeleteSlotType`
- `DeleteUtterances`
- `DescribeBot`
- `DescribeBotAlias`
- `DescribeBotLocale`
- `DescribeBotVersion`
- `DescribeCustomVocabularyMetadata`
- `DescribeExport`
- `DescribeImport`
- `DescribeIntent`
- `DescribeResourcePolicy`
- `DescribeSlot`
- `DescribeSlotType`
- `ListBotAliases`
- `ListBotLocales`
- `ListBotVersions`
- `ListBots`
- `ListCustomVocabularyItems`
- `ListExports`
- `ListImports`
- `ListIntents`
- `ListSlotTypes`
- `ListSlots`
- `ListTagsForResource`
- `StartImport`
- `TagResource`
- `UntagResource`
- `UpdateBot`
- `UpdateBotAlias`
- `UpdateBotLocale`
- `UpdateExport`
- `UpdateIntent`
- `UpdateResourcePolicy`
- `UpdateSlot`
- `UpdateSlotType`

<details><summary>Stubbed APIs (2) &mdash; routed but return an empty/default response</summary>

- `ListBuiltInIntents`
- `ListBuiltInSlotTypes`

</details>

<details><summary>Not yet implemented APIs (47)</summary>

- `CreateBotReplica`
- `CreateResourcePolicyStatement`
- `CreateTestSetDiscrepancyReport`
- `CreateUploadUrl`
- `DeleteBotAnalyzerRecommendation`
- `DeleteBotReplica`
- `DeleteResourcePolicyStatement`
- `DeleteTestSet`
- `DescribeBotAnalyzerRecommendation`
- `DescribeBotRecommendation`
- `DescribeBotReplica`
- `DescribeBotResourceGeneration`
- `DescribeTestExecution`
- `DescribeTestSet`
- `DescribeTestSetDiscrepancyReport`
- `DescribeTestSetGeneration`
- `GenerateBotElement`
- `GetTestExecutionArtifactsUrl`
- `ListAggregatedUtterances`
- `ListBotAliasReplicas`
- `ListBotAnalyzerHistory`
- `ListBotRecommendations`
- `ListBotReplicas`
- `ListBotResourceGenerations`
- `ListBotVersionReplicas`
- `ListIntentMetrics`
- `ListIntentPaths`
- `ListIntentStageMetrics`
- `ListRecommendedIntents`
- `ListSessionAnalyticsData`
- `ListSessionMetrics`
- `ListTestExecutionResultItems`
- `ListTestExecutions`
- `ListTestSetRecords`
- `ListTestSets`
- `ListUtteranceAnalyticsData`
- `ListUtteranceMetrics`
- `SearchAssociatedTranscripts`
- `StartBotAnalyzer`
- `StartBotRecommendation`
- `StartBotResourceGeneration`
- `StartTestExecution`
- `StartTestSetGeneration`
- `StopBotAnalyzer`
- `StopBotRecommendation`
- `UpdateBotRecommendation`
- `UpdateTestSet`

</details>
