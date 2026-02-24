//! Example: Directory Service
//!
//! Demonstrates using aws-sdk-directory with winterbaume.
//!
//! Run with:
//!   cargo run --example directory --package winterbaume-examples

use aws_sdk_directory::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_directory::DirectoryService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(DirectoryService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_directory::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_directory::Client::new(&config);

    let resp = client
        .describe_directories()
        .send()
        .await
        .expect("describe_directories should succeed");
    println!("Directories: {}", resp.directory_descriptions().len());
}
