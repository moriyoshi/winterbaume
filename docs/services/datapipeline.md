# winterbaume-datapipeline

Data Pipeline service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Data Pipeline |
| AWS model | `data-pipeline` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 19/19 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/19 operations (0.0%) |
| moto coverage | 0/19 operations (0.0%) |
| floci coverage | 0/19 operations (0.0%) |
| kumo coverage | 0/19 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws datapipeline list-pipelines
```

## Example

```rust
use aws_sdk_datapipeline::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_datapipeline::DataPipelineService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(DataPipelineService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_datapipeline::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_datapipeline::Client::new(&config);

    let resp = client
        .list_pipelines()
        .send()
        .await
        .expect("list_pipelines should succeed");
    println!("Data pipelines: {}", resp.pipeline_id_list().len());
}
```

## Implemented APIs (19)

- `ActivatePipeline`
- `AddTags`
- `CreatePipeline`
- `DeactivatePipeline`
- `DeletePipeline`
- `DescribeObjects`
- `DescribePipelines`
- `EvaluateExpression`
- `GetPipelineDefinition`
- `ListPipelines`
- `PollForTask`
- `PutPipelineDefinition`
- `QueryObjects`
- `RemoveTags`
- `ReportTaskProgress`
- `ReportTaskRunnerHeartbeat`
- `SetStatus`
- `SetTaskStatus`
- `ValidatePipelineDefinition`
