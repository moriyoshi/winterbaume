# winterbaume-sfn

Step Functions service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Step Functions |
| AWS model | `sfn` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 35/37 operations (94.6%) |
| stubs (routed, returns empty/default) | 2/37 operations (5.4%) |
| moto coverage | 29/37 operations (78.4%) |
| floci coverage | 0/37 operations (0.0%) |
| kumo coverage | 18/37 operations (48.6%) |
| Coverage report date | 2026-05-12 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws stepfunctions list-state-machines
```

## Example

```rust
use aws_sdk_sfn::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sfn::SfnService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(SfnService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sfn::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_sfn::Client::new(&config);

    let resp = client
        .list_state_machines()
        .send()
        .await
        .expect("list_state_machines should succeed");
    println!(
        "Step Functions state machines: {}",
        resp.state_machines().len()
    );
}
```

## Implemented APIs (35)

- `CreateActivity`
- `CreateStateMachine`
- `CreateStateMachineAlias`
- `DeleteActivity`
- `DeleteStateMachine`
- `DeleteStateMachineAlias`
- `DeleteStateMachineVersion`
- `DescribeActivity`
- `DescribeExecution`
- `DescribeMapRun`
- `DescribeStateMachine`
- `DescribeStateMachineAlias`
- `DescribeStateMachineForExecution`
- `GetActivityTask`
- `GetExecutionHistory`
- `ListActivities`
- `ListExecutions`
- `ListMapRuns`
- `ListStateMachineAliases`
- `ListStateMachineVersions`
- `ListStateMachines`
- `ListTagsForResource`
- `PublishStateMachineVersion`
- `RedriveExecution`
- `SendTaskFailure`
- `SendTaskHeartbeat`
- `SendTaskSuccess`
- `StartExecution`
- `StopExecution`
- `TagResource`
- `UntagResource`
- `UpdateMapRun`
- `UpdateStateMachine`
- `UpdateStateMachineAlias`
- `ValidateStateMachineDefinition`

<details><summary>Stubbed APIs (2) &mdash; routed but return an empty/default response</summary>

- `StartSyncExecution`
- `TestState`

</details>
