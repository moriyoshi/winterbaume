use aws_sdk_medialive::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_medialive::MediaLiveService;

async fn make_medialive_client() -> aws_sdk_medialive::Client {
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

fn default_input_attachments() -> Vec<aws_sdk_medialive::types::InputAttachment> {
    vec![aws_sdk_medialive::types::InputAttachment::builder()
        .input_id("an-attachment-id")
        .input_settings(
            aws_sdk_medialive::types::InputSettings::builder()
                .denoise_filter(aws_sdk_medialive::types::InputDenoiseFilter::Disabled)
                .input_filter(aws_sdk_medialive::types::InputFilter::Auto)
                .deblock_filter(aws_sdk_medialive::types::InputDeblockFilter::Disabled)
                .network_input_settings(
                    aws_sdk_medialive::types::NetworkInputSettings::builder()
                        .server_validation(
                            aws_sdk_medialive::types::NetworkInputServerValidation::CheckCryptographyAndValidateName,
                        )
                        .build(),
                )
                .source_end_behavior(aws_sdk_medialive::types::InputSourceEndBehavior::Continue)
                .filter_strength(1)
                .build(),
        )
        .build()]
}

fn default_destinations() -> Vec<aws_sdk_medialive::types::OutputDestination> {
    vec![
        aws_sdk_medialive::types::OutputDestination::builder()
            .id("destination.1")
            .build(),
        aws_sdk_medialive::types::OutputDestination::builder()
            .id("destination.2")
            .build(),
    ]
}

fn default_encoder_settings() -> aws_sdk_medialive::types::EncoderSettings {
    aws_sdk_medialive::types::EncoderSettings::builder()
        .timecode_config(
            aws_sdk_medialive::types::TimecodeConfig::builder()
                .source(aws_sdk_medialive::types::TimecodeConfigSource::Embedded)
                .build(),
        )
        .build()
}

// ==================== Channel tests (from test_medialive.py) ====================

#[tokio::test]
async fn test_create_channel_succeeds() {
    let client = make_medialive_client().await;

    let resp = client
        .create_channel()
        .name("test channel 1")
        .role_arn("arn:aws:iam::123456789012:role/TestMediaLiveChannelCreateRole")
        .set_input_attachments(Some(default_input_attachments()))
        .set_destinations(Some(default_destinations()))
        .encoder_settings(default_encoder_settings())
        .tags("Customer", "moto")
        .send()
        .await
        .expect("create_channel should succeed");

    let channel = resp.channel().unwrap();
    assert!(channel.arn().is_some());
    assert!(
        channel
            .arn()
            .unwrap()
            .starts_with("arn:aws:medialive:channel:")
    );
    assert_eq!(channel.name(), Some("test channel 1"));
    assert_eq!(
        channel.state(),
        Some(&aws_sdk_medialive::types::ChannelState::Creating)
    );
    assert!(channel.tags().is_some());
    assert_eq!(channel.tags().unwrap().get("Customer").unwrap(), "moto");
}

#[tokio::test]
async fn test_describe_channel_succeeds() {
    let client = make_medialive_client().await;

    let create_resp = client
        .create_channel()
        .name("test channel X")
        .role_arn("arn:aws:iam::123456789012:role/TestMediaLiveChannelCreateRole")
        .set_input_attachments(Some(default_input_attachments()))
        .set_destinations(Some(default_destinations()))
        .encoder_settings(default_encoder_settings())
        .tags("Customer", "moto")
        .send()
        .await
        .unwrap();

    let channel_id = create_resp.channel().unwrap().id().unwrap().to_string();

    let channel = client
        .describe_channel()
        .channel_id(&channel_id)
        .send()
        .await
        .expect("describe_channel should succeed");

    assert!(channel.arn().is_some());
    assert!(channel.arn().unwrap().contains(&channel_id));
    assert_eq!(channel.name(), Some("test channel X"));
    // After describe, CREATING transitions to IDLE
    assert_eq!(
        channel.state(),
        Some(&aws_sdk_medialive::types::ChannelState::Idle)
    );
    assert!(channel.tags().is_some());
    assert_eq!(channel.tags().unwrap().get("Customer").unwrap(), "moto");
}

#[tokio::test]
async fn test_delete_channel_moves_channel_in_deleted_state() {
    let client = make_medialive_client().await;

    let create_resp = client
        .create_channel()
        .name("test channel X")
        .role_arn("arn:aws:iam::123456789012:role/TestMediaLiveChannelCreateRole")
        .set_input_attachments(Some(default_input_attachments()))
        .set_destinations(Some(default_destinations()))
        .encoder_settings(default_encoder_settings())
        .send()
        .await
        .unwrap();

    let channel_id = create_resp.channel().unwrap().id().unwrap().to_string();

    let delete_resp = client
        .delete_channel()
        .channel_id(&channel_id)
        .send()
        .await
        .expect("delete_channel should succeed");

    assert_eq!(delete_resp.name(), Some("test channel X"));
    assert_eq!(
        delete_resp.state(),
        Some(&aws_sdk_medialive::types::ChannelState::Deleting)
    );
}

#[tokio::test]
async fn test_start_channel_succeeds() {
    let client = make_medialive_client().await;

    let create_resp = client
        .create_channel()
        .name("testchan1")
        .role_arn("arn:aws:iam::123456789012:role/TestMediaLiveChannelCreateRole")
        .set_input_attachments(Some(default_input_attachments()))
        .set_destinations(Some(default_destinations()))
        .encoder_settings(default_encoder_settings())
        .send()
        .await
        .unwrap();

    let channel_id = create_resp.channel().unwrap().id().unwrap().to_string();

    let start_resp = client
        .start_channel()
        .channel_id(&channel_id)
        .send()
        .await
        .expect("start_channel should succeed");

    assert_eq!(start_resp.name(), Some("testchan1"));
    assert_eq!(
        start_resp.state(),
        Some(&aws_sdk_medialive::types::ChannelState::Starting)
    );

    // After describe, should be RUNNING
    let describe_resp = client
        .describe_channel()
        .channel_id(&channel_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        describe_resp.state(),
        Some(&aws_sdk_medialive::types::ChannelState::Running)
    );
}

#[tokio::test]
async fn test_stop_channel_succeeds() {
    let client = make_medialive_client().await;

    let create_resp = client
        .create_channel()
        .name("testchan2")
        .role_arn("arn:aws:iam::123456789012:role/TestMediaLiveChannelCreateRole")
        .set_input_attachments(Some(default_input_attachments()))
        .set_destinations(Some(default_destinations()))
        .encoder_settings(default_encoder_settings())
        .send()
        .await
        .unwrap();

    let channel_id = create_resp.channel().unwrap().id().unwrap().to_string();

    client
        .start_channel()
        .channel_id(&channel_id)
        .send()
        .await
        .unwrap();

    let stop_resp = client
        .stop_channel()
        .channel_id(&channel_id)
        .send()
        .await
        .expect("stop_channel should succeed");

    assert_eq!(stop_resp.name(), Some("testchan2"));
    assert_eq!(
        stop_resp.state(),
        Some(&aws_sdk_medialive::types::ChannelState::Stopping)
    );

    // After describe, should be IDLE
    let describe_resp = client
        .describe_channel()
        .channel_id(&channel_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        describe_resp.state(),
        Some(&aws_sdk_medialive::types::ChannelState::Idle)
    );
}

#[tokio::test]
async fn test_update_channel_succeeds() {
    let client = make_medialive_client().await;

    let create_resp = client
        .create_channel()
        .name("Original Channel")
        .role_arn("arn:aws:iam::123456789012:role/TestMediaLiveChannelCreateRole")
        .set_input_attachments(Some(default_input_attachments()))
        .set_destinations(Some(default_destinations()))
        .encoder_settings(default_encoder_settings())
        .send()
        .await
        .unwrap();

    let channel_id = create_resp.channel().unwrap().id().unwrap().to_string();

    let update_resp = client
        .update_channel()
        .channel_id(&channel_id)
        .name("Updated Channel")
        .send()
        .await
        .expect("update_channel should succeed");

    let updated = update_resp.channel().unwrap();
    assert_eq!(
        updated.state(),
        Some(&aws_sdk_medialive::types::ChannelState::Updating)
    );
    assert_eq!(updated.name(), Some("Updated Channel"));

    // After describe, should be IDLE with updated name
    let describe_resp = client
        .describe_channel()
        .channel_id(&channel_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        describe_resp.state(),
        Some(&aws_sdk_medialive::types::ChannelState::Idle)
    );
    assert_eq!(describe_resp.name(), Some("Updated Channel"));
}

#[tokio::test]
async fn test_list_channels_succeeds() {
    let client = make_medialive_client().await;

    // Create 3 channels
    for idx in 0..3 {
        client
            .create_channel()
            .name(format!("test {idx}"))
            .role_arn("arn:aws:iam::123456789012:role/TestMediaLiveChannelCreateRole")
            .set_input_attachments(Some(default_input_attachments()))
            .set_destinations(Some(default_destinations()))
            .encoder_settings(default_encoder_settings())
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_channels()
        .send()
        .await
        .expect("list_channels should succeed");

    assert_eq!(resp.channels().len(), 3);
    assert_eq!(resp.channels()[0].name(), Some("test 0"));
    assert_eq!(
        resp.channels()[0].channel_class(),
        Some(&aws_sdk_medialive::types::ChannelClass::Standard)
    );
    assert_eq!(resp.channels()[0].pipelines_running_count(), Some(2));
}

// ==================== Input tests (from test_medialive.py) ====================

#[tokio::test]
async fn test_create_input_succeeds() {
    let client = make_medialive_client().await;

    let resp = client
        .create_input()
        .name("Input One")
        .r#type(aws_sdk_medialive::types::InputType::RtpPush)
        .role_arn("arn:aws:iam::123456789012:role/TestMediaLiveInputCreateRole")
        .input_devices(
            aws_sdk_medialive::types::InputDeviceSettings::builder()
                .id("1234-56")
                .build(),
        )
        .media_connect_flows(
            aws_sdk_medialive::types::MediaConnectFlowRequest::builder()
                .flow_arn("flow:1")
                .build(),
        )
        .sources(
            aws_sdk_medialive::types::InputSourceRequest::builder()
                .password_param("pwd431$%!")
                .url("scheme://url:1234/")
                .username("userX")
                .build(),
        )
        .tags("Customer", "moto")
        .send()
        .await
        .expect("create_input should succeed");

    let input = resp.input().unwrap();
    assert!(input.id().is_some());
    assert!(input.id().unwrap().len() > 1);
    assert!(input.arn().is_some());
    assert!(input.arn().unwrap().starts_with("arn:aws:medialive:input:"));
    assert_eq!(input.name(), Some("Input One"));
    assert!(input.attached_channels().is_empty());
    assert_eq!(
        input.input_class(),
        Some(&aws_sdk_medialive::types::InputClass::Standard)
    );
    assert_eq!(
        input.input_source_type(),
        Some(&aws_sdk_medialive::types::InputSourceType::Static)
    );
    assert_eq!(
        input.state(),
        Some(&aws_sdk_medialive::types::InputState::Creating)
    );
    assert!(input.tags().is_some());
    assert_eq!(input.tags().unwrap().get("Customer").unwrap(), "moto");
    assert_eq!(
        input.r#type(),
        Some(&aws_sdk_medialive::types::InputType::RtpPush)
    );
}

#[tokio::test]
async fn test_describe_input_succeeds() {
    let client = make_medialive_client().await;

    let create_resp = client
        .create_input()
        .name("Input Two")
        .r#type(aws_sdk_medialive::types::InputType::RtpPush)
        .role_arn("arn:aws:iam::123456789012:role/TestMediaLiveInputCreateRole")
        .tags("Customer", "moto")
        .send()
        .await
        .unwrap();

    let input_id = create_resp.input().unwrap().id().unwrap().to_string();
    assert_eq!(
        create_resp.input().unwrap().state(),
        Some(&aws_sdk_medialive::types::InputState::Creating)
    );

    let describe_resp = client
        .describe_input()
        .input_id(&input_id)
        .send()
        .await
        .expect("describe_input should succeed");

    assert_eq!(describe_resp.name(), Some("Input Two"));
    // After describe, CREATING transitions to DETACHED
    assert_eq!(
        describe_resp.state(),
        Some(&aws_sdk_medialive::types::InputState::Detached)
    );
}

#[tokio::test]
async fn test_delete_input_moves_input_in_deleted_state() {
    let client = make_medialive_client().await;

    let create_resp = client
        .create_input()
        .name("test input X")
        .r#type(aws_sdk_medialive::types::InputType::RtpPush)
        .role_arn("arn:aws:iam::123456789012:role/TestMediaLiveInputCreateRole")
        .send()
        .await
        .unwrap();

    let input_id = create_resp.input().unwrap().id().unwrap().to_string();

    client
        .delete_input()
        .input_id(&input_id)
        .send()
        .await
        .expect("delete_input should succeed");

    let describe_resp = client
        .describe_input()
        .input_id(&input_id)
        .send()
        .await
        .unwrap();

    assert_eq!(describe_resp.name(), Some("test input X"));
    assert_eq!(
        describe_resp.state(),
        Some(&aws_sdk_medialive::types::InputState::Deleted)
    );
}

#[tokio::test]
async fn test_update_input_succeeds() {
    let client = make_medialive_client().await;

    let create_resp = client
        .create_input()
        .name("test input X")
        .r#type(aws_sdk_medialive::types::InputType::RtpPush)
        .role_arn("arn:aws:iam::123456789012:role/TestMediaLiveInputCreateRole")
        .send()
        .await
        .unwrap();

    let input_id = create_resp.input().unwrap().id().unwrap().to_string();

    let update_resp = client
        .update_input()
        .input_id(&input_id)
        .name("test input U")
        .send()
        .await
        .expect("update_input should succeed");

    let input = update_resp.input().unwrap();
    assert_eq!(input.name(), Some("test input U"));
}

#[tokio::test]
async fn test_list_inputs_succeeds() {
    let client = make_medialive_client().await;

    for idx in 0..3 {
        client
            .create_input()
            .name(format!("Input {idx}"))
            .r#type(aws_sdk_medialive::types::InputType::RtpPush)
            .role_arn("arn:aws:iam::123456789012:role/TestMediaLiveInputCreateRole")
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_inputs()
        .send()
        .await
        .expect("list_inputs should succeed");

    assert_eq!(resp.inputs().len(), 3);
}

// ============================================================================
// Tests derived from AWS documentation: MediaLive
// ============================================================================

// ---- Channel not-found error tests ----

#[tokio::test]
async fn test_describe_channel_not_found() {
    let client = make_medialive_client().await;

    let err = client
        .describe_channel()
        .channel_id("nonexistent-channel-id")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_channel_not_found() {
    let client = make_medialive_client().await;

    let err = client
        .delete_channel()
        .channel_id("nonexistent-channel-id")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_start_channel_not_found() {
    let client = make_medialive_client().await;

    let err = client
        .start_channel()
        .channel_id("nonexistent-channel-id")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_stop_channel_not_found() {
    let client = make_medialive_client().await;

    let err = client
        .stop_channel()
        .channel_id("nonexistent-channel-id")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_channel_not_found() {
    let client = make_medialive_client().await;

    let err = client
        .update_channel()
        .channel_id("nonexistent-channel-id")
        .name("New Name")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "Expected NotFoundException, got: {err_str}"
    );
}

// ---- Channel class test ----

#[tokio::test]
async fn test_create_channel_single_pipeline() {
    let client = make_medialive_client().await;

    let resp = client
        .create_channel()
        .name("single-pipeline-channel")
        .channel_class(aws_sdk_medialive::types::ChannelClass::SinglePipeline)
        .role_arn("arn:aws:iam::123456789012:role/TestMediaLiveChannelCreateRole")
        .set_input_attachments(Some(default_input_attachments()))
        .set_destinations(Some(default_destinations()))
        .encoder_settings(default_encoder_settings())
        .send()
        .await
        .expect("create_channel with SINGLE_PIPELINE should succeed");

    let channel = resp.channel().unwrap();
    assert_eq!(
        channel.channel_class(),
        Some(&aws_sdk_medialive::types::ChannelClass::SinglePipeline)
    );
    assert_eq!(channel.pipelines_running_count(), Some(1));
}

// ---- Channel pagination test ----

#[tokio::test]
async fn test_list_channels_pagination() {
    let client = make_medialive_client().await;

    // Create 5 channels
    for idx in 0..5 {
        client
            .create_channel()
            .name(format!("pagchan {idx}"))
            .role_arn("arn:aws:iam::123456789012:role/TestMediaLiveChannelCreateRole")
            .set_input_attachments(Some(default_input_attachments()))
            .set_destinations(Some(default_destinations()))
            .encoder_settings(default_encoder_settings())
            .send()
            .await
            .unwrap();
    }

    // First page: max_results=3
    let page1 = client
        .list_channels()
        .max_results(3)
        .send()
        .await
        .expect("list_channels page1 should succeed");

    assert_eq!(page1.channels().len(), 3);
    let next_token = page1.next_token().expect("Expected a next_token for page1");

    // Second page: remaining 2
    let page2 = client
        .list_channels()
        .max_results(3)
        .next_token(next_token)
        .send()
        .await
        .expect("list_channels page2 should succeed");

    assert_eq!(page2.channels().len(), 2);
    assert!(
        page2.next_token().is_none(),
        "Expected no next_token on last page"
    );
}

// ---- Channel full lifecycle test ----

#[tokio::test]
async fn test_channel_full_lifecycle() {
    let client = make_medialive_client().await;

    // Create
    let create_resp = client
        .create_channel()
        .name("lifecycle-channel")
        .role_arn("arn:aws:iam::123456789012:role/TestMediaLiveChannelCreateRole")
        .set_input_attachments(Some(default_input_attachments()))
        .set_destinations(Some(default_destinations()))
        .encoder_settings(default_encoder_settings())
        .send()
        .await
        .unwrap();

    let channel_id = create_resp.channel().unwrap().id().unwrap().to_string();

    // Describe — state becomes IDLE
    let describe_resp = client
        .describe_channel()
        .channel_id(&channel_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        describe_resp.state(),
        Some(&aws_sdk_medialive::types::ChannelState::Idle)
    );
    assert_eq!(describe_resp.name(), Some("lifecycle-channel"));

    // Update name
    let update_resp = client
        .update_channel()
        .channel_id(&channel_id)
        .name("lifecycle-channel-updated")
        .send()
        .await
        .unwrap();
    assert_eq!(
        update_resp.channel().unwrap().name(),
        Some("lifecycle-channel-updated")
    );

    // Delete
    let delete_resp = client
        .delete_channel()
        .channel_id(&channel_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        delete_resp.state(),
        Some(&aws_sdk_medialive::types::ChannelState::Deleting)
    );

    // Describe after delete — should return NotFoundException (DELETING channels are invisible)
    let err = client
        .describe_channel()
        .channel_id(&channel_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "Expected NotFoundException after delete, got: {err_str}"
    );
}

// ---- Input not-found error tests ----

#[tokio::test]
async fn test_describe_input_not_found() {
    let client = make_medialive_client().await;

    let err = client
        .describe_input()
        .input_id("nonexistent-input-id")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_input_not_found() {
    let client = make_medialive_client().await;

    let err = client
        .delete_input()
        .input_id("nonexistent-input-id")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "Expected NotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_input_not_found() {
    let client = make_medialive_client().await;

    let err = client
        .update_input()
        .input_id("nonexistent-input-id")
        .name("new-name")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("NotFoundException"),
        "Expected NotFoundException, got: {err_str}"
    );
}

// ---- Input pagination test ----

#[tokio::test]
async fn test_list_inputs_pagination() {
    let client = make_medialive_client().await;

    // Create 5 inputs
    for idx in 0..5 {
        client
            .create_input()
            .name(format!("paginput {idx}"))
            .r#type(aws_sdk_medialive::types::InputType::RtpPush)
            .role_arn("arn:aws:iam::123456789012:role/TestMediaLiveInputCreateRole")
            .send()
            .await
            .unwrap();
    }

    // First page: max_results=2
    let page1 = client
        .list_inputs()
        .max_results(2)
        .send()
        .await
        .expect("list_inputs page1 should succeed");

    assert_eq!(page1.inputs().len(), 2);
    let next_token = page1.next_token().expect("Expected a next_token for page1");

    // Second page: next 2
    let page2 = client
        .list_inputs()
        .max_results(2)
        .next_token(next_token)
        .send()
        .await
        .expect("list_inputs page2 should succeed");

    assert_eq!(page2.inputs().len(), 2);
    let next_token2 = page2.next_token().expect("Expected a next_token for page2");

    // Third page: remaining 1
    let page3 = client
        .list_inputs()
        .max_results(2)
        .next_token(next_token2)
        .send()
        .await
        .expect("list_inputs page3 should succeed");

    assert_eq!(page3.inputs().len(), 1);
    assert!(
        page3.next_token().is_none(),
        "Expected no next_token on last page"
    );
}

// ---- Input full lifecycle test ----

#[tokio::test]
async fn test_input_full_lifecycle() {
    let client = make_medialive_client().await;

    // Create
    let create_resp = client
        .create_input()
        .name("lifecycle-input")
        .r#type(aws_sdk_medialive::types::InputType::RtpPush)
        .role_arn("arn:aws:iam::123456789012:role/TestMediaLiveInputCreateRole")
        .send()
        .await
        .unwrap();

    let input_id = create_resp.input().unwrap().id().unwrap().to_string();
    assert_eq!(
        create_resp.input().unwrap().state(),
        Some(&aws_sdk_medialive::types::InputState::Creating)
    );

    // Describe — state becomes DETACHED
    let describe_resp = client
        .describe_input()
        .input_id(&input_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        describe_resp.state(),
        Some(&aws_sdk_medialive::types::InputState::Detached)
    );
    assert_eq!(describe_resp.name(), Some("lifecycle-input"));

    // Update name
    let update_resp = client
        .update_input()
        .input_id(&input_id)
        .name("lifecycle-input-updated")
        .send()
        .await
        .unwrap();
    assert_eq!(
        update_resp.input().unwrap().name(),
        Some("lifecycle-input-updated")
    );

    // Delete
    client
        .delete_input()
        .input_id(&input_id)
        .send()
        .await
        .expect("delete_input should succeed");

    // Describe after delete — state is DELETED (inputs stay in store but marked DELETED)
    let describe_after_delete = client
        .describe_input()
        .input_id(&input_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        describe_after_delete.state(),
        Some(&aws_sdk_medialive::types::InputState::Deleted)
    );
    assert_eq!(
        describe_after_delete.name(),
        Some("lifecycle-input-updated")
    );
}
