//! Scenario tests for winterbaume-supportapp.
//!
//! Each test covers a use-case workflow chaining three or more operations.

use aws_sdk_supportapp::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_supportapp::SupportAppService;

async fn make_client() -> aws_sdk_supportapp::Client {
    let mock = MockAws::builder()
        .with_service(SupportAppService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_supportapp::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_supportapp::Client::new(&config)
}

/// Scenario: full Slack workspace and channel lifecycle.
///
/// 1. Register a workspace for the organisation.
/// 2. Create a channel configuration inside that workspace.
/// 3. Update the channel configuration.
/// 4. Verify the updated configuration is returned by list.
/// 5. Delete the channel, then delete the workspace.
/// 6. Assert neither channel nor workspace appears in subsequent list calls.
#[tokio::test]
async fn test_workspace_channel_full_lifecycle() {
    let client = make_client().await;

    // Step 1: register workspace.
    let reg = client
        .register_slack_workspace_for_organization()
        .team_id("T-SCENE-1")
        .send()
        .await
        .expect("register workspace");
    assert_eq!(reg.team_id(), Some("T-SCENE-1"));

    // Step 2: create channel.
    client
        .create_slack_channel_configuration()
        .team_id("T-SCENE-1")
        .channel_id("C-SCENE-1")
        .channel_role_arn("arn:aws:iam::123456789012:role/SupportRole")
        .notify_on_case_severity("all".into())
        .notify_on_create_or_reopen_case(true)
        .send()
        .await
        .expect("create channel");

    // Step 3: update channel — add resolve notification and a display name.
    let updated = client
        .update_slack_channel_configuration()
        .team_id("T-SCENE-1")
        .channel_id("C-SCENE-1")
        .channel_name("support-ops")
        .notify_on_resolve_case(true)
        .send()
        .await
        .expect("update channel");
    assert_eq!(updated.channel_name(), Some("support-ops"));
    assert_eq!(updated.notify_on_resolve_case(), Some(true));

    // Step 4: list channels and verify updated config.
    let channels = client
        .list_slack_channel_configurations()
        .send()
        .await
        .expect("list channels");
    assert_eq!(channels.slack_channel_configurations().len(), 1);
    let ch = &channels.slack_channel_configurations()[0];
    assert_eq!(ch.channel_id(), "C-SCENE-1");
    assert_eq!(ch.notify_on_resolve_case(), Some(true));

    // Step 5: delete channel then workspace.
    client
        .delete_slack_channel_configuration()
        .team_id("T-SCENE-1")
        .channel_id("C-SCENE-1")
        .send()
        .await
        .expect("delete channel");

    client
        .delete_slack_workspace_configuration()
        .team_id("T-SCENE-1")
        .send()
        .await
        .expect("delete workspace");

    // Step 6: verify both are gone.
    let channels = client
        .list_slack_channel_configurations()
        .send()
        .await
        .expect("list after delete");
    assert_eq!(channels.slack_channel_configurations().len(), 0);

    let workspaces = client
        .list_slack_workspace_configurations()
        .send()
        .await
        .expect("list workspaces after delete");
    assert_eq!(workspaces.slack_workspace_configurations().len(), 0);
}

/// Scenario: workspace cascade delete removes all member channels.
///
/// 1. Register a workspace.
/// 2. Create two channel configurations.
/// 3. Delete the workspace — both channels should be removed via cascade.
/// 4. Assert no channels remain.
#[tokio::test]
async fn test_workspace_cascade_delete_removes_channels() {
    let client = make_client().await;

    client
        .register_slack_workspace_for_organization()
        .team_id("T-CASCADE")
        .send()
        .await
        .expect("register");

    for ch_id in ["C-A", "C-B"] {
        client
            .create_slack_channel_configuration()
            .team_id("T-CASCADE")
            .channel_id(ch_id)
            .channel_role_arn("arn:aws:iam::123456789012:role/SupportRole")
            .notify_on_case_severity("none".into())
            .send()
            .await
            .expect("create");
    }

    // Confirm both channels exist.
    let list = client
        .list_slack_channel_configurations()
        .send()
        .await
        .expect("list");
    assert_eq!(list.slack_channel_configurations().len(), 2);

    // Delete workspace — expect cascade.
    client
        .delete_slack_workspace_configuration()
        .team_id("T-CASCADE")
        .send()
        .await
        .expect("cascade delete");

    let list = client
        .list_slack_channel_configurations()
        .send()
        .await
        .expect("list after cascade");
    assert_eq!(list.slack_channel_configurations().len(), 0);
}

/// Scenario: account alias lifecycle.
///
/// 1. Confirm no alias is set initially.
/// 2. Set an alias.
/// 3. Overwrite with a new alias.
/// 4. Delete the alias.
/// 5. Confirm alias is gone.
#[tokio::test]
async fn test_account_alias_full_lifecycle() {
    let client = make_client().await;

    // Step 1: initially no alias.
    let resp = client
        .get_account_alias()
        .send()
        .await
        .expect("get initial");
    assert_eq!(resp.account_alias(), None);

    // Step 2: set alias.
    client
        .put_account_alias()
        .account_alias("acme-corp")
        .send()
        .await
        .expect("put");

    let resp = client.get_account_alias().send().await.expect("get set");
    assert_eq!(resp.account_alias(), Some("acme-corp"));

    // Step 3: overwrite.
    client
        .put_account_alias()
        .account_alias("acme-corp-v2")
        .send()
        .await
        .expect("overwrite");

    let resp = client
        .get_account_alias()
        .send()
        .await
        .expect("get overwrite");
    assert_eq!(resp.account_alias(), Some("acme-corp-v2"));

    // Step 4: delete.
    client.delete_account_alias().send().await.expect("delete");

    // Step 5: confirm gone.
    let resp = client
        .get_account_alias()
        .send()
        .await
        .expect("get after delete");
    assert_eq!(resp.account_alias(), None);
}
