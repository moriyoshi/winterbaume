# winterbaume-mediapackagev2

MediaPackage v2 service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | MediaPackage v2 |
| AWS model | `mediapackagev2` |
| Protocol | restJson1 |
| winterbaume coverage | 7/30 operations (23.3%) |
| stubs (routed, returns empty/default) | 0/30 operations (0.0%) |
| moto coverage | 7/30 operations (23.3%) |
| floci coverage | 0/30 operations (0.0%) |
| kumo coverage | 0/30 operations (0.0%) |
| Coverage report date | 2026-05-17 |

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
aws mediapackagev2 list-channel-groups
```

## Example

```rust
use aws_sdk_mediapackagev2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_mediapackagev2::MediaPackageV2Service;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(MediaPackageV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_mediapackagev2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_mediapackagev2::Client::new(&config);

    let resp = client
        .list_channel_groups()
        .send()
        .await
        .expect("list_channel_groups should succeed");
    println!("MediaPackage v2 channel groups: {}", resp.items().len());
}
```

## Implemented APIs (7)

- `CreateChannel`
- `CreateChannelGroup`
- `DeleteChannel`
- `DeleteChannelGroup`
- `GetChannel`
- `GetChannelGroup`
- `ListChannelGroups`

<details><summary>Not yet implemented APIs (23)</summary>

- `CancelHarvestJob`
- `CreateHarvestJob`
- `CreateOriginEndpoint`
- `DeleteChannelPolicy`
- `DeleteOriginEndpoint`
- `DeleteOriginEndpointPolicy`
- `GetChannelPolicy`
- `GetHarvestJob`
- `GetOriginEndpoint`
- `GetOriginEndpointPolicy`
- `ListChannels`
- `ListHarvestJobs`
- `ListOriginEndpoints`
- `ListTagsForResource`
- `PutChannelPolicy`
- `PutOriginEndpointPolicy`
- `ResetChannelState`
- `ResetOriginEndpointState`
- `TagResource`
- `UntagResource`
- `UpdateChannel`
- `UpdateChannelGroup`
- `UpdateOriginEndpoint`

</details>
