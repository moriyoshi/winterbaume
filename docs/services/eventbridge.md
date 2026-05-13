# winterbaume-eventbridge

EventBridge service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | EventBridge |
| AWS model | `eventbridge` |
| Protocol | awsJson1.1 |
| winterbaume coverage | 57/57 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/57 operations (0.0%) |
| moto coverage | 45/57 operations (78.9%) |
| floci coverage | 0/57 operations (0.0%) |
| kumo coverage | 15/57 operations (26.3%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws events list-rules
```

## Example

```rust
use aws_sdk_eventbridge::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_eventbridge::EventBridgeService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(EventBridgeService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_eventbridge::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_eventbridge::Client::new(&config);

    let resp = client
        .list_rules()
        .send()
        .await
        .expect("list_rules should succeed");
    println!("EventBridge rules: {}", resp.rules().len());
}
```

## Implemented APIs (57)

- `ActivateEventSource`
- `CancelReplay`
- `CreateApiDestination`
- `CreateArchive`
- `CreateConnection`
- `CreateEndpoint`
- `CreateEventBus`
- `CreatePartnerEventSource`
- `DeactivateEventSource`
- `DeauthorizeConnection`
- `DeleteApiDestination`
- `DeleteArchive`
- `DeleteConnection`
- `DeleteEndpoint`
- `DeleteEventBus`
- `DeletePartnerEventSource`
- `DeleteRule`
- `DescribeApiDestination`
- `DescribeArchive`
- `DescribeConnection`
- `DescribeEndpoint`
- `DescribeEventBus`
- `DescribeEventSource`
- `DescribePartnerEventSource`
- `DescribeReplay`
- `DescribeRule`
- `DisableRule`
- `EnableRule`
- `ListApiDestinations`
- `ListArchives`
- `ListConnections`
- `ListEndpoints`
- `ListEventBuses`
- `ListEventSources`
- `ListPartnerEventSourceAccounts`
- `ListPartnerEventSources`
- `ListReplays`
- `ListRuleNamesByTarget`
- `ListRules`
- `ListTagsForResource`
- `ListTargetsByRule`
- `PutEvents`
- `PutPartnerEvents`
- `PutPermission`
- `PutRule`
- `PutTargets`
- `RemovePermission`
- `RemoveTargets`
- `StartReplay`
- `TagResource`
- `TestEventPattern`
- `UntagResource`
- `UpdateApiDestination`
- `UpdateArchive`
- `UpdateConnection`
- `UpdateEndpoint`
- `UpdateEventBus`
