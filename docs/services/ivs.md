# winterbaume-ivs

IVS (Interactive Video Service) implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | IVS |
| AWS model | `ivs` |
| Protocol | restJson1 |
| winterbaume coverage | 30/40 operations (75.0%) |
| stubs (routed, returns empty/default) | 5/40 operations (12.5%) |
| moto coverage | 6/40 operations (15.0%) |
| floci coverage | 0/40 operations (0.0%) |
| kumo coverage | 0/40 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws ivs list-channels
```

## Example

```rust
use aws_sdk_ivs::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ivs::IvsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(IvsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ivs::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_ivs::Client::new(&config);

    let resp = client
        .list_channels()
        .send()
        .await
        .expect("list_channels should succeed");
    println!("IVS channels: {}", resp.channels().len());
}
```

## Implemented APIs (30)

- `BatchGetChannel`
- `BatchGetStreamKey`
- `BatchStartViewerSessionRevocation`
- `CreateChannel`
- `CreatePlaybackRestrictionPolicy`
- `CreateRecordingConfiguration`
- `CreateStreamKey`
- `DeleteChannel`
- `DeletePlaybackKeyPair`
- `DeletePlaybackRestrictionPolicy`
- `DeleteRecordingConfiguration`
- `DeleteStreamKey`
- `GetChannel`
- `GetPlaybackKeyPair`
- `GetPlaybackRestrictionPolicy`
- `GetRecordingConfiguration`
- `GetStreamKey`
- `ImportPlaybackKeyPair`
- `ListChannels`
- `ListPlaybackKeyPairs`
- `ListPlaybackRestrictionPolicies`
- `ListRecordingConfigurations`
- `ListStreamKeys`
- `ListTagsForResource`
- `PutMetadata`
- `StartViewerSessionRevocation`
- `TagResource`
- `UntagResource`
- `UpdateChannel`
- `UpdatePlaybackRestrictionPolicy`

<details><summary>Stubbed APIs (5) &mdash; routed but return an empty/default response</summary>

- `GetStream`
- `GetStreamSession`
- `ListStreamSessions`
- `ListStreams`
- `StopStream`

</details>

<details><summary>Not yet implemented APIs (5)</summary>

- `CreateAdConfiguration`
- `DeleteAdConfiguration`
- `GetAdConfiguration`
- `InsertAdBreak`
- `ListAdConfigurations`

</details>
