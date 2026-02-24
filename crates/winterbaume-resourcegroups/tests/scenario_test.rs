//! End-to-end scenario tests for winterbaume ResourceGroups service.
//!
//! Each test simulates a coherent multi-step workflow rather than exercising
//! a single API call in isolation.

use aws_sdk_resourcegroups::config::BehaviorVersion;
use aws_sdk_resourcegroups::types::{
    GroupConfigurationItem, GroupConfigurationParameter, QueryType, ResourceQuery,
};
use winterbaume_core::MockAws;
use winterbaume_resourcegroups::ResourceGroupsService;

async fn make_client() -> aws_sdk_resourcegroups::Client {
    let mock = MockAws::builder()
        .with_service(ResourceGroupsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_resourcegroups::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_resourcegroups::Client::new(&config)
}

/// Scenario: application environment grouping and tag management.
///
/// An operator creates a resource group for a production environment, applies
/// organisational tags to the group, updates the query to narrow resource scope,
/// then removes stale tags before finally decommissioning the group.
#[tokio::test]
async fn test_application_environment_grouping() {
    let client = make_client().await;

    // Create a group representing the production environment.
    let create_resp = client
        .create_group()
        .name("prod-env-group")
        .description("Production environment resources")
        .resource_query(
            ResourceQuery::builder()
                .r#type(QueryType::TagFilters10)
                .query(
                    r#"{"ResourceTypeFilters":["AWS::AllSupported"],"TagFilters":[{"Key":"Env","Values":["prod"]}]}"#,
                )
                .build()
                .unwrap(),
        )
        .tags("owner", "platform-team")
        .tags("cost-centre", "engineering")
        .send()
        .await
        .expect("create_group should succeed");

    let group = create_resp.group().expect("should have group");
    assert_eq!(group.name(), "prod-env-group");
    let arn = group.group_arn().to_string();
    assert!(arn.contains("prod-env-group"));

    // Verify the group is listed.
    let list_resp = client
        .list_groups()
        .send()
        .await
        .expect("list_groups should succeed");
    let found = list_resp
        .group_identifiers()
        .iter()
        .any(|gi| gi.group_name().unwrap_or("") == "prod-env-group");
    assert!(found, "group should appear in list_groups");

    // Verify tags are accessible via get_tags.
    let tags_resp = client
        .get_tags()
        .arn(&arn)
        .send()
        .await
        .expect("get_tags should succeed");
    let empty_map = std::collections::HashMap::new();
    let tags = tags_resp.tags().unwrap_or(&empty_map);
    assert_eq!(tags.get("owner").map(|s| s.as_str()), Some("platform-team"));
    assert_eq!(
        tags.get("cost-centre").map(|s| s.as_str()),
        Some("engineering")
    );

    // Narrow the query to only EC2 instances.
    client
        .update_group_query()
        .group("prod-env-group")
        .resource_query(
            ResourceQuery::builder()
                .r#type(QueryType::TagFilters10)
                .query(
                    r#"{"ResourceTypeFilters":["AWS::EC2::Instance"],"TagFilters":[{"Key":"Env","Values":["prod"]}]}"#,
                )
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("update_group_query should succeed");

    // Remove the stale cost-centre tag.
    client
        .untag()
        .arn(&arn)
        .keys("cost-centre")
        .send()
        .await
        .expect("untag should succeed");

    let tags_resp2 = client
        .get_tags()
        .arn(&arn)
        .send()
        .await
        .expect("get_tags should succeed after untag");
    let tags2 = tags_resp2.tags().unwrap_or(&empty_map);
    assert!(
        tags2.get("cost-centre").is_none(),
        "cost-centre tag should be removed"
    );
    assert_eq!(
        tags2.get("owner").map(|s| s.as_str()),
        Some("platform-team")
    );

    // Decommission the group.
    client
        .delete_group()
        .group("prod-env-group")
        .send()
        .await
        .expect("delete_group should succeed");

    let result = client.get_group().group("prod-env-group").send().await;
    assert!(result.is_err(), "group should be gone after deletion");
}

/// Scenario: configuration-backed capacity reservation pool lifecycle.
///
/// An operator creates a configuration-backed group (no tag query), attaches
/// explicit resource ARNs via GroupResources, retrieves those members, then
/// removes them via UngroupResources before cleaning up the group.
#[tokio::test]
async fn test_configuration_backed_group_resource_management() {
    let client = make_client().await;

    // Create a capacity-reservation-pool configuration group.
    let param = GroupConfigurationParameter::builder()
        .name("allowed-resource-types")
        .values("AWS::EC2::CapacityReservation")
        .build()
        .unwrap();
    let config_item = GroupConfigurationItem::builder()
        .r#type("AWS::EC2::CapacityReservationPool")
        .parameters(param)
        .build()
        .unwrap();

    client
        .create_group()
        .name("cap-pool-group")
        .description("Capacity reservation pool")
        .configuration(config_item)
        .send()
        .await
        .expect("create_group with configuration should succeed");

    // Attach two synthetic resource ARNs.
    let arn_a = "arn:aws:ec2:us-east-1:123456789012:capacity-reservation/cr-aaa";
    let arn_b = "arn:aws:ec2:us-east-1:123456789012:capacity-reservation/cr-bbb";

    let group_resp = client
        .group_resources()
        .group("cap-pool-group")
        .resource_arns(arn_a)
        .resource_arns(arn_b)
        .send()
        .await
        .expect("group_resources should succeed");
    assert_eq!(group_resp.succeeded().len(), 2);

    // Verify both ARNs appear in list_group_resources.
    let list_resp = client
        .list_group_resources()
        .group("cap-pool-group")
        .send()
        .await
        .expect("list_group_resources should succeed");
    let resource_arns: Vec<&str> = list_resp
        .resources()
        .iter()
        .filter_map(|item| item.identifier())
        .filter_map(|ri| ri.resource_arn())
        .collect();
    assert!(
        resource_arns.contains(&arn_a),
        "should contain arn_a: {resource_arns:?}"
    );
    assert!(
        resource_arns.contains(&arn_b),
        "should contain arn_b: {resource_arns:?}"
    );

    // Remove arn_a from the group.
    let ungroup_resp = client
        .ungroup_resources()
        .group("cap-pool-group")
        .resource_arns(arn_a)
        .send()
        .await
        .expect("ungroup_resources should succeed");
    assert_eq!(ungroup_resp.succeeded().len(), 1);

    // Only arn_b should remain.
    let list_resp2 = client
        .list_group_resources()
        .group("cap-pool-group")
        .send()
        .await
        .expect("list_group_resources should succeed after ungrouping");
    let resource_arns2: Vec<&str> = list_resp2
        .resources()
        .iter()
        .filter_map(|item| item.identifier())
        .filter_map(|ri| ri.resource_arn())
        .collect();
    assert_eq!(resource_arns2.len(), 1);
    assert!(
        resource_arns2.contains(&arn_b),
        "only arn_b should remain: {resource_arns2:?}"
    );
    assert!(
        !resource_arns2.contains(&arn_a),
        "arn_a should have been removed"
    );

    // Clean up.
    client
        .delete_group()
        .group("cap-pool-group")
        .send()
        .await
        .expect("delete_group should succeed");
}

/// Scenario: tag-sync task lifecycle.
///
/// An operator creates a group, starts a tag-sync task to automatically
/// add resources matching a tag key/value pair, lists and filters the tasks,
/// then cancels it when the sync policy is no longer needed.
#[tokio::test]
async fn test_tag_sync_task_lifecycle() {
    let client = make_client().await;

    // Create the target group.
    client
        .create_group()
        .name("tag-sync-target")
        .description("Group managed by tag sync")
        .resource_query(
            ResourceQuery::builder()
                .r#type(QueryType::TagFilters10)
                .query(r#"{"ResourceTypeFilters":["AWS::AllSupported"],"TagFilters":[]}"#)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_group should succeed");

    // Start a tag-sync task.
    let start_resp = client
        .start_tag_sync_task()
        .group("tag-sync-target")
        .tag_key("ManagedBy")
        .tag_value("TagSync")
        .role_arn("arn:aws:iam::123456789012:role/tag-sync-role")
        .send()
        .await
        .expect("start_tag_sync_task should succeed");

    let task_arn = start_resp
        .task_arn()
        .expect("should have task_arn")
        .to_string();
    assert!(task_arn.contains("tag-sync-task"));

    // Get and verify task details.
    let get_resp = client
        .get_tag_sync_task()
        .task_arn(&task_arn)
        .send()
        .await
        .expect("get_tag_sync_task should succeed");
    assert_eq!(get_resp.group_name().unwrap_or(""), "tag-sync-target");
    assert_eq!(get_resp.tag_key().unwrap_or(""), "ManagedBy");
    assert_eq!(get_resp.tag_value().unwrap_or(""), "TagSync");

    // List tasks (unfiltered) — should include our task.
    let list_resp = client
        .list_tag_sync_tasks()
        .send()
        .await
        .expect("list_tag_sync_tasks should succeed");
    let task_arns: Vec<&str> = list_resp
        .tag_sync_tasks()
        .iter()
        .filter_map(|t| t.task_arn())
        .collect();
    assert!(
        task_arns.contains(&task_arn.as_str()),
        "task should appear in unfiltered list"
    );

    // Cancel the task.
    client
        .cancel_tag_sync_task()
        .task_arn(&task_arn)
        .send()
        .await
        .expect("cancel_tag_sync_task should succeed");

    // Verify status is CANCELLED.
    let get_after_cancel = client
        .get_tag_sync_task()
        .task_arn(&task_arn)
        .send()
        .await
        .expect("get_tag_sync_task should succeed after cancel");
    assert_eq!(
        get_after_cancel.status().map(|s| s.as_str()),
        Some("CANCELLED"),
        "task status should be CANCELLED"
    );

    // Clean up.
    client
        .delete_group()
        .group("tag-sync-target")
        .send()
        .await
        .expect("delete_group should succeed");
}
