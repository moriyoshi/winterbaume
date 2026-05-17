# winterbaume-codestarnotifications

AWS CodeStar Notifications service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | CodeStar Notifications |
| AWS model | `codestar-notifications` |
| Protocol | restJson1 |
| winterbaume coverage | 7/13 operations (53.8%) |
| stubs (routed, returns empty/default) | 0/13 operations (0.0%) |
| moto coverage | 0/13 operations (0.0%) |
| floci coverage | 0/13 operations (0.0%) |
| kumo coverage | 0/13 operations (0.0%) |
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
aws codestarnotifications help
```

## Example

```rust
use aws_sdk_codestarnotifications::config::BehaviorVersion;
use aws_sdk_codestarnotifications::types::{DetailType, NotificationRuleStatus, Target};
use winterbaume_codestarnotifications::CodeStarNotificationsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CodeStarNotificationsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codestarnotifications::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_codestarnotifications::Client::new(&config);
    let target = Target::builder()
        .target_type("SNS")
        .target_address("arn:aws:sns:us-east-1:123456789012:notify")
        .build();
    let resp = client
        .create_notification_rule()
        .name("demo-rule")
        .resource("arn:aws:codecommit:us-east-1:123456789012:my-repo")
        .detail_type(DetailType::Basic)
        .event_type_ids("codecommit-repository-pull-request-created")
        .targets(target)
        .status(NotificationRuleStatus::Enabled)
        .send()
        .await
        .expect("create");
    if let Some(arn) = resp.arn() {
        println!("Rule: {arn}");
    }
}
```

## Implemented APIs (7)

- `DeleteTarget`
- `ListEventTypes`
- `ListTargets`
- `Subscribe`
- `TagResource`
- `Unsubscribe`
- `UntagResource`

<details><summary>Not yet implemented APIs (6)</summary>

- `CreateNotificationRule`
- `DeleteNotificationRule`
- `DescribeNotificationRule`
- `ListNotificationRules`
- `ListTagsForResource`
- `UpdateNotificationRule`

</details>
