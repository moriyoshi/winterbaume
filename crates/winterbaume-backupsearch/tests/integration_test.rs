use std::sync::{Arc, Mutex};

use aws_sdk_backupsearch::config::BehaviorVersion;
use aws_sdk_backupsearch::types::{
    BackupCreationTimeFilter, ExportSpecification, ResourceType, S3ExportSpecification, SearchScope,
};
use winterbaume_backupsearch::BackupSearchService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_backupsearch::Client {
    let mock = MockAws::builder()
        .with_service(BackupSearchService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_backupsearch::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_backupsearch::Client::new(&config)
}

fn search_scope() -> SearchScope {
    SearchScope::builder()
        .backup_resource_types(ResourceType::Ebs)
        .backup_resource_creation_time(
            BackupCreationTimeFilter::builder()
                .created_after(aws_smithy_types::DateTime::from_secs(0))
                .build(),
        )
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_search_job_lifecycle() {
    let client = make_client().await;
    let start = client
        .start_search_job()
        .name("my-search")
        .search_scope(search_scope())
        .send()
        .await
        .expect("start");
    assert!(start.search_job_identifier().is_some());
    let id = start.search_job_identifier().unwrap().to_string();

    let got = client
        .get_search_job()
        .search_job_identifier(&id)
        .send()
        .await
        .expect("get");
    assert_eq!(got.name(), Some("my-search"));
    assert_eq!(
        got.status(),
        &aws_sdk_backupsearch::types::SearchJobState::Running
    );

    let list = client.list_search_jobs().send().await.expect("list");
    assert_eq!(list.search_jobs().len(), 1);

    client
        .stop_search_job()
        .search_job_identifier(&id)
        .send()
        .await
        .expect("stop");

    let got2 = client
        .get_search_job()
        .search_job_identifier(&id)
        .send()
        .await
        .expect("get2");
    assert_eq!(
        got2.status(),
        &aws_sdk_backupsearch::types::SearchJobState::Stopped
    );
}

#[tokio::test]
async fn test_get_search_job_not_found() {
    let client = make_client().await;
    let err = client
        .get_search_job()
        .search_job_identifier("missing")
        .send()
        .await
        .expect_err("should fail");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_stop_search_job_twice_conflict() {
    let client = make_client().await;
    let start = client
        .start_search_job()
        .name("conflict-test")
        .search_scope(search_scope())
        .send()
        .await
        .expect("start");
    let id = start.search_job_identifier().unwrap().to_string();

    client
        .stop_search_job()
        .search_job_identifier(&id)
        .send()
        .await
        .expect("stop ok");
    let err = client
        .stop_search_job()
        .search_job_identifier(&id)
        .send()
        .await
        .expect_err("second stop conflicts");
    assert!(format!("{err:?}").contains("ConflictException"));
}

#[tokio::test]
async fn test_list_search_job_backups_and_results_empty() {
    let client = make_client().await;
    let start = client
        .start_search_job()
        .name("ns")
        .search_scope(search_scope())
        .send()
        .await
        .expect("start");
    let id = start.search_job_identifier().unwrap().to_string();

    let backups = client
        .list_search_job_backups()
        .search_job_identifier(&id)
        .send()
        .await
        .expect("list backups");
    assert!(backups.results().is_empty());

    let results = client
        .list_search_job_results()
        .search_job_identifier(&id)
        .send()
        .await
        .expect("list results");
    assert!(results.results().is_empty());
}

#[tokio::test]
async fn test_export_job_lifecycle() {
    let client = make_client().await;
    let start = client
        .start_search_job()
        .name("for-export")
        .search_scope(search_scope())
        .send()
        .await
        .expect("start search");
    let search_id = start.search_job_identifier().unwrap().to_string();

    let export = client
        .start_search_result_export_job()
        .search_job_identifier(&search_id)
        .role_arn("arn:aws:iam::123456789012:role/export-role")
        .export_specification(ExportSpecification::S3ExportSpecification(
            S3ExportSpecification::builder()
                .destination_bucket("my-export-bucket")
                .destination_prefix("exports/")
                .build()
                .unwrap(),
        ))
        .send()
        .await
        .expect("start export");
    let export_id = export.export_job_identifier().to_string();

    let got = client
        .get_search_result_export_job()
        .export_job_identifier(&export_id)
        .send()
        .await
        .expect("get export");
    assert_eq!(got.export_job_identifier(), export_id.as_str());

    let list = client
        .list_search_result_export_jobs()
        .send()
        .await
        .expect("list export");
    assert_eq!(list.export_jobs().len(), 1);
}

#[tokio::test]
async fn test_export_job_requires_existing_search_job() {
    let client = make_client().await;
    let err = client
        .start_search_result_export_job()
        .search_job_identifier("nonexistent")
        .export_specification(ExportSpecification::S3ExportSpecification(
            S3ExportSpecification::builder()
                .destination_bucket("b")
                .destination_prefix("p")
                .build()
                .unwrap(),
        ))
        .send()
        .await
        .expect_err("should fail");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_tag_lifecycle_for_search_job() {
    let client = make_client().await;
    let start = client
        .start_search_job()
        .name("tagged")
        .search_scope(search_scope())
        .send()
        .await
        .expect("start");
    let arn = start.search_job_arn().unwrap().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags("Env", Some("prod".to_string()))
        .send()
        .await
        .expect("tag");

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags");
    assert_eq!(
        tags.tags().unwrap().get("Env"),
        Some(&Some("prod".to_string()))
    );

    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("Env")
        .send()
        .await
        .expect("untag");
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = BackupSearchService::new();
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
    use winterbaume_backupsearch::views::{BackupSearchStateView, SearchJobView};
    let svc = BackupSearchService::new();
    let mut view = BackupSearchStateView::default();
    view.search_jobs.insert(
        "id-1".to_string(),
        SearchJobView {
            identifier: "id-1".to_string(),
            arn: "arn:aws:backup-search:us-east-1:123456789012:search-job/id-1".to_string(),
            status: "RUNNING".to_string(),
            ..Default::default()
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .expect("restore");
    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snap.search_jobs.len(), 1);
}
