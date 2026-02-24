//! Example: SNS
//!
//! Demonstrates using aws-sdk-sns with winterbaume.
//!
//! Run with:
//!   cargo run --example sns --package winterbaume-examples

use aws_sdk_sns::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sns::SnsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(SnsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sns::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_sns::Client::new(&config);

    client
        .create_topic()
        .name("example-topic")
        .send()
        .await
        .expect("create_topic should succeed");
    let resp = client
        .list_topics()
        .send()
        .await
        .expect("list_topics should succeed");
    println!("SNS topics: {}", resp.topics().len());
}
