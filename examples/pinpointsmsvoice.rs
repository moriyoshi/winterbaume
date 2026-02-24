//! Example: Pinpoint SMS Voice
//!
//! Demonstrates using aws-sdk-pinpointsmsvoice with winterbaume.
//!
//! Run with:
//!   cargo run --example pinpointsmsvoice --package winterbaume

use aws_sdk_pinpointsmsvoice::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_pinpointsmsvoice::PinpointSmsVoiceService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(PinpointSmsVoiceService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_pinpointsmsvoice::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_pinpointsmsvoice::Client::new(&config);
    client
        .create_configuration_set()
        .configuration_set_name("demo-set")
        .send()
        .await
        .expect("create_configuration_set should succeed");
    let resp = client
        .list_configuration_sets()
        .send()
        .await
        .expect("list_configuration_sets should succeed");
    println!("Configuration sets: {:?}", resp.configuration_sets());
}
