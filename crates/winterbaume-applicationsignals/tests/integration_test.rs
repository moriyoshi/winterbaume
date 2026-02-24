use std::sync::{Arc, Mutex};

use aws_sdk_applicationsignals::config::BehaviorVersion;
use aws_sdk_applicationsignals::types::{
    Goal, RequestBasedServiceLevelIndicatorConfig, RequestBasedServiceLevelIndicatorMetricConfig,
    Tag,
};
use winterbaume_applicationsignals::ApplicationSignalsService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_applicationsignals::Client {
    let mock = MockAws::builder()
        .with_service(ApplicationSignalsService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_applicationsignals::config::Region::new("us-east-1"))
        .load()
        .await;
    aws_sdk_applicationsignals::Client::new(&config)
}

fn sample_request_sli_config() -> RequestBasedServiceLevelIndicatorConfig {
    RequestBasedServiceLevelIndicatorConfig::builder()
        .request_based_sli_metric_config(
            RequestBasedServiceLevelIndicatorMetricConfig::builder().build(),
        )
        .build()
}

#[tokio::test]
async fn test_slo_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_service_level_objective()
        .name("MySlo")
        .request_based_sli_config(sample_request_sli_config())
        .goal(Goal::builder().attainment_goal(99.0).build())
        .send()
        .await
        .expect("create");
    let arn = create.slo().unwrap().arn().to_string();

    let got = client
        .get_service_level_objective()
        .id("MySlo")
        .send()
        .await
        .expect("get");
    assert_eq!(got.slo().unwrap().name(), "MySlo");

    client
        .update_service_level_objective()
        .id("MySlo")
        .description("Updated")
        .send()
        .await
        .expect("update");

    let got2 = client
        .get_service_level_objective()
        .id("MySlo")
        .send()
        .await
        .expect("get2");
    assert_eq!(got2.slo().unwrap().description(), Some("Updated"));

    let list = client
        .list_service_level_objectives()
        .send()
        .await
        .expect("list");
    assert_eq!(list.slo_summaries().len(), 1);

    client
        .delete_service_level_objective()
        .id("MySlo")
        .send()
        .await
        .expect("delete");

    assert!(arn.contains("slo/"));
}

#[tokio::test]
async fn test_duplicate_slo_conflicts() {
    let client = make_client().await;
    client
        .create_service_level_objective()
        .name("Dup")
        .request_based_sli_config(sample_request_sli_config())
        .send()
        .await
        .expect("first");
    let err = client
        .create_service_level_objective()
        .name("Dup")
        .request_based_sli_config(sample_request_sli_config())
        .send()
        .await
        .expect_err("dup");
    assert!(format!("{err:?}").contains("ConflictException"));
}

#[tokio::test]
async fn test_get_slo_not_found() {
    let client = make_client().await;
    let err = client
        .get_service_level_objective()
        .id("missing")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_batch_get_budget_report() {
    let client = make_client().await;
    let _ = client
        .create_service_level_objective()
        .name("BudgetSlo")
        .request_based_sli_config(sample_request_sli_config())
        .send()
        .await
        .expect("create");

    let resp = client
        .batch_get_service_level_objective_budget_report()
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .slo_ids("BudgetSlo")
        .slo_ids("missing")
        .send()
        .await
        .expect("batch budget");
    assert_eq!(resp.reports().len(), 1);
    assert_eq!(resp.errors().len(), 1);
}

#[tokio::test]
async fn test_list_services_empty() {
    let client = make_client().await;
    let resp = client
        .list_services()
        .start_time(aws_smithy_types::DateTime::from_secs(0))
        .end_time(aws_smithy_types::DateTime::from_secs(1000))
        .send()
        .await
        .expect("list");
    assert!(resp.service_summaries().is_empty());
}

#[tokio::test]
async fn test_list_dependencies_empty() {
    let client = make_client().await;
    let resp = client
        .list_service_dependencies()
        .start_time(aws_smithy_types::DateTime::from_secs(0))
        .end_time(aws_smithy_types::DateTime::from_secs(1000))
        .key_attributes("Type", "Service")
        .send()
        .await
        .expect("list deps");
    assert!(resp.service_dependencies().is_empty());
}

#[tokio::test]
async fn test_grouping_configuration_round_trip() {
    let client = make_client().await;
    client
        .put_grouping_configuration()
        .send()
        .await
        .expect("put");

    let list = client
        .list_grouping_attribute_definitions()
        .send()
        .await
        .expect("list");
    assert!(list.updated_at().is_some());

    client
        .delete_grouping_configuration()
        .send()
        .await
        .expect("delete");

    let list2 = client
        .list_grouping_attribute_definitions()
        .send()
        .await
        .expect("list2");
    assert!(list2.updated_at().is_none());
}

#[tokio::test]
async fn test_start_discovery_idempotent() {
    let client = make_client().await;
    client.start_discovery().send().await.expect("first");
    client.start_discovery().send().await.expect("second");
}

#[tokio::test]
async fn test_tag_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_service_level_objective()
        .name("Tagged")
        .request_based_sli_config(sample_request_sli_config())
        .send()
        .await
        .expect("create");
    let arn = create.slo().unwrap().arn().to_string();

    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(Tag::builder().key("Env").value("prod").build().unwrap())
        .send()
        .await
        .expect("tag");

    let tags = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .expect("list tags");
    assert_eq!(tags.tags().len(), 1);

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
    let svc = ApplicationSignalsService::new();
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
    use winterbaume_applicationsignals::views::{
        ApplicationSignalsStateView, ServiceLevelObjectiveView,
    };
    let svc = ApplicationSignalsService::new();
    let mut view = ApplicationSignalsStateView::default();
    view.slos.insert(
        "id-1".to_string(),
        ServiceLevelObjectiveView {
            id: "id-1".to_string(),
            arn: "arn:aws:application-signals:us-east-1:123:slo/seed".to_string(),
            name: "seed".to_string(),
            evaluation_type: "PeriodBased".to_string(),
            metric_source_type: "ServiceOperation".to_string(),
            ..Default::default()
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .expect("restore");
    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snap.slos.len(), 1);
}
