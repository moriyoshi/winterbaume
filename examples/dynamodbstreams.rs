//! Example: DynamoDB Streams
//!
//! Demonstrates using aws-sdk-dynamodbstreams with winterbaume.
//!
//! Run with:
//!   cargo run --example dynamodbstreams --package winterbaume-examples

use aws_sdk_dynamodbstreams::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_dynamodbstreams::DynamoDbStreamsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(DynamoDbStreamsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dynamodbstreams::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_dynamodbstreams::Client::new(&config);

    let resp = client
        .list_streams()
        .send()
        .await
        .expect("list_streams should succeed");
    println!("DynamoDB streams: {}", resp.streams().len());
}
