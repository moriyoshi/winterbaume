//! Example: SQS
//!
//! Demonstrates using aws-sdk-sqs with winterbaume.
//!
//! Run with:
//!   cargo run --example sqs --package winterbaume-examples

use aws_sdk_sqs::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sqs::SqsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(SqsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sqs::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_sqs::Client::new(&config);

    client
        .create_queue()
        .queue_name("example-queue")
        .send()
        .await
        .expect("create_queue should succeed");
    let resp = client
        .list_queues()
        .send()
        .await
        .expect("list_queues should succeed");
    println!("SQS queues: {:?}", resp.queue_urls());
}
