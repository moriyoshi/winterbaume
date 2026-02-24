//! Example: Service Discovery
//!
//! Demonstrates using aws-sdk-servicediscovery with winterbaume.
//!
//! Run with:
//!   cargo run --example servicediscovery --package winterbaume-examples

use aws_sdk_servicediscovery::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_servicediscovery::ServiceDiscoveryService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ServiceDiscoveryService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_servicediscovery::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_servicediscovery::Client::new(&config);

    let resp = client
        .list_services()
        .send()
        .await
        .expect("list_services should succeed");
    println!("Service Discovery services: {}", resp.services().len());
}
