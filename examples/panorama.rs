//! Example: AWS Panorama
//!
//! Demonstrates using aws-sdk-panorama with winterbaume.
//!
//! Run with:
//!   cargo run --example panorama --package winterbaume-examples

use aws_sdk_panorama::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_panorama::PanoramaService;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(PanoramaService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_panorama::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_panorama::Client::new(&config);

    // Provision a device
    let prov = client
        .provision_device()
        .name("my-panorama-device")
        .description("A mock Panorama device")
        .send()
        .await
        .expect("provision_device should succeed");

    let device_id = prov.device_id().expect("should have device_id");
    println!("Provisioned device: {device_id}");

    // List devices
    let list = client
        .list_devices()
        .send()
        .await
        .expect("list_devices should succeed");
    println!("Devices: {}", list.devices().len());

    // Describe the device
    let desc = client
        .describe_device()
        .device_id(device_id)
        .send()
        .await
        .expect("describe_device should succeed");
    println!("Device name: {:?}", desc.name());
    println!("Device status: {:?}", desc.provisioning_status());
}
