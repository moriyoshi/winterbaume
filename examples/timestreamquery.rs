//! Example: Timestream Query
//!
//! Demonstrates using aws-sdk-timestreamquery with winterbaume.
//!
//! Run with:
//!   cargo run --example timestreamquery --package winterbaume-examples

use aws_sdk_timestreamquery::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_timestreamquery::TimestreamQueryService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(TimestreamQueryService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_timestreamquery::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_timestreamquery::Client::new(&config);

    let resp = client
        .list_scheduled_queries()
        .send()
        .await
        .expect("list_scheduled_queries should succeed");
    println!(
        "Timestream scheduled queries: {}",
        resp.scheduled_queries().len()
    );
}
