use aws_sdk_chatbot::config::BehaviorVersion;
use winterbaume_chatbot::ChatbotService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_chatbot::Client {
    let mock = MockAws::builder()
        .with_service(ChatbotService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_chatbot::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_chatbot::Client::new(&config)
}

// ---- Slack channel configuration tests ----

#[tokio::test]
async fn test_create_and_describe_slack_channel_configuration() {
    let client = make_client().await;

    client
        .create_slack_channel_configuration()
        .configuration_name("test-slack")
        .slack_team_id("T12345")
        .slack_channel_id("C12345")
        .iam_role_arn("arn:aws:iam::123456789012:role/chatbot-role")
        .send()
        .await
        .expect("create_slack_channel_configuration should succeed");

    let resp = client
        .describe_slack_channel_configurations()
        .send()
        .await
        .expect("describe_slack_channel_configurations should succeed");

    let configs = resp.slack_channel_configurations();
    assert_eq!(configs.len(), 1);
    let cfg = &configs[0];
    assert_eq!(cfg.configuration_name(), Some("test-slack"));
    assert_eq!(cfg.slack_team_id(), "T12345");
}

#[tokio::test]
async fn test_update_slack_channel_configuration() {
    let client = make_client().await;

    let resp = client
        .create_slack_channel_configuration()
        .configuration_name("upd-slack")
        .slack_team_id("T99")
        .slack_channel_id("C99")
        .iam_role_arn("arn:aws:iam::123456789012:role/chatbot-role")
        .send()
        .await
        .expect("create should succeed");

    let arn = resp
        .channel_configuration()
        .map(|c| c.chat_configuration_arn().to_string())
        .expect("should have config");

    let resp = client
        .update_slack_channel_configuration()
        .chat_configuration_arn(&arn)
        .slack_channel_id("C100")
        .iam_role_arn("arn:aws:iam::123456789012:role/new-role")
        .send()
        .await
        .expect("update should succeed");

    let cfg = resp.channel_configuration().expect("should have config");
    assert_eq!(cfg.slack_channel_id(), "C100");
    assert_eq!(
        cfg.iam_role_arn(),
        "arn:aws:iam::123456789012:role/new-role"
    );
}

#[tokio::test]
async fn test_delete_slack_channel_configuration() {
    let client = make_client().await;

    let resp = client
        .create_slack_channel_configuration()
        .configuration_name("del-slack")
        .slack_team_id("T77")
        .slack_channel_id("C77")
        .iam_role_arn("arn:aws:iam::123456789012:role/chatbot-role")
        .send()
        .await
        .expect("create should succeed");

    let arn = resp
        .channel_configuration()
        .map(|c| c.chat_configuration_arn().to_string())
        .expect("should have config");

    client
        .delete_slack_channel_configuration()
        .chat_configuration_arn(&arn)
        .send()
        .await
        .expect("delete should succeed");

    let resp = client
        .describe_slack_channel_configurations()
        .send()
        .await
        .expect("describe should succeed");

    assert_eq!(resp.slack_channel_configurations().len(), 0);
}

// ---- Chime webhook configuration tests ----

#[tokio::test]
async fn test_create_and_describe_chime_webhook_configuration() {
    let client = make_client().await;

    client
        .create_chime_webhook_configuration()
        .configuration_name("test-chime")
        .webhook_description("My Chime hook")
        .webhook_url("https://hooks.chime.aws/incomingwebhooks/test")
        .iam_role_arn("arn:aws:iam::123456789012:role/chatbot-role")
        .sns_topic_arns("arn:aws:sns:us-east-1:123456789012:my-topic")
        .send()
        .await
        .expect("create_chime_webhook_configuration should succeed");

    let resp = client
        .describe_chime_webhook_configurations()
        .send()
        .await
        .expect("describe_chime_webhook_configurations should succeed");

    let configs = resp.webhook_configurations();
    assert_eq!(configs.len(), 1);
    let cfg = &configs[0];
    assert_eq!(cfg.configuration_name(), Some("test-chime"));
}

#[tokio::test]
async fn test_delete_chime_webhook_configuration() {
    let client = make_client().await;

    let resp = client
        .create_chime_webhook_configuration()
        .configuration_name("del-chime")
        .webhook_description("To delete")
        .webhook_url("https://hooks.chime.aws/incomingwebhooks/del")
        .iam_role_arn("arn:aws:iam::123456789012:role/chatbot-role")
        .sns_topic_arns("arn:aws:sns:us-east-1:123456789012:topic")
        .send()
        .await
        .expect("create should succeed");

    let arn = resp
        .webhook_configuration()
        .map(|c| c.chat_configuration_arn().to_string())
        .expect("should have config");

    client
        .delete_chime_webhook_configuration()
        .chat_configuration_arn(&arn)
        .send()
        .await
        .expect("delete should succeed");

    let resp = client
        .describe_chime_webhook_configurations()
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(resp.webhook_configurations().len(), 0);
}

// ---- Teams channel configuration tests ----

#[tokio::test]
async fn test_create_and_list_microsoft_teams_channel_configuration() {
    let client = make_client().await;

    client
        .create_microsoft_teams_channel_configuration()
        .configuration_name("test-teams")
        .team_id("team-123")
        .tenant_id("tenant-456")
        .channel_id("channel-789")
        .iam_role_arn("arn:aws:iam::123456789012:role/chatbot-role")
        .send()
        .await
        .expect("create_microsoft_teams_channel_configuration should succeed");

    let resp = client
        .list_microsoft_teams_channel_configurations()
        .send()
        .await
        .expect("list_microsoft_teams_channel_configurations should succeed");

    let configs = resp.team_channel_configurations();
    assert_eq!(configs.len(), 1);
    let cfg = &configs[0];
    assert_eq!(cfg.configuration_name(), Some("test-teams"));
    assert_eq!(cfg.team_id(), "team-123");
}

#[tokio::test]
async fn test_delete_microsoft_teams_channel_configuration() {
    let client = make_client().await;

    let resp = client
        .create_microsoft_teams_channel_configuration()
        .configuration_name("del-teams")
        .team_id("team-del")
        .tenant_id("tenant-del")
        .channel_id("channel-del")
        .iam_role_arn("arn:aws:iam::123456789012:role/chatbot-role")
        .send()
        .await
        .expect("create should succeed");

    let arn = resp
        .channel_configuration()
        .map(|c| c.chat_configuration_arn().to_string())
        .expect("should have config");

    client
        .delete_microsoft_teams_channel_configuration()
        .chat_configuration_arn(&arn)
        .send()
        .await
        .expect("delete should succeed");

    let resp = client
        .list_microsoft_teams_channel_configurations()
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(resp.team_channel_configurations().len(), 0);
}

// ---- Coverage for FIX(terraform-e2e) handler fixes ----

#[tokio::test]
async fn test_get_microsoft_teams_channel_configuration() {
    let client = make_client().await;

    let resp = client
        .create_microsoft_teams_channel_configuration()
        .configuration_name("get-teams")
        .team_id("team-get")
        .tenant_id("tenant-get")
        .channel_id("channel-get")
        .iam_role_arn("arn:aws:iam::123456789012:role/chatbot-role")
        .send()
        .await
        .expect("create should succeed");

    let arn = resp
        .channel_configuration()
        .map(|c| c.chat_configuration_arn().to_string())
        .expect("should have config");

    let resp = client
        .get_microsoft_teams_channel_configuration()
        .chat_configuration_arn(&arn)
        .send()
        .await
        .expect("get should succeed");

    let cfg = resp.channel_configuration().expect("should have config");
    assert_eq!(cfg.configuration_name(), Some("get-teams"));
    assert_eq!(cfg.team_id(), "team-get");
    assert_eq!(cfg.tenant_id(), "tenant-get");
    assert_eq!(cfg.channel_id(), "channel-get");
}

#[tokio::test]
async fn test_update_microsoft_teams_channel_configuration() {
    let client = make_client().await;

    let resp = client
        .create_microsoft_teams_channel_configuration()
        .configuration_name("upd-teams")
        .team_id("team-upd")
        .tenant_id("tenant-upd")
        .channel_id("channel-upd")
        .iam_role_arn("arn:aws:iam::123456789012:role/chatbot-role")
        .send()
        .await
        .expect("create should succeed");

    let arn = resp
        .channel_configuration()
        .map(|c| c.chat_configuration_arn().to_string())
        .expect("should have config");

    let resp = client
        .update_microsoft_teams_channel_configuration()
        .chat_configuration_arn(&arn)
        .channel_id("channel-updated")
        .iam_role_arn("arn:aws:iam::123456789012:role/new-role")
        .send()
        .await
        .expect("update should succeed");

    let cfg = resp.channel_configuration().expect("should have config");
    assert_eq!(cfg.channel_id(), "channel-updated");
    assert_eq!(
        cfg.iam_role_arn(),
        "arn:aws:iam::123456789012:role/new-role"
    );
}

#[tokio::test]
async fn test_list_tags_for_resource_returns_tags() {
    let client = make_client().await;

    let resp = client
        .create_microsoft_teams_channel_configuration()
        .configuration_name("tagged-teams")
        .team_id("team-tag")
        .tenant_id("tenant-tag")
        .channel_id("channel-tag")
        .iam_role_arn("arn:aws:iam::123456789012:role/chatbot-role")
        .tags(
            aws_sdk_chatbot::types::Tag::builder()
                .tag_key("Env")
                .tag_value("test")
                .build()
                .unwrap(),
        )
        .tags(
            aws_sdk_chatbot::types::Tag::builder()
                .tag_key("Project")
                .tag_value("winterbaume")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create should succeed");

    let arn = resp
        .channel_configuration()
        .map(|c| c.chat_configuration_arn().to_string())
        .expect("should have config");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags();
    assert_eq!(tags.len(), 2);
    let tag_map: std::collections::HashMap<&str, &str> =
        tags.iter().map(|t| (t.tag_key(), t.tag_value())).collect();
    assert_eq!(tag_map.get("Env"), Some(&"test"));
    assert_eq!(tag_map.get("Project"), Some(&"winterbaume"));
}

#[tokio::test]
async fn test_tag_resource_and_untag_resource() {
    let client = make_client().await;

    let resp = client
        .create_slack_channel_configuration()
        .configuration_name("tag-slack")
        .slack_team_id("T-tag")
        .slack_channel_id("C-tag")
        .iam_role_arn("arn:aws:iam::123456789012:role/chatbot-role")
        .tags(
            aws_sdk_chatbot::types::Tag::builder()
                .tag_key("Initial")
                .tag_value("yes")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create should succeed");

    let arn = resp
        .channel_configuration()
        .map(|c| c.chat_configuration_arn().to_string())
        .expect("should have config");

    // Tag the resource with additional tags
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(
            aws_sdk_chatbot::types::Tag::builder()
                .tag_key("Added")
                .tag_value("later")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    // Verify both tags exist
    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags();
    assert_eq!(tags.len(), 2);
    let tag_map: std::collections::HashMap<&str, &str> =
        tags.iter().map(|t| (t.tag_key(), t.tag_value())).collect();
    assert_eq!(tag_map.get("Initial"), Some(&"yes"));
    assert_eq!(tag_map.get("Added"), Some(&"later"));

    // Untag the "Initial" key
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("Initial")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify only "Added" remains
    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].tag_key(), "Added");
    assert_eq!(tags[0].tag_value(), "later");
}

// ---- State view tests ----

#[tokio::test]
async fn test_state_view_snapshot_restore() {
    use std::collections::HashMap;

    use winterbaume_chatbot::views::{ChatbotStateView, SlackConfigView};
    use winterbaume_core::StatefulService;

    let svc = ChatbotService::new();

    let mut slack_configs = HashMap::new();
    slack_configs.insert(
        "arn:aws:chatbot:us-east-1:123456789012:chat-configuration/slack-channel/test".to_string(),
        SlackConfigView {
            arn: "arn:aws:chatbot:us-east-1:123456789012:chat-configuration/slack-channel/test"
                .to_string(),
            configuration_name: "test".to_string(),
            slack_team_id: "T1".to_string(),
            slack_channel_id: "C1".to_string(),
            slack_channel_name: None,
            iam_role_arn: "arn:aws:iam::123456789012:role/test".to_string(),
            sns_topic_arns: vec![],
            logging_level: None,
            guardrail_policy_arns: vec![],
            user_authorization_required: None,
            tags: HashMap::new(),
        },
    );
    svc.restore(
        "123456789012",
        "us-east-1",
        ChatbotStateView {
            slack_configs,
            ..Default::default()
        },
    )
    .await
    .expect("restore should succeed");

    let view = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(view.slack_configs.len(), 1);

    let svc2 = ChatbotService::new();
    svc2.restore("123456789012", "us-east-1", view)
        .await
        .expect("restore should succeed");
    let view2 = svc2.snapshot("123456789012", "us-east-1").await;
    assert_eq!(view2.slack_configs.len(), 1);
}

// ---- State change listener tests ----

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::sync::{Arc, Mutex};

    use winterbaume_core::StatefulService;

    let svc = ChatbotService::new();
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
        .unwrap();

    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[0],
        ("123456789012".to_string(), "us-east-1".to_string())
    );
}
