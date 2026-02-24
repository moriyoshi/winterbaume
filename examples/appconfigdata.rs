//! Example: AppConfig Data
//!
//! Demonstrates using aws-sdk-appconfigdata with winterbaume.
//!
//! Run with:
//!   cargo run --example appconfigdata --package winterbaume-examples

use aws_sdk_appconfigdata::config::BehaviorVersion;
use winterbaume_appconfigdata::AppConfigDataService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AppConfigDataService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appconfigdata::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_appconfigdata::Client::new(&config);

    let session = client
        .start_configuration_session()
        .application_identifier("MyApp")
        .environment_identifier("Prod")
        .configuration_profile_identifier("MyProfile")
        .send()
        .await
        .expect("start_configuration_session should succeed");
    println!(
        "AppConfigData session token: {}",
        session.initial_configuration_token().unwrap_or("<none>")
    );
}
