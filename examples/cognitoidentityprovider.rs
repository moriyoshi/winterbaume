//! Example: Cognito Identity Provider
//!
//! Demonstrates using aws-sdk-cognitoidentityprovider with winterbaume.
//!
//! Run with:
//!   cargo run --example cognitoidentityprovider --package winterbaume-examples

use aws_sdk_cognitoidentityprovider::config::BehaviorVersion;
use winterbaume_cognitoidentityprovider::CognitoIdentityProviderService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CognitoIdentityProviderService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cognitoidentityprovider::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_cognitoidentityprovider::Client::new(&config);

    let resp = client
        .list_user_pools()
        .max_results(60)
        .send()
        .await
        .expect("list_user_pools should succeed");
    println!("User pools: {}", resp.user_pools().len());
}
