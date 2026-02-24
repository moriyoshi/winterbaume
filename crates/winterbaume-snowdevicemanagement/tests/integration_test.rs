use std::sync::{Arc, Mutex};

use aws_sdk_snowdevicemanagement::config::BehaviorVersion;
use aws_sdk_snowdevicemanagement::types::{Command, Reboot};
use winterbaume_core::{MockAws, StatefulService};
use winterbaume_snowdevicemanagement::SnowDeviceManagementService;

async fn make_client() -> aws_sdk_snowdevicemanagement::Client {
    let mock = MockAws::builder()
        .with_service(SnowDeviceManagementService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_snowdevicemanagement::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;
    aws_sdk_snowdevicemanagement::Client::new(&config)
}

#[tokio::test]
async fn test_list_devices_seeded() {
    let client = make_client().await;
    let resp = client.list_devices().send().await.expect("list_devices");
    assert_eq!(resp.devices().len(), 1);
    assert_eq!(resp.devices()[0].managed_device_id(), Some("smd-default"));
}

#[tokio::test]
async fn test_describe_device() {
    let client = make_client().await;
    let resp = client
        .describe_device()
        .managed_device_id("smd-default")
        .send()
        .await
        .expect("describe_device");
    assert_eq!(resp.managed_device_id(), Some("smd-default"));
    assert_eq!(resp.device_state().map(|s| s.as_str()), Some("ACTIVATED"));
}

#[tokio::test]
async fn test_describe_device_not_found() {
    let client = make_client().await;
    let err = client
        .describe_device()
        .managed_device_id("missing")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_task_lifecycle() {
    let client = make_client().await;
    // Seed a device first via list call.
    let _ = client.list_devices().send().await.expect("list_devices");

    let cmd = Command::Reboot(Reboot::builder().build());
    let create = client
        .create_task()
        .command(cmd)
        .targets("smd-default")
        .description("reboot test")
        .send()
        .await
        .expect("create_task");
    let task_id = create.task_id().expect("task_id").to_string();

    let desc = client
        .describe_task()
        .task_id(&task_id)
        .send()
        .await
        .expect("describe");
    assert_eq!(desc.state().map(|s| s.as_str()), Some("QUEUED"));

    let list = client.list_tasks().send().await.expect("list");
    assert_eq!(list.tasks().len(), 1);

    // Verify execution was auto-created.
    let exec = client
        .describe_execution()
        .task_id(&task_id)
        .managed_device_id("smd-default")
        .send()
        .await
        .expect("execution");
    assert_eq!(exec.state().map(|s| s.as_str()), Some("QUEUED"));

    client
        .cancel_task()
        .task_id(&task_id)
        .send()
        .await
        .expect("cancel");

    let after = client
        .describe_task()
        .task_id(&task_id)
        .send()
        .await
        .expect("describe after");
    assert_eq!(after.state().map(|s| s.as_str()), Some("CANCELLED"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = SnowDeviceManagementService::new();
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
    assert_eq!(events.lock().unwrap().len(), 1);
}
