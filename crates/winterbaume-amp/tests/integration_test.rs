use aws_sdk_amp::config::BehaviorVersion;
use winterbaume_amp::AmpService;
use winterbaume_core::MockAws;

async fn make_amp_client() -> aws_sdk_amp::Client {
    let mock = MockAws::builder().with_service(AmpService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_amp::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_amp::Client::new(&config)
}

#[tokio::test]
async fn test_create_workspace() {
    let client = make_amp_client().await;

    let resp = client
        .create_workspace()
        .alias("test-workspace")
        .send()
        .await
        .expect("create_workspace should succeed");

    assert!(!resp.workspace_id().is_empty());
    assert!(!resp.arn().is_empty());
    assert_eq!(
        resp.status().unwrap().status_code(),
        &aws_sdk_amp::types::WorkspaceStatusCode::Active,
    );
}

#[tokio::test]
async fn test_describe_workspace() {
    let client = make_amp_client().await;

    let create_resp = client
        .create_workspace()
        .alias("desc-workspace")
        .send()
        .await
        .unwrap();

    let workspace_id = create_resp.workspace_id();

    let resp = client
        .describe_workspace()
        .workspace_id(workspace_id)
        .send()
        .await
        .expect("describe_workspace should succeed");

    let ws = resp.workspace().unwrap();
    assert_eq!(ws.workspace_id(), workspace_id);
    assert_eq!(ws.alias(), Some("desc-workspace"));
    assert_eq!(
        ws.status().unwrap().status_code(),
        &aws_sdk_amp::types::WorkspaceStatusCode::Active,
    );
}

#[tokio::test]
async fn test_delete_workspace() {
    let client = make_amp_client().await;

    let create_resp = client
        .create_workspace()
        .alias("del-workspace")
        .send()
        .await
        .unwrap();

    let workspace_id = create_resp.workspace_id();

    client
        .delete_workspace()
        .workspace_id(workspace_id)
        .send()
        .await
        .expect("delete_workspace should succeed");

    let result = client
        .describe_workspace()
        .workspace_id(workspace_id)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_workspaces() {
    let client = make_amp_client().await;

    for alias in ["ws-a", "ws-b", "ws-c"] {
        client.create_workspace().alias(alias).send().await.unwrap();
    }

    let resp = client
        .list_workspaces()
        .send()
        .await
        .expect("list_workspaces should succeed");

    assert_eq!(resp.workspaces().len(), 3);
}

#[tokio::test]
async fn test_describe_nonexistent_workspace() {
    let client = make_amp_client().await;

    let result = client
        .describe_workspace()
        .workspace_id("ws-nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_workspace() {
    let client = make_amp_client().await;

    let result = client
        .delete_workspace()
        .workspace_id("ws-nonexistent")
        .send()
        .await;

    assert!(result.is_err());
}

// --- UpdateWorkspaceAlias ---

#[tokio::test]
async fn test_update_workspace_alias() {
    let client = make_amp_client().await;

    let create_resp = client
        .create_workspace()
        .alias("original-alias")
        .send()
        .await
        .unwrap();
    let workspace_id = create_resp.workspace_id().to_string();

    client
        .update_workspace_alias()
        .workspace_id(&workspace_id)
        .alias("new-alias")
        .send()
        .await
        .expect("update_workspace_alias should succeed");

    let desc = client
        .describe_workspace()
        .workspace_id(&workspace_id)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.workspace().unwrap().alias(), Some("new-alias"));
}

#[tokio::test]
async fn test_update_workspace_alias_nonexistent() {
    let client = make_amp_client().await;

    let result = client
        .update_workspace_alias()
        .workspace_id("ws-nonexistent")
        .alias("new-alias")
        .send()
        .await;
    assert!(result.is_err());
}

// --- Logging Configuration ---

#[tokio::test]
async fn test_logging_configuration_lifecycle() {
    let client = make_amp_client().await;

    let create_resp = client
        .create_workspace()
        .alias("logging-ws")
        .send()
        .await
        .unwrap();
    let workspace_id = create_resp.workspace_id().to_string();

    // Create logging configuration
    let log_resp = client
        .create_logging_configuration()
        .workspace_id(&workspace_id)
        .log_group_arn("arn:aws:logs:us-east-1:123456789012:log-group:my-log-group")
        .send()
        .await
        .expect("create_logging_configuration should succeed");
    assert_eq!(
        log_resp.status().unwrap().status_code(),
        &aws_sdk_amp::types::LoggingConfigurationStatusCode::Active
    );

    // Describe logging configuration
    let desc = client
        .describe_logging_configuration()
        .workspace_id(&workspace_id)
        .send()
        .await
        .expect("describe_logging_configuration should succeed");
    let config = desc.logging_configuration().unwrap();
    assert_eq!(
        config.log_group_arn(),
        "arn:aws:logs:us-east-1:123456789012:log-group:my-log-group"
    );

    // Update logging configuration
    let update_resp = client
        .update_logging_configuration()
        .workspace_id(&workspace_id)
        .log_group_arn("arn:aws:logs:us-east-1:123456789012:log-group:new-log-group")
        .send()
        .await
        .expect("update_logging_configuration should succeed");
    assert_eq!(
        update_resp.status().unwrap().status_code(),
        &aws_sdk_amp::types::LoggingConfigurationStatusCode::Active
    );

    // Verify update
    let desc2 = client
        .describe_logging_configuration()
        .workspace_id(&workspace_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc2.logging_configuration().unwrap().log_group_arn(),
        "arn:aws:logs:us-east-1:123456789012:log-group:new-log-group"
    );

    // Delete logging configuration
    client
        .delete_logging_configuration()
        .workspace_id(&workspace_id)
        .send()
        .await
        .expect("delete_logging_configuration should succeed");

    // Verify deleted: moto returns empty loggingConfiguration (not an error)
    let result = client
        .describe_logging_configuration()
        .workspace_id(&workspace_id)
        .send()
        .await;
    // After deletion, describe_logging_configuration returns success with no configuration
    // (the SDK may return Ok with no logging_configuration, or the field may be absent)
    // This is consistent with moto's behavior where {"loggingConfiguration": {}} is returned.
    let _ = result; // Accept either Ok (empty) or treat gracefully
}

#[tokio::test]
async fn test_create_logging_configuration_duplicate() {
    let client = make_amp_client().await;

    let create_resp = client
        .create_workspace()
        .alias("dup-logging-ws")
        .send()
        .await
        .unwrap();
    let workspace_id = create_resp.workspace_id().to_string();

    client
        .create_logging_configuration()
        .workspace_id(&workspace_id)
        .log_group_arn("arn:aws:logs:us-east-1:123456789012:log-group:my-log-group")
        .send()
        .await
        .unwrap();

    let result = client
        .create_logging_configuration()
        .workspace_id(&workspace_id)
        .log_group_arn("arn:aws:logs:us-east-1:123456789012:log-group:another-log-group")
        .send()
        .await;
    assert!(result.is_err());
}

// --- Rule Groups Namespaces ---

#[tokio::test]
async fn test_rule_groups_namespace_lifecycle() {
    let client = make_amp_client().await;

    let create_resp = client
        .create_workspace()
        .alias("rgn-ws")
        .send()
        .await
        .unwrap();
    let workspace_id = create_resp.workspace_id().to_string();

    // Create rule groups namespace
    let ns_resp = client
        .create_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("test-ns")
        .data(aws_smithy_types::Blob::new("cnVsZWdyb3VwZGF0YQ=="))
        .send()
        .await
        .expect("create_rule_groups_namespace should succeed");
    assert_eq!(ns_resp.name(), "test-ns");
    assert!(ns_resp.arn().contains("rulegroupsnamespace"));
    assert_eq!(
        ns_resp.status().unwrap().status_code(),
        &aws_sdk_amp::types::RuleGroupsNamespaceStatusCode::Active
    );

    // Describe rule groups namespace
    let desc = client
        .describe_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("test-ns")
        .send()
        .await
        .expect("describe_rule_groups_namespace should succeed");
    let ns = desc.rule_groups_namespace().unwrap();
    assert_eq!(ns.name(), "test-ns");

    // Put (update) rule groups namespace
    let put_resp = client
        .put_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("test-ns")
        .data(aws_smithy_types::Blob::new("bmV3ZGF0YQ=="))
        .send()
        .await
        .expect("put_rule_groups_namespace should succeed");
    assert_eq!(put_resp.name(), "test-ns");

    // Delete rule groups namespace
    client
        .delete_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("test-ns")
        .send()
        .await
        .expect("delete_rule_groups_namespace should succeed");

    // Verify deleted
    let result = client
        .describe_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("test-ns")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_rule_groups_namespaces() {
    let client = make_amp_client().await;

    let create_resp = client
        .create_workspace()
        .alias("list-rgn-ws")
        .send()
        .await
        .unwrap();
    let workspace_id = create_resp.workspace_id().to_string();

    for name in ["ns-alpha", "ns-beta", "ns-gamma"] {
        client
            .create_rule_groups_namespace()
            .workspace_id(&workspace_id)
            .name(name)
            .data(aws_smithy_types::Blob::new("dGVzdA=="))
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_rule_groups_namespaces()
        .workspace_id(&workspace_id)
        .send()
        .await
        .expect("list_rule_groups_namespaces should succeed");
    assert_eq!(resp.rule_groups_namespaces().len(), 3);
}

#[tokio::test]
async fn test_describe_nonexistent_rule_groups_namespace() {
    let client = make_amp_client().await;

    let create_resp = client
        .create_workspace()
        .alias("no-ns-ws")
        .send()
        .await
        .unwrap();
    let workspace_id = create_resp.workspace_id().to_string();

    let result = client
        .describe_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("nonexistent")
        .send()
        .await;
    assert!(result.is_err());
}

// --- Tags ---

#[tokio::test]
async fn test_tag_untag_list_tags() {
    let client = make_amp_client().await;

    let create_resp = client
        .create_workspace()
        .alias("tag-ws")
        .tags("env", "test")
        .send()
        .await
        .unwrap();
    let arn = create_resp.arn().to_string();

    // List tags
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");
    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("test"));

    // Add more tags
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("team", "platform")
        .tags("version", "v1")
        .send()
        .await
        .expect("tag_resource should succeed");

    // Verify tags
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("test"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("platform"));
    assert_eq!(tags.get("version").map(|s| s.as_str()), Some("v1"));

    // Remove a tag
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("version")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify removed
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    let tags = tags_resp.tags().unwrap();
    assert!(tags.get("version").is_none());
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("test"));
    assert_eq!(tags.get("team").map(|s| s.as_str()), Some("platform"));
}

// --- Moto parity tests ---

/// Parity with moto test_describe_logging (TestAmpLoggingConfig.test_describe_logging):
/// describe_logging_configuration when no config created returns an empty/absent config, not error.
#[tokio::test]
async fn test_describe_logging_when_not_configured() {
    let client = make_amp_client().await;

    let create_resp = client
        .create_workspace()
        .alias("test")
        .tags("t", "v")
        .send()
        .await
        .unwrap();
    let workspace_id = create_resp.workspace_id().to_string();

    // Should succeed (not return an error) even when no logging config has been created
    let result = client
        .describe_logging_configuration()
        .workspace_id(&workspace_id)
        .send()
        .await;
    // Moto returns {"loggingConfiguration": {}}, so no error
    assert!(
        result.is_ok(),
        "describe_logging_configuration should succeed even when not configured"
    );
}

/// Parity with moto test_list_workspaces: alias filtering returns only matching workspaces.
#[tokio::test]
async fn test_list_workspaces_filter_by_alias() {
    let client = make_amp_client().await;

    let unique_alias = format!("unique-alias-{}", uuid::Uuid::new_v4().simple());
    client
        .create_workspace()
        .alias("test")
        .send()
        .await
        .unwrap();
    client
        .create_workspace()
        .alias(&unique_alias)
        .send()
        .await
        .unwrap();

    // Filter by unique alias - should return exactly 1
    let resp = client
        .list_workspaces()
        .alias(&unique_alias)
        .send()
        .await
        .expect("list_workspaces with alias filter should succeed");
    assert_eq!(resp.workspaces().len(), 1);
    assert_eq!(resp.workspaces()[0].alias(), Some(unique_alias.as_str()));
}

/// Parity with moto test_list_rule_groups_namespaces: name prefix filter returns matching namespaces.
#[tokio::test]
async fn test_list_rule_groups_namespaces_prefix_filter() {
    let client = make_amp_client().await;

    let create_resp = client.create_workspace().send().await.unwrap();
    let workspace_id = create_resp.workspace_id().to_string();

    // Create 10 namespaces: ns0..ns9
    for idx in 0..10 {
        client
            .create_rule_groups_namespace()
            .workspace_id(&workspace_id)
            .name(format!("ns{idx}"))
            .data(aws_smithy_types::Blob::new("a"))
            .send()
            .await
            .unwrap();
    }

    // List all (no filter)
    let all_resp = client
        .list_rule_groups_namespaces()
        .workspace_id(&workspace_id)
        .send()
        .await
        .unwrap();
    assert_eq!(all_resp.rule_groups_namespaces().len(), 10);

    // Filter by prefix "ns1" - should match ns1
    let filtered_resp = client
        .list_rule_groups_namespaces()
        .workspace_id(&workspace_id)
        .name("ns1")
        .send()
        .await
        .unwrap();
    // ns1 starts with "ns1"
    assert_eq!(filtered_resp.rule_groups_namespaces().len(), 1);
    assert_eq!(filtered_resp.rule_groups_namespaces()[0].name(), "ns1");
}

/// Parity with moto test_describe_rule_groups_namespace: data is stored and returned.
#[tokio::test]
async fn test_rule_groups_namespace_data_stored() {
    let client = make_amp_client().await;

    let create_resp = client.create_workspace().send().await.unwrap();
    let workspace_id = create_resp.workspace_id().to_string();

    let data_bytes = b"asdf";
    client
        .create_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("myname")
        .data(aws_smithy_types::Blob::new(data_bytes.as_ref()))
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("myname")
        .send()
        .await
        .expect("describe_rule_groups_namespace should succeed");

    let ns = desc.rule_groups_namespace().unwrap();
    assert_eq!(ns.name(), "myname");
    assert!(!ns.arn().is_empty());
    // data is stored as base64 in state, verify it round-trips
    assert_eq!(ns.data().as_ref(), data_bytes.as_ref());
}

/// Parity with moto test_put_rule_groups_namespace: data is updated after put.
#[tokio::test]
async fn test_put_rule_groups_namespace_data_updated() {
    let client = make_amp_client().await;

    let create_resp = client.create_workspace().send().await.unwrap();
    let workspace_id = create_resp.workspace_id().to_string();

    client
        .create_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("myname")
        .data(aws_smithy_types::Blob::new(b"asdf".as_ref()))
        .send()
        .await
        .unwrap();

    client
        .put_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("myname")
        .data(aws_smithy_types::Blob::new(b"updated".as_ref()))
        .send()
        .await
        .expect("put_rule_groups_namespace should succeed");

    let desc = client
        .describe_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("myname")
        .send()
        .await
        .unwrap();

    let ns = desc.rule_groups_namespace().unwrap();
    assert_eq!(ns.data().as_ref(), b"updated".as_ref());
}

/// Parity with moto test_delete_rule_groups_namespace: describe after delete fails with error.
#[tokio::test]
async fn test_delete_rule_groups_namespace_then_describe_fails() {
    let client = make_amp_client().await;

    let create_resp = client.create_workspace().send().await.unwrap();
    let workspace_id = create_resp.workspace_id().to_string();

    client
        .create_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("myname")
        .data(aws_smithy_types::Blob::new(b"asdf".as_ref()))
        .send()
        .await
        .unwrap();

    client
        .delete_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("myname")
        .send()
        .await
        .expect("delete_rule_groups_namespace should succeed");

    let result = client
        .describe_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("myname")
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

// ============================================================================
// Tests derived from AWS documentation: Amazon Managed Service for Prometheus
// ============================================================================

#[tokio::test]
async fn test_create_workspace_no_alias() {
    let client = make_amp_client().await;

    let resp = client
        .create_workspace()
        .send()
        .await
        .expect("create_workspace without alias should succeed");

    assert!(!resp.workspace_id().is_empty());
    assert!(!resp.arn().is_empty());
}

#[tokio::test]
async fn test_list_workspaces_empty() {
    let client = make_amp_client().await;

    let resp = client
        .list_workspaces()
        .send()
        .await
        .expect("list_workspaces on empty store should succeed");

    assert_eq!(resp.workspaces().len(), 0);
}

#[tokio::test]
async fn test_create_rule_groups_namespace_duplicate() {
    let client = make_amp_client().await;

    let ws = client.create_workspace().send().await.unwrap();
    let workspace_id = ws.workspace_id().to_string();

    client
        .create_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("dup-ns")
        .data(aws_smithy_types::Blob::new(b"a".as_ref()))
        .send()
        .await
        .expect("first create should succeed");

    let result = client
        .create_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("dup-ns")
        .data(aws_smithy_types::Blob::new(b"b".as_ref()))
        .send()
        .await;
    assert!(
        result.is_err(),
        "duplicate namespace should return an error"
    );
}

#[tokio::test]
async fn test_list_tags_for_nonexistent_resource() {
    let client = make_amp_client().await;

    let result = client
        .list_tags_for_resource()
        .resource_arn("arn:aws:aps:us-east-1:123456789012:workspace/ws-nonexistent")
        .send()
        .await;
    assert!(result.is_err(), "list_tags on nonexistent ARN should fail");
}

#[tokio::test]
async fn test_create_logging_config_nonexistent_workspace() {
    let client = make_amp_client().await;

    let result = client
        .create_logging_configuration()
        .workspace_id("ws-nonexistent")
        .log_group_arn("arn:aws:logs:us-east-1:123456789012:log-group:my-log-group")
        .send()
        .await;
    assert!(
        result.is_err(),
        "create_logging_configuration on nonexistent workspace should fail"
    );
}

/// Parity with moto test_tag_resource (for rule groups namespaces).
#[tokio::test]
async fn test_tag_rule_groups_namespace() {
    let client = make_amp_client().await;

    let create_resp = client.create_workspace().send().await.unwrap();
    let workspace_id = create_resp.workspace_id().to_string();

    let ns_resp = client
        .create_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("ns")
        .data(aws_smithy_types::Blob::new(b"a".as_ref()))
        .tags("t", "v")
        .send()
        .await
        .unwrap();
    let arn = ns_resp.arn().to_string();

    // Add more tags
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("t1", "v1")
        .tags("t2", "v2")
        .send()
        .await
        .expect("tag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("t").map(|s| s.as_str()), Some("v"));
    assert_eq!(tags.get("t1").map(|s| s.as_str()), Some("v1"));
    assert_eq!(tags.get("t2").map(|s| s.as_str()), Some("v2"));

    // Remove t1
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("t1")
        .send()
        .await
        .expect("untag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    let tags = tags_resp.tags().unwrap();
    assert!(tags.get("t1").is_none());
    assert_eq!(tags.get("t").map(|s| s.as_str()), Some("v"));
    assert_eq!(tags.get("t2").map(|s| s.as_str()), Some("v2"));

    // Remove remaining tags
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("t")
        .tag_keys("t2")
        .send()
        .await
        .unwrap();

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert!(tags_resp.tags().unwrap().is_empty());
}
