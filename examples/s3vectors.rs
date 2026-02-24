//! Example: S3 Vectors
//!
//! Demonstrates using aws-sdk-s3vectors with winterbaume.
//!
//! Run with:
//!   cargo run --example s3vectors --package winterbaume-examples

use aws_sdk_s3vectors::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_s3vectors::S3VectorsService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(S3VectorsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3vectors::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_s3vectors::Client::new(&config);

    // Create a vector bucket
    let resp = client
        .create_vector_bucket()
        .vector_bucket_name("my-vector-bucket")
        .send()
        .await
        .expect("create_vector_bucket should succeed");

    println!(
        "Created vector bucket ARN: {}",
        resp.vector_bucket_arn().unwrap_or_default()
    );

    // Get the vector bucket
    let get_resp = client
        .get_vector_bucket()
        .vector_bucket_name("my-vector-bucket")
        .send()
        .await
        .expect("get_vector_bucket should succeed");

    let bucket = get_resp.vector_bucket().expect("should have vector_bucket");
    println!("Vector bucket name: {}", bucket.vector_bucket_name());
    println!("Vector bucket ARN: {}", bucket.vector_bucket_arn());

    // List vector buckets
    let list_resp = client
        .list_vector_buckets()
        .send()
        .await
        .expect("list_vector_buckets should succeed");

    println!("Total vector buckets: {}", list_resp.vector_buckets().len());
}
