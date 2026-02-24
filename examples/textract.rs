//! Example: Textract
//!
//! Demonstrates using aws-sdk-textract with winterbaume.
//!
//! Run with:
//!   cargo run --example textract --package winterbaume-examples

use aws_sdk_textract::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_textract::TextractService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(TextractService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_textract::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_textract::Client::new(&config);

    let resp = client
        .list_adapters()
        .send()
        .await
        .expect("list_adapters should succeed");
    println!("Textract adapters: {}", resp.adapters().len());
}
