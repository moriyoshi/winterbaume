use std::sync::{Arc, Mutex};

use aws_sdk_bcmdashboards::config::BehaviorVersion;
use aws_sdk_bcmdashboards::types::ResourceTag;
use winterbaume_bcmdashboards::BcmDashboardsService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_bcmdashboards::Client {
    let mock = MockAws::builder()
        .with_service(BcmDashboardsService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_bcmdashboards::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_bcmdashboards::Client::new(&config)
}

#[tokio::test]
async fn test_dashboard_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_dashboard()
        .name("MyDashboard")
        .description("first")
        .send()
        .await
        .expect("create");
    let arn = create.arn().to_string();
    assert!(arn.contains("dashboard/"));

    let got = client.get_dashboard().arn(&arn).send().await.expect("get");
    assert_eq!(got.name(), "MyDashboard");
    assert_eq!(got.description(), Some("first"));

    client
        .update_dashboard()
        .arn(&arn)
        .description("second")
        .send()
        .await
        .expect("update");

    let got2 = client.get_dashboard().arn(&arn).send().await.expect("get2");
    assert_eq!(got2.description(), Some("second"));

    let list = client.list_dashboards().send().await.expect("list");
    assert_eq!(list.dashboards().len(), 1);

    client
        .delete_dashboard()
        .arn(&arn)
        .send()
        .await
        .expect("delete");

    let err = client
        .get_dashboard()
        .arn(&arn)
        .send()
        .await
        .expect_err("gone");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_duplicate_dashboard_name_conflicts() {
    let client = make_client().await;
    client
        .create_dashboard()
        .name("Dup")
        .send()
        .await
        .expect("first");
    let err = client
        .create_dashboard()
        .name("Dup")
        .send()
        .await
        .expect_err("dup");
    assert!(format!("{err:?}").contains("ConflictException"));
}

#[tokio::test]
async fn test_get_resource_policy_returns_static_policy() {
    let client = make_client().await;
    let create = client
        .create_dashboard()
        .name("WithPolicy")
        .send()
        .await
        .expect("create");
    let arn = create.arn().to_string();
    let resp = client
        .get_resource_policy()
        .resource_arn(&arn)
        .send()
        .await
        .expect("get policy");
    assert!(resp.policy_document().contains("Version"));
}

#[tokio::test]
async fn test_tag_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_dashboard()
        .name("Tagged")
        .send()
        .await
        .expect("create");
    let arn = create.arn().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .resource_tags(
            ResourceTag::builder()
                .key("Env")
                .value("prod")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag");

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags");
    let resource_tags = tags.resource_tags();
    assert_eq!(resource_tags.len(), 1);
    assert_eq!(resource_tags[0].key(), "Env");
    assert_eq!(resource_tags[0].value(), "prod");

    client
        .untag_resource()
        .resource_arn(&arn)
        .resource_tag_keys("Env")
        .send()
        .await
        .expect("untag");

    let tags2 = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list2");
    assert!(tags2.resource_tags().is_empty());
}

#[tokio::test]
async fn test_get_dashboard_not_found() {
    let client = make_client().await;
    let err = client
        .get_dashboard()
        .arn("arn:aws:bcm-dashboards:us-east-1:123:dashboard/nope")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = BcmDashboardsService::new();
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
    use winterbaume_bcmdashboards::views::{BcmDashboardsStateView, DashboardView};
    let svc = BcmDashboardsService::new();
    let mut view = BcmDashboardsStateView::default();
    view.dashboards.insert(
        "arn:aws:bcm-dashboards:us-east-1:123:dashboard/seed".to_string(),
        DashboardView {
            arn: "arn:aws:bcm-dashboards:us-east-1:123:dashboard/seed".to_string(),
            name: "seed".to_string(),
            r#type: "CUSTOM".to_string(),
            ..Default::default()
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .expect("restore");
    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snap.dashboards.len(), 1);
}
