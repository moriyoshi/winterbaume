//! Example: Cognito Identity
//!
//! Demonstrates using aws-sdk-cognitoidentity with winterbaume.
//!
//! Run with:
//!   cargo run --example cognitoidentity --package winterbaume-examples

use aws_sdk_cognitoidentity::config::BehaviorVersion;
use winterbaume_cognitoidentity::CognitoIdentityService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CognitoIdentityService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cognitoidentity::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cognitoidentity::Client::new(&config);

    let resp = client
        .list_identity_pools()
        .max_results(10)
        .send()
        .await
        .expect("list_identity_pools should succeed");
    println!("Identity pools: {}", resp.identity_pools().len());
}
