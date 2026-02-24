//! Example: CodeStar Notifications
//!
//! Demonstrates using aws-sdk-codestarnotifications with winterbaume.
//!
//! Run with:
//!   cargo run --example codestarnotifications --package winterbaume

use aws_sdk_codestarnotifications::config::BehaviorVersion;
use aws_sdk_codestarnotifications::types::{DetailType, NotificationRuleStatus, Target};
use winterbaume_codestarnotifications::CodeStarNotificationsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CodeStarNotificationsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codestarnotifications::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;

    let client = aws_sdk_codestarnotifications::Client::new(&config);
    let target = Target::builder()
        .target_type("SNS")
        .target_address("arn:aws:sns:us-east-1:123456789012:notify")
        .build();
    let resp = client
        .create_notification_rule()
        .name("demo-rule")
        .resource("arn:aws:codecommit:us-east-1:123456789012:my-repo")
        .detail_type(DetailType::Basic)
        .event_type_ids("codecommit-repository-pull-request-created")
        .targets(target)
        .status(NotificationRuleStatus::Enabled)
        .send()
        .await
        .expect("create");
    if let Some(arn) = resp.arn() {
        println!("Rule: {arn}");
    }
}
