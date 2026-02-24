//! Example: Cloud Directory
//!
//! Demonstrates using aws-sdk-clouddirectory with winterbaume.
//!
//! Run with:
//!   cargo run --example clouddirectory --package winterbaume-examples

use aws_sdk_clouddirectory::config::BehaviorVersion;
use winterbaume_clouddirectory::CloudDirectoryService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CloudDirectoryService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_clouddirectory::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_clouddirectory::Client::new(&config);

    // Create a schema
    let resp = client
        .create_schema()
        .name("ExampleSchema")
        .send()
        .await
        .expect("create_schema should succeed");

    let schema_arn = resp.schema_arn().expect("should have schema_arn");
    println!("Created schema ARN: {}", schema_arn);

    // Delete the schema
    client
        .delete_schema()
        .schema_arn(schema_arn)
        .send()
        .await
        .expect("delete_schema should succeed");

    println!("Deleted schema successfully");
}
