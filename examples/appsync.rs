//! Example: AppSync
//!
//! Demonstrates using aws-sdk-appsync with winterbaume.
//!
//! Run with:
//!   cargo run --example appsync --package winterbaume-examples

use aws_sdk_appsync::config::BehaviorVersion;
use winterbaume_appsync::AppSyncService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(AppSyncService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appsync::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_appsync::Client::new(&config);

    let resp = client
        .list_graphql_apis()
        .send()
        .await
        .expect("list_graphql_apis should succeed");
    println!("GraphQL APIs: {}", resp.graphql_apis().len());
}
