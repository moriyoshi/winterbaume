use std::sync::{Arc, Mutex};

use aws_sdk_applicationinsights::config::BehaviorVersion;
use aws_sdk_applicationinsights::types::Tag;
use winterbaume_applicationinsights::ApplicationInsightsService;
use winterbaume_core::{MockAws, StatefulService};

async fn make_client() -> aws_sdk_applicationinsights::Client {
    let mock = MockAws::builder()
        .with_service(ApplicationInsightsService::new())
        .build();
    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_applicationinsights::config::Region::new(
            "us-east-1",
        ))
        .load()
        .await;
    aws_sdk_applicationinsights::Client::new(&config)
}

#[tokio::test]
async fn test_application_lifecycle() {
    let client = make_client().await;
    let create = client
        .create_application()
        .resource_group_name("rg1")
        .ops_center_enabled(true)
        .send()
        .await
        .expect("create");
    assert_eq!(
        create.application_info().unwrap().resource_group_name(),
        Some("rg1")
    );

    let got = client
        .describe_application()
        .resource_group_name("rg1")
        .send()
        .await
        .expect("describe");
    assert_eq!(
        got.application_info().unwrap().ops_center_enabled(),
        Some(true)
    );

    client
        .update_application()
        .resource_group_name("rg1")
        .ops_center_enabled(false)
        .send()
        .await
        .expect("update");

    let list = client.list_applications().send().await.expect("list");
    assert_eq!(list.application_info_list().len(), 1);

    client
        .delete_application()
        .resource_group_name("rg1")
        .send()
        .await
        .expect("delete");
}

#[tokio::test]
async fn test_duplicate_application_conflicts() {
    let client = make_client().await;
    client
        .create_application()
        .resource_group_name("dup")
        .send()
        .await
        .expect("first");
    let err = client
        .create_application()
        .resource_group_name("dup")
        .send()
        .await
        .expect_err("dup");
    assert!(format!("{err:?}").contains("ResourceInUseException"));
}

#[tokio::test]
async fn test_component_lifecycle() {
    let client = make_client().await;
    client
        .create_application()
        .resource_group_name("rgc")
        .send()
        .await
        .expect("app");
    client
        .create_component()
        .resource_group_name("rgc")
        .component_name("comp1")
        .resource_list("arn:aws:ec2:::instance/i-001")
        .send()
        .await
        .expect("create comp");

    let got = client
        .describe_component()
        .resource_group_name("rgc")
        .component_name("comp1")
        .send()
        .await
        .expect("describe");
    assert_eq!(
        got.application_component().unwrap().component_name(),
        Some("comp1")
    );

    client
        .update_component_configuration()
        .resource_group_name("rgc")
        .component_name("comp1")
        .monitor(true)
        .tier("CUSTOM".into())
        .send()
        .await
        .expect("update config");

    let cfg = client
        .describe_component_configuration()
        .resource_group_name("rgc")
        .component_name("comp1")
        .send()
        .await
        .expect("desc config");
    assert_eq!(cfg.monitor(), Some(true));

    let list = client
        .list_components()
        .resource_group_name("rgc")
        .send()
        .await
        .expect("list");
    assert_eq!(list.application_component_list().len(), 1);

    client
        .delete_component()
        .resource_group_name("rgc")
        .component_name("comp1")
        .send()
        .await
        .expect("delete");
}

#[tokio::test]
async fn test_log_pattern_lifecycle() {
    let client = make_client().await;
    client
        .create_application()
        .resource_group_name("rgl")
        .send()
        .await
        .expect("app");

    client
        .create_log_pattern()
        .resource_group_name("rgl")
        .pattern_set_name("Java")
        .pattern_name("ExceptionPattern")
        .pattern("Exception")
        .rank(1)
        .send()
        .await
        .expect("create lp");

    let got = client
        .describe_log_pattern()
        .resource_group_name("rgl")
        .pattern_set_name("Java")
        .pattern_name("ExceptionPattern")
        .send()
        .await
        .expect("describe");
    assert_eq!(got.log_pattern().unwrap().pattern(), Some("Exception"));

    client
        .update_log_pattern()
        .resource_group_name("rgl")
        .pattern_set_name("Java")
        .pattern_name("ExceptionPattern")
        .pattern("Caused by")
        .rank(2)
        .send()
        .await
        .expect("update");

    let sets = client
        .list_log_pattern_sets()
        .resource_group_name("rgl")
        .send()
        .await
        .expect("list sets");
    assert_eq!(sets.log_pattern_sets().len(), 1);

    let patterns = client
        .list_log_patterns()
        .resource_group_name("rgl")
        .send()
        .await
        .expect("list patterns");
    assert_eq!(patterns.log_patterns().len(), 1);

    client
        .delete_log_pattern()
        .resource_group_name("rgl")
        .pattern_set_name("Java")
        .pattern_name("ExceptionPattern")
        .send()
        .await
        .expect("delete");
}

#[tokio::test]
async fn test_workload_lifecycle() {
    let client = make_client().await;
    client
        .create_application()
        .resource_group_name("rgw")
        .send()
        .await
        .expect("app");

    let add = client
        .add_workload()
        .resource_group_name("rgw")
        .component_name("comp")
        .send()
        .await
        .expect("add");
    let id = add.workload_id().expect("id").to_string();

    let got = client
        .describe_workload()
        .resource_group_name("rgw")
        .component_name("comp")
        .workload_id(&id)
        .send()
        .await
        .expect("describe");
    assert!(got.workload_id().is_some());

    let list = client
        .list_workloads()
        .resource_group_name("rgw")
        .component_name("comp")
        .send()
        .await
        .expect("list");
    assert_eq!(list.workload_list().len(), 1);

    client
        .remove_workload()
        .resource_group_name("rgw")
        .component_name("comp")
        .workload_id(&id)
        .send()
        .await
        .expect("remove");
}

#[tokio::test]
async fn test_problem_listing_default_empty() {
    let client = make_client().await;
    let resp = client.list_problems().send().await.expect("list problems");
    assert!(resp.problem_list().is_empty());
}

#[tokio::test]
async fn test_describe_problem_not_found() {
    let client = make_client().await;
    let err = client
        .describe_problem()
        .problem_id("missing")
        .send()
        .await
        .expect_err("missing");
    assert!(format!("{err:?}").contains("ResourceNotFoundException"));
}

#[tokio::test]
async fn test_tag_lifecycle() {
    let client = make_client().await;
    let arn = "arn:aws:applicationinsights:us-east-1:123:application/rgX";
    client
        .tag_resource()
        .resource_arn(arn)
        .tags(Tag::builder().key("Env").value("prod").build().unwrap())
        .send()
        .await
        .expect("tag");

    let tags = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .expect("list");
    assert_eq!(tags.tags().len(), 1);

    client
        .untag_resource()
        .resource_arn(arn)
        .tag_keys("Env")
        .send()
        .await
        .expect("untag");
}

#[tokio::test]
async fn test_state_change_listener_fires() {
    let svc = ApplicationInsightsService::new();
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
    use winterbaume_applicationinsights::views::{
        ApplicationInsightsStateView, ApplicationRecordView,
    };
    let svc = ApplicationInsightsService::new();
    let mut view = ApplicationInsightsStateView::default();
    view.applications.insert(
        "rg-seed".to_string(),
        ApplicationRecordView {
            resource_group_name: "rg-seed".to_string(),
            account_id: "123456789012".to_string(),
            life_cycle: "ACTIVE".to_string(),
            ..Default::default()
        },
    );
    svc.restore("123456789012", "us-east-1", view)
        .await
        .expect("restore");
    let snap = svc.snapshot("123456789012", "us-east-1").await;
    assert_eq!(snap.applications.len(), 1);
}
