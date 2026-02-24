use aws_sdk_workspacesweb::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_workspacesweb::WorkspacesWebService;

async fn make_workspacesweb_client() -> aws_sdk_workspacesweb::Client {
    let mock = MockAws::builder()
        .with_service(WorkspacesWebService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_workspacesweb::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_workspacesweb::Client::new(&config)
}

// ── Portal tests ────────────────────────────────────────────────────

#[tokio::test]
async fn test_create_portal() {
    let client = make_workspacesweb_client().await;

    let resp = client
        .create_portal()
        .display_name("Test Portal")
        .send()
        .await
        .expect("create_portal should succeed");

    assert!(!resp.portal_arn().is_empty());
    assert!(!resp.portal_endpoint().is_empty());
}

#[tokio::test]
async fn test_create_and_get_portal() {
    let client = make_workspacesweb_client().await;

    let create_resp = client
        .create_portal()
        .display_name("My Portal")
        .send()
        .await
        .expect("create_portal should succeed");

    let portal_arn = create_resp.portal_arn();

    let get_resp = client
        .get_portal()
        .portal_arn(portal_arn)
        .send()
        .await
        .expect("get_portal should succeed");

    let portal = get_resp.portal().expect("should have portal");
    assert_eq!(portal.display_name().unwrap_or_default(), "My Portal");
}

#[tokio::test]
async fn test_delete_portal() {
    let client = make_workspacesweb_client().await;

    let create_resp = client
        .create_portal()
        .display_name("Delete Me")
        .send()
        .await
        .expect("create_portal should succeed");

    let portal_arn = create_resp.portal_arn().to_string();

    client
        .delete_portal()
        .portal_arn(&portal_arn)
        .send()
        .await
        .expect("delete_portal should succeed");

    let result = client.get_portal().portal_arn(&portal_arn).send().await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_list_portals() {
    let client = make_workspacesweb_client().await;

    client
        .create_portal()
        .display_name("Portal A")
        .send()
        .await
        .unwrap();

    client
        .create_portal()
        .display_name("Portal B")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_portals()
        .send()
        .await
        .expect("list_portals should succeed");

    assert_eq!(resp.portals().len(), 2);
}

#[tokio::test]
async fn test_delete_nonexistent_portal_fails() {
    let client = make_workspacesweb_client().await;

    let result = client
        .delete_portal()
        .portal_arn("arn:aws:workspaces-web:us-east-1:123456789012:portal/nonexistent")
        .send()
        .await;
    assert!(result.is_err(), "deleting nonexistent portal should fail");
}

#[tokio::test]
async fn test_get_nonexistent_portal_fails() {
    let client = make_workspacesweb_client().await;

    let result = client
        .get_portal()
        .portal_arn("arn:aws:workspaces-web:us-east-1:123456789012:portal/nonexistent")
        .send()
        .await;
    assert!(result.is_err(), "getting nonexistent portal should fail");
}

#[tokio::test]
async fn test_create_portal_without_display_name() {
    let client = make_workspacesweb_client().await;

    let resp = client
        .create_portal()
        .send()
        .await
        .expect("create_portal without display name should succeed");

    assert!(!resp.portal_arn().is_empty());
}

// ── BrowserSettings tests ───────────────────────────────────────────

#[tokio::test]
async fn test_browser_settings_lifecycle() {
    let client = make_workspacesweb_client().await;

    // Create
    let create_resp = client
        .create_browser_settings()
        .browser_policy("{\"version\":\"1.0\"}")
        .send()
        .await
        .expect("create_browser_settings should succeed");

    let arn = create_resp.browser_settings_arn().to_string();
    assert!(!arn.is_empty());

    // Get
    let get_resp = client
        .get_browser_settings()
        .browser_settings_arn(&arn)
        .send()
        .await
        .expect("get_browser_settings should succeed");

    let bs = get_resp
        .browser_settings()
        .expect("should have browser settings");
    assert_eq!(bs.browser_settings_arn(), &arn);
    assert_eq!(
        bs.browser_policy().unwrap_or_default(),
        "{\"version\":\"1.0\"}"
    );

    // List
    let list_resp = client
        .list_browser_settings()
        .send()
        .await
        .expect("list_browser_settings should succeed");
    assert_eq!(list_resp.browser_settings().len(), 1);

    // Delete
    client
        .delete_browser_settings()
        .browser_settings_arn(&arn)
        .send()
        .await
        .expect("delete_browser_settings should succeed");

    // Verify gone
    let result = client
        .get_browser_settings()
        .browser_settings_arn(&arn)
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_delete_nonexistent_browser_settings_fails() {
    let client = make_workspacesweb_client().await;

    let result = client
        .delete_browser_settings()
        .browser_settings_arn(
            "arn:aws:workspaces-web:us-east-1:123456789012:browserSettings/nonexistent",
        )
        .send()
        .await;
    assert!(result.is_err());
}

// ── NetworkSettings tests ───────────────────────────────────────────

#[tokio::test]
async fn test_network_settings_lifecycle() {
    let client = make_workspacesweb_client().await;

    // Create
    let create_resp = client
        .create_network_settings()
        .vpc_id("vpc-12345")
        .subnet_ids("subnet-aaa")
        .subnet_ids("subnet-bbb")
        .security_group_ids("sg-111")
        .send()
        .await
        .expect("create_network_settings should succeed");

    let arn = create_resp.network_settings_arn().to_string();
    assert!(!arn.is_empty());

    // Get
    let get_resp = client
        .get_network_settings()
        .network_settings_arn(&arn)
        .send()
        .await
        .expect("get_network_settings should succeed");

    let ns = get_resp
        .network_settings()
        .expect("should have network settings");
    assert_eq!(ns.vpc_id().unwrap_or_default(), "vpc-12345");
    assert_eq!(ns.subnet_ids().len(), 2);
    assert_eq!(ns.security_group_ids().len(), 1);

    // List
    let list_resp = client
        .list_network_settings()
        .send()
        .await
        .expect("list_network_settings should succeed");
    assert_eq!(list_resp.network_settings().len(), 1);

    // Delete
    client
        .delete_network_settings()
        .network_settings_arn(&arn)
        .send()
        .await
        .expect("delete_network_settings should succeed");

    // Verify gone
    let result = client
        .get_network_settings()
        .network_settings_arn(&arn)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_network_settings_fails() {
    let client = make_workspacesweb_client().await;

    let result = client
        .delete_network_settings()
        .network_settings_arn(
            "arn:aws:workspaces-web:us-east-1:123456789012:networkSettings/nonexistent",
        )
        .send()
        .await;
    assert!(result.is_err());
}

// ── UserAccessLoggingSettings tests ─────────────────────────────────

#[tokio::test]
async fn test_user_access_logging_settings_lifecycle() {
    let client = make_workspacesweb_client().await;

    // Create
    let create_resp = client
        .create_user_access_logging_settings()
        .kinesis_stream_arn("arn:aws:kinesis:us-east-1:123456789012:stream/my-stream")
        .send()
        .await
        .expect("create_user_access_logging_settings should succeed");

    let arn = create_resp.user_access_logging_settings_arn().to_string();
    assert!(!arn.is_empty());

    // Get
    let get_resp = client
        .get_user_access_logging_settings()
        .user_access_logging_settings_arn(&arn)
        .send()
        .await
        .expect("get_user_access_logging_settings should succeed");

    let ual = get_resp
        .user_access_logging_settings()
        .expect("should have user access logging settings");
    assert_eq!(
        ual.kinesis_stream_arn().unwrap_or_default(),
        "arn:aws:kinesis:us-east-1:123456789012:stream/my-stream"
    );

    // List
    let list_resp = client
        .list_user_access_logging_settings()
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(list_resp.user_access_logging_settings().len(), 1);

    // Delete
    client
        .delete_user_access_logging_settings()
        .user_access_logging_settings_arn(&arn)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let result = client
        .get_user_access_logging_settings()
        .user_access_logging_settings_arn(&arn)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_user_access_logging_settings_fails() {
    let client = make_workspacesweb_client().await;

    let result = client
        .delete_user_access_logging_settings()
        .user_access_logging_settings_arn(
            "arn:aws:workspaces-web:us-east-1:123456789012:userAccessLoggingSettings/nonexistent",
        )
        .send()
        .await;
    assert!(result.is_err());
}

// ── UserSettings tests ──────────────────────────────────────────────

#[tokio::test]
async fn test_user_settings_lifecycle() {
    use aws_sdk_workspacesweb::types::EnabledType;

    let client = make_workspacesweb_client().await;

    // Create
    let create_resp = client
        .create_user_settings()
        .copy_allowed(EnabledType::Enabled)
        .paste_allowed(EnabledType::Enabled)
        .download_allowed(EnabledType::Disabled)
        .upload_allowed(EnabledType::Disabled)
        .print_allowed(EnabledType::Enabled)
        .disconnect_timeout_in_minutes(30)
        .send()
        .await
        .expect("create_user_settings should succeed");

    let arn = create_resp.user_settings_arn().to_string();
    assert!(!arn.is_empty());

    // Get
    let get_resp = client
        .get_user_settings()
        .user_settings_arn(&arn)
        .send()
        .await
        .expect("get_user_settings should succeed");

    let us = get_resp.user_settings().expect("should have user settings");
    assert_eq!(us.copy_allowed(), Some(&EnabledType::Enabled));
    assert_eq!(us.download_allowed(), Some(&EnabledType::Disabled));
    assert_eq!(us.disconnect_timeout_in_minutes(), Some(30));

    // List
    let list_resp = client
        .list_user_settings()
        .send()
        .await
        .expect("list should succeed");
    assert_eq!(list_resp.user_settings().len(), 1);

    // Delete
    client
        .delete_user_settings()
        .user_settings_arn(&arn)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let result = client
        .get_user_settings()
        .user_settings_arn(&arn)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_user_settings_fails() {
    let client = make_workspacesweb_client().await;

    let result = client
        .delete_user_settings()
        .user_settings_arn("arn:aws:workspaces-web:us-east-1:123456789012:userSettings/nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// ── Association tests ───────────────────────────────────────────────

#[tokio::test]
async fn test_associate_browser_settings() {
    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("Assoc Portal")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    let bs = client.create_browser_settings().send().await.unwrap();
    let bs_arn = bs.browser_settings_arn().to_string();

    let resp = client
        .associate_browser_settings()
        .portal_arn(&portal_arn)
        .browser_settings_arn(&bs_arn)
        .send()
        .await
        .expect("associate should succeed");

    assert_eq!(resp.portal_arn(), &portal_arn);
    assert_eq!(resp.browser_settings_arn(), &bs_arn);

    // Verify the portal now has the association
    let get_resp = client
        .get_portal()
        .portal_arn(&portal_arn)
        .send()
        .await
        .unwrap();
    let portal_detail = get_resp.portal().unwrap();
    assert_eq!(portal_detail.browser_settings_arn().unwrap(), &bs_arn);
}

#[tokio::test]
async fn test_associate_network_settings() {
    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("NS Portal")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    let ns = client
        .create_network_settings()
        .vpc_id("vpc-test")
        .subnet_ids("subnet-1")
        .security_group_ids("sg-1")
        .send()
        .await
        .unwrap();
    let ns_arn = ns.network_settings_arn().to_string();

    let resp = client
        .associate_network_settings()
        .portal_arn(&portal_arn)
        .network_settings_arn(&ns_arn)
        .send()
        .await
        .expect("associate should succeed");

    assert_eq!(resp.portal_arn(), &portal_arn);
    assert_eq!(resp.network_settings_arn(), &ns_arn);
}

#[tokio::test]
async fn test_associate_user_access_logging_settings() {
    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("UAL Portal")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    let ual = client
        .create_user_access_logging_settings()
        .kinesis_stream_arn("arn:aws:kinesis:us-east-1:123456789012:stream/test")
        .send()
        .await
        .unwrap();
    let ual_arn = ual.user_access_logging_settings_arn().to_string();

    let resp = client
        .associate_user_access_logging_settings()
        .portal_arn(&portal_arn)
        .user_access_logging_settings_arn(&ual_arn)
        .send()
        .await
        .expect("associate should succeed");

    assert_eq!(resp.portal_arn(), &portal_arn);
    assert_eq!(resp.user_access_logging_settings_arn(), &ual_arn);
}

#[tokio::test]
async fn test_associate_user_settings() {
    use aws_sdk_workspacesweb::types::EnabledType;

    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("US Portal")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    let us = client
        .create_user_settings()
        .copy_allowed(EnabledType::Enabled)
        .paste_allowed(EnabledType::Enabled)
        .download_allowed(EnabledType::Enabled)
        .upload_allowed(EnabledType::Enabled)
        .print_allowed(EnabledType::Enabled)
        .send()
        .await
        .unwrap();
    let us_arn = us.user_settings_arn().to_string();

    let resp = client
        .associate_user_settings()
        .portal_arn(&portal_arn)
        .user_settings_arn(&us_arn)
        .send()
        .await
        .expect("associate should succeed");

    assert_eq!(resp.portal_arn(), &portal_arn);
    assert_eq!(resp.user_settings_arn(), &us_arn);
}

// ── Tag tests ───────────────────────────────────────────────────────

#[tokio::test]
async fn test_tag_and_list_tags() {
    use aws_sdk_workspacesweb::types::Tag;

    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("Tagged Portal")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    // Tag the resource
    client
        .tag_resource()
        .resource_arn(&portal_arn)
        .tags(Tag::builder().key("env").value("prod").build().unwrap())
        .tags(
            Tag::builder()
                .key("team")
                .value("platform")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&portal_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags();
    assert_eq!(tags.len(), 2);
}

#[tokio::test]
async fn test_untag_resource() {
    use aws_sdk_workspacesweb::types::Tag;

    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("Untag Portal")
        .tags(Tag::builder().key("env").value("staging").build().unwrap())
        .tags(Tag::builder().key("team").value("eng").build().unwrap())
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    // Untag
    client
        .untag_resource()
        .resource_arn(&portal_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&portal_arn)
        .send()
        .await
        .unwrap();

    let tags = list_resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "team");
}

// ── Update tests ────────────────────────────────────────────────────

#[tokio::test]
async fn test_update_portal() {
    let client = make_workspacesweb_client().await;

    let create_resp = client
        .create_portal()
        .display_name("Before Update")
        .send()
        .await
        .unwrap();
    let portal_arn = create_resp.portal_arn().to_string();

    let update_resp = client
        .update_portal()
        .portal_arn(&portal_arn)
        .display_name("After Update")
        .send()
        .await
        .expect("update_portal should succeed");

    let portal = update_resp.portal().expect("should have portal");
    assert_eq!(portal.display_name().unwrap_or_default(), "After Update");
}

#[tokio::test]
async fn test_update_browser_settings() {
    let client = make_workspacesweb_client().await;

    let create_resp = client
        .create_browser_settings()
        .browser_policy("{\"version\":\"1.0\"}")
        .send()
        .await
        .unwrap();
    let arn = create_resp.browser_settings_arn().to_string();

    let update_resp = client
        .update_browser_settings()
        .browser_settings_arn(&arn)
        .browser_policy("{\"version\":\"2.0\"}")
        .send()
        .await
        .expect("update_browser_settings should succeed");

    let bs = update_resp
        .browser_settings()
        .expect("should have browser settings");
    assert_eq!(
        bs.browser_policy().unwrap_or_default(),
        "{\"version\":\"2.0\"}"
    );
}

#[tokio::test]
async fn test_update_network_settings() {
    let client = make_workspacesweb_client().await;

    let create_resp = client
        .create_network_settings()
        .vpc_id("vpc-old")
        .subnet_ids("subnet-1")
        .security_group_ids("sg-1")
        .send()
        .await
        .unwrap();
    let arn = create_resp.network_settings_arn().to_string();

    let update_resp = client
        .update_network_settings()
        .network_settings_arn(&arn)
        .vpc_id("vpc-new")
        .subnet_ids("subnet-2")
        .subnet_ids("subnet-3")
        .security_group_ids("sg-2")
        .send()
        .await
        .expect("update_network_settings should succeed");

    let ns = update_resp
        .network_settings()
        .expect("should have network settings");
    assert_eq!(ns.vpc_id().unwrap_or_default(), "vpc-new");
    assert_eq!(ns.subnet_ids().len(), 2);
    assert_eq!(ns.security_group_ids().len(), 1);
}

#[tokio::test]
async fn test_update_user_access_logging_settings() {
    let client = make_workspacesweb_client().await;

    let create_resp = client
        .create_user_access_logging_settings()
        .kinesis_stream_arn("arn:aws:kinesis:us-east-1:123456789012:stream/old-stream")
        .send()
        .await
        .unwrap();
    let arn = create_resp.user_access_logging_settings_arn().to_string();

    let update_resp = client
        .update_user_access_logging_settings()
        .user_access_logging_settings_arn(&arn)
        .kinesis_stream_arn("arn:aws:kinesis:us-east-1:123456789012:stream/new-stream")
        .send()
        .await
        .expect("update should succeed");

    let ual = update_resp
        .user_access_logging_settings()
        .expect("should have settings");
    assert_eq!(
        ual.kinesis_stream_arn().unwrap_or_default(),
        "arn:aws:kinesis:us-east-1:123456789012:stream/new-stream"
    );
}

#[tokio::test]
async fn test_update_user_settings() {
    use aws_sdk_workspacesweb::types::EnabledType;

    let client = make_workspacesweb_client().await;

    let create_resp = client
        .create_user_settings()
        .copy_allowed(EnabledType::Disabled)
        .paste_allowed(EnabledType::Disabled)
        .download_allowed(EnabledType::Disabled)
        .upload_allowed(EnabledType::Disabled)
        .print_allowed(EnabledType::Disabled)
        .send()
        .await
        .unwrap();
    let arn = create_resp.user_settings_arn().to_string();

    let update_resp = client
        .update_user_settings()
        .user_settings_arn(&arn)
        .copy_allowed(EnabledType::Enabled)
        .paste_allowed(EnabledType::Enabled)
        .disconnect_timeout_in_minutes(60)
        .send()
        .await
        .expect("update should succeed");

    let us = update_resp.user_settings().expect("should have settings");
    assert_eq!(us.copy_allowed(), Some(&EnabledType::Enabled));
    assert_eq!(us.paste_allowed(), Some(&EnabledType::Enabled));
    assert_eq!(us.disconnect_timeout_in_minutes(), Some(60));
}

// ── Disassociate tests ──────────────────────────────────────────────

#[tokio::test]
async fn test_disassociate_browser_settings() {
    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("Disassoc BS Portal")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    let bs = client.create_browser_settings().send().await.unwrap();
    let bs_arn = bs.browser_settings_arn().to_string();

    client
        .associate_browser_settings()
        .portal_arn(&portal_arn)
        .browser_settings_arn(&bs_arn)
        .send()
        .await
        .unwrap();

    client
        .disassociate_browser_settings()
        .portal_arn(&portal_arn)
        .send()
        .await
        .expect("disassociate should succeed");

    let get_resp = client
        .get_portal()
        .portal_arn(&portal_arn)
        .send()
        .await
        .unwrap();
    let portal_detail = get_resp.portal().unwrap();
    assert!(
        portal_detail.browser_settings_arn().is_none(),
        "browser_settings_arn should be cleared"
    );
}

#[tokio::test]
async fn test_disassociate_network_settings() {
    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("Disassoc NS Portal")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    let ns = client
        .create_network_settings()
        .vpc_id("vpc-test")
        .subnet_ids("subnet-1")
        .security_group_ids("sg-1")
        .send()
        .await
        .unwrap();
    let ns_arn = ns.network_settings_arn().to_string();

    client
        .associate_network_settings()
        .portal_arn(&portal_arn)
        .network_settings_arn(&ns_arn)
        .send()
        .await
        .unwrap();

    client
        .disassociate_network_settings()
        .portal_arn(&portal_arn)
        .send()
        .await
        .expect("disassociate should succeed");

    let get_resp = client
        .get_portal()
        .portal_arn(&portal_arn)
        .send()
        .await
        .unwrap();
    let portal_detail = get_resp.portal().unwrap();
    assert!(
        portal_detail.network_settings_arn().is_none(),
        "network_settings_arn should be cleared"
    );
}

#[tokio::test]
async fn test_disassociate_user_access_logging_settings() {
    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("Disassoc UAL Portal")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    let ual = client
        .create_user_access_logging_settings()
        .kinesis_stream_arn("arn:aws:kinesis:us-east-1:123456789012:stream/test")
        .send()
        .await
        .unwrap();
    let ual_arn = ual.user_access_logging_settings_arn().to_string();

    client
        .associate_user_access_logging_settings()
        .portal_arn(&portal_arn)
        .user_access_logging_settings_arn(&ual_arn)
        .send()
        .await
        .unwrap();

    client
        .disassociate_user_access_logging_settings()
        .portal_arn(&portal_arn)
        .send()
        .await
        .expect("disassociate should succeed");

    let get_resp = client
        .get_portal()
        .portal_arn(&portal_arn)
        .send()
        .await
        .unwrap();
    let portal_detail = get_resp.portal().unwrap();
    assert!(
        portal_detail.user_access_logging_settings_arn().is_none(),
        "user_access_logging_settings_arn should be cleared"
    );
}

#[tokio::test]
async fn test_disassociate_user_settings() {
    use aws_sdk_workspacesweb::types::EnabledType;

    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("Disassoc US Portal")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    let us = client
        .create_user_settings()
        .copy_allowed(EnabledType::Enabled)
        .paste_allowed(EnabledType::Enabled)
        .download_allowed(EnabledType::Enabled)
        .upload_allowed(EnabledType::Enabled)
        .print_allowed(EnabledType::Enabled)
        .send()
        .await
        .unwrap();
    let us_arn = us.user_settings_arn().to_string();

    client
        .associate_user_settings()
        .portal_arn(&portal_arn)
        .user_settings_arn(&us_arn)
        .send()
        .await
        .unwrap();

    client
        .disassociate_user_settings()
        .portal_arn(&portal_arn)
        .send()
        .await
        .expect("disassociate should succeed");

    let get_resp = client
        .get_portal()
        .portal_arn(&portal_arn)
        .send()
        .await
        .unwrap();
    let portal_detail = get_resp.portal().unwrap();
    assert!(
        portal_detail.user_settings_arn().is_none(),
        "user_settings_arn should be cleared"
    );
}

// ── Identity Provider tests ─────────────────────────────────────────

#[tokio::test]
async fn test_identity_provider_lifecycle() {
    use aws_sdk_workspacesweb::types::IdentityProviderType;

    let client = make_workspacesweb_client().await;

    // First create a portal (required for identity providers)
    let portal = client
        .create_portal()
        .display_name("IdP Portal")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    // Create identity provider
    let create_resp = client
        .create_identity_provider()
        .portal_arn(&portal_arn)
        .identity_provider_name("TestIdP")
        .identity_provider_type(IdentityProviderType::Saml)
        .identity_provider_details("MetadataURL", "https://example.com/saml/metadata")
        .send()
        .await
        .expect("create_identity_provider should succeed");

    let idp_arn = create_resp.identity_provider_arn().to_string();
    assert!(!idp_arn.is_empty());

    // Get identity provider
    let get_resp = client
        .get_identity_provider()
        .identity_provider_arn(&idp_arn)
        .send()
        .await
        .expect("get_identity_provider should succeed");

    let idp = get_resp
        .identity_provider()
        .expect("should have identity provider");
    assert_eq!(idp.identity_provider_name().unwrap_or_default(), "TestIdP");

    // List identity providers for portal
    let list_resp = client
        .list_identity_providers()
        .portal_arn(&portal_arn)
        .send()
        .await
        .expect("list_identity_providers should succeed");
    assert_eq!(list_resp.identity_providers().len(), 1);

    // Update identity provider
    let update_resp = client
        .update_identity_provider()
        .identity_provider_arn(&idp_arn)
        .identity_provider_name("UpdatedIdP")
        .send()
        .await
        .expect("update_identity_provider should succeed");

    let updated_idp = update_resp
        .identity_provider()
        .expect("should have identity provider");
    assert_eq!(
        updated_idp.identity_provider_name().unwrap_or_default(),
        "UpdatedIdP"
    );

    // Delete identity provider
    client
        .delete_identity_provider()
        .identity_provider_arn(&idp_arn)
        .send()
        .await
        .expect("delete_identity_provider should succeed");

    // Verify gone
    let result = client
        .get_identity_provider()
        .identity_provider_arn(&idp_arn)
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

// ── IP Access Settings tests ────────────────────────────────────────

#[tokio::test]
async fn test_ip_access_settings_lifecycle() {
    use aws_sdk_workspacesweb::types::IpRule;

    let client = make_workspacesweb_client().await;

    // Create
    let create_resp = client
        .create_ip_access_settings()
        .display_name("Office IPs")
        .description("Corporate office IP ranges")
        .ip_rules(
            IpRule::builder()
                .ip_range("10.0.0.0/8")
                .description("Private network")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_ip_access_settings should succeed");

    let arn = create_resp.ip_access_settings_arn().to_string();
    assert!(!arn.is_empty());

    // Get
    let get_resp = client
        .get_ip_access_settings()
        .ip_access_settings_arn(&arn)
        .send()
        .await
        .expect("get_ip_access_settings should succeed");

    let ias = get_resp
        .ip_access_settings()
        .expect("should have ip access settings");
    assert_eq!(ias.display_name().unwrap_or_default(), "Office IPs");

    // List
    let list_resp = client
        .list_ip_access_settings()
        .send()
        .await
        .expect("list_ip_access_settings should succeed");
    assert_eq!(list_resp.ip_access_settings().len(), 1);

    // Update
    let update_resp = client
        .update_ip_access_settings()
        .ip_access_settings_arn(&arn)
        .display_name("Updated IPs")
        .ip_rules(
            IpRule::builder()
                .ip_range("192.168.0.0/16")
                .description("New range")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update_ip_access_settings should succeed");

    let updated_ias = update_resp
        .ip_access_settings()
        .expect("should have ip access settings");
    assert_eq!(
        updated_ias.display_name().unwrap_or_default(),
        "Updated IPs"
    );

    // Delete
    client
        .delete_ip_access_settings()
        .ip_access_settings_arn(&arn)
        .send()
        .await
        .expect("delete_ip_access_settings should succeed");

    // Verify gone
    let result = client
        .get_ip_access_settings()
        .ip_access_settings_arn(&arn)
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_associate_ip_access_settings() {
    use aws_sdk_workspacesweb::types::IpRule;

    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("IpAS Portal")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    let ias = client
        .create_ip_access_settings()
        .ip_rules(IpRule::builder().ip_range("10.0.0.0/8").build().unwrap())
        .send()
        .await
        .unwrap();
    let ias_arn = ias.ip_access_settings_arn().to_string();

    let resp = client
        .associate_ip_access_settings()
        .portal_arn(&portal_arn)
        .ip_access_settings_arn(&ias_arn)
        .send()
        .await
        .expect("associate should succeed");

    assert_eq!(resp.portal_arn(), &portal_arn);
    assert_eq!(resp.ip_access_settings_arn(), &ias_arn);

    // Verify via get portal
    let get_resp = client
        .get_portal()
        .portal_arn(&portal_arn)
        .send()
        .await
        .unwrap();
    let portal_detail = get_resp.portal().unwrap();
    assert_eq!(portal_detail.ip_access_settings_arn().unwrap(), &ias_arn);
}

#[tokio::test]
async fn test_disassociate_ip_access_settings() {
    use aws_sdk_workspacesweb::types::IpRule;

    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("Disassoc IpAS Portal")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    let ias = client
        .create_ip_access_settings()
        .ip_rules(IpRule::builder().ip_range("10.0.0.0/8").build().unwrap())
        .send()
        .await
        .unwrap();
    let ias_arn = ias.ip_access_settings_arn().to_string();

    client
        .associate_ip_access_settings()
        .portal_arn(&portal_arn)
        .ip_access_settings_arn(&ias_arn)
        .send()
        .await
        .unwrap();

    client
        .disassociate_ip_access_settings()
        .portal_arn(&portal_arn)
        .send()
        .await
        .expect("disassociate should succeed");

    let get_resp = client
        .get_portal()
        .portal_arn(&portal_arn)
        .send()
        .await
        .unwrap();
    let portal_detail = get_resp.portal().unwrap();
    assert!(
        portal_detail.ip_access_settings_arn().is_none(),
        "ip_access_settings_arn should be cleared"
    );
}

// ── Trust Store tests ───────────────────────────────────────────────

#[tokio::test]
async fn test_trust_store_lifecycle() {
    let client = make_workspacesweb_client().await;

    // Create
    let create_resp = client
        .create_trust_store()
        .certificate_list(aws_sdk_workspacesweb::primitives::Blob::new(
            b"fake-cert-data-1".to_vec(),
        ))
        .send()
        .await
        .expect("create_trust_store should succeed");

    let arn = create_resp.trust_store_arn().to_string();
    assert!(!arn.is_empty());

    // Get
    let get_resp = client
        .get_trust_store()
        .trust_store_arn(&arn)
        .send()
        .await
        .expect("get_trust_store should succeed");

    let ts = get_resp.trust_store().expect("should have trust store");
    assert_eq!(ts.trust_store_arn(), &arn);

    // List
    let list_resp = client
        .list_trust_stores()
        .send()
        .await
        .expect("list_trust_stores should succeed");
    assert_eq!(list_resp.trust_stores().len(), 1);

    // Delete
    client
        .delete_trust_store()
        .trust_store_arn(&arn)
        .send()
        .await
        .expect("delete_trust_store should succeed");

    // Verify gone
    let result = client.get_trust_store().trust_store_arn(&arn).send().await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_associate_trust_store() {
    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("TS Portal")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    let ts = client
        .create_trust_store()
        .certificate_list(aws_sdk_workspacesweb::primitives::Blob::new(
            b"cert-data".to_vec(),
        ))
        .send()
        .await
        .unwrap();
    let ts_arn = ts.trust_store_arn().to_string();

    let resp = client
        .associate_trust_store()
        .portal_arn(&portal_arn)
        .trust_store_arn(&ts_arn)
        .send()
        .await
        .expect("associate should succeed");

    assert_eq!(resp.portal_arn(), &portal_arn);
    assert_eq!(resp.trust_store_arn(), &ts_arn);
}

#[tokio::test]
async fn test_disassociate_trust_store() {
    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("Disassoc TS Portal")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    let ts = client
        .create_trust_store()
        .certificate_list(aws_sdk_workspacesweb::primitives::Blob::new(
            b"cert-data".to_vec(),
        ))
        .send()
        .await
        .unwrap();
    let ts_arn = ts.trust_store_arn().to_string();

    client
        .associate_trust_store()
        .portal_arn(&portal_arn)
        .trust_store_arn(&ts_arn)
        .send()
        .await
        .unwrap();

    client
        .disassociate_trust_store()
        .portal_arn(&portal_arn)
        .send()
        .await
        .expect("disassociate should succeed");

    let get_resp = client
        .get_portal()
        .portal_arn(&portal_arn)
        .send()
        .await
        .unwrap();
    let portal_detail = get_resp.portal().unwrap();
    assert!(
        portal_detail.trust_store_arn().is_none(),
        "trust_store_arn should be cleared"
    );
}

// ── Data Protection Settings tests ──────────────────────────────────

#[tokio::test]
async fn test_data_protection_settings_lifecycle() {
    let client = make_workspacesweb_client().await;

    // Create
    let create_resp = client
        .create_data_protection_settings()
        .display_name("My DPS")
        .description("Test data protection settings")
        .send()
        .await
        .expect("create_data_protection_settings should succeed");

    let arn = create_resp.data_protection_settings_arn().to_string();
    assert!(!arn.is_empty());

    // Get
    let get_resp = client
        .get_data_protection_settings()
        .data_protection_settings_arn(&arn)
        .send()
        .await
        .expect("get_data_protection_settings should succeed");

    let dps = get_resp
        .data_protection_settings()
        .expect("should have data protection settings");
    assert_eq!(dps.display_name().unwrap_or_default(), "My DPS");
    assert_eq!(
        dps.description().unwrap_or_default(),
        "Test data protection settings"
    );

    // List
    let list_resp = client
        .list_data_protection_settings()
        .send()
        .await
        .expect("list_data_protection_settings should succeed");
    assert_eq!(list_resp.data_protection_settings().len(), 1);

    // Update
    let update_resp = client
        .update_data_protection_settings()
        .data_protection_settings_arn(&arn)
        .display_name("Updated DPS")
        .description("Updated description")
        .send()
        .await
        .expect("update_data_protection_settings should succeed");

    let updated_dps = update_resp
        .data_protection_settings()
        .expect("should have settings");
    assert_eq!(
        updated_dps.display_name().unwrap_or_default(),
        "Updated DPS"
    );

    // Delete
    client
        .delete_data_protection_settings()
        .data_protection_settings_arn(&arn)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let result = client
        .get_data_protection_settings()
        .data_protection_settings_arn(&arn)
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_associate_data_protection_settings() {
    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("DPS Portal")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    let dps = client
        .create_data_protection_settings()
        .display_name("Assoc DPS")
        .send()
        .await
        .unwrap();
    let dps_arn = dps.data_protection_settings_arn().to_string();

    let resp = client
        .associate_data_protection_settings()
        .portal_arn(&portal_arn)
        .data_protection_settings_arn(&dps_arn)
        .send()
        .await
        .expect("associate should succeed");

    assert_eq!(resp.portal_arn(), &portal_arn);
    assert_eq!(resp.data_protection_settings_arn(), &dps_arn);
}

#[tokio::test]
async fn test_disassociate_data_protection_settings() {
    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("Disassoc DPS Portal")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    let dps = client
        .create_data_protection_settings()
        .display_name("Disassoc DPS")
        .send()
        .await
        .unwrap();
    let dps_arn = dps.data_protection_settings_arn().to_string();

    client
        .associate_data_protection_settings()
        .portal_arn(&portal_arn)
        .data_protection_settings_arn(&dps_arn)
        .send()
        .await
        .unwrap();

    client
        .disassociate_data_protection_settings()
        .portal_arn(&portal_arn)
        .send()
        .await
        .expect("disassociate should succeed");

    let get_resp = client
        .get_portal()
        .portal_arn(&portal_arn)
        .send()
        .await
        .unwrap();
    let portal_detail = get_resp.portal().unwrap();
    assert!(
        portal_detail.data_protection_settings_arn().is_none(),
        "data_protection_settings_arn should be cleared"
    );
}

// ── Session Logger tests ────────────────────────────────────────────

#[tokio::test]
async fn test_session_logger_lifecycle() {
    let client = make_workspacesweb_client().await;

    // Create
    let create_resp = client
        .create_session_logger()
        .display_name("My Logger")
        .send()
        .await
        .expect("create_session_logger should succeed");

    let arn = create_resp.session_logger_arn().to_string();
    assert!(!arn.is_empty());

    // Get
    let get_resp = client
        .get_session_logger()
        .session_logger_arn(&arn)
        .send()
        .await
        .expect("get_session_logger should succeed");

    let sl = get_resp
        .session_logger()
        .expect("should have session logger");
    assert_eq!(sl.display_name().unwrap_or_default(), "My Logger");

    // List
    let list_resp = client
        .list_session_loggers()
        .send()
        .await
        .expect("list_session_loggers should succeed");
    assert_eq!(list_resp.session_loggers().len(), 1);

    // Update
    let update_resp = client
        .update_session_logger()
        .session_logger_arn(&arn)
        .display_name("Updated Logger")
        .send()
        .await
        .expect("update_session_logger should succeed");

    let updated_sl = update_resp
        .session_logger()
        .expect("should have session logger");
    assert_eq!(
        updated_sl.display_name().unwrap_or_default(),
        "Updated Logger"
    );

    // Delete
    client
        .delete_session_logger()
        .session_logger_arn(&arn)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let result = client
        .get_session_logger()
        .session_logger_arn(&arn)
        .send()
        .await;
    assert!(result.is_err(), "get after delete should fail");
}

#[tokio::test]
async fn test_associate_session_logger() {
    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("SL Portal")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    let sl = client
        .create_session_logger()
        .display_name("Assoc Logger")
        .send()
        .await
        .unwrap();
    let sl_arn = sl.session_logger_arn().to_string();

    let resp = client
        .associate_session_logger()
        .portal_arn(&portal_arn)
        .session_logger_arn(&sl_arn)
        .send()
        .await
        .expect("associate should succeed");

    assert_eq!(resp.portal_arn(), &portal_arn);
    assert_eq!(resp.session_logger_arn(), &sl_arn);
}

#[tokio::test]
async fn test_disassociate_session_logger() {
    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("Disassoc SL Portal")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    let sl = client
        .create_session_logger()
        .display_name("Disassoc Logger")
        .send()
        .await
        .unwrap();
    let sl_arn = sl.session_logger_arn().to_string();

    client
        .associate_session_logger()
        .portal_arn(&portal_arn)
        .session_logger_arn(&sl_arn)
        .send()
        .await
        .unwrap();

    client
        .disassociate_session_logger()
        .portal_arn(&portal_arn)
        .send()
        .await
        .expect("disassociate should succeed");

    let get_resp = client
        .get_portal()
        .portal_arn(&portal_arn)
        .send()
        .await
        .unwrap();
    let portal_detail = get_resp.portal().unwrap();
    assert!(
        portal_detail.session_logger_arn().is_none(),
        "session_logger_arn should be cleared"
    );
}

// ── GetPortalServiceProviderMetadata test ───────────────────────────

#[tokio::test]
async fn test_get_portal_service_provider_metadata() {
    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("SAML Portal")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();

    let resp = client
        .get_portal_service_provider_metadata()
        .portal_arn(&portal_arn)
        .send()
        .await
        .expect("get_portal_service_provider_metadata should succeed");

    assert_eq!(resp.portal_arn(), &portal_arn);
    let metadata = resp
        .service_provider_saml_metadata()
        .expect("should have metadata");
    assert!(
        metadata.contains("EntityDescriptor"),
        "should contain SAML metadata"
    );
}

/// Regression: GET /portals/{portalId+}/sessions has the portal ARN colons
/// percent-encoded by the SDK. The handler must URL-decode the segments
/// before using portalId as a state-map key, otherwise the lookup fails.
#[tokio::test]
async fn test_list_sessions_url_encoded_portal_arn() {
    let client = make_workspacesweb_client().await;

    let portal = client
        .create_portal()
        .display_name("Sessions ARN URL-encoded")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();
    assert!(portal_arn.contains(':'), "portal ARN must contain colons");

    // The SDK percent-encodes colons in the path param. The fact that this
    // returns Ok (not 404) demonstrates that the handler correctly
    // URL-decoded the path segments before looking up the portal.
    let resp = client
        .list_sessions()
        .portal_id(&portal_arn)
        .send()
        .await
        .expect("list_sessions should succeed for URL-encoded portal ARN");

    // No sessions yet; just assert the call routed and returned successfully.
    assert!(resp.sessions().is_empty());
}

/// Regression: ARN colons round-trip through TagResource and
/// ListTagsForResource even when the SDK URL-encodes them in the path.
#[tokio::test]
async fn test_tag_resource_url_encoded_arn_roundtrip() {
    use aws_sdk_workspacesweb::types::Tag;

    let client = make_workspacesweb_client().await;
    let portal = client
        .create_portal()
        .display_name("URL-encoded ARN tag")
        .send()
        .await
        .unwrap();
    let portal_arn = portal.portal_arn().to_string();
    assert!(portal_arn.contains(':'));

    client
        .tag_resource()
        .resource_arn(&portal_arn)
        .tags(Tag::builder().key("env").value("prod").build().unwrap())
        .send()
        .await
        .expect("tag_resource");

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&portal_arn)
        .send()
        .await
        .expect("list_tags_for_resource");
    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "env");
    assert_eq!(tags[0].value(), "prod");
}
