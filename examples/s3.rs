//! Example: S3
//!
//! Demonstrates using aws-sdk-s3 with winterbaume.
//!
//! Run with:
//!   cargo run --example s3 --package winterbaume-examples

use aws_sdk_s3::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_s3::S3Service;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(S3Service::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_s3::Client::new(&config);

    client
        .create_bucket()
        .bucket("example-bucket")
        .send()
        .await
        .expect("create_bucket should succeed");
    let resp = client
        .list_buckets()
        .send()
        .await
        .expect("list_buckets should succeed");
    println!(
        "S3 buckets: {:?}",
        resp.buckets()
            .iter()
            .map(|b| b.name().unwrap_or_default())
            .collect::<Vec<_>>()
    );
}
