use std::sync::{Arc, Mutex};

use aws_sdk_costandusagereport::config::BehaviorVersion;
use aws_sdk_costandusagereport::types::{
    AdditionalArtifact, CompressionFormat, ReportFormat, ReportVersioning, SchemaElement, Tag,
    TimeUnit,
};
use winterbaume_core::{MockAws, StatefulService};
use winterbaume_costandusagereport::CostAndUsageReportService;

async fn make_client() -> aws_sdk_costandusagereport::Client {
    let mock = MockAws::builder()
        .with_service(CostAndUsageReportService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_costandusagereport::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_costandusagereport::Client::new(&config)
}

fn sample_definition() -> aws_sdk_costandusagereport::types::ReportDefinition {
    aws_sdk_costandusagereport::types::ReportDefinition::builder()
        .report_name("daily-cur")
        .time_unit(TimeUnit::Daily)
        .format(ReportFormat::Csv)
        .compression(CompressionFormat::Gzip)
        .additional_schema_elements(SchemaElement::Resources)
        .s3_bucket("my-bucket")
        .s3_prefix("reports/")
        .s3_region("us-east-1".into())
        .additional_artifacts(AdditionalArtifact::Athena)
        .refresh_closed_reports(true)
        .report_versioning(ReportVersioning::OverwriteReport)
        .build()
        .expect("definition")
}

#[tokio::test]
async fn test_put_then_describe() {
    let client = make_client().await;
    client
        .put_report_definition()
        .report_definition(sample_definition())
        .send()
        .await
        .expect("put");
    let resp = client
        .describe_report_definitions()
        .send()
        .await
        .expect("describe");
    let defs = resp.report_definitions();
    assert_eq!(defs.len(), 1);
    assert_eq!(defs[0].report_name(), "daily-cur");
    assert_eq!(defs[0].s3_bucket(), "my-bucket");
}

#[tokio::test]
async fn test_put_duplicate_fails() {
    let client = make_client().await;
    client
        .put_report_definition()
        .report_definition(sample_definition())
        .send()
        .await
        .expect("put");
    let err = client
        .put_report_definition()
        .report_definition(sample_definition())
        .send()
        .await
        .expect_err("dup");
    assert!(format!("{err:?}").contains("DuplicateReportNameException"));
}

#[tokio::test]
async fn test_modify_unknown_fails() {
    let client = make_client().await;
    let err = client
        .modify_report_definition()
        .report_name("ghost")
        .report_definition(sample_definition())
        .send()
        .await
        .expect_err("modify");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_delete_round_trip() {
    let client = make_client().await;
    client
        .put_report_definition()
        .report_definition(sample_definition())
        .send()
        .await
        .expect("put");
    let resp = client
        .delete_report_definition()
        .report_name("daily-cur")
        .send()
        .await
        .expect("delete");
    assert!(resp.response_message().is_some());
    let resp = client
        .describe_report_definitions()
        .send()
        .await
        .expect("describe");
    assert_eq!(resp.report_definitions().len(), 0);
}

#[tokio::test]
async fn test_tag_lifecycle() {
    let client = make_client().await;
    client
        .put_report_definition()
        .report_definition(sample_definition())
        .send()
        .await
        .expect("put");
    client
        .tag_resource()
        .report_name("daily-cur")
        .tags(
            Tag::builder()
                .key("env")
                .value("prod")
                .build()
                .expect("tag"),
        )
        .send()
        .await
        .expect("tag");
    let listed = client
        .list_tags_for_resource()
        .report_name("daily-cur")
        .send()
        .await
        .expect("list-tags");
    let tags = listed.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), "env");
    assert_eq!(tags[0].value(), "prod");

    client
        .untag_resource()
        .report_name("daily-cur")
        .tag_keys("env")
        .send()
        .await
        .expect("untag");
    let listed = client
        .list_tags_for_resource()
        .report_name("daily-cur")
        .send()
        .await
        .expect("list-tags");
    assert!(listed.tags().is_empty());
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = CostAndUsageReportService::new();
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
