//! Example: API Gateway
//!
//! Demonstrates using aws-sdk-apigateway with winterbaume.
//!
//! Run with:
//!   cargo run --example apigateway --package winterbaume-examples

use aws_sdk_apigateway::config::BehaviorVersion;
use winterbaume_apigateway::ApiGatewayService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ApiGatewayService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_apigateway::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_apigateway::Client::new(&config);

    let resp = client
        .create_rest_api()
        .name("my-api")
        .description("Example API")
        .send()
        .await
        .expect("create_rest_api should succeed");

    let api_id = resp.id().expect("id should be set");
    println!("Created REST API: {}", api_id);
    println!("Name: {}", resp.name().unwrap_or_default());
    println!("Description: {}", resp.description().unwrap_or_default());

    let get_resp = client
        .get_rest_api()
        .rest_api_id(api_id)
        .send()
        .await
        .expect("get_rest_api should succeed");

    println!("Fetched API name: {}", get_resp.name().unwrap_or_default());
}
