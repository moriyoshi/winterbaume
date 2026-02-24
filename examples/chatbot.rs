//! Example: AWS Chatbot
//!
//! Demonstrates using aws-sdk-chatbot with winterbaume.
//!
//! Run with:
//!   cargo run --example chatbot --package winterbaume-examples

use aws_sdk_chatbot::config::BehaviorVersion;
use winterbaume_chatbot::ChatbotService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ChatbotService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_chatbot::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_chatbot::Client::new(&config);

    let resp = client
        .describe_slack_channel_configurations()
        .send()
        .await
        .expect("describe_slack_channel_configurations should succeed");
    println!(
        "Chatbot Slack configurations: {:?}",
        resp.slack_channel_configurations()
    );
}
