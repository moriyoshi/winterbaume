//! End-to-end scenario tests for the winterbaume CloudWatch service.
//!
//! Each scenario chains 3+ operations and asserts business outcomes rather
//! than per-API return shapes.

use aws_sdk_cloudwatch::config::BehaviorVersion;
use aws_sdk_cloudwatch::types::{ComparisonOperator, StateValue, Statistic};
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

/// Scenario: Alarm lifecycle — create, fire, acknowledge, delete.
///
/// Models the typical operations a monitoring system performs when reacting
/// to an SLO breach: an alarm is created, its state is set to ALARM to
/// simulate a breach, the alarm is confirmed via describe, state is reset
/// to OK after remediation, and finally the alarm is deleted on clean-up.
#[tokio::test]
async fn test_alarm_breach_and_remediation_lifecycle() {
    let client = make_cw_client().await;

    // Step 1: Create a metric alarm for CPU headroom.
    client
        .put_metric_alarm()
        .alarm_name("CpuHeadroomAlarm")
        .metric_name("CPUUtilization")
        .namespace("AWS/EC2")
        .threshold(85.0)
        .comparison_operator(ComparisonOperator::GreaterThanThreshold)
        .evaluation_periods(2)
        .period(60)
        .statistic(Statistic::Average)
        .alarm_description("Fires when average CPU exceeds 85%")
        .send()
        .await
        .expect("put_metric_alarm should succeed");

    // Step 2: Simulate a breach — set the alarm to ALARM state.
    client
        .set_alarm_state()
        .alarm_name("CpuHeadroomAlarm")
        .state_value(StateValue::Alarm)
        .state_reason("CPU rose above threshold during load test")
        .send()
        .await
        .expect("set_alarm_state to ALARM should succeed");

    // Step 3: Verify the alarm is in ALARM state.
    let resp = client
        .describe_alarms()
        .alarm_names("CpuHeadroomAlarm")
        .send()
        .await
        .expect("describe_alarms should succeed");

    let alarms = resp.metric_alarms();
    assert_eq!(alarms.len(), 1, "Exactly one alarm expected");
    assert_eq!(
        alarms[0].state_value(),
        Some(&StateValue::Alarm),
        "Alarm must be in ALARM state after set_alarm_state"
    );

    // Step 4: Remediation complete — reset to OK.
    client
        .set_alarm_state()
        .alarm_name("CpuHeadroomAlarm")
        .state_value(StateValue::Ok)
        .state_reason("CPU returned to normal after auto-scaling")
        .send()
        .await
        .expect("set_alarm_state to OK should succeed");

    let resp = client
        .describe_alarms()
        .alarm_names("CpuHeadroomAlarm")
        .send()
        .await
        .unwrap();
    assert_eq!(
        resp.metric_alarms()[0].state_value(),
        Some(&StateValue::Ok),
        "Alarm must be in OK state after remediation"
    );

    // Step 5: Delete the alarm on clean-up.
    client
        .delete_alarms()
        .alarm_names("CpuHeadroomAlarm")
        .send()
        .await
        .expect("delete_alarms should succeed");

    let resp = client
        .describe_alarms()
        .alarm_names("CpuHeadroomAlarm")
        .send()
        .await
        .unwrap();
    assert!(
        resp.metric_alarms().is_empty(),
        "No alarms should remain after deletion"
    );
}

/// Scenario: Dashboard deploy, inspect, and teardown.
///
/// Models the workflow a dashboard-as-code pipeline uses: publish a dashboard,
/// verify it is discoverable via list and directly readable, then delete it
/// and confirm it is gone.
#[tokio::test]
async fn test_dashboard_publish_inspect_teardown() {
    let client = make_cw_client().await;

    let body = r#"{"widgets":[]}"#;

    // Step 1: Create the dashboard.
    client
        .put_dashboard()
        .dashboard_name("SREDashboard")
        .dashboard_body(body)
        .send()
        .await
        .expect("put_dashboard should succeed");

    // Step 2: Confirm it appears in the listing.
    let list_resp = client
        .list_dashboards()
        .dashboard_name_prefix("SRE")
        .send()
        .await
        .expect("list_dashboards should succeed");

    assert_eq!(
        list_resp.dashboard_entries().len(),
        1,
        "One dashboard expected after creation"
    );
    assert_eq!(
        list_resp.dashboard_entries()[0].dashboard_name(),
        Some("SREDashboard")
    );

    // Step 3: Fetch the dashboard body directly.
    let get_resp = client
        .get_dashboard()
        .dashboard_name("SREDashboard")
        .send()
        .await
        .expect("get_dashboard should succeed");

    assert_eq!(
        get_resp.dashboard_body(),
        Some(body),
        "Retrieved body must match what was put"
    );

    // Step 4: Delete and verify it is gone.
    client
        .delete_dashboards()
        .dashboard_names("SREDashboard")
        .send()
        .await
        .expect("delete_dashboards should succeed");

    let list_resp = client
        .list_dashboards()
        .dashboard_name_prefix("SRE")
        .send()
        .await
        .unwrap();
    assert!(
        list_resp.dashboard_entries().is_empty(),
        "No dashboards should remain after deletion"
    );
}

/// Scenario: Metric stream full lifecycle — create, start, stop, delete.
///
/// Models a streaming pipeline operator: provision a metric stream to a
/// Firehose delivery stream, verify it is listed and readable, stop streaming
/// temporarily, then remove it on decomission.
#[tokio::test]
async fn test_metric_stream_provision_operate_decomission() {
    let client = make_cw_client().await;

    let firehose_arn = "arn:aws:firehose:us-east-1:123456789012:deliverystream/MyStream";
    let role_arn = "arn:aws:iam::123456789012:role/MetricStreamRole";

    // Step 1: Provision the metric stream.
    let put_resp = client
        .put_metric_stream()
        .name("ProdMetricStream")
        .firehose_arn(firehose_arn)
        .role_arn(role_arn)
        .output_format(aws_sdk_cloudwatch::types::MetricStreamOutputFormat::Json)
        .send()
        .await
        .expect("put_metric_stream should succeed");

    assert!(
        put_resp.arn().is_some(),
        "put_metric_stream must return an ARN"
    );

    // Step 2: Confirm the stream appears in list and has correct details.
    let list_resp = client
        .list_metric_streams()
        .send()
        .await
        .expect("list_metric_streams should succeed");

    assert!(
        list_resp
            .entries()
            .iter()
            .any(|e| e.name() == Some("ProdMetricStream")),
        "New stream must appear in list"
    );

    // Step 3: Stop the stream to pause delivery.
    client
        .stop_metric_streams()
        .names("ProdMetricStream")
        .send()
        .await
        .expect("stop_metric_streams should succeed");

    let get_resp = client
        .get_metric_stream()
        .name("ProdMetricStream")
        .send()
        .await
        .expect("get_metric_stream should succeed");

    assert_eq!(
        get_resp.state(),
        Some("stopped"),
        "Stream state must be stopped after stop_metric_streams"
    );

    // Step 4: Restart the stream.
    client
        .start_metric_streams()
        .names("ProdMetricStream")
        .send()
        .await
        .expect("start_metric_streams should succeed");

    let get_resp = client
        .get_metric_stream()
        .name("ProdMetricStream")
        .send()
        .await
        .unwrap();
    assert_eq!(
        get_resp.state(),
        Some("running"),
        "Stream state must be running after start_metric_streams"
    );

    // Step 5: Decomission — delete the stream.
    client
        .delete_metric_stream()
        .name("ProdMetricStream")
        .send()
        .await
        .expect("delete_metric_stream should succeed");

    let list_resp = client.list_metric_streams().send().await.unwrap();
    assert!(
        !list_resp
            .entries()
            .iter()
            .any(|e| e.name() == Some("ProdMetricStream")),
        "Deleted stream must not appear in list"
    );
}
