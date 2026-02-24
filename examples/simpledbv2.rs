//! Example: SimpleDB
//!
//! Demonstrates using aws-sdk-simpledbv2 with winterbaume.
//!
//! Run with:
//!   cargo run --example simpledbv2 --package winterbaume-examples

use aws_sdk_simpledbv2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_simpledbv2::SimpleDbV2Service;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SimpleDbV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_simpledbv2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_simpledbv2::Client::new(&config);

    let resp = client
        .list_exports()
        .send()
        .await
        .expect("list_exports should succeed");
    println!(
        "SimpleDB export summaries: {}",
        resp.export_summaries().len()
    );
}
