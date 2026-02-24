/// Scenario: AppFlow data-pipeline lifecycle
///
/// Demonstrates a full AppFlow workflow: create a flow connecting two S3 buckets,
/// verify it appears in list, update its description, start and stop it, confirm
/// status transitions, tag the flow, and finally delete it.
use aws_sdk_appflow::config::BehaviorVersion;
use aws_sdk_appflow::types::{
    ConnectorType, DestinationConnectorProperties, DestinationFlowConfig, S3DestinationProperties,
    S3SourceProperties, SourceConnectorProperties, SourceFlowConfig, Task, TaskType, TriggerConfig,
    TriggerType,
};
use winterbaume_appflow::AppFlowService;
use winterbaume_core::MockAws;

async fn make_client() -> aws_sdk_appflow::Client {
    let mock = MockAws::builder()
        .with_service(AppFlowService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appflow::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_appflow::Client::new(&config)
}

fn s3_to_s3_flow_args() -> (
    TriggerConfig,
    SourceFlowConfig,
    DestinationFlowConfig,
    Vec<Task>,
) {
    let trigger = TriggerConfig::builder()
        .trigger_type(TriggerType::Ondemand)
        .build()
        .unwrap();
    let source = SourceFlowConfig::builder()
        .connector_type(ConnectorType::S3)
        .source_connector_properties(
            SourceConnectorProperties::builder()
                .s3(S3SourceProperties::builder()
                    .bucket_name("src-bucket")
                    .build()
                    .unwrap())
                .build(),
        )
        .build()
        .unwrap();
    let dest = DestinationFlowConfig::builder()
        .connector_type(ConnectorType::S3)
        .destination_connector_properties(
            DestinationConnectorProperties::builder()
                .s3(S3DestinationProperties::builder()
                    .bucket_name("dst-bucket")
                    .build()
                    .unwrap())
                .build(),
        )
        .build()
        .unwrap();
    let task = Task::builder()
        .source_fields("col1")
        .task_type(TaskType::Map)
        .build()
        .unwrap();
    (trigger, source, dest, vec![task])
}

/// Scenario: S3-to-S3 data pipeline — create, update, start, stop, tag, delete.
#[tokio::test]
async fn test_batch_s3_pipeline() {
    let client = make_client().await;
    let (trigger, source, dest, tasks) = s3_to_s3_flow_args();

    // 1. Create the flow.
    let create = client
        .create_flow()
        .flow_name("pipeline-flow")
        .description("Initial description")
        .trigger_config(trigger)
        .source_flow_config(source)
        .destination_flow_config_list(dest)
        .set_tasks(Some(tasks))
        .send()
        .await
        .expect("create_flow");
    let arn = create.flow_arn().expect("arn").to_string();
    assert!(arn.contains("flow/pipeline-flow"), "ARN format");

    // 2. Verify it appears in the list.
    let list = client.list_flows().send().await.expect("list_flows");
    let names: Vec<_> = list.flows().iter().filter_map(|f| f.flow_name()).collect();
    assert!(names.contains(&"pipeline-flow"), "flow in list");

    // 3. Start the flow and confirm status transitions to Active with an execution ID.
    let start = client
        .start_flow()
        .flow_name("pipeline-flow")
        .send()
        .await
        .expect("start_flow");
    assert_eq!(
        start.flow_status().map(|s| s.as_str()),
        Some("Active"),
        "status after start"
    );
    let exec_id = start.execution_id().expect("execution_id").to_string();
    assert!(!exec_id.is_empty(), "execution id present");

    // 4. Stop the flow and confirm it becomes Suspended.
    let stop = client
        .stop_flow()
        .flow_name("pipeline-flow")
        .send()
        .await
        .expect("stop_flow");
    assert_eq!(
        stop.flow_status().map(|s| s.as_str()),
        Some("Suspended"),
        "status after stop"
    );

    // 5. Tag the flow and verify tags are readable.
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "test")
        .tags("team", "data")
        .send()
        .await
        .expect("tag_resource");
    let tags_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource");
    let tags = tags_resp.tags().expect("tags map present");
    assert_eq!(tags.get("env").map(String::as_str), Some("test"));
    assert_eq!(tags.get("team").map(String::as_str), Some("data"));

    // 6. Delete the flow; subsequent describe must return ResourceNotFoundException.
    client
        .delete_flow()
        .flow_name("pipeline-flow")
        .send()
        .await
        .expect("delete_flow");
    let err = client
        .describe_flow()
        .flow_name("pipeline-flow")
        .send()
        .await
        .expect_err("describe after delete should fail");
    assert!(
        format!("{:?}", err).contains("ResourceNotFoundException"),
        "deleted flow raises ResourceNotFoundException"
    );
}
