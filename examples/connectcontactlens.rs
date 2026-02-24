//! Example: Connect Contact Lens
//!
//! Demonstrates using aws-sdk-connectcontactlens with winterbaume.
//!
//! Run with:
//!   cargo run --example connectcontactlens --package winterbaume

use aws_sdk_connectcontactlens::config::BehaviorVersion;
use winterbaume_connectcontactlens::ConnectContactLensService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ConnectContactLensService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_connectcontactlens::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_connectcontactlens::Client::new(&config);
    let resp = client
        .list_realtime_contact_analysis_segments()
        .instance_id("11111111-1111-1111-1111-111111111111")
        .contact_id("22222222-2222-2222-2222-222222222222")
        .send()
        .await
        .expect("list");
    println!("Segments: {}", resp.segments().len());
}
