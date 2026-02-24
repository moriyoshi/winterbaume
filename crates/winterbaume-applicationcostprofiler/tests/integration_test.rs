use std::sync::{Arc, Mutex};

use aws_sdk_applicationcostprofiler::config::BehaviorVersion;
use aws_sdk_applicationcostprofiler::types::{
    Format, ReportFrequency, S3Location, SourceS3Location,
};
use winterbaume_applicationcostprofiler::ApplicationCostProfilerService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_applicationcostprofiler::Client {
    let mock = MockAws::builder()
        .with_service(ApplicationCostProfilerService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_applicationcostprofiler::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;
    aws_sdk_applicationcostprofiler::Client::new(&config)
}

fn s3() -> S3Location {
    S3Location::builder()
        .bucket("my-cost-bucket")
        .prefix("reports/")
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_report_definition_lifecycle() {
    let client = make_client().await;
    let resp = client
        .put_report_definition()
        .report_id("daily-report")
        .report_description("Daily cost report")
        .report_frequency(ReportFrequency::Daily)
        .format(Format::Csv)
        .destination_s3_location(s3())
        .send()
        .await
        .expect("put");
    assert_eq!(resp.report_id(), Some("daily-report"));

    let get = client
        .get_report_definition()
        .report_id("daily-report")
        .send()
        .await
        .expect("get");
    assert_eq!(get.report_id(), "daily-report");
    assert_eq!(get.report_description(), "Daily cost report");
    assert_eq!(get.report_frequency(), &ReportFrequency::Daily);
    assert_eq!(get.format(), &Format::Csv);

    client
        .update_report_definition()
        .report_id("daily-report")
        .report_description("Updated daily cost report")
        .report_frequency(ReportFrequency::Monthly)
        .format(Format::Parquet)
        .destination_s3_location(s3())
        .send()
        .await
        .expect("update");

    let get = client
        .get_report_definition()
        .report_id("daily-report")
        .send()
        .await
        .expect("get after update");
    assert_eq!(get.report_description(), "Updated daily cost report");
    assert_eq!(get.report_frequency(), &ReportFrequency::Monthly);
    assert_eq!(get.format(), &Format::Parquet);

    client
        .delete_report_definition()
        .report_id("daily-report")
        .send()
        .await
        .expect("delete");

    let err = client
        .get_report_definition()
        .report_id("daily-report")
        .send()
        .await
        .expect_err("get after delete should fail");
    assert!(format!("{:?}", err).contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_put_duplicate_fails() {
    let client = make_client().await;
    client
        .put_report_definition()
        .report_id("dup")
        .report_description("desc")
        .report_frequency(ReportFrequency::Daily)
        .format(Format::Csv)
        .destination_s3_location(s3())
        .send()
        .await
        .expect("first");
    let err = client
        .put_report_definition()
        .report_id("dup")
        .report_description("desc")
        .report_frequency(ReportFrequency::Daily)
        .format(Format::Csv)
        .destination_s3_location(s3())
        .send()
        .await
        .expect_err("dup");
    assert!(format!("{:?}", err).contains("ConflictException"));
}

#[tokio::test]
async fn test_list_report_definitions() {
    let client = make_client().await;
    for n in ["a", "b", "c"] {
        client
            .put_report_definition()
            .report_id(n)
            .report_description("desc")
            .report_frequency(ReportFrequency::Daily)
            .format(Format::Csv)
            .destination_s3_location(s3())
            .send()
            .await
            .expect("put");
    }
    let resp = client.list_report_definitions().send().await.expect("list");
    assert_eq!(resp.report_definitions().len(), 3);
}

#[tokio::test]
async fn test_get_missing_report_definition() {
    let client = make_client().await;
    let err = client
        .get_report_definition()
        .report_id("missing")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{:?}", err).contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_import_application_usage() {
    let client = make_client().await;
    let resp = client
        .import_application_usage()
        .source_s3_location(
            SourceS3Location::builder()
                .bucket("src-bucket")
                .key("usage.csv")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("import");
    assert!(!resp.import_id().is_empty());
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = ApplicationCostProfilerService::new();
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
