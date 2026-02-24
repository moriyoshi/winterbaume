//! Example: BCM Recommended Actions
//!
//! Demonstrates using aws-sdk-bcmrecommendedactions with winterbaume.
//!
//! Run with:
//!   cargo run --example bcmrecommendedactions --package winterbaume

use aws_sdk_bcmrecommendedactions::config::BehaviorVersion;
use winterbaume_bcmrecommendedactions::BcmRecommendedActionsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(BcmRecommendedActionsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_bcmrecommendedactions::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_bcmrecommendedactions::Client::new(&config);

    let resp = client
        .list_recommended_actions()
        .send()
        .await
        .expect("list_recommended_actions should succeed");
    println!(
        "BCM recommended actions: {}",
        resp.recommended_actions().len()
    );
}
