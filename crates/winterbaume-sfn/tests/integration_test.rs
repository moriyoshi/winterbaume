//! Integration tests for winterbaume Step Functions service.

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

const SIMPLE_DEFINITION: &str = r#"{"Comment": "An example of the Amazon States Language using a choice state.","StartAt": "DefaultState","States": {"DefaultState": {"Type": "Fail","Error": "DefaultStateError","Cause": "No Matches!"}}}"#;

fn default_role() -> String {
    format!("arn:aws:iam::{ACCOUNT_ID}:role/unknown_sf_role")
}

// =============================================================================
// State Machine Creation tests (from moto: test_state_machine_creation_succeeds)
// =============================================================================

#[tokio::test]
async fn test_state_machine_creation_succeeds() {
    let client = make_sfn_client().await;
    let name = "example_step_function";

    let resp = client
        .create_state_machine()
        .name(name)
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .expect("create_state_machine should succeed");

    assert_eq!(
        resp.state_machine_arn(),
        &format!("arn:aws:states:us-east-1:{ACCOUNT_ID}:stateMachine:{name}")
    );
    // creation_date should be populated
    let _ = resp.creation_date();
}

// From moto: test_state_machine_list_returns_empty_list_by_default
#[tokio::test]
async fn test_state_machine_list_returns_empty_list_by_default() {
    let client = make_sfn_client().await;

    let resp = client.list_state_machines().send().await.unwrap();
    assert_eq!(resp.state_machines().len(), 0);
}

// From moto: test_state_machine_list_returns_created_state_machines
#[tokio::test]
async fn test_state_machine_list_returns_created_state_machines() {
    let client = make_sfn_client().await;

    let m1 = client
        .create_state_machine()
        .name("name1")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let m2 = client
        .create_state_machine()
        .name("name2")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let sm_list = client.list_state_machines().send().await.unwrap();
    assert_eq!(sm_list.state_machines().len(), 2);

    let names: Vec<&str> = sm_list
        .state_machines()
        .iter()
        .map(|sm| sm.name())
        .collect();
    assert!(names.contains(&"name1"));
    assert!(names.contains(&"name2"));

    let arns: Vec<&str> = sm_list
        .state_machines()
        .iter()
        .map(|sm| sm.state_machine_arn())
        .collect();
    assert!(arns.contains(&m1.state_machine_arn()));
    assert!(arns.contains(&m2.state_machine_arn()));
}

// From moto: test_state_machine_creation_is_idempotent_by_name
#[tokio::test]
async fn test_state_machine_creation_is_not_idempotent() {
    let client = make_sfn_client().await;

    client
        .create_state_machine()
        .name("name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let sm_list = client.list_state_machines().send().await.unwrap();
    assert_eq!(sm_list.state_machines().len(), 1);

    // Creating with same name should fail (StateMachineAlreadyExists)
    let result = client
        .create_state_machine()
        .name("name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await;
    assert!(result.is_err());

    // Creating with different name should succeed
    client
        .create_state_machine()
        .name("diff_name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let sm_list = client.list_state_machines().send().await.unwrap();
    assert_eq!(sm_list.state_machines().len(), 2);
}

// =============================================================================
// Describe State Machine tests (from moto: test_state_machine_creation_can_be_described)
// =============================================================================

#[tokio::test]
async fn test_state_machine_creation_can_be_described() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_state_machine()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .unwrap();

    assert_eq!(desc.definition(), SIMPLE_DEFINITION);
    assert_eq!(desc.name(), "name");
    assert_eq!(desc.role_arn(), default_role());
    assert_eq!(desc.state_machine_arn(), sm.state_machine_arn());
    assert_eq!(
        desc.status(),
        Some(&aws_sdk_sfn::types::StateMachineStatus::Active)
    );
    assert_eq!(
        desc.r#type(),
        &aws_sdk_sfn::types::StateMachineType::Standard
    );
}

// From moto: test_state_machine_throws_error_when_describing_unknown_machine
#[tokio::test]
async fn test_state_machine_throws_error_when_describing_unknown_machine() {
    let client = make_sfn_client().await;

    let unknown = format!("arn:aws:states:us-east-1:{ACCOUNT_ID}:stateMachine:unknown");
    let result = client
        .describe_state_machine()
        .state_machine_arn(&unknown)
        .send()
        .await;
    assert!(result.is_err());
}

// =============================================================================
// Delete State Machine tests (from moto: test_state_machine_can_be_deleted)
// =============================================================================

#[tokio::test]
async fn test_state_machine_can_be_deleted() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    client
        .delete_state_machine()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .expect("delete_state_machine should succeed");

    let sm_list = client.list_state_machines().send().await.unwrap();
    assert_eq!(sm_list.state_machines().len(), 0);
}

// From moto: test_state_machine_can_deleted_nonexisting_machine
#[tokio::test]
async fn test_state_machine_can_delete_nonexisting_machine() {
    let client = make_sfn_client().await;

    let unknown = format!("arn:aws:states:us-east-1:{ACCOUNT_ID}:stateMachine:unknown");
    client
        .delete_state_machine()
        .state_machine_arn(&unknown)
        .send()
        .await
        .expect("delete non-existing state machine should succeed (idempotent)");

    let sm_list = client.list_state_machines().send().await.unwrap();
    assert_eq!(sm_list.state_machines().len(), 0);
}

// =============================================================================
// Tagging tests (from moto: test_state_machine_tagging, test_state_machine_untagging)
// =============================================================================

#[tokio::test]
async fn test_state_machine_tagging() {
    let client = make_sfn_client().await;

    let tags = vec![
        aws_sdk_sfn::types::Tag::builder()
            .key("tag_key1")
            .value("tag_value1")
            .build(),
        aws_sdk_sfn::types::Tag::builder()
            .key("tag_key2")
            .value("tag_value2")
            .build(),
    ];

    let machine = client
        .create_state_machine()
        .name("test-with-tags")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .set_tags(Some(tags))
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(machine.state_machine_arn())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 2);

    let keys: Vec<&str> = resp.tags().iter().map(|t| t.key().unwrap_or("")).collect();
    assert!(keys.contains(&"tag_key1"));
    assert!(keys.contains(&"tag_key2"));
}

#[tokio::test]
async fn test_state_machine_tag_resource() {
    let client = make_sfn_client().await;

    let machine = client
        .create_state_machine()
        .name("test")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();
    let sm_arn = machine.state_machine_arn();

    // Add tags after creation
    let tags = vec![
        aws_sdk_sfn::types::Tag::builder()
            .key("tag_key1")
            .value("tag_value1")
            .build(),
        aws_sdk_sfn::types::Tag::builder()
            .key("tag_key2")
            .value("tag_value2")
            .build(),
    ];

    client
        .tag_resource()
        .resource_arn(sm_arn)
        .set_tags(Some(tags))
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(sm_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 2);

    // Update tag and add new one
    let tags_update = vec![
        aws_sdk_sfn::types::Tag::builder()
            .key("tag_key1")
            .value("tag_value1_new")
            .build(),
        aws_sdk_sfn::types::Tag::builder()
            .key("tag_key3")
            .value("tag_value3")
            .build(),
    ];

    client
        .tag_resource()
        .resource_arn(sm_arn)
        .set_tags(Some(tags_update))
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(sm_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 3);

    let tag1 = resp
        .tags()
        .iter()
        .find(|t| t.key() == Some("tag_key1"))
        .unwrap();
    assert_eq!(tag1.value(), Some("tag_value1_new"));
}

// From moto: test_state_machine_untagging
#[tokio::test]
async fn test_state_machine_untagging() {
    let client = make_sfn_client().await;

    let tags = vec![
        aws_sdk_sfn::types::Tag::builder()
            .key("tag_key1")
            .value("tag_value1")
            .build(),
        aws_sdk_sfn::types::Tag::builder()
            .key("tag_key2")
            .value("tag_value2")
            .build(),
        aws_sdk_sfn::types::Tag::builder()
            .key("tag_key3")
            .value("tag_value3")
            .build(),
    ];

    let machine = client
        .create_state_machine()
        .name("test")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .set_tags(Some(tags))
        .send()
        .await
        .unwrap();
    let sm_arn = machine.state_machine_arn();

    client
        .untag_resource()
        .resource_arn(sm_arn)
        .tag_keys("tag_key1")
        .tag_keys("tag_key2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(sm_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), Some("tag_key3"));
    assert_eq!(resp.tags()[0].value(), Some("tag_value3"));
}

// From moto: test_state_machine_list_tags_for_machine_without_tags
#[tokio::test]
async fn test_state_machine_list_tags_for_machine_without_tags() {
    let client = make_sfn_client().await;

    let machine = client
        .create_state_machine()
        .name("name1")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(machine.state_machine_arn())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 0);
}

// From moto: test_state_machine_list_tags_for_nonexisting_machine
#[tokio::test]
async fn test_state_machine_list_tags_for_nonexisting_machine() {
    let client = make_sfn_client().await;

    let unknown = format!("arn:aws:states:us-east-1:{ACCOUNT_ID}:stateMachine:unknown");
    let resp = client
        .list_tags_for_resource()
        .resource_arn(&unknown)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 0);
}

// =============================================================================
// Start Execution tests (from moto: test_state_machine_start_execution)
// =============================================================================

#[tokio::test]
async fn test_state_machine_start_execution() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let exec = client
        .start_execution()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .unwrap();

    assert!(!exec.execution_arn().is_empty());
    assert!(exec.execution_arn().starts_with(&format!(
        "arn:aws:states:us-east-1:{ACCOUNT_ID}:execution:name:"
    )));
    let _ = exec.start_date();
}

// From moto: test_state_machine_start_execution_with_custom_name
#[tokio::test]
async fn test_state_machine_start_execution_with_custom_name() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let exec = client
        .start_execution()
        .state_machine_arn(sm.state_machine_arn())
        .name("execution_name")
        .send()
        .await
        .unwrap();

    let expected_arn =
        format!("arn:aws:states:us-east-1:{ACCOUNT_ID}:execution:name:execution_name");
    assert_eq!(exec.execution_arn(), &expected_arn);
}

// From moto: test_state_machine_start_execution_fails_on_duplicate_execution_name_with_different_input
#[tokio::test]
async fn test_start_execution_fails_on_duplicate_name_with_different_input() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    client
        .start_execution()
        .state_machine_arn(sm.state_machine_arn())
        .name("execution_name")
        .input(r#"{"a": "b", "c": "d"}"#)
        .send()
        .await
        .unwrap();

    // Different input with same name should fail
    let result = client
        .start_execution()
        .state_machine_arn(sm.state_machine_arn())
        .name("execution_name")
        .input(r#"{"c": "d", "a": "b"}"#)
        .send()
        .await;

    assert!(result.is_err());
}

// From moto: test_state_machine_start_execution_is_idempotent_by_name_and_input
#[tokio::test]
async fn test_start_execution_is_idempotent_by_name_and_input() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let execution_input = r#"{"a": "b", "c": "d"}"#;

    let exec1 = client
        .start_execution()
        .state_machine_arn(sm.state_machine_arn())
        .name("execution_name")
        .input(execution_input)
        .send()
        .await
        .unwrap();

    let exec2 = client
        .start_execution()
        .state_machine_arn(sm.state_machine_arn())
        .name("execution_name")
        .input(execution_input)
        .send()
        .await
        .unwrap();

    assert_eq!(exec1.execution_arn(), exec2.execution_arn());

    // Check idempotency - should still be only 1 execution
    let list = client
        .list_executions()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .unwrap();
    assert_eq!(list.executions().len(), 1);
}

// =============================================================================
// Describe Execution tests (from moto: test_state_machine_describe_execution_with_no_input)
// =============================================================================

#[tokio::test]
async fn test_state_machine_describe_execution_with_no_input() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let exec = client
        .start_execution()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_execution()
        .execution_arn(exec.execution_arn())
        .send()
        .await
        .unwrap();

    assert_eq!(desc.execution_arn(), exec.execution_arn());
    // Default input should be "{}" when not specified
    assert_eq!(desc.input(), Some("{}"));
    assert_eq!(desc.state_machine_arn(), sm.state_machine_arn());
    assert_eq!(desc.status(), &aws_sdk_sfn::types::ExecutionStatus::Running);
    assert!(desc.stop_date().is_none());
}

// From moto: test_state_machine_describe_execution_with_custom_input
#[tokio::test]
async fn test_state_machine_describe_execution_with_custom_input() {
    let client = make_sfn_client().await;

    let execution_input = r#"{"input_key": "input_val"}"#;

    let sm = client
        .create_state_machine()
        .name("name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let exec = client
        .start_execution()
        .state_machine_arn(sm.state_machine_arn())
        .input(execution_input)
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_execution()
        .execution_arn(exec.execution_arn())
        .send()
        .await
        .unwrap();

    assert_eq!(desc.execution_arn(), exec.execution_arn());
    assert_eq!(desc.input(), Some(execution_input));
    assert_eq!(desc.state_machine_arn(), sm.state_machine_arn());
    assert_eq!(desc.status(), &aws_sdk_sfn::types::ExecutionStatus::Running);
    assert!(desc.stop_date().is_none());
}

// From moto: test_execution_throws_error_when_describing_unknown_execution
#[tokio::test]
async fn test_execution_throws_error_when_describing_unknown_execution() {
    let client = make_sfn_client().await;

    let unknown = format!("arn:aws:states:us-east-1:{ACCOUNT_ID}:execution:unknown");
    let result = client
        .describe_execution()
        .execution_arn(&unknown)
        .send()
        .await;
    assert!(result.is_err());
}

// =============================================================================
// List Executions tests (from moto: test_state_machine_list_executions)
// =============================================================================

#[tokio::test]
async fn test_state_machine_list_executions() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let exec = client
        .start_execution()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .unwrap();

    let exec_arn = exec.execution_arn();
    let exec_name = exec_arn.rsplit(':').next().unwrap();

    let executions = client
        .list_executions()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .unwrap();

    assert_eq!(executions.executions().len(), 1);
    assert_eq!(executions.executions()[0].execution_arn(), exec_arn);
    assert_eq!(executions.executions()[0].name(), exec_name);
    assert_eq!(
        executions.executions()[0].state_machine_arn(),
        sm.state_machine_arn()
    );
    assert_eq!(
        executions.executions()[0].status(),
        &aws_sdk_sfn::types::ExecutionStatus::Running
    );
    assert!(executions.executions()[0].stop_date().is_none());
}

// From moto: test_state_machine_list_executions_when_none_exist
#[tokio::test]
async fn test_state_machine_list_executions_when_none_exist() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let executions = client
        .list_executions()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .unwrap();

    assert_eq!(executions.executions().len(), 0);
}

// =============================================================================
// Stop Execution tests (from moto: test_state_machine_stop_execution)
// =============================================================================

#[tokio::test]
async fn test_state_machine_stop_execution() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let exec = client
        .start_execution()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .unwrap();

    let stop = client
        .stop_execution()
        .execution_arn(exec.execution_arn())
        .send()
        .await
        .expect("stop_execution should succeed");

    // stop_date is always set on StopExecutionOutput
    let _ = stop.stop_date();

    // Describe should show ABORTED status
    let desc = client
        .describe_execution()
        .execution_arn(exec.execution_arn())
        .send()
        .await
        .unwrap();
    assert_eq!(desc.status(), &aws_sdk_sfn::types::ExecutionStatus::Aborted);
    assert!(desc.stop_date().is_some());

    // List executions should also show stop date
    let executions = client
        .list_executions()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .unwrap();
    assert!(executions.executions()[0].stop_date().is_some());
}

// From moto: test_state_machine_stop_raises_error_when_unknown_execution
#[tokio::test]
async fn test_state_machine_stop_raises_error_when_unknown_execution() {
    let client = make_sfn_client().await;

    client
        .create_state_machine()
        .name("test-state-machine")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let unknown =
        format!("arn:aws:states:us-east-1:{ACCOUNT_ID}:execution:test-state-machine:unknown");
    let result = client.stop_execution().execution_arn(&unknown).send().await;
    assert!(result.is_err());
}

// =============================================================================
// Activity tests
// =============================================================================

#[tokio::test]
async fn test_create_activity() {
    let client = make_sfn_client().await;

    let resp = client
        .create_activity()
        .name("my-activity")
        .send()
        .await
        .expect("create_activity should succeed");

    assert_eq!(
        resp.activity_arn(),
        &format!("arn:aws:states:us-east-1:{ACCOUNT_ID}:activity:my-activity")
    );
    let _ = resp.creation_date();
}

#[tokio::test]
async fn test_describe_activity() {
    let client = make_sfn_client().await;

    let created = client
        .create_activity()
        .name("my-activity")
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_activity()
        .activity_arn(created.activity_arn())
        .send()
        .await
        .expect("describe_activity should succeed");

    assert_eq!(desc.activity_arn(), created.activity_arn());
    assert_eq!(desc.name(), "my-activity");
}

#[tokio::test]
async fn test_describe_activity_unknown() {
    let client = make_sfn_client().await;

    let unknown = format!("arn:aws:states:us-east-1:{ACCOUNT_ID}:activity:unknown");
    let result = client
        .describe_activity()
        .activity_arn(&unknown)
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_activity() {
    let client = make_sfn_client().await;

    let created = client
        .create_activity()
        .name("my-activity")
        .send()
        .await
        .unwrap();

    client
        .delete_activity()
        .activity_arn(created.activity_arn())
        .send()
        .await
        .expect("delete_activity should succeed");

    let list = client.list_activities().send().await.unwrap();
    assert_eq!(list.activities().len(), 0);
}

#[tokio::test]
async fn test_delete_activity_idempotent() {
    let client = make_sfn_client().await;

    let unknown = format!("arn:aws:states:us-east-1:{ACCOUNT_ID}:activity:unknown");
    client
        .delete_activity()
        .activity_arn(&unknown)
        .send()
        .await
        .expect("delete non-existing activity should succeed (idempotent)");
}

#[tokio::test]
async fn test_list_activities() {
    let client = make_sfn_client().await;

    // Empty initially
    let list = client.list_activities().send().await.unwrap();
    assert_eq!(list.activities().len(), 0);

    client.create_activity().name("act1").send().await.unwrap();
    client.create_activity().name("act2").send().await.unwrap();

    let list = client.list_activities().send().await.unwrap();
    assert_eq!(list.activities().len(), 2);

    let names: Vec<&str> = list.activities().iter().map(|a| a.name()).collect();
    assert!(names.contains(&"act1"));
    assert!(names.contains(&"act2"));
}

// =============================================================================
// Task callback tests
// =============================================================================

#[tokio::test]
async fn test_send_task_success_unknown_token() {
    let client = make_sfn_client().await;

    let result = client
        .send_task_success()
        .task_token("unknown-token")
        .output("{}")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_send_task_failure_unknown_token() {
    let client = make_sfn_client().await;

    let result = client
        .send_task_failure()
        .task_token("unknown-token")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_send_task_heartbeat_unknown_token() {
    let client = make_sfn_client().await;

    let result = client
        .send_task_heartbeat()
        .task_token("unknown-token")
        .send()
        .await;
    assert!(result.is_err());
}

// =============================================================================
// GetExecutionHistory tests
// =============================================================================

#[tokio::test]
async fn test_get_execution_history() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let exec = client
        .start_execution()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .unwrap();

    let history = client
        .get_execution_history()
        .execution_arn(exec.execution_arn())
        .send()
        .await
        .expect("get_execution_history should succeed");

    // Execution generates 4 default history events (matching moto behavior)
    assert_eq!(history.events().len(), 4);
    // First event should be ExecutionStarted
    assert_eq!(history.events()[0].r#type().as_str(), "ExecutionStarted");
    // Last event should be ExecutionSucceeded
    assert_eq!(history.events()[3].r#type().as_str(), "ExecutionSucceeded");
}

#[tokio::test]
async fn test_get_execution_history_unknown_execution() {
    let client = make_sfn_client().await;

    let unknown = format!("arn:aws:states:us-east-1:{ACCOUNT_ID}:execution:unknown:unknown");
    let result = client
        .get_execution_history()
        .execution_arn(&unknown)
        .send()
        .await;
    assert!(result.is_err());
}

// =============================================================================
// Map run tests
// =============================================================================

#[tokio::test]
async fn test_list_map_runs_empty() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let exec = client
        .start_execution()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .unwrap();

    let runs = client
        .list_map_runs()
        .execution_arn(exec.execution_arn())
        .send()
        .await
        .expect("list_map_runs should succeed");

    assert_eq!(runs.map_runs().len(), 0);
}

#[tokio::test]
async fn test_list_map_runs_unknown_execution() {
    let client = make_sfn_client().await;

    let unknown = format!("arn:aws:states:us-east-1:{ACCOUNT_ID}:execution:unknown:unknown");
    let result = client.list_map_runs().execution_arn(&unknown).send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_map_run_unknown() {
    let client = make_sfn_client().await;

    let unknown = format!("arn:aws:states:us-east-1:{ACCOUNT_ID}:mapRun:unknown:unknown");
    let result = client.describe_map_run().map_run_arn(&unknown).send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_map_run_unknown() {
    let client = make_sfn_client().await;

    let unknown = format!("arn:aws:states:us-east-1:{ACCOUNT_ID}:mapRun:unknown:unknown");
    let result = client.update_map_run().map_run_arn(&unknown).send().await;
    assert!(result.is_err());
}

// =============================================================================
// DescribeStateMachineForExecution tests
// =============================================================================

#[tokio::test]
async fn test_describe_state_machine_for_execution() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let exec = client
        .start_execution()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_state_machine_for_execution()
        .execution_arn(exec.execution_arn())
        .send()
        .await
        .expect("describe_state_machine_for_execution should succeed");

    assert_eq!(desc.state_machine_arn(), sm.state_machine_arn());
    assert_eq!(desc.name(), "name");
    assert_eq!(desc.definition(), SIMPLE_DEFINITION);
    assert_eq!(desc.role_arn(), default_role());
}

#[tokio::test]
async fn test_describe_state_machine_for_execution_unknown() {
    let client = make_sfn_client().await;

    let unknown = format!("arn:aws:states:us-east-1:{ACCOUNT_ID}:execution:unknown:unknown");
    let result = client
        .describe_state_machine_for_execution()
        .execution_arn(&unknown)
        .send()
        .await;
    assert!(result.is_err());
}

// =============================================================================
// Update State Machine tests (from moto: test_update_state_machine - simplified)
// =============================================================================

#[tokio::test]
async fn test_update_state_machine() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("test")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();
    let sm_arn = sm.state_machine_arn();

    let updated_role = format!("{}-updated", default_role());
    let updated_definition = SIMPLE_DEFINITION.replace("DefaultState", "DefaultStateUpdated");

    client
        .update_state_machine()
        .state_machine_arn(sm_arn)
        .definition(&updated_definition)
        .role_arn(&updated_role)
        .send()
        .await
        .expect("update_state_machine should succeed");

    let desc = client
        .describe_state_machine()
        .state_machine_arn(sm_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(desc.definition(), updated_definition);
    assert_eq!(desc.role_arn(), updated_role);
}

// ============================================================================
// Ported from moto: test_stepfunctions.py
// ============================================================================

// Ported from moto: test_stepfunctions.py::test_create_activity_with_duplicate_name
#[tokio::test]
async fn test_create_activity_with_duplicate_name() {
    let client = make_sfn_client().await;

    client
        .create_activity()
        .name("test-activity")
        .send()
        .await
        .unwrap();

    // Duplicate name should error
    let err = client
        .create_activity()
        .name("test-activity")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ActivityAlreadyExists"),
        "Expected ActivityAlreadyExists error, got: {err_str}"
    );
}

// Ported from moto: test_stepfunctions.py::test_delete_activity (verifies describe fails after delete)
#[tokio::test]
async fn test_delete_activity_then_describe_fails() {
    let client = make_sfn_client().await;

    let created = client
        .create_activity()
        .name("test-activity")
        .send()
        .await
        .unwrap();

    // Describe should succeed before delete
    let desc = client
        .describe_activity()
        .activity_arn(created.activity_arn())
        .send()
        .await
        .unwrap();
    assert_eq!(desc.name(), "test-activity");

    // Delete the activity
    client
        .delete_activity()
        .activity_arn(created.activity_arn())
        .send()
        .await
        .unwrap();

    // Describe should fail after delete
    let err = client
        .describe_activity()
        .activity_arn(created.activity_arn())
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ActivityDoesNotExist"),
        "Expected ActivityDoesNotExist error, got: {err_str}"
    );
}

// Ported from moto: test_stepfunctions.py::test_list_activities_returns_created_activities
#[tokio::test]
async fn test_list_activities_returns_created_activities() {
    let client = make_sfn_client().await;

    let a1 = client
        .create_activity()
        .name("test-activity-1")
        .send()
        .await
        .unwrap();

    let a2 = client
        .create_activity()
        .name("test-activity-2")
        .send()
        .await
        .unwrap();

    let activities = client.list_activities().send().await.unwrap();
    assert_eq!(activities.activities().len(), 2);

    let arns: Vec<&str> = activities
        .activities()
        .iter()
        .map(|a| a.activity_arn())
        .collect();
    assert!(arns.contains(&a1.activity_arn()));
    assert!(arns.contains(&a2.activity_arn()));
}

// Ported from moto: test_stepfunctions.py::test_activity_tagging
#[tokio::test]
async fn test_activity_tagging() {
    let client = make_sfn_client().await;

    // Test tags are added on resource creation
    let tags = vec![
        aws_sdk_sfn::types::Tag::builder()
            .key("tag_key1")
            .value("tag_value1")
            .build(),
        aws_sdk_sfn::types::Tag::builder()
            .key("tag_key2")
            .value("tag_value2")
            .build(),
    ];
    let activity = client
        .create_activity()
        .name("test-with-tags")
        .set_tags(Some(tags))
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(activity.activity_arn())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 2);

    let keys: Vec<&str> = resp.tags().iter().map(|t| t.key().unwrap_or("")).collect();
    assert!(keys.contains(&"tag_key1"));
    assert!(keys.contains(&"tag_key2"));

    // Test tags are added after creation with tag_resource
    let activity2 = client
        .create_activity()
        .name("test-activity")
        .send()
        .await
        .unwrap();

    let tags2 = vec![
        aws_sdk_sfn::types::Tag::builder()
            .key("tag_key1")
            .value("tag_value1")
            .build(),
        aws_sdk_sfn::types::Tag::builder()
            .key("tag_key2")
            .value("tag_value2")
            .build(),
    ];
    client
        .tag_resource()
        .resource_arn(activity2.activity_arn())
        .set_tags(Some(tags2))
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(activity2.activity_arn())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 2);

    // Update tag and add new one
    let tags_update = vec![
        aws_sdk_sfn::types::Tag::builder()
            .key("tag_key1")
            .value("tag_value1_new")
            .build(),
        aws_sdk_sfn::types::Tag::builder()
            .key("tag_key3")
            .value("tag_value3")
            .build(),
    ];
    client
        .tag_resource()
        .resource_arn(activity2.activity_arn())
        .set_tags(Some(tags_update))
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(activity2.activity_arn())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 3);

    let tag1 = resp
        .tags()
        .iter()
        .find(|t| t.key() == Some("tag_key1"))
        .unwrap();
    assert_eq!(tag1.value(), Some("tag_value1_new"));
}

// Ported from moto: test_stepfunctions.py::test_activity_untagging
#[tokio::test]
async fn test_activity_untagging() {
    let client = make_sfn_client().await;

    let tags = vec![
        aws_sdk_sfn::types::Tag::builder()
            .key("tag_key1")
            .value("tag_value1")
            .build(),
        aws_sdk_sfn::types::Tag::builder()
            .key("tag_key2")
            .value("tag_value2")
            .build(),
        aws_sdk_sfn::types::Tag::builder()
            .key("tag_key3")
            .value("tag_value3")
            .build(),
    ];

    let activity = client
        .create_activity()
        .name("test-activity")
        .set_tags(Some(tags))
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(activity.activity_arn())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 3);

    client
        .untag_resource()
        .resource_arn(activity.activity_arn())
        .tag_keys("tag_key1")
        .tag_keys("tag_key2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(activity.activity_arn())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), Some("tag_key3"));
    assert_eq!(resp.tags()[0].value(), Some("tag_value3"));
}

// Ported from moto: test_stepfunctions.py::test_activity_list_tags_for_created_activity
#[tokio::test]
async fn test_activity_list_tags_for_created_activity() {
    let client = make_sfn_client().await;

    let activity = client
        .create_activity()
        .name("test-activity")
        .tags(
            aws_sdk_sfn::types::Tag::builder()
                .key("tag_key")
                .value("tag_value")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(activity.activity_arn())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), Some("tag_key"));
    assert_eq!(resp.tags()[0].value(), Some("tag_value"));
}

// Ported from moto: test_stepfunctions.py::test_activity_list_tags_for_activity_without_tags
#[tokio::test]
async fn test_activity_list_tags_for_activity_without_tags() {
    let client = make_sfn_client().await;

    let activity = client
        .create_activity()
        .name("test-activity")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(activity.activity_arn())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 0);
}

// Ported from moto: test_stepfunctions.py::test_activity_list_tags_for_nonexisting_activity
#[tokio::test]
async fn test_activity_list_tags_for_nonexisting_activity() {
    let client = make_sfn_client().await;

    let non_existing = format!("arn:aws:states:us-east-1:{ACCOUNT_ID}:activity:unknown");
    let resp = client
        .list_tags_for_resource()
        .resource_arn(&non_existing)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 0);
}

// Ported from moto: test_stepfunctions.py::test_state_machine_start_execution_bad_arn_raises_exception
#[tokio::test]
async fn test_state_machine_start_execution_bad_arn_raises_exception() {
    let client = make_sfn_client().await;

    let result = client
        .start_execution()
        .state_machine_arn("bad")
        .send()
        .await;
    assert!(result.is_err());
}

// Ported from moto: test_stepfunctions.py::test_state_machine_start_execution_with_custom_input
#[tokio::test]
async fn test_state_machine_start_execution_with_custom_input() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let execution_input = r#"{"input_key": "input_value"}"#;
    let exec = client
        .start_execution()
        .state_machine_arn(sm.state_machine_arn())
        .input(execution_input)
        .send()
        .await
        .unwrap();

    // Verify execution ARN format
    assert!(exec.execution_arn().starts_with(&format!(
        "arn:aws:states:us-east-1:{ACCOUNT_ID}:execution:name:"
    )));
}

// Ported from moto: test_stepfunctions.py::test_state_machine_get_execution_history_contains_expected_success_events_when_started
#[tokio::test]
async fn test_execution_history_contains_expected_events() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("test-state-machine")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let exec = client
        .start_execution()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .unwrap();

    let history = client
        .get_execution_history()
        .execution_arn(exec.execution_arn())
        .send()
        .await
        .unwrap();

    assert_eq!(history.events().len(), 4);

    // Verify event types in order
    let event_types: Vec<&str> = history
        .events()
        .iter()
        .map(|e| e.r#type().as_str())
        .collect();
    assert_eq!(event_types[0], "ExecutionStarted");
    assert_eq!(event_types[1], "PassStateEntered");
    assert_eq!(event_types[2], "PassStateExited");
    assert_eq!(event_types[3], "ExecutionSucceeded");

    // Verify event IDs
    assert_eq!(history.events()[0].id(), 1);
    assert_eq!(history.events()[1].id(), 2);
    assert_eq!(history.events()[2].id(), 3);
    assert_eq!(history.events()[3].id(), 4);

    // Verify previous event IDs
    assert_eq!(history.events()[0].previous_event_id(), 0);
    assert_eq!(history.events()[1].previous_event_id(), 0);
    assert_eq!(history.events()[2].previous_event_id(), 2);
    assert_eq!(history.events()[3].previous_event_id(), 3);
}

// Ported from moto: test_stepfunctions.py::test_state_machine_get_execution_history_throws_error_with_unknown_execution
#[tokio::test]
async fn test_execution_history_throws_error_with_unknown_execution() {
    let client = make_sfn_client().await;

    client
        .create_state_machine()
        .name("test-state-machine")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let unknown =
        format!("arn:aws:states:us-east-1:{ACCOUNT_ID}:execution:test-state-machine:unknown");
    let err = client
        .get_execution_history()
        .execution_arn(&unknown)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ExecutionDoesNotExist"),
        "Expected ExecutionDoesNotExist error, got: {err_str}"
    );
}

// Ported from moto: test_stepfunctions.py::test_state_machine_stop_raises_error_when_unknown_execution (extended error check)
#[tokio::test]
async fn test_stop_execution_error_message() {
    let client = make_sfn_client().await;

    client
        .create_state_machine()
        .name("test-state-machine")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let unknown =
        format!("arn:aws:states:us-east-1:{ACCOUNT_ID}:execution:test-state-machine:unknown");
    let err = client
        .stop_execution()
        .execution_arn(&unknown)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ExecutionDoesNotExist"),
        "Expected ExecutionDoesNotExist error, got: {err_str}"
    );
    assert!(
        err_str.contains("Execution Does Not Exist:"),
        "Expected 'Execution Does Not Exist:' message, got: {err_str}"
    );
}

// Ported from moto: test_stepfunctions.py::test_state_machine_list_tags_for_created_machine
#[tokio::test]
async fn test_state_machine_list_tags_for_created_machine() {
    let client = make_sfn_client().await;

    let machine = client
        .create_state_machine()
        .name("name1")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .tags(
            aws_sdk_sfn::types::Tag::builder()
                .key("tag_key")
                .value("tag_value")
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(machine.state_machine_arn())
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 1);
    assert_eq!(resp.tags()[0].key(), Some("tag_key"));
    assert_eq!(resp.tags()[0].value(), Some("tag_value"));
}

// Ported from moto: test_stepfunctions.py::test_state_machine_can_be_described_by_execution
#[tokio::test]
async fn test_state_machine_can_be_described_by_execution() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let exec = client
        .start_execution()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .unwrap();

    let desc = client
        .describe_state_machine_for_execution()
        .execution_arn(exec.execution_arn())
        .send()
        .await
        .unwrap();

    assert_eq!(desc.definition(), SIMPLE_DEFINITION);
    assert_eq!(desc.name(), "name");
    assert_eq!(desc.role_arn(), default_role());
    assert_eq!(desc.state_machine_arn(), sm.state_machine_arn());
}

// Ported from moto: test_stepfunctions.py::test_state_machine_creation_fails_with_invalid_names (simplified - a few representative invalid names)
#[tokio::test]
async fn test_state_machine_creation_is_idempotent_by_name_error() {
    let client = make_sfn_client().await;

    client
        .create_state_machine()
        .name("test")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    let err = client
        .create_state_machine()
        .name("test")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("StateMachineAlreadyExists"),
        "Expected StateMachineAlreadyExists error, got: {err_str}"
    );
}

// Ported from moto: test_stepfunctions.py::test_state_machine_start_execution_fails_on_duplicate_execution_name_with_different_input (error message check)
#[tokio::test]
async fn test_start_execution_duplicate_name_error_message() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("name")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .unwrap();

    client
        .start_execution()
        .state_machine_arn(sm.state_machine_arn())
        .name("execution_name")
        .input(r#"{"a": "b", "c": "d"}"#)
        .send()
        .await
        .unwrap();

    let err = client
        .start_execution()
        .state_machine_arn(sm.state_machine_arn())
        .name("execution_name")
        .input(r#"{"c": "d", "a": "b"}"#)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ExecutionAlreadyExists"),
        "Expected ExecutionAlreadyExists error, got: {err_str}"
    );
    assert!(
        err_str.contains("Execution Already Exists:"),
        "Expected 'Execution Already Exists:' message, got: {err_str}"
    );
}

// ============================================================================
// Coverage for FIX(terraform-e2e) handler fixes
// ============================================================================

// Covers FIX(terraform-e2e): ValidateStateMachineDefinition returns success
// (handlers.rs:113) — terraform provider calls ValidateStateMachineDefinition
// before CreateStateMachine. Return an empty-result success response.
#[tokio::test]
async fn test_validate_state_machine_definition_returns_ok() {
    let client = make_sfn_client().await;

    let resp = client
        .validate_state_machine_definition()
        .definition(SIMPLE_DEFINITION)
        .send()
        .await
        .expect("validate_state_machine_definition should succeed");

    assert_eq!(
        resp.result(),
        &aws_sdk_sfn::types::ValidateStateMachineDefinitionResultCode::Ok
    );
    assert!(resp.diagnostics().is_empty(), "diagnostics should be empty");
}

// Covers FIX(terraform-e2e): ListStateMachineVersions returns empty list
// (handlers.rs:119) — terraform provider calls ListStateMachineVersions after
// CreateStateMachine. Return an empty list.
#[tokio::test]
async fn test_list_state_machine_versions_returns_empty() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("versions_test")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .expect("create_state_machine should succeed");

    let resp = client
        .list_state_machine_versions()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .expect("list_state_machine_versions should succeed");

    assert!(
        resp.state_machine_versions().is_empty(),
        "versions list should be empty"
    );
}

// =============================================================================
// PublishStateMachineVersion / DeleteStateMachineVersion tests
// =============================================================================

#[tokio::test]
async fn test_publish_state_machine_version() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("publish_ver_test")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .expect("create should succeed");

    let resp = client
        .publish_state_machine_version()
        .state_machine_arn(sm.state_machine_arn())
        .description("first version")
        .send()
        .await
        .expect("publish_state_machine_version should succeed");

    let version_arn = resp.state_machine_version_arn();
    assert!(
        version_arn.contains(":stateMachine:publish_ver_test:"),
        "version ARN should contain the state machine name"
    );
    let _ = resp.creation_date(); // non-optional, just confirm it exists

    // Verify it appears in the versions list
    let list_resp = client
        .list_state_machine_versions()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .expect("list_state_machine_versions should succeed");
    assert_eq!(list_resp.state_machine_versions().len(), 1);
}

#[tokio::test]
async fn test_delete_state_machine_version() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("delete_ver_test")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .expect("create should succeed");

    let published = client
        .publish_state_machine_version()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .expect("publish should succeed");

    // Delete the version
    client
        .delete_state_machine_version()
        .state_machine_version_arn(published.state_machine_version_arn())
        .send()
        .await
        .expect("delete_state_machine_version should succeed");

    // Verify it no longer appears in the list
    let list_resp = client
        .list_state_machine_versions()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .expect("list_state_machine_versions should succeed");
    assert!(
        list_resp.state_machine_versions().is_empty(),
        "versions list should be empty after deletion"
    );
}

// =============================================================================
// State Machine Alias CRUD tests
// =============================================================================

#[tokio::test]
async fn test_create_and_describe_state_machine_alias() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("alias_test")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .expect("create should succeed");

    let published = client
        .publish_state_machine_version()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .expect("publish should succeed");

    let routing_item = aws_sdk_sfn::types::RoutingConfigurationListItem::builder()
        .state_machine_version_arn(published.state_machine_version_arn())
        .weight(100)
        .build()
        .expect("routing config should build");

    let create_resp = client
        .create_state_machine_alias()
        .name("prod")
        .description("production alias")
        .routing_configuration(routing_item)
        .send()
        .await
        .expect("create_state_machine_alias should succeed");

    let alias_arn = create_resp.state_machine_alias_arn();
    assert!(
        alias_arn.contains(":stateMachine:alias_test:prod"),
        "alias ARN should contain the alias name"
    );

    // Describe the alias
    let desc_resp = client
        .describe_state_machine_alias()
        .state_machine_alias_arn(alias_arn)
        .send()
        .await
        .expect("describe_state_machine_alias should succeed");

    assert_eq!(desc_resp.name(), Some("prod"));
    assert_eq!(desc_resp.description(), Some("production alias"));
    assert_eq!(desc_resp.routing_configuration().len(), 1);
}

#[tokio::test]
async fn test_list_state_machine_aliases() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("alias_list_test")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .expect("create should succeed");

    let published = client
        .publish_state_machine_version()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .expect("publish should succeed");

    let routing_item = aws_sdk_sfn::types::RoutingConfigurationListItem::builder()
        .state_machine_version_arn(published.state_machine_version_arn())
        .weight(100)
        .build()
        .expect("routing config should build");

    client
        .create_state_machine_alias()
        .name("staging")
        .routing_configuration(routing_item)
        .send()
        .await
        .expect("create alias should succeed");

    let list_resp = client
        .list_state_machine_aliases()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .expect("list_state_machine_aliases should succeed");

    assert_eq!(list_resp.state_machine_aliases().len(), 1);
}

#[tokio::test]
async fn test_update_state_machine_alias() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("alias_update_test")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .expect("create should succeed");

    let published = client
        .publish_state_machine_version()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .expect("publish should succeed");

    let routing_item = aws_sdk_sfn::types::RoutingConfigurationListItem::builder()
        .state_machine_version_arn(published.state_machine_version_arn())
        .weight(100)
        .build()
        .expect("routing config should build");

    let create_resp = client
        .create_state_machine_alias()
        .name("canary")
        .routing_configuration(routing_item)
        .send()
        .await
        .expect("create alias should succeed");

    let alias_arn = create_resp.state_machine_alias_arn();

    // Update the alias description
    let update_resp = client
        .update_state_machine_alias()
        .state_machine_alias_arn(alias_arn)
        .description("updated description")
        .send()
        .await
        .expect("update_state_machine_alias should succeed");

    let _ = update_resp.update_date(); // non-optional, confirm it exists

    // Verify the update took effect
    let desc_resp = client
        .describe_state_machine_alias()
        .state_machine_alias_arn(alias_arn)
        .send()
        .await
        .expect("describe should succeed after update");
    assert_eq!(desc_resp.description(), Some("updated description"));
}

#[tokio::test]
async fn test_delete_state_machine_alias() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("alias_delete_test")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .expect("create should succeed");

    let published = client
        .publish_state_machine_version()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .expect("publish should succeed");

    let routing_item = aws_sdk_sfn::types::RoutingConfigurationListItem::builder()
        .state_machine_version_arn(published.state_machine_version_arn())
        .weight(100)
        .build()
        .expect("routing config should build");

    let create_resp = client
        .create_state_machine_alias()
        .name("ephemeral")
        .routing_configuration(routing_item)
        .send()
        .await
        .expect("create alias should succeed");

    let alias_arn = create_resp.state_machine_alias_arn();

    client
        .delete_state_machine_alias()
        .state_machine_alias_arn(alias_arn)
        .send()
        .await
        .expect("delete_state_machine_alias should succeed");

    // Verify it's gone: describe should fail
    let result = client
        .describe_state_machine_alias()
        .state_machine_alias_arn(alias_arn)
        .send()
        .await;
    assert!(result.is_err(), "describe should fail after deletion");
}

// =============================================================================
// GetActivityTask test
// =============================================================================

#[tokio::test]
async fn test_get_activity_task_returns_empty() {
    let client = make_sfn_client().await;

    let activity = client
        .create_activity()
        .name("task_poll_test")
        .send()
        .await
        .expect("create_activity should succeed");

    let resp = client
        .get_activity_task()
        .activity_arn(activity.activity_arn())
        .send()
        .await
        .expect("get_activity_task should succeed");

    // No tasks queued in the mock — token should be absent/empty
    assert!(
        resp.task_token().is_none() || resp.task_token() == Some(""),
        "task_token should be empty when no tasks are queued"
    );
}

// NOTE: StartSyncExecution and TestState route to sync-states.*.amazonaws.com
// which requires a separate URL pattern registration. Skipped until the service
// registers that endpoint.

// =============================================================================
// RedriveExecution test
// =============================================================================

#[tokio::test]
async fn test_redrive_execution() {
    let client = make_sfn_client().await;

    let sm = client
        .create_state_machine()
        .name("redrive_test")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .send()
        .await
        .expect("create should succeed");

    let exec = client
        .start_execution()
        .state_machine_arn(sm.state_machine_arn())
        .send()
        .await
        .expect("start_execution should succeed");

    // Stop the execution so it transitions to ABORTED
    client
        .stop_execution()
        .execution_arn(exec.execution_arn())
        .send()
        .await
        .expect("stop_execution should succeed");

    // Redrive the stopped execution
    let redrive_resp = client
        .redrive_execution()
        .execution_arn(exec.execution_arn())
        .send()
        .await
        .expect("redrive_execution should succeed");

    let _ = redrive_resp.redrive_date(); // non-optional, confirm it exists
}

// NOTE: TestState also routes to sync-states.*.amazonaws.com — skipped
// until the service registers that endpoint.

// =============================================================================
// ValidateStateMachineDefinition tests
// =============================================================================

#[tokio::test]
async fn test_validate_state_machine_definition_valid() {
    let client = make_sfn_client().await;

    let valid_asl = r#"{
        "StartAt": "Only",
        "States": {
            "Only": { "Type": "Succeed" }
        }
    }"#;

    let resp = client
        .validate_state_machine_definition()
        .definition(valid_asl)
        .send()
        .await
        .expect("validate should succeed");

    assert_eq!(resp.result().as_str(), "OK");
    assert!(resp.diagnostics().is_empty());
}

#[tokio::test]
async fn test_validate_state_machine_definition_missing_start_at() {
    let client = make_sfn_client().await;

    let asl = r#"{
        "States": {
            "A": { "Type": "Succeed" }
        }
    }"#;

    let resp = client
        .validate_state_machine_definition()
        .definition(asl)
        .send()
        .await
        .expect("validate should succeed");

    assert_eq!(resp.result().as_str(), "FAIL");
    assert!(
        resp.diagnostics()
            .iter()
            .any(|d| d.code() == "MISSING_START_AT")
    );
}

#[tokio::test]
async fn test_validate_state_machine_definition_invalid_json() {
    let client = make_sfn_client().await;

    let resp = client
        .validate_state_machine_definition()
        .definition("not json at all")
        .send()
        .await
        .expect("validate should succeed");

    assert_eq!(resp.result().as_str(), "FAIL");
    assert!(
        resp.diagnostics()
            .iter()
            .any(|d| d.code() == "INVALID_JSON")
    );
}

#[tokio::test]
async fn test_validate_state_machine_definition_no_such_state() {
    let client = make_sfn_client().await;

    let asl = r#"{
        "StartAt": "Ghost",
        "States": {
            "A": { "Type": "Succeed" }
        }
    }"#;

    let resp = client
        .validate_state_machine_definition()
        .definition(asl)
        .send()
        .await
        .expect("validate should succeed");

    assert_eq!(resp.result().as_str(), "FAIL");
    assert!(
        resp.diagnostics()
            .iter()
            .any(|d| d.code() == "NO_SUCH_STATE")
    );
}

// =============================================================================
// V1/V2 bug regression: ValidateStateMachineDefinition edge cases
// =============================================================================

// Regression for V1: Map states using the modern `ItemProcessor` field must be
// accepted.  Previously winterbaume-sfn-asl-eval only recognised the deprecated
// `Iterator` field and would emit a spurious MISSING_ITERATOR diagnostic.
#[tokio::test]
async fn test_validate_state_machine_definition_map_with_item_processor() {
    let client = make_sfn_client().await;

    let asl = r#"{
        "StartAt": "M",
        "States": {
            "M": {
                "Type": "Map",
                "ItemProcessor": {
                    "StartAt": "Inner",
                    "States": {
                        "Inner": { "Type": "Succeed" }
                    }
                },
                "End": true
            }
        }
    }"#;

    let resp = client
        .validate_state_machine_definition()
        .definition(asl)
        .send()
        .await
        .expect("validate should succeed");

    assert_eq!(
        resp.result().as_str(),
        "OK",
        "Map state with ItemProcessor should be valid; diagnostics: {:?}",
        resp.diagnostics()
    );
    assert!(
        resp.diagnostics().is_empty(),
        "expected no diagnostics, got: {:?}",
        resp.diagnostics()
    );
}

// Regression for V2 (missing Type): a state that omits the required `Type` field
// must produce a FAIL result with a MISSING_TYPE diagnostic.
#[tokio::test]
async fn test_validate_state_machine_definition_missing_type() {
    let client = make_sfn_client().await;

    let asl = r#"{
        "StartAt": "X",
        "States": {
            "X": { "End": true }
        }
    }"#;

    let resp = client
        .validate_state_machine_definition()
        .definition(asl)
        .send()
        .await
        .expect("validate should succeed");

    assert_eq!(
        resp.result().as_str(),
        "FAIL",
        "state without Type must be invalid"
    );
    assert!(
        resp.diagnostics()
            .iter()
            .any(|d| d.code() == "MISSING_TYPE"),
        "expected MISSING_TYPE diagnostic, got: {:?}",
        resp.diagnostics()
    );
    assert!(
        resp.diagnostics()
            .iter()
            .all(|d| d.severity().as_str() == "ERROR"),
        "all diagnostics on a FAIL result must have ERROR severity, got: {:?}",
        resp.diagnostics()
    );
}

// Regression for V2 (End+Next conflict): a state that sets both `End: true` and
// a `Next` field simultaneously must produce a FAIL result with a
// CONFLICTING_END_AND_NEXT diagnostic.
#[tokio::test]
async fn test_validate_state_machine_definition_end_and_next_conflict() {
    let client = make_sfn_client().await;

    let asl = r#"{
        "StartAt": "A",
        "States": {
            "A": { "Type": "Pass", "End": true, "Next": "A" }
        }
    }"#;

    let resp = client
        .validate_state_machine_definition()
        .definition(asl)
        .send()
        .await
        .expect("validate should succeed");

    assert_eq!(
        resp.result().as_str(),
        "FAIL",
        "state with both End and Next must be invalid"
    );
    assert!(
        resp.diagnostics()
            .iter()
            .any(|d| d.code() == "CONFLICTING_END_AND_NEXT"),
        "expected CONFLICTING_END_AND_NEXT diagnostic, got: {:?}",
        resp.diagnostics()
    );
    assert!(
        resp.diagnostics()
            .iter()
            .all(|d| d.severity().as_str() == "ERROR"),
        "all diagnostics on a FAIL result must have ERROR severity, got: {:?}",
        resp.diagnostics()
    );
}

// =============================================================================
// FIX(terraform-e2e) coverage: CreateStateMachine with publish=true
// =============================================================================

// Covers FIX(terraform-e2e): when publish=true, CreateStateMachine auto-publishes
// a version and returns stateMachineVersionArn (handlers.rs:193).
// aws_sfn_alias resources use state_machine_version_arn from the create response,
// and an empty string would cause a provider validation error.
#[tokio::test]
async fn test_create_state_machine_with_publish_returns_version_arn() {
    let client = make_sfn_client().await;

    let resp = client
        .create_state_machine()
        .name("publish_on_create_test")
        .definition(SIMPLE_DEFINITION)
        .role_arn(default_role())
        .publish(true)
        .send()
        .await
        .expect("create_state_machine with publish=true should succeed");

    let version_arn = resp
        .state_machine_version_arn()
        .expect("stateMachineVersionArn must be present when publish=true");
    assert!(
        version_arn.contains(":stateMachine:publish_on_create_test:"),
        "version ARN should embed the state machine name, got: {version_arn}"
    );
}
