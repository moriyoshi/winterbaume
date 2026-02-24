//! Example: EC2
//!
//! Demonstrates using aws-sdk-ec2 with winterbaume.
//!
//! Run with:
//!   cargo run --example ec2 --package winterbaume-examples

use aws_sdk_ec2::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_ec2::Ec2Service;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(Ec2Service::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_ec2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_ec2::Client::new(&config);

    // Describe availability zones
    let resp = client
        .describe_availability_zones()
        .send()
        .await
        .expect("describe_availability_zones should succeed");

    println!("Availability Zones:");
    for az in resp.availability_zones() {
        println!(
            "  {} (state: {})",
            az.zone_name().unwrap_or_default(),
            az.state().map(|s| s.as_str()).unwrap_or("unknown"),
        );
    }

    // Describe account attributes
    let resp = client
        .describe_account_attributes()
        .send()
        .await
        .expect("describe_account_attributes should succeed");

    println!("\nAccount Attributes:");
    for attr in resp.account_attributes() {
        println!(
            "  {}: {:?}",
            attr.attribute_name().unwrap_or_default(),
            attr.attribute_values()
                .iter()
                .map(|v| v.attribute_value().unwrap_or_default())
                .collect::<Vec<_>>(),
        );
    }
}
