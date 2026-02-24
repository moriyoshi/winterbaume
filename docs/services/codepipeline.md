# winterbaume-codepipeline

CodePipeline service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | CodePipeline |
| AWS model | `codepipeline` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 44/44 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/44 operations (0.0%) |
| moto coverage | 8/44 operations (18.2%) |
| floci coverage | 0/44 operations (0.0%) |
| kumo coverage | 0/44 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws codepipeline list-pipelines
```

## Example

```rust
use aws_sdk_codepipeline::config::BehaviorVersion;
use winterbaume_codepipeline::CodePipelineService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CodePipelineService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codepipeline::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_codepipeline::Client::new(&config);

    let resp = client
        .list_pipelines()
        .send()
        .await
        .expect("list_pipelines should succeed");
    println!("Pipelines: {}", resp.pipelines().len());
}
```

## Implemented APIs (44)

- `AcknowledgeJob`
- `AcknowledgeThirdPartyJob`
- `CreateCustomActionType`
- `CreatePipeline`
- `DeleteCustomActionType`
- `DeletePipeline`
- `DeleteWebhook`
- `DeregisterWebhookWithThirdParty`
- `DisableStageTransition`
- `EnableStageTransition`
- `GetActionType`
- `GetJobDetails`
- `GetPipeline`
- `GetPipelineExecution`
- `GetPipelineState`
- `GetThirdPartyJobDetails`
- `ListActionExecutions`
- `ListActionTypes`
- `ListDeployActionExecutionTargets`
- `ListPipelineExecutions`
- `ListPipelines`
- `ListRuleExecutions`
- `ListRuleTypes`
- `ListTagsForResource`
- `ListWebhooks`
- `OverrideStageCondition`
- `PollForJobs`
- `PollForThirdPartyJobs`
- `PutActionRevision`
- `PutApprovalResult`
- `PutJobFailureResult`
- `PutJobSuccessResult`
- `PutThirdPartyJobFailureResult`
- `PutThirdPartyJobSuccessResult`
- `PutWebhook`
- `RegisterWebhookWithThirdParty`
- `RetryStageExecution`
- `RollbackStage`
- `StartPipelineExecution`
- `StopPipelineExecution`
- `TagResource`
- `UntagResource`
- `UpdateActionType`
- `UpdatePipeline`
