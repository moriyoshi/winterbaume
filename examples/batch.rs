//! Example: Batch
//!
//! Demonstrates using aws-sdk-batch with winterbaume.
//!
//! Run with:
//!   cargo run --example batch --package winterbaume-examples

use aws_sdk_batch::config::BehaviorVersion;
use winterbaume_batch::BatchService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(BatchService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_batch::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_batch::Client::new(&config);

    let resp = client
        .describe_job_queues()
        .send()
        .await
        .expect("describe_job_queues should succeed");
    println!("Job queues: {}", resp.job_queues().len());
}
