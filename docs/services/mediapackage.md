# winterbaume-mediapackage

AWS Elemental MediaPackage service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | MediaPackage |
| AWS model | `mediapackage` |
| Protocol | restJson1 |
| winterbaume coverage | 9/19 operations (47.4%) |
| stubs (routed, returns empty/default) | 0/19 operations (0.0%) |
| moto coverage | 9/19 operations (47.4%) |
| floci coverage | 0/19 operations (0.0%) |
| kumo coverage | 0/19 operations (0.0%) |
| fakecloud coverage | 0/19 operations (0.0%) |
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
aws mediapackage list-channels
```

## Example

```rust
use aws_sdk_mediapackage::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_mediapackage::MediaPackageService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(MediaPackageService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_mediapackage::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_mediapackage::Client::new(&config);

    let resp = client
        .list_channels()
        .send()
        .await
        .expect("list_channels should succeed");
    println!("MediaPackage channels: {}", resp.channels().len());
}
```

## Implemented APIs (9)

- `CreateChannel`
- `CreateOriginEndpoint`
- `DeleteChannel`
- `DeleteOriginEndpoint`
- `DescribeChannel`
- `DescribeOriginEndpoint`
- `ListChannels`
- `ListOriginEndpoints`
- `UpdateOriginEndpoint`

<details><summary>Not yet implemented APIs (10)</summary>

- `ConfigureLogs`
- `CreateHarvestJob`
- `DescribeHarvestJob`
- `ListHarvestJobs`
- `ListTagsForResource`
- `RotateChannelCredentials`
- `RotateIngestEndpointCredentials`
- `TagResource`
- `UntagResource`
- `UpdateChannel`

</details>
