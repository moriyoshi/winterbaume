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

/// Scenario: Workspace with rule groups namespaces lifecycle.
///
/// Demonstrates a complete workflow where a workspace is created, rule groups namespaces
/// are configured, tags are managed, and cleanup is performed.
#[tokio::test]
async fn test_workspace_with_rule_groups_lifecycle() {
    // Scenario: Create a workspace, attach rule groups namespaces, manage tags, then clean up.
    let client = make_amp_client().await;

    // Step 1: Create a workspace with initial tags.
    let create_resp = client
        .create_workspace()
        .alias("prod-workspace")
        .tags("env", "production")
        .tags("owner", "platform-team")
        .send()
        .await
        .expect("create_workspace should succeed");

    let workspace_id = create_resp.workspace_id().to_string();
    let workspace_arn = create_resp.arn().to_string();

    assert!(!workspace_id.is_empty());
    assert!(workspace_arn.contains("workspace"));

    // Step 2: Create two rule groups namespaces for alerting rules.
    client
        .create_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("infra-rules")
        .data(aws_smithy_types::Blob::new(b"Z3JvdXBzOgotIG5hbWU6IGluZnJh"))
        .tags("team", "infra")
        .send()
        .await
        .expect("create_rule_groups_namespace for infra-rules should succeed");

    client
        .create_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("app-rules")
        .data(aws_smithy_types::Blob::new(b"Z3JvdXBzOgotIG5hbWU6IGFwcA=="))
        .tags("team", "app")
        .send()
        .await
        .expect("create_rule_groups_namespace for app-rules should succeed");

    // Step 3: Verify both namespaces are listed.
    let list_resp = client
        .list_rule_groups_namespaces()
        .workspace_id(&workspace_id)
        .send()
        .await
        .expect("list_rule_groups_namespaces should succeed");

    assert_eq!(
        list_resp.rule_groups_namespaces().len(),
        2,
        "both namespaces should be listed"
    );

    // Step 4: Update the infra-rules namespace data.
    client
        .put_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("infra-rules")
        .data(aws_smithy_types::Blob::new(
            b"Z3JvdXBzOgotIG5hbWU6IGluZnJhLXVwZGF0ZWQ=",
        ))
        .send()
        .await
        .expect("put_rule_groups_namespace should succeed");

    let desc = client
        .describe_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("infra-rules")
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc.rule_groups_namespace().unwrap().data().as_ref(),
        b"Z3JvdXBzOgotIG5hbWU6IGluZnJhLXVwZGF0ZWQ="
    );

    // Step 5: Tag the workspace with additional metadata, then list tags.
    client
        .tag_resource()
        .resource_arn(&workspace_arn)
        .tags("cost-centre", "obs")
        .send()
        .await
        .expect("tag_resource should succeed");

    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&workspace_arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = tags_resp.tags().unwrap();
    assert_eq!(tags.get("env").map(|s| s.as_str()), Some("production"));
    assert_eq!(tags.get("cost-centre").map(|s| s.as_str()), Some("obs"));

    // Step 6: Delete one namespace and verify it is gone.
    client
        .delete_rule_groups_namespace()
        .workspace_id(&workspace_id)
        .name("app-rules")
        .send()
        .await
        .expect("delete_rule_groups_namespace should succeed");

    let list_after = client
        .list_rule_groups_namespaces()
        .workspace_id(&workspace_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        list_after.rule_groups_namespaces().len(),
        1,
        "only infra-rules should remain"
    );

    // Step 7: Delete the workspace itself.
    client
        .delete_workspace()
        .workspace_id(&workspace_id)
        .send()
        .await
        .expect("delete_workspace should succeed");

    // Step 8: Verify workspace is gone.
    let result = client
        .describe_workspace()
        .workspace_id(&workspace_id)
        .send()
        .await;
    assert!(result.is_err(), "describe after delete should fail");
}

/// Scenario: Workspace logging configuration lifecycle.
///
/// Demonstrates attaching, updating, and removing a logging configuration from a workspace,
/// chaining create-workspace → create-logging → update-logging → describe-logging → delete-logging.
#[tokio::test]
async fn test_workspace_logging_lifecycle() {
    // Scenario: Full logging config lifecycle on a workspace.
    let client = make_amp_client().await;

    // Step 1: Create workspace.
    let ws = client
        .create_workspace()
        .alias("logging-scenario-ws")
        .send()
        .await
        .expect("create_workspace should succeed");
    let workspace_id = ws.workspace_id().to_string();

    // Step 2: Attach logging configuration.
    let log_arn = "arn:aws:logs:us-east-1:123456789012:log-group:amp-logs";
    client
        .create_logging_configuration()
        .workspace_id(&workspace_id)
        .log_group_arn(log_arn)
        .send()
        .await
        .expect("create_logging_configuration should succeed");

    // Step 3: Verify it is stored.
    let desc = client
        .describe_logging_configuration()
        .workspace_id(&workspace_id)
        .send()
        .await
        .expect("describe_logging_configuration should succeed");

    assert_eq!(
        desc.logging_configuration().unwrap().log_group_arn(),
        log_arn
    );

    // Step 4: Update the log group ARN.
    let new_log_arn = "arn:aws:logs:us-east-1:123456789012:log-group:amp-logs-v2";
    client
        .update_logging_configuration()
        .workspace_id(&workspace_id)
        .log_group_arn(new_log_arn)
        .send()
        .await
        .expect("update_logging_configuration should succeed");

    let desc2 = client
        .describe_logging_configuration()
        .workspace_id(&workspace_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc2.logging_configuration().unwrap().log_group_arn(),
        new_log_arn
    );

    // Step 5: Delete logging configuration.
    client
        .delete_logging_configuration()
        .workspace_id(&workspace_id)
        .send()
        .await
        .expect("delete_logging_configuration should succeed");

    // Step 6: Clean up workspace.
    client
        .delete_workspace()
        .workspace_id(&workspace_id)
        .send()
        .await
        .expect("delete_workspace should succeed");
}
