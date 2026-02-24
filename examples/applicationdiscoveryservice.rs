//! Example: Application Discovery Service
//!
//! Demonstrates using aws-sdk-applicationdiscovery with winterbaume.
//!
//! Run with:
//!   cargo run --example applicationdiscoveryservice --package winterbaume

use aws_sdk_applicationdiscovery::config::BehaviorVersion;
use winterbaume_applicationdiscoveryservice::ApplicationDiscoveryService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ApplicationDiscoveryService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_applicationdiscovery::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_applicationdiscovery::Client::new(&config);

    let resp = client
        .describe_agents()
        .send()
        .await
        .expect("describe_agents should succeed");
    println!("Application Discovery agents: {}", resp.agents_info().len());
}
