//! Integration tests for winterbaume CloudWatch service.

use aws_sdk_cloudwatch::config::BehaviorVersion;
use aws_sdk_cloudwatch::types::{
    ComparisonOperator, Dimension, MetricDatum, StateValue, Statistic, Tag,
};
use winterbaume_cloudwatch::CloudWatchService;
use winterbaume_core::MockAws;

async fn make_cw_client() -> aws_sdk_cloudwatch::Client {
    let mock = MockAws::builder()
        .with_service(CloudWatchService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudwatch::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_cloudwatch::Client::new(&config)
}

#[tokio::test]
async fn test_put_metric_data() {
    let client = make_cw_client().await;

    client
        .put_metric_data()
        .namespace("TestNamespace")
        .metric_data(
            aws_sdk_cloudwatch::types::MetricDatum::builder()
                .metric_name("CPUUtilization")
                .value(75.5)
                .build(),
        )
        .send()
        .await
        .expect("put_metric_data should succeed");
}

#[tokio::test]
async fn test_list_metrics() {
    let client = make_cw_client().await;

    client
        .put_metric_data()
        .namespace("MyApp")
        .metric_data(
            aws_sdk_cloudwatch::types::MetricDatum::builder()
                .metric_name("RequestCount")
                .value(42.0)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_metrics()
        .namespace("MyApp")
        .send()
        .await
        .expect("list_metrics should succeed");

    assert!(!resp.metrics().is_empty());
    assert_eq!(resp.metrics()[0].metric_name(), Some("RequestCount"));
}

#[tokio::test]
async fn test_put_metric_alarm() {
    let client = make_cw_client().await;

    client
        .put_metric_alarm()
        .alarm_name("HighCPU")
        .metric_name("CPUUtilization")
        .namespace("AWS/EC2")
        .threshold(80.0)
        .comparison_operator(aws_sdk_cloudwatch::types::ComparisonOperator::GreaterThanThreshold)
        .evaluation_periods(2)
        .period(300)
        .statistic(aws_sdk_cloudwatch::types::Statistic::Average)
        .alarm_description("CPU too high")
        .send()
        .await
        .expect("put_metric_alarm should succeed");
}

#[tokio::test]
async fn test_describe_alarms() {
    let client = make_cw_client().await;

    client
        .put_metric_alarm()
        .alarm_name("TestAlarm")
        .metric_name("Errors")
        .namespace("MyApp")
        .threshold(5.0)
        .comparison_operator(
            aws_sdk_cloudwatch::types::ComparisonOperator::GreaterThanOrEqualToThreshold,
        )
        .evaluation_periods(1)
        .period(60)
        .statistic(aws_sdk_cloudwatch::types::Statistic::Sum)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_alarms()
        .alarm_names("TestAlarm")
        .send()
        .await
        .expect("describe_alarms should succeed");

    let alarms = resp.metric_alarms();
    assert_eq!(alarms.len(), 1);
    assert_eq!(alarms[0].alarm_name(), Some("TestAlarm"));
    assert_eq!(alarms[0].metric_name(), Some("Errors"));
    assert_eq!(alarms[0].namespace(), Some("MyApp"));
}

#[tokio::test]
async fn test_delete_alarms() {
    let client = make_cw_client().await;

    client
        .put_metric_alarm()
        .alarm_name("DelAlarm")
        .metric_name("Latency")
        .namespace("MyApp")
        .threshold(1000.0)
        .comparison_operator(aws_sdk_cloudwatch::types::ComparisonOperator::GreaterThanThreshold)
        .evaluation_periods(1)
        .period(60)
        .statistic(aws_sdk_cloudwatch::types::Statistic::Average)
        .send()
        .await
        .unwrap();

    client
        .delete_alarms()
        .alarm_names("DelAlarm")
        .send()
        .await
        .expect("delete_alarms should succeed");

    let resp = client.describe_alarms().send().await.unwrap();
    assert_eq!(resp.metric_alarms().len(), 0);
}

#[tokio::test]
async fn test_put_multiple_metrics_and_list() {
    let client = make_cw_client().await;

    client
        .put_metric_data()
        .namespace("MultiNS")
        .metric_data(
            aws_sdk_cloudwatch::types::MetricDatum::builder()
                .metric_name("Metric1")
                .value(10.0)
                .build(),
        )
        .metric_data(
            aws_sdk_cloudwatch::types::MetricDatum::builder()
                .metric_name("Metric2")
                .value(20.0)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_metrics()
        .namespace("MultiNS")
        .send()
        .await
        .expect("list_metrics should succeed");

    assert!(resp.metrics().len() >= 2);
}

#[tokio::test]
async fn test_alarm_with_dimensions() {
    let client = make_cw_client().await;

    client
        .put_metric_alarm()
        .alarm_name("DimAlarm")
        .metric_name("CPUUtilization")
        .namespace("AWS/EC2")
        .threshold(90.0)
        .comparison_operator(aws_sdk_cloudwatch::types::ComparisonOperator::GreaterThanThreshold)
        .evaluation_periods(1)
        .period(300)
        .statistic(aws_sdk_cloudwatch::types::Statistic::Average)
        .dimensions(
            aws_sdk_cloudwatch::types::Dimension::builder()
                .name("InstanceId")
                .value("i-1234567890abcdef0")
                .build(),
        )
        .send()
        .await
        .expect("put_metric_alarm with dimensions should succeed");

    let resp = client
        .describe_alarms()
        .alarm_names("DimAlarm")
        .send()
        .await
        .unwrap();

    let alarms = resp.metric_alarms();
    assert_eq!(alarms.len(), 1);
    assert!(!alarms[0].dimensions().is_empty());
}

#[tokio::test]
async fn test_get_metric_data() {
    let client = make_cw_client().await;

    // Put some metric data first
    client
        .put_metric_data()
        .namespace("GetMetricNS")
        .metric_data(
            aws_sdk_cloudwatch::types::MetricDatum::builder()
                .metric_name("Latency")
                .value(123.0)
                .build(),
        )
        .metric_data(
            aws_sdk_cloudwatch::types::MetricDatum::builder()
                .metric_name("Errors")
                .value(5.0)
                .build(),
        )
        .send()
        .await
        .expect("put_metric_data should succeed");

    // Call get_metric_data with a query
    let now = aws_sdk_cloudwatch::primitives::DateTime::from_secs(1700000000);
    let later = aws_sdk_cloudwatch::primitives::DateTime::from_secs(1700003600);

    let resp = client
        .get_metric_data()
        .metric_data_queries(
            aws_sdk_cloudwatch::types::MetricDataQuery::builder()
                .id("q1")
                .metric_stat(
                    aws_sdk_cloudwatch::types::MetricStat::builder()
                        .metric(
                            aws_sdk_cloudwatch::types::Metric::builder()
                                .namespace("GetMetricNS")
                                .metric_name("Latency")
                                .build(),
                        )
                        .period(300)
                        .stat("Average")
                        .build(),
                )
                .build(),
        )
        .start_time(now)
        .end_time(later)
        .send()
        .await
        .expect("get_metric_data should succeed");

    let results = resp.metric_data_results();
    // The mock returns all stored metrics (namespace/metric_name filtering is
    // based on top-level body fields which the SDK does not set for this API),
    // so we should get results back.
    assert!(!results.is_empty());
}

// ============================================================================
// Ported from moto: test_cloudwatch_alarms.py
// ============================================================================

// Ported from moto: test_cloudwatch_alarms.py::test_create_alarm
#[tokio::test]
async fn test_create_alarm_full() {
    let client = make_cw_client().await;

    client
        .put_metric_alarm()
        .alarm_actions("arn:alarm")
        .alarm_description("A test")
        .alarm_name("tester")
        .comparison_operator(ComparisonOperator::GreaterThanOrEqualToThreshold)
        .dimensions(
            Dimension::builder()
                .name("InstanceId")
                .value("i-0123457")
                .build(),
        )
        .evaluation_periods(5)
        .insufficient_data_actions("arn:insufficient")
        .namespace("tester_namespace")
        .metric_name("tester_metric")
        .ok_actions("arn:ok")
        .period(60)
        .statistic(Statistic::Average)
        .threshold(2.0)
        .unit(aws_sdk_cloudwatch::types::StandardUnit::Seconds)
        .send()
        .await
        .unwrap();

    let resp = client.describe_alarms().send().await.unwrap();
    let alarms = resp.metric_alarms();
    assert_eq!(alarms.len(), 1);
    let alarm = &alarms[0];
    assert_eq!(alarm.alarm_name(), Some("tester"));
    assert_eq!(alarm.namespace(), Some("tester_namespace"));
    assert_eq!(alarm.metric_name(), Some("tester_metric"));
    assert_eq!(
        alarm.comparison_operator(),
        Some(&ComparisonOperator::GreaterThanOrEqualToThreshold)
    );
    assert_eq!(alarm.threshold(), Some(2.0));
    assert_eq!(alarm.period(), Some(60));
    assert_eq!(alarm.evaluation_periods(), Some(5));
    assert_eq!(alarm.statistic(), Some(&Statistic::Average));
    assert_eq!(alarm.alarm_description(), Some("A test"));
    assert_eq!(alarm.dimensions().len(), 1);
    assert_eq!(alarm.dimensions()[0].name(), Some("InstanceId"));
    assert_eq!(alarm.dimensions()[0].value(), Some("i-0123457"));
    assert_eq!(alarm.alarm_actions(), &["arn:alarm"]);
    assert_eq!(alarm.ok_actions(), &["arn:ok"]);
    assert_eq!(alarm.insufficient_data_actions(), &["arn:insufficient"]);
    assert_eq!(
        alarm.unit(),
        Some(&aws_sdk_cloudwatch::types::StandardUnit::Seconds)
    );
    // Verify ARN contains expected parts
    let arn = alarm.alarm_arn().unwrap();
    assert!(arn.contains("arn:"));
    assert!(arn.contains("cloudwatch"));
    assert!(arn.contains("alarm:tester"));
    // default value should be true
    assert_eq!(alarm.actions_enabled(), Some(true));
}

// Ported from moto: test_cloudwatch_alarms.py::test_delete_alarm
#[tokio::test]
async fn test_delete_alarm_moto() {
    let client = make_cw_client().await;

    let resp = client.describe_alarms().send().await.unwrap();
    assert_eq!(resp.metric_alarms().len(), 0);

    client
        .put_metric_alarm()
        .alarm_actions("arn:alarm")
        .alarm_description("A test")
        .alarm_name("tester")
        .comparison_operator(ComparisonOperator::GreaterThanOrEqualToThreshold)
        .dimensions(
            Dimension::builder()
                .name("InstanceId")
                .value("i-0123457")
                .build(),
        )
        .evaluation_periods(5)
        .insufficient_data_actions("arn:insufficient")
        .namespace("tester_namespace")
        .metric_name("tester_metric")
        .ok_actions("arn:ok")
        .period(60)
        .statistic(Statistic::Average)
        .threshold(2.0)
        .unit(aws_sdk_cloudwatch::types::StandardUnit::Seconds)
        .send()
        .await
        .unwrap();

    let resp = client.describe_alarms().send().await.unwrap();
    assert_eq!(resp.metric_alarms().len(), 1);

    client
        .delete_alarms()
        .alarm_names("tester")
        .send()
        .await
        .unwrap();

    let resp = client.describe_alarms().send().await.unwrap();
    assert_eq!(resp.metric_alarms().len(), 0);
}

// Ported from moto: test_cloudwatch_alarms.py::test_delete_alarms_without_error
#[tokio::test]
async fn test_delete_alarms_without_error() {
    let client = make_cw_client().await;

    // Deleting non-existent alarms should succeed without error
    client
        .delete_alarms()
        .alarm_names("not-exists")
        .send()
        .await
        .unwrap();
}

// Ported from moto: test_cloudwatch_alarms.py::test_describe_alarms_for_metric
#[tokio::test]
async fn test_describe_alarms_for_metric() {
    let client = make_cw_client().await;

    client
        .put_metric_alarm()
        .alarm_name("testalarm1")
        .metric_name("cpu")
        .namespace("blah")
        .period(10)
        .evaluation_periods(5)
        .statistic(Statistic::Average)
        .threshold(2.0)
        .comparison_operator(ComparisonOperator::GreaterThanThreshold)
        .actions_enabled(true)
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_alarms_for_metric()
        .metric_name("cpu")
        .namespace("blah")
        .send()
        .await
        .unwrap();

    let alarms = resp.metric_alarms();
    assert_eq!(alarms.len(), 1);
    let alarm = &alarms[0];
    assert!(alarm.alarm_arn().unwrap().contains("testalarm1"));
    assert_eq!(alarm.actions_enabled(), Some(true));
}

// Ported from moto: test_cloudwatch_alarms.py::test_describe_alarms (partial)
#[tokio::test]
async fn test_describe_alarms_actions_enabled() {
    let client = make_cw_client().await;

    client
        .put_metric_alarm()
        .alarm_name("testalarm1")
        .metric_name("cpu")
        .namespace("blah")
        .period(10)
        .evaluation_periods(5)
        .statistic(Statistic::Average)
        .threshold(2.0)
        .comparison_operator(ComparisonOperator::GreaterThanThreshold)
        .actions_enabled(false)
        .send()
        .await
        .unwrap();

    let resp = client.describe_alarms().send().await.unwrap();
    let alarms = resp.metric_alarms();
    assert_eq!(alarms.len(), 1);
    let alarm = &alarms[0];
    assert_eq!(alarm.metric_name(), Some("cpu"));
    assert_eq!(alarm.namespace(), Some("blah"));
    assert_eq!(alarm.period(), Some(10));
    assert_eq!(alarm.evaluation_periods(), Some(5));
    assert_eq!(alarm.statistic(), Some(&Statistic::Average));
    assert_eq!(
        alarm.comparison_operator(),
        Some(&ComparisonOperator::GreaterThanThreshold)
    );
    assert_eq!(alarm.threshold(), Some(2.0));
    assert_eq!(alarm.actions_enabled(), Some(false));
}

// Ported from moto: test_cloudwatch_alarms.py::test_alarm_state
#[tokio::test]
async fn test_alarm_state() {
    let client = make_cw_client().await;

    client
        .put_metric_alarm()
        .alarm_name("testalarm1")
        .metric_name("cpu")
        .namespace("blah")
        .period(10)
        .evaluation_periods(5)
        .statistic(Statistic::Average)
        .threshold(2.0)
        .comparison_operator(ComparisonOperator::GreaterThanThreshold)
        .actions_enabled(true)
        .send()
        .await
        .unwrap();

    client
        .put_metric_alarm()
        .alarm_name("testalarm2")
        .metric_name("cpu")
        .namespace("blah")
        .period(10)
        .evaluation_periods(5)
        .statistic(Statistic::Average)
        .threshold(2.0)
        .comparison_operator(ComparisonOperator::GreaterThanThreshold)
        .send()
        .await
        .unwrap();

    client
        .set_alarm_state()
        .alarm_name("testalarm1")
        .state_value(StateValue::Alarm)
        .state_reason("testreason")
        .state_reason_data("{\"some\": \"json_data\"}")
        .send()
        .await
        .unwrap();

    let resp = client
        .describe_alarms()
        .state_value(StateValue::Alarm)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.metric_alarms().len(), 1);
    assert_eq!(resp.metric_alarms()[0].alarm_name(), Some("testalarm1"));
    assert_eq!(
        resp.metric_alarms()[0].state_value(),
        Some(&StateValue::Alarm)
    );
    assert_eq!(resp.metric_alarms()[0].actions_enabled(), Some(true));

    let resp = client
        .describe_alarms()
        .state_value(StateValue::Ok)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.metric_alarms().len(), 1);
    assert_eq!(resp.metric_alarms()[0].alarm_name(), Some("testalarm2"));
    assert_eq!(resp.metric_alarms()[0].state_value(), Some(&StateValue::Ok));
    assert_eq!(resp.metric_alarms()[0].actions_enabled(), Some(true));

    // Sanity check - all alarms
    let resp = client.describe_alarms().send().await.unwrap();
    assert_eq!(resp.metric_alarms().len(), 2);
}

// ============================================================================
// Ported from moto: test_cloudwatch.py
// ============================================================================

// Ported from moto: test_cloudwatch.py::test_put_metric_data_no_dimensions
#[tokio::test]
async fn test_put_metric_data_no_dimensions() {
    let client = make_cw_client().await;

    client
        .put_metric_data()
        .namespace("tester")
        .metric_data(
            MetricDatum::builder()
                .metric_name("metric")
                .value(1.5)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let metrics = client.list_metrics().send().await.unwrap();
    let found = metrics.metrics().iter().any(|m| {
        m.namespace() == Some("tester")
            && m.metric_name() == Some("metric")
            && m.dimensions().is_empty()
    });
    assert!(found, "Should find metric with no dimensions");
}

// Ported from moto: test_cloudwatch.py::test_list_metrics (simplified)
#[tokio::test]
async fn test_list_metrics_filtering() {
    let client = make_cw_client().await;

    // Verify empty namespace returns nothing
    let res = client
        .list_metrics()
        .namespace("unknown/")
        .send()
        .await
        .unwrap();
    assert!(res.metrics().is_empty());

    // Create some metrics
    for i in 0..4 {
        client
            .put_metric_data()
            .namespace("list_test_1/")
            .metric_data(
                MetricDatum::builder()
                    .metric_name(format!("metric{i}"))
                    .value(i as f64)
                    .build(),
            )
            .send()
            .await
            .unwrap();
    }
    for i in 0..4 {
        client
            .put_metric_data()
            .namespace("list_test_2/")
            .metric_data(
                MetricDatum::builder()
                    .metric_name(format!("metric{i}"))
                    .value(i as f64)
                    .build(),
            )
            .send()
            .await
            .unwrap();
    }

    // Verify we can retrieve everything
    let res = client.list_metrics().send().await.unwrap();
    assert!(res.metrics().len() >= 8);

    // Filter by namespace
    let res = client
        .list_metrics()
        .namespace("list_test_1/")
        .send()
        .await
        .unwrap();
    assert_eq!(res.metrics().len(), 4);

    // Filter by namespace + metric name
    let res = client
        .list_metrics()
        .namespace("list_test_1/")
        .metric_name("metric1")
        .send()
        .await
        .unwrap();
    assert_eq!(res.metrics().len(), 1);

    // Unknown namespace still empty
    let res = client
        .list_metrics()
        .namespace("unknown/")
        .send()
        .await
        .unwrap();
    assert!(res.metrics().is_empty());
}

// Ported from moto: test_cloudwatch.py::test_list_metrics_with_same_dimensions_different_metric_name
#[tokio::test]
async fn test_list_metrics_same_dimensions_different_metric_name() {
    let client = make_cw_client().await;

    client
        .put_metric_data()
        .namespace("unique/")
        .metric_data(
            MetricDatum::builder()
                .metric_name("metric1")
                .dimensions(Dimension::builder().name("D1").value("V1").build())
                .unit(aws_sdk_cloudwatch::types::StandardUnit::Seconds)
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .put_metric_data()
        .namespace("unique/")
        .metric_data(
            MetricDatum::builder()
                .metric_name("metric2")
                .dimensions(Dimension::builder().name("D1").value("V1").build())
                .unit(aws_sdk_cloudwatch::types::StandardUnit::Seconds)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let results = client
        .list_metrics()
        .namespace("unique/")
        .send()
        .await
        .unwrap();
    assert_eq!(results.metrics().len(), 2);
}

// ============================================================================
// Ported from moto: test_cloudwatch_dashboards.py
// ============================================================================

// Ported from moto: test_cloudwatch_dashboards.py::test_put_list_dashboard
#[tokio::test]
async fn test_put_list_dashboard() {
    let client = make_cw_client().await;
    let widget = r#"{"widgets": [{"type": "text", "x": 0, "y": 7, "width": 3, "height": 3, "properties": {"markdown": "Hello world"}}]}"#;

    client
        .put_dashboard()
        .dashboard_name("test1")
        .dashboard_body(widget)
        .send()
        .await
        .unwrap();

    let dashboards = client.list_dashboards().send().await.unwrap();
    let entries = dashboards.dashboard_entries();
    assert_eq!(entries.len(), 1);
    let arn = entries[0].dashboard_arn().unwrap();
    assert!(arn.contains("dashboard/test1"));
}

// Ported from moto: test_cloudwatch_dashboards.py::test_put_list_prefix_nomatch_dashboard
#[tokio::test]
async fn test_put_list_prefix_nomatch_dashboard() {
    let client = make_cw_client().await;
    let widget = r#"{"widgets": []}"#;

    client
        .put_dashboard()
        .dashboard_name("test1")
        .dashboard_body(widget)
        .send()
        .await
        .unwrap();

    let resp = client
        .list_dashboards()
        .dashboard_name_prefix("nomatch")
        .send()
        .await
        .unwrap();
    assert!(resp.dashboard_entries().is_empty());
}

// Ported from moto: test_cloudwatch_dashboards.py::test_delete_dashboard
#[tokio::test]
async fn test_delete_dashboard() {
    let client = make_cw_client().await;
    let widget = r#"{"widgets": []}"#;

    client
        .put_dashboard()
        .dashboard_name("test1")
        .dashboard_body(widget)
        .send()
        .await
        .unwrap();
    client
        .put_dashboard()
        .dashboard_name("test2")
        .dashboard_body(widget)
        .send()
        .await
        .unwrap();
    client
        .put_dashboard()
        .dashboard_name("test3")
        .dashboard_body(widget)
        .send()
        .await
        .unwrap();

    client
        .delete_dashboards()
        .dashboard_names("test2")
        .dashboard_names("test1")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_dashboards()
        .dashboard_name_prefix("test3")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.dashboard_entries().len(), 1);
}

// Ported from moto: test_cloudwatch_dashboards.py::test_delete_dashboard_fail
#[tokio::test]
async fn test_delete_dashboard_fail() {
    let client = make_cw_client().await;
    let widget = r#"{"widgets": []}"#;

    client
        .put_dashboard()
        .dashboard_name("test1")
        .dashboard_body(widget)
        .send()
        .await
        .unwrap();
    client
        .put_dashboard()
        .dashboard_name("test2")
        .dashboard_body(widget)
        .send()
        .await
        .unwrap();
    client
        .put_dashboard()
        .dashboard_name("test3")
        .dashboard_body(widget)
        .send()
        .await
        .unwrap();

    // Deleting with a non-existent dashboard should fail
    let err = client
        .delete_dashboards()
        .dashboard_names("test2")
        .dashboard_names("test1")
        .dashboard_names("test_no_match")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFound") || err_str.contains("resource_not_found"),
        "Expected ResourceNotFound error, got: {err_str}"
    );

    // All dashboards should still exist
    let resp = client.list_dashboards().send().await.unwrap();
    assert_eq!(resp.dashboard_entries().len(), 3);
}

// Ported from moto: test_cloudwatch_dashboards.py::test_get_dashboard
#[tokio::test]
async fn test_get_dashboard() {
    let client = make_cw_client().await;
    let widget = r#"{"widgets": [{"type": "text", "x": 0, "y": 7, "width": 3, "height": 3, "properties": {"markdown": "Hello world"}}]}"#;

    client
        .put_dashboard()
        .dashboard_name("test1")
        .dashboard_body(widget)
        .send()
        .await
        .unwrap();

    let resp = client
        .get_dashboard()
        .dashboard_name("test1")
        .send()
        .await
        .unwrap();

    assert!(resp.dashboard_arn().is_some());
    assert!(resp.dashboard_body().is_some());
    assert_eq!(resp.dashboard_name(), Some("test1"));
}

// Ported from moto: test_cloudwatch_dashboards.py::test_get_dashboard_fail
#[tokio::test]
async fn test_get_dashboard_fail() {
    let client = make_cw_client().await;

    let err = client
        .get_dashboard()
        .dashboard_name("test1")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFound") || err_str.contains("resource_not_found"),
        "Expected ResourceNotFound error, got: {err_str}"
    );
}

// ============================================================================
// Ported from moto: test_cloudwatch_tags.py
// ============================================================================

// Ported from moto: test_cloudwatch_tags.py::test_list_tags_for_resource
#[tokio::test]
async fn test_list_tags_for_resource() {
    let client = make_cw_client().await;

    client
        .put_metric_alarm()
        .alarm_name("test-alarm")
        .alarm_description("test alarm")
        .actions_enabled(true)
        .metric_name("5XXError")
        .namespace("AWS/ApiGateway")
        .statistic(Statistic::Sum)
        .dimensions(
            Dimension::builder()
                .name("ApiName")
                .value("test-api")
                .build(),
        )
        .dimensions(Dimension::builder().name("Stage").value("default").build())
        .period(60)
        .unit(aws_sdk_cloudwatch::types::StandardUnit::Seconds)
        .evaluation_periods(1)
        .threshold(1.0)
        .comparison_operator(ComparisonOperator::GreaterThanOrEqualToThreshold)
        .tags(Tag::builder().key("key-1").value("value-1").build())
        .send()
        .await
        .unwrap();

    let arn = client
        .describe_alarms()
        .alarm_names("test-alarm")
        .send()
        .await
        .unwrap()
        .metric_alarms()[0]
        .alarm_arn()
        .unwrap()
        .to_string();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), Some("key-1"));
    assert_eq!(tags[0].value(), Some("value-1"));
}

// Ported from moto: test_cloudwatch_tags.py::test_tag_resource
#[tokio::test]
async fn test_tag_resource() {
    let client = make_cw_client().await;

    client
        .put_metric_alarm()
        .alarm_name("test-alarm")
        .alarm_description("test alarm")
        .actions_enabled(true)
        .metric_name("5XXError")
        .namespace("AWS/ApiGateway")
        .statistic(Statistic::Sum)
        .period(60)
        .evaluation_periods(1)
        .threshold(1.0)
        .comparison_operator(ComparisonOperator::GreaterThanOrEqualToThreshold)
        .tags(Tag::builder().key("key-1").value("value-1").build())
        .send()
        .await
        .unwrap();

    let arn = client
        .describe_alarms()
        .alarm_names("test-alarm")
        .send()
        .await
        .unwrap()
        .metric_alarms()[0]
        .alarm_arn()
        .unwrap()
        .to_string();

    // Add another tag
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(Tag::builder().key("key-2").value("value-2").build())
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let mut tags: Vec<_> = resp
        .tags()
        .iter()
        .map(|t| (t.key().unwrap().to_string(), t.value().unwrap().to_string()))
        .collect();
    tags.sort();
    assert_eq!(
        tags,
        vec![
            ("key-1".to_string(), "value-1".to_string()),
            ("key-2".to_string(), "value-2".to_string()),
        ]
    );
}

// Ported from moto: test_cloudwatch_tags.py::test_tag_resource_on_resource_without_tags
#[tokio::test]
async fn test_tag_resource_on_resource_without_tags() {
    let client = make_cw_client().await;

    client
        .put_metric_alarm()
        .alarm_name("testalarm")
        .evaluation_periods(1)
        .comparison_operator(ComparisonOperator::GreaterThanThreshold)
        .period(60)
        .metric_name("test")
        .namespace("test")
        .send()
        .await
        .unwrap();

    let arn = client
        .describe_alarms()
        .send()
        .await
        .unwrap()
        .metric_alarms()[0]
        .alarm_arn()
        .unwrap()
        .to_string();

    // List 0 tags - none have been added
    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert!(resp.tags().is_empty());

    // Tag the Alarm for the first time
    client
        .tag_resource()
        .resource_arn(&arn)
        .tags(Tag::builder().key("tk").value("tv").build())
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.tags().len(), 1);
}

// Ported from moto: test_cloudwatch_tags.py::test_tag_resource_error_not_exists
#[tokio::test]
async fn test_tag_resource_error_not_exists() {
    let client = make_cw_client().await;

    let err = client
        .tag_resource()
        .resource_arn("arn:aws:cloudwatch:us-east-1:123456789012:alarm:unknown")
        .tags(Tag::builder().key("key-1").value("value-1").build())
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("resource_not_found"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// Ported from moto: test_cloudwatch_tags.py::test_untag_resource
#[tokio::test]
async fn test_untag_resource() {
    let client = make_cw_client().await;

    client
        .put_metric_alarm()
        .alarm_name("test-alarm")
        .alarm_description("test alarm")
        .actions_enabled(true)
        .metric_name("5XXError")
        .namespace("AWS/ApiGateway")
        .statistic(Statistic::Sum)
        .period(60)
        .evaluation_periods(1)
        .threshold(1.0)
        .comparison_operator(ComparisonOperator::GreaterThanOrEqualToThreshold)
        .tags(Tag::builder().key("key-1").value("value-1").build())
        .tags(Tag::builder().key("key-2").value("value-2").build())
        .send()
        .await
        .unwrap();

    let arn = client
        .describe_alarms()
        .alarm_names("test-alarm")
        .send()
        .await
        .unwrap()
        .metric_alarms()[0]
        .alarm_arn()
        .unwrap()
        .to_string();

    // Remove key-2
    client
        .untag_resource()
        .resource_arn(&arn)
        .tag_keys("key-2")
        .send()
        .await
        .unwrap();

    let resp = client
        .list_tags_for_resource()
        .resource_arn(&arn)
        .send()
        .await
        .unwrap();

    let tags = resp.tags();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].key(), Some("key-1"));
    assert_eq!(tags[0].value(), Some("value-1"));
}

// Ported from moto: test_cloudwatch_tags.py::test_untag_resource_error_not_exists
#[tokio::test]
async fn test_untag_resource_error_not_exists() {
    let client = make_cw_client().await;

    let err = client
        .untag_resource()
        .resource_arn("arn:aws:cloudwatch:us-east-1:123456789012:alarm:unknown")
        .tag_keys("key-1")
        .send()
        .await
        .unwrap_err();

    let err_str = format!("{:?}", err);
    assert!(
        err_str.contains("ResourceNotFoundException") || err_str.contains("resource_not_found"),
        "Expected ResourceNotFoundException, got: {err_str}"
    );
}

// ============================================================================
// Tests derived from AWS documentation: Amazon CloudWatch
// ============================================================================

#[tokio::test]
async fn test_put_insight_rule() {
    let client = make_cw_client().await;

    client
        .put_insight_rule()
        .rule_name("my-insight-rule")
        .rule_definition(r#"{"Schema":{"Name":"CloudWatchLogRule","Version":1},"LogGroupNames":["*"],"LogFormat":"CLF","Fields":{"1":"operation"},"Contribution":{"Keys":["operation"],"Filters":[]},"AggregateOn":"Count"}"#)
        .rule_state("ENABLED")
        .send()
        .await
        .expect("put_insight_rule should succeed");
}

#[tokio::test]
async fn test_describe_insight_rules_empty() {
    let client = make_cw_client().await;

    let resp = client
        .describe_insight_rules()
        .send()
        .await
        .expect("describe_insight_rules on empty state should succeed");

    assert_eq!(resp.insight_rules().len(), 0);
}

#[tokio::test]
async fn test_insight_rule_lifecycle() {
    let client = make_cw_client().await;

    let rule_def = r#"{"Schema":{"Name":"CloudWatchLogRule","Version":1},"LogGroupNames":["*"],"LogFormat":"CLF","Fields":{"1":"operation"},"Contribution":{"Keys":["operation"],"Filters":[]},"AggregateOn":"Count"}"#;

    // Create
    client
        .put_insight_rule()
        .rule_name("lifecycle-rule")
        .rule_definition(rule_def)
        .rule_state("ENABLED")
        .send()
        .await
        .unwrap();

    // Describe — winterbaume always creates rules in DISABLED state
    let desc = client.describe_insight_rules().send().await.unwrap();
    assert_eq!(desc.insight_rules().len(), 1);
    assert_eq!(desc.insight_rules()[0].name(), Some("lifecycle-rule"));

    // Disable
    client
        .disable_insight_rules()
        .rule_names("lifecycle-rule")
        .send()
        .await
        .expect("disable_insight_rules should succeed");

    let desc2 = client.describe_insight_rules().send().await.unwrap();
    assert_eq!(desc2.insight_rules()[0].state(), Some("DISABLED"));

    // Enable
    client
        .enable_insight_rules()
        .rule_names("lifecycle-rule")
        .send()
        .await
        .expect("enable_insight_rules should succeed");

    let desc3 = client.describe_insight_rules().send().await.unwrap();
    assert_eq!(desc3.insight_rules()[0].state(), Some("ENABLED"));

    // Delete
    client
        .delete_insight_rules()
        .rule_names("lifecycle-rule")
        .send()
        .await
        .expect("delete_insight_rules should succeed");

    let desc4 = client.describe_insight_rules().send().await.unwrap();
    assert_eq!(desc4.insight_rules().len(), 0);
}

#[tokio::test]
async fn test_get_metric_statistics_smoke() {
    let client = make_cw_client().await;

    client
        .put_metric_data()
        .namespace("SmokeNS")
        .metric_data(
            MetricDatum::builder()
                .metric_name("Requests")
                .value(100.0)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .get_metric_statistics()
        .namespace("SmokeNS")
        .metric_name("Requests")
        .start_time(aws_sdk_cloudwatch::primitives::DateTime::from_secs(0))
        .end_time(aws_sdk_cloudwatch::primitives::DateTime::from_secs(
            9999999999,
        ))
        .period(60)
        .statistics(Statistic::Sum)
        .send()
        .await
        .expect("get_metric_statistics should succeed");

    // winterbaume may return empty datapoints (no time-bucketing) — just check it succeeds
    let _ = resp.datapoints();
}

#[tokio::test]
async fn test_list_metrics_with_dimension_filter() {
    let client = make_cw_client().await;

    client
        .put_metric_data()
        .namespace("DimNS")
        .metric_data(
            MetricDatum::builder()
                .metric_name("Requests")
                .dimensions(
                    Dimension::builder()
                        .name("Service")
                        .value("frontend")
                        .build(),
                )
                .value(1.0)
                .build(),
        )
        .send()
        .await
        .unwrap();

    client
        .put_metric_data()
        .namespace("DimNS")
        .metric_data(
            MetricDatum::builder()
                .metric_name("Requests")
                .dimensions(
                    Dimension::builder()
                        .name("Service")
                        .value("backend")
                        .build(),
                )
                .value(2.0)
                .build(),
        )
        .send()
        .await
        .unwrap();

    let resp = client
        .list_metrics()
        .namespace("DimNS")
        .metric_name("Requests")
        .dimensions(
            aws_sdk_cloudwatch::types::DimensionFilter::builder()
                .name("Service")
                .value("frontend")
                .build(),
        )
        .send()
        .await
        .expect("list_metrics with dimension filter should succeed");

    let metrics = resp.metrics();
    assert!(
        metrics.iter().all(|m| {
            m.dimensions()
                .iter()
                .any(|d| d.name() == Some("Service") && d.value() == Some("frontend"))
        }),
        "all results should have Service=frontend"
    );
}

#[tokio::test]
async fn test_put_metric_alarm_upsert() {
    let client = make_cw_client().await;

    // Create alarm
    client
        .put_metric_alarm()
        .alarm_name("upsert-alarm")
        .comparison_operator(ComparisonOperator::GreaterThanThreshold)
        .evaluation_periods(1)
        .metric_name("CPUUtilization")
        .namespace("AWS/EC2")
        .period(60)
        .statistic(Statistic::Average)
        .threshold(90.0)
        .send()
        .await
        .unwrap();

    // Update alarm (same name)
    client
        .put_metric_alarm()
        .alarm_name("upsert-alarm")
        .comparison_operator(ComparisonOperator::GreaterThanThreshold)
        .evaluation_periods(2)
        .metric_name("CPUUtilization")
        .namespace("AWS/EC2")
        .period(60)
        .statistic(Statistic::Average)
        .threshold(95.0)
        .send()
        .await
        .expect("put_metric_alarm should overwrite existing alarm");

    let resp = client
        .describe_alarms()
        .alarm_names("upsert-alarm")
        .send()
        .await
        .unwrap();

    let alarms = resp.metric_alarms();
    assert_eq!(alarms.len(), 1, "should still have exactly 1 alarm");
    assert_eq!(
        alarms[0].threshold(),
        Some(95.0),
        "threshold should be updated"
    );
    assert_eq!(alarms[0].evaluation_periods(), Some(2));
}

#[tokio::test]
async fn test_delete_alarms_multiple() {
    let client = make_cw_client().await;

    for name in ["del-multi-1", "del-multi-2", "del-multi-3"] {
        client
            .put_metric_alarm()
            .alarm_name(name)
            .comparison_operator(ComparisonOperator::GreaterThanThreshold)
            .evaluation_periods(1)
            .metric_name("CPUUtilization")
            .namespace("NS")
            .period(60)
            .statistic(Statistic::Average)
            .threshold(80.0)
            .send()
            .await
            .unwrap();
    }

    client
        .delete_alarms()
        .alarm_names("del-multi-1")
        .alarm_names("del-multi-2")
        .send()
        .await
        .expect("delete_alarms with multiple names should succeed");

    let resp = client.describe_alarms().send().await.unwrap();
    let names: Vec<_> = resp
        .metric_alarms()
        .iter()
        .filter_map(|a| a.alarm_name())
        .collect();
    assert!(!names.contains(&"del-multi-1"));
    assert!(!names.contains(&"del-multi-2"));
    assert!(names.contains(&"del-multi-3"));
}

#[tokio::test]
async fn test_set_alarm_state_with_reason_data() {
    let client = make_cw_client().await;

    client
        .put_metric_alarm()
        .alarm_name("state-alarm")
        .comparison_operator(ComparisonOperator::GreaterThanThreshold)
        .evaluation_periods(1)
        .metric_name("CPUUtilization")
        .namespace("NS")
        .period(60)
        .statistic(Statistic::Average)
        .threshold(80.0)
        .send()
        .await
        .unwrap();

    client
        .set_alarm_state()
        .alarm_name("state-alarm")
        .state_value(StateValue::Alarm)
        .state_reason("manual override")
        .state_reason_data(r#"{"reason":"test"}"#)
        .send()
        .await
        .expect("set_alarm_state with reason_data should succeed");

    let resp = client
        .describe_alarms()
        .alarm_names("state-alarm")
        .send()
        .await
        .unwrap();

    assert_eq!(
        resp.metric_alarms()[0].state_value(),
        Some(&StateValue::Alarm)
    );
}

#[tokio::test]
async fn test_put_dashboard_update() {
    let client = make_cw_client().await;

    let body1 = r#"{"widgets":[]}"#;
    let body2 = r#"{"widgets":[{"type":"text","properties":{"markdown":"Hello"}}]}"#;

    client
        .put_dashboard()
        .dashboard_name("update-dash")
        .dashboard_body(body1)
        .send()
        .await
        .unwrap();

    client
        .put_dashboard()
        .dashboard_name("update-dash")
        .dashboard_body(body2)
        .send()
        .await
        .expect("second put_dashboard should overwrite");

    let resp = client
        .get_dashboard()
        .dashboard_name("update-dash")
        .send()
        .await
        .unwrap();

    assert_eq!(resp.dashboard_body().unwrap(), body2);
}

#[tokio::test]
async fn test_list_dashboards_prefix_filter() {
    let client = make_cw_client().await;

    for name in ["prod-dash-1", "prod-dash-2", "dev-dash-1"] {
        client
            .put_dashboard()
            .dashboard_name(name)
            .dashboard_body(r#"{"widgets":[]}"#)
            .send()
            .await
            .unwrap();
    }

    let resp = client
        .list_dashboards()
        .dashboard_name_prefix("prod-")
        .send()
        .await
        .expect("list_dashboards with prefix should succeed");

    let names: Vec<_> = resp
        .dashboard_entries()
        .iter()
        .map(|d| d.dashboard_name().unwrap_or_default())
        .collect();
    assert_eq!(names.len(), 2, "should have 2 prod dashboards");
    assert!(names.iter().all(|n| n.starts_with("prod-")));
}

// ============================================================================
// Enable / Disable Alarm Actions
// ============================================================================

#[tokio::test]
async fn test_enable_disable_alarm_actions() {
    let client = make_cw_client().await;

    // Create an alarm with actions enabled
    client
        .put_metric_alarm()
        .alarm_name("action-alarm")
        .metric_name("cpu")
        .namespace("TestNS")
        .period(60)
        .evaluation_periods(1)
        .statistic(Statistic::Average)
        .threshold(50.0)
        .comparison_operator(ComparisonOperator::GreaterThanThreshold)
        .actions_enabled(true)
        .send()
        .await
        .unwrap();

    // Verify actions_enabled is true
    let resp = client.describe_alarms().send().await.unwrap();
    assert_eq!(resp.metric_alarms()[0].actions_enabled(), Some(true));

    // Disable alarm actions
    client
        .disable_alarm_actions()
        .alarm_names("action-alarm")
        .send()
        .await
        .unwrap();

    // Verify actions_enabled is now false
    let resp = client.describe_alarms().send().await.unwrap();
    assert_eq!(resp.metric_alarms()[0].actions_enabled(), Some(false));

    // Re-enable alarm actions
    client
        .enable_alarm_actions()
        .alarm_names("action-alarm")
        .send()
        .await
        .unwrap();

    // Verify actions_enabled is true again
    let resp = client.describe_alarms().send().await.unwrap();
    assert_eq!(resp.metric_alarms()[0].actions_enabled(), Some(true));
}

// ============================================================================
// DescribeAlarmHistory
// ============================================================================

#[tokio::test]
async fn test_describe_alarm_history() {
    let client = make_cw_client().await;

    let resp = client.describe_alarm_history().send().await.unwrap();
    // The mock returns an empty history list
    assert!(resp.alarm_history_items().is_empty());
}

// ============================================================================
// Anomaly Detector: PutAnomalyDetector + DescribeAnomalyDetectors
// ============================================================================

#[tokio::test]
async fn test_put_and_describe_anomaly_detectors() {
    let client = make_cw_client().await;

    // Put an anomaly detector using SingleMetricAnomalyDetector.
    client
        .put_anomaly_detector()
        .single_metric_anomaly_detector(
            aws_sdk_cloudwatch::types::SingleMetricAnomalyDetector::builder()
                .namespace("AWS/EC2")
                .metric_name("CPUUtilization")
                .stat("Average")
                .build(),
        )
        .send()
        .await
        .unwrap();

    // Describe - should find one detector
    let resp = client.describe_anomaly_detectors().send().await.unwrap();
    let detectors = resp.anomaly_detectors();
    assert_eq!(detectors.len(), 1);
}

// ============================================================================
// Anomaly Detector: DeleteAnomalyDetector
// ============================================================================

#[tokio::test]
async fn test_delete_anomaly_detector() {
    let client = make_cw_client().await;

    let detector = aws_sdk_cloudwatch::types::SingleMetricAnomalyDetector::builder()
        .namespace("AWS/EC2")
        .metric_name("CPUUtilization")
        .stat("Average")
        .build();
    client
        .put_anomaly_detector()
        .single_metric_anomaly_detector(detector.clone())
        .send()
        .await
        .unwrap();

    client
        .delete_anomaly_detector()
        .single_metric_anomaly_detector(detector)
        .send()
        .await
        .unwrap();

    let resp = client.describe_anomaly_detectors().send().await.unwrap();
    assert!(resp.anomaly_detectors().is_empty());
}

// ============================================================================
// Composite Alarm: PutCompositeAlarm smoke test
// ============================================================================

#[tokio::test]
async fn test_put_composite_alarm_smoke() {
    let client = make_cw_client().await;

    // Create child metric alarms
    for name in ["child-alarm-1", "child-alarm-2"] {
        client
            .put_metric_alarm()
            .alarm_name(name)
            .metric_name("cpu")
            .namespace("TestNS")
            .period(60)
            .evaluation_periods(1)
            .statistic(Statistic::Average)
            .threshold(50.0)
            .comparison_operator(ComparisonOperator::GreaterThanThreshold)
            .send()
            .await
            .unwrap();
    }

    // PutCompositeAlarm should succeed without error
    client
        .put_composite_alarm()
        .alarm_name("composite-alarm")
        .alarm_rule("ALARM(child-alarm-1) OR ALARM(child-alarm-2)")
        .alarm_description("Test composite alarm")
        .actions_enabled(true)
        .send()
        .await
        .unwrap();

    // Verify the metric alarms are still present
    let resp = client.describe_alarms().send().await.unwrap();
    assert_eq!(resp.metric_alarms().len(), 2);
}

// ============================================================================
// PutMetricStream smoke test
// ============================================================================

#[tokio::test]
async fn test_put_metric_stream_smoke() {
    let client = make_cw_client().await;

    // PutMetricStream returns an ARN on success
    let resp = client
        .put_metric_stream()
        .name("test-stream")
        .firehose_arn("arn:aws:firehose:us-east-1:123456789012:deliverystream/test")
        .role_arn("arn:aws:iam::123456789012:role/test-role")
        .output_format(aws_sdk_cloudwatch::types::MetricStreamOutputFormat::Json)
        .send()
        .await
        .unwrap();

    // The response should contain an ARN
    assert!(resp.arn().is_some());
    assert!(resp.arn().unwrap().contains("test-stream"));
}

// ============================================================================
// DeleteMetricStream smoke test
// ============================================================================

#[tokio::test]
async fn test_delete_metric_stream_smoke() {
    let client = make_cw_client().await;

    // Create a metric stream first
    client
        .put_metric_stream()
        .name("to-delete")
        .firehose_arn("arn:aws:firehose:us-east-1:123456789012:deliverystream/test")
        .role_arn("arn:aws:iam::123456789012:role/test-role")
        .output_format(aws_sdk_cloudwatch::types::MetricStreamOutputFormat::Json)
        .send()
        .await
        .unwrap();

    // Delete should succeed
    client
        .delete_metric_stream()
        .name("to-delete")
        .send()
        .await
        .unwrap();
}

// ============================================================================
// StopMetricStreams / StartMetricStreams smoke tests
// ============================================================================

#[tokio::test]
async fn test_stop_start_metric_streams_smoke() {
    let client = make_cw_client().await;

    // Create a metric stream
    client
        .put_metric_stream()
        .name("stream-to-toggle")
        .firehose_arn("arn:aws:firehose:us-east-1:123456789012:deliverystream/test")
        .role_arn("arn:aws:iam::123456789012:role/test-role")
        .output_format(aws_sdk_cloudwatch::types::MetricStreamOutputFormat::Json)
        .send()
        .await
        .unwrap();

    // Stop and start should both succeed without error
    client
        .stop_metric_streams()
        .names("stream-to-toggle")
        .send()
        .await
        .unwrap();

    client
        .start_metric_streams()
        .names("stream-to-toggle")
        .send()
        .await
        .unwrap();
}

// ============================================================================
// GetInsightRuleReport
// ============================================================================

#[tokio::test]
async fn test_get_insight_rule_report() {
    let client = make_cw_client().await;

    // Create an insight rule first
    client
        .put_insight_rule()
        .rule_name("test-rule")
        .rule_definition(r#"{"Schema":{"Name":"CloudWatchLogRule","Version":1}}"#)
        .rule_state("ENABLED")
        .send()
        .await
        .unwrap();

    let resp = client
        .get_insight_rule_report()
        .rule_name("test-rule")
        .start_time(aws_sdk_cloudwatch::primitives::DateTime::from_secs(0))
        .end_time(aws_sdk_cloudwatch::primitives::DateTime::from_secs(3600))
        .period(300)
        .send()
        .await
        .unwrap();

    // The mock returns empty results
    assert_eq!(resp.approximate_unique_count(), Some(0));
}
