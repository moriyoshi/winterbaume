use std::sync::{Arc, Mutex};

use aws_sdk_codegurusecurity::config::BehaviorVersion;
use aws_sdk_codegurusecurity::types::{AnalysisType, EncryptionConfig, ResourceId, ScanType};
use winterbaume_codegurusecurity::CodeGuruSecurityService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_codegurusecurity::Client {
    let mock = MockAws::builder()
        .with_service(CodeGuruSecurityService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_codegurusecurity::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_codegurusecurity::Client::new(&config)
}

async fn create_scan(client: &aws_sdk_codegurusecurity::Client, name: &str) -> String {
    let resource = ResourceId::CodeArtifactId("artifact-123".to_string());
    let resp = client
        .create_scan()
        .scan_name(name)
        .resource_id(resource)
        .scan_type(ScanType::Standard)
        .analysis_type(AnalysisType::Security)
        .send()
        .await
        .expect("create");
    resp.scan_name_arn().expect("arn").to_string()
}

#[tokio::test]
async fn test_create_get_scan() {
    let client = make_client().await;
    let arn = create_scan(&client, "my-scan").await;
    let got = client
        .get_scan()
        .scan_name("my-scan")
        .send()
        .await
        .expect("get");
    assert_eq!(got.scan_name(), "my-scan");
    assert_eq!(got.scan_name_arn(), Some(arn.as_str()));
    assert_eq!(got.scan_state().as_str(), "Successful");
}

#[tokio::test]
async fn test_get_unknown_scan() {
    let client = make_client().await;
    let err = client
        .get_scan()
        .scan_name("ghost")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_list_scans() {
    let client = make_client().await;
    create_scan(&client, "scan-a").await;
    create_scan(&client, "scan-b").await;
    let resp = client.list_scans().send().await.expect("list");
    assert_eq!(resp.summaries().len(), 2);
}

#[tokio::test]
async fn test_create_upload_url() {
    let client = make_client().await;
    let resp = client
        .create_upload_url()
        .scan_name("my-scan")
        .send()
        .await
        .expect("upload");
    assert!(!resp.code_artifact_id().is_empty());
    assert!(!resp.s3_url().is_empty());
}

#[tokio::test]
async fn test_account_configuration_round_trip() {
    let client = make_client().await;
    let cfg = EncryptionConfig::builder()
        .kms_key_arn("arn:aws:kms:us-east-1:123456789012:key/abc")
        .build();
    client
        .update_account_configuration()
        .encryption_config(cfg)
        .send()
        .await
        .expect("update");
    let got = client
        .get_account_configuration()
        .send()
        .await
        .expect("get");
    assert_eq!(
        got.encryption_config().and_then(|c| c.kms_key_arn()),
        Some("arn:aws:kms:us-east-1:123456789012:key/abc")
    );
}

#[tokio::test]
async fn test_metrics_summary_and_findings_metrics() {
    let client = make_client().await;
    let summary = client
        .get_metrics_summary()
        .date(aws_sdk_codegurusecurity::primitives::DateTime::from_secs(0))
        .send()
        .await
        .expect("summary");
    assert!(summary.metrics_summary().is_some());
    let metrics = client
        .list_findings_metrics()
        .start_date(aws_sdk_codegurusecurity::primitives::DateTime::from_secs(0))
        .end_date(aws_sdk_codegurusecurity::primitives::DateTime::from_secs(
            1_000_000,
        ))
        .send()
        .await
        .expect("metrics");
    assert_eq!(metrics.findings_metrics().len(), 0);
}

#[tokio::test]
async fn test_tag_lifecycle() {
    let client = make_client().await;
    let arn = create_scan(&client, "tagged-scan").await;
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "prod")
        .send()
        .await
        .expect("tag");
    let listed = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list");
    assert_eq!(
        listed.tags().and_then(|t| t.get("env")),
        Some(&"prod".to_string())
    );

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag");
    let listed = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list");
    assert!(listed.tags().map(|t| t.is_empty()).unwrap_or(true));
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = CodeGuruSecurityService::new();
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
