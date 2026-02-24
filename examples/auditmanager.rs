//! Example: AWS Audit Manager
//!
//! Demonstrates using aws-sdk-auditmanager with winterbaume.
//!
//! Run with:
//!   cargo run --example auditmanager --package winterbaume-examples

use aws_sdk_auditmanager::config::BehaviorVersion;
use winterbaume_auditmanager::AuditManagerService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AuditManagerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_auditmanager::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_auditmanager::Client::new(&config);

    let resp = client
        .list_controls()
        .control_type(aws_sdk_auditmanager::types::ControlType::Custom)
        .send()
        .await
        .expect("list_controls should succeed");
    println!("Audit Manager controls: {:?}", resp.control_metadata_list());
}
