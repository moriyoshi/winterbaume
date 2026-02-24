use aws_sdk_connectcampaigns::config::BehaviorVersion;
use winterbaume_connectcampaigns::ConnectCampaignsService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_connectcampaigns::Client {
    let mock = MockAws::builder()
        .with_service(ConnectCampaignsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_connectcampaigns::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_connectcampaigns::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_describe_campaign() {
    let client = make_client().await;

    let dialer = aws_sdk_connectcampaigns::types::DialerConfig::ProgressiveDialerConfig(
        aws_sdk_connectcampaigns::types::ProgressiveDialerConfig::builder()
            .bandwidth_allocation(0.5)
            .build()
            .unwrap(),
    );

    let outbound = aws_sdk_connectcampaigns::types::OutboundCallConfig::builder()
        .connect_contact_flow_id("flow-123")
        .build()
        .unwrap();

    let resp = client
        .create_campaign()
        .name("test-campaign")
        .connect_instance_id("instance-123")
        .dialer_config(dialer)
        .outbound_call_config(outbound)
        .send()
        .await
        .expect("create_campaign should succeed");

    let campaign_id = resp.id().expect("should have id");
    assert!(!campaign_id.is_empty());
    assert!(resp.arn().is_some());

    let describe_resp = client
        .describe_campaign()
        .id(campaign_id)
        .send()
        .await
        .expect("describe_campaign should succeed");

    let campaign = describe_resp.campaign().expect("should have campaign");
    assert_eq!(campaign.name(), "test-campaign");
    assert_eq!(campaign.connect_instance_id(), "instance-123");
}

#[tokio::test]
async fn test_delete_campaign() {
    let client = make_client().await;

    let dialer = aws_sdk_connectcampaigns::types::DialerConfig::ProgressiveDialerConfig(
        aws_sdk_connectcampaigns::types::ProgressiveDialerConfig::builder()
            .bandwidth_allocation(0.5)
            .build()
            .unwrap(),
    );

    let outbound = aws_sdk_connectcampaigns::types::OutboundCallConfig::builder()
        .connect_contact_flow_id("flow-123")
        .build()
        .unwrap();

    let resp = client
        .create_campaign()
        .name("to-delete")
        .connect_instance_id("instance-456")
        .dialer_config(dialer)
        .outbound_call_config(outbound)
        .send()
        .await
        .unwrap();

    let campaign_id = resp.id().unwrap().to_string();

    client
        .delete_campaign()
        .id(&campaign_id)
        .send()
        .await
        .expect("delete_campaign should succeed");

    let result = client.describe_campaign().id(&campaign_id).send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_campaigns() {
    let client = make_client().await;

    let dialer = aws_sdk_connectcampaigns::types::DialerConfig::ProgressiveDialerConfig(
        aws_sdk_connectcampaigns::types::ProgressiveDialerConfig::builder()
            .bandwidth_allocation(0.5)
            .build()
            .unwrap(),
    );

    let outbound = aws_sdk_connectcampaigns::types::OutboundCallConfig::builder()
        .connect_contact_flow_id("flow-123")
        .build()
        .unwrap();

    client
        .create_campaign()
        .name("campaign-1")
        .connect_instance_id("instance-1")
        .dialer_config(dialer.clone())
        .outbound_call_config(outbound.clone())
        .send()
        .await
        .unwrap();

    let dialer2 = aws_sdk_connectcampaigns::types::DialerConfig::ProgressiveDialerConfig(
        aws_sdk_connectcampaigns::types::ProgressiveDialerConfig::builder()
            .bandwidth_allocation(0.8)
            .build()
            .unwrap(),
    );

    let outbound2 = aws_sdk_connectcampaigns::types::OutboundCallConfig::builder()
        .connect_contact_flow_id("flow-456")
        .build()
        .unwrap();

    client
        .create_campaign()
        .name("campaign-2")
        .connect_instance_id("instance-2")
        .dialer_config(dialer2)
        .outbound_call_config(outbound2)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_campaigns()
        .send()
        .await
        .expect("list_campaigns should succeed");

    assert_eq!(resp.campaign_summary_list().len(), 2);
}

#[tokio::test]
async fn test_describe_nonexistent_campaign() {
    let client = make_client().await;

    let result = client.describe_campaign().id("nonexistent-id").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_campaign() {
    let client = make_client().await;

    let result = client.delete_campaign().id("nonexistent-id").send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_campaign_state() {
    let client = make_client().await;

    let dialer = aws_sdk_connectcampaigns::types::DialerConfig::ProgressiveDialerConfig(
        aws_sdk_connectcampaigns::types::ProgressiveDialerConfig::builder()
            .bandwidth_allocation(0.5)
            .build()
            .unwrap(),
    );

    let outbound = aws_sdk_connectcampaigns::types::OutboundCallConfig::builder()
        .connect_contact_flow_id("flow-123")
        .build()
        .unwrap();

    let resp = client
        .create_campaign()
        .name("state-test")
        .connect_instance_id("instance-state")
        .dialer_config(dialer)
        .outbound_call_config(outbound)
        .send()
        .await
        .unwrap();

    let campaign_id = resp.id().unwrap();

    let state_resp = client
        .get_campaign_state()
        .id(campaign_id)
        .send()
        .await
        .expect("get_campaign_state should succeed");

    assert_eq!(
        state_resp.state(),
        Some(&aws_sdk_connectcampaigns::types::CampaignState::Initialized)
    );
}

#[tokio::test]
async fn test_start_stop_campaign() {
    let client = make_client().await;

    let dialer = aws_sdk_connectcampaigns::types::DialerConfig::ProgressiveDialerConfig(
        aws_sdk_connectcampaigns::types::ProgressiveDialerConfig::builder()
            .bandwidth_allocation(0.5)
            .build()
            .unwrap(),
    );

    let outbound = aws_sdk_connectcampaigns::types::OutboundCallConfig::builder()
        .connect_contact_flow_id("flow-123")
        .build()
        .unwrap();

    let resp = client
        .create_campaign()
        .name("start-stop-test")
        .connect_instance_id("instance-ss")
        .dialer_config(dialer)
        .outbound_call_config(outbound)
        .send()
        .await
        .unwrap();

    let campaign_id = resp.id().unwrap().to_string();

    // Start campaign
    client
        .start_campaign()
        .id(&campaign_id)
        .send()
        .await
        .expect("start_campaign should succeed");

    let state_resp = client
        .get_campaign_state()
        .id(&campaign_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        state_resp.state(),
        Some(&aws_sdk_connectcampaigns::types::CampaignState::Running)
    );

    // Stop campaign
    client
        .stop_campaign()
        .id(&campaign_id)
        .send()
        .await
        .expect("stop_campaign should succeed");

    let state_resp = client
        .get_campaign_state()
        .id(&campaign_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        state_resp.state(),
        Some(&aws_sdk_connectcampaigns::types::CampaignState::Stopped)
    );
}

#[tokio::test]
async fn test_pause_resume_campaign() {
    let client = make_client().await;

    let dialer = aws_sdk_connectcampaigns::types::DialerConfig::ProgressiveDialerConfig(
        aws_sdk_connectcampaigns::types::ProgressiveDialerConfig::builder()
            .bandwidth_allocation(0.5)
            .build()
            .unwrap(),
    );

    let outbound = aws_sdk_connectcampaigns::types::OutboundCallConfig::builder()
        .connect_contact_flow_id("flow-123")
        .build()
        .unwrap();

    let resp = client
        .create_campaign()
        .name("pause-resume-test")
        .connect_instance_id("instance-pr")
        .dialer_config(dialer)
        .outbound_call_config(outbound)
        .send()
        .await
        .unwrap();

    let campaign_id = resp.id().unwrap().to_string();

    // Start first, then pause
    client
        .start_campaign()
        .id(&campaign_id)
        .send()
        .await
        .unwrap();

    client
        .pause_campaign()
        .id(&campaign_id)
        .send()
        .await
        .expect("pause_campaign should succeed");

    let state_resp = client
        .get_campaign_state()
        .id(&campaign_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        state_resp.state(),
        Some(&aws_sdk_connectcampaigns::types::CampaignState::Paused)
    );

    // Resume
    client
        .resume_campaign()
        .id(&campaign_id)
        .send()
        .await
        .expect("resume_campaign should succeed");

    let state_resp = client
        .get_campaign_state()
        .id(&campaign_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        state_resp.state(),
        Some(&aws_sdk_connectcampaigns::types::CampaignState::Running)
    );
}

#[tokio::test]
async fn test_start_instance_onboarding_and_get_config() {
    let client = make_client().await;

    let encryption_config = aws_sdk_connectcampaigns::types::EncryptionConfig::builder()
        .enabled(true)
        .encryption_type(aws_sdk_connectcampaigns::types::EncryptionType::Kms)
        .key_arn("arn:aws:kms:us-east-1:123456789012:key/test-key")
        .build();

    let resp = client
        .start_instance_onboarding_job()
        .connect_instance_id("instance-onboard")
        .encryption_config(encryption_config)
        .send()
        .await
        .expect("start_instance_onboarding_job should succeed");

    let job_status = resp
        .connect_instance_onboarding_job_status()
        .expect("should have job status");
    assert_eq!(job_status.connect_instance_id(), "instance-onboard");
    assert_eq!(
        job_status.status(),
        &aws_sdk_connectcampaigns::types::InstanceOnboardingJobStatusCode::Succeeded
    );

    // Now get the instance config
    let config_resp = client
        .get_connect_instance_config()
        .connect_instance_id("instance-onboard")
        .send()
        .await
        .expect("get_connect_instance_config should succeed");

    let config = config_resp
        .connect_instance_config()
        .expect("should have config");
    assert_eq!(config.connect_instance_id(), "instance-onboard");
    assert!(
        config
            .encryption_config()
            .expect("should have encryption_config")
            .enabled()
    );
}

#[tokio::test]
async fn test_tag_and_list_tags_for_resource() {
    let client = make_client().await;

    let dialer = aws_sdk_connectcampaigns::types::DialerConfig::ProgressiveDialerConfig(
        aws_sdk_connectcampaigns::types::ProgressiveDialerConfig::builder()
            .bandwidth_allocation(0.5)
            .build()
            .unwrap(),
    );

    let outbound = aws_sdk_connectcampaigns::types::OutboundCallConfig::builder()
        .connect_contact_flow_id("flow-123")
        .build()
        .unwrap();

    let resp = client
        .create_campaign()
        .name("tag-test")
        .connect_instance_id("instance-tag")
        .dialer_config(dialer)
        .outbound_call_config(outbound)
        .send()
        .await
        .unwrap();

    let arn = resp.arn().unwrap().to_string();

    // Tag the resource
    client
        .tag_resource()
        .arn(&arn)
        .tags("env", "test")
        .tags("team", "platform")
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let tags_resp = client
        .list_tags_for_resource()
        .arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("env"), Some(&"test".to_string()));
    assert_eq!(tags.get("team"), Some(&"platform".to_string()));
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_client().await;

    let dialer = aws_sdk_connectcampaigns::types::DialerConfig::ProgressiveDialerConfig(
        aws_sdk_connectcampaigns::types::ProgressiveDialerConfig::builder()
            .bandwidth_allocation(0.5)
            .build()
            .unwrap(),
    );

    let outbound = aws_sdk_connectcampaigns::types::OutboundCallConfig::builder()
        .connect_contact_flow_id("flow-123")
        .build()
        .unwrap();

    let resp = client
        .create_campaign()
        .name("untag-test")
        .connect_instance_id("instance-untag")
        .dialer_config(dialer)
        .outbound_call_config(outbound)
        .send()
        .await
        .unwrap();

    let arn = resp.arn().unwrap().to_string();

    // Add tags
    client
        .tag_resource()
        .arn(&arn)
        .tags("env", "test")
        .tags("team", "platform")
        .tags("version", "1")
        .send()
        .await
        .unwrap();

    // Remove one tag
    client
        .untag_resource()
        .arn(&arn)
        .tag_keys("team")
        .send()
        .await
        .expect("untag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .arn(&arn)
        .send()
        .await
        .unwrap();

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("env"), Some(&"test".to_string()));
    assert!(tags.get("team").is_none());
    assert_eq!(tags.get("version"), Some(&"1".to_string()));
}

/// Regression: the SDK URL-encodes ARN colons in path-style /tags/{arn+}
/// requests. The handler must URL-decode the trailing path segments
/// before using the result as the state-map key, otherwise tags
/// disappear between TagResource and ListTagsForResource calls.
#[tokio::test]
async fn test_tag_resource_url_encoded_arn_roundtrip() {
    let client = make_client().await;

    let dialer = aws_sdk_connectcampaigns::types::DialerConfig::ProgressiveDialerConfig(
        aws_sdk_connectcampaigns::types::ProgressiveDialerConfig::builder()
            .bandwidth_allocation(0.5)
            .build()
            .unwrap(),
    );
    let outbound = aws_sdk_connectcampaigns::types::OutboundCallConfig::builder()
        .connect_contact_flow_id("flow-1")
        .build()
        .unwrap();

    let resp = client
        .create_campaign()
        .name("urlenc-tag")
        .connect_instance_id("instance-urlenc")
        .dialer_config(dialer)
        .outbound_call_config(outbound)
        .send()
        .await
        .unwrap();

    let arn = resp.arn().unwrap().to_string();
    assert!(arn.contains(':'), "campaign ARN must contain colons");

    client
        .tag_resource()
        .arn(&arn)
        .tags("env", "prod")
        .send()
        .await
        .expect("tag_resource");
    let tags_resp = client
        .list_tags_for_resource()
        .arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource");
    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("env"), Some(&"prod".to_string()));

    client
        .untag_resource()
        .arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource");
    let tags_resp = client
        .list_tags_for_resource()
        .arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource");
    assert!(tags_resp.tags().map(|t| t.is_empty()).unwrap_or(true));
}

#[tokio::test]
async fn test_create_delete_then_list_empty() {
    let client = make_client().await;

    let dialer = aws_sdk_connectcampaigns::types::DialerConfig::ProgressiveDialerConfig(
        aws_sdk_connectcampaigns::types::ProgressiveDialerConfig::builder()
            .bandwidth_allocation(0.5)
            .build()
            .unwrap(),
    );

    let outbound = aws_sdk_connectcampaigns::types::OutboundCallConfig::builder()
        .connect_contact_flow_id("flow-123")
        .build()
        .unwrap();

    let resp = client
        .create_campaign()
        .name("temp-campaign")
        .connect_instance_id("instance-temp")
        .dialer_config(dialer)
        .outbound_call_config(outbound)
        .send()
        .await
        .unwrap();

    let campaign_id = resp.id().unwrap().to_string();

    let list = client.list_campaigns().send().await.unwrap();
    assert_eq!(list.campaign_summary_list().len(), 1);

    client
        .delete_campaign()
        .id(&campaign_id)
        .send()
        .await
        .unwrap();

    let list = client.list_campaigns().send().await.unwrap();
    assert_eq!(list.campaign_summary_list().len(), 0);
}

// ============================================================================
// Tests derived from AWS documentation: Amazon Connect Outbound Campaigns
// ============================================================================

/// Helper to build a minimal progressive DialerConfig for tests.
fn progressive_dialer() -> aws_sdk_connectcampaigns::types::DialerConfig {
    aws_sdk_connectcampaigns::types::DialerConfig::ProgressiveDialerConfig(
        aws_sdk_connectcampaigns::types::ProgressiveDialerConfig::builder()
            .bandwidth_allocation(0.5)
            .build()
            .unwrap(),
    )
}

/// Helper to build a minimal OutboundCallConfig for tests.
fn minimal_outbound() -> aws_sdk_connectcampaigns::types::OutboundCallConfig {
    aws_sdk_connectcampaigns::types::OutboundCallConfig::builder()
        .connect_contact_flow_id("flow-default")
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_create_campaign_with_tags() {
    let client = make_client().await;

    let resp = client
        .create_campaign()
        .name("tagged-campaign")
        .connect_instance_id("instance-tagged")
        .dialer_config(progressive_dialer())
        .outbound_call_config(minimal_outbound())
        .tags("env", "prod")
        .tags("team", "outbound")
        .send()
        .await
        .expect("create_campaign with tags should succeed");

    // Tags should be echoed back in the create response
    let tags = resp
        .tags()
        .expect("tags should be present in create response");
    assert_eq!(tags.get("env"), Some(&"prod".to_string()));
    assert_eq!(tags.get("team"), Some(&"outbound".to_string()));

    // Describe should also reflect the tags
    let campaign_id = resp.id().unwrap();
    let describe_resp = client
        .describe_campaign()
        .id(campaign_id)
        .send()
        .await
        .unwrap();
    let campaign = describe_resp.campaign().unwrap();
    let desc_tags = campaign.tags().expect("describe should return tags");
    assert_eq!(desc_tags.get("env"), Some(&"prod".to_string()));
    assert_eq!(desc_tags.get("team"), Some(&"outbound".to_string()));
}

#[tokio::test]
async fn test_create_campaign_agentless_dialer() {
    let client = make_client().await;

    let agentless_dialer = aws_sdk_connectcampaigns::types::DialerConfig::AgentlessDialerConfig(
        aws_sdk_connectcampaigns::types::AgentlessDialerConfig::builder()
            .dialing_capacity(0.75)
            .build(),
    );

    let resp = client
        .create_campaign()
        .name("agentless-campaign")
        .connect_instance_id("instance-agentless")
        .dialer_config(agentless_dialer)
        .outbound_call_config(minimal_outbound())
        .send()
        .await
        .expect("create_campaign with agentless dialer should succeed");

    assert!(!resp.id().unwrap_or_default().is_empty());
    assert!(resp.arn().is_some());

    let campaign_id = resp.id().unwrap();
    let describe_resp = client
        .describe_campaign()
        .id(campaign_id)
        .send()
        .await
        .unwrap();
    let campaign = describe_resp.campaign().unwrap();

    // Dialer config should be agentless type
    let dialer = campaign
        .dialer_config()
        .expect("dialer_config should be present");
    assert!(
        matches!(
            dialer,
            aws_sdk_connectcampaigns::types::DialerConfig::AgentlessDialerConfig(_)
        ),
        "Expected AgentlessDialerConfig, got: {:?}",
        dialer
    );
}

#[tokio::test]
async fn test_create_campaign_predictive_dialer() {
    let client = make_client().await;

    let predictive_dialer = aws_sdk_connectcampaigns::types::DialerConfig::PredictiveDialerConfig(
        aws_sdk_connectcampaigns::types::PredictiveDialerConfig::builder()
            .bandwidth_allocation(0.8)
            .dialing_capacity(1.0)
            .build()
            .unwrap(),
    );

    let resp = client
        .create_campaign()
        .name("predictive-campaign")
        .connect_instance_id("instance-predictive")
        .dialer_config(predictive_dialer)
        .outbound_call_config(minimal_outbound())
        .send()
        .await
        .expect("create_campaign with predictive dialer should succeed");

    let campaign_id = resp.id().unwrap();
    let describe_resp = client
        .describe_campaign()
        .id(campaign_id)
        .send()
        .await
        .unwrap();
    let campaign = describe_resp.campaign().unwrap();
    let dialer = campaign
        .dialer_config()
        .expect("dialer_config should be present");
    assert!(
        matches!(
            dialer,
            aws_sdk_connectcampaigns::types::DialerConfig::PredictiveDialerConfig(_)
        ),
        "Expected PredictiveDialerConfig, got: {:?}",
        dialer
    );
}

#[tokio::test]
async fn test_create_campaign_full_outbound_config() {
    let client = make_client().await;

    let amd_config = aws_sdk_connectcampaigns::types::AnswerMachineDetectionConfig::builder()
        .enable_answer_machine_detection(true)
        .await_answer_machine_prompt(false)
        .build()
        .unwrap();

    let outbound = aws_sdk_connectcampaigns::types::OutboundCallConfig::builder()
        .connect_contact_flow_id("flow-full")
        .connect_queue_id("queue-999")
        .connect_source_phone_number("+15550001234")
        .answer_machine_detection_config(amd_config)
        .build()
        .unwrap();

    let resp = client
        .create_campaign()
        .name("full-outbound-campaign")
        .connect_instance_id("instance-full")
        .dialer_config(progressive_dialer())
        .outbound_call_config(outbound)
        .send()
        .await
        .expect("create_campaign with full outbound config should succeed");

    let campaign_id = resp.id().unwrap();
    let describe_resp = client
        .describe_campaign()
        .id(campaign_id)
        .send()
        .await
        .unwrap();
    let campaign = describe_resp.campaign().unwrap();
    let obc = campaign
        .outbound_call_config()
        .expect("outbound_call_config should be present");

    assert_eq!(obc.connect_contact_flow_id(), "flow-full");
    assert_eq!(obc.connect_queue_id(), Some("queue-999"));
    assert_eq!(obc.connect_source_phone_number(), Some("+15550001234"));

    let amd = obc
        .answer_machine_detection_config()
        .expect("AMD config should be present");
    assert!(amd.enable_answer_machine_detection());
    assert_eq!(amd.await_answer_machine_prompt(), Some(false));
}

#[tokio::test]
async fn test_campaign_arn_format() {
    let client = make_client().await;

    let resp = client
        .create_campaign()
        .name("arn-format-test")
        .connect_instance_id("instance-arn")
        .dialer_config(progressive_dialer())
        .outbound_call_config(minimal_outbound())
        .send()
        .await
        .unwrap();

    let arn = resp.arn().expect("arn should be present");
    assert!(
        arn.starts_with("arn:aws:connect-campaigns:"),
        "ARN should start with arn:aws:connect-campaigns:, got: {arn}"
    );
    assert!(
        arn.contains("campaign/"),
        "ARN should contain campaign/, got: {arn}"
    );
}

#[tokio::test]
async fn test_get_campaign_state_not_found() {
    let client = make_client().await;

    let err = client
        .get_campaign_state()
        .id("nonexistent-state-id")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_start_already_running_conflict() {
    let client = make_client().await;

    let resp = client
        .create_campaign()
        .name("already-running")
        .connect_instance_id("instance-ar")
        .dialer_config(progressive_dialer())
        .outbound_call_config(minimal_outbound())
        .send()
        .await
        .unwrap();
    let campaign_id = resp.id().unwrap().to_string();

    // Start once — succeeds
    client
        .start_campaign()
        .id(&campaign_id)
        .send()
        .await
        .expect("first start should succeed");

    // Start again while running — ConflictException
    let err = client
        .start_campaign()
        .id(&campaign_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConflictException"),
        "Expected ConflictException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_stop_already_stopped_conflict() {
    let client = make_client().await;

    let resp = client
        .create_campaign()
        .name("already-stopped")
        .connect_instance_id("instance-as")
        .dialer_config(progressive_dialer())
        .outbound_call_config(minimal_outbound())
        .send()
        .await
        .unwrap();
    let campaign_id = resp.id().unwrap().to_string();

    // Start then stop
    client
        .start_campaign()
        .id(&campaign_id)
        .send()
        .await
        .unwrap();
    client
        .stop_campaign()
        .id(&campaign_id)
        .send()
        .await
        .unwrap();

    // Stop again — ConflictException
    let err = client
        .stop_campaign()
        .id(&campaign_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConflictException"),
        "Expected ConflictException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_pause_non_running_conflict() {
    let client = make_client().await;

    let resp = client
        .create_campaign()
        .name("pause-initialized")
        .connect_instance_id("instance-pi")
        .dialer_config(progressive_dialer())
        .outbound_call_config(minimal_outbound())
        .send()
        .await
        .unwrap();
    let campaign_id = resp.id().unwrap().to_string();

    // Campaign is Initialized — pause should fail
    let err = client
        .pause_campaign()
        .id(&campaign_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConflictException"),
        "Expected ConflictException when pausing Initialized campaign, got: {err_str}"
    );
}

#[tokio::test]
async fn test_resume_non_paused_conflict() {
    let client = make_client().await;

    let resp = client
        .create_campaign()
        .name("resume-running")
        .connect_instance_id("instance-rr")
        .dialer_config(progressive_dialer())
        .outbound_call_config(minimal_outbound())
        .send()
        .await
        .unwrap();
    let campaign_id = resp.id().unwrap().to_string();

    // Start (Running state) then try to resume
    client
        .start_campaign()
        .id(&campaign_id)
        .send()
        .await
        .unwrap();

    let err = client
        .resume_campaign()
        .id(&campaign_id)
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ConflictException"),
        "Expected ConflictException when resuming Running campaign, got: {err_str}"
    );
}

#[tokio::test]
async fn test_start_campaign_not_found() {
    let client = make_client().await;

    let err = client
        .start_campaign()
        .id("nonexistent-start-id")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_stop_campaign_not_found() {
    let client = make_client().await;

    let err = client
        .stop_campaign()
        .id("nonexistent-stop-id")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_pause_campaign_not_found() {
    let client = make_client().await;

    let err = client
        .pause_campaign()
        .id("nonexistent-pause-id")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_resume_campaign_not_found() {
    let client = make_client().await;

    let err = client
        .resume_campaign()
        .id("nonexistent-resume-id")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_connect_instance_config_not_found() {
    let client = make_client().await;

    let err = client
        .get_connect_instance_config()
        .connect_instance_id("nonexistent-instance-id")
        .send()
        .await
        .unwrap_err();
    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_campaigns_summary_fields() {
    let client = make_client().await;

    let create_resp = client
        .create_campaign()
        .name("summary-fields-test")
        .connect_instance_id("instance-sf")
        .dialer_config(progressive_dialer())
        .outbound_call_config(minimal_outbound())
        .send()
        .await
        .unwrap();

    let created_id = create_resp.id().unwrap().to_string();
    let created_arn = create_resp.arn().unwrap().to_string();

    let list_resp = client.list_campaigns().send().await.unwrap();
    let summaries = list_resp.campaign_summary_list();
    assert!(!summaries.is_empty(), "Should have at least one campaign");

    // Find our campaign by id
    let summary = summaries
        .iter()
        .find(|s| s.id() == created_id)
        .expect("Created campaign should appear in list");

    assert_eq!(summary.id(), created_id, "summary id should match");
    assert_eq!(summary.arn(), created_arn, "summary arn should match");
    assert_eq!(
        summary.name(),
        "summary-fields-test",
        "summary name should match"
    );
    assert_eq!(
        summary.connect_instance_id(),
        "instance-sf",
        "summary connectInstanceId should match"
    );
}

#[tokio::test]
async fn test_campaign_lifecycle() {
    let client = make_client().await;

    // Create
    let resp = client
        .create_campaign()
        .name("lifecycle-campaign")
        .connect_instance_id("instance-lifecycle")
        .dialer_config(progressive_dialer())
        .outbound_call_config(minimal_outbound())
        .send()
        .await
        .expect("create should succeed");

    let campaign_id = resp.id().unwrap().to_string();

    // Initialized state
    let state_resp = client
        .get_campaign_state()
        .id(&campaign_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        state_resp.state(),
        Some(&aws_sdk_connectcampaigns::types::CampaignState::Initialized)
    );

    // Start -> Running
    client
        .start_campaign()
        .id(&campaign_id)
        .send()
        .await
        .expect("start should succeed");
    let state_resp = client
        .get_campaign_state()
        .id(&campaign_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        state_resp.state(),
        Some(&aws_sdk_connectcampaigns::types::CampaignState::Running)
    );

    // Pause -> Paused
    client
        .pause_campaign()
        .id(&campaign_id)
        .send()
        .await
        .expect("pause should succeed");
    let state_resp = client
        .get_campaign_state()
        .id(&campaign_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        state_resp.state(),
        Some(&aws_sdk_connectcampaigns::types::CampaignState::Paused)
    );

    // Resume -> Running
    client
        .resume_campaign()
        .id(&campaign_id)
        .send()
        .await
        .expect("resume should succeed");
    let state_resp = client
        .get_campaign_state()
        .id(&campaign_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        state_resp.state(),
        Some(&aws_sdk_connectcampaigns::types::CampaignState::Running)
    );

    // Stop -> Stopped
    client
        .stop_campaign()
        .id(&campaign_id)
        .send()
        .await
        .expect("stop should succeed");
    let state_resp = client
        .get_campaign_state()
        .id(&campaign_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        state_resp.state(),
        Some(&aws_sdk_connectcampaigns::types::CampaignState::Stopped)
    );

    // Delete
    client
        .delete_campaign()
        .id(&campaign_id)
        .send()
        .await
        .expect("delete should succeed");

    // Gone
    let result = client.describe_campaign().id(&campaign_id).send().await;
    assert!(result.is_err(), "describe after delete should fail");
    let err_str = format!("{:?}", result.unwrap_err());
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException after delete, got: {err_str}"
    );
}
