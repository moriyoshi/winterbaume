use std::sync::{Arc, Mutex};

use aws_sdk_pinpointsmsvoice::config::BehaviorVersion;
use aws_sdk_pinpointsmsvoice::types::{
    EventDestinationDefinition, PlainTextMessageType, SnsDestination, VoiceMessageContent,
};
use winterbaume_core::{MockAws, StatefulService};
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

#[tokio::test]
async fn test_configuration_set_lifecycle() {
    let client = make_client().await;
    client
        .create_configuration_set()
        .configuration_set_name("voice-cs-1")
        .send()
        .await
        .expect("create");

    let list = client.list_configuration_sets().send().await.expect("list");
    assert_eq!(list.configuration_sets().len(), 1);
    assert_eq!(list.configuration_sets()[0], "voice-cs-1");

    client
        .delete_configuration_set()
        .configuration_set_name("voice-cs-1")
        .send()
        .await
        .expect("delete");
}

#[tokio::test]
async fn test_duplicate_configuration_set_returns_conflict() {
    let client = make_client().await;
    client
        .create_configuration_set()
        .configuration_set_name("dup")
        .send()
        .await
        .expect("create");
    let err = client
        .create_configuration_set()
        .configuration_set_name("dup")
        .send()
        .await
        .expect_err("duplicate");
    assert!(format!("{err:?}").contains("AlreadyExists"));
}

#[tokio::test]
async fn test_event_destination_lifecycle() {
    let client = make_client().await;
    client
        .create_configuration_set()
        .configuration_set_name("voice-cs-2")
        .send()
        .await
        .expect("create cs");

    let dest = EventDestinationDefinition::builder()
        .enabled(true)
        .matching_event_types("INITIATED_CALL".into())
        .sns_destination(
            SnsDestination::builder()
                .topic_arn("arn:aws:sns:us-east-1:123:topic/calls")
                .build(),
        )
        .build();
    client
        .create_configuration_set_event_destination()
        .configuration_set_name("voice-cs-2")
        .event_destination_name("dest-1")
        .event_destination(dest)
        .send()
        .await
        .expect("create dest");

    let resp = client
        .get_configuration_set_event_destinations()
        .configuration_set_name("voice-cs-2")
        .send()
        .await
        .expect("get");
    assert_eq!(resp.event_destinations().len(), 1);
    assert_eq!(resp.event_destinations()[0].name(), Some("dest-1"));

    client
        .delete_configuration_set_event_destination()
        .configuration_set_name("voice-cs-2")
        .event_destination_name("dest-1")
        .send()
        .await
        .expect("delete");
}

#[tokio::test]
async fn test_send_voice_message() {
    let client = make_client().await;
    let content = VoiceMessageContent::builder()
        .plain_text_message(
            PlainTextMessageType::builder()
                .text("Hello world")
                .language_code("en-US")
                .build(),
        )
        .build();
    let resp = client
        .send_voice_message()
        .destination_phone_number("+15551234567")
        .origination_phone_number("+15559876543")
        .content(content)
        .send()
        .await
        .expect("send");
    let id = resp.message_id().expect("id");
    assert!(id.starts_with("msg-"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = PinpointSmsVoiceService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });
    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .expect("restore");
    assert_eq!(events.lock().unwrap().len(), 1);
}
