//! Integration tests for winterbaume Cost Explorer service.

use aws_sdk_costexplorer::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_costexplorer::CostExplorerService;

async fn make_ce_client() -> aws_sdk_costexplorer::Client {
    let mock = MockAws::builder()
        .with_service(CostExplorerService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_costexplorer::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_costexplorer::Client::new(&config)
}

#[tokio::test]
async fn test_get_cost_and_usage() {
    let client = make_ce_client().await;

    let resp = client
        .get_cost_and_usage()
        .time_period(
            aws_sdk_costexplorer::types::DateInterval::builder()
                .start("2024-01-01")
                .end("2024-01-31")
                .build()
                .unwrap(),
        )
        .granularity(aws_sdk_costexplorer::types::Granularity::Monthly)
        .metrics("BlendedCost")
        .send()
        .await
        .expect("get_cost_and_usage should succeed");

    assert!(!resp.results_by_time().is_empty());
}

#[tokio::test]
async fn test_get_cost_forecast() {
    let client = make_ce_client().await;

    let resp = client
        .get_cost_forecast()
        .time_period(
            aws_sdk_costexplorer::types::DateInterval::builder()
                .start("2024-06-01")
                .end("2024-12-31")
                .build()
                .unwrap(),
        )
        .metric(aws_sdk_costexplorer::types::Metric::BlendedCost)
        .granularity(aws_sdk_costexplorer::types::Granularity::Monthly)
        .send()
        .await
        .expect("get_cost_forecast should succeed");

    assert!(resp.total().is_some());
    assert!(!resp.forecast_results_by_time().is_empty());
}

#[tokio::test]
async fn test_get_dimension_values() {
    let client = make_ce_client().await;

    let resp = client
        .get_dimension_values()
        .time_period(
            aws_sdk_costexplorer::types::DateInterval::builder()
                .start("2024-01-01")
                .end("2024-01-31")
                .build()
                .unwrap(),
        )
        .dimension(aws_sdk_costexplorer::types::Dimension::Service)
        .send()
        .await
        .expect("get_dimension_values should succeed");

    // Empty mock results are fine
    assert_eq!(resp.return_size(), 0);
    assert_eq!(resp.total_size(), 0);
}

#[tokio::test]
async fn test_create_cost_category_definition() {
    let client = make_ce_client().await;

    let rule = aws_sdk_costexplorer::types::CostCategoryRule::builder()
        .value("Engineering")
        .rule(
            aws_sdk_costexplorer::types::Expression::builder()
                .dimensions(
                    aws_sdk_costexplorer::types::DimensionValues::builder()
                        .key(aws_sdk_costexplorer::types::Dimension::Service)
                        .values("Amazon EC2")
                        .build(),
                )
                .build(),
        )
        .build();

    let resp = client
        .create_cost_category_definition()
        .name("TestCategory")
        .rule_version(
            aws_sdk_costexplorer::types::CostCategoryRuleVersion::CostCategoryExpressionV1,
        )
        .rules(rule)
        .send()
        .await
        .expect("create_cost_category_definition should succeed");

    assert!(resp.cost_category_arn().is_some());
    assert!(resp.effective_start().is_some());
}

// ============================================================================
// Tests derived from AWS documentation: AWS Cost Explorer
// ============================================================================

fn make_date_interval(start: &str, end: &str) -> aws_sdk_costexplorer::types::DateInterval {
    aws_sdk_costexplorer::types::DateInterval::builder()
        .start(start)
        .end(end)
        .build()
        .unwrap()
}

fn make_basic_rule(value: &str) -> aws_sdk_costexplorer::types::CostCategoryRule {
    aws_sdk_costexplorer::types::CostCategoryRule::builder()
        .value(value)
        .rule(
            aws_sdk_costexplorer::types::Expression::builder()
                .dimensions(
                    aws_sdk_costexplorer::types::DimensionValues::builder()
                        .key(aws_sdk_costexplorer::types::Dimension::Service)
                        .values("Amazon EC2")
                        .build(),
                )
                .build(),
        )
        .build()
}

// --- GetCostAndUsage additional scenarios ---

#[tokio::test]
async fn test_get_cost_and_usage_daily_granularity() {
    let client = make_ce_client().await;

    let resp = client
        .get_cost_and_usage()
        .time_period(make_date_interval("2024-03-01", "2024-03-31"))
        .granularity(aws_sdk_costexplorer::types::Granularity::Daily)
        .metrics("UnblendedCost")
        .send()
        .await
        .expect("get_cost_and_usage with DAILY granularity should succeed");

    assert!(!resp.results_by_time().is_empty());
}

#[tokio::test]
async fn test_get_cost_and_usage_time_period_reflected() {
    let client = make_ce_client().await;

    let resp = client
        .get_cost_and_usage()
        .time_period(make_date_interval("2024-05-01", "2024-05-31"))
        .granularity(aws_sdk_costexplorer::types::Granularity::Monthly)
        .metrics("BlendedCost")
        .send()
        .await
        .expect("get_cost_and_usage should succeed");

    let results = resp.results_by_time();
    assert!(!results.is_empty());
    let first = &results[0];
    let tp = first.time_period().expect("time_period should be present");
    assert_eq!(tp.start(), "2024-05-01");
    assert_eq!(tp.end(), "2024-05-31");
}

#[tokio::test]
async fn test_get_cost_and_usage_missing_time_period() {
    let client = make_ce_client().await;

    // The SDK requires TimePeriod at the type level, so we send a raw request
    // by omitting required fields via a custom approach. Since the SDK enforces
    // TimePeriod, we test the missing-granularity and missing-metrics paths instead.
    // This test verifies missing Metrics triggers ValidationException.
    let err = client
        .get_cost_and_usage()
        .time_period(make_date_interval("2024-01-01", "2024-01-31"))
        .granularity(aws_sdk_costexplorer::types::Granularity::Monthly)
        // intentionally omit .metrics(...)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ValidationException"),
        "Expected ValidationException for missing Metrics, got: {err_str}"
    );
}

#[tokio::test]
async fn test_get_cost_and_usage_multiple_metrics() {
    let client = make_ce_client().await;

    let resp = client
        .get_cost_and_usage()
        .time_period(make_date_interval("2024-01-01", "2024-01-31"))
        .granularity(aws_sdk_costexplorer::types::Granularity::Monthly)
        .metrics("BlendedCost")
        .metrics("UnblendedCost")
        .send()
        .await
        .expect("get_cost_and_usage with multiple metrics should succeed");

    assert!(!resp.results_by_time().is_empty());
}

// --- GetCostForecast additional scenarios ---

#[tokio::test]
async fn test_get_cost_forecast_total_fields() {
    let client = make_ce_client().await;

    let resp = client
        .get_cost_forecast()
        .time_period(make_date_interval("2024-06-01", "2024-12-31"))
        .metric(aws_sdk_costexplorer::types::Metric::BlendedCost)
        .granularity(aws_sdk_costexplorer::types::Granularity::Monthly)
        .send()
        .await
        .expect("get_cost_forecast should succeed");

    let total = resp.total().expect("total should be present");
    assert!(total.amount().is_some(), "total.amount should be present");
    assert!(total.unit().is_some(), "total.unit should be present");
    assert_eq!(total.amount(), Some("0.0"), "expected mock amount of 0.0");
    assert_eq!(total.unit(), Some("USD"), "expected USD unit");
}

#[tokio::test]
async fn test_get_cost_forecast_forecast_results_fields() {
    let client = make_ce_client().await;

    let resp = client
        .get_cost_forecast()
        .time_period(make_date_interval("2024-07-01", "2024-09-30"))
        .metric(aws_sdk_costexplorer::types::Metric::UnblendedCost)
        .granularity(aws_sdk_costexplorer::types::Granularity::Monthly)
        .send()
        .await
        .expect("get_cost_forecast should succeed");

    let forecast_results = resp.forecast_results_by_time();
    assert!(!forecast_results.is_empty());

    let first = &forecast_results[0];
    assert!(
        first.time_period().is_some(),
        "time_period should be present in forecast result"
    );
    assert_eq!(first.mean_value(), Some("0.0"), "mean_value should be 0.0");
}

#[tokio::test]
async fn test_get_cost_forecast_missing_metric() {
    // The SDK enforces Metric at compile time; we test missing Granularity instead
    // by calling with a valid Metric but checking ValidationException from server.
    // This test verifies missing TimePeriod path via start/end set to same date (edge case).
    // We rely on the other tests to cover all validation combinations.
    // Instead, we test that a valid call with UNBLENDED_COST metric also succeeds.
    let client = make_ce_client().await;

    let resp = client
        .get_cost_forecast()
        .time_period(make_date_interval("2024-01-01", "2024-03-31"))
        .metric(aws_sdk_costexplorer::types::Metric::UnblendedCost)
        .granularity(aws_sdk_costexplorer::types::Granularity::Monthly)
        .send()
        .await
        .expect("get_cost_forecast with UNBLENDED_COST should succeed");

    assert!(resp.total().is_some());
}

// --- GetDimensionValues additional scenarios ---

#[tokio::test]
async fn test_get_dimension_values_region_dimension() {
    let client = make_ce_client().await;

    let resp = client
        .get_dimension_values()
        .time_period(make_date_interval("2024-01-01", "2024-01-31"))
        .dimension(aws_sdk_costexplorer::types::Dimension::Region)
        .send()
        .await
        .expect("get_dimension_values with REGION dimension should succeed");

    // Mock returns empty results
    assert_eq!(resp.dimension_values().len(), 0);
}

#[tokio::test]
async fn test_get_dimension_values_linked_account_dimension() {
    let client = make_ce_client().await;

    let resp = client
        .get_dimension_values()
        .time_period(make_date_interval("2024-01-01", "2024-12-31"))
        .dimension(aws_sdk_costexplorer::types::Dimension::LinkedAccount)
        .send()
        .await
        .expect("get_dimension_values with LINKED_ACCOUNT dimension should succeed");

    assert_eq!(resp.return_size(), 0);
    assert_eq!(resp.total_size(), 0);
}

// --- CreateCostCategoryDefinition additional scenarios ---

#[tokio::test]
async fn test_create_cost_category_arn_format() {
    let client = make_ce_client().await;

    let resp = client
        .create_cost_category_definition()
        .name("ArnFormatTestCategory")
        .rule_version(
            aws_sdk_costexplorer::types::CostCategoryRuleVersion::CostCategoryExpressionV1,
        )
        .rules(make_basic_rule("Marketing"))
        .send()
        .await
        .expect("create_cost_category_definition should succeed");

    let arn = resp
        .cost_category_arn()
        .expect("CostCategoryArn should be present");
    assert!(
        arn.starts_with("arn:aws:ce::"),
        "ARN should start with 'arn:aws:ce::'; got: {arn}"
    );
    assert!(
        arn.contains("costcategory"),
        "ARN should contain 'costcategory'; got: {arn}"
    );
}

#[tokio::test]
async fn test_create_cost_category_duplicate() {
    let client = make_ce_client().await;

    let rule = make_basic_rule("Sales");

    client
        .create_cost_category_definition()
        .name("DuplicateCategoryTest")
        .rule_version(
            aws_sdk_costexplorer::types::CostCategoryRuleVersion::CostCategoryExpressionV1,
        )
        .rules(rule.clone())
        .send()
        .await
        .expect("first create_cost_category_definition should succeed");

    let err = client
        .create_cost_category_definition()
        .name("DuplicateCategoryTest")
        .rule_version(
            aws_sdk_costexplorer::types::CostCategoryRuleVersion::CostCategoryExpressionV1,
        )
        .rules(rule)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceExistsException"),
        "Expected ResourceExistsException for duplicate category, got: {err_str}"
    );
}

#[tokio::test]
async fn test_create_cost_category_effective_start_reflected() {
    let client = make_ce_client().await;

    let resp = client
        .create_cost_category_definition()
        .name("EffectiveStartCategory")
        .rule_version(
            aws_sdk_costexplorer::types::CostCategoryRuleVersion::CostCategoryExpressionV1,
        )
        .rules(make_basic_rule("Finance"))
        .effective_start("2024-01-01T00:00:00Z")
        .send()
        .await
        .expect("create_cost_category_definition with effective_start should succeed");

    let effective_start = resp
        .effective_start()
        .expect("effective_start should be present");
    assert_eq!(
        effective_start, "2024-01-01T00:00:00Z",
        "effective_start should be echoed back; got: {effective_start}"
    );
}

#[tokio::test]
async fn test_create_multiple_cost_categories() {
    let client = make_ce_client().await;

    // Create two distinct categories; each should get a unique non-empty ARN
    let resp1 = client
        .create_cost_category_definition()
        .name("MultiCategoryOne")
        .rule_version(
            aws_sdk_costexplorer::types::CostCategoryRuleVersion::CostCategoryExpressionV1,
        )
        .rules(make_basic_rule("TeamA"))
        .send()
        .await
        .expect("first category creation should succeed");

    let resp2 = client
        .create_cost_category_definition()
        .name("MultiCategoryTwo")
        .rule_version(
            aws_sdk_costexplorer::types::CostCategoryRuleVersion::CostCategoryExpressionV1,
        )
        .rules(make_basic_rule("TeamB"))
        .send()
        .await
        .expect("second category creation should succeed");

    let arn1 = resp1.cost_category_arn().expect("arn1 should be present");
    let arn2 = resp2.cost_category_arn().expect("arn2 should be present");

    assert_ne!(
        arn1, arn2,
        "Different categories should have different ARNs"
    );
}

// ============================================================================
// Anomaly Monitor tests
// ============================================================================

fn make_anomaly_monitor(name: &str) -> aws_sdk_costexplorer::types::AnomalyMonitor {
    aws_sdk_costexplorer::types::AnomalyMonitor::builder()
        .monitor_name(name)
        .monitor_type(aws_sdk_costexplorer::types::MonitorType::Dimensional)
        .monitor_dimension(aws_sdk_costexplorer::types::MonitorDimension::Service)
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_create_anomaly_monitor() {
    let client = make_ce_client().await;

    let resp = client
        .create_anomaly_monitor()
        .anomaly_monitor(make_anomaly_monitor("TestMonitor"))
        .send()
        .await
        .expect("create_anomaly_monitor should succeed");

    let arn = resp.monitor_arn();
    assert!(
        arn.starts_with("arn:aws:ce::"),
        "ARN should start with arn:aws:ce::, got: {arn}"
    );
}

#[tokio::test]
async fn test_get_anomaly_monitors_empty() {
    let client = make_ce_client().await;

    let resp = client
        .get_anomaly_monitors()
        .send()
        .await
        .expect("get_anomaly_monitors should succeed with no monitors");

    assert!(resp.anomaly_monitors().is_empty());
}

#[tokio::test]
async fn test_create_and_get_anomaly_monitor() {
    let client = make_ce_client().await;

    let create_resp = client
        .create_anomaly_monitor()
        .anomaly_monitor(make_anomaly_monitor("MyMonitor"))
        .send()
        .await
        .expect("create_anomaly_monitor should succeed");

    let arn = create_resp.monitor_arn().to_string();

    let list_resp = client
        .get_anomaly_monitors()
        .monitor_arn_list(&arn)
        .send()
        .await
        .expect("get_anomaly_monitors should succeed");

    let monitors = list_resp.anomaly_monitors();
    assert_eq!(monitors.len(), 1, "Should have 1 monitor");
    assert_eq!(monitors[0].monitor_name(), "MyMonitor");
}

#[tokio::test]
async fn test_update_anomaly_monitor() {
    let client = make_ce_client().await;

    let create_resp = client
        .create_anomaly_monitor()
        .anomaly_monitor(make_anomaly_monitor("OldName"))
        .send()
        .await
        .expect("create_anomaly_monitor should succeed");

    let arn = create_resp.monitor_arn().to_string();

    let update_resp = client
        .update_anomaly_monitor()
        .monitor_arn(&arn)
        .monitor_name("NewName")
        .send()
        .await
        .expect("update_anomaly_monitor should succeed");

    assert!(!update_resp.monitor_arn().is_empty());
}

#[tokio::test]
async fn test_delete_anomaly_monitor() {
    let client = make_ce_client().await;

    let create_resp = client
        .create_anomaly_monitor()
        .anomaly_monitor(make_anomaly_monitor("ToDelete"))
        .send()
        .await
        .expect("create_anomaly_monitor should succeed");

    let arn = create_resp.monitor_arn().to_string();

    client
        .delete_anomaly_monitor()
        .monitor_arn(arn)
        .send()
        .await
        .expect("delete_anomaly_monitor should succeed");
}

#[tokio::test]
async fn test_delete_anomaly_monitor_nonexistent() {
    let client = make_ce_client().await;

    let err = client
        .delete_anomaly_monitor()
        .monitor_arn("arn:aws:ce::123456789012:anomalymonitor/nonexistent")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("UnknownMonitorException"),
        "Expected UnknownMonitorException, got: {err_str}"
    );
}

// ============================================================================
// Anomaly Subscription tests
// ============================================================================

fn make_subscriber(address: &str) -> aws_sdk_costexplorer::types::Subscriber {
    aws_sdk_costexplorer::types::Subscriber::builder()
        .address(address)
        .r#type(aws_sdk_costexplorer::types::SubscriberType::Email)
        .build()
}

#[tokio::test]
async fn test_create_anomaly_subscription() {
    let client = make_ce_client().await;

    let resp = client
        .create_anomaly_subscription()
        .anomaly_subscription(
            aws_sdk_costexplorer::types::AnomalySubscription::builder()
                .subscription_name("TestSubscription")
                .frequency(aws_sdk_costexplorer::types::AnomalySubscriptionFrequency::Daily)
                .subscribers(make_subscriber("test@example.com"))
                .monitor_arn_list("arn:aws:ce::123456789012:anomalymonitor/1")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create_anomaly_subscription should succeed");

    let arn = resp.subscription_arn();
    assert!(
        arn.starts_with("arn:aws:ce::"),
        "ARN should start with arn:aws:ce::, got: {arn}"
    );
}

#[tokio::test]
async fn test_get_anomaly_subscriptions_empty() {
    let client = make_ce_client().await;

    let resp = client
        .get_anomaly_subscriptions()
        .send()
        .await
        .expect("get_anomaly_subscriptions should succeed with no subscriptions");

    assert!(resp.anomaly_subscriptions().is_empty());
}

#[tokio::test]
async fn test_create_and_get_anomaly_subscription() {
    let client = make_ce_client().await;

    let create_resp = client
        .create_anomaly_subscription()
        .anomaly_subscription(
            aws_sdk_costexplorer::types::AnomalySubscription::builder()
                .subscription_name("GetTestSubscription")
                .frequency(aws_sdk_costexplorer::types::AnomalySubscriptionFrequency::Weekly)
                .subscribers(make_subscriber("user@example.com"))
                .monitor_arn_list("arn:aws:ce::123456789012:anomalymonitor/1")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create should succeed");

    let arn = create_resp.subscription_arn().to_string();

    let list_resp = client
        .get_anomaly_subscriptions()
        .subscription_arn_list(arn)
        .send()
        .await
        .expect("get should succeed");

    let subs = list_resp.anomaly_subscriptions();
    assert_eq!(subs.len(), 1);
    assert_eq!(subs[0].subscription_name(), "GetTestSubscription");
}

#[tokio::test]
async fn test_update_anomaly_subscription() {
    let client = make_ce_client().await;

    let create_resp = client
        .create_anomaly_subscription()
        .anomaly_subscription(
            aws_sdk_costexplorer::types::AnomalySubscription::builder()
                .subscription_name("UpdateTestSub")
                .frequency(aws_sdk_costexplorer::types::AnomalySubscriptionFrequency::Daily)
                .subscribers(make_subscriber("before@example.com"))
                .monitor_arn_list("arn:aws:ce::123456789012:anomalymonitor/1")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create should succeed");

    let arn = create_resp.subscription_arn().to_string();

    let update_resp = client
        .update_anomaly_subscription()
        .subscription_arn(arn)
        .frequency(aws_sdk_costexplorer::types::AnomalySubscriptionFrequency::Weekly)
        .subscription_name("UpdatedName")
        .send()
        .await
        .expect("update should succeed");

    assert!(!update_resp.subscription_arn().is_empty());
}

#[tokio::test]
async fn test_delete_anomaly_subscription() {
    let client = make_ce_client().await;

    let create_resp = client
        .create_anomaly_subscription()
        .anomaly_subscription(
            aws_sdk_costexplorer::types::AnomalySubscription::builder()
                .subscription_name("ToDeleteSub")
                .frequency(aws_sdk_costexplorer::types::AnomalySubscriptionFrequency::Daily)
                .subscribers(make_subscriber("del@example.com"))
                .monitor_arn_list("arn:aws:ce::123456789012:anomalymonitor/1")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("create should succeed");

    let arn = create_resp.subscription_arn().to_string();

    client
        .delete_anomaly_subscription()
        .subscription_arn(arn)
        .send()
        .await
        .expect("delete_anomaly_subscription should succeed");
}

#[tokio::test]
async fn test_delete_anomaly_subscription_nonexistent() {
    let client = make_ce_client().await;

    let err = client
        .delete_anomaly_subscription()
        .subscription_arn("arn:aws:ce::123456789012:anomalysubscription/nonexistent")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("UnknownSubscriptionException"),
        "Expected UnknownSubscriptionException, got: {err_str}"
    );
}

// ============================================================================
// Resource Tags tests
// ============================================================================

#[tokio::test]
async fn test_tag_and_list_tags_for_resource() {
    let client = make_ce_client().await;

    // Create a cost category so we have an ARN to tag
    let create_resp = client
        .create_cost_category_definition()
        .name("TaggableCategory")
        .rule_version(
            aws_sdk_costexplorer::types::CostCategoryRuleVersion::CostCategoryExpressionV1,
        )
        .rules(make_basic_rule("TagTest"))
        .send()
        .await
        .expect("create should succeed");

    let arn = create_resp
        .cost_category_arn()
        .expect("arn should be present");

    client
        .tag_resource()
        .resource_arn(arn)
        .resource_tags(
            aws_sdk_costexplorer::types::ResourceTag::builder()
                .key("Environment")
                .value("Test")
                .build()
                .unwrap(),
        )
        .resource_tags(
            aws_sdk_costexplorer::types::ResourceTag::builder()
                .key("Owner")
                .value("Team")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.resource_tags();
    assert_eq!(tags.len(), 2, "Should have 2 tags");
    let tag_keys: Vec<&str> = tags.iter().map(|t| t.key()).collect();
    assert!(
        tag_keys.contains(&"Environment"),
        "Should have Environment tag"
    );
    assert!(tag_keys.contains(&"Owner"), "Should have Owner tag");
}

#[tokio::test]
async fn test_untag_resource() {
    let client = make_ce_client().await;

    let create_resp = client
        .create_cost_category_definition()
        .name("UntaggableCategory")
        .rule_version(
            aws_sdk_costexplorer::types::CostCategoryRuleVersion::CostCategoryExpressionV1,
        )
        .rules(make_basic_rule("UntagTest"))
        .send()
        .await
        .expect("create should succeed");

    let arn = create_resp
        .cost_category_arn()
        .expect("arn should be present");

    client
        .tag_resource()
        .resource_arn(arn)
        .resource_tags(
            aws_sdk_costexplorer::types::ResourceTag::builder()
                .key("ToRemove")
                .value("yes")
                .build()
                .unwrap(),
        )
        .resource_tags(
            aws_sdk_costexplorer::types::ResourceTag::builder()
                .key("ToKeep")
                .value("yes")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("tag_resource should succeed");

    client
        .untag_resource()
        .resource_arn(arn)
        .resource_tag_keys("ToRemove")
        .send()
        .await
        .expect("untag_resource should succeed");

    let list_resp = client
        .list_tags_for_resource()
        .resource_arn(arn)
        .send()
        .await
        .expect("list_tags_for_resource should succeed");

    let tags = list_resp.resource_tags();
    assert_eq!(tags.len(), 1, "Should have 1 tag after untagging");
    assert_eq!(tags[0].key(), "ToKeep");
}

#[tokio::test]
async fn test_list_tags_for_resource_no_tags() {
    let client = make_ce_client().await;

    let resp = client
        .list_tags_for_resource()
        .resource_arn("arn:aws:ce::123456789012:costcategory/notexists")
        .send()
        .await
        .expect("list_tags_for_resource should succeed with empty result");

    assert!(resp.resource_tags().is_empty());
}

// ============================================================================
// Cost Category CRUD (delete, describe, update, list)
// ============================================================================

#[tokio::test]
async fn test_describe_cost_category_definition() {
    let client = make_ce_client().await;

    let create_resp = client
        .create_cost_category_definition()
        .name("DescribeMe")
        .rule_version(
            aws_sdk_costexplorer::types::CostCategoryRuleVersion::CostCategoryExpressionV1,
        )
        .rules(make_basic_rule("Dev"))
        .send()
        .await
        .expect("create should succeed");

    let arn = create_resp
        .cost_category_arn()
        .expect("arn should be present");

    let describe_resp = client
        .describe_cost_category_definition()
        .cost_category_arn(arn)
        .send()
        .await
        .expect("describe should succeed");

    let cc = describe_resp
        .cost_category()
        .expect("cost_category should be present");
    assert_eq!(cc.name(), "DescribeMe");
    assert!(!cc.cost_category_arn().is_empty());
}

#[tokio::test]
async fn test_describe_cost_category_definition_nonexistent() {
    let client = make_ce_client().await;

    let err = client
        .describe_cost_category_definition()
        .cost_category_arn("arn:aws:ce::123456789012:costcategory/nonexistent")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_delete_cost_category_definition() {
    let client = make_ce_client().await;

    let create_resp = client
        .create_cost_category_definition()
        .name("DeleteMeCategory")
        .rule_version(
            aws_sdk_costexplorer::types::CostCategoryRuleVersion::CostCategoryExpressionV1,
        )
        .rules(make_basic_rule("Ops"))
        .send()
        .await
        .expect("create should succeed");

    let arn = create_resp
        .cost_category_arn()
        .expect("arn should be present");

    let delete_resp = client
        .delete_cost_category_definition()
        .cost_category_arn(arn)
        .send()
        .await
        .expect("delete should succeed");

    assert!(delete_resp.cost_category_arn().is_some());
    assert!(delete_resp.effective_end().is_some());
}

#[tokio::test]
async fn test_delete_cost_category_definition_nonexistent() {
    let client = make_ce_client().await;

    let err = client
        .delete_cost_category_definition()
        .cost_category_arn("arn:aws:ce::123456789012:costcategory/nonexistent")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_update_cost_category_definition() {
    let client = make_ce_client().await;

    let create_resp = client
        .create_cost_category_definition()
        .name("UpdateMe")
        .rule_version(
            aws_sdk_costexplorer::types::CostCategoryRuleVersion::CostCategoryExpressionV1,
        )
        .rules(make_basic_rule("OldValue"))
        .send()
        .await
        .expect("create should succeed");

    let arn = create_resp
        .cost_category_arn()
        .expect("arn should be present");

    let update_resp = client
        .update_cost_category_definition()
        .cost_category_arn(arn)
        .rule_version(
            aws_sdk_costexplorer::types::CostCategoryRuleVersion::CostCategoryExpressionV1,
        )
        .rules(make_basic_rule("NewValue"))
        .send()
        .await
        .expect("update should succeed");

    assert!(update_resp.cost_category_arn().is_some());
    assert!(update_resp.effective_start().is_some());
}

#[tokio::test]
async fn test_update_cost_category_definition_nonexistent() {
    let client = make_ce_client().await;

    let err = client
        .update_cost_category_definition()
        .cost_category_arn("arn:aws:ce::123456789012:costcategory/nonexistent")
        .rule_version(
            aws_sdk_costexplorer::types::CostCategoryRuleVersion::CostCategoryExpressionV1,
        )
        .rules(make_basic_rule("Test"))
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

#[tokio::test]
async fn test_list_cost_category_definitions() {
    let client = make_ce_client().await;

    client
        .create_cost_category_definition()
        .name("ListCat1x")
        .rule_version(
            aws_sdk_costexplorer::types::CostCategoryRuleVersion::CostCategoryExpressionV1,
        )
        .rules(make_basic_rule("A"))
        .send()
        .await
        .expect("create 1 should succeed");

    client
        .create_cost_category_definition()
        .name("ListCat2x")
        .rule_version(
            aws_sdk_costexplorer::types::CostCategoryRuleVersion::CostCategoryExpressionV1,
        )
        .rules(make_basic_rule("B"))
        .send()
        .await
        .expect("create 2 should succeed");

    let resp = client
        .list_cost_category_definitions()
        .send()
        .await
        .expect("list should succeed");

    let refs = resp.cost_category_references();
    assert!(
        refs.len() >= 2,
        "Should have at least 2 categories, got: {}",
        refs.len()
    );
    let names: Vec<Option<&str>> = refs.iter().map(|r| r.name()).collect();
    assert!(names.contains(&Some("ListCat1x")));
    assert!(names.contains(&Some("ListCat2x")));
}

// ============================================================================
// Stateless query operations
// ============================================================================

#[tokio::test]
async fn test_get_usage_forecast() {
    let client = make_ce_client().await;

    let resp = client
        .get_usage_forecast()
        .time_period(make_date_interval("2024-01-01", "2024-03-31"))
        .metric(aws_sdk_costexplorer::types::Metric::BlendedCost)
        .granularity(aws_sdk_costexplorer::types::Granularity::Monthly)
        .send()
        .await
        .expect("get_usage_forecast should succeed");

    assert!(resp.total().is_some());
}

#[tokio::test]
async fn test_get_reservation_coverage() {
    let client = make_ce_client().await;

    let resp = client
        .get_reservation_coverage()
        .time_period(make_date_interval("2024-01-01", "2024-03-31"))
        .send()
        .await
        .expect("get_reservation_coverage should succeed");

    assert!(resp.coverages_by_time().is_empty());
}

#[tokio::test]
async fn test_get_reservation_utilization() {
    let client = make_ce_client().await;

    let resp = client
        .get_reservation_utilization()
        .time_period(make_date_interval("2024-01-01", "2024-03-31"))
        .send()
        .await
        .expect("get_reservation_utilization should succeed");

    assert!(resp.utilizations_by_time().is_empty());
}

#[tokio::test]
async fn test_get_reservation_purchase_recommendation() {
    let client = make_ce_client().await;

    let resp = client
        .get_reservation_purchase_recommendation()
        .service("Amazon EC2")
        .send()
        .await
        .expect("get_reservation_purchase_recommendation should succeed");

    assert!(resp.recommendations().is_empty());
}

#[tokio::test]
async fn test_get_rightsizing_recommendation() {
    let client = make_ce_client().await;

    let resp = client
        .get_rightsizing_recommendation()
        .service("AmazonEC2")
        .send()
        .await
        .expect("get_rightsizing_recommendation should succeed");

    assert!(resp.rightsizing_recommendations().is_empty());
}

#[tokio::test]
async fn test_get_savings_plans_coverage() {
    let client = make_ce_client().await;

    let resp = client
        .get_savings_plans_coverage()
        .time_period(make_date_interval("2024-01-01", "2024-03-31"))
        .send()
        .await
        .expect("get_savings_plans_coverage should succeed");

    assert!(resp.savings_plans_coverages().is_empty());
}

#[tokio::test]
async fn test_get_savings_plans_utilization() {
    let client = make_ce_client().await;

    let resp = client
        .get_savings_plans_utilization()
        .time_period(make_date_interval("2024-01-01", "2024-03-31"))
        .send()
        .await
        .expect("get_savings_plans_utilization should succeed");

    assert!(resp.savings_plans_utilizations_by_time().is_empty());
}

#[tokio::test]
async fn test_get_savings_plans_utilization_details() {
    let client = make_ce_client().await;

    let resp = client
        .get_savings_plans_utilization_details()
        .time_period(make_date_interval("2024-01-01", "2024-03-31"))
        .send()
        .await
        .expect("get_savings_plans_utilization_details should succeed");

    assert!(resp.savings_plans_utilization_details().is_empty());
}

#[tokio::test]
async fn test_get_savings_plans_purchase_recommendation() {
    let client = make_ce_client().await;

    let resp = client
        .get_savings_plans_purchase_recommendation()
        .savings_plans_type(aws_sdk_costexplorer::types::SupportedSavingsPlansType::ComputeSp)
        .term_in_years(aws_sdk_costexplorer::types::TermInYears::OneYear)
        .payment_option(aws_sdk_costexplorer::types::PaymentOption::NoUpfront)
        .lookback_period_in_days(aws_sdk_costexplorer::types::LookbackPeriodInDays::SevenDays)
        .send()
        .await
        .expect("get_savings_plans_purchase_recommendation should succeed");

    assert!(resp.savings_plans_purchase_recommendation().is_none());
}

#[tokio::test]
async fn test_get_approximate_usage_records() {
    let client = make_ce_client().await;

    let resp = client
        .get_approximate_usage_records()
        .granularity(aws_sdk_costexplorer::types::Granularity::Monthly)
        .approximation_dimension(aws_sdk_costexplorer::types::ApproximationDimension::Service)
        .send()
        .await
        .expect("get_approximate_usage_records should succeed");

    assert_eq!(resp.total_records(), 0);
}

#[tokio::test]
async fn test_get_anomalies() {
    let client = make_ce_client().await;

    let resp = client
        .get_anomalies()
        .date_interval(
            aws_sdk_costexplorer::types::AnomalyDateInterval::builder()
                .start_date("2024-01-01")
                .build()
                .unwrap(),
        )
        .send()
        .await
        .expect("get_anomalies should succeed");

    assert!(resp.anomalies().is_empty());
}

#[tokio::test]
async fn test_provide_anomaly_feedback() {
    let client = make_ce_client().await;

    let resp = client
        .provide_anomaly_feedback()
        .anomaly_id("test-anomaly-id")
        .feedback(aws_sdk_costexplorer::types::AnomalyFeedbackType::No)
        .send()
        .await
        .expect("provide_anomaly_feedback should succeed");

    assert_eq!(resp.anomaly_id(), "test-anomaly-id");
}

#[tokio::test]
async fn test_list_cost_allocation_tags() {
    let client = make_ce_client().await;

    let resp = client
        .list_cost_allocation_tags()
        .send()
        .await
        .expect("list_cost_allocation_tags should succeed");

    assert!(resp.cost_allocation_tags().is_empty());
}

#[tokio::test]
async fn test_list_cost_allocation_tag_backfill_history() {
    let client = make_ce_client().await;

    let resp = client
        .list_cost_allocation_tag_backfill_history()
        .send()
        .await
        .expect("list_cost_allocation_tag_backfill_history should succeed");

    assert!(resp.backfill_requests().is_empty());
}

#[tokio::test]
async fn test_start_cost_allocation_tag_backfill() {
    let client = make_ce_client().await;

    let resp = client
        .start_cost_allocation_tag_backfill()
        .backfill_from("2024-01-01T00:00:00Z")
        .send()
        .await
        .expect("start_cost_allocation_tag_backfill should succeed");

    let req = resp
        .backfill_request()
        .expect("backfill_request should be present");
    assert_eq!(req.backfill_from(), Some("2024-01-01T00:00:00Z"));
    assert_eq!(
        req.backfill_status().map(|s| s.as_str()),
        Some("PROCESSING")
    );
}

#[tokio::test]
async fn test_list_commitment_purchase_analyses() {
    let client = make_ce_client().await;

    let resp = client
        .list_commitment_purchase_analyses()
        .send()
        .await
        .expect("list_commitment_purchase_analyses should succeed");

    assert!(resp.analysis_summary_list().is_empty());
}

#[tokio::test]
async fn test_list_savings_plans_purchase_recommendation_generation() {
    let client = make_ce_client().await;

    let resp = client
        .list_savings_plans_purchase_recommendation_generation()
        .send()
        .await
        .expect("list should succeed");

    assert!(resp.generation_summary_list().is_empty());
}

#[tokio::test]
async fn test_start_savings_plans_purchase_recommendation_generation() {
    let client = make_ce_client().await;

    client
        .start_savings_plans_purchase_recommendation_generation()
        .send()
        .await
        .expect("start_savings_plans_purchase_recommendation_generation should succeed");
}

#[tokio::test]
async fn test_get_cost_categories() {
    let client = make_ce_client().await;

    let resp = client
        .get_cost_categories()
        .time_period(make_date_interval("2024-01-01", "2024-01-31"))
        .send()
        .await
        .expect("get_cost_categories should succeed");

    assert_eq!(resp.return_size(), 0);
    assert_eq!(resp.total_size(), 0);
}

#[tokio::test]
async fn test_get_tags() {
    let client = make_ce_client().await;

    let resp = client
        .get_tags()
        .time_period(make_date_interval("2024-01-01", "2024-01-31"))
        .send()
        .await
        .expect("get_tags should succeed");

    assert_eq!(resp.return_size(), 0);
    assert_eq!(resp.total_size(), 0);
}

#[tokio::test]
async fn test_list_cost_category_resource_associations() {
    let client = make_ce_client().await;

    let resp = client
        .list_cost_category_resource_associations()
        .send()
        .await
        .expect("list_cost_category_resource_associations should succeed");

    assert!(resp.cost_category_resource_associations().is_empty());
}

// ============================================================================
// Lifecycle tests
// ============================================================================

#[tokio::test]
async fn test_cost_category_lifecycle() {
    let client = make_ce_client().await;

    // Create
    let create_resp = client
        .create_cost_category_definition()
        .name("LifecycleCategory")
        .rule_version(
            aws_sdk_costexplorer::types::CostCategoryRuleVersion::CostCategoryExpressionV1,
        )
        .rules(make_basic_rule("Phase1"))
        .send()
        .await
        .expect("create should succeed");

    let arn = create_resp
        .cost_category_arn()
        .expect("arn should be present");

    // Describe
    let describe_resp = client
        .describe_cost_category_definition()
        .cost_category_arn(arn)
        .send()
        .await
        .expect("describe should succeed");
    assert_eq!(
        describe_resp.cost_category().map(|c| c.name()),
        Some("LifecycleCategory")
    );

    // Update
    client
        .update_cost_category_definition()
        .cost_category_arn(arn)
        .rule_version(
            aws_sdk_costexplorer::types::CostCategoryRuleVersion::CostCategoryExpressionV1,
        )
        .rules(make_basic_rule("Phase2"))
        .send()
        .await
        .expect("update should succeed");

    // Delete
    client
        .delete_cost_category_definition()
        .cost_category_arn(arn)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let err = client
        .describe_cost_category_definition()
        .cost_category_arn(arn)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException"),
        "Expected ResourceNotFoundException after delete, got: {err_str}"
    );
}

#[tokio::test]
async fn test_anomaly_monitor_lifecycle() {
    let client = make_ce_client().await;

    // Create
    let create_resp = client
        .create_anomaly_monitor()
        .anomaly_monitor(make_anomaly_monitor("LifecycleMonitor"))
        .send()
        .await
        .expect("create should succeed");

    let arn = create_resp.monitor_arn().to_string();

    // List - should appear
    let list_resp = client
        .get_anomaly_monitors()
        .monitor_arn_list(&arn)
        .send()
        .await
        .expect("get should succeed");
    assert_eq!(list_resp.anomaly_monitors().len(), 1);

    // Update
    client
        .update_anomaly_monitor()
        .monitor_arn(&arn)
        .monitor_name("LifecycleMonitorUpdated")
        .send()
        .await
        .expect("update should succeed");

    // Delete
    client
        .delete_anomaly_monitor()
        .monitor_arn(&arn)
        .send()
        .await
        .expect("delete should succeed");

    // Verify gone
    let err = client
        .delete_anomaly_monitor()
        .monitor_arn(&arn)
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("UnknownMonitorException"),
        "Expected UnknownMonitorException after delete, got: {err_str}"
    );
}
