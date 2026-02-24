//! Example: EC2 Instance Connect
//!
//! Demonstrates using aws-sdk-ec2instanceconnect with winterbaume.
//!
//! Run with:
//!   cargo run --example ec2instanceconnect --package winterbaume-examples

use aws_sdk_ec2instanceconnect::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ec2instanceconnect::Ec2InstanceConnectService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(Ec2InstanceConnectService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ec2instanceconnect::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_ec2instanceconnect::Client::new(&config);

    // EC2 Instance Connect sends SSH keys to running instances.
    // This example demonstrates client setup for the EC2 Instance Connect service.
    println!("EC2 Instance Connect client ready. Use send_ssh_public_key() to push an SSH key.");
    let _client = client;
}
