# winterbaume-cloudwatch

CloudWatch service implementation for winterbaume.

## Coverage

| Metric | Value |
|---|---|
| Service | CloudWatch |
| AWS model | `cloudwatch` |
| Protocol | awsJson1.0 |
| winterbaume coverage | 38/46 operations (82.6%) |
| stubs (routed, returns empty/default) | 5/46 operations (10.9%) |
| moto coverage | 20/46 operations (43.5%) |
| floci coverage | 0/46 operations (0.0%) |
| kumo coverage | 7/46 operations (15.2%) |
| Coverage report date | 2026-05-06 |

## Server-mode usage

Start `winterbaume-server` and point the AWS CLI at it:

```sh
cargo run -p winterbaume-server -- --host 127.0.0.1 --port 5555
```

```sh
export AWS_ENDPOINT_URL=http://localhost:5555
aws cloudwatch list-metrics
```

## Example

```rust
use aws_sdk_cloudwatch::config::BehaviorVersion;
use winterbaume_cloudwatch::CloudWatchService;
use winterbaume_core::MockAws;

#[tokio::main]
async fn main() {
    let mock = MockAws::builder()
        .with_service(CloudWatchService::new())
        .build();

    let config = aws_config::defaults(BehaviorVersion::latest())
        .http_client(mock.http_client())
        .credentials_provider(mock.credentials_provider())
        .region(aws_sdk_cloudwatch::config::Region::new("us-east-1"))
        .load()
        .await;

    let client = aws_sdk_cloudwatch::Client::new(&config);

    let resp = client
        .list_metrics()
        .send()
        .await
        .expect("list_metrics should succeed");
    println!("Metrics: {}", resp.metrics().len());
}
```

## Implemented APIs (38)

- `DeleteAlarmMuteRule`
- `DeleteAlarms`
- `DeleteAnomalyDetector`
- `DeleteDashboards`
- `DeleteInsightRules`
- `DeleteMetricStream`
- `DescribeAlarms`
- `DescribeAlarmsForMetric`
- `DescribeAnomalyDetectors`
- `DescribeInsightRules`
- `DisableAlarmActions`
- `DisableInsightRules`
- `EnableAlarmActions`
- `EnableInsightRules`
- `GetAlarmMuteRule`
- `GetDashboard`
- `GetMetricData`
- `GetMetricStream`
- `ListAlarmMuteRules`
- `ListDashboards`
- `ListManagedInsightRules`
- `ListMetricStreams`
- `ListMetrics`
- `ListTagsForResource`
- `PutAlarmMuteRule`
- `PutAnomalyDetector`
- `PutCompositeAlarm`
- `PutDashboard`
- `PutInsightRule`
- `PutManagedInsightRules`
- `PutMetricAlarm`
- `PutMetricData`
- `PutMetricStream`
- `SetAlarmState`
- `StartMetricStreams`
- `StopMetricStreams`
- `TagResource`
- `UntagResource`

<details><summary>Stubbed APIs (5) &mdash; routed but return an empty/default response</summary>

- `DescribeAlarmContributors`
- `DescribeAlarmHistory`
- `GetInsightRuleReport`
- `GetMetricStatistics`
- `GetMetricWidgetImage`

</details>

<details><summary>Not yet implemented APIs (3)</summary>

- `GetOTelEnrichment`
- `StartOTelEnrichment`
- `StopOTelEnrichment`

</details>
