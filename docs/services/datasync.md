# winterbaume-datasync

DataSync service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | DataSync |
| AWS model | `datasync` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 8/53 operations (15.1%) |
| stubs (routed, returns empty/default) | 0/53 operations (0.0%) |
| moto coverage | 6/53 operations (11.3%) |
| floci coverage | 0/53 operations (0.0%) |
| kumo coverage | 0/53 operations (0.0%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws datasync list-agents
```

## Example

```rust
use aws_sdk_datasync::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_datasync::DataSyncService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(DataSyncService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_datasync::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_datasync::Client::new(&config);

    let resp = client
        .list_agents()
        .send()
        .await
        .expect("list_agents should succeed");
    println!("DataSync agents: {}", resp.agents().len());
}
```

## Implemented APIs (8)

- `CancelTaskExecution`
- `CreateTask`
- `DeleteLocation`
- `DeleteTask`
- `DescribeTask`
- `ListTasks`
- `StartTaskExecution`
- `UpdateTask`

<details><summary>Not yet implemented APIs (45)</summary>

- `CreateAgent`
- `CreateLocationAzureBlob`
- `CreateLocationEfs`
- `CreateLocationFsxLustre`
- `CreateLocationFsxOntap`
- `CreateLocationFsxOpenZfs`
- `CreateLocationFsxWindows`
- `CreateLocationHdfs`
- `CreateLocationNfs`
- `CreateLocationObjectStorage`
- `CreateLocationS3`
- `CreateLocationSmb`
- `DeleteAgent`
- `DescribeAgent`
- `DescribeLocationAzureBlob`
- `DescribeLocationEfs`
- `DescribeLocationFsxLustre`
- `DescribeLocationFsxOntap`
- `DescribeLocationFsxOpenZfs`
- `DescribeLocationFsxWindows`
- `DescribeLocationHdfs`
- `DescribeLocationNfs`
- `DescribeLocationObjectStorage`
- `DescribeLocationS3`
- `DescribeLocationSmb`
- `DescribeTaskExecution`
- `ListAgents`
- `ListLocations`
- `ListTagsForResource`
- `ListTaskExecutions`
- `TagResource`
- `UntagResource`
- `UpdateAgent`
- `UpdateLocationAzureBlob`
- `UpdateLocationEfs`
- `UpdateLocationFsxLustre`
- `UpdateLocationFsxOntap`
- `UpdateLocationFsxOpenZfs`
- `UpdateLocationFsxWindows`
- `UpdateLocationHdfs`
- `UpdateLocationNfs`
- `UpdateLocationObjectStorage`
- `UpdateLocationS3`
- `UpdateLocationSmb`
- `UpdateTaskExecution`

</details>
