use std::sync::{Arc, Mutex};

use aws_sdk_applicationdiscovery::config::BehaviorVersion;
use aws_sdk_applicationdiscovery::types::Tag;
use winterbaume_applicationdiscoveryservice::ApplicationDiscoveryService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_applicationdiscovery::Client {
    let mock = MockAws::builder()
        .with_service(ApplicationDiscoveryService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_applicationdiscovery::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;
    aws_sdk_applicationdiscovery::Client::new(&config)
}

#[tokio::test]
async fn test_application_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_application()
        .name("MyApp")
        .description("test")
        .send()
        .await
        .expect("create");
    let id = create.configuration_id().expect("id").to_string();
    assert!(id.starts_with("d-application-"));

    let list = client
        .list_configurations()
        .configuration_type("APPLICATION".into())
        .send()
        .await
        .expect("list");
    assert_eq!(list.configurations().len(), 1);

    client
        .update_application()
        .configuration_id(&id)
        .name("Renamed")
        .send()
        .await
        .expect("update");

    client
        .delete_applications()
        .configuration_ids(&id)
        .send()
        .await
        .expect("delete");

    let list2 = client
        .list_configurations()
        .configuration_type("APPLICATION".into())
        .send()
        .await
        .expect("list2");
    assert!(list2.configurations().is_empty());
}

#[tokio::test]
async fn test_tags_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_application()
        .name("Tagged")
        .send()
        .await
        .expect("create");
    let id = create.configuration_id().unwrap().to_string();

    client
        .create_tags()
        .configuration_ids(&id)
        .tags(Tag::builder().key("Env").value("prod").build().unwrap())
        .send()
        .await
        .expect("create tags");

    let resp = client.describe_tags().send().await.expect("desc tags");
    assert_eq!(resp.tags().len(), 1);

    client
        .delete_tags()
        .configuration_ids(&id)
        .tags(Tag::builder().key("Env").value("prod").build().unwrap())
        .send()
        .await
        .expect("delete tags");

    let resp2 = client.describe_tags().send().await.expect("desc tags2");
    assert!(resp2.tags().is_empty());
}

#[tokio::test]
async fn test_describe_agents_default_empty() {
    let client = make_client().await;
    let resp = client.describe_agents().send().await.expect("describe");
    assert!(resp.agents_info().is_empty());
}

#[tokio::test]
async fn test_describe_continuous_exports_after_start() {
    let client = make_client().await;
    let resp = client
        .start_continuous_export()
        .send()
        .await
        .expect("start");
    let export_id = resp.export_id().expect("id").to_string();

    let list = client
        .describe_continuous_exports()
        .send()
        .await
        .expect("describe");
    assert_eq!(list.descriptions().len(), 1);

    client
        .stop_continuous_export()
        .export_id(&export_id)
        .send()
        .await
        .expect("stop");
}

#[tokio::test]
async fn test_start_import_task_records_state() {
    let client = make_client().await;
    let resp = client
        .start_import_task()
        .name("import1")
        .import_url("https://example.com/data.csv")
        .send()
        .await
        .expect("start import");
    let task = resp.task().expect("task");
    assert_eq!(task.name(), Some("import1"));

    let list = client
        .describe_import_tasks()
        .send()
        .await
        .expect("desc imports");
    assert_eq!(list.tasks().len(), 1);
}

#[tokio::test]
async fn test_start_export_task_records_state() {
    let client = make_client().await;
    client
        .start_export_task()
        .send()
        .await
        .expect("start export");

    let list = client
        .describe_export_tasks()
        .send()
        .await
        .expect("desc exports");
    assert_eq!(list.exports_info().len(), 1);
}

#[tokio::test]
async fn test_associate_disassociate_configuration_items() {
    let client = make_client().await;
    let app = client
        .create_application()
        .name("AssocApp")
        .send()
        .await
        .expect("create");
    let app_id = app.configuration_id().unwrap().to_string();

    client
        .associate_configuration_items_to_application()
        .application_configuration_id(&app_id)
        .configuration_ids("d-server-001")
        .send()
        .await
        .expect("associate");

    client
        .disassociate_configuration_items_from_application()
        .application_configuration_id(&app_id)
        .configuration_ids("d-server-001")
        .send()
        .await
        .expect("disassociate");
}

#[tokio::test]
async fn test_get_discovery_summary() {
    let client = make_client().await;
    client
        .create_application()
        .name("S1")
        .send()
        .await
        .expect("c1");
    client
        .create_application()
        .name("S2")
        .send()
        .await
        .expect("c2");

    let resp = client
        .get_discovery_summary()
        .send()
        .await
        .expect("summary");
    assert_eq!(resp.applications(), 2);
}

#[tokio::test]
async fn test_batch_delete_task_lifecycle() {
    let client = make_client().await;
    let resp = client
        .start_batch_delete_configuration_task()
        .configuration_type("SERVER".into())
        .send()
        .await
        .expect("start");
    let id = resp.task_id().expect("id").to_string();

    let desc = client
        .describe_batch_delete_configuration_task()
        .task_id(&id)
        .send()
        .await
        .expect("desc");
    assert_eq!(
        desc.task().unwrap().status(),
        Some(
            &aws_sdk_applicationdiscovery::types::BatchDeleteConfigurationTaskStatus::Initializing
        )
    );
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = ApplicationDiscoveryService::new();
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

#[tokio::test]
async fn test_state_view_round_trip() {
    use winterbaume_applicationdiscoveryservice::views::{
        ApplicationDiscoveryStateView, ApplicationView,
    };
    let svc = ApplicationDiscoveryService::new();
    let mut view = ApplicationDiscoveryStateView::default();
    view.applications.insert(
        "d-application-seed".to_string(),
        ApplicationView {
            configuration_id: "d-application-seed".to_string(),
            name: "Seed".to_string(),
            ..Default::default()
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .expect("restore");
    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snap.applications.len(), 1);
}
