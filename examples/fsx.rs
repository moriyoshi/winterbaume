//! Example: FSx
//!
//! Demonstrates using aws-sdk-fsx with winterbaume.
//!
//! Run with:
//!   cargo run --example fsx --package winterbaume-examples

use aws_sdk_fsx::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_fsx::FsxService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(FsxService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_fsx::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_fsx::Client::new(&config);

    let resp = client
        .describe_file_systems()
        .send()
        .await
        .expect("describe_file_systems should succeed");
    println!("FSx file systems: {}", resp.file_systems().len());
}
