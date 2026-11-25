//! Raw-protocol tests for the three request protocols CloudWatch declares.
//!
//! The AWS SDK for Rust sends `rpcv2Cbor`, the AWS CLI v2 and boto3 send
//! `awsJson1_0` ( issue #69 ), and older SDKs and Terraform send `awsQuery`.
//! These tests drive the service handler directly with each wire format.

use serde_json::{Value, json};
use winterbaume_cloudwatch::CloudWatchService;
use winterbaume_cloudwatch::views::CloudwatchStateView;
use winterbaume_core::{MockRequest, MockResponse, MockService, StatefulService};

const ACCOUNT: &str = "123456789012";
const REGION: &str = "us-east-1";
const ENDPOINT: &str = "https://monitoring.us-east-1.amazonaws.com/";

fn sigv4_headers(headers: &mut http::HeaderMap) {
    headers.insert(
        http::header::AUTHORIZATION,
        "AWS4-HMAC-SHA256 Credential=test/20261122/us-east-1/monitoring/aws4_request, \
         SignedHeaders=host;x-amz-date, Signature=0000"
            .parse()
            .unwrap(),
    );
    headers.insert("x-amz-date", "20261122T000000Z".parse().unwrap());
}

/// An `awsJson1_0` request, as the AWS CLI v2 sends it.
fn json_request(operation: &str, body: Value) -> MockRequest {
    let mut headers = http::HeaderMap::new();
    sigv4_headers(&mut headers);
    headers.insert(
        http::header::CONTENT_TYPE,
        "application/x-amz-json-1.0".parse().unwrap(),
    );
    headers.insert(
        "x-amz-target",
        format!("GraniteServiceVersion20100801.{operation}")
            .parse()
            .unwrap(),
    );
    MockRequest {
        method: "POST".into(),
        uri: ENDPOINT.into(),
        headers,
        body: serde_json::to_vec(&body).unwrap().into(),
    }
}

/// An `rpcv2Cbor` request, as the AWS SDK for Rust sends it.
fn cbor_request(operation: &str, body: Value) -> MockRequest {
    let mut bytes = Vec::new();
    ciborium::into_writer(&body, &mut bytes).unwrap();
    let mut headers = http::HeaderMap::new();
    sigv4_headers(&mut headers);
    headers.insert("smithy-protocol", "rpc-v2-cbor".parse().unwrap());
    headers.insert(
        http::header::CONTENT_TYPE,
        "application/cbor".parse().unwrap(),
    );
    MockRequest {
        method: "POST".into(),
        uri: format!(
            "https://monitoring.us-east-1.amazonaws.com/service/GraniteServiceVersion20100801/operation/{operation}"
        ),
        headers,
        body: bytes.into(),
    }
}

/// An `awsQuery` request with a form-encoded body.
fn query_request(form: &str) -> MockRequest {
    let mut headers = http::HeaderMap::new();
    sigv4_headers(&mut headers);
    headers.insert(
        http::header::CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    MockRequest {
        method: "POST".into(),
        uri: ENDPOINT.into(),
        headers,
        body: form.to_string().into(),
    }
}

/// Assert an `awsJson1_0` response and return its JSON body.
fn json_body(response: &MockResponse, status: u16) -> Value {
    let text = String::from_utf8_lossy(&response.body);
    assert_eq!(response.status, status, "body: {text}");
    assert_eq!(
        response.headers[http::header::CONTENT_TYPE],
        "application/x-amz-json-1.0"
    );
    assert!(!response.headers.contains_key("smithy-protocol"));
    serde_json::from_slice(&response.body).unwrap_or_else(|e| panic!("{e}: {text}"))
}

fn xml_body(response: &MockResponse, status: u16) -> String {
    let text = String::from_utf8(response.body.to_vec()).unwrap();
    assert_eq!(response.status, status, "body: {text}");
    text
}

fn put_alarm_json() -> Value {
    json!({
        "AlarmName": "wb-alarm",
        "Namespace": "AWS/SQS",
        "MetricName": "ApproximateNumberOfMessagesVisible",
        "Dimensions": [{"Name": "QueueName", "Value": "wb-dlq"}],
        "Statistic": "Maximum",
        "Period": 60,
        "EvaluationPeriods": 5,
        "DatapointsToAlarm": 3,
        "Threshold": 0,
        "ComparisonOperator": "GreaterThanThreshold",
        "TreatMissingData": "notBreaching",
        "ActionsEnabled": false,
    })
}

fn now_secs() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

/// Assert that a JSON timestamp member is an epoch-seconds number close to
/// now, not the CBOR Tag 1 sentinel object or an ISO string.
fn assert_epoch_number(value: &Value, member: &str) {
    let ts = &value[member];
    assert!(ts.is_number(), "{member} must be a number, got {ts}");
    let secs = ts.as_f64().unwrap();
    assert!(
        (now_secs() - secs).abs() < 300.0,
        "{member} = {secs} is not a current epoch-seconds value"
    );
}

#[tokio::test]
async fn json_put_metric_alarm_and_describe_alarms() {
    let service = CloudWatchService::new();

    let out = service
        .handle(json_request("PutMetricAlarm", put_alarm_json()))
        .await;
    assert_eq!(json_body(&out, 200), json!({}));

    let out = service
        .handle(json_request(
            "DescribeAlarms",
            json!({"AlarmNames": ["wb-alarm"]}),
        ))
        .await;
    let body = json_body(&out, 200);
    let alarms = body["MetricAlarms"].as_array().unwrap();
    assert_eq!(alarms.len(), 1);
    let alarm = &alarms[0];
    assert_eq!(alarm["AlarmName"], "wb-alarm");
    assert_eq!(
        alarm["AlarmArn"],
        "arn:aws:cloudwatch:us-east-1:123456789012:alarm:wb-alarm"
    );
    assert_eq!(alarm["TreatMissingData"], "notBreaching");
    assert_eq!(alarm["DatapointsToAlarm"], 3);
    assert_eq!(alarm["EvaluationPeriods"], 5);
    assert_eq!(alarm["Period"], 60);
    assert_eq!(alarm["Statistic"], "Maximum");
    assert_eq!(alarm["ActionsEnabled"], false);
    assert_eq!(alarm["StateValue"], "INSUFFICIENT_DATA");
    assert_eq!(alarm["StateReason"], "Unchecked: Initial alarm creation");
    assert_eq!(
        alarm["Dimensions"],
        json!([{"Name": "QueueName", "Value": "wb-dlq"}])
    );
    for member in [
        "StateUpdatedTimestamp",
        "StateTransitionedTimestamp",
        "AlarmConfigurationUpdatedTimestamp",
    ] {
        assert_epoch_number(alarm, member);
    }
    assert!(!out.body.windows(9).any(|w| w == b"__cbor_ep"));

    // StateValue filters see the initial state.
    let out = service
        .handle(json_request(
            "DescribeAlarms",
            json!({"StateValue": "INSUFFICIENT_DATA"}),
        ))
        .await;
    assert_eq!(
        json_body(&out, 200)["MetricAlarms"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let out = service
        .handle(json_request("DescribeAlarms", json!({"StateValue": "OK"})))
        .await;
    assert_eq!(json_body(&out, 200)["MetricAlarms"], json!([]));
}

#[tokio::test]
async fn json_empty_body_is_an_empty_structure() {
    let service = CloudWatchService::new();
    let mut request = json_request("DescribeAlarms", json!({}));
    request.body = bytes::Bytes::new();
    let body = json_body(&service.handle(request).await, 200);
    assert_eq!(body["MetricAlarms"], json!([]));
}

#[tokio::test]
async fn json_errors_carry_type_message_and_query_error_header() {
    let service = CloudWatchService::new();

    let out = service
        .handle(json_request(
            "SetAlarmState",
            json!({"AlarmName": "missing", "StateValue": "ALARM", "StateReason": "x"}),
        ))
        .await;
    let body = json_body(&out, 404);
    assert_eq!(body["__type"], "ResourceNotFound");
    assert!(body["message"].as_str().unwrap().contains("missing"));
    assert_eq!(out.headers["x-amzn-query-error"], "ResourceNotFound;Sender");

    let out = service
        .handle(json_request("PutMetricAlarm", json!({"MetricName": "M"})))
        .await;
    assert_eq!(json_body(&out, 400)["__type"], "MissingParameter");
    assert_eq!(out.headers["x-amzn-query-error"], "MissingParameter;Sender");

    let out = service
        .handle(json_request("NoSuchOperation", json!({})))
        .await;
    assert_eq!(json_body(&out, 400)["__type"], "InvalidAction");

    let mut malformed = json_request("DescribeAlarms", json!({}));
    malformed.body = bytes::Bytes::from_static(b"{not json");
    let out = service.handle(malformed).await;
    assert_eq!(json_body(&out, 400)["__type"], "SerializationException");
}

#[tokio::test]
async fn json_put_metric_data_and_list_metrics() {
    let service = CloudWatchService::new();
    let out = service
        .handle(json_request(
            "PutMetricData",
            json!({
                "Namespace": "MyApp",
                "MetricData": [{
                    "MetricName": "RequestCount",
                    "Value": 42.5,
                    "Timestamp": now_secs(),
                    "Dimensions": [{"Name": "Stage", "Value": "prod"}],
                }],
            }),
        ))
        .await;
    assert_eq!(json_body(&out, 200), json!({}));

    let out = service
        .handle(json_request("ListMetrics", json!({"Namespace": "MyApp"})))
        .await;
    let body = json_body(&out, 200);
    assert_eq!(
        body["Metrics"],
        json!([{
            "Namespace": "MyApp",
            "MetricName": "RequestCount",
            "Dimensions": [{"Name": "Stage", "Value": "prod"}],
        }])
    );

    // A list of timestamps renders as a list of epoch-seconds numbers.
    let out = service
        .handle(json_request(
            "GetMetricData",
            json!({"Namespace": "MyApp", "MetricName": "RequestCount"}),
        ))
        .await;
    let body = json_body(&out, 200);
    let result = &body["MetricDataResults"][0];
    assert_eq!(result["Values"], json!([42.5]));
    assert!(result["Timestamps"][0].is_number(), "{result}");
}

#[tokio::test]
async fn json_blob_members_are_base64_strings() {
    let service = CloudWatchService::new();
    let out = service
        .handle(json_request(
            "GetMetricWidgetImage",
            json!({"MetricWidget": "{}"}),
        ))
        .await;
    let body = json_body(&out, 200);
    let image = body["MetricWidgetImage"].as_str().unwrap();
    use base64::Engine;
    let png = base64::engine::general_purpose::STANDARD
        .decode(image)
        .unwrap();
    assert!(png.starts_with(b"\x89PNG"));
}

#[tokio::test]
async fn cbor_errors_carry_query_error_header() {
    let service = CloudWatchService::new();
    let out = service
        .handle(cbor_request(
            "SetAlarmState",
            json!({"AlarmName": "missing", "StateValue": "ALARM", "StateReason": "x"}),
        ))
        .await;
    assert_eq!(out.status, 404);
    assert_eq!(out.headers[http::header::CONTENT_TYPE], "application/cbor");
    assert_eq!(out.headers["x-amzn-query-error"], "ResourceNotFound;Sender");
}

#[tokio::test]
async fn query_put_metric_alarm_and_describe_alarms() {
    let service = CloudWatchService::new();
    let out = service
        .handle(query_request(
            "Action=PutMetricAlarm&Version=2010-08-01&AlarmName=wb-alarm\
             &Namespace=AWS%2FSQS&MetricName=ApproximateNumberOfMessagesVisible\
             &Dimensions.member.1.Name=QueueName&Dimensions.member.1.Value=123\
             &Statistic=Maximum&Period=300&EvaluationPeriods=5&DatapointsToAlarm=3\
             &Threshold=0.5&ComparisonOperator=GreaterThanThreshold\
             &TreatMissingData=notBreaching&ActionsEnabled=false",
        ))
        .await;
    xml_body(&out, 200);

    let out = service
        .handle(query_request(
            "Action=DescribeAlarms&Version=2010-08-01&AlarmNames.member.1=wb-alarm",
        ))
        .await;
    let xml = xml_body(&out, 200);
    for fragment in [
        "<TreatMissingData>notBreaching</TreatMissingData>",
        "<DatapointsToAlarm>3</DatapointsToAlarm>",
        "<StateValue>INSUFFICIENT_DATA</StateValue>",
        // Numbers, booleans and numeric-looking strings keep their values.
        "<Period>300</Period>",
        "<EvaluationPeriods>5</EvaluationPeriods>",
        "<Threshold>0.5</Threshold>",
        "<ActionsEnabled>false</ActionsEnabled>",
        "<Value>123</Value>",
    ] {
        assert!(xml.contains(fragment), "missing {fragment} in {xml}");
    }
    // awsQuery timestamps are ISO 8601 strings.
    let start = xml.find("<StateUpdatedTimestamp>").unwrap() + 23;
    let ts = &xml[start..start + 24];
    assert!(
        ts.parse::<chrono::DateTime<chrono::Utc>>().is_ok(),
        "StateUpdatedTimestamp {ts} is not RFC 3339"
    );
}

#[tokio::test]
async fn query_error_is_xml() {
    let service = CloudWatchService::new();
    let out = service
        .handle(query_request(
            "Action=SetAlarmState&Version=2010-08-01&AlarmName=missing&StateValue=OK&StateReason=x",
        ))
        .await;
    let xml = xml_body(&out, 404);
    assert!(xml.contains("<Code>ResourceNotFound</Code>"), "{xml}");
}

#[tokio::test]
async fn protocols_share_state() {
    let service = CloudWatchService::new();
    let out = service
        .handle(json_request("PutMetricAlarm", put_alarm_json()))
        .await;
    json_body(&out, 200);

    // The CBOR answer tags the same timestamp with CBOR Tag 1.
    let out = service
        .handle(cbor_request(
            "DescribeAlarms",
            json!({"AlarmNames": ["wb-alarm"]}),
        ))
        .await;
    assert_eq!(out.status, 200);
    let cbor: ciborium::Value = ciborium::from_reader(&out.body[..]).unwrap();
    let alarms = cbor_field(&cbor, "MetricAlarms").as_array().unwrap();
    let alarm = &alarms[0];
    assert_eq!(
        cbor_field(alarm, "TreatMissingData").as_text(),
        Some("notBreaching")
    );
    assert!(matches!(
        cbor_field(alarm, "StateUpdatedTimestamp"),
        ciborium::Value::Tag(1, _)
    ));

    let out = service
        .handle(query_request(
            "Action=DescribeAlarms&Version=2010-08-01&StateValue=INSUFFICIENT_DATA",
        ))
        .await;
    assert!(xml_body(&out, 200).contains("<AlarmName>wb-alarm</AlarmName>"));
}

fn cbor_field<'a>(value: &'a ciborium::Value, name: &str) -> &'a ciborium::Value {
    &value
        .as_map()
        .unwrap()
        .iter()
        .find(|(key, _)| key.as_text() == Some(name))
        .unwrap_or_else(|| panic!("no {name}"))
        .1
}

/// Every metric alarm over `awsJson1_0`, sorted by name.
async fn describe_all_sorted(service: &CloudWatchService) -> Vec<Value> {
    let out = service
        .handle(json_request("DescribeAlarms", json!({})))
        .await;
    let mut alarms = json_body(&out, 200)["MetricAlarms"]
        .as_array()
        .unwrap()
        .clone();
    alarms.sort_by(|a, b| a["AlarmName"].as_str().cmp(&b["AlarmName"].as_str()));
    alarms
}

#[tokio::test]
async fn alarm_settings_survive_snapshot_and_restore() {
    let service = CloudWatchService::new();
    let mut alarm = put_alarm_json();
    alarm["ExtendedStatistic"] = json!("p90");
    alarm.as_object_mut().unwrap().remove("Statistic");
    alarm["EvaluateLowSampleCountPercentile"] = json!("ignore");
    json_body(
        &service.handle(json_request("PutMetricAlarm", alarm)).await,
        200,
    );
    json_body(
        &service
            .handle(json_request(
                "PutMetricAlarm",
                json!({
                    "AlarmName": "math-alarm",
                    "EvaluationPeriods": 1,
                    "Threshold": 10,
                    "ComparisonOperator": "GreaterThanThreshold",
                    "Metrics": [
                        {"Id": "e1", "Expression": "m1 * 2", "ReturnData": true},
                        {"Id": "m1", "ReturnData": false, "MetricStat": {
                            "Metric": {"Namespace": "NS", "MetricName": "M"},
                            "Period": 60, "Stat": "Sum"}},
                    ],
                }),
            ))
            .await,
        200,
    );
    json_body(
        &service
            .handle(json_request(
                "SetAlarmState",
                json!({"AlarmName": "wb-alarm", "StateValue": "ALARM",
                       "StateReason": "r", "StateReasonData": "{\"a\":1}"}),
            ))
            .await,
        200,
    );
    let before = describe_all_sorted(&service).await;

    // Round-trip through the serialised form a persistent backend would store.
    let snapshot = service.snapshot(ACCOUNT, REGION).await;
    let stored = serde_json::to_string(&snapshot).unwrap();
    let restored = CloudWatchService::new();
    restored
        .restore(ACCOUNT, REGION, serde_json::from_str(&stored).unwrap())
        .await
        .unwrap();
    let after = describe_all_sorted(&restored).await;
    assert_eq!(before, after);

    let wb = &after[1];
    assert_eq!(wb["ExtendedStatistic"], "p90");
    assert_eq!(wb["EvaluateLowSampleCountPercentile"], "ignore");
    assert_eq!(wb["StateReasonData"], "{\"a\":1}");
    assert_eq!(wb["StateValue"], "ALARM");
    let math = &after[0];
    assert_eq!(math["Metrics"][0]["Expression"], "m1 * 2");
    assert_eq!(math["Metrics"][1]["MetricStat"]["Stat"], "Sum");
    assert!(math.get("MetricName").is_none());
}

#[tokio::test]
async fn snapshot_from_before_issue_68_still_loads() {
    // The alarm view as it was serialised before the new members existed.
    let old = json!({
        "alarms": {
            "legacy": {
                "alarm_name": "legacy",
                "alarm_arn": "arn:aws:cloudwatch:us-east-1:123456789012:alarm:legacy",
                "metric_name": "CPUUtilization",
                "namespace": "AWS/EC2",
                "threshold": 80.0,
                "comparison_operator": "GreaterThanThreshold",
                "evaluation_periods": 2,
                "period": 300,
                "statistic": "Average",
                "state_value": "OK",
                "state_reason": "Unchecked: Initial alarm creation",
                "actions_enabled": true,
                "alarm_description": null,
                "alarm_actions": [],
                "ok_actions": [],
                "insufficient_data_actions": [],
                "dimensions": [],
                "unit": null
            }
        }
    });
    let view: CloudwatchStateView = serde_json::from_value(old).unwrap();
    let service = CloudWatchService::new();
    service.restore(ACCOUNT, REGION, view).await.unwrap();

    let body = json_body(
        &service
            .handle(json_request("DescribeAlarms", json!({})))
            .await,
        200,
    );
    let alarm = &body["MetricAlarms"][0];
    assert_eq!(alarm["AlarmName"], "legacy");
    assert_eq!(alarm["Threshold"], 80.0);
    assert_eq!(alarm["Period"], 300);
    assert_eq!(alarm["StateValue"], "OK");
    assert!(alarm.get("TreatMissingData").is_none());
    assert_epoch_number(alarm, "StateUpdatedTimestamp");
}
