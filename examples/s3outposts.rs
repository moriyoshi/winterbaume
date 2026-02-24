//! Example: S3 on Outposts
//!
//! Demonstrates using aws-sdk-s3outposts with winterbaume.
//!
//! Run with:
//!   cargo run --example s3outposts --package winterbaume

use aws_sdk_s3outposts::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_s3outposts::S3OutpostsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(S3OutpostsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3outposts::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_s3outposts::Client::new(&config);
    let resp = client
        .list_outposts_with_s3()
        .send()
        .await
        .expect("list_outposts_with_s3 should succeed");
    for o in resp.outposts() {
        println!(
            "Outpost: {:?} ({} bytes)",
            o.outpost_id(),
            o.capacity_in_bytes()
        );
    }
}
