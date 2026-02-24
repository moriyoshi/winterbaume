use std::sync::{Arc, Mutex};

use aws_sdk_pi::config::BehaviorVersion;
use aws_sdk_pi::types::{ServiceType, Tag};
use winterbaume_core::{MockAws, StatefulService};
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
async fn test_report_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_performance_analysis_report()
        .service_type(ServiceType::Rds)
        .identifier("db-ABC")
        .start_time(aws_smithy_types::DateTime::from_secs(1_700_000_000))
        .end_time(aws_smithy_types::DateTime::from_secs(1_700_003_600))
        .send()
        .await
        .expect("create");
    let id = create.analysis_report_id().expect("id").to_string();
    assert!(id.starts_with("report-"));

    let got = client
        .get_performance_analysis_report()
        .service_type(ServiceType::Rds)
        .identifier("db-ABC")
        .analysis_report_id(&id)
        .send()
        .await
        .expect("get");
    let report = got.analysis_report().expect("report");
    assert_eq!(report.analysis_report_id(), id);
    assert_eq!(report.identifier(), Some("db-ABC"));

    let list = client
        .list_performance_analysis_reports()
        .service_type(ServiceType::Rds)
        .identifier("db-ABC")
        .send()
        .await
        .expect("list");
    assert_eq!(list.analysis_reports().len(), 1);

    client
        .delete_performance_analysis_report()
        .service_type(ServiceType::Rds)
        .identifier("db-ABC")
        .analysis_report_id(&id)
        .send()
        .await
        .expect("delete");
}

#[tokio::test]
async fn test_get_report_not_found() {
    let client = make_client().await;
    let err = client
        .get_performance_analysis_report()
        .service_type(ServiceType::Rds)
        .identifier("db-ABC")
        .analysis_report_id("missing")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("NotAuthorizedException"));
}

#[tokio::test]
async fn test_describe_dimension_keys_empty() {
    let client = make_client().await;
    let resp = client
        .describe_dimension_keys()
        .service_type(ServiceType::Rds)
        .identifier("db-ABC")
        .start_time(aws_smithy_types::DateTime::from_secs(1_700_000_000))
        .end_time(aws_smithy_types::DateTime::from_secs(1_700_003_600))
        .metric("db.load.avg")
        .group_by(
            aws_sdk_pi::types::DimensionGroup::builder()
                .group("db.sql")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("describe");
    assert_eq!(resp.keys().len(), 0);
}

#[tokio::test]
async fn test_tag_lifecycle() {
    let client = make_client().await;
    let arn = "arn:aws:pi:us-east-1:123:metrics/RDS/db-ABC";
    client
        .tag_resource()
        .service_type(ServiceType::Rds)
        .resource_arn(arn)
        .tags(Tag::builder().key("Env").value("prod").build().unwrap())
        .send()
        .await
        .expect("tag");

    let resp = client
        .list_tags_for_resource()
        .service_type(ServiceType::Rds)
        .resource_arn(arn)
        .send()
        .await
        .expect("list");
    assert_eq!(resp.tags().len(), 1);

    client
        .untag_resource()
        .service_type(ServiceType::Rds)
        .resource_arn(arn)
        .tag_keys("Env")
        .send()
        .await
        .expect("untag");
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = PiService::new();
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
