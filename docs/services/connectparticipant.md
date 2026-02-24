# winterbaume-connectparticipant

AWS Connect Participant service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Connect Participant |
| AWS model | `connectparticipant` |
| Protocol | restJson1 |
| winterbaume coverage | 7/11 operations (63.6%) |
| stubs (routed, returns empty/default) | 0/11 operations (0.0%) |
| moto coverage | 0/11 operations (0.0%) |
| floci coverage | 0/11 operations (0.0%) |
| kumo coverage | 0/11 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws connectparticipant help
```

## Example

```rust
use aws_sdk_connectparticipant::config::BehaviorVersion;
use winterbaume_connectparticipant::ConnectParticipantService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ConnectParticipantService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_connectparticipant::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_connectparticipant::Client::new(&config);
    let resp = client
        .create_participant_connection()
        .participant_token("demo-token")
        .r#type("WEBSOCKET".into())
        .r#type("CONNECTION_CREDENTIALS".into())
        .send()
        .await
        .expect("create_participant_connection should succeed");
    if let Some(creds) = resp.connection_credentials() {
        println!("Connection token: {:?}", creds.connection_token());
    }
}
```

## Implemented APIs (7)

- `CompleteAttachmentUpload`
- `DescribeView`
- `GetAttachment`
- `GetTranscript`
- `SendEvent`
- `SendMessage`
- `StartAttachmentUpload`

<details><summary>Not yet implemented APIs (4)</summary>

- `CancelParticipantAuthentication`
- `CreateParticipantConnection`
- `DisconnectParticipant`
- `GetAuthenticationUrl`

</details>
