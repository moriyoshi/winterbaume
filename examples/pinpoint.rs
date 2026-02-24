//! Example: Pinpoint
//!
//! Demonstrates using aws-sdk-pinpoint with winterbaume.
//!
//! Run with:
//!   cargo run --example pinpoint --package winterbaume-examples

use aws_sdk_pinpoint::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_pinpoint::PinpointService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(PinpointService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_pinpoint::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_pinpoint::Client::new(&config);

    let resp = client
        .get_apps()
        .send()
        .await
        .expect("get_apps should succeed");
    let count = resp
        .applications_response()
        .map(|r| r.item().len())
        .unwrap_or(0);
    println!("Pinpoint apps: {}", count);
}
