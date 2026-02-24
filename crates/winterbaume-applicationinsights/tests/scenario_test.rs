/// Scenario: Full application monitoring setup pipeline.
///
/// An operator creates an Application Insights application, adds custom
/// components with resource lists, configures monitoring on those components,
/// attaches log patterns for error detection, and adds a workload. The test
/// then verifies the full resource graph is queryable and cleans up.
use aws_sdk_applicationinsights::config::BehaviorVersion;
use winterbaume_applicationinsights::ApplicationInsightsService;
use winterbaume_core::MockAws;

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

/// Scenario: Application monitoring setup — create app, components, log patterns, workload,
/// verify the full graph, then tear down.
#[tokio::test]
async fn test_application_monitoring_setup_pipeline() {
    let client = make_client().await;

    // Step 1: Create the application.
    let create = client
        .create_application()
        .resource_group_name("rg-pipeline")
        .ops_center_enabled(true)
        .cwe_monitor_enabled(true)
        .send()
        .await
        .expect("create application");
    let app_info = create.application_info().expect("app_info");
    assert_eq!(app_info.resource_group_name(), Some("rg-pipeline"));
    assert_eq!(app_info.ops_center_enabled(), Some(true));

    // Step 2: Add two components representing different tiers.
    client
        .create_component()
        .resource_group_name("rg-pipeline")
        .component_name("web-tier")
        .resource_list("arn:aws:elasticloadbalancing:us-east-1:123:loadbalancer/app/web/abc")
        .send()
        .await
        .expect("create web-tier component");

    client
        .create_component()
        .resource_group_name("rg-pipeline")
        .component_name("db-tier")
        .resource_list("arn:aws:rds:us-east-1:123:db:mydb")
        .send()
        .await
        .expect("create db-tier component");

    // Step 3: Configure monitoring on both components.
    client
        .update_component_configuration()
        .resource_group_name("rg-pipeline")
        .component_name("web-tier")
        .monitor(true)
        .tier("DEFAULT".into())
        .send()
        .await
        .expect("configure web-tier monitoring");

    client
        .update_component_configuration()
        .resource_group_name("rg-pipeline")
        .component_name("db-tier")
        .monitor(true)
        .tier("RDS_MYSQL".into())
        .send()
        .await
        .expect("configure db-tier monitoring");

    // Step 4: Create log patterns for error detection.
    client
        .create_log_pattern()
        .resource_group_name("rg-pipeline")
        .pattern_set_name("AppErrors")
        .pattern_name("JavaException")
        .pattern("Exception in thread")
        .rank(1)
        .send()
        .await
        .expect("create log pattern");

    client
        .create_log_pattern()
        .resource_group_name("rg-pipeline")
        .pattern_set_name("AppErrors")
        .pattern_name("NullPointer")
        .pattern("NullPointerException")
        .rank(2)
        .send()
        .await
        .expect("create second log pattern");

    // Step 5: Add a workload to the web-tier component.
    let add_wl = client
        .add_workload()
        .resource_group_name("rg-pipeline")
        .component_name("web-tier")
        .send()
        .await
        .expect("add workload");
    let workload_id = add_wl.workload_id().expect("workload_id").to_string();

    // Step 6: Verify the full resource graph.
    let components = client
        .list_components()
        .resource_group_name("rg-pipeline")
        .send()
        .await
        .expect("list components");
    assert_eq!(
        components.application_component_list().len(),
        2,
        "expected two components"
    );

    let patterns = client
        .list_log_patterns()
        .resource_group_name("rg-pipeline")
        .send()
        .await
        .expect("list log patterns");
    assert_eq!(
        patterns.log_patterns().len(),
        2,
        "expected two log patterns"
    );

    let pattern_sets = client
        .list_log_pattern_sets()
        .resource_group_name("rg-pipeline")
        .send()
        .await
        .expect("list log pattern sets");
    assert_eq!(
        pattern_sets.log_pattern_sets().len(),
        1,
        "expected one pattern set"
    );

    let workloads = client
        .list_workloads()
        .resource_group_name("rg-pipeline")
        .component_name("web-tier")
        .send()
        .await
        .expect("list workloads");
    assert_eq!(workloads.workload_list().len(), 1, "expected one workload");

    let web_cfg = client
        .describe_component_configuration()
        .resource_group_name("rg-pipeline")
        .component_name("web-tier")
        .send()
        .await
        .expect("describe web-tier config");
    assert_eq!(web_cfg.monitor(), Some(true));

    // Step 7: Verify describe_component_configuration_recommendation is accessible.
    let _rec = client
        .describe_component_configuration_recommendation()
        .resource_group_name("rg-pipeline")
        .component_name("web-tier")
        .tier("DEFAULT".into())
        .send()
        .await
        .expect("recommendation");

    // Step 8: Tear down — remove workload, delete components, delete application.
    client
        .remove_workload()
        .resource_group_name("rg-pipeline")
        .component_name("web-tier")
        .workload_id(&workload_id)
        .send()
        .await
        .expect("remove workload");

    client
        .delete_component()
        .resource_group_name("rg-pipeline")
        .component_name("web-tier")
        .send()
        .await
        .expect("delete web-tier");

    client
        .delete_component()
        .resource_group_name("rg-pipeline")
        .component_name("db-tier")
        .send()
        .await
        .expect("delete db-tier");

    client
        .delete_application()
        .resource_group_name("rg-pipeline")
        .send()
        .await
        .expect("delete application");

    // After deletion, listing should reflect empty state.
    let apps = client.list_applications().send().await.expect("list apps");
    assert!(
        apps.application_info_list().is_empty(),
        "expected no applications after deletion"
    );
}
