use std::sync::{Arc, Mutex};

use aws_sdk_chimesdkmeetings::config::BehaviorVersion;
use aws_sdk_chimesdkmeetings::types::{AttendeeCapabilities, MediaCapabilities};
use winterbaume_chimesdkmeetings::ChimeSdkMeetingsService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_chimesdkmeetings::Client {
    let mock = MockAws::builder()
        .with_service(ChimeSdkMeetingsService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_chimesdkmeetings::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_chimesdkmeetings::Client::new(&config)
}

#[tokio::test]
async fn test_create_get_delete_meeting() {
    let client = make_client().await;
    let resp = client
        .create_meeting()
        .client_request_token("ct")
        .external_meeting_id("ext-1")
        .media_region("us-east-1")
        .send()
        .await
        .expect("create_meeting");
    let meeting_id = resp
        .meeting()
        .and_then(|m| m.meeting_id())
        .expect("id")
        .to_string();
    let got = client
        .get_meeting()
        .meeting_id(&meeting_id)
        .send()
        .await
        .expect("get_meeting");
    let m = got.meeting().expect("meeting");
    assert_eq!(m.meeting_id(), Some(meeting_id.as_str()));
    assert_eq!(m.media_region(), Some("us-east-1"));
    let mp = m.media_placement().expect("media");
    assert!(mp.audio_host_url().is_some());

    client
        .delete_meeting()
        .meeting_id(&meeting_id)
        .send()
        .await
        .expect("delete_meeting");
    let err = client
        .get_meeting()
        .meeting_id(&meeting_id)
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("NotFoundException"));
}

#[tokio::test]
async fn test_attendee_lifecycle() {
    let client = make_client().await;
    let resp = client
        .create_meeting()
        .client_request_token("ct")
        .external_meeting_id("ext-2")
        .media_region("us-east-1")
        .send()
        .await
        .expect("create_meeting");
    let meeting_id = resp
        .meeting()
        .and_then(|m| m.meeting_id())
        .expect("id")
        .to_string();
    let resp = client
        .create_attendee()
        .meeting_id(&meeting_id)
        .external_user_id("alice")
        .send()
        .await
        .expect("create_attendee");
    let attendee_id = resp
        .attendee()
        .and_then(|a| a.attendee_id())
        .expect("attendee_id")
        .to_string();

    let listed = client
        .list_attendees()
        .meeting_id(&meeting_id)
        .send()
        .await
        .expect("list");
    assert_eq!(listed.attendees().len(), 1);

    let updated = client
        .update_attendee_capabilities()
        .meeting_id(&meeting_id)
        .attendee_id(&attendee_id)
        .capabilities(
            AttendeeCapabilities::builder()
                .audio(MediaCapabilities::Receive)
                .content(MediaCapabilities::Receive)
                .video(MediaCapabilities::Receive)
                .build()
                .expect("caps"),
        )
        .send()
        .await
        .expect("update_caps");
    assert_eq!(
        updated
            .attendee()
            .and_then(|a| a.capabilities())
            .map(|c| c.audio().as_str()),
        Some("Receive")
    );

    client
        .delete_attendee()
        .meeting_id(&meeting_id)
        .attendee_id(&attendee_id)
        .send()
        .await
        .expect("delete_attendee");
    let listed = client
        .list_attendees()
        .meeting_id(&meeting_id)
        .send()
        .await
        .expect("list");
    assert!(listed.attendees().is_empty());
}

#[tokio::test]
async fn test_create_meeting_with_attendees() {
    let client = make_client().await;
    let item = aws_sdk_chimesdkmeetings::types::CreateAttendeeRequestItem::builder()
        .external_user_id("alice")
        .build()
        .expect("item");
    let resp = client
        .create_meeting_with_attendees()
        .client_request_token("ct")
        .external_meeting_id("ext-3")
        .media_region("us-east-1")
        .attendees(item)
        .send()
        .await
        .expect("create");
    assert!(resp.meeting().is_some());
    assert_eq!(resp.attendees().len(), 1);
}

#[tokio::test]
async fn test_get_unknown_meeting() {
    let client = make_client().await;
    let err = client
        .get_meeting()
        .meeting_id("ghost")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("NotFoundException"));
}

#[tokio::test]
async fn test_transcription_toggle() {
    let client = make_client().await;
    let resp = client
        .create_meeting()
        .client_request_token("ct")
        .external_meeting_id("ext-4")
        .media_region("us-east-1")
        .send()
        .await
        .expect("create_meeting");
    let meeting_id = resp.meeting().and_then(|m| m.meeting_id()).expect("id");
    let cfg = aws_sdk_chimesdkmeetings::types::TranscriptionConfiguration::builder()
        .engine_transcribe_settings(
            aws_sdk_chimesdkmeetings::types::EngineTranscribeSettings::builder()
                .language_code(aws_sdk_chimesdkmeetings::types::TranscribeLanguageCode::EnUs)
                .build(),
        )
        .build();
    client
        .start_meeting_transcription()
        .meeting_id(meeting_id)
        .transcription_configuration(cfg)
        .send()
        .await
        .expect("start");
    client
        .stop_meeting_transcription()
        .meeting_id(meeting_id)
        .send()
        .await
        .expect("stop");
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = ChimeSdkMeetingsService::new();
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
