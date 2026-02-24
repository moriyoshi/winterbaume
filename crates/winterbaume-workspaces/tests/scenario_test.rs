//! Scenario tests for winterbaume WorkSpaces service.
//!
//! Each test exercises an end-to-end use-case by chaining 3+ operations and
//! asserting business outcomes rather than per-API return shapes.

use aws_sdk_workspaces::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_workspaces::WorkSpacesService;

/// Helper to create a configured WorkSpaces client backed by winterbaume.
async fn make_workspaces_client() -> aws_sdk_workspaces::Client {
    let mock = MockAws::builder()
        .with_service(WorkSpacesService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_workspaces::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_workspaces::Client::new(&config)
}

/// Scenario: full workspace lifecycle — create, tag, modify properties, stop, start, terminate.
///
/// Verifies that a workspace can be provisioned, tagged, reconfigured, and torn down.
#[tokio::test]
async fn test_workspace_full_lifecycle() {
    // Scenario: Workspace lifecycle
    // 1. Create workspace
    // 2. Tag it
    // 3. Modify its running mode
    // 4. Stop and restart it
    // 5. Terminate it and confirm it disappears

    let client = make_workspaces_client().await;

    // Step 1: Create
    let req = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-lifecycle-test")
        .user_name("alice")
        .bundle_id("wsb-lifecycle")
        .build()
        .unwrap();

    let create_resp = client
        .create_workspaces()
        .workspaces(req)
        .send()
        .await
        .expect("create_workspaces should succeed");

    assert!(
        create_resp.failed_requests().is_empty(),
        "no workspace creation failures expected"
    );
    let workspace_id = create_resp.pending_requests()[0]
        .workspace_id()
        .unwrap()
        .to_string();

    // Step 2: Tag the workspace
    let env_tag = aws_sdk_workspaces::types::Tag::builder()
        .key("Environment")
        .value("Staging")
        .build()
        .unwrap();
    client
        .create_tags()
        .resource_id(&workspace_id)
        .tags(env_tag)
        .send()
        .await
        .expect("create_tags should succeed");

    let tags_resp = client
        .describe_tags()
        .resource_id(&workspace_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        tags_resp.tag_list().len(),
        1,
        "workspace should have one tag"
    );
    assert_eq!(tags_resp.tag_list()[0].key(), "Environment");

    // Step 3: Modify running mode to ALWAYS_ON
    client
        .modify_workspace_properties()
        .workspace_id(&workspace_id)
        .workspace_properties(
            aws_sdk_workspaces::types::WorkspaceProperties::builder()
                .running_mode(aws_sdk_workspaces::types::RunningMode::AlwaysOn)
                .build(),
        )
        .send()
        .await
        .expect("modify_workspace_properties should succeed");

    // Step 4: Stop, then start
    let stop_req = aws_sdk_workspaces::types::StopRequest::builder()
        .workspace_id(&workspace_id)
        .build();
    let stop_resp = client
        .stop_workspaces()
        .stop_workspace_requests(stop_req)
        .send()
        .await
        .unwrap();
    assert!(
        stop_resp.failed_requests().is_empty(),
        "stop should succeed"
    );

    let start_req = aws_sdk_workspaces::types::StartRequest::builder()
        .workspace_id(&workspace_id)
        .build();
    let start_resp = client
        .start_workspaces()
        .start_workspace_requests(start_req)
        .send()
        .await
        .unwrap();
    assert!(
        start_resp.failed_requests().is_empty(),
        "start should succeed"
    );

    // Step 5: Terminate and confirm removal
    let term_req = aws_sdk_workspaces::types::TerminateRequest::builder()
        .workspace_id(&workspace_id)
        .build()
        .unwrap();
    let term_resp = client
        .terminate_workspaces()
        .terminate_workspace_requests(term_req)
        .send()
        .await
        .unwrap();
    assert!(
        term_resp.failed_requests().is_empty(),
        "terminate should succeed"
    );

    let desc_resp = client
        .describe_workspaces()
        .workspace_ids(&workspace_id)
        .send()
        .await
        .unwrap();
    assert!(
        desc_resp.workspaces().is_empty(),
        "terminated workspace must not appear in describe"
    );
}

/// Scenario: workspace image pipeline — create workspace, capture image, create bundle from
/// image, provision new workspace using that bundle, clean up.
///
/// Verifies that the image and bundle creation pipeline works end-to-end.
#[tokio::test]
async fn test_workspace_image_bundle_pipeline() {
    // Scenario: Image/bundle pipeline
    // 1. Create a base workspace
    // 2. Capture it as an image
    // 3. Create a bundle using that image
    // 4. Confirm bundle is listed
    // 5. Delete bundle and image

    let client = make_workspaces_client().await;

    // Step 1: Create base workspace
    let req = aws_sdk_workspaces::types::WorkspaceRequest::builder()
        .directory_id("d-image-pipeline")
        .user_name("bob")
        .bundle_id("wsb-base")
        .build()
        .unwrap();
    let create_resp = client
        .create_workspaces()
        .workspaces(req)
        .send()
        .await
        .unwrap();
    let workspace_id = create_resp.pending_requests()[0]
        .workspace_id()
        .unwrap()
        .to_string();

    // Step 2: Capture image
    let image_resp = client
        .create_workspace_image()
        .name("golden-image")
        .description("Golden image for test")
        .workspace_id(&workspace_id)
        .send()
        .await
        .expect("create_workspace_image should succeed");
    let image_id = image_resp.image_id().unwrap().to_string();
    assert_eq!(
        image_resp.state().map(|s| s.as_str()),
        Some("AVAILABLE"),
        "image should be AVAILABLE"
    );

    // Step 3: Create bundle
    let bundle_resp = client
        .create_workspace_bundle()
        .bundle_name("golden-bundle")
        .bundle_description("Bundle from golden image")
        .image_id(&image_id)
        .compute_type(
            aws_sdk_workspaces::types::ComputeType::builder()
                .name(aws_sdk_workspaces::types::Compute::Value)
                .build(),
        )
        .user_storage(
            aws_sdk_workspaces::types::UserStorage::builder()
                .capacity("50")
                .build()
                .expect("user_storage build should succeed"),
        )
        .send()
        .await
        .expect("create_workspace_bundle should succeed");
    let bundle_id = bundle_resp
        .workspace_bundle()
        .and_then(|b| b.bundle_id())
        .unwrap()
        .to_string();

    // Step 4: Confirm bundle is listed
    let desc_bundles = client
        .describe_workspace_bundles()
        .bundle_ids(&bundle_id)
        .send()
        .await
        .unwrap();
    assert_eq!(
        desc_bundles.bundles().len(),
        1,
        "bundle should appear in describe"
    );
    assert_eq!(
        desc_bundles.bundles()[0].name(),
        Some("golden-bundle"),
        "bundle name should match"
    );

    // Step 5: Delete bundle then image
    client
        .delete_workspace_bundle()
        .bundle_id(&bundle_id)
        .send()
        .await
        .expect("delete_workspace_bundle should succeed");

    client
        .delete_workspace_image()
        .image_id(&image_id)
        .send()
        .await
        .expect("delete_workspace_image should succeed");

    // Confirm image is gone
    let remaining = client
        .describe_workspace_images()
        .image_ids(&image_id)
        .send()
        .await
        .unwrap();
    assert!(
        remaining.images().is_empty(),
        "deleted image must not appear in describe"
    );
}

/// Scenario: IP group access control — create IP group, add/update rules, associate with
/// directory, revoke rules, and clean up.
///
/// Verifies that IP access control groups can be managed across their full lifecycle.
#[tokio::test]
async fn test_ip_group_access_control_lifecycle() {
    // Scenario: IP group lifecycle
    // 1. Create a directory
    // 2. Create an IP group with initial rules
    // 3. Authorise additional rules
    // 4. Associate with directory
    // 5. Update all rules to a new set
    // 6. Revoke a rule and confirm it is gone
    // 7. Delete the group

    let client = make_workspaces_client().await;

    // Step 1: Register a directory
    client
        .register_workspace_directory()
        .directory_id("d-ipgroup-test")
        .send()
        .await
        .expect("register_workspace_directory should succeed");

    // Step 2: Create IP group
    let rule1 = aws_sdk_workspaces::types::IpRuleItem::builder()
        .ip_rule("10.0.0.0/8")
        .rule_desc("Internal network")
        .build();
    let create_resp = client
        .create_ip_group()
        .group_name("Corp-Access")
        .group_desc("Corporate IP access group")
        .user_rules(rule1)
        .send()
        .await
        .expect("create_ip_group should succeed");
    let group_id = create_resp.group_id().unwrap().to_string();

    // Step 3: Authorise an additional rule
    let rule2 = aws_sdk_workspaces::types::IpRuleItem::builder()
        .ip_rule("192.168.1.0/24")
        .rule_desc("VPN range")
        .build();
    client
        .authorize_ip_rules()
        .group_id(&group_id)
        .user_rules(rule2)
        .send()
        .await
        .expect("authorize_ip_rules should succeed");

    // Step 4: Associate with directory
    client
        .associate_ip_groups()
        .directory_id("d-ipgroup-test")
        .group_ids(&group_id)
        .send()
        .await
        .expect("associate_ip_groups should succeed");

    // Step 5: Replace all rules
    let new_rule = aws_sdk_workspaces::types::IpRuleItem::builder()
        .ip_rule("172.16.0.0/12")
        .rule_desc("New range only")
        .build();
    client
        .update_rules_of_ip_group()
        .group_id(&group_id)
        .user_rules(new_rule)
        .send()
        .await
        .expect("update_rules_of_ip_group should succeed");

    let groups = client
        .describe_ip_groups()
        .group_ids(&group_id)
        .send()
        .await
        .unwrap();
    assert_eq!(groups.result().len(), 1, "should have one group");
    assert_eq!(
        groups.result()[0].user_rules().len(),
        1,
        "should have exactly one rule after update"
    );
    assert_eq!(
        groups.result()[0].user_rules()[0].ip_rule(),
        Some("172.16.0.0/12")
    );

    // Step 6: Revoke the remaining rule
    client
        .revoke_ip_rules()
        .group_id(&group_id)
        .user_rules("172.16.0.0/12")
        .send()
        .await
        .expect("revoke_ip_rules should succeed");

    let empty_group = client
        .describe_ip_groups()
        .group_ids(&group_id)
        .send()
        .await
        .unwrap();
    assert!(
        empty_group.result()[0].user_rules().is_empty(),
        "all rules should be revoked"
    );

    // Step 7: Delete the group
    client
        .delete_ip_group()
        .group_id(&group_id)
        .send()
        .await
        .expect("delete_ip_group should succeed");

    let after_delete = client
        .describe_ip_groups()
        .group_ids(&group_id)
        .send()
        .await
        .unwrap();
    assert!(
        after_delete.result().is_empty(),
        "deleted group must not appear in describe"
    );
}
