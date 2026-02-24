//! Example: CloudFront KeyValueStore
//!
//! Demonstrates using aws-sdk-cloudfrontkeyvaluestore with winterbaume.
//!
//! Run with:
//!   cargo run --example cloudfrontkeyvaluestore --package winterbaume

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
