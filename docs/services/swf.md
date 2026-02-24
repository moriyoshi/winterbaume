# winterbaume-swf

Amazon SWF service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | SWF |
| AWS model | `swf` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 30/39 operations (76.9%) |
| stubs (routed, returns empty/default) | 0/39 operations (0.0%) |
| moto coverage | 19/39 operations (48.7%) |
| floci coverage | 0/39 operations (0.0%) |
| kumo coverage | 0/39 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws swf list-domains --registration-status REGISTERED
```

## Example

```rust
use aws_sdk_swf::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_swf::SwfService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(SwfService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_swf::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_swf::Client::new(&config);

    use aws_sdk_swf::types::RegistrationStatus;
    let resp = client
        .list_domains()
        .registration_status(RegistrationStatus::Registered)
        .send()
        .await
        .expect("list_domains should succeed");
    println!("SWF domains: {}", resp.domain_infos().len());
}
```

## Implemented APIs (30)

- `CountClosedWorkflowExecutions`
- `CountOpenWorkflowExecutions`
- `CountPendingActivityTasks`
- `CountPendingDecisionTasks`
- `DeprecateActivityType`
- `DeprecateDomain`
- `DeprecateWorkflowType`
- `DescribeActivityType`
- `DescribeDomain`
- `DescribeWorkflowExecution`
- `DescribeWorkflowType`
- `GetWorkflowExecutionHistory`
- `ListActivityTypes`
- `ListClosedWorkflowExecutions`
- `ListDomains`
- `ListOpenWorkflowExecutions`
- `ListWorkflowTypes`
- `PollForActivityTask`
- `PollForDecisionTask`
- `RecordActivityTaskHeartbeat`
- `RegisterActivityType`
- `RegisterDomain`
- `RegisterWorkflowType`
- `RespondActivityTaskCompleted`
- `RespondActivityTaskFailed`
- `RespondDecisionTaskCompleted`
- `SignalWorkflowExecution`
- `StartWorkflowExecution`
- `TerminateWorkflowExecution`
- `UndeprecateDomain`

<details><summary>Not yet implemented APIs (9)</summary>

- `DeleteActivityType`
- `DeleteWorkflowType`
- `ListTagsForResource`
- `RequestCancelWorkflowExecution`
- `RespondActivityTaskCanceled`
- `TagResource`
- `UndeprecateActivityType`
- `UndeprecateWorkflowType`
- `UntagResource`

</details>
