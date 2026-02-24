//! Example: API Gateway Management API
//!
//! Demonstrates using aws-sdk-apigatewaymanagement with winterbaume.
//!
//! Run with:
//!   cargo run --example apigatewaymanagement --package winterbaume-examples

use aws_sdk_apigatewaymanagement::config::BehaviorVersion;
use winterbaume_apigatewaymanagement::ApiGatewayManagementService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ApiGatewayManagementService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_apigatewaymanagement::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_apigatewaymanagement::Client::new(&config);

    let resp = client
        .get_connection()
        .connection_id("example-conn-001")
        .send()
        .await
        .expect("get_connection should succeed");

    println!(
        "API Gateway Management API: connectedAt={:?}, identity={:?}",
        resp.connected_at(),
        resp.identity()
    );
}
