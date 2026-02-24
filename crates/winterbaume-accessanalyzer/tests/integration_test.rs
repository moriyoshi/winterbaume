use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use aws_sdk_accessanalyzer::config::BehaviorVersion;
use winterbaume_accessanalyzer::{AccessAnalyzerService, AccessAnalyzerStateView};
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> (aws_sdk_accessanalyzer::Client, AccessAnalyzerService) {
    let svc = AccessAnalyzerService::new();
    let mock = MockAws::builder()
        .with_service(AccessAnalyzerService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_accessanalyzer::config::Region::new("us-east-1"))
        .load()
        .await;
    (aws_sdk_accessanalyzer::Client::new(&config), svc)
}

// -------------------------------------------------------------------------
// Analyzer CRUD tests
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_get_analyzer() {
    let (client, _svc) = make_client().await;

    let create_resp = client
        .create_analyzer()
        .analyzer_name("test-analyzer")
        .r#type(aws_sdk_accessanalyzer::types::Type::Account)
        .send()
        .await
        .expect("create_analyzer should succeed");

    let arn = create_resp.arn().expect("arn should be present");
    assert!(arn.contains("test-analyzer"));

    // Get the analyzer
    let get_resp = client
        .get_analyzer()
        .analyzer_name("test-analyzer")
        .send()
        .await
        .expect("get_analyzer should succeed");

    let analyzer = get_resp.analyzer().expect("analyzer should be present");
    assert_eq!(analyzer.name(), "test-analyzer");
    assert_eq!(analyzer.arn(), arn);
    assert_eq!(
        analyzer.r#type(),
        &aws_sdk_accessanalyzer::types::Type::Account
    );
    assert_eq!(
        analyzer.status(),
        &aws_sdk_accessanalyzer::types::AnalyzerStatus::Active
    );
}

#[tokio::test]
async fn test_list_analyzers() {
    let (client, _svc) = make_client().await;

    client
        .create_analyzer()
        .analyzer_name("analyzer-1")
        .r#type(aws_sdk_accessanalyzer::types::Type::Account)
        .send()
        .await
        .expect("create_analyzer should succeed");

    client
        .create_analyzer()
        .analyzer_name("analyzer-2")
        .r#type(aws_sdk_accessanalyzer::types::Type::Account)
        .send()
        .await
        .expect("create_analyzer should succeed");

    let list_resp = client
        .list_analyzers()
        .send()
        .await
        .expect("list_analyzers should succeed");

    let analyzers = list_resp.analyzers();
    assert_eq!(analyzers.len(), 2);
}

#[tokio::test]
async fn test_delete_analyzer() {
    let (client, _svc) = make_client().await;

    client
        .create_analyzer()
        .analyzer_name("to-delete")
        .r#type(aws_sdk_accessanalyzer::types::Type::Account)
        .send()
        .await
        .expect("create_analyzer should succeed");

    client
        .delete_analyzer()
        .analyzer_name("to-delete")
        .send()
        .await
        .expect("delete_analyzer should succeed");

    // Verify it's gone
    let err = client
        .get_analyzer()
        .analyzer_name("to-delete")
        .send()
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_analyzer() {
    let (client, _svc) = make_client().await;

    let err = client
        .get_analyzer()
        .analyzer_name("does-not-exist")
        .send()
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_delete_nonexistent_analyzer() {
    let (client, _svc) = make_client().await;

    let err = client
        .delete_analyzer()
        .analyzer_name("does-not-exist")
        .send()
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_create_duplicate_analyzer() {
    let (client, _svc) = make_client().await;

    client
        .create_analyzer()
        .analyzer_name("dup-analyzer")
        .r#type(aws_sdk_accessanalyzer::types::Type::Account)
        .send()
        .await
        .expect("first create should succeed");

    let err = client
        .create_analyzer()
        .analyzer_name("dup-analyzer")
        .r#type(aws_sdk_accessanalyzer::types::Type::Account)
        .send()
        .await;
    assert!(err.is_err());
}

// -------------------------------------------------------------------------
// Lifecycle test
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_analyzer_lifecycle() {
    let (client, _svc) = make_client().await;

    // Create
    let create_resp = client
        .create_analyzer()
        .analyzer_name("lifecycle-test")
        .r#type(aws_sdk_accessanalyzer::types::Type::Account)
        .send()
        .await
        .expect("create should succeed");

    let arn = create_resp.arn().unwrap().to_string();

    // Describe
    let get_resp = client
        .get_analyzer()
        .analyzer_name("lifecycle-test")
        .send()
        .await
        .expect("get should succeed");
    let analyzer = get_resp.analyzer().expect("analyzer present");
    assert_eq!(analyzer.arn(), arn.as_str());

    // Delete
    client
        .delete_analyzer()
        .analyzer_name("lifecycle-test")
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let list_resp = client
        .list_analyzers()
        .send()
        .await
        .expect("list should succeed");
    assert!(list_resp.analyzers().is_empty());
}

// -------------------------------------------------------------------------
// Archive Rule tests
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_create_and_get_archive_rule() {
    let (client, _svc) = make_client().await;

    client
        .create_analyzer()
        .analyzer_name("rule-test-analyzer")
        .r#type(aws_sdk_accessanalyzer::types::Type::Account)
        .send()
        .await
        .expect("create_analyzer should succeed");

    let criterion = aws_sdk_accessanalyzer::types::Criterion::builder()
        .eq("s3")
        .build();

    client
        .create_archive_rule()
        .analyzer_name("rule-test-analyzer")
        .rule_name("test-rule")
        .filter("resourceType", criterion)
        .send()
        .await
        .expect("create_archive_rule should succeed");

    let get_resp = client
        .get_archive_rule()
        .analyzer_name("rule-test-analyzer")
        .rule_name("test-rule")
        .send()
        .await
        .expect("get_archive_rule should succeed");

    let rule = get_resp.archive_rule().expect("archive_rule present");
    assert_eq!(rule.rule_name(), "test-rule");
    assert!(rule.filter().contains_key("resourceType"));
}

#[tokio::test]
async fn test_list_archive_rules() {
    let (client, _svc) = make_client().await;

    client
        .create_analyzer()
        .analyzer_name("list-rules-analyzer")
        .r#type(aws_sdk_accessanalyzer::types::Type::Account)
        .send()
        .await
        .unwrap();

    let criterion = aws_sdk_accessanalyzer::types::Criterion::builder()
        .eq("s3")
        .build();

    client
        .create_archive_rule()
        .analyzer_name("list-rules-analyzer")
        .rule_name("rule-1")
        .filter("resourceType", criterion.clone())
        .send()
        .await
        .unwrap();

    client
        .create_archive_rule()
        .analyzer_name("list-rules-analyzer")
        .rule_name("rule-2")
        .filter("resourceType", criterion)
        .send()
        .await
        .unwrap();

    let list_resp = client
        .list_archive_rules()
        .analyzer_name("list-rules-analyzer")
        .send()
        .await
        .expect("list_archive_rules should succeed");

    assert_eq!(list_resp.archive_rules().len(), 2);
}

#[tokio::test]
async fn test_delete_archive_rule() {
    let (client, _svc) = make_client().await;

    client
        .create_analyzer()
        .analyzer_name("delete-rule-analyzer")
        .r#type(aws_sdk_accessanalyzer::types::Type::Account)
        .send()
        .await
        .unwrap();

    let criterion = aws_sdk_accessanalyzer::types::Criterion::builder()
        .eq("s3")
        .build();

    client
        .create_archive_rule()
        .analyzer_name("delete-rule-analyzer")
        .rule_name("to-delete")
        .filter("resourceType", criterion)
        .send()
        .await
        .unwrap();

    client
        .delete_archive_rule()
        .analyzer_name("delete-rule-analyzer")
        .rule_name("to-delete")
        .send()
        .await
        .expect("delete_archive_rule should succeed");

    // Verify it's gone
    let err = client
        .get_archive_rule()
        .analyzer_name("delete-rule-analyzer")
        .rule_name("to-delete")
        .send()
        .await;
    assert!(err.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_archive_rule() {
    let (client, _svc) = make_client().await;

    client
        .create_analyzer()
        .analyzer_name("nonexistent-rule-analyzer")
        .r#type(aws_sdk_accessanalyzer::types::Type::Account)
        .send()
        .await
        .unwrap();

    let err = client
        .get_archive_rule()
        .analyzer_name("nonexistent-rule-analyzer")
        .rule_name("no-such-rule")
        .send()
        .await;
    assert!(err.is_err());
}

// -------------------------------------------------------------------------
// Tag tests
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_tag_resource() {
    let (client, _svc) = make_client().await;

    let create_resp = client
        .create_analyzer()
        .analyzer_name("tag-test")
        .r#type(aws_sdk_accessanalyzer::types::Type::Account)
        .send()
        .await
        .unwrap();

    let arn = create_resp.arn().unwrap().to_string();

    // Add tags
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("env", "test")
        .tags("team", "security")
        .send()
        .await
        .expect("tag_resource should succeed");

    // List tags
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.tags().expect("tags should be present");
    assert_eq!(tags.get("env").map(String::as_str), Some("test"));
    assert_eq!(tags.get("team").map(String::as_str), Some("security"));
}

#[tokio::test]
async fn test_untag_resource() {
    let (client, _svc) = make_client().await;

    let create_resp = client
        .create_analyzer()
        .analyzer_name("untag-test")
        .r#type(aws_sdk_accessanalyzer::types::Type::Account)
        .tags("env", "test")
        .tags("team", "security")
        .send()
        .await
        .unwrap();

    let arn = create_resp.arn().unwrap().to_string();

    // Remove one tag
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("env")
        .send()
        .await
        .expect("untag_resource should succeed");

    // Verify
    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let tags = list_resp.tags().expect("tags should be present");
    assert!(tags.get("env").is_none());
    assert_eq!(tags.get("team").map(String::as_str), Some("security"));
}

// -------------------------------------------------------------------------
// State view tests
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_snapshot_and_restore() {
    let svc = AccessAnalyzerService::new();

    // Pre-seed state via restore
    let mut view = AccessAnalyzerStateView::default();
    view.analyzers.insert(
        "snap-test".to_string(),
        winterbaume_accessanalyzer::views::AnalyzerView {
            arn: "arn:aws:access-analyzer:us-east-1:123456789012:analyzer/snap-test".to_string(),
            name: "snap-test".to_string(),
            analyzer_type: "ACCOUNT".to_string(),
            status: "ACTIVE".to_string(),
            created_at: "2024-01-01T00:00:00.000Z".to_string(),
            tags: HashMap::new(),
            archive_rules: HashMap::new(),
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    // Snapshot
    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert!(snap.analyzers.contains_key("snap-test"));

    // Restore to a different scope
    svc.restore("123456789012", "us-west-2", snap.clone())
        .await
        .unwrap();
    let snap2 = svc.snapshot("123456789012", "us-west-2").await;
    assert!(snap2.analyzers.contains_key("snap-test"));
}

#[tokio::test]
async fn test_merge_is_additive() {
    let svc = AccessAnalyzerService::new();

    // Pre-seed analyzer A
    let mut view_a = AccessAnalyzerStateView::default();
    view_a.analyzers.insert(
        "analyzer-a".to_string(),
        winterbaume_accessanalyzer::views::AnalyzerView {
            arn: "arn:aws:access-analyzer:us-east-1:123456789012:analyzer/analyzer-a".to_string(),
            name: "analyzer-a".to_string(),
            analyzer_type: "ACCOUNT".to_string(),
            status: "ACTIVE".to_string(),
            created_at: "2024-01-01T00:00:00.000Z".to_string(),
            tags: HashMap::new(),
            archive_rules: HashMap::new(),
        },
    );
    svc.restore("123456789012", "us-east-1", view_a)
        .await
        .unwrap();

    // Merge in analyzer B without removing A
    let mut view_b = AccessAnalyzerStateView::default();
    view_b.analyzers.insert(
        "analyzer-b".to_string(),
        winterbaume_accessanalyzer::views::AnalyzerView {
            arn: "arn:aws:access-analyzer:us-east-1:123456789012:analyzer/analyzer-b".to_string(),
            name: "analyzer-b".to_string(),
            analyzer_type: "ACCOUNT".to_string(),
            status: "ACTIVE".to_string(),
            created_at: "2024-01-01T00:00:00.000Z".to_string(),
            tags: HashMap::new(),
            archive_rules: HashMap::new(),
        },
    );

    svc.merge("123456789012", "us-east-1", view_b)
        .await
        .unwrap();

    let final_view = svc.snapshot("123456789012", "us-east-1").await;
    assert!(final_view.analyzers.contains_key("analyzer-a"));
    assert!(final_view.analyzers.contains_key("analyzer-b"));
}

// -------------------------------------------------------------------------
// State change notification tests
// -------------------------------------------------------------------------

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = AccessAnalyzerService::new();
    let events: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));
    let events2 = Arc::clone(&events);
    svc.notifier().subscribe(move |account_id, region, _view| {
        events2
            .lock()
            .unwrap()
            .push((account_id.to_string(), region.to_string()));
    });

    svc.restore(
        "123456789012",
        "us-east-1",
        AccessAnalyzerStateView::default(),
    )
    .await
    .unwrap();

    let got = events.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[0],
        ("123456789012".to_string(), "us-east-1".to_string())
    );
}

#[tokio::test]
async fn test_state_change_listener_snapshot_reflects_mutation() {
    let svc = AccessAnalyzerService::new();

    // Pre-seed state via restore (ignore first event)
    let mut view = AccessAnalyzerStateView::default();
    view.analyzers.insert(
        "pre-existing".to_string(),
        winterbaume_accessanalyzer::views::AnalyzerView {
            arn: "arn:aws:access-analyzer:us-east-1:123456789012:analyzer/pre-existing".to_string(),
            name: "pre-existing".to_string(),
            analyzer_type: "ACCOUNT".to_string(),
            status: "ACTIVE".to_string(),
            created_at: "2024-01-01T00:00:00.000Z".to_string(),
            tags: HashMap::new(),
            archive_rules: HashMap::new(),
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .unwrap();

    // Re-subscribe and capture snapshot
    let snapshots: Arc<Mutex<Vec<AccessAnalyzerStateView>>> = Arc::new(Mutex::new(vec![]));
    let snapshots2 = Arc::clone(&snapshots);
    svc.notifier().subscribe(move |_account_id, _region, view| {
        snapshots2.lock().unwrap().push(view.clone());
    });

    // Merge a second analyzer
    let mut view2 = AccessAnalyzerStateView::default();
    view2.analyzers.insert(
        "merged-in".to_string(),
        winterbaume_accessanalyzer::views::AnalyzerView {
            arn: "arn:aws:access-analyzer:us-east-1:123456789012:analyzer/merged-in".to_string(),
            name: "merged-in".to_string(),
            analyzer_type: "ACCOUNT".to_string(),
            status: "ACTIVE".to_string(),
            created_at: "2024-01-01T00:00:00.000Z".to_string(),
            tags: HashMap::new(),
            archive_rules: HashMap::new(),
        },
    );
    svc.merge("123456789012", "us-east-1", view2).await.unwrap();

    let got = snapshots.lock().unwrap();
    assert_eq!(got.len(), 1);
    assert!(got[0].analyzers.contains_key("pre-existing"));
    assert!(got[0].analyzers.contains_key("merged-in"));
}
