//! Example: Amazon Lex V2 Models
//!
//! Demonstrates using aws-sdk-lexmodelsv2 with winterbaume.
//!
//! Run with:
//!   cargo run --example lexmodelsv2 --package winterbaume-examples

use aws_sdk_lexmodelsv2::config::BehaviorVersion;
use aws_sdk_lexmodelsv2::types::DataPrivacy;
use winterbaume_core::MockAws;
use winterbaume_lexmodelsv2::LexModelsV2Service;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(LexModelsV2Service::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_lexmodelsv2::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_lexmodelsv2::Client::new(&config);

    // Create a bot
    let create_resp = client
        .create_bot()
        .bot_name("ExampleBot")
        .role_arn("arn:aws:iam::123456789012:role/LexBotRole")
        .data_privacy(DataPrivacy::builder().child_directed(false).build())
        .idle_session_ttl_in_seconds(300)
        .send()
        .await
        .expect("create_bot should succeed");
    println!("Created bot: {:?}", create_resp.bot_id());

    // List bots
    let list_resp = client
        .list_bots()
        .send()
        .await
        .expect("list_bots should succeed");
    println!("Bots: {:?}", list_resp.bot_summaries());
}
