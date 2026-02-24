//! Example: MQ
//!
//! Demonstrates using aws-sdk-mq with winterbaume.
//!
//! Run with:
//!   cargo run --example mq --package winterbaume-examples

use aws_sdk_mq::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_mq::MqService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(MqService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_mq::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_mq::Client::new(&config);

    let resp = client
        .list_brokers()
        .send()
        .await
        .expect("list_brokers should succeed");
    println!("MQ brokers: {}", resp.broker_summaries().len());
}
