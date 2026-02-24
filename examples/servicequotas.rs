//! Example: Service Quotas
//!
//! Demonstrates using aws-sdk-servicequotas with winterbaume.
//!
//! Run with:
//!   cargo run --example servicequotas --package winterbaume-examples

use aws_sdk_servicequotas::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_servicequotas::ServiceQuotasService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ServiceQuotasService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_servicequotas::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_servicequotas::Client::new(&config);

    let resp = client
        .list_services()
        .send()
        .await
        .expect("list_services should succeed");
    println!("Service Quotas services: {}", resp.services().len());
}
