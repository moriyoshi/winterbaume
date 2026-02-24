//! Example: Polly
//!
//! Demonstrates using aws-sdk-polly with winterbaume.
//!
//! Run with:
//!   cargo run --example polly --package winterbaume-examples

use aws_sdk_polly::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_polly::PollyService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(PollyService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_polly::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_polly::Client::new(&config);

    let resp = client
        .describe_voices()
        .send()
        .await
        .expect("describe_voices should succeed");
    println!("Polly voices: {}", resp.voices().len());
}
