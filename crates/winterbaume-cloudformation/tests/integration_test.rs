use aws_sdk_cloudformation::Client;
use aws_sdk_cloudformation::config::BehaviorVersion;
use winterbaume_cloudformation::CloudFormationService;
use winterbaume_core::MockAws;

async fn make_client() -> Client {
    let svc = CloudFormationService::new();
    let mock = MockAws::builder().with_service(svc).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudformation::config::Region::new("us-east-1"))
        .load()
        .await;
    Client::new(&config)
}

// ---------------------------------------------------------------------------
// Stack CRUD
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_describe_stack() {
    let client = make_client().await;
    let resp = client
        .create_stack()
        .stack_name("my-stack")
        .template_body("{}")
        .send()
        .await
        .expect("create_stack should succeed");
    let stack_id = resp.stack_id().unwrap();
    assert!(
        stack_id.contains("my-stack"),
        "stack_id should contain stack name"
    );

    let resp2 = client
        .describe_stacks()
        .stack_name("my-stack")
        .send()
        .await
        .expect("describe_stacks should succeed");
    let stacks = resp2.stacks();
    assert_eq!(stacks.len(), 1);
    assert_eq!(stacks[0].stack_name().unwrap(), "my-stack");
    assert_eq!(
        stacks[0].stack_status().unwrap().as_str(),
        "CREATE_COMPLETE"
    );
}

#[tokio::test]
async fn test_describe_nonexistent_stack() {
    let client = make_client().await;
    assert!(
        client
            .describe_stacks()
            .stack_name("nonexistent")
            .send()
            .await
            .is_err(),
        "should fail for nonexistent stack"
    );
}

#[tokio::test]
async fn test_create_duplicate_stack() {
    let client = make_client().await;
    client
        .create_stack()
        .stack_name("dup-stack")
        .send()
        .await
        .expect("first create should succeed");
    assert!(
        client
            .create_stack()
            .stack_name("dup-stack")
            .send()
            .await
            .is_err(),
        "second create should fail"
    );
}

#[tokio::test]
async fn test_update_stack() {
    let client = make_client().await;
    client
        .create_stack()
        .stack_name("update-stack")
        .template_body("{}")
        .send()
        .await
        .expect("create_stack should succeed");

    let resp = client
        .update_stack()
        .stack_name("update-stack")
        .template_body("{\"updated\": true}")
        .send()
        .await
        .expect("update_stack should succeed");
    assert!(resp.stack_id().is_some());

    let desc = client
        .describe_stacks()
        .stack_name("update-stack")
        .send()
        .await
        .expect("describe after update");
    let stack = &desc.stacks()[0];
    assert_eq!(stack.stack_status().unwrap().as_str(), "UPDATE_COMPLETE");
}

#[tokio::test]
async fn test_delete_stack() {
    let client = make_client().await;
    client
        .create_stack()
        .stack_name("del-stack")
        .send()
        .await
        .expect("create_stack should succeed");

    client
        .delete_stack()
        .stack_name("del-stack")
        .send()
        .await
        .expect("delete_stack should succeed");

    // After deletion, describe should fail
    assert!(
        client
            .describe_stacks()
            .stack_name("del-stack")
            .send()
            .await
            .is_err(),
        "should fail after deletion"
    );
}

#[tokio::test]
async fn test_list_stacks() {
    let client = make_client().await;
    client
        .create_stack()
        .stack_name("list-stack-1")
        .send()
        .await
        .unwrap();
    client
        .create_stack()
        .stack_name("list-stack-2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_stacks()
        .send()
        .await
        .expect("list_stacks should succeed");
    let summaries = resp.stack_summaries();
    let names: Vec<&str> = summaries.iter().filter_map(|s| s.stack_name()).collect();
    assert!(names.contains(&"list-stack-1"));
    assert!(names.contains(&"list-stack-2"));
}

#[tokio::test]
async fn test_get_template() {
    let client = make_client().await;
    let template = "{\"AWSTemplateFormatVersion\": \"2010-09-09\"}";
    client
        .create_stack()
        .stack_name("template-stack")
        .template_body(template)
        .send()
        .await
        .unwrap();

    let resp = client
        .get_template()
        .stack_name("template-stack")
        .send()
        .await
        .expect("get_template should succeed");
    assert_eq!(resp.template_body().unwrap(), template);
}

#[tokio::test]
async fn test_describe_stack_events() {
    let client = make_client().await;
    client
        .create_stack()
        .stack_name("events-stack")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stack_events()
        .stack_name("events-stack")
        .send()
        .await
        .expect("describe_stack_events should succeed");
    let events = resp.stack_events();
    assert!(!events.is_empty(), "should have at least one event");
    assert_eq!(events[0].stack_name().unwrap(), "events-stack");
}

#[tokio::test]
async fn test_describe_stack_resources_empty() {
    let client = make_client().await;
    client
        .create_stack()
        .stack_name("res-stack")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_stack_resources()
        .stack_name("res-stack")
        .send()
        .await
        .expect("describe_stack_resources should succeed");
    // No resources added by default
    assert_eq!(resp.stack_resources().len(), 0);
}

#[tokio::test]
async fn test_list_stack_resources() {
    let client = make_client().await;
    client
        .create_stack()
        .stack_name("lsr-stack")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_stack_resources()
        .stack_name("lsr-stack")
        .send()
        .await
        .expect("list_stack_resources should succeed");
    assert_eq!(resp.stack_resource_summaries().len(), 0);
}

#[tokio::test]
async fn test_get_set_stack_policy() {
    let client = make_client().await;
    client
        .create_stack()
        .stack_name("policy-stack")
        .send()
        .await
        .unwrap();

    let policy =
        r#"{"Statement":[{"Effect":"Allow","Action":"Update:*","Principal":"*","Resource":"*"}]}"#;
    client
        .set_stack_policy()
        .stack_name("policy-stack")
        .stack_policy_body(policy)
        .send()
        .await
        .expect("set_stack_policy should succeed");

    let resp = client
        .get_stack_policy()
        .stack_name("policy-stack")
        .send()
        .await
        .expect("get_stack_policy should succeed");
    assert_eq!(resp.stack_policy_body().unwrap(), policy);
}

#[tokio::test]
async fn test_list_exports_empty() {
    let client = make_client().await;
    let resp = client
        .list_exports()
        .send()
        .await
        .expect("list_exports should succeed");
    // may have exports from other stacks, just verify it doesn't error
    let _ = resp.exports();
}

// ---------------------------------------------------------------------------
// ChangeSets
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_changeset_lifecycle() {
    let client = make_client().await;
    client
        .create_stack()
        .stack_name("cs-stack")
        .send()
        .await
        .unwrap();

    let resp = client
        .create_change_set()
        .stack_name("cs-stack")
        .change_set_name("my-cs")
        .send()
        .await
        .expect("create_change_set should succeed");
    assert!(resp.id().is_some());

    let desc = client
        .describe_change_set()
        .stack_name("cs-stack")
        .change_set_name("my-cs")
        .send()
        .await
        .expect("describe_change_set should succeed");
    assert_eq!(desc.change_set_name().unwrap(), "my-cs");
    assert_eq!(desc.execution_status().unwrap().as_str(), "AVAILABLE");

    let list = client
        .list_change_sets()
        .stack_name("cs-stack")
        .send()
        .await
        .expect("list_change_sets should succeed");
    assert_eq!(list.summaries().len(), 1);

    client
        .execute_change_set()
        .stack_name("cs-stack")
        .change_set_name("my-cs")
        .send()
        .await
        .expect("execute_change_set should succeed");

    client
        .delete_change_set()
        .stack_name("cs-stack")
        .change_set_name("my-cs")
        .send()
        .await
        .expect("delete_change_set should succeed");

    let list2 = client
        .list_change_sets()
        .stack_name("cs-stack")
        .send()
        .await
        .expect("list_change_sets after delete");
    assert_eq!(list2.summaries().len(), 0);
}

// ---------------------------------------------------------------------------
// ValidateTemplate
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_validate_template() {
    let client = make_client().await;
    let resp = client
        .validate_template()
        .template_body("{}")
        .send()
        .await
        .expect("validate_template should succeed");
    let _ = resp.description();
}

// ---------------------------------------------------------------------------
// StackSets
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_stack_set_lifecycle() {
    let client = make_client().await;

    let create = client
        .create_stack_set()
        .stack_set_name("my-ss")
        .description("Test stack set")
        .send()
        .await
        .expect("create_stack_set should succeed");
    assert!(create.stack_set_id().is_some());

    let desc = client
        .describe_stack_set()
        .stack_set_name("my-ss")
        .send()
        .await
        .expect("describe_stack_set should succeed");
    let ss = desc.stack_set().unwrap();
    assert_eq!(ss.stack_set_name().unwrap(), "my-ss");
    assert_eq!(ss.status().unwrap().as_str(), "ACTIVE");

    let list = client
        .list_stack_sets()
        .send()
        .await
        .expect("list_stack_sets should succeed");
    let names: Vec<&str> = list
        .summaries()
        .iter()
        .filter_map(|s| s.stack_set_name())
        .collect();
    assert!(names.contains(&"my-ss"));

    let update = client
        .update_stack_set()
        .stack_set_name("my-ss")
        .description("Updated description")
        .send()
        .await
        .expect("update_stack_set should succeed");
    assert!(update.operation_id().is_some());

    client
        .delete_stack_set()
        .stack_set_name("my-ss")
        .send()
        .await
        .expect("delete_stack_set should succeed");

    assert!(
        client
            .describe_stack_set()
            .stack_set_name("my-ss")
            .send()
            .await
            .is_err(),
        "describe after delete should fail"
    );
}

// ---------------------------------------------------------------------------
// StackInstances
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_stack_instances() {
    let client = make_client().await;

    client
        .create_stack_set()
        .stack_set_name("inst-ss")
        .send()
        .await
        .unwrap();

    let create = client
        .create_stack_instances()
        .stack_set_name("inst-ss")
        .set_accounts(Some(vec!["123456789012".to_string()]))
        .set_regions(Some(vec!["us-east-1".to_string()]))
        .send()
        .await
        .expect("create_stack_instances should succeed");
    assert!(create.operation_id().is_some());

    let desc = client
        .describe_stack_instance()
        .stack_set_name("inst-ss")
        .stack_instance_account("123456789012")
        .stack_instance_region("us-east-1")
        .send()
        .await
        .expect("describe_stack_instance should succeed");
    let inst = desc.stack_instance().unwrap();
    assert_eq!(inst.account().unwrap(), "123456789012");
    assert_eq!(inst.region().unwrap(), "us-east-1");

    let list = client
        .list_stack_instances()
        .stack_set_name("inst-ss")
        .send()
        .await
        .expect("list_stack_instances should succeed");
    assert_eq!(list.summaries().len(), 1);

    let del = client
        .delete_stack_instances()
        .stack_set_name("inst-ss")
        .set_accounts(Some(vec!["123456789012".to_string()]))
        .set_regions(Some(vec!["us-east-1".to_string()]))
        .retain_stacks(false)
        .send()
        .await
        .expect("delete_stack_instances should succeed");
    assert!(del.operation_id().is_some());

    let list2 = client
        .list_stack_instances()
        .stack_set_name("inst-ss")
        .send()
        .await
        .expect("list_stack_instances after delete");
    assert_eq!(list2.summaries().len(), 0);
}

// ---------------------------------------------------------------------------
// StackSetOperations
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_stack_set_operations() {
    let client = make_client().await;

    client
        .create_stack_set()
        .stack_set_name("ops-ss")
        .send()
        .await
        .unwrap();

    let update = client
        .update_stack_set()
        .stack_set_name("ops-ss")
        .description("update1")
        .send()
        .await
        .unwrap();
    let op_id = update.operation_id().unwrap().to_string();

    let desc = client
        .describe_stack_set_operation()
        .stack_set_name("ops-ss")
        .operation_id(&op_id)
        .send()
        .await
        .expect("describe_stack_set_operation should succeed");
    let op = desc.stack_set_operation().unwrap();
    assert_eq!(op.operation_id().unwrap(), op_id);

    let list = client
        .list_stack_set_operations()
        .stack_set_name("ops-ss")
        .send()
        .await
        .expect("list_stack_set_operations should succeed");
    assert_eq!(list.summaries().len(), 1);

    let results = client
        .list_stack_set_operation_results()
        .stack_set_name("ops-ss")
        .operation_id(&op_id)
        .send()
        .await
        .expect("list_stack_set_operation_results should succeed");
    let _ = results.summaries();
}

// ---------------------------------------------------------------------------
// StatefulService / state view tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_snapshot_and_restore() {
    use std::collections::HashMap;

    use winterbaume_cloudformation::CloudFormationStateView;
    use winterbaume_cloudformation::views::StackView;
    use winterbaume_core::StatefulService;

    let svc = CloudFormationService::new();

    // Seed state
    let mock = MockAws::builder()
        .with_service(CloudFormationService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudformation::config::Region::new("us-east-1"))
        .load()
        .await;
    let client = Client::new(&config);

    // Restore a view with one stack
    let mut stacks = HashMap::new();
    stacks.insert(
        "test-stack".to_string(),
        StackView {
            stack_id: "arn:aws:cloudformation:us-east-1:123456789012:stack/test-stack/abc"
                .to_string(),
            stack_name: "test-stack".to_string(),
            stack_status: "CREATE_COMPLETE".to_string(),
            creation_time: "2024-01-01T00:00:00.000Z".to_string(),
            last_updated_time: None,
            deletion_time: None,
            description: None,
            template_body: Some("{}".to_string()),
            stack_policy_body: None,
            parameters: vec![],
            outputs: vec![],
            tags: vec![],
            capabilities: vec![],
            resources: vec![],
            events: vec![],
            change_sets: vec![],
            exports: vec![],
            role_arn: None,
            timeout_in_minutes: None,
            disable_rollback: false,
            enable_termination_protection: false,
        },
    );
    let view = CloudFormationStateView {
        stacks,
        stack_sets: HashMap::new(),
        stack_instances: vec![],
        ..Default::default()
    };
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    // Snapshot should contain the stack
    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert!(snap.stacks.contains_key("test-stack"));

    let _ = (client, config);
}

#[tokio::test]
async fn test_merge_adds_resources() {
    use std::collections::HashMap;

    use winterbaume_cloudformation::CloudFormationStateView;
    use winterbaume_cloudformation::views::StackView;
    use winterbaume_core::StatefulService;

    let svc = CloudFormationService::new();

    let make_view = |name: &str| {
        let mut stacks = HashMap::new();
        stacks.insert(
            name.to_string(),
            StackView {
                stack_id: format!("arn:aws:cloudformation:us-east-1:123456789012:stack/{name}/abc"),
                stack_name: name.to_string(),
                stack_status: "CREATE_COMPLETE".to_string(),
                creation_time: "2024-01-01T00:00:00.000Z".to_string(),
                last_updated_time: None,
                deletion_time: None,
                description: None,
                template_body: None,
                stack_policy_body: None,
                parameters: vec![],
                outputs: vec![],
                tags: vec![],
                capabilities: vec![],
                resources: vec![],
                events: vec![],
                change_sets: vec![],
                exports: vec![],
                role_arn: None,
                timeout_in_minutes: None,
                disable_rollback: false,
                enable_termination_protection: false,
            },
        );
        CloudFormationStateView {
            stacks,
            stack_sets: HashMap::new(),
            stack_instances: vec![],
            ..Default::default()
        }
    };

    svc.merge("123456789012", "us-east-1", make_view("stack-a"))
        .await
        .unwrap();
    svc.merge("123456789012", "us-east-1", make_view("stack-b"))
        .await
        .unwrap();

    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert!(
        snap.stacks.contains_key("stack-a"),
        "stack-a should be present"
    );
    assert!(
        snap.stacks.contains_key("stack-b"),
        "stack-b should be present"
    );
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use winterbaume_cloudformation::CloudFormationStateView;
    use winterbaume_core::StatefulService;

    let svc = CloudFormationService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore(
        "123456789012",
        "us-east-1",
        CloudFormationStateView {
            stacks: HashMap::new(),
            stack_sets: HashMap::new(),
            stack_instances: vec![],
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[0],
        ("123456789012".to_string(), "us-east-1".to_string())
    );
}

#[tokio::test]
async fn test_state_change_listener_snapshot_reflects_mutation() {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    use winterbaume_cloudformation::CloudFormationStateView;
    use winterbaume_cloudformation::views::StackView;
    use winterbaume_core::StatefulService;

    let svc = CloudFormationService::new();

    let mut stacks = HashMap::new();
    stacks.insert(
        "snap-stack".to_string(),
        StackView {
            stack_id: "arn:aws:cloudformation:us-east-1:123456789012:stack/snap-stack/abc"
                .to_string(),
            stack_name: "snap-stack".to_string(),
            stack_status: "CREATE_COMPLETE".to_string(),
            creation_time: "2024-01-01T00:00:00.000Z".to_string(),
            last_updated_time: None,
            deletion_time: None,
            description: None,
            template_body: None,
            stack_policy_body: None,
            parameters: vec![],
            outputs: vec![],
            tags: vec![],
            capabilities: vec![],
            resources: vec![],
            events: vec![],
            change_sets: vec![],
            exports: vec![],
            role_arn: None,
            timeout_in_minutes: None,
            disable_rollback: false,
            enable_termination_protection: false,
        },
    );
    let view = CloudFormationStateView {
        stacks,
        stack_sets: HashMap::new(),
        stack_instances: vec![],
        ..Default::default()
    };

    let snapshots: Arc<Mutex<Vec<CloudFormationStateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert!(
        got[0].stacks.contains_key("snap-stack"),
        "snapshot should reflect the restored state"
    );
}

// ---------------------------------------------------------------------------
// DescribeStackResource
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_describe_stack_resource() {
    let client = make_client().await;
    client
        .create_stack()
        .stack_name("dsr-stack")
        .template_body("{}")
        .send()
        .await
        .unwrap();

    // With no resources, describing a specific resource should fail
    let result = client
        .describe_stack_resource()
        .stack_name("dsr-stack")
        .logical_resource_id("MyBucket")
        .send()
        .await;
    assert!(result.is_err(), "should fail for nonexistent resource");
}

// ---------------------------------------------------------------------------
// UpdateTerminationProtection
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_update_termination_protection() {
    let client = make_client().await;
    client
        .create_stack()
        .stack_name("tp-stack")
        .template_body("{}")
        .send()
        .await
        .unwrap();

    let resp = client
        .update_termination_protection()
        .stack_name("tp-stack")
        .enable_termination_protection(true)
        .send()
        .await
        .expect("update_termination_protection should succeed");
    assert!(resp.stack_id().is_some());
}

// ---------------------------------------------------------------------------
// DescribeAccountLimits
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_describe_account_limits() {
    let client = make_client().await;
    let resp = client
        .describe_account_limits()
        .send()
        .await
        .expect("describe_account_limits should succeed");
    let limits = resp.account_limits();
    assert!(limits.len() >= 2, "should have at least 2 account limits");
}

// ---------------------------------------------------------------------------
// ListImports
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_list_imports() {
    let client = make_client().await;
    let resp = client
        .list_imports()
        .export_name("some-export")
        .send()
        .await
        .expect("list_imports should succeed");
    // No exports exist, so imports list should be empty
    assert!(resp.imports().is_empty());
}

// ---------------------------------------------------------------------------
// GetTemplateSummary
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_get_template_summary() {
    let client = make_client().await;
    let resp = client
        .get_template_summary()
        .template_body("{}")
        .send()
        .await
        .expect("get_template_summary should succeed");
    // The handler returns version "2010-09-09"
    let _ = resp.description();
}

// ---------------------------------------------------------------------------
// CancelUpdateStack
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_cancel_update_stack() {
    let client = make_client().await;
    client
        .create_stack()
        .stack_name("cancel-stack")
        .template_body("{}")
        .send()
        .await
        .unwrap();

    let result = client
        .cancel_update_stack()
        .stack_name("cancel-stack")
        .send()
        .await;
    // cancel_update_stack succeeds or returns an error depending on state;
    // the important thing is we exercise the handler path
    let _ = result;
}

// ---------------------------------------------------------------------------
// RollbackStack
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_rollback_stack() {
    let client = make_client().await;
    client
        .create_stack()
        .stack_name("rb-stack")
        .template_body("{}")
        .send()
        .await
        .unwrap();

    let result = client.rollback_stack().stack_name("rb-stack").send().await;
    // Exercise the handler — result may vary by state implementation
    let _ = result;
}

// ---------------------------------------------------------------------------
// SignalResource
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_signal_resource() {
    let client = make_client().await;
    client
        .create_stack()
        .stack_name("any-stack")
        .template_body("{}")
        .send()
        .await
        .unwrap();
    let resp = client
        .signal_resource()
        .stack_name("any-stack")
        .logical_resource_id("MyResource")
        .unique_id("id-1")
        .status(aws_sdk_cloudformation::types::ResourceSignalStatus::Success)
        .send()
        .await
        .expect("signal_resource should succeed");
    let _ = resp;
}

// ---------------------------------------------------------------------------
// EstimateTemplateCost
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_estimate_template_cost() {
    let client = make_client().await;
    let resp = client
        .estimate_template_cost()
        .template_body("{}")
        .send()
        .await
        .expect("estimate_template_cost should succeed");
    assert!(resp.url().is_some(), "should return a URL");
}

// ---------------------------------------------------------------------------
// ContinueUpdateRollback
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_continue_update_rollback() {
    let client = make_client().await;
    client
        .create_stack()
        .stack_name("cur-stack")
        .template_body("{}")
        .send()
        .await
        .unwrap();

    let resp = client
        .continue_update_rollback()
        .stack_name("cur-stack")
        .send()
        .await
        .expect("continue_update_rollback should succeed");
    let _ = resp;
}

// ---------------------------------------------------------------------------
// ListTypes
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_list_types() {
    let client = make_client().await;
    let resp = client
        .list_types()
        .send()
        .await
        .expect("list_types should succeed");
    let _ = resp.type_summaries();
}

// ---------------------------------------------------------------------------
// StopStackSetOperation
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_stop_stack_set_operation() {
    let client = make_client().await;
    client
        .create_stack_set()
        .stack_set_name("stop-ss")
        .send()
        .await
        .unwrap();

    // Create an operation via update
    let update = client
        .update_stack_set()
        .stack_set_name("stop-ss")
        .description("trigger op")
        .send()
        .await
        .unwrap();
    let op_id = update.operation_id().unwrap().to_string();

    let result = client
        .stop_stack_set_operation()
        .stack_set_name("stop-ss")
        .operation_id(&op_id)
        .send()
        .await;
    // Exercise the handler path
    let _ = result;
}

// ---------------------------------------------------------------------------
// UpdateStackInstances
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_update_stack_instances() {
    let client = make_client().await;
    client
        .create_stack_set()
        .stack_set_name("upd-inst-ss")
        .send()
        .await
        .unwrap();

    client
        .create_stack_instances()
        .stack_set_name("upd-inst-ss")
        .set_accounts(Some(vec!["123456789012".to_string()]))
        .set_regions(Some(vec!["us-east-1".to_string()]))
        .send()
        .await
        .unwrap();

    let resp = client
        .update_stack_instances()
        .stack_set_name("upd-inst-ss")
        .set_accounts(Some(vec!["123456789012".to_string()]))
        .set_regions(Some(vec!["us-east-1".to_string()]))
        .send()
        .await
        .expect("update_stack_instances should succeed");
    assert!(resp.operation_id().is_some());
}
