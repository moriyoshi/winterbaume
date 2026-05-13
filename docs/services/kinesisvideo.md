# winterbaume-kinesisvideo

Amazon Kinesis Video Streams service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | Kinesis Video |
| AWS model | `kinesis-video` |
| Protocol | restJson1 |
| winterbaume coverage | 32/32 operations (100.0%) |
| stubs (routed, returns empty/default) | 0/32 operations (0.0%) |
| moto coverage | 0/32 operations (0.0%) |
| floci coverage | 0/32 operations (0.0%) |
| kumo coverage | 0/32 operations (0.0%) |
| Coverage report date | 2026-05-13 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws kinesisvideo list-streams
```

## Example

```rust
use aws_sdk_kinesisvideo::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_kinesisvideo::KinesisVideoService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(KinesisVideoService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_kinesisvideo::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_kinesisvideo::Client::new(&config);

    let resp = client
        .list_streams()
        .send()
        .await
        .expect("list_streams should succeed");
    println!("Kinesis Video streams: {}", resp.stream_info_list().len());
}
```

## Implemented APIs (32)

- `CreateSignalingChannel`
- `CreateStream`
- `DeleteEdgeConfiguration`
- `DeleteSignalingChannel`
- `DeleteStream`
- `DescribeEdgeConfiguration`
- `DescribeImageGenerationConfiguration`
- `DescribeMappedResourceConfiguration`
- `DescribeMediaStorageConfiguration`
- `DescribeNotificationConfiguration`
- `DescribeSignalingChannel`
- `DescribeStream`
- `DescribeStreamStorageConfiguration`
- `GetDataEndpoint`
- `GetSignalingChannelEndpoint`
- `ListEdgeAgentConfigurations`
- `ListSignalingChannels`
- `ListStreams`
- `ListTagsForResource`
- `ListTagsForStream`
- `StartEdgeConfigurationUpdate`
- `TagResource`
- `TagStream`
- `UntagResource`
- `UntagStream`
- `UpdateDataRetention`
- `UpdateImageGenerationConfiguration`
- `UpdateMediaStorageConfiguration`
- `UpdateNotificationConfiguration`
- `UpdateSignalingChannel`
- `UpdateStream`
- `UpdateStreamStorageConfiguration`
