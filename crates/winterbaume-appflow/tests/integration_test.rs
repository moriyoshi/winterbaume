use std::sync::{Arc, Mutex};

use aws_sdk_appflow::config::BehaviorVersion;
use aws_sdk_appflow::types::{
    ConnectorType, DestinationConnectorProperties, DestinationFlowConfig, S3DestinationProperties,
    S3SourceProperties, SourceConnectorProperties, SourceFlowConfig, Task, TaskType, TriggerConfig,
    TriggerType,
};
use winterbaume_appflow::AppFlowService;
use winterbaume_core::{MockAws, StatefulService};

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

fn minimal_create_args() -> (
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
                    .bucket_name("source-bucket")
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
                    .bucket_name("dest-bucket")
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

#[tokio::test]
async fn test_flow_lifecycle() {
    let client = make_client().await;
    let (trigger, source, dest, tasks) = minimal_create_args();

    let create = client
        .create_flow()
        .flow_name("test-flow")
        .trigger_config(trigger.clone())
        .source_flow_config(source.clone())
        .destination_flow_config_list(dest.clone())
        .set_tasks(Some(tasks.clone()))
        .send()
        .await
        .expect("create_flow");
    let arn = create.flow_arn().expect("arn").to_string();
    assert!(arn.contains("flow/test-flow"));
    assert!(create.flow_status().is_some());

    let describe = client
        .describe_flow()
        .flow_name("test-flow")
        .send()
        .await
        .expect("describe_flow");
    assert_eq!(describe.flow_name(), Some("test-flow"));
    assert_eq!(describe.flow_arn(), Some(arn.as_str()));

    client
        .delete_flow()
        .flow_name("test-flow")
        .send()
        .await
        .expect("delete_flow");

    let err = client
        .describe_flow()
        .flow_name("test-flow")
        .send()
        .await
        .expect_err("describe after delete should fail");
    assert!(format!("{:?}", err).contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_create_duplicate_flow_fails() {
    let client = make_client().await;
    let (trigger, source, dest, tasks) = minimal_create_args();
    client
        .create_flow()
        .flow_name("dup-flow")
        .trigger_config(trigger.clone())
        .source_flow_config(source.clone())
        .destination_flow_config_list(dest.clone())
        .set_tasks(Some(tasks.clone()))
        .send()
        .await
        .expect("first create");
    let err = client
        .create_flow()
        .flow_name("dup-flow")
        .trigger_config(trigger)
        .source_flow_config(source)
        .destination_flow_config_list(dest)
        .set_tasks(Some(tasks))
        .send()
        .await
        .expect_err("dup create should fail");
    assert!(format!("{:?}", err).contains("ConflictException"));
}

#[tokio::test]
async fn test_list_flows() {
    let client = make_client().await;
    let (trigger, source, dest, tasks) = minimal_create_args();
    for n in ["a-flow", "b-flow", "c-flow"] {
        client
            .create_flow()
            .flow_name(n)
            .trigger_config(trigger.clone())
            .source_flow_config(source.clone())
            .destination_flow_config_list(dest.clone())
            .set_tasks(Some(tasks.clone()))
            .send()
            .await
            .expect("create");
    }
    let resp = client.list_flows().send().await.expect("list");
    assert_eq!(resp.flows().len(), 3);
}

#[tokio::test]
async fn test_start_and_stop_flow() {
    let client = make_client().await;
    let (trigger, source, dest, tasks) = minimal_create_args();
    client
        .create_flow()
        .flow_name("start-flow")
        .trigger_config(trigger)
        .source_flow_config(source)
        .destination_flow_config_list(dest)
        .set_tasks(Some(tasks))
        .send()
        .await
        .expect("create");

    let start = client
        .start_flow()
        .flow_name("start-flow")
        .send()
        .await
        .expect("start_flow");
    assert!(start.execution_id().is_some());
    assert_eq!(start.flow_status().map(|s| s.as_str()), Some("Active"));

    let stop = client
        .stop_flow()
        .flow_name("start-flow")
        .send()
        .await
        .expect("stop_flow");
    assert_eq!(stop.flow_status().map(|s| s.as_str()), Some("Suspended"));
}

#[tokio::test]
async fn test_describe_missing_flow() {
    let client = make_client().await;
    let err = client
        .describe_flow()
        .flow_name("missing")
        .send()
        .await
        .expect_err("missing flow should fail");
    assert!(format!("{:?}", err).contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = AppFlowService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore("123456789012", "us-east-1", Default::default())
        .await
        .expect("restore");
    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
}
