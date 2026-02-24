//! Example: Connect Participant
//!
//! Demonstrates using aws-sdk-connectparticipant with winterbaume.
//!
//! Run with:
//!   cargo run --example connectparticipant --package winterbaume

use aws_sdk_connectparticipant::config::BehaviorVersion;
use winterbaume_connectparticipant::ConnectParticipantService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(ConnectParticipantService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_connectparticipant::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_connectparticipant::Client::new(&config);
    let resp = client
        .create_participant_connection()
        .participant_token("demo-token")
        .r#type("WEBSOCKET".into())
        .r#type("CONNECTION_CREDENTIALS".into())
        .send()
        .await
        .expect("create_participant_connection should succeed");
    if let Some(creds) = resp.connection_credentials() {
        println!("Connection token: {:?}", creds.connection_token());
    }
}
