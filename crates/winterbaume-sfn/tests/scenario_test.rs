//! End-to-end scenario tests for winterbaume Step Functions service.
//!
//! Each test exercises a realistic use-case workflow chaining 3+ operations
//! and asserts business outcomes rather than per-API return shapes.

use aws_sdk_sfn::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sfn::SfnService;

const ACCOUNT_ID: &str = "123456789012";

async fn make_sfn_client() -> aws_sdk_sfn::Client {
    let mock = MockAws::builder().with_service(SfnService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sfn::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_sfn::Client::new(&config)
}

const SIMPLE_DEFINITION: &str = r#"{"Comment":"Simple pass state","StartAt":"Pass","States":{"Pass":{"Type":"Pass","End":true}}}"#;

fn default_role() -> String {
    format!("arn:aws:iam::{ACCOUNT_ID}:role/sfn_role")
}

/// Scenario: Basic state machine lifecycle — create, start execution, describe, stop, delete.
///
/// Covers: CreateStateMachine → StartExecution → DescribeExecution → StopExecution →
///         DescribeExecution (verify stopped) → DeleteStateMachine.
#[tokio::test]
async fn test_state_machine_full_lifecycle() {
    let client = make_sfn_client().await;

    // Create a state machine.
    let create_resp = client
        .create_state_machine()
        .name("lifecycle-test")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .expect("CreateStateMachine should succeed");
    let sm_arn = create_resp.state_machine_arn().to_string();
    assert!(sm_arn.contains(":stateMachine:lifecycle-test"));

    // Start an execution.
    let start_resp = client
        .start_execution()
        .state_machine_arn(&sm_arn)
        .name("run-1")
        .input(r#"{"key":"value"}"#)
        .send()
        .await
        .expect("StartExecution should succeed");
    let exec_arn = start_resp.execution_arn().to_string();
    assert!(exec_arn.contains(":execution:lifecycle-test:run-1"));

    // Describe the execution — should be RUNNING.
    let desc_resp = client
        .describe_execution()
        .execution_arn(&exec_arn)
        .send()
        .await
        .expect("DescribeExecution should succeed");
    assert_eq!(
        format!("{:?}", desc_resp.status()),
        "Running",
        "execution should start in RUNNING state"
    );

    // Stop the execution.
    client
        .stop_execution()
        .execution_arn(&exec_arn)
        .send()
        .await
        .expect("StopExecution should succeed");

    // Describe again — should be ABORTED.
    let desc_after = client
        .describe_execution()
        .execution_arn(&exec_arn)
        .send()
        .await
        .expect("DescribeExecution after stop should succeed");
    assert_eq!(
        format!("{:?}", desc_after.status()),
        "Aborted",
        "execution should be ABORTED after stop"
    );

    // Delete the state machine — idempotent.
    client
        .delete_state_machine()
        .state_machine_arn(&sm_arn)
        .send()
        .await
        .expect("DeleteStateMachine should succeed");

    // Verify it is gone.
    let list_resp = client
        .list_state_machines()
        .send()
        .await
        .expect("ListStateMachines should succeed");
    assert!(
        list_resp
            .state_machines()
            .iter()
            .all(|sm| sm.state_machine_arn() != sm_arn),
        "deleted state machine should not appear in list"
    );
}

/// Scenario: Versioning and alias routing — create, publish two versions, create alias,
///           update alias routing, delete alias and versions.
///
/// Covers: CreateStateMachine → PublishStateMachineVersion (×2) →
///         CreateStateMachineAlias → UpdateStateMachineAlias →
///         DescribeStateMachineAlias (verify routing) → DeleteStateMachineAlias →
///         DeleteStateMachineVersion (×2).
#[tokio::test]
async fn test_versioning_and_alias_routing() {
    let client = make_sfn_client().await;

    // Create a state machine.
    let create_resp = client
        .create_state_machine()
        .name("versioned-sm")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .expect("CreateStateMachine should succeed");
    let sm_arn = create_resp.state_machine_arn().to_string();

    // Publish version 1.
    let v1 = client
        .publish_state_machine_version()
        .state_machine_arn(&sm_arn)
        .description("v1")
        .send()
        .await
        .expect("PublishStateMachineVersion v1 should succeed");
    let v1_arn = v1.state_machine_version_arn().to_string();
    assert!(
        v1_arn.contains(":stateMachine:versioned-sm:"),
        "v1 ARN should embed state machine name"
    );

    // Publish version 2.
    let v2 = client
        .publish_state_machine_version()
        .state_machine_arn(&sm_arn)
        .description("v2")
        .send()
        .await
        .expect("PublishStateMachineVersion v2 should succeed");
    let v2_arn = v2.state_machine_version_arn().to_string();
    assert_ne!(v1_arn, v2_arn, "v1 and v2 should have different ARNs");

    // Create an alias pointing 100% to v1.
    let alias = client
        .create_state_machine_alias()
        .name("stable")
        .routing_configuration(
            aws_sdk_sfn::types::RoutingConfigurationListItem::builder()
                .state_machine_version_arn(&v1_arn)
                .weight(100)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("CreateStateMachineAlias should succeed");
    let alias_arn = alias.state_machine_alias_arn().to_string();
    assert!(alias_arn.contains(":stateMachine:versioned-sm:stable"));

    // Update the alias to split traffic 50/50 between v1 and v2.
    client
        .update_state_machine_alias()
        .state_machine_alias_arn(&alias_arn)
        .routing_configuration(
            aws_sdk_sfn::types::RoutingConfigurationListItem::builder()
                .state_machine_version_arn(&v1_arn)
                .weight(50)
                .build()
                .unwrap(),
        )
        .routing_configuration(
            aws_sdk_sfn::types::RoutingConfigurationListItem::builder()
                .state_machine_version_arn(&v2_arn)
                .weight(50)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("UpdateStateMachineAlias should succeed");

    // Describe alias — routing should now have two entries.
    let desc_alias = client
        .describe_state_machine_alias()
        .state_machine_alias_arn(&alias_arn)
        .send()
        .await
        .expect("DescribeStateMachineAlias should succeed");
    assert_eq!(
        desc_alias.routing_configuration().len(),
        2,
        "alias routing should have 2 entries after update"
    );

    // Delete alias and both versions.
    client
        .delete_state_machine_alias()
        .state_machine_alias_arn(&alias_arn)
        .send()
        .await
        .expect("DeleteStateMachineAlias should succeed");
    client
        .delete_state_machine_version()
        .state_machine_version_arn(&v1_arn)
        .send()
        .await
        .expect("DeleteStateMachineVersion v1 should succeed");
    client
        .delete_state_machine_version()
        .state_machine_version_arn(&v2_arn)
        .send()
        .await
        .expect("DeleteStateMachineVersion v2 should succeed");

    // Versions list should now be empty.
    let versions = client
        .list_state_machine_versions()
        .state_machine_arn(&sm_arn)
        .send()
        .await
        .expect("ListStateMachineVersions should succeed");
    assert!(
        versions.state_machine_versions().is_empty(),
        "all versions should be deleted"
    );
}

/// Scenario: Execution redrive — fail an execution then redrive it back to running.
///
/// Covers: CreateStateMachine → StartExecution → StopExecution (ABORTED) →
///         RedriveExecution → DescribeExecution (verify RUNNING again) →
///         GetExecutionHistory.
#[tokio::test]
async fn test_execution_redrive_pipeline() {
    let client = make_sfn_client().await;

    // Create state machine.
    let sm = client
        .create_state_machine()
        .name("redrive-sm")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .expect("CreateStateMachine should succeed");
    let sm_arn = sm.state_machine_arn().to_string();

    // Start execution.
    let start = client
        .start_execution()
        .state_machine_arn(&sm_arn)
        .name("redrive-run")
        .send()
        .await
        .expect("StartExecution should succeed");
    let exec_arn = start.execution_arn().to_string();

    // Abort the execution.
    client
        .stop_execution()
        .execution_arn(&exec_arn)
        .send()
        .await
        .expect("StopExecution should succeed");

    // Verify ABORTED.
    let after_stop = client
        .describe_execution()
        .execution_arn(&exec_arn)
        .send()
        .await
        .expect("DescribeExecution should succeed");
    assert_eq!(
        format!("{:?}", after_stop.status()),
        "Aborted",
        "should be ABORTED after stop"
    );

    // Redrive — should move it back to RUNNING.
    client
        .redrive_execution()
        .execution_arn(&exec_arn)
        .send()
        .await
        .expect("RedriveExecution should succeed");

    let after_redrive = client
        .describe_execution()
        .execution_arn(&exec_arn)
        .send()
        .await
        .expect("DescribeExecution after redrive should succeed");
    assert_eq!(
        format!("{:?}", after_redrive.status()),
        "Running",
        "execution should be RUNNING after redrive"
    );

    // Get execution history — should have at least one event.
    let history = client
        .get_execution_history()
        .execution_arn(&exec_arn)
        .send()
        .await
        .expect("GetExecutionHistory should succeed");
    assert!(
        !history.events().is_empty(),
        "execution history should contain at least one event"
    );
}

/// Scenario: Activity-backed tagging lifecycle — create activity, tag it, list tags,
///           untag, verify removal, delete.
///
/// Covers: CreateActivity → TagResource → ListTagsForResource →
///         UntagResource → ListTagsForResource (verify removal) → DeleteActivity.
#[tokio::test]
async fn test_activity_tagging_lifecycle() {
    let client = make_sfn_client().await;

    // Create an activity.
    let activity = client
        .create_activity()
        .name("tagged-activity")
        .send()
        .await
        .expect("CreateActivity should succeed");
    let activity_arn = activity.activity_arn().to_string();

    // Tag the activity.
    client
        .tag_resource()
        .resource_arn(&activity_arn)
        .tags(
            aws_sdk_sfn::types::Tag::builder()
                .key("env")
                .value("prod")
                .build(),
        )
        .tags(
            aws_sdk_sfn::types::Tag::builder()
                .key("owner")
                .value("team-a")
                .build(),
        )
        .send()
        .await
        .expect("TagResource should succeed");

    // List tags — should have two.
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&activity_arn)
        .send()
        .await
        .expect("ListTagsForResource should succeed");
    assert_eq!(tags_resp.tags().len(), 2, "activity should have 2 tags");

    // Untag one key.
    client
        .untag_resource()
        .resource_arn(&activity_arn)
        .tag_keys("env")
        .send()
        .await
        .expect("UntagResource should succeed");

    // List tags — should now have only one.
    let tags_after = client
        .list_tags_for_resource()
        .resource_arn(&activity_arn)
        .send()
        .await
        .expect("ListTagsForResource after untag should succeed");
    assert_eq!(
        tags_after.tags().len(),
        1,
        "activity should have 1 tag after untag"
    );
    assert_eq!(
        tags_after.tags()[0].key(),
        Some("owner"),
        "remaining tag key should be 'owner'"
    );

    // Delete the activity.
    client
        .delete_activity()
        .activity_arn(&activity_arn)
        .send()
        .await
        .expect("DeleteActivity should succeed");

    // Confirm it is gone — describe should fail.
    let result = client
        .describe_activity()
        .activity_arn(&activity_arn)
        .send()
        .await;
    assert!(
        result.is_err(),
        "describe on deleted activity should return an error"
    );
}
