# winterbaume-cloudfrontkeyvaluestore

AWS CloudFront KeyValueStore service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | CloudFront KeyValueStore |
| AWS model | `cloudfront-keyvaluestore` |
| Protocol | restJson1 |
| winterbaume coverage | 5/6 operations (83.3%) |
| stubs (routed, returns empty/default) | 0/6 operations (0.0%) |
| moto coverage | 0/6 operations (0.0%) |
| floci coverage | 0/6 operations (0.0%) |
| kumo coverage | 0/6 operations (0.0%) |
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
aws cloudfrontkeyvaluestore help
```

## Example

```rust
use aws_sdk_cloudfrontkeyvaluestore::config::BehaviorVersion;
use winterbaume_cloudfrontkeyvaluestore::CloudFrontKvsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CloudFrontKvsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudfrontkeyvaluestore::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_cloudfrontkeyvaluestore::Client::new(&config);
    let arn = "arn:aws:cloudfront::123:key-value-store/demo";
    let initial = client
        .describe_key_value_store()
        .kvs_arn(arn)
        .send()
        .await
        .expect("describe should succeed");
    let etag = initial.e_tag();
    client
        .put_key()
        .kvs_arn(arn)
        .key("greeting")
        .value("hello")
        .if_match(etag)
        .send()
        .await
        .expect("put should succeed");
    println!(
        "Stored greeting in {} (item count: {:?})",
        arn,
        initial.item_count()
    );
}
```

## Implemented APIs (5)

- `DeleteKey`
- `GetKey`
- `ListKeys`
- `PutKey`
- `UpdateKeys`

<details><summary>Not yet implemented APIs (1)</summary>

- `DescribeKeyValueStore`

</details>
