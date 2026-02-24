//! Example: S3 Tables
//!
//! Demonstrates using aws-sdk-s3tables with winterbaume.
//!
//! Run with:
//!   cargo run --example s3tables --package winterbaume-examples

use aws_sdk_s3tables::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_s3tables::S3TablesService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(S3TablesService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3tables::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_s3tables::Client::new(&config);

    // Create a table bucket
    let resp = client
        .create_table_bucket()
        .name("my-table-bucket")
        .send()
        .await
        .expect("create_table_bucket should succeed");

    let arn = resp.arn();
    println!("Created table bucket ARN: {}", arn);

    // Get the table bucket
    let get_resp = client
        .get_table_bucket()
        .table_bucket_arn(arn)
        .send()
        .await
        .expect("get_table_bucket should succeed");

    println!("Table bucket name: {}", get_resp.name());
    println!("Table bucket ARN: {}", get_resp.arn());

    // List table buckets
    let list_resp = client
        .list_table_buckets()
        .send()
        .await
        .expect("list_table_buckets should succeed");

    println!("Total table buckets: {}", list_resp.table_buckets().len());
}
