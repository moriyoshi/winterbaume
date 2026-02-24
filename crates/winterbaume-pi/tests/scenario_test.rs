/// Scenario: Performance analysis report lifecycle
///
/// Creates a PI analysis report, verifies it appears in list and get, tags it,
/// reads back tags, removes the tags, then deletes the report. Exercises 7
/// operations end-to-end against the mock.
use aws_sdk_pi::config::BehaviorVersion;
use aws_sdk_pi::types::{ServiceType, Tag};
use winterbaume_core::MockAws;
use winterbaume_pi::PiService;

async fn make_client() -> aws_sdk_pi::Client {
    let mock = MockAws::builder().with_service(PiService::new()).build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_pi::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_pi::Client::new(&config)
}

#[tokio::test]
async fn test_analysis_report_pipeline() {
    let client = make_client().await;

    // Create report
    let create = client
        .create_performance_analysis_report()
        .service_type(ServiceType::Rds)
        .identifier("db-pipeline-01")
        .start_time(aws_smithy_types::DateTime::from_secs(1_700_000_000))
        .end_time(aws_smithy_types::DateTime::from_secs(1_700_003_600))
        .tags(Tag::builder().key("Project").value("test").build().unwrap())
        .send()
        .await
        .expect("create report");
    let report_id = create.analysis_report_id().expect("id").to_string();
    assert!(report_id.starts_with("report-"));

    // Get report — verify it reflects the submitted data
    let got = client
        .get_performance_analysis_report()
        .service_type(ServiceType::Rds)
        .identifier("db-pipeline-01")
        .analysis_report_id(&report_id)
        .send()
        .await
        .expect("get report");
    let report = got.analysis_report().expect("report");
    assert_eq!(report.analysis_report_id(), report_id);
    assert_eq!(report.identifier(), Some("db-pipeline-01"));
    assert!(format!("{:?}", report.status()).contains("Succeeded"));

    // List — report appears
    let list = client
        .list_performance_analysis_reports()
        .service_type(ServiceType::Rds)
        .identifier("db-pipeline-01")
        .send()
        .await
        .expect("list reports");
    assert_eq!(list.analysis_reports().len(), 1);

    // Tag the report ARN (mock treats ARN as opaque key)
    let arn = format!("arn:aws:pi:us-east-1:123:metrics/RDS/db-pipeline-01/{report_id}");
    client
        .tag_resource()
        .service_type(ServiceType::Rds)
        .resource_arn(&arn)
        .tags(Tag::builder().key("Stage").value("review").build().unwrap())
        .send()
        .await
        .expect("tag resource");

    let tags_resp = client
        .list_tags_for_resource()
        .service_type(ServiceType::Rds)
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags");
    assert_eq!(tags_resp.tags().len(), 1);
    assert_eq!(tags_resp.tags()[0].key(), "Stage");

    // Untag
    client
        .untag_resource()
        .service_type(ServiceType::Rds)
        .resource_arn(&arn)
        .tag_keys("Stage")
        .send()
        .await
        .expect("untag resource");

    // Delete
    client
        .delete_performance_analysis_report()
        .service_type(ServiceType::Rds)
        .identifier("db-pipeline-01")
        .analysis_report_id(&report_id)
        .send()
        .await
        .expect("delete report");

    // Verify gone
    let err = client
        .get_performance_analysis_report()
        .service_type(ServiceType::Rds)
        .identifier("db-pipeline-01")
        .analysis_report_id(&report_id)
        .send()
        .await
        .expect_err("report should be deleted");
    assert!(format!("{err:?}").contains("NotAuthorizedException"));
}
