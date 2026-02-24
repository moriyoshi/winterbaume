//! End-to-end scenario tests for the Pinpoint SMS Voice service.
//!
//! These tests chain multiple operations together to exercise realistic
//! use-case workflows, asserting business outcomes rather than per-API
//! response shapes.

use aws_sdk_pinpointsmsvoice::config::BehaviorVersion;
use aws_sdk_pinpointsmsvoice::types::{
    EventDestinationDefinition, PlainTextMessageType, SnsDestination, VoiceMessageContent,
};
use winterbaume_core::MockAws;
use winterbaume_pinpointsmsvoice::PinpointSmsVoiceService;

async fn make_client() -> aws_sdk_pinpointsmsvoice::Client {
    let mock = MockAws::builder()
        .with_service(PinpointSmsVoiceService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_pinpointsmsvoice::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_pinpointsmsvoice::Client::new(&config)
}

/// Scenario: Configure a configuration set with an event destination,
/// send a voice message through it, then tear down cleanly.
///
/// Chains: CreateConfigurationSet → CreateConfigurationSetEventDestination
///   → GetConfigurationSetEventDestinations → SendVoiceMessage
///   → DeleteConfigurationSetEventDestination → DeleteConfigurationSet
#[tokio::test]
async fn test_voice_campaign_pipeline() {
    let client = make_client().await;

    // 1. Create a configuration set for the campaign.
    client
        .create_configuration_set()
        .configuration_set_name("campaign-cs")
        .send()
        .await
        .expect("create configuration set");

    // 2. Attach an SNS event destination to capture call events.
    let dest = EventDestinationDefinition::builder()
        .enabled(true)
        .matching_event_types("INITIATED_CALL".into())
        .matching_event_types("COMPLETED_CALL".into())
        .sns_destination(
            SnsDestination::builder()
                .topic_arn("arn:aws:sns:us-east-1:123456789012:voice-events")
                .build(),
        )
        .build();
    client
        .create_configuration_set_event_destination()
        .configuration_set_name("campaign-cs")
        .event_destination_name("sns-events")
        .event_destination(dest)
        .send()
        .await
        .expect("create event destination");

    // 3. Verify the destination is retrievable.
    let get_resp = client
        .get_configuration_set_event_destinations()
        .configuration_set_name("campaign-cs")
        .send()
        .await
        .expect("get event destinations");
    assert_eq!(get_resp.event_destinations().len(), 1);
    assert_eq!(get_resp.event_destinations()[0].name(), Some("sns-events"));

    // 4. Send a voice message through the configured set.
    let content = VoiceMessageContent::builder()
        .plain_text_message(
            PlainTextMessageType::builder()
                .text("Your appointment is confirmed.")
                .language_code("en-US")
                .build(),
        )
        .build();
    let send_resp = client
        .send_voice_message()
        .destination_phone_number("+15551234567")
        .origination_phone_number("+15559876543")
        .configuration_set_name("campaign-cs")
        .content(content)
        .send()
        .await
        .expect("send voice message");
    let msg_id = send_resp.message_id().expect("message_id must be set");
    assert!(msg_id.starts_with("msg-"), "message id format: {msg_id}");

    // 5. Tear down: remove destination then configuration set.
    client
        .delete_configuration_set_event_destination()
        .configuration_set_name("campaign-cs")
        .event_destination_name("sns-events")
        .send()
        .await
        .expect("delete event destination");

    client
        .delete_configuration_set()
        .configuration_set_name("campaign-cs")
        .send()
        .await
        .expect("delete configuration set");

    // 6. Confirm the configuration set is gone.
    let list = client.list_configuration_sets().send().await.expect("list");
    assert!(
        !list
            .configuration_sets()
            .contains(&"campaign-cs".to_string()),
        "campaign-cs should be deleted"
    );
}

/// Scenario: Multiple configuration sets coexist and are listed correctly.
///
/// Chains: CreateConfigurationSet × 3 → ListConfigurationSets
///   → DeleteConfigurationSet × 3
#[tokio::test]
async fn test_multi_configuration_set_management() {
    let client = make_client().await;

    for name in &["cs-alpha", "cs-beta", "cs-gamma"] {
        client
            .create_configuration_set()
            .configuration_set_name(*name)
            .send()
            .await
            .expect("create");
    }

    let list = client.list_configuration_sets().send().await.expect("list");
    let names = list.configuration_sets();
    assert!(names.contains(&"cs-alpha".to_string()));
    assert!(names.contains(&"cs-beta".to_string()));
    assert!(names.contains(&"cs-gamma".to_string()));

    for name in &["cs-alpha", "cs-beta", "cs-gamma"] {
        client
            .delete_configuration_set()
            .configuration_set_name(*name)
            .send()
            .await
            .expect("delete");
    }

    let list2 = client
        .list_configuration_sets()
        .send()
        .await
        .expect("list after delete");
    assert!(list2.configuration_sets().is_empty());
}
