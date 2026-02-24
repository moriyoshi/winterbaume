# winterbaume-kinesisvideoarchivedmedia

Kinesis Video Archived Media service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Kinesis Video Archived Media |
| AWS model | `kinesis-video-archived-media` |
| Protocol | restJson1 |
| winterbaume coverage | 6/6 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/6 operations (0.0%) |
| moto coverage | 3/6 operations (50.0%) |
| floci coverage | 0/6 operations (0.0%) |
| kumo coverage | 0/6 operations (0.0%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws kinesis-video-archived-media list-fragments --stream-arn arn:aws:kinesisvideo:us-east-1:123456789012:stream/my-stream/000000000000
```

## Example

```rust
use aws_sdk_kinesisvideoarchivedmedia::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_kinesisvideoarchivedmedia::KinesisVideoArchivedMediaService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(KinesisVideoArchivedMediaService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_kinesisvideoarchivedmedia::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_kinesisvideoarchivedmedia::Client::new(&config);

    // Kinesis Video Archived Media requires a stream ARN or name.
    // This example demonstrates client setup for the KinesisVideoArchivedMedia service.
    println!(
        "KinesisVideoArchivedMedia client ready. Use get_hls_streaming_session_url() to stream."
    );
    let _client = client;
}
```

## Implemented APIs (6)

- `GetClip`
- `GetDASHStreamingSessionURL`
- `GetHLSStreamingSessionURL`
- `GetImages`
- `GetMediaForFragmentList`
- `ListFragments`
