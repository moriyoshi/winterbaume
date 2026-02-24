//! Example: Cloud9
//!
//! Demonstrates using aws-sdk-cloud9 with winterbaume.
//!
//! Run with:
//!   cargo run --example cloud9 --package winterbaume

use aws_sdk_cloud9::config::BehaviorVersion;
use winterbaume_cloud9::Cloud9Service;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(Cloud9Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloud9::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cloud9::Client::new(&config);
    let create = client
        .create_environment_ec2()
        .name("demo")
        .image_id("amazonlinux-2023-x86_64")
        .instance_type("t3.small")
        .send()
        .await
        .expect("create_environment_ec2 should succeed");
    println!("Created environment: {:?}", create.environment_id());
}
