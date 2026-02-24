//! Example: Amazon S3 Files
//!
//! Demonstrates using aws-sdk-s3files with winterbaume.
//!
//! Run with:
//!   cargo run --example s3files --package winterbaume-examples

use aws_sdk_s3files::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_s3files::S3FilesService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(S3FilesService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3files::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_s3files::Client::new(&config);

    let resp = client
        .list_file_systems()
        .send()
        .await
        .expect("list_file_systems should succeed");
    println!("S3 Files: {:?}", resp.file_systems());
}
