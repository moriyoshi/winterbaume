//! Example: EFS
//!
//! Demonstrates using aws-sdk-efs with winterbaume.
//!
//! Run with:
//!   cargo run --example efs --package winterbaume-examples

use aws_sdk_efs::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_efs::EfsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(EfsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_efs::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_efs::Client::new(&config);

    let resp = client
        .describe_file_systems()
        .send()
        .await
        .expect("describe_file_systems should succeed");
    println!("EFS file systems: {}", resp.file_systems().len());
}
