//! Example: API Gateway V2
//!
//! Demonstrates using aws-sdk-apigatewayv2 with winterbaume.
//!
//! Run with:
//!   cargo run --example apigatewayv2 --package winterbaume-examples

use aws_sdk_apigatewayv2::config::BehaviorVersion;
use winterbaume_apigatewayv2::ApiGatewayV2Service;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ApiGatewayV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_apigatewayv2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_apigatewayv2::Client::new(&config);

    let resp = client
        .get_apis()
        .send()
        .await
        .expect("get_apis should succeed");

    println!("APIs: {:?}", resp.items());
}
