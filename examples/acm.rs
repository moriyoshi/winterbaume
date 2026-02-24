//! Example: ACM (Certificate Manager)
//!
//! Demonstrates using aws-sdk-acm with winterbaume.
//!
//! Run with:
//!   cargo run --example acm --package winterbaume-examples

use aws_sdk_acm::config::BehaviorVersion;
use winterbaume_acm::AcmService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(AcmService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_acm::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_acm::Client::new(&config);

    let resp = client
        .list_certificates()
        .send()
        .await
        .expect("list_certificates should succeed");
    println!("Certificates: {}", resp.certificate_summary_list().len());
}
