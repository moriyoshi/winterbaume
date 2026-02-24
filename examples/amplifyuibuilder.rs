//! Example: Amplify UI Builder
//!
//! Demonstrates using aws-sdk-amplifyuibuilder with winterbaume.
//!
//! Run with:
//!   cargo run --example amplifyuibuilder --package winterbaume

use aws_sdk_amplifyuibuilder::config::BehaviorVersion;
use winterbaume_amplifyuibuilder::AmplifyUiBuilderService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AmplifyUiBuilderService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_amplifyuibuilder::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_amplifyuibuilder::Client::new(&config);

    let resp = client
        .list_components()
        .app_id("app1")
        .environment_name("staging")
        .send()
        .await
        .expect("list_components should succeed");
    println!("Amplify UI Builder components: {}", resp.entities().len());
}
