# winterbaume-personalizeevents

AWS Personalize Events service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Personalize Events |
| AWS model | `personalize-events` |
| Protocol | restJson1 |
| winterbaume coverage | 5/5 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/5 operations (0.0%) |
| moto coverage | 0/5 operations (0.0%) |
| floci coverage | 0/5 operations (0.0%) |
| kumo coverage | 0/5 operations (0.0%) |
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
aws personalizeevents help
```

## Example

```rust
use aws_sdk_personalizeevents::config::BehaviorVersion;
use aws_sdk_personalizeevents::types::Event;
use winterbaume_core::MockAws;
use winterbaume_personalizeevents::PersonalizeEventsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(PersonalizeEventsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_personalizeevents::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_personalizeevents::Client::new(&config);
    let event = Event::builder()
        .event_type("click")
        .sent_at(aws_smithy_types::DateTime::from_secs(1_700_000_000))
        .build()
        .expect("event");

    client
        .put_events()
        .tracking_id("track-1")
        .session_id("session-1")
        .event_list(event)
        .send()
        .await
        .expect("put_events should succeed");

    println!("Recorded an event for session session-1");
}
```

## Implemented APIs (5)

- `PutActionInteractions`
- `PutActions`
- `PutEvents`
- `PutItems`
- `PutUsers`
