use std::sync::{Arc, Mutex};

use aws_sdk_appintegrations::config::BehaviorVersion;
use aws_sdk_appintegrations::types::{ApplicationSourceConfig, EventFilter, ExternalUrlConfig};
use winterbaume_appintegrations::AppIntegrationsService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_appintegrations::Client {
    let mock = MockAws::builder()
        .with_service(AppIntegrationsService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_appintegrations::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_appintegrations::Client::new(&config)
}

fn source_config() -> ApplicationSourceConfig {
    ApplicationSourceConfig::builder()
        .external_url_config(
            ExternalUrlConfig::builder()
                .access_url("https://example.com/app")
                .build()
                .unwrap(),
        )
        .build()
}

#[tokio::test]
async fn test_create_get_list_application() {
    let client = make_client().await;
    let create = client
        .create_application()
        .name("MyApp")
        .namespace("ns1")
        .application_source_config(source_config())
        .description("hello")
        .send()
        .await
        .expect("create");
    assert!(create.id().is_some());
    let arn = create.arn().expect("arn").to_string();

    let got = client
        .get_application()
        .arn(&arn)
        .send()
        .await
        .expect("get by arn");
    assert_eq!(got.name(), Some("MyApp"));
    assert_eq!(got.namespace(), Some("ns1"));
    assert_eq!(got.description(), Some("hello"));

    let list = client.list_applications().send().await.expect("list");
    assert_eq!(list.applications().len(), 1);
    assert_eq!(list.applications()[0].name(), Some("MyApp"));
}

#[tokio::test]
async fn test_duplicate_application_name() {
    let client = make_client().await;
    client
        .create_application()
        .name("App1")
        .namespace("ns")
        .application_source_config(source_config())
        .send()
        .await
        .expect("first");
    let err = client
        .create_application()
        .name("App1")
        .namespace("ns")
        .application_source_config(source_config())
        .send()
        .await
        .expect_err("dup");
    assert!(format!("{err:?}").contains("DuplicateResourceException"));
}

#[tokio::test]
async fn test_update_and_delete_application() {
    let client = make_client().await;
    let create = client
        .create_application()
        .name("EditMe")
        .namespace("ns")
        .application_source_config(source_config())
        .send()
        .await
        .expect("create");
    let arn = create.arn().expect("arn").to_string();

    client
        .update_application()
        .arn(&arn)
        .description("new desc")
        .send()
        .await
        .expect("update");

    let got = client
        .get_application()
        .arn(&arn)
        .send()
        .await
        .expect("get");
    assert_eq!(got.description(), Some("new desc"));

    client
        .delete_application()
        .arn(&arn)
        .send()
        .await
        .expect("delete");

    let err = client
        .get_application()
        .arn(&arn)
        .send()
        .await
        .expect_err("gone");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_data_integration_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_data_integration()
        .name("DI1")
        .kms_key("arn:aws:kms:us-east-1:123456789012:key/abc")
        .description("integ")
        .source_uri("Salesforce://AppIntegrations")
        .send()
        .await
        .expect("create di");
    let id = create.id().expect("id").to_string();
    assert_eq!(create.name(), Some("DI1"));
    assert!(create.arn().is_some());

    let got = client
        .get_data_integration()
        .identifier(&id)
        .send()
        .await
        .expect("get");
    assert_eq!(
        got.kms_key(),
        Some("arn:aws:kms:us-east-1:123456789012:key/abc")
    );

    client
        .update_data_integration()
        .identifier(&id)
        .description("v2")
        .send()
        .await
        .expect("update");

    let list = client.list_data_integrations().send().await.expect("list");
    assert_eq!(list.data_integrations().len(), 1);

    client
        .delete_data_integration()
        .data_integration_identifier(&id)
        .send()
        .await
        .expect("delete");
}

#[tokio::test]
async fn test_data_integration_association_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_data_integration()
        .name("DI-assoc")
        .kms_key("arn:aws:kms:us-east-1:123456789012:key/abc")
        .send()
        .await
        .expect("create di");
    let id = create.id().expect("id").to_string();

    let assoc = client
        .create_data_integration_association()
        .data_integration_identifier(&id)
        .client_id("client-001")
        .destination_uri("arn:aws:s3:::my-bucket")
        .send()
        .await
        .expect("create association");
    assert!(assoc.data_integration_association_id().is_some());

    let list = client
        .list_data_integration_associations()
        .data_integration_identifier(&id)
        .send()
        .await
        .expect("list assoc");
    assert_eq!(list.data_integration_associations().len(), 1);

    // Cannot delete DI with associations
    let err = client
        .delete_data_integration()
        .data_integration_identifier(&id)
        .send()
        .await
        .expect_err("in-use");
    assert!(format!("{err:?}").contains("ResourceQuotaExceededException"));
}

#[tokio::test]
async fn test_event_integration_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_event_integration()
        .name("EI1")
        .event_filter(
            EventFilter::builder()
                .source("aws.partner/foo")
                .build()
                .unwrap(),
        )
        .event_bridge_bus("default")
        .send()
        .await
        .expect("create");
    assert!(create.event_integration_arn().is_some());

    let got = client
        .get_event_integration()
        .name("EI1")
        .send()
        .await
        .expect("get");
    assert_eq!(got.event_bridge_bus(), Some("default"));
    assert_eq!(got.event_filter().unwrap().source(), "aws.partner/foo");

    client
        .update_event_integration()
        .name("EI1")
        .description("updated")
        .send()
        .await
        .expect("update");

    let got2 = client
        .get_event_integration()
        .name("EI1")
        .send()
        .await
        .expect("get2");
    assert_eq!(got2.description(), Some("updated"));

    let list = client.list_event_integrations().send().await.expect("list");
    assert_eq!(list.event_integrations().len(), 1);

    client
        .delete_event_integration()
        .name("EI1")
        .send()
        .await
        .expect("delete");
}

#[tokio::test]
async fn test_event_integration_validation() {
    let client = make_client().await;
    let err = client
        .create_event_integration()
        .name("X")
        .event_bridge_bus("default")
        .send()
        .await
        .expect_err("missing event filter");
    assert!(format!("{err:?}").contains("ValidationException"));
}

#[tokio::test]
async fn test_event_integration_associations_listing() {
    let client = make_client().await;
    client
        .create_event_integration()
        .name("EI-assoc")
        .event_filter(
            EventFilter::builder()
                .source("aws.partner/foo")
                .build()
                .unwrap(),
        )
        .event_bridge_bus("default")
        .send()
        .await
        .expect("create");

    let list = client
        .list_event_integration_associations()
        .event_integration_name("EI-assoc")
        .send()
        .await
        .expect("list");
    // No mock-side associations are created automatically.
    assert!(list.event_integration_associations().is_empty());
}

#[tokio::test]
async fn test_tag_lifecycle_for_application() {
    let client = make_client().await;
    let create = client
        .create_application()
        .name("Tagged")
        .namespace("ns")
        .application_source_config(source_config())
        .send()
        .await
        .expect("create");
    let arn = create.arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("Env", "prod")
        .send()
        .await
        .expect("tag");

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags");
    assert_eq!(tags.tags().unwrap().get("Env"), Some(&"prod".to_string()));

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("Env")
        .send()
        .await
        .expect("untag");

    let tags2 = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags2");
    assert!(tags2.tags().map(|t| t.is_empty()).unwrap_or(true));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = AppIntegrationsService::new();
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

#[tokio::test]
async fn test_state_view_round_trip() {
    use winterbaume_appintegrations::views::{AppIntegrationsStateView, ApplicationView};

    let svc = AppIntegrationsService::new();

    let mut view = AppIntegrationsStateView::default();
    view.applications.insert(
        "id-1".to_string(),
        ApplicationView {
            id: "id-1".to_string(),
            arn: "arn:aws:app-integrations:us-east-1:123456789012:application/id-1".to_string(),
            name: "snap-app".to_string(),
            namespace: "ns".to_string(),
            ..Default::default()
        },
    );

    svc.restore("123456789012", "us-east-1", view)
        .await
        .expect("restore");
    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snap.applications.len(), 1);
    assert!(snap.applications.contains_key("id-1"));
}
