//! Integration tests for winterbaume SWF service.
//!
//! These tests verify that aws-sdk-swf operations work end-to-end
//! through the winterbaume mock infrastructure.

use aws_sdk_swf::config::BehaviorVersion;
use aws_sdk_swf::types::RegistrationStatus;
use winterbaume_core::MockAws;
use winterbaume_swf::SwfService;

/// Helper to create a configured SWF client backed by winterbaume.
async fn make_swf_client() -> aws_sdk_swf::Client {
    let mock = MockAws::builder().with_service(SwfService::new()).build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_swf::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_swf::Client::new(&config)
}

/// Helper to register a domain.
async fn register_domain(client: &aws_sdk_swf::Client, name: &str) {
    client
        .register_domain()
        .name(name)
        .workflow_execution_retention_period_in_days("30")
        .send()
        .await
        .expect("register_domain should succeed");
}

/// Helper to register a workflow type in a domain.
async fn register_workflow_type(
    client: &aws_sdk_swf::Client,
    domain: &str,
    name: &str,
    version: &str,
) {
    client
        .register_workflow_type()
        .domain(domain)
        .name(name)
        .version(version)
        .default_execution_start_to_close_timeout("3600")
        .default_task_start_to_close_timeout("300")
        .send()
        .await
        .expect("register_workflow_type should succeed");
}

/// Helper to register an activity type in a domain.
async fn register_activity_type(
    client: &aws_sdk_swf::Client,
    domain: &str,
    name: &str,
    version: &str,
) {
    client
        .register_activity_type()
        .domain(domain)
        .name(name)
        .version(version)
        .send()
        .await
        .expect("register_activity_type should succeed");
}

/// Helper to start a workflow execution and return the run_id.
async fn start_workflow(
    client: &aws_sdk_swf::Client,
    domain: &str,
    workflow_id: &str,
    wt_name: &str,
    wt_version: &str,
) -> String {
    let resp = client
        .start_workflow_execution()
        .domain(domain)
        .workflow_id(workflow_id)
        .workflow_type(
            aws_sdk_swf::types::WorkflowType::builder()
                .name(wt_name)
                .version(wt_version)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("start_workflow_execution should succeed");
    resp.run_id().expect("should have run_id").to_string()
}

// ==================== Domain tests ====================

#[tokio::test]
async fn test_register_and_describe_domain() {
    let client = make_swf_client().await;

    client
        .register_domain()
        .name("test-domain")
        .description("A test domain")
        .workflow_execution_retention_period_in_days("30")
        .send()
        .await
        .expect("register_domain should succeed");

    let resp = client
        .describe_domain()
        .name("test-domain")
        .send()
        .await
        .expect("describe_domain should succeed");

    let info = resp.domain_info().expect("should have domain_info");
    assert_eq!(info.name(), "test-domain");
    assert_eq!(info.status(), &RegistrationStatus::Registered);
    assert_eq!(info.description(), Some("A test domain"));
    assert!(info.arn().is_some(), "should have ARN");

    let config = resp.configuration().expect("should have configuration");
    assert_eq!(config.workflow_execution_retention_period_in_days(), "30");
}

#[tokio::test]
async fn test_deprecate_domain() {
    let client = make_swf_client().await;
    register_domain(&client, "dep-domain").await;

    client
        .deprecate_domain()
        .name("dep-domain")
        .send()
        .await
        .expect("deprecate_domain should succeed");

    // Describe should show DEPRECATED status
    let resp = client
        .describe_domain()
        .name("dep-domain")
        .send()
        .await
        .expect("describe_domain should succeed after deprecation");

    let info = resp.domain_info().expect("should have domain_info");
    assert_eq!(info.status(), &RegistrationStatus::Deprecated);
}

#[tokio::test]
async fn test_list_domains_empty() {
    let client = make_swf_client().await;

    let resp = client
        .list_domains()
        .registration_status(RegistrationStatus::Registered)
        .send()
        .await
        .expect("list_domains should succeed");

    assert!(
        resp.domain_infos().is_empty(),
        "should have no domains initially"
    );
}

#[tokio::test]
async fn test_list_domains_filters_by_status() {
    let client = make_swf_client().await;

    register_domain(&client, "active-domain-1").await;
    register_domain(&client, "active-domain-2").await;
    register_domain(&client, "to-deprecate").await;

    // Deprecate one
    client
        .deprecate_domain()
        .name("to-deprecate")
        .send()
        .await
        .unwrap();

    // List REGISTERED
    let resp = client
        .list_domains()
        .registration_status(RegistrationStatus::Registered)
        .send()
        .await
        .expect("list_domains REGISTERED should succeed");

    let names: Vec<&str> = resp.domain_infos().iter().map(|d| d.name()).collect();
    assert_eq!(names.len(), 2);
    assert!(names.contains(&"active-domain-1"));
    assert!(names.contains(&"active-domain-2"));

    // List DEPRECATED
    let resp = client
        .list_domains()
        .registration_status(RegistrationStatus::Deprecated)
        .send()
        .await
        .expect("list_domains DEPRECATED should succeed");

    let names: Vec<&str> = resp.domain_infos().iter().map(|d| d.name()).collect();
    assert_eq!(names.len(), 1);
    assert_eq!(names[0], "to-deprecate");
}

#[tokio::test]
async fn test_register_duplicate_domain_fails() {
    let client = make_swf_client().await;
    register_domain(&client, "dup-domain").await;

    let result = client
        .register_domain()
        .name("dup-domain")
        .workflow_execution_retention_period_in_days("30")
        .send()
        .await;

    assert!(
        result.is_err(),
        "registering a duplicate domain should fail"
    );
}

#[tokio::test]
async fn test_describe_nonexistent_domain_fails() {
    let client = make_swf_client().await;

    let result = client.describe_domain().name("nonexistent").send().await;

    assert!(
        result.is_err(),
        "describing a nonexistent domain should fail"
    );
}

#[tokio::test]
async fn test_deprecate_nonexistent_domain_fails() {
    let client = make_swf_client().await;

    let result = client.deprecate_domain().name("nonexistent").send().await;

    assert!(
        result.is_err(),
        "deprecating a nonexistent domain should fail"
    );
}

#[tokio::test]
async fn test_deprecate_already_deprecated_domain_fails() {
    let client = make_swf_client().await;
    register_domain(&client, "double-dep").await;

    client
        .deprecate_domain()
        .name("double-dep")
        .send()
        .await
        .unwrap();

    let result = client.deprecate_domain().name("double-dep").send().await;

    assert!(
        result.is_err(),
        "deprecating an already deprecated domain should fail"
    );
}

#[tokio::test]
async fn test_domain_crud_lifecycle() {
    let client = make_swf_client().await;

    // Create
    client
        .register_domain()
        .name("lifecycle-domain")
        .description("Lifecycle test")
        .workflow_execution_retention_period_in_days("10")
        .send()
        .await
        .expect("register should succeed");

    // Read
    let resp = client
        .describe_domain()
        .name("lifecycle-domain")
        .send()
        .await
        .expect("describe should succeed");
    let info = resp.domain_info().unwrap();
    assert_eq!(info.name(), "lifecycle-domain");
    assert_eq!(info.status(), &RegistrationStatus::Registered);

    // List - should appear in REGISTERED
    let resp = client
        .list_domains()
        .registration_status(RegistrationStatus::Registered)
        .send()
        .await
        .unwrap();
    assert!(
        resp.domain_infos()
            .iter()
            .any(|d| d.name() == "lifecycle-domain"),
        "should be in REGISTERED list"
    );

    // Deprecate
    client
        .deprecate_domain()
        .name("lifecycle-domain")
        .send()
        .await
        .expect("deprecate should succeed");

    // Read after deprecation
    let resp = client
        .describe_domain()
        .name("lifecycle-domain")
        .send()
        .await
        .expect("describe should still succeed after deprecation");
    let info = resp.domain_info().unwrap();
    assert_eq!(info.status(), &RegistrationStatus::Deprecated);

    // List - should now appear in DEPRECATED, not REGISTERED
    let resp = client
        .list_domains()
        .registration_status(RegistrationStatus::Registered)
        .send()
        .await
        .unwrap();
    assert!(
        !resp
            .domain_infos()
            .iter()
            .any(|d| d.name() == "lifecycle-domain"),
        "should NOT be in REGISTERED list after deprecation"
    );

    let resp = client
        .list_domains()
        .registration_status(RegistrationStatus::Deprecated)
        .send()
        .await
        .unwrap();
    assert!(
        resp.domain_infos()
            .iter()
            .any(|d| d.name() == "lifecycle-domain"),
        "should be in DEPRECATED list"
    );
}

// ==================== Activity Type tests ====================

#[tokio::test]
async fn test_deprecate_activity_type() {
    let client = make_swf_client().await;
    register_domain(&client, "at-domain").await;
    register_activity_type(&client, "at-domain", "my-activity", "1.0").await;

    client
        .deprecate_activity_type()
        .domain("at-domain")
        .activity_type(
            aws_sdk_swf::types::ActivityType::builder()
                .name("my-activity")
                .version("1.0")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("deprecate_activity_type should succeed");

    // Verify deprecated
    let resp = client
        .describe_activity_type()
        .domain("at-domain")
        .activity_type(
            aws_sdk_swf::types::ActivityType::builder()
                .name("my-activity")
                .version("1.0")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("describe_activity_type should succeed");

    let info = resp.type_info().expect("should have type_info");
    assert_eq!(info.status(), &RegistrationStatus::Deprecated);
}

#[tokio::test]
async fn test_describe_activity_type() {
    let client = make_swf_client().await;
    register_domain(&client, "desc-at-domain").await;
    register_activity_type(&client, "desc-at-domain", "my-activity", "1.0").await;

    let resp = client
        .describe_activity_type()
        .domain("desc-at-domain")
        .activity_type(
            aws_sdk_swf::types::ActivityType::builder()
                .name("my-activity")
                .version("1.0")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("describe_activity_type should succeed");

    let info = resp.type_info().expect("should have type_info");
    assert_eq!(info.activity_type().unwrap().name(), "my-activity");
    assert_eq!(info.activity_type().unwrap().version(), "1.0");
    assert_eq!(info.status(), &RegistrationStatus::Registered);
}

#[tokio::test]
async fn test_describe_nonexistent_activity_type_fails() {
    let client = make_swf_client().await;
    register_domain(&client, "noat-domain").await;

    let result = client
        .describe_activity_type()
        .domain("noat-domain")
        .activity_type(
            aws_sdk_swf::types::ActivityType::builder()
                .name("nonexistent")
                .version("1.0")
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(result.is_err(), "should fail for nonexistent activity type");
}

// ==================== Workflow Type tests ====================

#[tokio::test]
async fn test_deprecate_workflow_type() {
    let client = make_swf_client().await;
    register_domain(&client, "wt-domain").await;
    register_workflow_type(&client, "wt-domain", "my-workflow", "1.0").await;

    client
        .deprecate_workflow_type()
        .domain("wt-domain")
        .workflow_type(
            aws_sdk_swf::types::WorkflowType::builder()
                .name("my-workflow")
                .version("1.0")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("deprecate_workflow_type should succeed");

    // Verify deprecated
    let resp = client
        .describe_workflow_type()
        .domain("wt-domain")
        .workflow_type(
            aws_sdk_swf::types::WorkflowType::builder()
                .name("my-workflow")
                .version("1.0")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("describe_workflow_type should succeed");

    let info = resp.type_info().expect("should have type_info");
    assert_eq!(info.status(), &RegistrationStatus::Deprecated);
}

#[tokio::test]
async fn test_describe_workflow_type() {
    let client = make_swf_client().await;
    register_domain(&client, "desc-wt-domain").await;
    register_workflow_type(&client, "desc-wt-domain", "my-workflow", "2.0").await;

    let resp = client
        .describe_workflow_type()
        .domain("desc-wt-domain")
        .workflow_type(
            aws_sdk_swf::types::WorkflowType::builder()
                .name("my-workflow")
                .version("2.0")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("describe_workflow_type should succeed");

    let info = resp.type_info().expect("should have type_info");
    assert_eq!(info.workflow_type().unwrap().name(), "my-workflow");
    assert_eq!(info.workflow_type().unwrap().version(), "2.0");
    assert_eq!(info.status(), &RegistrationStatus::Registered);
}

#[tokio::test]
async fn test_describe_nonexistent_workflow_type_fails() {
    let client = make_swf_client().await;
    register_domain(&client, "nowt-domain").await;

    let result = client
        .describe_workflow_type()
        .domain("nowt-domain")
        .workflow_type(
            aws_sdk_swf::types::WorkflowType::builder()
                .name("nonexistent")
                .version("1.0")
                .build()
                .unwrap(),
        )
        .send()
        .await;

    assert!(result.is_err(), "should fail for nonexistent workflow type");
}

// ==================== Workflow Execution tests ====================

#[tokio::test]
async fn test_describe_workflow_execution() {
    let client = make_swf_client().await;
    register_domain(&client, "exec-domain").await;
    register_workflow_type(&client, "exec-domain", "exec-wf", "1.0").await;
    let run_id = start_workflow(&client, "exec-domain", "wf-001", "exec-wf", "1.0").await;

    let resp = client
        .describe_workflow_execution()
        .domain("exec-domain")
        .execution(
            aws_sdk_swf::types::WorkflowExecution::builder()
                .workflow_id("wf-001")
                .run_id(&run_id)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("describe_workflow_execution should succeed");

    let info = resp.execution_info().expect("should have execution_info");
    assert_eq!(info.execution().unwrap().workflow_id(), "wf-001");
    assert_eq!(
        info.execution_status(),
        &aws_sdk_swf::types::ExecutionStatus::Open
    );
}

#[tokio::test]
async fn test_terminate_workflow_execution() {
    let client = make_swf_client().await;
    register_domain(&client, "term-domain").await;
    register_workflow_type(&client, "term-domain", "term-wf", "1.0").await;
    let run_id = start_workflow(&client, "term-domain", "wf-term", "term-wf", "1.0").await;

    client
        .terminate_workflow_execution()
        .domain("term-domain")
        .workflow_id("wf-term")
        .run_id(&run_id)
        .reason("testing")
        .send()
        .await
        .expect("terminate_workflow_execution should succeed");

    // Verify closed
    let resp = client
        .describe_workflow_execution()
        .domain("term-domain")
        .execution(
            aws_sdk_swf::types::WorkflowExecution::builder()
                .workflow_id("wf-term")
                .run_id(&run_id)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("describe should succeed after termination");

    let info = resp.execution_info().expect("should have execution_info");
    assert_eq!(
        info.execution_status(),
        &aws_sdk_swf::types::ExecutionStatus::Closed
    );
    assert_eq!(
        info.close_status(),
        Some(&aws_sdk_swf::types::CloseStatus::Terminated)
    );
}

#[tokio::test]
async fn test_get_workflow_execution_history() {
    let client = make_swf_client().await;
    register_domain(&client, "hist-domain").await;
    register_workflow_type(&client, "hist-domain", "hist-wf", "1.0").await;
    let run_id = start_workflow(&client, "hist-domain", "wf-hist", "hist-wf", "1.0").await;

    let resp = client
        .get_workflow_execution_history()
        .domain("hist-domain")
        .execution(
            aws_sdk_swf::types::WorkflowExecution::builder()
                .workflow_id("wf-hist")
                .run_id(&run_id)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("get_workflow_execution_history should succeed");

    let events = resp.events();
    assert!(!events.is_empty(), "should have at least one history event");
    assert_eq!(
        events[0].event_type(),
        &aws_sdk_swf::types::EventType::WorkflowExecutionStarted
    );
}

#[tokio::test]
async fn test_count_closed_workflow_executions() {
    let client = make_swf_client().await;
    register_domain(&client, "cnt-closed-domain").await;
    register_workflow_type(&client, "cnt-closed-domain", "cnt-wf", "1.0").await;

    // Start and terminate a workflow
    let run_id = start_workflow(&client, "cnt-closed-domain", "wf-cnt", "cnt-wf", "1.0").await;
    client
        .terminate_workflow_execution()
        .domain("cnt-closed-domain")
        .workflow_id("wf-cnt")
        .run_id(&run_id)
        .send()
        .await
        .unwrap();

    let resp = client
        .count_closed_workflow_executions()
        .domain("cnt-closed-domain")
        .start_time_filter(
            aws_sdk_swf::types::ExecutionTimeFilter::builder()
                .oldest_date(aws_smithy_types::DateTime::from_secs(0))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("count_closed_workflow_executions should succeed");

    assert_eq!(resp.count(), 1);
}

#[tokio::test]
async fn test_count_open_workflow_executions() {
    let client = make_swf_client().await;
    register_domain(&client, "cnt-open-domain").await;
    register_workflow_type(&client, "cnt-open-domain", "cnt-wf", "1.0").await;

    start_workflow(&client, "cnt-open-domain", "wf-open-1", "cnt-wf", "1.0").await;

    let resp = client
        .count_open_workflow_executions()
        .domain("cnt-open-domain")
        .start_time_filter(
            aws_sdk_swf::types::ExecutionTimeFilter::builder()
                .oldest_date(aws_smithy_types::DateTime::from_secs(0))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("count_open_workflow_executions should succeed");

    assert_eq!(resp.count(), 1);
}

#[tokio::test]
async fn test_count_pending_activity_tasks() {
    let client = make_swf_client().await;
    register_domain(&client, "cnt-pat-domain").await;

    let resp = client
        .count_pending_activity_tasks()
        .domain("cnt-pat-domain")
        .task_list(
            aws_sdk_swf::types::TaskList::builder()
                .name("default")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("count_pending_activity_tasks should succeed");

    assert_eq!(resp.count(), 0);
}

#[tokio::test]
async fn test_count_pending_decision_tasks() {
    let client = make_swf_client().await;
    register_domain(&client, "cnt-pdt-domain").await;

    let resp = client
        .count_pending_decision_tasks()
        .domain("cnt-pdt-domain")
        .task_list(
            aws_sdk_swf::types::TaskList::builder()
                .name("default")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("count_pending_decision_tasks should succeed");

    assert_eq!(resp.count(), 0);
}

#[tokio::test]
async fn test_list_open_workflow_executions() {
    let client = make_swf_client().await;
    register_domain(&client, "list-open-domain").await;
    register_workflow_type(&client, "list-open-domain", "list-wf", "1.0").await;

    start_workflow(&client, "list-open-domain", "wf-lo-1", "list-wf", "1.0").await;
    start_workflow(&client, "list-open-domain", "wf-lo-2", "list-wf", "1.0").await;

    let resp = client
        .list_open_workflow_executions()
        .domain("list-open-domain")
        .start_time_filter(
            aws_sdk_swf::types::ExecutionTimeFilter::builder()
                .oldest_date(aws_smithy_types::DateTime::from_secs(0))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("list_open_workflow_executions should succeed");

    assert_eq!(resp.execution_infos().len(), 2);
}

#[tokio::test]
async fn test_list_closed_workflow_executions() {
    let client = make_swf_client().await;
    register_domain(&client, "list-closed-domain").await;
    register_workflow_type(&client, "list-closed-domain", "list-wf", "1.0").await;

    let run_id = start_workflow(&client, "list-closed-domain", "wf-lc-1", "list-wf", "1.0").await;
    client
        .terminate_workflow_execution()
        .domain("list-closed-domain")
        .workflow_id("wf-lc-1")
        .run_id(&run_id)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_closed_workflow_executions()
        .domain("list-closed-domain")
        .start_time_filter(
            aws_sdk_swf::types::ExecutionTimeFilter::builder()
                .oldest_date(aws_smithy_types::DateTime::from_secs(0))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("list_closed_workflow_executions should succeed");

    assert_eq!(resp.execution_infos().len(), 1);
    let info = &resp.execution_infos()[0];
    assert_eq!(
        info.execution_status(),
        &aws_sdk_swf::types::ExecutionStatus::Closed
    );
}

#[tokio::test]
async fn test_poll_for_decision_task() {
    let client = make_swf_client().await;
    register_domain(&client, "poll-domain").await;
    register_workflow_type(&client, "poll-domain", "poll-wf", "1.0").await;
    start_workflow(&client, "poll-domain", "wf-poll", "poll-wf", "1.0").await;

    let resp = client
        .poll_for_decision_task()
        .domain("poll-domain")
        .task_list(
            aws_sdk_swf::types::TaskList::builder()
                .name("default")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("poll_for_decision_task should succeed");

    // Should return a task with events
    assert!(!resp.events().is_empty(), "should have events");
    assert!(!resp.task_token().is_empty(), "should have a task token");
}

#[tokio::test]
async fn test_terminate_nonexistent_execution_fails() {
    let client = make_swf_client().await;
    register_domain(&client, "term-fail-domain").await;

    let result = client
        .terminate_workflow_execution()
        .domain("term-fail-domain")
        .workflow_id("nonexistent")
        .send()
        .await;

    assert!(
        result.is_err(),
        "terminating nonexistent execution should fail"
    );
}

#[tokio::test]
async fn test_workflow_execution_lifecycle() {
    let client = make_swf_client().await;
    register_domain(&client, "life-domain").await;
    register_workflow_type(&client, "life-domain", "life-wf", "1.0").await;

    // Start
    let run_id = start_workflow(&client, "life-domain", "wf-life", "life-wf", "1.0").await;

    // Count open = 1
    let resp = client
        .count_open_workflow_executions()
        .domain("life-domain")
        .start_time_filter(
            aws_sdk_swf::types::ExecutionTimeFilter::builder()
                .oldest_date(aws_smithy_types::DateTime::from_secs(0))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(resp.count(), 1);

    // Describe
    let resp = client
        .describe_workflow_execution()
        .domain("life-domain")
        .execution(
            aws_sdk_swf::types::WorkflowExecution::builder()
                .workflow_id("wf-life")
                .run_id(&run_id)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(
        resp.execution_info().unwrap().execution_status(),
        &aws_sdk_swf::types::ExecutionStatus::Open
    );

    // Get history
    let resp = client
        .get_workflow_execution_history()
        .domain("life-domain")
        .execution(
            aws_sdk_swf::types::WorkflowExecution::builder()
                .workflow_id("wf-life")
                .run_id(&run_id)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    assert!(!resp.events().is_empty());

    // Terminate
    client
        .terminate_workflow_execution()
        .domain("life-domain")
        .workflow_id("wf-life")
        .run_id(&run_id)
        .send()
        .await
        .unwrap();

    // Count open = 0, closed = 1
    let resp = client
        .count_open_workflow_executions()
        .domain("life-domain")
        .start_time_filter(
            aws_sdk_swf::types::ExecutionTimeFilter::builder()
                .oldest_date(aws_smithy_types::DateTime::from_secs(0))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(resp.count(), 0);

    let resp = client
        .count_closed_workflow_executions()
        .domain("life-domain")
        .start_time_filter(
            aws_sdk_swf::types::ExecutionTimeFilter::builder()
                .oldest_date(aws_smithy_types::DateTime::from_secs(0))
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();
    assert_eq!(resp.count(), 1);
}

// ==================== Error-type specific tests from moto ====================

#[tokio::test]
async fn test_register_domain_arn_format() {
    // Moto: domain["arn"] == f"arn:aws:swf:us-west-1:{ACCOUNT_ID}:/domain/test-domain"
    let client = make_swf_client().await;

    register_domain(&client, "arn-domain").await;

    let resp = client
        .describe_domain()
        .name("arn-domain")
        .send()
        .await
        .expect("describe_domain should succeed");

    let info = resp.domain_info().expect("should have domain_info");
    let arn = info.arn().expect("should have arn");
    assert!(
        arn.contains("arn:aws:swf:"),
        "ARN should start with arn:aws:swf:"
    );
    assert!(arn.contains("arn-domain"), "ARN should contain domain name");
}

#[tokio::test]
async fn test_register_duplicate_domain_specific_error() {
    // Moto: ex.value.response["Error"]["Code"] == "DomainAlreadyExistsFault"
    let client = make_swf_client().await;
    register_domain(&client, "dup-domain-err").await;

    let err = client
        .register_domain()
        .name("dup-domain-err")
        .workflow_execution_retention_period_in_days("30")
        .send()
        .await
        .unwrap_err();

    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_domain_already_exists_fault(),
        "expected DomainAlreadyExistsFault, got: {svc_err:?}"
    );
}

#[tokio::test]
async fn test_deprecate_non_existent_domain_specific_error() {
    // Moto: ex.value.response["Error"]["Code"] == "UnknownResourceFault"
    let client = make_swf_client().await;

    let err = client
        .deprecate_domain()
        .name("non-existent-domain")
        .send()
        .await
        .unwrap_err();

    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_unknown_resource_fault(),
        "expected UnknownResourceFault, got: {svc_err:?}"
    );
}

#[tokio::test]
async fn test_deprecate_already_deprecated_domain_specific_error() {
    // Moto: ex.value.response["Error"]["Code"] == "DomainDeprecatedFault"
    let client = make_swf_client().await;
    register_domain(&client, "dep-err-domain").await;
    client
        .deprecate_domain()
        .name("dep-err-domain")
        .send()
        .await
        .unwrap();

    let err = client
        .deprecate_domain()
        .name("dep-err-domain")
        .send()
        .await
        .unwrap_err();

    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_domain_deprecated_fault(),
        "expected DomainDeprecatedFault, got: {svc_err:?}"
    );
}

#[tokio::test]
async fn test_register_duplicate_workflow_type_specific_error() {
    // Moto: ex.value.response["Error"]["Code"] == "TypeAlreadyExistsFault"
    let client = make_swf_client().await;
    register_domain(&client, "dup-wt-domain").await;
    register_workflow_type(&client, "dup-wt-domain", "dup-workflow", "1.0").await;

    let err = client
        .register_workflow_type()
        .domain("dup-wt-domain")
        .name("dup-workflow")
        .version("1.0")
        .send()
        .await
        .unwrap_err();

    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_type_already_exists_fault(),
        "expected TypeAlreadyExistsFault, got: {svc_err:?}"
    );
}

#[tokio::test]
async fn test_register_duplicate_activity_type_specific_error() {
    // Moto: ex.value.response["Error"]["Code"] == "TypeAlreadyExistsFault"
    let client = make_swf_client().await;
    register_domain(&client, "dup-at-domain").await;
    register_activity_type(&client, "dup-at-domain", "dup-activity", "1.0").await;

    let err = client
        .register_activity_type()
        .domain("dup-at-domain")
        .name("dup-activity")
        .version("1.0")
        .send()
        .await
        .unwrap_err();

    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_type_already_exists_fault(),
        "expected TypeAlreadyExistsFault, got: {svc_err:?}"
    );
}

#[tokio::test]
async fn test_start_already_started_workflow_specific_error() {
    // Moto: ex.value.response["Error"]["Code"] == "WorkflowExecutionAlreadyStartedFault"
    let client = make_swf_client().await;
    register_domain(&client, "already-started-domain").await;
    register_workflow_type(&client, "already-started-domain", "already-wf", "1.0").await;
    start_workflow(
        &client,
        "already-started-domain",
        "wf-dup",
        "already-wf",
        "1.0",
    )
    .await;

    let err = client
        .start_workflow_execution()
        .domain("already-started-domain")
        .workflow_id("wf-dup")
        .workflow_type(
            aws_sdk_swf::types::WorkflowType::builder()
                .name("already-wf")
                .version("1.0")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap_err();

    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_workflow_execution_already_started_fault(),
        "expected WorkflowExecutionAlreadyStartedFault, got: {svc_err:?}"
    );
}

#[tokio::test]
async fn test_start_workflow_on_deprecated_type_specific_error() {
    // Moto: ex.value.response["Error"]["Code"] == "TypeDeprecatedFault"
    let client = make_swf_client().await;
    register_domain(&client, "dep-type-domain").await;
    register_workflow_type(&client, "dep-type-domain", "dep-wf", "1.0").await;

    client
        .deprecate_workflow_type()
        .domain("dep-type-domain")
        .workflow_type(
            aws_sdk_swf::types::WorkflowType::builder()
                .name("dep-wf")
                .version("1.0")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap();

    let err = client
        .start_workflow_execution()
        .domain("dep-type-domain")
        .workflow_id("wf-dep-type")
        .workflow_type(
            aws_sdk_swf::types::WorkflowType::builder()
                .name("dep-wf")
                .version("1.0")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .unwrap_err();

    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_type_deprecated_fault(),
        "expected TypeDeprecatedFault, got: {svc_err:?}"
    );
}

#[tokio::test]
async fn test_describe_non_existent_domain_specific_error() {
    // Moto: ex.value.response["Error"]["Code"] == "UnknownResourceFault"
    let client = make_swf_client().await;

    let err = client
        .describe_domain()
        .name("non-existent")
        .send()
        .await
        .unwrap_err();

    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_unknown_resource_fault(),
        "expected UnknownResourceFault, got: {svc_err:?}"
    );
}

// ==================== UndeprecateDomain tests ====================

#[tokio::test]
async fn test_undeprecate_domain() {
    let client = make_swf_client().await;
    register_domain(&client, "undep-domain").await;

    // Deprecate first
    client
        .deprecate_domain()
        .name("undep-domain")
        .send()
        .await
        .expect("deprecate_domain should succeed");

    // Verify it's deprecated
    let resp = client
        .describe_domain()
        .name("undep-domain")
        .send()
        .await
        .unwrap();
    assert_eq!(
        resp.domain_info().unwrap().status(),
        &RegistrationStatus::Deprecated
    );

    // Undeprecate
    client
        .undeprecate_domain()
        .name("undep-domain")
        .send()
        .await
        .expect("undeprecate_domain should succeed");

    // Verify it's registered again
    let resp = client
        .describe_domain()
        .name("undep-domain")
        .send()
        .await
        .unwrap();
    assert_eq!(
        resp.domain_info().unwrap().status(),
        &RegistrationStatus::Registered
    );
}

#[tokio::test]
async fn test_undeprecate_active_domain_fails() {
    let client = make_swf_client().await;
    register_domain(&client, "active-domain").await;

    // Undeprecating a registered domain should fail
    let err = client
        .undeprecate_domain()
        .name("active-domain")
        .send()
        .await
        .unwrap_err();

    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_domain_already_exists_fault(),
        "expected DomainAlreadyExistsFault, got: {svc_err:?}"
    );
}

// ==================== SignalWorkflowExecution tests ====================

#[tokio::test]
async fn test_signal_workflow_execution() {
    let client = make_swf_client().await;
    register_domain(&client, "signal-domain").await;
    register_workflow_type(&client, "signal-domain", "MyWorkflow", "1.0").await;

    let run_id = start_workflow(&client, "signal-domain", "signal-wf", "MyWorkflow", "1.0").await;

    // Signal the workflow
    client
        .signal_workflow_execution()
        .domain("signal-domain")
        .workflow_id("signal-wf")
        .signal_name("MySignal")
        .input("signal-data")
        .send()
        .await
        .expect("signal_workflow_execution should succeed");

    // Verify history has the signal event
    let history = client
        .get_workflow_execution_history()
        .domain("signal-domain")
        .execution(
            aws_sdk_swf::types::WorkflowExecution::builder()
                .workflow_id("signal-wf")
                .run_id(&run_id)
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("get_workflow_execution_history should succeed");

    let event_types: Vec<String> = history
        .events()
        .iter()
        .map(|e| e.event_type().as_str().to_string())
        .collect();
    assert!(
        event_types.iter().any(|e| e == "WorkflowExecutionSignaled"),
        "Expected WorkflowExecutionSignaled event, got: {event_types:?}"
    );
}

#[tokio::test]
async fn test_signal_nonexistent_workflow_fails() {
    let client = make_swf_client().await;
    register_domain(&client, "sig-fail-domain").await;

    let err = client
        .signal_workflow_execution()
        .domain("sig-fail-domain")
        .workflow_id("nonexistent-wf")
        .signal_name("MySignal")
        .send()
        .await
        .unwrap_err();

    let svc_err = err.into_service_error();
    assert!(
        svc_err.is_unknown_resource_fault(),
        "expected UnknownResourceFault, got: {svc_err:?}"
    );
}

// ==================== RespondDecisionTaskCompleted tests ====================

#[tokio::test]
async fn test_respond_decision_task_completed() {
    let client = make_swf_client().await;
    register_domain(&client, "rdt-domain").await;
    register_workflow_type(&client, "rdt-domain", "RDTWorkflow", "1.0").await;

    start_workflow(&client, "rdt-domain", "rdt-wf", "RDTWorkflow", "1.0").await;

    // Poll for a decision task
    let poll_resp = client
        .poll_for_decision_task()
        .domain("rdt-domain")
        .task_list(
            aws_sdk_swf::types::TaskList::builder()
                .name("default")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("poll_for_decision_task should succeed");

    let task_token = poll_resp.task_token();
    assert!(
        !task_token.is_empty(),
        "task_token should be non-empty when a workflow is open"
    );

    // Respond to the decision task
    client
        .respond_decision_task_completed()
        .task_token(task_token)
        .send()
        .await
        .expect("respond_decision_task_completed should succeed");
}

// ==================== RespondActivityTaskCompleted/Failed tests ====================

#[tokio::test]
async fn test_respond_activity_task_completed() {
    let client = make_swf_client().await;
    register_domain(&client, "rat-domain").await;
    register_workflow_type(&client, "rat-domain", "RATWorkflow", "1.0").await;

    start_workflow(&client, "rat-domain", "rat-wf", "RATWorkflow", "1.0").await;

    // Poll for activity task
    let poll_resp = client
        .poll_for_activity_task()
        .domain("rat-domain")
        .task_list(
            aws_sdk_swf::types::TaskList::builder()
                .name("default")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("poll_for_activity_task should succeed");

    let task_token = poll_resp.task_token();
    assert!(
        !task_token.is_empty(),
        "task_token should be non-empty when a workflow is open with matching task_list"
    );

    // Complete the activity task
    client
        .respond_activity_task_completed()
        .task_token(task_token)
        .result("activity-result")
        .send()
        .await
        .expect("respond_activity_task_completed should succeed");
}

#[tokio::test]
async fn test_respond_activity_task_failed() {
    let client = make_swf_client().await;
    register_domain(&client, "raf-domain").await;
    register_workflow_type(&client, "raf-domain", "RAFWorkflow", "1.0").await;

    start_workflow(&client, "raf-domain", "raf-wf", "RAFWorkflow", "1.0").await;

    // Poll for activity task
    let poll_resp = client
        .poll_for_activity_task()
        .domain("raf-domain")
        .task_list(
            aws_sdk_swf::types::TaskList::builder()
                .name("default")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("poll_for_activity_task should succeed");

    let task_token = poll_resp.task_token();

    // Fail the activity task
    client
        .respond_activity_task_failed()
        .task_token(task_token)
        .reason("something went wrong")
        .details("detailed error info")
        .send()
        .await
        .expect("respond_activity_task_failed should succeed");
}

// ==================== RecordActivityTaskHeartbeat tests ====================

#[tokio::test]
async fn test_record_activity_task_heartbeat() {
    let client = make_swf_client().await;
    register_domain(&client, "hb-domain").await;
    register_workflow_type(&client, "hb-domain", "HBWorkflow", "1.0").await;

    start_workflow(&client, "hb-domain", "hb-wf", "HBWorkflow", "1.0").await;

    // Poll for activity task to get a valid token
    let poll_resp = client
        .poll_for_activity_task()
        .domain("hb-domain")
        .task_list(
            aws_sdk_swf::types::TaskList::builder()
                .name("default")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("poll_for_activity_task should succeed");

    let task_token = poll_resp.task_token();

    // Send heartbeat
    let hb_resp = client
        .record_activity_task_heartbeat()
        .task_token(task_token)
        .details("still working")
        .send()
        .await
        .expect("record_activity_task_heartbeat should succeed");

    // cancelRequested should be false (no cancel has been requested)
    assert!(
        !hb_resp.cancel_requested(),
        "cancelRequested should be false"
    );
}

// ==================== ListActivityTypes / ListWorkflowTypes tests ====================

#[tokio::test]
async fn test_list_activity_types() {
    let client = make_swf_client().await;
    register_domain(&client, "lat-domain").await;
    register_activity_type(&client, "lat-domain", "ActivityA", "1.0").await;
    register_activity_type(&client, "lat-domain", "ActivityB", "2.0").await;

    let resp = client
        .list_activity_types()
        .domain("lat-domain")
        .registration_status(RegistrationStatus::Registered)
        .send()
        .await
        .expect("list_activity_types should succeed");

    let types = resp.type_infos();
    assert_eq!(types.len(), 2, "Expected 2 activity types");
    let names: Vec<&str> = types
        .iter()
        .filter_map(|t| t.activity_type().map(|at| at.name()))
        .collect();
    assert!(names.contains(&"ActivityA"), "Expected ActivityA");
    assert!(names.contains(&"ActivityB"), "Expected ActivityB");
}

#[tokio::test]
async fn test_list_workflow_types() {
    let client = make_swf_client().await;
    register_domain(&client, "lwt-domain").await;
    register_workflow_type(&client, "lwt-domain", "WorkflowX", "1.0").await;
    register_workflow_type(&client, "lwt-domain", "WorkflowY", "1.0").await;

    let resp = client
        .list_workflow_types()
        .domain("lwt-domain")
        .registration_status(RegistrationStatus::Registered)
        .send()
        .await
        .expect("list_workflow_types should succeed");

    let types = resp.type_infos();
    assert_eq!(types.len(), 2, "Expected 2 workflow types");
    let names: Vec<&str> = types
        .iter()
        .filter_map(|t| t.workflow_type().map(|wt| wt.name()))
        .collect();
    assert!(names.contains(&"WorkflowX"), "Expected WorkflowX");
    assert!(names.contains(&"WorkflowY"), "Expected WorkflowY");
}
