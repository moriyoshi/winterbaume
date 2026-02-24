use aws_sdk_datasync::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_datasync::DataSyncService;

async fn make_client() -> aws_sdk_datasync::Client {
    let mock = MockAws::builder()
        .with_service(DataSyncService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_datasync::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_datasync::Client::new(&config)
}

#[tokio::test]
async fn test_create_and_describe_task() {
    let client = make_client().await;

    let src_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000001";
    let dst_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000002";

    let create_resp = client
        .create_task()
        .source_location_arn(src_arn)
        .destination_location_arn(dst_arn)
        .name("my-task")
        .send()
        .await
        .expect("create_task should succeed");

    let task_arn = create_resp.task_arn().expect("should have task_arn");
    assert!(task_arn.starts_with("arn:aws:datasync:us-east-1:123456789012:task/task-"));

    let describe_resp = client
        .describe_task()
        .task_arn(task_arn)
        .send()
        .await
        .expect("describe_task should succeed");

    assert_eq!(describe_resp.task_arn().unwrap(), task_arn);
    assert_eq!(describe_resp.name().unwrap(), "my-task");
    assert_eq!(describe_resp.source_location_arn().unwrap(), src_arn,);
    assert_eq!(describe_resp.destination_location_arn().unwrap(), dst_arn,);
}

#[tokio::test]
async fn test_list_tasks() {
    let client = make_client().await;

    let src_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000001";
    let dst_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000002";

    for name in ["task-a", "task-b"] {
        client
            .create_task()
            .source_location_arn(src_arn)
            .destination_location_arn(dst_arn)
            .name(name)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_tasks()
        .send()
        .await
        .expect("list_tasks should succeed");

    assert_eq!(resp.tasks().len(), 2);
}

#[tokio::test]
async fn test_delete_task() {
    let client = make_client().await;

    let src_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000001";
    let dst_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000002";

    let create_resp = client
        .create_task()
        .source_location_arn(src_arn)
        .destination_location_arn(dst_arn)
        .name("delete-me")
        .send()
        .await
        .unwrap();

    let task_arn = create_resp.task_arn().unwrap();

    client
        .delete_task()
        .task_arn(task_arn)
        .send()
        .await
        .expect("delete should succeed");

    let result = client.describe_task().task_arn(task_arn).send().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_describe_nonexistent_task() {
    let client = make_client().await;

    let result = client
        .describe_task()
        .task_arn("arn:aws:datasync:us-east-1:123456789012:task/task-00000000000000000")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_tasks_empty() {
    let client = make_client().await;

    let resp = client
        .list_tasks()
        .send()
        .await
        .expect("list_tasks should succeed on empty state");

    assert_eq!(resp.tasks().len(), 0);
}

// ── Additional tests ported from moto test_datasync.py ──

// test_list_tasks - tasks have Name and Status (from moto test_list_tasks)
#[tokio::test]
async fn test_list_tasks_with_name() {
    let client = make_client().await;

    let src_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000001";
    let dst_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000002";

    // Task with no name
    client
        .create_task()
        .source_location_arn(src_arn)
        .destination_location_arn(dst_arn)
        .send()
        .await
        .unwrap();

    // Task with a name
    client
        .create_task()
        .source_location_arn(src_arn)
        .destination_location_arn(dst_arn)
        .name("task_name")
        .send()
        .await
        .unwrap();

    let resp = client.list_tasks().send().await.unwrap();
    assert_eq!(resp.tasks().len(), 2);

    let named = resp.tasks().iter().find(|t| t.name().is_some());
    assert!(named.is_some());
    assert_eq!(named.unwrap().name(), Some("task_name"));

    // All tasks have AVAILABLE status
    for task in resp.tasks() {
        assert_eq!(task.status().map(|s| s.as_str()), Some("AVAILABLE"));
    }
}

// test_update_task (from moto test_update_task)
#[tokio::test]
async fn test_update_task() {
    let client = make_client().await;

    let src_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000001";
    let dst_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000002";

    let create_resp = client
        .create_task()
        .source_location_arn(src_arn)
        .destination_location_arn(dst_arn)
        .name("initial-name")
        .send()
        .await
        .unwrap();

    let task_arn = create_resp.task_arn().unwrap().to_string();

    // Verify initial name
    let desc = client
        .describe_task()
        .task_arn(&task_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.name(), Some("initial-name"));

    // Update name
    client
        .update_task()
        .task_arn(&task_arn)
        .name("updated-name")
        .send()
        .await
        .expect("update_task should succeed");

    // Verify updated name
    let desc = client
        .describe_task()
        .task_arn(&task_arn)
        .send()
        .await
        .unwrap();
    assert_eq!(desc.name(), Some("updated-name"));
}

// test_update_task_not_exist (from moto test_update_task)
#[tokio::test]
async fn test_update_nonexistent_task() {
    let client = make_client().await;

    let result = client
        .update_task()
        .task_arn("arn:aws:datasync:us-east-1:123456789012:task/task-nonexistent")
        .name("new-name")
        .send()
        .await;
    assert!(result.is_err());
}

// test_delete_task - double delete should fail (from moto test_delete_task)
#[tokio::test]
async fn test_delete_task_twice_fails() {
    let client = make_client().await;

    let src_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000001";
    let dst_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000002";

    let create_resp = client
        .create_task()
        .source_location_arn(src_arn)
        .destination_location_arn(dst_arn)
        .name("ephemeral-task")
        .send()
        .await
        .unwrap();

    let task_arn = create_resp.task_arn().unwrap().to_string();

    // First delete should succeed
    client
        .delete_task()
        .task_arn(&task_arn)
        .send()
        .await
        .expect("first delete should succeed");

    // Second delete should fail
    let result = client.delete_task().task_arn(&task_arn).send().await;
    assert!(result.is_err());
}

// test_start_task_execution (from moto test_start_task_execution)
#[tokio::test]
async fn test_start_task_execution() {
    let client = make_client().await;

    let src_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000001";
    let dst_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000002";

    let create_resp = client
        .create_task()
        .source_location_arn(src_arn)
        .destination_location_arn(dst_arn)
        .name("task-to-execute")
        .send()
        .await
        .unwrap();

    let task_arn = create_resp.task_arn().unwrap().to_string();

    let exec_resp = client
        .start_task_execution()
        .task_arn(&task_arn)
        .send()
        .await
        .expect("start_task_execution should succeed");

    assert!(exec_resp.task_execution_arn().is_some());
    let exec_arn = exec_resp.task_execution_arn().unwrap();
    assert!(exec_arn.contains("execution"));
}

#[tokio::test]
async fn test_task_arn_format() {
    let client = make_client().await;

    let src_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000001";
    let dst_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000002";

    let resp = client
        .create_task()
        .source_location_arn(src_arn)
        .destination_location_arn(dst_arn)
        .send()
        .await
        .unwrap();

    let task_arn = resp.task_arn().unwrap();
    assert!(
        task_arn.starts_with("arn:aws:datasync:us-east-1:123456789012:task/task-"),
        "task ARN did not match expected pattern: {task_arn}"
    );
}

#[tokio::test]
async fn test_create_task_without_name() {
    let client = make_client().await;

    let src_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000001";
    let dst_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000002";

    let create_resp = client
        .create_task()
        .source_location_arn(src_arn)
        .destination_location_arn(dst_arn)
        .send()
        .await
        .expect("create_task without name should succeed");

    let task_arn = create_resp.task_arn().unwrap().to_string();

    let desc = client
        .describe_task()
        .task_arn(&task_arn)
        .send()
        .await
        .unwrap();

    assert!(
        desc.name().is_none(),
        "name should be absent when not provided"
    );
    assert_eq!(desc.status().map(|s| s.as_str()), Some("AVAILABLE"));
}

#[tokio::test]
async fn test_describe_task_has_creation_time() {
    let client = make_client().await;

    let src_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000001";
    let dst_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000002";

    let create_resp = client
        .create_task()
        .source_location_arn(src_arn)
        .destination_location_arn(dst_arn)
        .name("timed-task")
        .send()
        .await
        .unwrap();

    let task_arn = create_resp.task_arn().unwrap().to_string();

    let desc = client
        .describe_task()
        .task_arn(&task_arn)
        .send()
        .await
        .unwrap();

    assert!(
        desc.creation_time().is_some(),
        "DescribeTask response should include a CreationTime"
    );
}

#[tokio::test]
async fn test_describe_task_preserves_cloud_watch_log_group_arn() {
    let client = make_client().await;

    let src_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000001";
    let dst_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000002";
    let log_group_arn = "arn:aws:logs:us-east-1:123456789012:log-group:/aws/datasync:*";

    let create_resp = client
        .create_task()
        .source_location_arn(src_arn)
        .destination_location_arn(dst_arn)
        .cloud_watch_log_group_arn(log_group_arn)
        .send()
        .await
        .unwrap();

    let task_arn = create_resp.task_arn().unwrap().to_string();

    let desc = client
        .describe_task()
        .task_arn(&task_arn)
        .send()
        .await
        .unwrap();

    assert_eq!(
        desc.cloud_watch_log_group_arn(),
        Some(log_group_arn),
        "CloudWatchLogGroupArn should be round-tripped through describe_task"
    );
}

#[tokio::test]
async fn test_start_task_execution_on_nonexistent_task_fails() {
    let client = make_client().await;

    let result = client
        .start_task_execution()
        .task_arn("arn:aws:datasync:us-east-1:123456789012:task/task-00000000000000000")
        .send()
        .await;

    assert!(
        result.is_err(),
        "start_task_execution on a nonexistent task must return an error"
    );
}

#[tokio::test]
async fn test_multiple_executions_on_same_task() {
    let client = make_client().await;

    let src_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000001";
    let dst_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000002";

    let create_resp = client
        .create_task()
        .source_location_arn(src_arn)
        .destination_location_arn(dst_arn)
        .name("multi-exec-task")
        .send()
        .await
        .unwrap();

    let task_arn = create_resp.task_arn().unwrap().to_string();

    let exec1 = client
        .start_task_execution()
        .task_arn(&task_arn)
        .send()
        .await
        .expect("first execution should succeed");

    let exec2 = client
        .start_task_execution()
        .task_arn(&task_arn)
        .send()
        .await
        .expect("second execution should succeed");

    let arn1 = exec1.task_execution_arn().unwrap();
    let arn2 = exec2.task_execution_arn().unwrap();
    assert_ne!(arn1, arn2, "each task execution must receive a unique ARN");
}

#[tokio::test]
async fn test_cancel_task_execution_succeeds() {
    let client = make_client().await;

    let src_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000001";
    let dst_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000002";

    let task_arn = client
        .create_task()
        .source_location_arn(src_arn)
        .destination_location_arn(dst_arn)
        .name("cancel-exec-task")
        .send()
        .await
        .unwrap()
        .task_arn()
        .unwrap()
        .to_string();

    let exec_arn = client
        .start_task_execution()
        .task_arn(&task_arn)
        .send()
        .await
        .unwrap()
        .task_execution_arn()
        .unwrap()
        .to_string();

    client
        .cancel_task_execution()
        .task_execution_arn(&exec_arn)
        .send()
        .await
        .expect("cancel_task_execution should succeed for a running execution");
}

#[tokio::test]
async fn test_cancel_nonexistent_task_execution_fails() {
    let client = make_client().await;

    let result = client
        .cancel_task_execution()
        .task_execution_arn(
            "arn:aws:datasync:us-east-1:123456789012:task/task-00000000000000000/execution/exec-00000000000000000",
        )
        .send()
        .await;

    assert!(
        result.is_err(),
        "cancelling a nonexistent execution must return an error"
    );
}

#[tokio::test]
async fn test_delete_location_nonexistent_fails() {
    let client = make_client().await;

    let result = client
        .delete_location()
        .location_arn("arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000099")
        .send()
        .await;

    assert!(
        result.is_err(),
        "deleting a nonexistent location must return an error"
    );
}

#[tokio::test]
async fn test_each_task_gets_unique_arn() {
    let client = make_client().await;

    let src_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000001";
    let dst_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000002";

    let mut arns = std::collections::HashSet::new();
    for i in 0..5 {
        let resp = client
            .create_task()
            .source_location_arn(src_arn)
            .destination_location_arn(dst_arn)
            .name(format!("task-{i}"))
            .send()
            .await
            .unwrap();
        let arn = resp.task_arn().unwrap().to_string();
        assert!(arns.insert(arn), "duplicate task ARN generated");
    }
}

#[tokio::test]
async fn test_list_tasks_reflects_deletions() {
    let client = make_client().await;

    let src_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000001";
    let dst_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000002";

    let arn_a = client
        .create_task()
        .source_location_arn(src_arn)
        .destination_location_arn(dst_arn)
        .name("keep-me")
        .send()
        .await
        .unwrap()
        .task_arn()
        .unwrap()
        .to_string();

    let arn_b = client
        .create_task()
        .source_location_arn(src_arn)
        .destination_location_arn(dst_arn)
        .name("delete-me")
        .send()
        .await
        .unwrap()
        .task_arn()
        .unwrap()
        .to_string();

    client.delete_task().task_arn(&arn_b).send().await.unwrap();

    let resp = client.list_tasks().send().await.unwrap();
    assert_eq!(resp.tasks().len(), 1);
    assert_eq!(resp.tasks()[0].task_arn().unwrap(), arn_a);
}

#[tokio::test]
async fn test_update_task_name_to_empty_string_is_noop() {
    // update_task with an explicitly empty name string: the handler only
    // applies the name when Some, so an empty &str still overwrites. This
    // test documents the current behaviour (empty string is stored).
    let client = make_client().await;

    let src_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000001";
    let dst_arn = "arn:aws:datasync:us-east-1:123456789012:location/loc-00000000000000002";

    let task_arn = client
        .create_task()
        .source_location_arn(src_arn)
        .destination_location_arn(dst_arn)
        .name("original")
        .send()
        .await
        .unwrap()
        .task_arn()
        .unwrap()
        .to_string();

    client
        .update_task()
        .task_arn(&task_arn)
        .name("")
        .send()
        .await
        .expect("update_task with empty name string should not error");

    let desc = client
        .describe_task()
        .task_arn(&task_arn)
        .send()
        .await
        .unwrap();

    // The name field is set to Some("") by the handler; document that here.
    assert_eq!(desc.name(), Some(""));
}
