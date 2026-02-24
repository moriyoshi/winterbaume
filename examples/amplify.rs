//! Example: Amplify
//!
//! Demonstrates using aws-sdk-amplify with winterbaume.
//!
//! Run with:
//!   cargo run --example amplify --package winterbaume-examples

use aws_sdk_amplify::config::BehaviorVersion;
use winterbaume_amplify::AmplifyService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AmplifyService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_amplify::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_amplify::Client::new(&config);

    let resp = client
        .list_apps()
        .send()
        .await
        .expect("list_apps should succeed");
    println!("Amplify apps: {}", resp.apps().len());
}
