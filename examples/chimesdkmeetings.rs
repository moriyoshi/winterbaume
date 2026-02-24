//! Example: Chime SDK Meetings
//!
//! Demonstrates using aws-sdk-chimesdkmeetings with winterbaume.
//!
//! Run with:
//!   cargo run --example chimesdkmeetings --package winterbaume

use aws_sdk_chimesdkmeetings::config::BehaviorVersion;
use winterbaume_chimesdkmeetings::ChimeSdkMeetingsService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ChimeSdkMeetingsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_chimesdkmeetings::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_chimesdkmeetings::Client::new(&config);
    let resp = client
        .create_meeting()
        .client_request_token("ct")
        .external_meeting_id("demo")
        .media_region("us-east-1")
        .send()
        .await
        .expect("create_meeting");
    if let Some(m) = resp.meeting() {
        println!("Meeting: {:?}", m.meeting_id());
    }
}
