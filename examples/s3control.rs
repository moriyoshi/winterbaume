//! Example: S3 Control
//!
//! Demonstrates using aws-sdk-s3control with winterbaume.
//!
//! Run with:
//!   cargo run --example s3control --package winterbaume-examples

use aws_sdk_s3control::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_s3control::S3ControlService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(S3ControlService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_s3control::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_s3control::Client::new(&config);

    // Create an access point
    let resp = client
        .create_access_point()
        .account_id("123456789012")
        .name("my-ap")
        .bucket("my-bucket")
        .send()
        .await
        .expect("create_access_point should succeed");

    println!(
        "Created access point ARN: {}",
        resp.access_point_arn().unwrap_or_default()
    );

    // Get the access point
    let get_resp = client
        .get_access_point()
        .account_id("123456789012")
        .name("my-ap")
        .send()
        .await
        .expect("get_access_point should succeed");

    println!("Access point name: {}", get_resp.name().unwrap_or_default());
    println!("Bucket: {}", get_resp.bucket().unwrap_or_default());
    println!(
        "Network origin: {}",
        get_resp
            .network_origin()
            .map(|n| n.as_str())
            .unwrap_or("unknown")
    );
}
