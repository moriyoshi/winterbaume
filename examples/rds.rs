//! Example: RDS
//!
//! Demonstrates using aws-sdk-rds with winterbaume.
//!
//! Run with:
//!   cargo run --example rds --package winterbaume-examples

use aws_sdk_rds::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_rds::RdsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(RdsService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_rds::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_rds::Client::new(&config);

    // Create a DB instance
    let resp = client
        .create_db_instance()
        .db_instance_identifier("example-db")
        .db_instance_class("db.t3.micro")
        .engine("mysql")
        .send()
        .await
        .expect("create_db_instance should succeed");

    let inst = resp.db_instance().expect("should have db_instance");
    println!(
        "Created DB instance: {}",
        inst.db_instance_identifier().unwrap_or_default()
    );
    println!("Engine: {}", inst.engine().unwrap_or_default());
    println!("Status: {}", inst.db_instance_status().unwrap_or_default());
    println!("ARN: {}", inst.db_instance_arn().unwrap_or_default());

    // Describe DB instances
    let desc = client
        .describe_db_instances()
        .send()
        .await
        .expect("describe_db_instances should succeed");

    println!("\nTotal DB instances: {}", desc.db_instances().len());
}
