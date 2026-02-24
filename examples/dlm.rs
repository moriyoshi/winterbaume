//! Example: Data Lifecycle Manager
//!
//! Demonstrates using aws-sdk-dlm with winterbaume.
//!
//! Run with:
//!   cargo run --example dlm --package winterbaume

use aws_sdk_dlm::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_dlm::DlmService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(DlmService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_dlm::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_dlm::Client::new(&config);

    let resp = client
        .get_lifecycle_policies()
        .send()
        .await
        .expect("get_lifecycle_policies should succeed");
    println!("DLM lifecycle policies: {}", resp.policies().len());
}
