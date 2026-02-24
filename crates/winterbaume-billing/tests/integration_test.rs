use std::sync::{Arc, Mutex};

use aws_sdk_billing::config::BehaviorVersion;
use aws_sdk_billing::types::ResourceTag;
use winterbaume_billing::BillingService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_billing::Client {
    let mock = MockAws::builder()
        .with_service(BillingService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_billing::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_billing::Client::new(&config)
}

async fn create_view(client: &aws_sdk_billing::Client, name: &str) -> String {
    let resp = client
        .create_billing_view()
        .name(name)
        .send()
        .await
        .expect("create");
    resp.arn().to_string()
}

#[tokio::test]
async fn test_create_get_delete_view() {
    let client = make_client().await;
    let arn = create_view(&client, "my-view").await;

    let got = client
        .get_billing_view()
        .arn(&arn)
        .send()
        .await
        .expect("get");
    let view = got.billing_view().expect("view");
    assert_eq!(view.arn(), Some(arn.as_str()));
    assert_eq!(view.name(), Some("my-view"));

    client
        .delete_billing_view()
        .arn(&arn)
        .send()
        .await
        .expect("delete");
    let err = client
        .get_billing_view()
        .arn(&arn)
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_list_views() {
    let client = make_client().await;
    create_view(&client, "view-a").await;
    create_view(&client, "view-b").await;
    let resp = client.list_billing_views().send().await.expect("list");
    assert_eq!(resp.billing_views().len(), 2);
}

#[tokio::test]
async fn test_update_view() {
    let client = make_client().await;
    let arn = create_view(&client, "renamed-pre").await;
    client
        .update_billing_view()
        .arn(&arn)
        .name("renamed-post")
        .description("test desc")
        .send()
        .await
        .expect("update");
    let got = client
        .get_billing_view()
        .arn(&arn)
        .send()
        .await
        .expect("get");
    let view = got.billing_view().expect("view");
    assert_eq!(view.name(), Some("renamed-post"));
    assert_eq!(view.description(), Some("test desc"));
}

#[tokio::test]
async fn test_associate_disassociate_sources() {
    let client = make_client().await;
    let arn = create_view(&client, "src-view").await;
    let source_arn = "arn:aws:billing::123456789012:billingview/primary";
    client
        .associate_source_views()
        .arn(&arn)
        .source_views(source_arn)
        .send()
        .await
        .expect("associate");
    let listed = client
        .list_source_views_for_billing_view()
        .arn(&arn)
        .send()
        .await
        .expect("list");
    assert_eq!(listed.source_views().len(), 1);

    client
        .disassociate_source_views()
        .arn(&arn)
        .source_views(source_arn)
        .send()
        .await
        .expect("disassociate");
    let listed = client
        .list_source_views_for_billing_view()
        .arn(&arn)
        .send()
        .await
        .expect("list");
    assert_eq!(listed.source_views().len(), 0);
}

#[tokio::test]
async fn test_tag_lifecycle() {
    let client = make_client().await;
    let arn = create_view(&client, "tagged-view").await;
    let tag = ResourceTag::builder()
        .key("env")
        .value("prod")
        .build()
        .expect("tag");
    client
        .tag_resource()
        .resource_arn(&arn)
        .resource_tags(tag)
        .send()
        .await
        .expect("tag");
    let listed = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list");
    let tags = listed.resource_tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "env");
    assert_eq!(tags[0].value(), Some("prod"));

    client
        .untag_resource()
        .resource_arn(&arn)
        .resource_tag_keys("env")
        .send()
        .await
        .expect("untag");
    let listed = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list");
    assert_eq!(listed.resource_tags().len(), 0);
}

#[tokio::test]
async fn test_get_unknown_view() {
    let client = make_client().await;
    let err = client
        .get_billing_view()
        .arn("arn:aws:billing::123456789012:billingview/ghost")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = BillingService::new();
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
