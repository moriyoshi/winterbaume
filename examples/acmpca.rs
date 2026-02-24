//! Example: ACM PCA (Private CA)
//!
//! Demonstrates using aws-sdk-acmpca with winterbaume.
//!
//! Run with:
//!   cargo run --example acmpca --package winterbaume-examples

use aws_sdk_acmpca::config::BehaviorVersion;
use winterbaume_acmpca::AcmPcaService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AcmPcaService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_acmpca::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_acmpca::Client::new(&config);

    let resp = client
        .list_certificate_authorities()
        .send()
        .await
        .expect("list_certificate_authorities should succeed");
    println!(
        "Certificate authorities: {}",
        resp.certificate_authorities().len()
    );
}
