# winterbaume-chimesdkmeetings

AWS Chime SDK Meetings service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Chime SDK Meetings |
| AWS model | `chime-sdk-meetings` |
| Protocol | restJson1 |
| winterbaume coverage | 12/16 operations (75.0%) |
| stubs (routed, returns empty/default) | 0/16 operations (0.0%) |
| moto coverage | 0/16 operations (0.0%) |
| floci coverage | 0/16 operations (0.0%) |
| kumo coverage | 0/16 operations (0.0%) |
| fakecloud coverage | 0/16 operations (0.0%) |
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
aws chimesdkmeetings help
```

## Example

```rust
use aws_sdk_chimesdkmeetings::config::BehaviorVersion;
use winterbaume_chimesdkmeetings::ChimeSdkMeetingsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ChimeSdkMeetingsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_chimesdkmeetings::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_chimesdkmeetings::Client::new(&config);
    let resp = client
        .create_meeting()
        .client_request_token("ct")
        .external_meeting_id("demo")
        .media_region("us-east-1")
        .send()
        .await
        .expect("create_meeting");
    if let Some(m) = resp.meeting() {
        println!("Meeting: {:?}", m.meeting_id());
    }
}
```

## Implemented APIs (12)

- `BatchCreateAttendee`
- `CreateAttendee`
- `CreateMeeting`
- `CreateMeetingWithAttendees`
- `DeleteAttendee`
- `DeleteMeeting`
- `GetAttendee`
- `GetMeeting`
- `ListAttendees`
- `TagResource`
- `UntagResource`
- `UpdateAttendeeCapabilities`

<details><summary>Not yet implemented APIs (4)</summary>

- `BatchUpdateAttendeeCapabilitiesExcept`
- `ListTagsForResource`
- `StartMeetingTranscription`
- `StopMeetingTranscription`

</details>
