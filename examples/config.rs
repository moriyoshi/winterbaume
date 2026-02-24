//! Example: Config
//!
//! Demonstrates using aws-sdk-config with winterbaume.
//!
//! Run with:
//!   cargo run --example config --package winterbaume-examples

use aws_sdk_config::config::BehaviorVersion;
use winterbaume_config::ConfigService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ConfigService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_config::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_config::Client::new(&config);

    let resp = client
        .describe_configuration_recorders()
        .send()
        .await
        .expect("describe_configuration_recorders should succeed");
    println!(
        "Configuration recorders: {}",
        resp.configuration_recorders().len()
    );
}
