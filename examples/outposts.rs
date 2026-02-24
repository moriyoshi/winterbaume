//! Example: AWS Outposts
//!
//! Demonstrates using aws-sdk-outposts with winterbaume.
//!
//! Run with:
//!   cargo run --example outposts --package winterbaume-examples

use aws_sdk_outposts::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_outposts::OutpostsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(OutpostsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_outposts::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_outposts::Client::new(&config);

    let resp = client
        .list_sites()
        .send()
        .await
        .expect("list_sites should succeed");
    println!("Outposts sites: {}", resp.sites().len());
}
