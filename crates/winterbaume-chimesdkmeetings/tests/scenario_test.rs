/// Scenario: Full meeting collaboration session
///
/// Models a complete meeting lifecycle:
/// 1. Create a meeting.
/// 2. Add multiple attendees with default capabilities.
/// 3. Restrict one attendee's capabilities (listen-only).
/// 4. Tag the meeting resource.
/// 5. Verify attendee list reflects capability update.
/// 6. Delete an attendee and confirm list shrinks.
/// 7. Start and stop transcription.
/// 8. Delete the meeting and confirm it is gone.
use aws_sdk_chimesdkmeetings::config::BehaviorVersion;
use aws_sdk_chimesdkmeetings::types::{AttendeeCapabilities, MediaCapabilities};
use winterbaume_chimesdkmeetings::ChimeSdkMeetingsService;
use winterbaume_core::MockAws;

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
async fn test_meeting_collaboration_session() {
    let client = make_client().await;

    // Step 1: Create meeting.
    let create_resp = client
        .create_meeting()
        .client_request_token("collab-ct")
        .external_meeting_id("collab-ext-1")
        .media_region("us-east-1")
        .send()
        .await
        .expect("create_meeting");
    let meeting = create_resp.meeting().expect("meeting");
    let meeting_id = meeting.meeting_id().expect("meeting_id").to_string();
    let meeting_arn = meeting.meeting_arn().expect("meeting_arn").to_string();

    // Step 2: Add two attendees.
    let alice_resp = client
        .create_attendee()
        .meeting_id(&meeting_id)
        .external_user_id("alice")
        .send()
        .await
        .expect("create_attendee alice");
    let alice_id = alice_resp
        .attendee()
        .and_then(|a| a.attendee_id())
        .expect("alice_id")
        .to_string();

    let bob_resp = client
        .create_attendee()
        .meeting_id(&meeting_id)
        .external_user_id("bob")
        .send()
        .await
        .expect("create_attendee bob");
    let bob_id = bob_resp
        .attendee()
        .and_then(|a| a.attendee_id())
        .expect("bob_id")
        .to_string();

    let listed = client
        .list_attendees()
        .meeting_id(&meeting_id)
        .send()
        .await
        .expect("list_attendees");
    assert_eq!(listed.attendees().len(), 2, "two attendees present");

    // Step 3: Restrict Bob to receive-only capabilities.
    let updated = client
        .update_attendee_capabilities()
        .meeting_id(&meeting_id)
        .attendee_id(&bob_id)
        .capabilities(
            AttendeeCapabilities::builder()
                .audio(MediaCapabilities::Receive)
                .content(MediaCapabilities::None)
                .video(MediaCapabilities::Receive)
                .build()
                .expect("caps"),
        )
        .send()
        .await
        .expect("update_attendee_capabilities");
    let bob_caps = updated
        .attendee()
        .and_then(|a| a.capabilities())
        .expect("bob_caps");
    assert_eq!(bob_caps.audio().as_str(), "Receive");
    assert_eq!(bob_caps.content().as_str(), "None");

    // Step 4: Tag the meeting resource.
    client
        .tag_resource()
        .resource_arn(&meeting_arn)
        .tags(
            aws_sdk_chimesdkmeetings::types::Tag::builder()
                .key("Environment")
                .value("test")
                .build()
                .expect("tag"),
        )
        .send()
        .await
        .expect("tag_resource");
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&meeting_arn)
        .send()
        .await
        .expect("list_tags");
    assert_eq!(tags_resp.tags().len(), 1);
    assert_eq!(tags_resp.tags()[0].key(), "Environment");

    // Step 5: Verify Alice still has default send-receive capabilities.
    let alice_get = client
        .get_attendee()
        .meeting_id(&meeting_id)
        .attendee_id(&alice_id)
        .send()
        .await
        .expect("get_attendee alice");
    let alice_caps = alice_get
        .attendee()
        .and_then(|a| a.capabilities())
        .expect("alice_caps");
    assert_eq!(alice_caps.audio().as_str(), "SendReceive");

    // Step 6: Delete Bob; list should shrink to one attendee.
    client
        .delete_attendee()
        .meeting_id(&meeting_id)
        .attendee_id(&bob_id)
        .send()
        .await
        .expect("delete_attendee bob");
    let listed = client
        .list_attendees()
        .meeting_id(&meeting_id)
        .send()
        .await
        .expect("list after delete");
    assert_eq!(listed.attendees().len(), 1, "only alice remains");

    // Step 7: Start and stop transcription.
    let cfg = aws_sdk_chimesdkmeetings::types::TranscriptionConfiguration::builder()
        .engine_transcribe_settings(
            aws_sdk_chimesdkmeetings::types::EngineTranscribeSettings::builder()
                .language_code(aws_sdk_chimesdkmeetings::types::TranscribeLanguageCode::EnUs)
                .build(),
        )
        .build();
    client
        .start_meeting_transcription()
        .meeting_id(&meeting_id)
        .transcription_configuration(cfg)
        .send()
        .await
        .expect("start transcription");
    client
        .stop_meeting_transcription()
        .meeting_id(&meeting_id)
        .send()
        .await
        .expect("stop transcription");

    // Step 8: Delete the meeting; subsequent get must return NotFoundException.
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
        .expect_err("should be gone");
    assert!(
        format!("{err:?}").contains("NotFoundException"),
        "expected NotFoundException, got {err:?}"
    );
}

/// Scenario: Batch attendee operations
///
/// Tests the batch-create and batch-update-capabilities-except paths.
#[tokio::test]
async fn test_batch_attendee_operations() {
    let client = make_client().await;

    // Create meeting.
    let meeting_id = client
        .create_meeting()
        .client_request_token("batch-ct")
        .external_meeting_id("batch-ext")
        .media_region("us-east-1")
        .send()
        .await
        .expect("create_meeting")
        .meeting()
        .and_then(|m| m.meeting_id())
        .expect("meeting_id")
        .to_string();

    // Batch-create three attendees.
    let items: Vec<_> = ["carol", "dave", "eve"]
        .iter()
        .map(|name| {
            aws_sdk_chimesdkmeetings::types::CreateAttendeeRequestItem::builder()
                .external_user_id(*name)
                .build()
                .expect("item")
        })
        .collect();
    let batch_resp = client
        .batch_create_attendee()
        .meeting_id(&meeting_id)
        .set_attendees(Some(items))
        .send()
        .await
        .expect("batch_create_attendee");
    assert_eq!(batch_resp.attendees().len(), 3, "three attendees created");
    assert!(batch_resp.errors().is_empty(), "no batch errors");

    let carol_id = batch_resp.attendees()[0]
        .attendee_id()
        .expect("carol_id")
        .to_string();

    // Update capabilities for all except carol.
    let excluded = aws_sdk_chimesdkmeetings::types::AttendeeIdItem::builder()
        .attendee_id(&carol_id)
        .build()
        .expect("excluded");
    client
        .batch_update_attendee_capabilities_except()
        .meeting_id(&meeting_id)
        .excluded_attendee_ids(excluded)
        .capabilities(
            AttendeeCapabilities::builder()
                .audio(MediaCapabilities::None)
                .content(MediaCapabilities::None)
                .video(MediaCapabilities::None)
                .build()
                .expect("caps"),
        )
        .send()
        .await
        .expect("batch_update_except");

    // Carol should still have SendReceive; the others should have None.
    let carol = client
        .get_attendee()
        .meeting_id(&meeting_id)
        .attendee_id(&carol_id)
        .send()
        .await
        .expect("get carol")
        .attendee()
        .expect("carol attendee")
        .clone();
    assert_eq!(
        carol.capabilities().map(|c| c.audio().as_str()),
        Some("SendReceive"),
        "carol unchanged"
    );
}
