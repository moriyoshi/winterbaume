use aws_sdk_sagemakermetrics::config::BehaviorVersion;
use winterbaume_core::MockAws;
use winterbaume_sagemakermetrics::SageMakerMetricsService;

async fn make_client() -> aws_sdk_sagemakermetrics::Client {
    let mock = MockAws::builder()
        .with_service(SageMakerMetricsService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_sagemakermetrics::config::Region::new("us-east-1"))
        .load()
        .await;

    aws_sdk_sagemakermetrics::Client::new(&config)
}

#[tokio::test]
async fn test_batch_put_metrics_single() {
    let client = make_client().await;

    let metric = aws_sdk_sagemakermetrics::types::RawMetricData::builder()
        .metric_name("loss")
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .value(0.5)
        .build();

    let resp = client
        .batch_put_metrics()
        .trial_component_name("my-trial-component")
        .metric_data(metric)
        .send()
        .await
        .expect("batch_put_metrics should succeed");

    // Errors list should be empty on success
    assert!(resp.errors().is_empty());
}

#[tokio::test]
async fn test_batch_put_metrics_multiple() {
    let client = make_client().await;

    let m1 = aws_sdk_sagemakermetrics::types::RawMetricData::builder()
        .metric_name("loss")
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .value(0.5)
        .build();

    let m2 = aws_sdk_sagemakermetrics::types::RawMetricData::builder()
        .metric_name("accuracy")
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000001))
        .value(0.85)
        .step(1)
        .build();

    let resp = client
        .batch_put_metrics()
        .trial_component_name("my-trial-component")
        .metric_data(m1)
        .metric_data(m2)
        .send()
        .await
        .expect("batch_put_metrics should succeed");

    assert!(resp.errors().is_empty());
}

#[tokio::test]
async fn test_batch_put_metrics_with_step() {
    let client = make_client().await;

    let metric = aws_sdk_sagemakermetrics::types::RawMetricData::builder()
        .metric_name("loss")
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .value(0.3)
        .step(10)
        .build();

    let resp = client
        .batch_put_metrics()
        .trial_component_name("trial-with-step")
        .metric_data(metric)
        .send()
        .await
        .expect("batch_put_metrics with step should succeed");

    assert!(resp.errors().is_empty());
}

#[tokio::test]
async fn test_batch_get_metrics_empty() {
    let client = make_client().await;

    let query = aws_sdk_sagemakermetrics::types::MetricQuery::builder()
        .metric_name("loss")
        .resource_arn(
            "arn:aws:sagemaker:us-east-1:123456789012:experiment-trial-component/nonexistent",
        )
        .metric_stat(aws_sdk_sagemakermetrics::types::MetricStatistic::Avg)
        .period(aws_sdk_sagemakermetrics::types::Period::OneMinute)
        .build();

    let resp = client
        .batch_get_metrics()
        .metric_queries(query)
        .send()
        .await
        .expect("batch_get_metrics should succeed");

    let results = resp.metric_query_results();
    assert_eq!(results.len(), 1);
}

#[tokio::test]
async fn test_batch_put_then_get_metrics() {
    let client = make_client().await;

    // Put some metrics first
    let m1 = aws_sdk_sagemakermetrics::types::RawMetricData::builder()
        .metric_name("loss")
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .value(0.9)
        .step(0)
        .build();

    let m2 = aws_sdk_sagemakermetrics::types::RawMetricData::builder()
        .metric_name("loss")
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000060))
        .value(0.5)
        .step(1)
        .build();

    client
        .batch_put_metrics()
        .trial_component_name("training-job-1")
        .metric_data(m1)
        .metric_data(m2)
        .send()
        .await
        .expect("batch_put_metrics should succeed");

    // Now query them
    let query = aws_sdk_sagemakermetrics::types::MetricQuery::builder()
        .metric_name("loss")
        .resource_arn(
            "arn:aws:sagemaker:us-east-1:123456789012:experiment-trial-component/training-job-1",
        )
        .metric_stat(aws_sdk_sagemakermetrics::types::MetricStatistic::Avg)
        .period(aws_sdk_sagemakermetrics::types::Period::OneMinute)
        .build();

    let resp = client
        .batch_get_metrics()
        .metric_queries(query)
        .send()
        .await
        .expect("batch_get_metrics should succeed");

    let results = resp.metric_query_results();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].metric_values().len(), 2);
    assert!((results[0].metric_values()[0] - 0.9).abs() < f64::EPSILON);
    assert!((results[0].metric_values()[1] - 0.5).abs() < f64::EPSILON);
}

#[tokio::test]
async fn test_batch_get_metrics_multiple_queries() {
    let client = make_client().await;

    // Put metrics for two different metric names
    let m1 = aws_sdk_sagemakermetrics::types::RawMetricData::builder()
        .metric_name("loss")
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .value(0.5)
        .build();

    let m2 = aws_sdk_sagemakermetrics::types::RawMetricData::builder()
        .metric_name("accuracy")
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .value(0.95)
        .build();

    client
        .batch_put_metrics()
        .trial_component_name("multi-metric-job")
        .metric_data(m1)
        .metric_data(m2)
        .send()
        .await
        .unwrap();

    // Query both metric names
    let q1 = aws_sdk_sagemakermetrics::types::MetricQuery::builder()
        .metric_name("loss")
        .resource_arn(
            "arn:aws:sagemaker:us-east-1:123456789012:experiment-trial-component/multi-metric-job",
        )
        .metric_stat(aws_sdk_sagemakermetrics::types::MetricStatistic::Avg)
        .period(aws_sdk_sagemakermetrics::types::Period::OneMinute)
        .build();

    let q2 = aws_sdk_sagemakermetrics::types::MetricQuery::builder()
        .metric_name("accuracy")
        .resource_arn(
            "arn:aws:sagemaker:us-east-1:123456789012:experiment-trial-component/multi-metric-job",
        )
        .metric_stat(aws_sdk_sagemakermetrics::types::MetricStatistic::Avg)
        .period(aws_sdk_sagemakermetrics::types::Period::OneMinute)
        .build();

    let resp = client
        .batch_get_metrics()
        .metric_queries(q1)
        .metric_queries(q2)
        .send()
        .await
        .expect("batch_get_metrics should succeed");

    let results = resp.metric_query_results();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].metric_values().len(), 1);
    assert_eq!(results[1].metric_values().len(), 1);
}

// ============================================================================
// Tests derived from AWS documentation: SageMaker Metrics
// ============================================================================

#[tokio::test]
async fn test_batch_put_empty_metric_data_returns_error() {
    // Empty MetricData should return a validation error
    use bytes::Bytes;
    use winterbaume_core::{MockRequest, MockService};
    use winterbaume_sagemakermetrics::SageMakerMetricsService;

    let service = SageMakerMetricsService::new();
    let body = serde_json::json!({
        "TrialComponentName": "test-component",
        "MetricData": []
    });
    let req = MockRequest {
        method: "PUT".to_string(),
        uri: "https://metrics.sagemaker.us-east-1.amazonaws.com/BatchPutMetrics".to_string(),
        headers: http::HeaderMap::new(),
        body: Bytes::from(body.to_string()),
    };
    let resp = service.handle(req).await;
    assert_eq!(resp.status, 400, "empty MetricData should return 400");
}

#[tokio::test]
async fn test_batch_get_metrics_status_is_complete() {
    let client = make_client().await;

    let metric = aws_sdk_sagemakermetrics::types::RawMetricData::builder()
        .metric_name("loss")
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .value(0.42)
        .build();

    client
        .batch_put_metrics()
        .trial_component_name("status-test-component")
        .metric_data(metric)
        .send()
        .await
        .expect("batch_put_metrics should succeed");

    let query = aws_sdk_sagemakermetrics::types::MetricQuery::builder()
        .metric_name("loss")
        .resource_arn("arn:aws:sagemaker:us-east-1:123456789012:experiment-trial-component/status-test-component")
        .metric_stat(aws_sdk_sagemakermetrics::types::MetricStatistic::Avg)
        .period(aws_sdk_sagemakermetrics::types::Period::OneMinute)
        .build();

    let resp = client
        .batch_get_metrics()
        .metric_queries(query)
        .send()
        .await
        .expect("batch_get_metrics should succeed");

    let results = resp.metric_query_results();
    assert_eq!(results.len(), 1);
    // Status should be Complete
    assert_eq!(
        results[0].status(),
        Some(&aws_sdk_sagemakermetrics::types::MetricQueryResultStatus::Complete)
    );
    assert_eq!(results[0].metric_values().len(), 1);
    assert!((results[0].metric_values()[0] - 0.42).abs() < f64::EPSILON);
}

#[tokio::test]
async fn test_batch_get_metrics_x_axis_step() {
    // When XAxisType=IterationNumber (Step), x_axis_values should be step numbers
    let client = make_client().await;

    let m1 = aws_sdk_sagemakermetrics::types::RawMetricData::builder()
        .metric_name("loss")
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
        .value(0.9)
        .step(5)
        .build();

    let m2 = aws_sdk_sagemakermetrics::types::RawMetricData::builder()
        .metric_name("loss")
        .timestamp(aws_smithy_types::DateTime::from_secs(1700000060))
        .value(0.7)
        .step(10)
        .build();

    client
        .batch_put_metrics()
        .trial_component_name("step-axis-component")
        .metric_data(m1)
        .metric_data(m2)
        .send()
        .await
        .expect("batch_put_metrics should succeed");

    let query = aws_sdk_sagemakermetrics::types::MetricQuery::builder()
        .metric_name("loss")
        .resource_arn("arn:aws:sagemaker:us-east-1:123456789012:experiment-trial-component/step-axis-component")
        .metric_stat(aws_sdk_sagemakermetrics::types::MetricStatistic::Avg)
        .period(aws_sdk_sagemakermetrics::types::Period::OneMinute)
        .x_axis_type(aws_sdk_sagemakermetrics::types::XAxisType::IterationNumber)
        .build();

    let resp = client
        .batch_get_metrics()
        .metric_queries(query)
        .send()
        .await
        .expect("batch_get_metrics should succeed");

    let results = resp.metric_query_results();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].metric_values().len(), 2);
    // x_axis_values should be step numbers: 5 and 10
    assert_eq!(results[0].x_axis_values().len(), 2);
    assert_eq!(results[0].x_axis_values()[0], 5);
    assert_eq!(results[0].x_axis_values()[1], 10);
}

#[tokio::test]
async fn test_batch_get_metrics_different_trial_components_isolated() {
    // Metrics for different trial components should not bleed into each other
    let client = make_client().await;

    for (name, value) in [("component-alpha", 0.1), ("component-beta", 0.9)] {
        let metric = aws_sdk_sagemakermetrics::types::RawMetricData::builder()
            .metric_name("loss")
            .timestamp(aws_smithy_types::DateTime::from_secs(1700000000))
            .value(value)
            .build();
        client
            .batch_put_metrics()
            .trial_component_name(name)
            .metric_data(metric)
            .send()
            .await
            .unwrap();
    }

    let query_alpha = aws_sdk_sagemakermetrics::types::MetricQuery::builder()
        .metric_name("loss")
        .resource_arn(
            "arn:aws:sagemaker:us-east-1:123456789012:experiment-trial-component/component-alpha",
        )
        .metric_stat(aws_sdk_sagemakermetrics::types::MetricStatistic::Avg)
        .period(aws_sdk_sagemakermetrics::types::Period::OneMinute)
        .build();

    let resp = client
        .batch_get_metrics()
        .metric_queries(query_alpha)
        .send()
        .await
        .unwrap();

    let results = resp.metric_query_results();
    assert_eq!(results[0].metric_values().len(), 1);
    assert!(
        (results[0].metric_values()[0] - 0.1).abs() < f64::EPSILON,
        "component-alpha should only return its own metrics"
    );
}
