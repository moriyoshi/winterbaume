use std::sync::{Arc, Mutex};

use aws_sdk_artifact::config::BehaviorVersion;
use winterbaume_artifact::ArtifactService;
use winterbaume_artifact::views::{ArtifactStateView, ReportView};
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_artifact::Client {
    let mock = MockAws::builder()
        .with_service(ArtifactService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_artifact::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_artifact::Client::new(&config)
}

#[tokio::test]
async fn test_get_account_settings_default() {
    let client = make_client().await;
    let resp = client
        .get_account_settings()
        .send()
        .await
        .expect("get_account_settings");
    let settings = resp.account_settings().expect("settings");
    assert_eq!(
        settings
            .notification_subscription_status()
            .map(|s| s.as_str()),
        Some("NOT_SUBSCRIBED")
    );
}

#[tokio::test]
async fn test_put_account_settings_round_trip() {
    let client = make_client().await;
    client
        .put_account_settings()
        .notification_subscription_status(
            aws_sdk_artifact::types::NotificationSubscriptionStatus::Subscribed,
        )
        .send()
        .await
        .expect("put");
    let resp = client.get_account_settings().send().await.expect("get");
    let settings = resp.account_settings().expect("settings");
    assert_eq!(
        settings
            .notification_subscription_status()
            .map(|s| s.as_str()),
        Some("SUBSCRIBED")
    );
}

#[tokio::test]
async fn test_list_reports_empty_by_default() {
    let client = make_client().await;
    let resp = client.list_reports().send().await.expect("list");
    assert!(resp.reports().is_empty());
}

#[tokio::test]
async fn test_list_customer_agreements_empty_by_default() {
    let client = make_client().await;
    let resp = client
        .list_customer_agreements()
        .send()
        .await
        .expect("list");
    assert_eq!(resp.customer_agreements().len(), 0);
}

#[tokio::test]
async fn test_get_report_metadata_not_found() {
    let client = make_client().await;
    let err = client
        .get_report_metadata()
        .report_id("missing-report")
        .send()
        .await
        .expect_err("should fail");
    assert!(format!("{:?}", err).contains("ResourceNotFoundException"));
}

// State-view round-trip tests exercise the typed view contract directly,
// since AWS Artifact has no public API for creating reports / agreements
// ( those are AWS-managed ). The terraform converter and the
// `merge`/`restore` paths are how state is seeded in real use.

#[tokio::test]
async fn test_state_view_round_trip_with_report() {
    let svc = ArtifactService::new();
    let mut view = ArtifactStateView::default();
    view.reports.insert(
        "soc2-2024".to_string(),
        ReportView {
            id: "soc2-2024".to_string(),
            version: 1,
            name: "SOC 2 Report 2024".to_string(),
            description: Some("Annual audit".to_string()),
            category: Some("CERTIFICATIONS_AND_ATTESTATIONS".to_string()),
            series: Some("SOC2".to_string()),
            state: "PUBLISHED".to_string(),
            arn: "arn:aws:artifact:::report/soc2-2024".to_string(),
            document_url: "https://example.com/soc2-2024.pdf".to_string(),
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .expect("restore");

    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert!(snap.reports.contains_key("soc2-2024"));
    assert_eq!(snap.reports["soc2-2024"].name, "SOC 2 Report 2024");
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = ArtifactService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore("123456789012", "us-east-1", ArtifactStateView::default())
        .await
        .expect("restore");
    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
}
