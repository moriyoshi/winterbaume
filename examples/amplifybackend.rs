//! Example: Amplify Backend
//!
//! Demonstrates using aws-sdk-amplifybackend with winterbaume.
//!
//! Run with:
//!   cargo run --example amplifybackend --package winterbaume-examples

use aws_sdk_amplifybackend::config::BehaviorVersion;
use winterbaume_amplifybackend::AmplifyBackendService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AmplifyBackendService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_amplifybackend::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_amplifybackend::Client::new(&config);

    let resp = client
        .create_backend()
        .app_id("d1example")
        .app_name("ExampleApp")
        .backend_environment_name("dev")
        .send()
        .await
        .expect("create_backend should succeed");
    println!(
        "AmplifyBackend created: app={:?} env={:?}",
        resp.app_id(),
        resp.backend_environment_name()
    );
}
