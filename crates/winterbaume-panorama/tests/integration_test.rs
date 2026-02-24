use aws_sdk_panorama::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_panorama::PanoramaService;

async fn make_client() -> aws_sdk_panorama::Client {
    let mock = MockAws::builder()
        .with_service(PanoramaService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_panorama::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_panorama::Client::new(&config)
}

#[tokio::test]
async fn test_provision_and_describe_device() {
    let client = make_client().await;

    let resp = client
        .provision_device()
        .name("my-device")
        .description("A test device")
        .send()
        .await
        .expect("provision_device should succeed");

    let device_id = resp.device_id().expect("should have device_id");
    assert!(!device_id.is_empty());

    let desc = client
        .describe_device()
        .device_id(device_id)
        .send()
        .await
        .expect("describe_device should succeed");

    assert_eq!(desc.name(), Some("my-device"));
    assert_eq!(desc.description(), Some("A test device"));
}

#[tokio::test]
async fn test_list_devices() {
    let client = make_client().await;

    client
        .provision_device()
        .name("device-a")
        .send()
        .await
        .expect("provision_device a should succeed");

    client
        .provision_device()
        .name("device-b")
        .send()
        .await
        .expect("provision_device b should succeed");

    let resp = client
        .list_devices()
        .send()
        .await
        .expect("list_devices should succeed");

    assert_eq!(resp.devices().len(), 2);
}

#[tokio::test]
async fn test_delete_device() {
    let client = make_client().await;

    let prov = client
        .provision_device()
        .name("device-to-delete")
        .send()
        .await
        .unwrap();

    let device_id = prov.device_id().unwrap().to_string();

    client
        .delete_device()
        .device_id(&device_id)
        .send()
        .await
        .expect("delete_device should succeed");

    let result = client.describe_device().device_id(&device_id).send().await;
    assert!(result.is_err(), "describe after delete should fail");
}

#[tokio::test]
async fn test_update_device_metadata() {
    let client = make_client().await;

    let prov = client
        .provision_device()
        .name("device-update")
        .send()
        .await
        .unwrap();

    let device_id = prov.device_id().unwrap().to_string();

    let upd = client
        .update_device_metadata()
        .device_id(&device_id)
        .description("Updated description")
        .send()
        .await
        .expect("update_device_metadata should succeed");

    assert_eq!(upd.device_id(), Some(device_id.as_str()));
}

#[tokio::test]
async fn test_create_and_describe_application_instance() {
    let client = make_client().await;

    // First provision a device to use as default runtime context
    let prov = client
        .provision_device()
        .name("runtime-device")
        .send()
        .await
        .unwrap();
    let device_id = prov.device_id().unwrap().to_string();

    let resp = client
        .create_application_instance()
        .name("my-app")
        .default_runtime_context_device(&device_id)
        .manifest_payload(aws_sdk_panorama::types::ManifestPayload::PayloadData(
            "{}".to_string(),
        ))
        .send()
        .await
        .expect("create_application_instance should succeed");

    let app_id = resp.application_instance_id().to_string();
    assert!(!app_id.is_empty());

    let desc = client
        .describe_application_instance()
        .application_instance_id(&app_id)
        .send()
        .await
        .expect("describe_application_instance should succeed");

    assert_eq!(desc.name(), Some("my-app"));
    assert_eq!(
        desc.default_runtime_context_device(),
        Some(device_id.as_str())
    );
}

#[tokio::test]
async fn test_list_application_instances() {
    let client = make_client().await;

    let prov = client
        .provision_device()
        .name("device-for-apps")
        .send()
        .await
        .unwrap();
    let device_id = prov.device_id().unwrap().to_string();

    for name in ["app-1", "app-2"] {
        client
            .create_application_instance()
            .name(name)
            .default_runtime_context_device(&device_id)
            .manifest_payload(aws_sdk_panorama::types::ManifestPayload::PayloadData(
                "{}".to_string(),
            ))
            .send()
            .await
            .expect("create_application_instance should succeed");
    }

    let resp = client
        .list_application_instances()
        .send()
        .await
        .expect("list_application_instances should succeed");

    assert_eq!(resp.application_instances().len(), 2);
}

#[tokio::test]
async fn test_create_node_from_template_job() {
    let client = make_client().await;

    let resp = client
        .create_node_from_template_job()
        .node_name("my-node")
        .template_type(aws_sdk_panorama::types::TemplateType::RtspCameraStream)
        .output_package_name("my-package")
        .output_package_version("1.0.0")
        .send()
        .await
        .expect("create_node_from_template_job should succeed");

    let job_id = resp.job_id().to_string();
    assert!(!job_id.is_empty());

    let desc = client
        .describe_node_from_template_job()
        .job_id(&job_id)
        .send()
        .await
        .expect("describe_node_from_template_job should succeed");

    assert_eq!(desc.node_name(), "my-node");
    assert_eq!(desc.job_id(), job_id.as_str());
}

#[tokio::test]
async fn test_list_nodes() {
    let client = make_client().await;

    let resp = client
        .list_nodes()
        .send()
        .await
        .expect("list_nodes should succeed");

    // Empty list is valid
    let _ = resp.nodes();
}

#[tokio::test]
async fn test_describe_nonexistent_device_returns_error() {
    let client = make_client().await;

    let result = client
        .describe_device()
        .device_id("nonexistent-device-id")
        .send()
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_device_returns_error() {
    let client = make_client().await;

    let result = client
        .delete_device()
        .device_id("nonexistent-device-id")
        .send()
        .await;
    assert!(result.is_err());
}
