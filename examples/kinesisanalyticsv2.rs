//! Example: Kinesis Analytics V2
//!
//! Demonstrates using aws-sdk-kinesisanalyticsv2 with winterbaume.
//!
//! Run with:
//!   cargo run --example kinesisanalyticsv2 --package winterbaume-examples

use aws_sdk_kinesisanalyticsv2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_kinesisanalyticsv2::KinesisAnalyticsV2Service;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(KinesisAnalyticsV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_kinesisanalyticsv2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_kinesisanalyticsv2::Client::new(&config);

    let resp = client
        .list_applications()
        .send()
        .await
        .expect("list_applications should succeed");
    println!(
        "Kinesis Analytics V2 applications: {}",
        resp.application_summaries().len()
    );
}
