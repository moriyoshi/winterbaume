//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloudwatch

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;
/// Convert `serde_json::Value` to `ciborium::Value` for CBOR encoding.
///
/// Recognises the `{"__cbor_epoch_seconds": N}` sentinel and encodes it as
/// CBOR Tag 1 (epoch-based date/time) per the rpc-v2-cbor protocol.
pub fn json_to_cbor(json: &serde_json::Value) -> ciborium::Value {
    match json {
        serde_json::Value::Null => ciborium::Value::Null,
        serde_json::Value::Bool(b) => ciborium::Value::Bool(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                ciborium::Value::Integer(i.into())
            } else if let Some(u) = n.as_u64() {
                ciborium::Value::Integer(u.into())
            } else if let Some(f) = n.as_f64() {
                ciborium::Value::Float(f)
            } else {
                ciborium::Value::Null
            }
        }
        serde_json::Value::String(s) => ciborium::Value::Text(s.clone()),
        serde_json::Value::Array(arr) => {
            ciborium::Value::Array(arr.iter().map(json_to_cbor).collect())
        }
        serde_json::Value::Object(obj) => {
            // Detect the cbor_timestamp sentinel.
            if obj.len() == 1 {
                if let Some(serde_json::Value::Number(n)) = obj.get("__cbor_epoch_seconds") {
                    let inner = if let Some(i) = n.as_i64() {
                        ciborium::Value::Integer(i.into())
                    } else if let Some(f) = n.as_f64() {
                        ciborium::Value::Float(f)
                    } else {
                        ciborium::Value::Null
                    };
                    return ciborium::Value::Tag(1, Box::new(inner));
                }
            }
            let map: Vec<(ciborium::Value, ciborium::Value)> = obj
                .iter()
                .map(|(k, v)| (ciborium::Value::Text(k.clone()), json_to_cbor(v)))
                .collect();
            ciborium::Value::Map(map)
        }
    }
}

/// Convert `ciborium::Value` to `serde_json::Value` for request decoding.
pub fn cbor_to_json(cbor: ciborium::Value) -> serde_json::Value {
    match cbor {
        ciborium::Value::Null => serde_json::Value::Null,
        ciborium::Value::Bool(b) => serde_json::Value::Bool(b),
        ciborium::Value::Integer(i) => {
            let n: i128 = i.into();
            if let Ok(v) = i64::try_from(n) {
                serde_json::json!(v)
            } else {
                serde_json::json!(n as f64)
            }
        }
        ciborium::Value::Float(f) => serde_json::json!(f),
        ciborium::Value::Text(s) => serde_json::Value::String(s),
        ciborium::Value::Bytes(b) => {
            use base64::Engine;
            serde_json::Value::String(base64::engine::general_purpose::STANDARD.encode(b))
        }
        ciborium::Value::Array(arr) => {
            serde_json::Value::Array(arr.into_iter().map(cbor_to_json).collect())
        }
        ciborium::Value::Map(map) => {
            let obj = map
                .into_iter()
                .filter_map(|(k, v)| {
                    let key = match k {
                        ciborium::Value::Text(s) => s,
                        _ => return None,
                    };
                    Some((key, cbor_to_json(v)))
                })
                .collect();
            serde_json::Value::Object(obj)
        }
        ciborium::Value::Tag(_, inner) => cbor_to_json(*inner),
        _ => serde_json::Value::Null,
    }
}

/// Build a CBOR-encoded error response.
pub fn cbor_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    let body = serde_json::json!({
        "__type": code,
        "message": message,
    });
    let cbor_val = json_to_cbor(&body);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(status, buf)
}

fn strip_outer_element(xml: &str) -> &str {
    // Find the end of the opening tag
    if let Some(close_pos) = xml.find('>') {
        let inner_start = close_pos + 1;
        // Find the last closing tag
        if let Some(last_open) = xml.rfind('<') {
            if last_open >= inner_start {
                return &xml[inner_start..last_open];
            }
        }
    }
    xml
}

/// Serialize void response for rpc-v2-cbor protocol.
pub fn serialize_delete_alarm_mute_rule_response() -> MockResponse {
    let mut buf = Vec::new();
    ciborium::into_writer(&ciborium::Value::Map(vec![]), &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize void response for rpc-v2-cbor protocol.
pub fn serialize_delete_alarms_response() -> MockResponse {
    let mut buf = Vec::new();
    ciborium::into_writer(&ciborium::Value::Map(vec![]), &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_delete_anomaly_detector_response(
    result: &DeleteAnomalyDetectorOutput,
) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_delete_dashboards_response(result: &DeleteDashboardsOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_delete_insight_rules_response(result: &DeleteInsightRulesOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_delete_metric_stream_response(result: &DeleteMetricStreamOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_describe_alarm_contributors_response(
    result: &DescribeAlarmContributorsOutput,
) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_describe_alarm_history_response(
    result: &DescribeAlarmHistoryOutput,
) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_describe_alarms_response(result: &DescribeAlarmsOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_describe_alarms_for_metric_response(
    result: &DescribeAlarmsForMetricOutput,
) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_describe_anomaly_detectors_response(
    result: &DescribeAnomalyDetectorsOutput,
) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_describe_insight_rules_response(
    result: &DescribeInsightRulesOutput,
) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize void response for rpc-v2-cbor protocol.
pub fn serialize_disable_alarm_actions_response() -> MockResponse {
    let mut buf = Vec::new();
    ciborium::into_writer(&ciborium::Value::Map(vec![]), &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_disable_insight_rules_response(
    result: &DisableInsightRulesOutput,
) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize void response for rpc-v2-cbor protocol.
pub fn serialize_enable_alarm_actions_response() -> MockResponse {
    let mut buf = Vec::new();
    ciborium::into_writer(&ciborium::Value::Map(vec![]), &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_enable_insight_rules_response(result: &EnableInsightRulesOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_get_alarm_mute_rule_response(result: &GetAlarmMuteRuleOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_get_dashboard_response(result: &GetDashboardOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_get_insight_rule_report_response(
    result: &GetInsightRuleReportOutput,
) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_get_metric_data_response(result: &GetMetricDataOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_get_metric_statistics_response(
    result: &GetMetricStatisticsOutput,
) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_get_metric_stream_response(result: &GetMetricStreamOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_get_metric_widget_image_response(
    result: &GetMetricWidgetImageOutput,
) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_list_alarm_mute_rules_response(result: &ListAlarmMuteRulesOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_list_dashboards_response(result: &ListDashboardsOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_list_managed_insight_rules_response(
    result: &ListManagedInsightRulesOutput,
) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_list_metric_streams_response(result: &ListMetricStreamsOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_list_metrics_response(result: &ListMetricsOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceOutput,
) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize void response for rpc-v2-cbor protocol.
pub fn serialize_put_alarm_mute_rule_response() -> MockResponse {
    let mut buf = Vec::new();
    ciborium::into_writer(&ciborium::Value::Map(vec![]), &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_put_anomaly_detector_response(result: &PutAnomalyDetectorOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize void response for rpc-v2-cbor protocol.
pub fn serialize_put_composite_alarm_response() -> MockResponse {
    let mut buf = Vec::new();
    ciborium::into_writer(&ciborium::Value::Map(vec![]), &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_put_dashboard_response(result: &PutDashboardOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_put_insight_rule_response(result: &PutInsightRuleOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_put_managed_insight_rules_response(
    result: &PutManagedInsightRulesOutput,
) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize void response for rpc-v2-cbor protocol.
pub fn serialize_put_metric_alarm_response() -> MockResponse {
    let mut buf = Vec::new();
    ciborium::into_writer(&ciborium::Value::Map(vec![]), &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize void response for rpc-v2-cbor protocol.
pub fn serialize_put_metric_data_response() -> MockResponse {
    let mut buf = Vec::new();
    ciborium::into_writer(&ciborium::Value::Map(vec![]), &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_put_metric_stream_response(result: &PutMetricStreamOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize void response for rpc-v2-cbor protocol.
pub fn serialize_set_alarm_state_response() -> MockResponse {
    let mut buf = Vec::new();
    ciborium::into_writer(&ciborium::Value::Map(vec![]), &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_start_metric_streams_response(result: &StartMetricStreamsOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_stop_metric_streams_response(result: &StopMetricStreamsOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_tag_resource_response(result: &TagResourceOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize response for rpc-v2-cbor protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceOutput) -> MockResponse {
    let json_val =
        serde_json::to_value(result).unwrap_or(serde_json::Value::Object(Default::default()));
    let cbor_val = json_to_cbor(&json_val);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(200, buf)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_alarm_mute_rule_response_json() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_alarms_response_json() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_anomaly_detector_response_json(
    result: &DeleteAnomalyDetectorOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_dashboards_response_json(result: &DeleteDashboardsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_insight_rules_response_json(
    result: &DeleteInsightRulesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_metric_stream_response_json(
    result: &DeleteMetricStreamOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_alarm_contributors_response_json(
    result: &DescribeAlarmContributorsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_alarm_history_response_json(
    result: &DescribeAlarmHistoryOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_alarms_response_json(result: &DescribeAlarmsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_alarms_for_metric_response_json(
    result: &DescribeAlarmsForMetricOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_anomaly_detectors_response_json(
    result: &DescribeAnomalyDetectorsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_insight_rules_response_json(
    result: &DescribeInsightRulesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_disable_alarm_actions_response_json() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_disable_insight_rules_response_json(
    result: &DisableInsightRulesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_enable_alarm_actions_response_json() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_enable_insight_rules_response_json(
    result: &EnableInsightRulesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_alarm_mute_rule_response_json(
    result: &GetAlarmMuteRuleOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_dashboard_response_json(result: &GetDashboardOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_insight_rule_report_response_json(
    result: &GetInsightRuleReportOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_metric_data_response_json(result: &GetMetricDataOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_metric_statistics_response_json(
    result: &GetMetricStatisticsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_metric_stream_response_json(result: &GetMetricStreamOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_metric_widget_image_response_json(
    result: &GetMetricWidgetImageOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_alarm_mute_rules_response_json(
    result: &ListAlarmMuteRulesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_dashboards_response_json(result: &ListDashboardsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_managed_insight_rules_response_json(
    result: &ListManagedInsightRulesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_metric_streams_response_json(
    result: &ListMetricStreamsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_metrics_response_json(result: &ListMetricsOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_resource_response_json(
    result: &ListTagsForResourceOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_alarm_mute_rule_response_json() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_anomaly_detector_response_json(
    result: &PutAnomalyDetectorOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_composite_alarm_response_json() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_dashboard_response_json(result: &PutDashboardOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_insight_rule_response_json(result: &PutInsightRuleOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_managed_insight_rules_response_json(
    result: &PutManagedInsightRulesOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_metric_alarm_response_json() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_put_metric_data_response_json() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_put_metric_stream_response_json(result: &PutMetricStreamOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_set_alarm_state_response_json() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_start_metric_streams_response_json(
    result: &StartMetricStreamsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_stop_metric_streams_response_json(
    result: &StopMetricStreamsOutput,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_tag_resource_response_json(result: &TagResourceOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_untag_resource_response_json(result: &UntagResourceOutput) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_alarm_mute_rule_response_query() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteAlarmMuteRuleResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteAlarmMuteRuleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_delete_alarms_response_query() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteAlarmsResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteAlarmsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_anomaly_detector_response_query(
    result: &DeleteAnomalyDetectorOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DeleteAnomalyDetectorResult>{inner_xml}</DeleteAnomalyDetectorResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteAnomalyDetectorResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteAnomalyDetectorResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_dashboards_response_query(result: &DeleteDashboardsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteDashboardsResult>{inner_xml}</DeleteDashboardsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteDashboardsResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteDashboardsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_insight_rules_response_query(
    result: &DeleteInsightRulesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteInsightRulesResult>{inner_xml}</DeleteInsightRulesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteInsightRulesResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteInsightRulesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_delete_metric_stream_response_query(
    result: &DeleteMetricStreamOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DeleteMetricStreamResult>{inner_xml}</DeleteMetricStreamResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DeleteMetricStreamResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DeleteMetricStreamResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_alarm_contributors_response_query(
    result: &DescribeAlarmContributorsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeAlarmContributorsResult>{inner_xml}</DescribeAlarmContributorsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeAlarmContributorsResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeAlarmContributorsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_alarm_history_response_query(
    result: &DescribeAlarmHistoryOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeAlarmHistoryResult>{inner_xml}</DescribeAlarmHistoryResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeAlarmHistoryResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeAlarmHistoryResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_alarms_response_query(result: &DescribeAlarmsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DescribeAlarmsResult>{inner_xml}</DescribeAlarmsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeAlarmsResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeAlarmsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_alarms_for_metric_response_query(
    result: &DescribeAlarmsForMetricOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeAlarmsForMetricResult>{inner_xml}</DescribeAlarmsForMetricResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeAlarmsForMetricResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeAlarmsForMetricResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_anomaly_detectors_response_query(
    result: &DescribeAnomalyDetectorsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeAnomalyDetectorsResult>{inner_xml}</DescribeAnomalyDetectorsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeAnomalyDetectorsResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeAnomalyDetectorsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_describe_insight_rules_response_query(
    result: &DescribeInsightRulesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<DescribeInsightRulesResult>{inner_xml}</DescribeInsightRulesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DescribeInsightRulesResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DescribeInsightRulesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_disable_alarm_actions_response_query() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DisableAlarmActionsResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DisableAlarmActionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_disable_insight_rules_response_query(
    result: &DisableInsightRulesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<DisableInsightRulesResult>{inner_xml}</DisableInsightRulesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<DisableInsightRulesResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</DisableInsightRulesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_enable_alarm_actions_response_query() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<EnableAlarmActionsResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</EnableAlarmActionsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_enable_insight_rules_response_query(
    result: &EnableInsightRulesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<EnableInsightRulesResult>{inner_xml}</EnableInsightRulesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<EnableInsightRulesResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</EnableInsightRulesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_alarm_mute_rule_response_query(
    result: &GetAlarmMuteRuleOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetAlarmMuteRuleResult>{inner_xml}</GetAlarmMuteRuleResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetAlarmMuteRuleResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetAlarmMuteRuleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_dashboard_response_query(result: &GetDashboardOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetDashboardResult>{inner_xml}</GetDashboardResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetDashboardResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetDashboardResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_insight_rule_report_response_query(
    result: &GetInsightRuleReportOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<GetInsightRuleReportResult>{inner_xml}</GetInsightRuleReportResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetInsightRuleReportResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetInsightRuleReportResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_metric_data_response_query(result: &GetMetricDataOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetMetricDataResult>{inner_xml}</GetMetricDataResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetMetricDataResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetMetricDataResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_metric_statistics_response_query(
    result: &GetMetricStatisticsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetMetricStatisticsResult>{inner_xml}</GetMetricStatisticsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetMetricStatisticsResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetMetricStatisticsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_metric_stream_response_query(result: &GetMetricStreamOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<GetMetricStreamResult>{inner_xml}</GetMetricStreamResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetMetricStreamResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetMetricStreamResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_get_metric_widget_image_response_query(
    result: &GetMetricWidgetImageOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<GetMetricWidgetImageResult>{inner_xml}</GetMetricWidgetImageResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<GetMetricWidgetImageResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</GetMetricWidgetImageResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_alarm_mute_rules_response_query(
    result: &ListAlarmMuteRulesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListAlarmMuteRulesResult>{inner_xml}</ListAlarmMuteRulesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListAlarmMuteRulesResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListAlarmMuteRulesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_dashboards_response_query(result: &ListDashboardsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListDashboardsResult>{inner_xml}</ListDashboardsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListDashboardsResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListDashboardsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_managed_insight_rules_response_query(
    result: &ListManagedInsightRulesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<ListManagedInsightRulesResult>{inner_xml}</ListManagedInsightRulesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListManagedInsightRulesResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListManagedInsightRulesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_metric_streams_response_query(
    result: &ListMetricStreamsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListMetricStreamsResult>{inner_xml}</ListMetricStreamsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListMetricStreamsResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListMetricStreamsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_metrics_response_query(result: &ListMetricsOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListMetricsResult>{inner_xml}</ListMetricsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListMetricsResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListMetricsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_list_tags_for_resource_response_query(
    result: &ListTagsForResourceOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<ListTagsForResourceResult>{inner_xml}</ListTagsForResourceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<ListTagsForResourceResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</ListTagsForResourceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_put_alarm_mute_rule_response_query() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutAlarmMuteRuleResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutAlarmMuteRuleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_put_anomaly_detector_response_query(
    result: &PutAnomalyDetectorOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<PutAnomalyDetectorResult>{inner_xml}</PutAnomalyDetectorResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutAnomalyDetectorResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutAnomalyDetectorResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_put_composite_alarm_response_query() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutCompositeAlarmResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutCompositeAlarmResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_put_dashboard_response_query(result: &PutDashboardOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<PutDashboardResult>{inner_xml}</PutDashboardResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutDashboardResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutDashboardResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_put_insight_rule_response_query(result: &PutInsightRuleOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<PutInsightRuleResult>{inner_xml}</PutInsightRuleResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutInsightRuleResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutInsightRuleResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_put_managed_insight_rules_response_query(
    result: &PutManagedInsightRulesOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml =
        format!("<PutManagedInsightRulesResult>{inner_xml}</PutManagedInsightRulesResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutManagedInsightRulesResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutManagedInsightRulesResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_put_metric_alarm_response_query() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutMetricAlarmResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutMetricAlarmResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_put_metric_data_response_query() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutMetricDataResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutMetricDataResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_put_metric_stream_response_query(result: &PutMetricStreamOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<PutMetricStreamResult>{inner_xml}</PutMetricStreamResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<PutMetricStreamResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</PutMetricStreamResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize void response for awsQuery protocol.
pub fn serialize_set_alarm_state_response_query() -> MockResponse {
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<SetAlarmStateResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</SetAlarmStateResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_start_metric_streams_response_query(
    result: &StartMetricStreamsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<StartMetricStreamsResult>{inner_xml}</StartMetricStreamsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<StartMetricStreamsResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</StartMetricStreamsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_stop_metric_streams_response_query(
    result: &StopMetricStreamsOutput,
) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<StopMetricStreamsResult>{inner_xml}</StopMetricStreamsResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<StopMetricStreamsResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</StopMetricStreamsResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_tag_resource_response_query(result: &TagResourceOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<TagResourceResult>{inner_xml}</TagResourceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<TagResourceResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</TagResourceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Serialize response for awsQuery protocol.
pub fn serialize_untag_resource_response_query(result: &UntagResourceOutput) -> MockResponse {
    let result_xml_wrapped = quick_xml::se::to_string(result).unwrap_or_default();
    // Strip the outer wrapper (serde rename) and re-wrap with the correct op Result name.
    let inner_xml = strip_outer_element(&result_xml_wrapped);
    let result_xml = format!("<UntagResourceResult>{inner_xml}</UntagResourceResult>");
    let request_id = uuid::Uuid::new_v4();
    let xml = format!(
        r#"<UntagResourceResponse xmlns="http://monitoring.amazonaws.com/doc/2010-08-01/">
  {result_xml}
  <ResponseMetadata>
    <RequestId>{request_id}</RequestId>
  </ResponseMetadata>
</UntagResourceResponse>"#
    );
    MockResponse::xml(200, xml)
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_delete_alarm_mute_rule_request(
    body: &[u8],
) -> Result<DeleteAlarmMuteRuleInput, String> {
    if body.is_empty() {
        return Ok(DeleteAlarmMuteRuleInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize DeleteAlarmMuteRule request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_delete_alarms_request(body: &[u8]) -> Result<DeleteAlarmsInput, String> {
    if body.is_empty() {
        return Ok(DeleteAlarmsInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize DeleteAlarms request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_delete_anomaly_detector_request(
    body: &[u8],
) -> Result<DeleteAnomalyDetectorInput, String> {
    if body.is_empty() {
        return Ok(DeleteAnomalyDetectorInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize DeleteAnomalyDetector request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_delete_dashboards_request(body: &[u8]) -> Result<DeleteDashboardsInput, String> {
    if body.is_empty() {
        return Ok(DeleteDashboardsInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize DeleteDashboards request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_delete_insight_rules_request(
    body: &[u8],
) -> Result<DeleteInsightRulesInput, String> {
    if body.is_empty() {
        return Ok(DeleteInsightRulesInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize DeleteInsightRules request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_delete_metric_stream_request(
    body: &[u8],
) -> Result<DeleteMetricStreamInput, String> {
    if body.is_empty() {
        return Ok(DeleteMetricStreamInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize DeleteMetricStream request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_describe_alarm_contributors_request(
    body: &[u8],
) -> Result<DescribeAlarmContributorsInput, String> {
    if body.is_empty() {
        return Ok(DescribeAlarmContributorsInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize DescribeAlarmContributors request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_describe_alarm_history_request(
    body: &[u8],
) -> Result<DescribeAlarmHistoryInput, String> {
    if body.is_empty() {
        return Ok(DescribeAlarmHistoryInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize DescribeAlarmHistory request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_describe_alarms_request(body: &[u8]) -> Result<DescribeAlarmsInput, String> {
    if body.is_empty() {
        return Ok(DescribeAlarmsInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize DescribeAlarms request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_describe_alarms_for_metric_request(
    body: &[u8],
) -> Result<DescribeAlarmsForMetricInput, String> {
    if body.is_empty() {
        return Ok(DescribeAlarmsForMetricInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize DescribeAlarmsForMetric request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_describe_anomaly_detectors_request(
    body: &[u8],
) -> Result<DescribeAnomalyDetectorsInput, String> {
    if body.is_empty() {
        return Ok(DescribeAnomalyDetectorsInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize DescribeAnomalyDetectors request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_describe_insight_rules_request(
    body: &[u8],
) -> Result<DescribeInsightRulesInput, String> {
    if body.is_empty() {
        return Ok(DescribeInsightRulesInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize DescribeInsightRules request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_disable_alarm_actions_request(
    body: &[u8],
) -> Result<DisableAlarmActionsInput, String> {
    if body.is_empty() {
        return Ok(DisableAlarmActionsInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize DisableAlarmActions request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_disable_insight_rules_request(
    body: &[u8],
) -> Result<DisableInsightRulesInput, String> {
    if body.is_empty() {
        return Ok(DisableInsightRulesInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize DisableInsightRules request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_enable_alarm_actions_request(
    body: &[u8],
) -> Result<EnableAlarmActionsInput, String> {
    if body.is_empty() {
        return Ok(EnableAlarmActionsInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize EnableAlarmActions request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_enable_insight_rules_request(
    body: &[u8],
) -> Result<EnableInsightRulesInput, String> {
    if body.is_empty() {
        return Ok(EnableInsightRulesInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize EnableInsightRules request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_get_alarm_mute_rule_request(
    body: &[u8],
) -> Result<GetAlarmMuteRuleInput, String> {
    if body.is_empty() {
        return Ok(GetAlarmMuteRuleInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize GetAlarmMuteRule request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_get_dashboard_request(body: &[u8]) -> Result<GetDashboardInput, String> {
    if body.is_empty() {
        return Ok(GetDashboardInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize GetDashboard request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_get_insight_rule_report_request(
    body: &[u8],
) -> Result<GetInsightRuleReportInput, String> {
    if body.is_empty() {
        return Ok(GetInsightRuleReportInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize GetInsightRuleReport request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_get_metric_data_request(body: &[u8]) -> Result<GetMetricDataInput, String> {
    if body.is_empty() {
        return Ok(GetMetricDataInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize GetMetricData request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_get_metric_statistics_request(
    body: &[u8],
) -> Result<GetMetricStatisticsInput, String> {
    if body.is_empty() {
        return Ok(GetMetricStatisticsInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize GetMetricStatistics request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_get_metric_stream_request(body: &[u8]) -> Result<GetMetricStreamInput, String> {
    if body.is_empty() {
        return Ok(GetMetricStreamInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize GetMetricStream request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_get_metric_widget_image_request(
    body: &[u8],
) -> Result<GetMetricWidgetImageInput, String> {
    if body.is_empty() {
        return Ok(GetMetricWidgetImageInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize GetMetricWidgetImage request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_list_alarm_mute_rules_request(
    body: &[u8],
) -> Result<ListAlarmMuteRulesInput, String> {
    if body.is_empty() {
        return Ok(ListAlarmMuteRulesInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize ListAlarmMuteRules request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_list_dashboards_request(body: &[u8]) -> Result<ListDashboardsInput, String> {
    if body.is_empty() {
        return Ok(ListDashboardsInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize ListDashboards request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_list_managed_insight_rules_request(
    body: &[u8],
) -> Result<ListManagedInsightRulesInput, String> {
    if body.is_empty() {
        return Ok(ListManagedInsightRulesInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize ListManagedInsightRules request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_list_metric_streams_request(
    body: &[u8],
) -> Result<ListMetricStreamsInput, String> {
    if body.is_empty() {
        return Ok(ListMetricStreamsInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize ListMetricStreams request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_list_metrics_request(body: &[u8]) -> Result<ListMetricsInput, String> {
    if body.is_empty() {
        return Ok(ListMetricsInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize ListMetrics request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_list_tags_for_resource_request(
    body: &[u8],
) -> Result<ListTagsForResourceInput, String> {
    if body.is_empty() {
        return Ok(ListTagsForResourceInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize ListTagsForResource request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_put_alarm_mute_rule_request(
    body: &[u8],
) -> Result<PutAlarmMuteRuleInput, String> {
    if body.is_empty() {
        return Ok(PutAlarmMuteRuleInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize PutAlarmMuteRule request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_put_anomaly_detector_request(
    body: &[u8],
) -> Result<PutAnomalyDetectorInput, String> {
    if body.is_empty() {
        return Ok(PutAnomalyDetectorInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize PutAnomalyDetector request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_put_composite_alarm_request(
    body: &[u8],
) -> Result<PutCompositeAlarmInput, String> {
    if body.is_empty() {
        return Ok(PutCompositeAlarmInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize PutCompositeAlarm request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_put_dashboard_request(body: &[u8]) -> Result<PutDashboardInput, String> {
    if body.is_empty() {
        return Ok(PutDashboardInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize PutDashboard request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_put_insight_rule_request(body: &[u8]) -> Result<PutInsightRuleInput, String> {
    if body.is_empty() {
        return Ok(PutInsightRuleInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize PutInsightRule request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_put_managed_insight_rules_request(
    body: &[u8],
) -> Result<PutManagedInsightRulesInput, String> {
    if body.is_empty() {
        return Ok(PutManagedInsightRulesInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize PutManagedInsightRules request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_put_metric_alarm_request(body: &[u8]) -> Result<PutMetricAlarmInput, String> {
    if body.is_empty() {
        return Ok(PutMetricAlarmInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize PutMetricAlarm request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_put_metric_data_request(body: &[u8]) -> Result<PutMetricDataInput, String> {
    if body.is_empty() {
        return Ok(PutMetricDataInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize PutMetricData request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_put_metric_stream_request(body: &[u8]) -> Result<PutMetricStreamInput, String> {
    if body.is_empty() {
        return Ok(PutMetricStreamInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize PutMetricStream request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_set_alarm_state_request(body: &[u8]) -> Result<SetAlarmStateInput, String> {
    if body.is_empty() {
        return Ok(SetAlarmStateInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize SetAlarmState request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_start_metric_streams_request(
    body: &[u8],
) -> Result<StartMetricStreamsInput, String> {
    if body.is_empty() {
        return Ok(StartMetricStreamsInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize StartMetricStreams request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_stop_metric_streams_request(
    body: &[u8],
) -> Result<StopMetricStreamsInput, String> {
    if body.is_empty() {
        return Ok(StopMetricStreamsInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize StopMetricStreams request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_tag_resource_request(body: &[u8]) -> Result<TagResourceInput, String> {
    if body.is_empty() {
        return Ok(TagResourceInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize TagResource request: {e}"))
}

/// Deserialize request for rpc-v2-cbor protocol.
pub fn deserialize_untag_resource_request(body: &[u8]) -> Result<UntagResourceInput, String> {
    if body.is_empty() {
        return Ok(UntagResourceInput::default());
    }
    let cbor_val: ciborium::Value =
        ciborium::from_reader(body).map_err(|e| format!("CBOR decode error: {e}"))?;
    let json_val = cbor_to_json(cbor_val);
    serde_json::from_value(json_val)
        .map_err(|e| format!("Failed to deserialize UntagResource request: {e}"))
}

fn deserialize_entity_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Entity>, String> {
    let mut item = Entity::default();
    let mut found = false;
    Ok(if found { Some(item) } else { None })
}

fn deserialize_metric_stream_filter_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MetricStreamFilter>, String> {
    let mut item = MetricStreamFilter::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.MetricNames");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.metric_names = Some(sub_items);
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.Namespace")) {
        item.namespace = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_mute_targets_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MuteTargets>, String> {
    let mut item = MuteTargets::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.AlarmNames");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.alarm_names = sub_items;
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_range_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Range>, String> {
    let mut item = Range::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.EndTime")) {
        item.end_time = value
            .parse::<f64>()
            .map_err(|e| format!("failed to parse EndTime: {e}"))?;
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.StartTime")) {
        item.start_time = value
            .parse::<f64>()
            .map_err(|e| format!("failed to parse StartTime: {e}"))?;
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_metric_characteristics_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MetricCharacteristics>, String> {
    let mut item = MetricCharacteristics::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.PeriodicSpikes")) {
        item.periodic_spikes = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse PeriodicSpikes: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_metric_stream_statistics_configuration_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MetricStreamStatisticsConfiguration>, String> {
    let mut item = MetricStreamStatisticsConfiguration::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.AdditionalStatistics");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(value.to_string()),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.additional_statistics = sub_items;
            found = true;
        }
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.IncludeMetrics");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_metric_stream_statistics_metric_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.include_metrics = sub_items;
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_label_options_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<LabelOptions>, String> {
    let mut item = LabelOptions::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Timezone")) {
        item.timezone = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_dimension_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Dimension>, String> {
    let mut item = Dimension::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Name")) {
        item.name = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Value")) {
        item.value = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_metric_datum_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MetricDatum>, String> {
    let mut item = MetricDatum::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Counts");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(
                    value
                        .parse::<f64>()
                        .map_err(|e| format!("failed to parse list item: {e}"))?,
                ),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.counts = Some(sub_items);
            found = true;
        }
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Dimensions");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_dimension_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.dimensions = Some(sub_items);
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.MetricName")) {
        item.metric_name = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.StorageResolution")) {
        item.storage_resolution = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse StorageResolution: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Timestamp")) {
        item.timestamp = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse Timestamp: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Unit")) {
        item.unit = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Value")) {
        item.value = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse Value: {e}"))?,
        );
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Values");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => sub_items.push(
                    value
                        .parse::<f64>()
                        .map_err(|e| format!("failed to parse list item: {e}"))?,
                ),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.values = Some(sub_items);
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_metric_stream_statistics_metric_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MetricStreamStatisticsMetric>, String> {
    let mut item = MetricStreamStatisticsMetric::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.MetricName")) {
        item.metric_name = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Namespace")) {
        item.namespace = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_rule_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Rule>, String> {
    let mut item = Rule::default();
    let mut found = false;
    Ok(if found { Some(item) } else { None })
}

fn deserialize_anomaly_detector_configuration_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<AnomalyDetectorConfiguration>, String> {
    let mut item = AnomalyDetectorConfiguration::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.ExcludedTimeRanges");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_range_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.excluded_time_ranges = Some(sub_items);
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.MetricTimezone")) {
        item.metric_timezone = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_single_metric_anomaly_detector_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<SingleMetricAnomalyDetector>, String> {
    let mut item = SingleMetricAnomalyDetector::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.AccountId")) {
        item.account_id = Some(value.to_string());
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Dimensions");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_dimension_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.dimensions = Some(sub_items);
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.MetricName")) {
        item.metric_name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Namespace")) {
        item.namespace = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Stat")) {
        item.stat = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_schedule_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Schedule>, String> {
    let mut item = Schedule::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Duration")) {
        item.duration = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Expression")) {
        item.expression = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Timezone")) {
        item.timezone = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_tag_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Tag>, String> {
    let mut item = Tag::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Key")) {
        item.key = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Value")) {
        item.value = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_metric_stat_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MetricStat>, String> {
    let mut item = MetricStat::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Period")) {
        item.period = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse Period: {e}"))?;
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Stat")) {
        item.stat = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Unit")) {
        item.unit = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_metric_data_query_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MetricDataQuery>, String> {
    let mut item = MetricDataQuery::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.AccountId")) {
        item.account_id = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Expression")) {
        item.expression = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Id")) {
        item.id = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Label")) {
        item.label = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Period")) {
        item.period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Period: {e}"))?,
        );
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.ReturnData")) {
        item.return_data = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ReturnData: {e}"))?,
        );
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_metric_math_anomaly_detector_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<MetricMathAnomalyDetector>, String> {
    let mut item = MetricMathAnomalyDetector::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.MetricDataQueries");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_metric_data_query_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.metric_data_queries = Some(sub_items);
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_managed_rule_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<ManagedRule>, String> {
    let mut item = ManagedRule::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.ResourceARN")) {
        item.resource_a_r_n = value.to_string();
        found = true;
    }
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Tags");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.tags = Some(sub_items);
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.TemplateName")) {
        item.template_name = value.to_string();
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_entity_metric_data_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<EntityMetricData>, String> {
    let mut item = EntityMetricData::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.MetricData");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_metric_datum_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.metric_data = Some(sub_items);
            found = true;
        }
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_dimension_filter_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<DimensionFilter>, String> {
    let mut item = DimensionFilter::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Name")) {
        item.name = value.to_string();
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Value")) {
        item.value = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_statistic_set_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<StatisticSet>, String> {
    let mut item = StatisticSet::default();
    let mut found = false;
    if let Some(value) = params.get(&format!("{prefix}.Maximum")) {
        item.maximum = value
            .parse::<f64>()
            .map_err(|e| format!("failed to parse Maximum: {e}"))?;
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Minimum")) {
        item.minimum = value
            .parse::<f64>()
            .map_err(|e| format!("failed to parse Minimum: {e}"))?;
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.SampleCount")) {
        item.sample_count = value
            .parse::<f64>()
            .map_err(|e| format!("failed to parse SampleCount: {e}"))?;
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Sum")) {
        item.sum = value
            .parse::<f64>()
            .map_err(|e| format!("failed to parse Sum: {e}"))?;
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

fn deserialize_metric_from_query(
    params: &std::collections::HashMap<String, String>,
    prefix: &str,
) -> Result<Option<Metric>, String> {
    let mut item = Metric::default();
    let mut found = false;
    {
        let mut sub_items = Vec::new();
        let sub_list_prefix = format!("{prefix}.Dimensions");
        for i in 1.. {
            let item_key = format!("{sub_list_prefix}.member.{i}");
            match deserialize_dimension_from_query(params, &item_key)? {
                Some(sub_item) => sub_items.push(sub_item),
                None => break,
            }
        }
        if !sub_items.is_empty() {
            item.dimensions = Some(sub_items);
            found = true;
        }
    }
    if let Some(value) = params.get(&format!("{prefix}.MetricName")) {
        item.metric_name = Some(value.to_string());
        found = true;
    }
    if let Some(value) = params.get(&format!("{prefix}.Namespace")) {
        item.namespace = Some(value.to_string());
        found = true;
    }
    Ok(if found { Some(item) } else { None })
}

/// Deserialize awsQuery request for DeleteAlarmMuteRule.
pub fn deserialize_delete_alarm_mute_rule_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteAlarmMuteRuleInput, String> {
    let mut input = DeleteAlarmMuteRuleInput::default();
    if let Some(value) = params.get("AlarmMuteRuleName") {
        input.alarm_mute_rule_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteAlarms.
pub fn deserialize_delete_alarms_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteAlarmsInput, String> {
    let mut input = DeleteAlarmsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AlarmNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.alarm_names = items;
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteAnomalyDetector.
pub fn deserialize_delete_anomaly_detector_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteAnomalyDetectorInput, String> {
    let mut input = DeleteAnomalyDetectorInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Dimensions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_dimension_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.dimensions = Some(items);
        }
    }
    if let Some(val) =
        deserialize_metric_math_anomaly_detector_from_query(params, "MetricMathAnomalyDetector")?
    {
        input.metric_math_anomaly_detector = Some(val);
    }
    if let Some(value) = params.get("MetricName") {
        input.metric_name = Some(value.to_string());
    }
    if let Some(value) = params.get("Namespace") {
        input.namespace = Some(value.to_string());
    }
    if let Some(val) = deserialize_single_metric_anomaly_detector_from_query(
        params,
        "SingleMetricAnomalyDetector",
    )? {
        input.single_metric_anomaly_detector = Some(val);
    }
    if let Some(value) = params.get("Stat") {
        input.stat = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteDashboards.
pub fn deserialize_delete_dashboards_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteDashboardsInput, String> {
    let mut input = DeleteDashboardsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "DashboardNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.dashboard_names = items;
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteInsightRules.
pub fn deserialize_delete_insight_rules_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteInsightRulesInput, String> {
    let mut input = DeleteInsightRulesInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "RuleNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.rule_names = items;
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DeleteMetricStream.
pub fn deserialize_delete_metric_stream_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<DeleteMetricStreamInput, String> {
    let mut input = DeleteMetricStreamInput::default();
    if let Some(value) = params.get("Name") {
        input.name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeAlarmContributors.
pub fn deserialize_describe_alarm_contributors_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeAlarmContributorsInput, String> {
    let mut input = DescribeAlarmContributorsInput::default();
    if let Some(value) = params.get("AlarmName") {
        input.alarm_name = value.to_string();
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeAlarmHistory.
pub fn deserialize_describe_alarm_history_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeAlarmHistoryInput, String> {
    let mut input = DescribeAlarmHistoryInput::default();
    if let Some(value) = params.get("AlarmContributorId") {
        input.alarm_contributor_id = Some(value.to_string());
    }
    if let Some(value) = params.get("AlarmName") {
        input.alarm_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "AlarmTypes".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.alarm_types = Some(items);
        }
    }
    if let Some(value) = params.get("EndDate") {
        input.end_date = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse EndDate: {e}"))?,
        );
    }
    if let Some(value) = params.get("HistoryItemType") {
        input.history_item_type = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("ScanBy") {
        input.scan_by = Some(value.to_string());
    }
    if let Some(value) = params.get("StartDate") {
        input.start_date = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse StartDate: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeAlarms.
pub fn deserialize_describe_alarms_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeAlarmsInput, String> {
    let mut input = DescribeAlarmsInput::default();
    if let Some(value) = params.get("ActionPrefix") {
        input.action_prefix = Some(value.to_string());
    }
    if let Some(value) = params.get("AlarmNamePrefix") {
        input.alarm_name_prefix = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "AlarmNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.alarm_names = Some(items);
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "AlarmTypes".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.alarm_types = Some(items);
        }
    }
    if let Some(value) = params.get("ChildrenOfAlarmName") {
        input.children_of_alarm_name = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("ParentsOfAlarmName") {
        input.parents_of_alarm_name = Some(value.to_string());
    }
    if let Some(value) = params.get("StateValue") {
        input.state_value = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeAlarmsForMetric.
pub fn deserialize_describe_alarms_for_metric_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeAlarmsForMetricInput, String> {
    let mut input = DescribeAlarmsForMetricInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Dimensions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_dimension_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.dimensions = Some(items);
        }
    }
    if let Some(value) = params.get("ExtendedStatistic") {
        input.extended_statistic = Some(value.to_string());
    }
    if let Some(value) = params.get("MetricName") {
        input.metric_name = value.to_string();
    }
    if let Some(value) = params.get("Namespace") {
        input.namespace = value.to_string();
    }
    if let Some(value) = params.get("Period") {
        input.period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Period: {e}"))?,
        );
    }
    if let Some(value) = params.get("Statistic") {
        input.statistic = Some(value.to_string());
    }
    if let Some(value) = params.get("Unit") {
        input.unit = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeAnomalyDetectors.
pub fn deserialize_describe_anomaly_detectors_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeAnomalyDetectorsInput, String> {
    let mut input = DescribeAnomalyDetectorsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AnomalyDetectorTypes".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.anomaly_detector_types = Some(items);
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Dimensions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_dimension_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.dimensions = Some(items);
        }
    }
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("MetricName") {
        input.metric_name = Some(value.to_string());
    }
    if let Some(value) = params.get("Namespace") {
        input.namespace = Some(value.to_string());
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DescribeInsightRules.
pub fn deserialize_describe_insight_rules_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<DescribeInsightRulesInput, String> {
    let mut input = DescribeInsightRulesInput::default();
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for DisableAlarmActions.
pub fn deserialize_disable_alarm_actions_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<DisableAlarmActionsInput, String> {
    let mut input = DisableAlarmActionsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AlarmNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.alarm_names = items;
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for DisableInsightRules.
pub fn deserialize_disable_insight_rules_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<DisableInsightRulesInput, String> {
    let mut input = DisableInsightRulesInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "RuleNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.rule_names = items;
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for EnableAlarmActions.
pub fn deserialize_enable_alarm_actions_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<EnableAlarmActionsInput, String> {
    let mut input = EnableAlarmActionsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "AlarmNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.alarm_names = items;
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for EnableInsightRules.
pub fn deserialize_enable_insight_rules_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<EnableInsightRulesInput, String> {
    let mut input = EnableInsightRulesInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "RuleNames".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.rule_names = items;
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetAlarmMuteRule.
pub fn deserialize_get_alarm_mute_rule_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetAlarmMuteRuleInput, String> {
    let mut input = GetAlarmMuteRuleInput::default();
    if let Some(value) = params.get("AlarmMuteRuleName") {
        input.alarm_mute_rule_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetDashboard.
pub fn deserialize_get_dashboard_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetDashboardInput, String> {
    let mut input = GetDashboardInput::default();
    if let Some(value) = params.get("DashboardName") {
        input.dashboard_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetInsightRuleReport.
pub fn deserialize_get_insight_rule_report_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetInsightRuleReportInput, String> {
    let mut input = GetInsightRuleReportInput::default();
    if let Some(value) = params.get("EndTime") {
        input.end_time = value
            .parse::<f64>()
            .map_err(|e| format!("failed to parse EndTime: {e}"))?;
    }
    if let Some(value) = params.get("MaxContributorCount") {
        input.max_contributor_count = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxContributorCount: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Metrics".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.metrics = Some(items);
        }
    }
    if let Some(value) = params.get("OrderBy") {
        input.order_by = Some(value.to_string());
    }
    if let Some(value) = params.get("Period") {
        input.period = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse Period: {e}"))?;
    }
    if let Some(value) = params.get("RuleName") {
        input.rule_name = value.to_string();
    }
    if let Some(value) = params.get("StartTime") {
        input.start_time = value
            .parse::<f64>()
            .map_err(|e| format!("failed to parse StartTime: {e}"))?;
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetMetricData.
pub fn deserialize_get_metric_data_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetMetricDataInput, String> {
    let mut input = GetMetricDataInput::default();
    if let Some(value) = params.get("EndTime") {
        input.end_time = value
            .parse::<f64>()
            .map_err(|e| format!("failed to parse EndTime: {e}"))?;
    }
    if let Some(val) = deserialize_label_options_from_query(params, "LabelOptions")? {
        input.label_options = Some(val);
    }
    if let Some(value) = params.get("MaxDatapoints") {
        input.max_datapoints = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxDatapoints: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "MetricDataQueries".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_metric_data_query_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.metric_data_queries = items;
        }
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("ScanBy") {
        input.scan_by = Some(value.to_string());
    }
    if let Some(value) = params.get("StartTime") {
        input.start_time = value
            .parse::<f64>()
            .map_err(|e| format!("failed to parse StartTime: {e}"))?;
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetMetricStatistics.
pub fn deserialize_get_metric_statistics_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetMetricStatisticsInput, String> {
    let mut input = GetMetricStatisticsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Dimensions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_dimension_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.dimensions = Some(items);
        }
    }
    if let Some(value) = params.get("EndTime") {
        input.end_time = value
            .parse::<f64>()
            .map_err(|e| format!("failed to parse EndTime: {e}"))?;
    }
    {
        let mut items = Vec::new();
        let list_prefix = "ExtendedStatistics".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.extended_statistics = Some(items);
        }
    }
    if let Some(value) = params.get("MetricName") {
        input.metric_name = value.to_string();
    }
    if let Some(value) = params.get("Namespace") {
        input.namespace = value.to_string();
    }
    if let Some(value) = params.get("Period") {
        input.period = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse Period: {e}"))?;
    }
    if let Some(value) = params.get("StartTime") {
        input.start_time = value
            .parse::<f64>()
            .map_err(|e| format!("failed to parse StartTime: {e}"))?;
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Statistics".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.statistics = Some(items);
        }
    }
    if let Some(value) = params.get("Unit") {
        input.unit = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetMetricStream.
pub fn deserialize_get_metric_stream_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetMetricStreamInput, String> {
    let mut input = GetMetricStreamInput::default();
    if let Some(value) = params.get("Name") {
        input.name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for GetMetricWidgetImage.
pub fn deserialize_get_metric_widget_image_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<GetMetricWidgetImageInput, String> {
    let mut input = GetMetricWidgetImageInput::default();
    if let Some(value) = params.get("MetricWidget") {
        input.metric_widget = value.to_string();
    }
    if let Some(value) = params.get("OutputFormat") {
        input.output_format = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListAlarmMuteRules.
pub fn deserialize_list_alarm_mute_rules_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListAlarmMuteRulesInput, String> {
    let mut input = ListAlarmMuteRulesInput::default();
    if let Some(value) = params.get("AlarmName") {
        input.alarm_name = Some(value.to_string());
    }
    if let Some(value) = params.get("MaxRecords") {
        input.max_records = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxRecords: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Statuses".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.statuses = Some(items);
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListDashboards.
pub fn deserialize_list_dashboards_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListDashboardsInput, String> {
    let mut input = ListDashboardsInput::default();
    if let Some(value) = params.get("DashboardNamePrefix") {
        input.dashboard_name_prefix = Some(value.to_string());
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListManagedInsightRules.
pub fn deserialize_list_managed_insight_rules_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListManagedInsightRulesInput, String> {
    let mut input = ListManagedInsightRulesInput::default();
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("ResourceARN") {
        input.resource_a_r_n = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListMetricStreams.
pub fn deserialize_list_metric_streams_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListMetricStreamsInput, String> {
    let mut input = ListMetricStreamsInput::default();
    if let Some(value) = params.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse MaxResults: {e}"))?,
        );
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListMetrics.
pub fn deserialize_list_metrics_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListMetricsInput, String> {
    let mut input = ListMetricsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Dimensions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_dimension_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.dimensions = Some(items);
        }
    }
    if let Some(value) = params.get("IncludeLinkedAccounts") {
        input.include_linked_accounts = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse IncludeLinkedAccounts: {e}"))?,
        );
    }
    if let Some(value) = params.get("MetricName") {
        input.metric_name = Some(value.to_string());
    }
    if let Some(value) = params.get("Namespace") {
        input.namespace = Some(value.to_string());
    }
    if let Some(value) = params.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = params.get("OwningAccount") {
        input.owning_account = Some(value.to_string());
    }
    if let Some(value) = params.get("RecentlyActive") {
        input.recently_active = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for ListTagsForResource.
pub fn deserialize_list_tags_for_resource_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceInput, String> {
    let mut input = ListTagsForResourceInput::default();
    if let Some(value) = params.get("ResourceARN") {
        input.resource_a_r_n = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutAlarmMuteRule.
pub fn deserialize_put_alarm_mute_rule_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutAlarmMuteRuleInput, String> {
    let mut input = PutAlarmMuteRuleInput::default();
    if let Some(value) = params.get("Description") {
        input.description = Some(value.to_string());
    }
    if let Some(value) = params.get("ExpireDate") {
        input.expire_date = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse ExpireDate: {e}"))?,
        );
    }
    if let Some(val) = deserialize_mute_targets_from_query(params, "MuteTargets")? {
        input.mute_targets = Some(val);
    }
    if let Some(value) = params.get("Name") {
        input.name = value.to_string();
    }
    if let Some(val) = deserialize_rule_from_query(params, "Rule")? {
        input.rule = val;
    }
    if let Some(value) = params.get("StartDate") {
        input.start_date = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse StartDate: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(items);
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutAnomalyDetector.
pub fn deserialize_put_anomaly_detector_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutAnomalyDetectorInput, String> {
    let mut input = PutAnomalyDetectorInput::default();
    if let Some(val) =
        deserialize_anomaly_detector_configuration_from_query(params, "Configuration")?
    {
        input.configuration = Some(val);
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Dimensions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_dimension_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.dimensions = Some(items);
        }
    }
    if let Some(val) =
        deserialize_metric_characteristics_from_query(params, "MetricCharacteristics")?
    {
        input.metric_characteristics = Some(val);
    }
    if let Some(val) =
        deserialize_metric_math_anomaly_detector_from_query(params, "MetricMathAnomalyDetector")?
    {
        input.metric_math_anomaly_detector = Some(val);
    }
    if let Some(value) = params.get("MetricName") {
        input.metric_name = Some(value.to_string());
    }
    if let Some(value) = params.get("Namespace") {
        input.namespace = Some(value.to_string());
    }
    if let Some(val) = deserialize_single_metric_anomaly_detector_from_query(
        params,
        "SingleMetricAnomalyDetector",
    )? {
        input.single_metric_anomaly_detector = Some(val);
    }
    if let Some(value) = params.get("Stat") {
        input.stat = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutCompositeAlarm.
pub fn deserialize_put_composite_alarm_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutCompositeAlarmInput, String> {
    let mut input = PutCompositeAlarmInput::default();
    if let Some(value) = params.get("ActionsEnabled") {
        input.actions_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ActionsEnabled: {e}"))?,
        );
    }
    if let Some(value) = params.get("ActionsSuppressor") {
        input.actions_suppressor = Some(value.to_string());
    }
    if let Some(value) = params.get("ActionsSuppressorExtensionPeriod") {
        input.actions_suppressor_extension_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse ActionsSuppressorExtensionPeriod: {e}"))?,
        );
    }
    if let Some(value) = params.get("ActionsSuppressorWaitPeriod") {
        input.actions_suppressor_wait_period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse ActionsSuppressorWaitPeriod: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "AlarmActions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.alarm_actions = Some(items);
        }
    }
    if let Some(value) = params.get("AlarmDescription") {
        input.alarm_description = Some(value.to_string());
    }
    if let Some(value) = params.get("AlarmName") {
        input.alarm_name = value.to_string();
    }
    if let Some(value) = params.get("AlarmRule") {
        input.alarm_rule = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "InsufficientDataActions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.insufficient_data_actions = Some(items);
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "OKActions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.o_k_actions = Some(items);
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(items);
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutDashboard.
pub fn deserialize_put_dashboard_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutDashboardInput, String> {
    let mut input = PutDashboardInput::default();
    if let Some(value) = params.get("DashboardBody") {
        input.dashboard_body = value.to_string();
    }
    if let Some(value) = params.get("DashboardName") {
        input.dashboard_name = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutInsightRule.
pub fn deserialize_put_insight_rule_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutInsightRuleInput, String> {
    let mut input = PutInsightRuleInput::default();
    if let Some(value) = params.get("ApplyOnTransformedLogs") {
        input.apply_on_transformed_logs = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ApplyOnTransformedLogs: {e}"))?,
        );
    }
    if let Some(value) = params.get("RuleDefinition") {
        input.rule_definition = value.to_string();
    }
    if let Some(value) = params.get("RuleName") {
        input.rule_name = value.to_string();
    }
    if let Some(value) = params.get("RuleState") {
        input.rule_state = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(items);
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutManagedInsightRules.
pub fn deserialize_put_managed_insight_rules_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutManagedInsightRulesInput, String> {
    let mut input = PutManagedInsightRulesInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "ManagedRules".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_managed_rule_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.managed_rules = items;
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutMetricAlarm.
pub fn deserialize_put_metric_alarm_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutMetricAlarmInput, String> {
    let mut input = PutMetricAlarmInput::default();
    if let Some(value) = params.get("ActionsEnabled") {
        input.actions_enabled = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse ActionsEnabled: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "AlarmActions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.alarm_actions = Some(items);
        }
    }
    if let Some(value) = params.get("AlarmDescription") {
        input.alarm_description = Some(value.to_string());
    }
    if let Some(value) = params.get("AlarmName") {
        input.alarm_name = value.to_string();
    }
    if let Some(value) = params.get("ComparisonOperator") {
        input.comparison_operator = value.to_string();
    }
    if let Some(value) = params.get("DatapointsToAlarm") {
        input.datapoints_to_alarm = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse DatapointsToAlarm: {e}"))?,
        );
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Dimensions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_dimension_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.dimensions = Some(items);
        }
    }
    if let Some(value) = params.get("EvaluateLowSampleCountPercentile") {
        input.evaluate_low_sample_count_percentile = Some(value.to_string());
    }
    if let Some(value) = params.get("EvaluationPeriods") {
        input.evaluation_periods = value
            .parse::<i32>()
            .map_err(|e| format!("failed to parse EvaluationPeriods: {e}"))?;
    }
    if let Some(value) = params.get("ExtendedStatistic") {
        input.extended_statistic = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "InsufficientDataActions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.insufficient_data_actions = Some(items);
        }
    }
    if let Some(value) = params.get("MetricName") {
        input.metric_name = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Metrics".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_metric_data_query_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.metrics = Some(items);
        }
    }
    if let Some(value) = params.get("Namespace") {
        input.namespace = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "OKActions".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.o_k_actions = Some(items);
        }
    }
    if let Some(value) = params.get("Period") {
        input.period = Some(
            value
                .parse::<i32>()
                .map_err(|e| format!("failed to parse Period: {e}"))?,
        );
    }
    if let Some(value) = params.get("Statistic") {
        input.statistic = Some(value.to_string());
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(items);
        }
    }
    if let Some(value) = params.get("Threshold") {
        input.threshold = Some(
            value
                .parse::<f64>()
                .map_err(|e| format!("failed to parse Threshold: {e}"))?,
        );
    }
    if let Some(value) = params.get("ThresholdMetricId") {
        input.threshold_metric_id = Some(value.to_string());
    }
    if let Some(value) = params.get("TreatMissingData") {
        input.treat_missing_data = Some(value.to_string());
    }
    if let Some(value) = params.get("Unit") {
        input.unit = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutMetricData.
pub fn deserialize_put_metric_data_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutMetricDataInput, String> {
    let mut input = PutMetricDataInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "EntityMetricData".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_entity_metric_data_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.entity_metric_data = Some(items);
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "MetricData".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_metric_datum_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.metric_data = Some(items);
        }
    }
    if let Some(value) = params.get("Namespace") {
        input.namespace = value.to_string();
    }
    if let Some(value) = params.get("StrictEntityValidation") {
        input.strict_entity_validation = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse StrictEntityValidation: {e}"))?,
        );
    }
    Ok(input)
}

/// Deserialize awsQuery request for PutMetricStream.
pub fn deserialize_put_metric_stream_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<PutMetricStreamInput, String> {
    let mut input = PutMetricStreamInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "ExcludeFilters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_metric_stream_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.exclude_filters = Some(items);
        }
    }
    if let Some(value) = params.get("FirehoseArn") {
        input.firehose_arn = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "IncludeFilters".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_metric_stream_filter_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.include_filters = Some(items);
        }
    }
    if let Some(value) = params.get("IncludeLinkedAccountsMetrics") {
        input.include_linked_accounts_metrics = Some(
            value
                .parse::<bool>()
                .map_err(|e| format!("failed to parse IncludeLinkedAccountsMetrics: {e}"))?,
        );
    }
    if let Some(value) = params.get("Name") {
        input.name = value.to_string();
    }
    if let Some(value) = params.get("OutputFormat") {
        input.output_format = value.to_string();
    }
    if let Some(value) = params.get("RoleArn") {
        input.role_arn = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "StatisticsConfigurations".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_metric_stream_statistics_configuration_from_query(params, &item_key)?
            {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.statistics_configurations = Some(items);
        }
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = Some(items);
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for SetAlarmState.
pub fn deserialize_set_alarm_state_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<SetAlarmStateInput, String> {
    let mut input = SetAlarmStateInput::default();
    if let Some(value) = params.get("AlarmName") {
        input.alarm_name = value.to_string();
    }
    if let Some(value) = params.get("StateReason") {
        input.state_reason = value.to_string();
    }
    if let Some(value) = params.get("StateReasonData") {
        input.state_reason_data = Some(value.to_string());
    }
    if let Some(value) = params.get("StateValue") {
        input.state_value = value.to_string();
    }
    Ok(input)
}

/// Deserialize awsQuery request for StartMetricStreams.
pub fn deserialize_start_metric_streams_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<StartMetricStreamsInput, String> {
    let mut input = StartMetricStreamsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Names".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.names = items;
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for StopMetricStreams.
pub fn deserialize_stop_metric_streams_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<StopMetricStreamsInput, String> {
    let mut input = StopMetricStreamsInput::default();
    {
        let mut items = Vec::new();
        let list_prefix = "Names".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.names = items;
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for TagResource.
pub fn deserialize_tag_resource_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<TagResourceInput, String> {
    let mut input = TagResourceInput::default();
    if let Some(value) = params.get("ResourceARN") {
        input.resource_a_r_n = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "Tags".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match deserialize_tag_from_query(params, &item_key)? {
                Some(item) => items.push(item),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tags = items;
        }
    }
    Ok(input)
}

/// Deserialize awsQuery request for UntagResource.
pub fn deserialize_untag_resource_request_query(
    params: &std::collections::HashMap<String, String>,
) -> Result<UntagResourceInput, String> {
    let mut input = UntagResourceInput::default();
    if let Some(value) = params.get("ResourceARN") {
        input.resource_a_r_n = value.to_string();
    }
    {
        let mut items = Vec::new();
        let list_prefix = "TagKeys".to_string();
        for i in 1.. {
            let item_key = format!("{list_prefix}.member.{i}");
            match params.get(&item_key) {
                Some(value) => items.push(value.to_string()),
                None => break,
            }
        }
        if !items.is_empty() {
            input.tag_keys = items;
        }
    }
    Ok(input)
}
