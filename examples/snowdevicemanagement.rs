//! Example: Snow Device Management
//!
//! Demonstrates using aws-sdk-snowdevicemanagement with winterbaume.
//!
//! Run with:
//!   cargo run --example snowdevicemanagement --package winterbaume

use aws_sdk_snowdevicemanagement::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_snowdevicemanagement::SnowDeviceManagementService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(SnowDeviceManagementService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_snowdevicemanagement::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_snowdevicemanagement::Client::new(&config);
    let resp = client
        .list_devices()
        .send()
        .await
        .expect("list_devices should succeed");
    for d in resp.devices() {
        println!("Device: {:?}", d.managed_device_id());
    }
}
