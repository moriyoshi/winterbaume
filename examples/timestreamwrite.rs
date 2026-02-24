//! Example: Timestream Write
//!
//! Demonstrates using aws-sdk-timestreamwrite with winterbaume.
//!
//! Run with:
//!   cargo run --example timestreamwrite --package winterbaume-examples

use aws_sdk_timestreamwrite::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_timestreamwrite::TimestreamWriteService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(TimestreamWriteService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_timestreamwrite::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_timestreamwrite::Client::new(&config);

    let resp = client
        .list_databases()
        .send()
        .await
        .expect("list_databases should succeed");
    println!("Timestream Write databases: {}", resp.databases().len());
}
