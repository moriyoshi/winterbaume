//! Example: AMP (Managed Prometheus)
//!
//! Demonstrates using aws-sdk-amp with winterbaume.
//!
//! Run with:
//!   cargo run --example amp --package winterbaume-examples

use aws_sdk_amp::config::BehaviorVersion;
use winterbaume_amp::AmpService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder().with_service(AmpService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_amp::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_amp::Client::new(&config);

    let resp = client
        .list_workspaces()
        .send()
        .await
        .expect("list_workspaces should succeed");
    println!("Workspaces: {}", resp.workspaces().len());
}
