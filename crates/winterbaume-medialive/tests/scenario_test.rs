//! End-to-end scenario tests for MediaLive.
//!
//! Each scenario chains 3+ operations and asserts business outcomes.

use aws_sdk_medialive::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_medialive::MediaLiveService;

async fn make_client() -> aws_sdk_medialive::Client {
    let mock = MockAws::builder()
        .with_service(MediaLiveService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_medialive::config::Region::new("eu-west-1"))
        .load()
        .await;

    aws_sdk_medialive::Client::new(&config)
}

fn basic_encoder_settings() -> aws_sdk_medialive::types::EncoderSettings {
    aws_sdk_medialive::types::EncoderSettings::builder()
        .timecode_config(
            aws_sdk_medialive::types::TimecodeConfig::builder()
                .source(aws_sdk_medialive::types::TimecodeConfigSource::Embedded)
                .build(),
        )
        .build()
}

fn basic_destinations() -> Vec<aws_sdk_medialive::types::OutputDestination> {
    vec![
        aws_sdk_medialive::types::OutputDestination::builder()
            .id("dest.1")
            .build(),
    ]
}

/// Scenario: channel broadcast pipeline
///
/// A user provisions a channel, starts a broadcast, verifies running state,
/// stops the broadcast, and confirms the channel returns to idle.
#[tokio::test]
async fn test_channel_broadcast_pipeline() {
    // Scenario: channel broadcast pipeline
    // 1. Create channel → CREATING
    // 2. Describe channel → IDLE (AWS transitions CREATING → IDLE)
    // 3. Start channel → STARTING
    // 4. Describe channel → RUNNING
    // 5. Stop channel → STOPPING
    // 6. Describe channel → IDLE
    // 7. Delete channel → DELETING

    let client = make_client().await;

    // 1. Create
    let create_resp = client
        .create_channel()
        .name("broadcast-channel")
        .role_arn("arn:aws:iam::123456789012:role/MediaLiveRole")
        .set_destinations(Some(basic_destinations()))
        .encoder_settings(basic_encoder_settings())
        .send()
        .await
        .expect("create_channel should succeed");

    let channel_id = create_resp.channel().unwrap().id().unwrap().to_string();
    assert_eq!(
        create_resp.channel().unwrap().state(),
        Some(&aws_sdk_medialive::types::ChannelState::Creating)
    );

    // 2. Describe → IDLE
    let describe = client
        .describe_channel()
        .channel_id(&channel_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        describe.state(),
        Some(&aws_sdk_medialive::types::ChannelState::Idle)
    );

    // 3. Start → STARTING
    let start = client
        .start_channel()
        .channel_id(&channel_id)
        .send()
        .await
        .expect("start_channel should succeed");
    assert_eq!(
        start.state(),
        Some(&aws_sdk_medialive::types::ChannelState::Starting)
    );

    // 4. Describe → RUNNING
    let describe2 = client
        .describe_channel()
        .channel_id(&channel_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        describe2.state(),
        Some(&aws_sdk_medialive::types::ChannelState::Running)
    );

    // 5. Stop → STOPPING
    let stop = client
        .stop_channel()
        .channel_id(&channel_id)
        .send()
        .await
        .expect("stop_channel should succeed");
    assert_eq!(
        stop.state(),
        Some(&aws_sdk_medialive::types::ChannelState::Stopping)
    );

    // 6. Describe → IDLE
    let describe3 = client
        .describe_channel()
        .channel_id(&channel_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        describe3.state(),
        Some(&aws_sdk_medialive::types::ChannelState::Idle)
    );

    // 7. Delete → DELETING; subsequent describe fails
    client
        .delete_channel()
        .channel_id(&channel_id)
        .send()
        .await
        .expect("delete_channel should succeed");

    let err = client
        .describe_channel()
        .channel_id(&channel_id)
        .send()
        .await
        .unwrap_err();
    assert!(
        format!("{:?}", err).contains("NotFoundException"),
        "Deleted channel should not be found"
    );
}

/// Scenario: input provisioning and channel association
///
/// A user creates a reusable input, provisions a channel that references it,
/// verifies both resources are discoverable via list APIs, then tears everything
/// down in order.
#[tokio::test]
async fn test_input_provisioning_and_channel_association() {
    // Scenario: input provisioning and channel association
    // 1. Create input
    // 2. Describe input → DETACHED
    // 3. Create channel with input attachment referencing the input ID
    // 4. List channels → channel visible
    // 5. List inputs → input visible
    // 6. Update input name
    // 7. Delete input → marked DELETED
    // 8. Delete channel → marked DELETING

    let client = make_client().await;

    // 1. Create input
    let input_resp = client
        .create_input()
        .name("scenario-input")
        .r#type(aws_sdk_medialive::types::InputType::RtpPush)
        .role_arn("arn:aws:iam::123456789012:role/MediaLiveRole")
        .send()
        .await
        .expect("create_input should succeed");

    let input_id = input_resp.input().unwrap().id().unwrap().to_string();

    // 2. Describe input → DETACHED
    let desc_input = client
        .describe_input()
        .input_id(&input_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc_input.state(),
        Some(&aws_sdk_medialive::types::InputState::Detached)
    );

    // 3. Create channel referencing that input
    let input_attachment = aws_sdk_medialive::types::InputAttachment::builder()
        .input_id(&input_id)
        .build();

    let channel_resp = client
        .create_channel()
        .name("scenario-channel")
        .role_arn("arn:aws:iam::123456789012:role/MediaLiveRole")
        .set_input_attachments(Some(vec![input_attachment]))
        .set_destinations(Some(basic_destinations()))
        .encoder_settings(basic_encoder_settings())
        .send()
        .await
        .expect("create_channel should succeed");

    let channel_id = channel_resp.channel().unwrap().id().unwrap().to_string();

    // 4. List channels → at least our channel is visible
    let list_channels = client
        .list_channels()
        .send()
        .await
        .expect("list_channels should succeed");
    assert!(
        list_channels
            .channels()
            .iter()
            .any(|c| c.id() == Some(channel_id.as_str())),
        "Created channel should appear in list"
    );

    // 5. List inputs → our input is visible
    let list_inputs = client
        .list_inputs()
        .send()
        .await
        .expect("list_inputs should succeed");
    assert!(
        list_inputs
            .inputs()
            .iter()
            .any(|i| i.id() == Some(input_id.as_str())),
        "Created input should appear in list"
    );

    // 6. Update input name
    let update_input = client
        .update_input()
        .input_id(&input_id)
        .name("scenario-input-renamed")
        .send()
        .await
        .expect("update_input should succeed");
    assert_eq!(
        update_input.input().unwrap().name(),
        Some("scenario-input-renamed")
    );

    // 7. Delete input
    client
        .delete_input()
        .input_id(&input_id)
        .send()
        .await
        .expect("delete_input should succeed");

    let desc_deleted = client
        .describe_input()
        .input_id(&input_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc_deleted.state(),
        Some(&aws_sdk_medialive::types::InputState::Deleted)
    );

    // 8. Delete channel
    let del_ch = client
        .delete_channel()
        .channel_id(&channel_id)
        .send()
        .await
        .expect("delete_channel should succeed");
    assert_eq!(
        del_ch.state(),
        Some(&aws_sdk_medialive::types::ChannelState::Deleting)
    );
}
