//! Example: CloudTrail Data
//!
//! Demonstrates using aws-sdk-cloudtraildata with winterbaume.
//!
//! Run with:
//!   cargo run --example cloudtraildata --package winterbaume

use aws_sdk_cloudtraildata::config::BehaviorVersion;
use aws_sdk_cloudtraildata::types::AuditEvent;
use winterbaume_cloudtraildata::CloudTrailDataService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CloudTrailDataService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudtraildata::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cloudtraildata::Client::new(&config);
    let resp = client
        .put_audit_events()
        .channel_arn("arn:aws:cloudtrail:us-east-1:123:channel/demo")
        .audit_events(
            AuditEvent::builder()
                .id("client-1")
                .event_data(r#"{"eventName":"DemoEvent"}"#)
                .build()
                .expect("event"),
        )
        .send()
        .await
        .expect("put_audit_events should succeed");
    println!("Ingested {} events", resp.successful().len());
}
