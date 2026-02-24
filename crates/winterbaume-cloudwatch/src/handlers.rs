use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{CloudWatchError, CloudWatchState};
use crate::types::{Dimension, ManagedInsightRule, MetricDatum, PutMetricAlarmInput};
use crate::views::CloudwatchStateView;
use crate::wire;

/// CloudWatch service handler that processes rpc-v2-cbor protocol requests.
///
/// The latest AWS SDK sends CloudWatch requests using the Smithy rpc-v2-cbor
/// protocol with URLs like:
///   POST /service/GraniteServiceVersion20100801/operation/{Action}
/// Body is CBOR-encoded. Response is also CBOR-encoded.
pub struct CloudWatchService {
    pub(crate) state: Arc<BackendState<CloudWatchState>>,
    pub(crate) notifier: StateChangeNotifier<CloudwatchStateView>,
}

impl CloudWatchService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CloudWatchService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CloudWatchService {
    fn service_name(&self) -> &str {
        "monitoring"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://monitoring\..*\.amazonaws\.com",
            r"https?://monitoring\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl CloudWatchService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        // Detect protocol: CBOR (path-based) vs Query (form-encoded body)
        let is_cbor = extract_action_from_path(&request.uri).is_some();

        let (action, body) = if is_cbor {
            // CBOR protocol: action from URL path, body from CBOR
            let action = extract_action_from_path(&request.uri).unwrap();
            let body: Value = if request.body.is_empty() {
                json!({})
            } else {
                match ciborium::from_reader::<ciborium::Value, _>(&request.body[..]) {
                    Ok(cbor_val) => wire::cbor_to_json(cbor_val),
                    Err(_) => {
                        return wire::cbor_error_response(
                            400,
                            "SerializationException",
                            "Invalid CBOR body",
                        );
                    }
                }
            };
            (action, body)
        } else {
            // Query protocol: action and params from form-encoded body
            let body_str = std::str::from_utf8(&request.body).unwrap_or("");
            let params = winterbaume_core::protocol::parse_query_string(body_str);
            let action = match params.get("Action") {
                Some(a) => a.clone(),
                None => {
                    return query_error_response(
                        400,
                        "MissingAction",
                        "Missing 'Action' parameter",
                    );
                }
            };
            let body = query_params_to_json(&params);
            (action, body)
        };

        let state = self.state.get(account_id, &region);

        let response = self
            .dispatch_action(&action, &state, &body, account_id, &region)
            .await;

        if is_cbor {
            if response.status >= 200 && response.status < 300 {
                self.notify_state_changed(account_id, &region).await;
            }
            response
        } else {
            // Convert CBOR response to XML for query protocol callers
            let xml_resp = cbor_response_to_xml(&action, response);
            if xml_resp.status >= 200 && xml_resp.status < 300 {
                self.notify_state_changed(account_id, &region).await;
            }
            xml_resp
        }
    }

    async fn dispatch_action(
        &self,
        action: &str,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        match action {
            "PutMetricData" => self.handle_put_metric_data(state, body).await,
            "GetMetricData" => self.handle_get_metric_data(state, body).await,
            "ListMetrics" => self.handle_list_metrics(state, body).await,
            "PutMetricAlarm" => {
                self.handle_put_metric_alarm(state, body, account_id, region)
                    .await
            }
            "DescribeAlarms" => self.handle_describe_alarms(state, body).await,
            "DescribeAlarmsForMetric" => self.handle_describe_alarms_for_metric(state, body).await,
            "DeleteAlarms" => self.handle_delete_alarms(state, body).await,
            "PutDashboard" => {
                self.handle_put_dashboard(state, body, account_id, region)
                    .await
            }
            "GetDashboard" => self.handle_get_dashboard(state, body).await,
            "ListDashboards" => self.handle_list_dashboards(state, body).await,
            "DeleteDashboards" => self.handle_delete_dashboards(state, body).await,
            "PutInsightRule" => self.handle_put_insight_rule(state, body).await,
            "DescribeInsightRules" => self.handle_describe_insight_rules(state, body).await,
            "EnableInsightRules" => self.handle_enable_insight_rules(state, body).await,
            "DisableInsightRules" => self.handle_disable_insight_rules(state, body).await,
            "DeleteInsightRules" => self.handle_delete_insight_rules(state, body).await,
            "GetMetricStatistics" => self.handle_get_metric_statistics(state, body).await,
            "SetAlarmState" => self.handle_set_alarm_state(state, body).await,
            "TagResource" => self.handle_tag_resource(state, body).await,
            "UntagResource" => self.handle_untag_resource(state, body).await,
            "ListTagsForResource" => self.handle_list_tags_for_resource(state, body).await,
            "DisableAlarmActions" => self.handle_disable_alarm_actions(state, body).await,
            "EnableAlarmActions" => self.handle_enable_alarm_actions(state, body).await,
            "DescribeAlarmHistory" => self.handle_describe_alarm_history(state, body).await,
            "DescribeAlarmContributors" => {
                self.handle_describe_alarm_contributors(state, body).await
            }
            "PutCompositeAlarm" => {
                self.handle_put_composite_alarm(state, body, account_id, region)
                    .await
            }
            "PutAnomalyDetector" => self.handle_put_anomaly_detector(state, body).await,
            "DeleteAnomalyDetector" => self.handle_delete_anomaly_detector(state, body).await,
            "DescribeAnomalyDetectors" => self.handle_describe_anomaly_detectors(state, body).await,
            "PutMetricStream" => {
                self.handle_put_metric_stream(state, body, account_id, region)
                    .await
            }
            "DeleteMetricStream" => self.handle_delete_metric_stream(state, body).await,
            "GetMetricStream" => self.handle_get_metric_stream(state, body).await,
            "ListMetricStreams" => self.handle_list_metric_streams(state, body).await,
            "StartMetricStreams" => self.handle_start_metric_streams(state, body).await,
            "StopMetricStreams" => self.handle_stop_metric_streams(state, body).await,
            "PutAlarmMuteRule" => {
                self.handle_put_alarm_mute_rule(state, body, account_id, region)
                    .await
            }
            "GetAlarmMuteRule" => self.handle_get_alarm_mute_rule(state, body).await,
            "DeleteAlarmMuteRule" => self.handle_delete_alarm_mute_rule(state, body).await,
            "ListAlarmMuteRules" => self.handle_list_alarm_mute_rules(state, body).await,
            "GetMetricWidgetImage" => self.handle_get_metric_widget_image(state, body).await,
            "GetInsightRuleReport" => self.handle_get_insight_rule_report(state, body).await,
            "ListManagedInsightRules" => self.handle_list_managed_insight_rules(state, body).await,
            "PutManagedInsightRules" => self.handle_put_managed_insight_rules(state, body).await,
            _ => wire::cbor_error_response(
                400,
                "InvalidAction",
                &format!("Could not find operation {action} for CloudWatch"),
            ),
        }
    }

    async fn handle_put_metric_data(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let namespace = match body.get("Namespace").and_then(|v| v.as_str()) {
            Some(ns) => ns,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'Namespace'");
            }
        };

        let metric_data = extract_metric_data_from_json(body);

        let mut state = state.write().await;
        match state.put_metric_data(namespace, metric_data) {
            Ok(()) => cbor_response(200, json!({})),
            Err(e) => cw_error_response(&e),
        }
    }

    async fn handle_get_metric_data(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let namespace = body.get("Namespace").and_then(|v| v.as_str());
        let metric_name = body.get("MetricName").and_then(|v| v.as_str());

        let state = state.read().await;
        let data = state.get_metric_data(namespace, metric_name);

        let results: Vec<wire::MetricDataResult> = data
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let epoch = m.timestamp.timestamp() as f64;
                wire::MetricDataResult {
                    id: Some(format!("metric{i}")),
                    label: Some(m.metric_name.clone()),
                    timestamps: Some(vec![epoch]),
                    values: Some(vec![m.value.unwrap_or(0.0)]),
                    status_code: Some("Complete".to_string()),
                    ..Default::default()
                }
            })
            .collect();

        wire::serialize_get_metric_data_response(&wire::GetMetricDataOutput {
            metric_data_results: Some(results),
            ..Default::default()
        })
    }

    async fn handle_list_metrics(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let namespace = body.get("Namespace").and_then(|v| v.as_str());
        let metric_name = body.get("MetricName").and_then(|v| v.as_str());

        let state = state.read().await;
        let metrics = state.list_metrics(namespace, metric_name);

        let entries = metrics.iter().map(metric_to_model).collect();
        wire::serialize_list_metrics_response(&wire::ListMetricsOutput {
            metrics: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_put_metric_alarm(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let alarm_name = match body.get("AlarmName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'AlarmName'");
            }
        };
        let metric_name = match body.get("MetricName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'MetricName'");
            }
        };
        let namespace = match body.get("Namespace").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'Namespace'");
            }
        };
        let threshold = body
            .get("Threshold")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let comparison_operator = body
            .get("ComparisonOperator")
            .and_then(|v| v.as_str())
            .unwrap_or("GreaterThanThreshold");
        let evaluation_periods = body
            .get("EvaluationPeriods")
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as u32;
        let period = body.get("Period").and_then(|v| v.as_u64()).unwrap_or(60) as u32;
        let statistic = body
            .get("Statistic")
            .and_then(|v| v.as_str())
            .unwrap_or("Average");
        let alarm_description = body.get("AlarmDescription").and_then(|v| v.as_str());

        let alarm_actions: Vec<String> = extract_string_list(body, "AlarmActions");
        let ok_actions: Vec<String> = extract_string_list(body, "OKActions");
        let insufficient_data_actions: Vec<String> =
            extract_string_list(body, "InsufficientDataActions");

        let actions_enabled = body
            .get("ActionsEnabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let unit = body
            .get("Unit")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let dimensions = extract_dimensions_from_json(body, "Dimensions");
        let tags = extract_tags_from_json(body);

        let input = PutMetricAlarmInput {
            alarm_name: alarm_name.to_string(),
            metric_name: metric_name.to_string(),
            namespace: namespace.to_string(),
            threshold,
            comparison_operator: comparison_operator.to_string(),
            evaluation_periods,
            period,
            statistic: statistic.to_string(),
            alarm_description: alarm_description.map(|s| s.to_string()),
            alarm_actions,
            ok_actions,
            insufficient_data_actions,
            actions_enabled,
            dimensions,
            unit,
            tags,
        };

        let mut state = state.write().await;
        match state.put_metric_alarm(input, account_id, region) {
            Ok(()) => cbor_response(200, json!({})),
            Err(e) => cw_error_response(&e),
        }
    }

    async fn handle_describe_alarms(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let alarm_names: Vec<String> = extract_string_list(body, "AlarmNames");
        let state_value = body.get("StateValue").and_then(|v| v.as_str());
        let alarm_types: Vec<String> = extract_string_list(body, "AlarmTypes");

        // When AlarmTypes is omitted, include both kinds.
        let include_metric =
            alarm_types.is_empty() || alarm_types.iter().any(|t| t == "MetricAlarm");
        let include_composite =
            alarm_types.is_empty() || alarm_types.iter().any(|t| t == "CompositeAlarm");

        let names_ref = if alarm_names.is_empty() {
            None
        } else {
            Some(alarm_names.as_slice())
        };

        let state = state.read().await;

        let metric_entries = if include_metric {
            state
                .describe_alarms(names_ref, state_value)
                .iter()
                .map(|alarm| alarm_to_model(alarm))
                .collect()
        } else {
            vec![]
        };

        let composite_entries = if include_composite {
            state
                .composite_alarms
                .values()
                .filter(|a| match names_ref {
                    Some(names) => names.iter().any(|n| n == &a.alarm_name),
                    None => true,
                })
                .filter(|a| match state_value {
                    Some(sv) => a.state_value.as_str() == sv,
                    None => true,
                })
                .map(composite_alarm_to_model)
                .collect()
        } else {
            vec![]
        };

        wire::serialize_describe_alarms_response(&wire::DescribeAlarmsOutput {
            metric_alarms: Some(metric_entries),
            composite_alarms: Some(composite_entries),
            ..Default::default()
        })
    }

    async fn handle_describe_alarms_for_metric(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let metric_name = match body.get("MetricName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'MetricName'");
            }
        };
        let namespace = match body.get("Namespace").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'Namespace'");
            }
        };

        let state = state.read().await;
        let alarms = state.describe_alarms_for_metric(metric_name, namespace);

        let entries = alarms.iter().map(|alarm| alarm_to_model(alarm)).collect();
        wire::serialize_describe_alarms_for_metric_response(&wire::DescribeAlarmsForMetricOutput {
            metric_alarms: Some(entries),
        })
    }

    async fn handle_delete_alarms(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let alarm_names: Vec<String> = extract_string_list(body, "AlarmNames");

        if alarm_names.is_empty() {
            return wire::cbor_error_response(400, "MissingParameter", "Missing 'AlarmNames'");
        }

        let mut state = state.write().await;
        match state.delete_alarms(&alarm_names) {
            Ok(()) => cbor_response(200, json!({})),
            Err(e) => cw_error_response(&e),
        }
    }
    // --- Dashboard handlers ---

    async fn handle_put_dashboard(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let dashboard_name = match body.get("DashboardName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return wire::cbor_error_response(
                    400,
                    "MissingParameter",
                    "Missing 'DashboardName'",
                );
            }
        };
        let dashboard_body = match body.get("DashboardBody").and_then(|v| v.as_str()) {
            Some(b) => b,
            None => {
                return wire::cbor_error_response(
                    400,
                    "MissingParameter",
                    "Missing 'DashboardBody'",
                );
            }
        };
        let mut state = state.write().await;
        match state.put_dashboard(dashboard_name, dashboard_body, account_id, region) {
            Ok(()) => {
                let arn =
                    format!("arn:aws:cloudwatch:{region}:{account_id}:dashboard/{dashboard_name}");
                cbor_response(
                    200,
                    json!({ "DashboardValidationMessages": [], "DashboardArn": arn }),
                )
            }
            Err(e) => cw_error_response(&e),
        }
    }

    async fn handle_get_dashboard(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let dashboard_name = match body.get("DashboardName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return wire::cbor_error_response(
                    400,
                    "MissingParameter",
                    "Missing 'DashboardName'",
                );
            }
        };
        let state = state.read().await;
        match state.get_dashboard(dashboard_name) {
            Ok(d) => wire::serialize_get_dashboard_response(&wire::GetDashboardOutput {
                dashboard_arn: Some(d.dashboard_arn.clone()),
                dashboard_body: Some(d.dashboard_body.clone()),
                dashboard_name: Some(d.dashboard_name.clone()),
            }),
            Err(e) => cw_error_response(&e),
        }
    }

    async fn handle_list_dashboards(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let prefix = body.get("DashboardNamePrefix").and_then(|v| v.as_str());
        let state = state.read().await;
        let dashboards = state.list_dashboards(prefix);

        let entries: Vec<wire::DashboardEntry> = dashboards
            .iter()
            .map(|d| {
                let epoch = d.last_modified.timestamp() as f64;
                wire::DashboardEntry {
                    dashboard_name: Some(d.dashboard_name.clone()),
                    dashboard_arn: Some(d.dashboard_arn.clone()),
                    last_modified: Some(epoch),
                    size: Some(d.dashboard_body.len() as i64),
                }
            })
            .collect();

        wire::serialize_list_dashboards_response(&wire::ListDashboardsOutput {
            dashboard_entries: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_delete_dashboards(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let dashboard_names: Vec<String> = body
            .get("DashboardNames")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        if dashboard_names.is_empty() {
            return wire::cbor_error_response(400, "MissingParameter", "Missing 'DashboardNames'");
        }
        let mut state = state.write().await;
        match state.delete_dashboards(&dashboard_names) {
            Ok(()) => cbor_response(200, json!({})),
            Err(e) => cw_error_response(&e),
        }
    }

    // --- Insight Rule handlers ---

    async fn handle_put_insight_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let rule_name = match body.get("RuleName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'RuleName'");
            }
        };
        let rule_definition = match body.get("RuleDefinition").and_then(|v| v.as_str()) {
            Some(d) => d,
            None => {
                return wire::cbor_error_response(
                    400,
                    "MissingParameter",
                    "Missing 'RuleDefinition'",
                );
            }
        };
        let schema = body
            .get("RuleState")
            .and_then(|v| v.as_str())
            .unwrap_or("CloudWatchLogRule/1");
        let mut state = state.write().await;
        match state.put_insight_rule(rule_name, schema, rule_definition) {
            Ok(()) => cbor_response(200, json!({})),
            Err(e) => cw_error_response(&e),
        }
    }

    async fn handle_describe_insight_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        _body: &Value,
    ) -> MockResponse {
        let state = state.read().await;
        let rules = state.describe_insight_rules();
        let entries = rules
            .iter()
            .map(|rule| insight_rule_to_model(rule))
            .collect();
        wire::serialize_describe_insight_rules_response(&wire::DescribeInsightRulesOutput {
            insight_rules: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_enable_insight_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let rule_names: Vec<String> = body
            .get("RuleNames")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        if rule_names.is_empty() {
            return wire::cbor_error_response(400, "MissingParameter", "Missing 'RuleNames'");
        }
        let mut state = state.write().await;
        match state.enable_insight_rules(&rule_names) {
            Ok(()) => cbor_response(200, json!({ "Failures": [] })),
            Err(e) => cw_error_response(&e),
        }
    }

    async fn handle_disable_insight_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let rule_names: Vec<String> = body
            .get("RuleNames")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        if rule_names.is_empty() {
            return wire::cbor_error_response(400, "MissingParameter", "Missing 'RuleNames'");
        }
        let mut state = state.write().await;
        match state.disable_insight_rules(&rule_names) {
            Ok(()) => cbor_response(200, json!({ "Failures": [] })),
            Err(e) => cw_error_response(&e),
        }
    }

    async fn handle_delete_insight_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let rule_names: Vec<String> = body
            .get("RuleNames")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        if rule_names.is_empty() {
            return wire::cbor_error_response(400, "MissingParameter", "Missing 'RuleNames'");
        }
        let mut state = state.write().await;
        match state.delete_insight_rules(&rule_names) {
            Ok(()) => cbor_response(200, json!({ "Failures": [] })),
            Err(e) => cw_error_response(&e),
        }
    }

    // --- GetMetricStatistics handler ---

    // STUB[no-telemetry]: Metric statistics require real infrastructure data (aggregated
    //   data points over time from real metric ingestion); always returns empty datapoints.
    async fn handle_get_metric_statistics(
        &self,
        _state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        _body: &Value,
    ) -> MockResponse {
        // Return empty datapoints -- metric statistics are hard to mock meaningfully.
        wire::serialize_get_metric_statistics_response(&wire::GetMetricStatisticsOutput {
            label: Some(String::new()),
            datapoints: Some(Vec::new()),
        })
    }

    // --- SetAlarmState handler ---

    async fn handle_set_alarm_state(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let alarm_name = match body.get("AlarmName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'AlarmName'");
            }
        };
        let state_value = match body.get("StateValue").and_then(|v| v.as_str()) {
            Some(s) => s,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'StateValue'");
            }
        };
        let state_reason = body
            .get("StateReason")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let mut state = state.write().await;
        match state.set_alarm_state(alarm_name, state_value, state_reason) {
            Ok(()) => cbor_response(200, json!({})),
            Err(e) => cw_error_response(&e),
        }
    }

    // --- Tag handlers ---

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let resource_arn = match body.get("ResourceARN").and_then(|v| v.as_str()) {
            Some(a) => a,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'ResourceARN'");
            }
        };
        let tags = extract_tags_from_json(body);
        let mut state = state.write().await;
        match state.tag_resource(resource_arn, tags) {
            Ok(()) => cbor_response(200, json!({})),
            Err(e) => cw_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let resource_arn = match body.get("ResourceARN").and_then(|v| v.as_str()) {
            Some(a) => a,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'ResourceARN'");
            }
        };
        let tag_keys: Vec<String> = body
            .get("TagKeys")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let mut state = state.write().await;
        match state.untag_resource(resource_arn, &tag_keys) {
            Ok(()) => cbor_response(200, json!({})),
            Err(e) => cw_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let resource_arn = match body.get("ResourceARN").and_then(|v| v.as_str()) {
            Some(a) => a,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'ResourceARN'");
            }
        };
        let state = state.read().await;
        let tags = state.list_tags_for_resource(resource_arn);
        let tag_entries = tags
            .iter()
            .map(|(k, v)| wire::Tag {
                key: k.clone(),
                value: v.clone(),
            })
            .collect();
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceOutput {
            tags: Some(tag_entries),
        })
    }
    // --- DisableAlarmActions / EnableAlarmActions ---

    async fn handle_disable_alarm_actions(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let alarm_names: Vec<String> = extract_string_list(body, "AlarmNames");
        let mut state = state.write().await;
        state.disable_alarm_actions(&alarm_names);
        wire::serialize_disable_alarm_actions_response()
    }

    async fn handle_enable_alarm_actions(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let alarm_names: Vec<String> = extract_string_list(body, "AlarmNames");
        let mut state = state.write().await;
        state.enable_alarm_actions(&alarm_names);
        wire::serialize_enable_alarm_actions_response()
    }

    // --- DescribeAlarmHistory ---

    // STUB[no-telemetry]: Alarm history is runtime telemetry driven by real alarm state
    //   transitions; the mock never fires alarm evaluations, so history is always empty.
    async fn handle_describe_alarm_history(
        &self,
        _state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        _body: &Value,
    ) -> MockResponse {
        wire::serialize_describe_alarm_history_response(&wire::DescribeAlarmHistoryOutput {
            alarm_history_items: Some(Vec::new()),
            ..Default::default()
        })
    }

    // --- DescribeAlarmContributors ---

    // STUB[no-telemetry]: Alarm contributors are derived from real metric evaluation
    //   and Contributor Insights data; the mock has no evaluation engine.
    async fn handle_describe_alarm_contributors(
        &self,
        _state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        _body: &Value,
    ) -> MockResponse {
        wire::serialize_describe_alarm_contributors_response(
            &wire::DescribeAlarmContributorsOutput {
                alarm_contributors: Some(Vec::new()),
                ..Default::default()
            },
        )
    }

    // --- PutCompositeAlarm ---

    async fn handle_put_composite_alarm(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let alarm_name = match body.get("AlarmName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'AlarmName'");
            }
        };
        let alarm_rule = match body.get("AlarmRule").and_then(|v| v.as_str()) {
            Some(r) => r,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'AlarmRule'");
            }
        };
        let alarm_description = body
            .get("AlarmDescription")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let actions_enabled = body
            .get("ActionsEnabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let alarm_actions = extract_string_list(body, "AlarmActions");
        let ok_actions = extract_string_list(body, "OKActions");
        let insufficient_data_actions = extract_string_list(body, "InsufficientDataActions");

        let mut state = state.write().await;
        state.put_composite_alarm(
            alarm_name,
            alarm_rule,
            account_id,
            region,
            alarm_description,
            actions_enabled,
            alarm_actions,
            ok_actions,
            insufficient_data_actions,
        );
        wire::serialize_put_composite_alarm_response()
    }

    // --- PutAnomalyDetector / DeleteAnomalyDetector / DescribeAnomalyDetectors ---

    async fn handle_put_anomaly_detector(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        // Support both the legacy flat format and the newer SingleMetricAnomalyDetector wrapper.
        let single = body.get("SingleMetricAnomalyDetector");
        let metric_src = single.unwrap_or(body);
        let namespace = metric_src
            .get("Namespace")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let metric_name = metric_src
            .get("MetricName")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let stat = metric_src
            .get("Stat")
            .and_then(|v| v.as_str())
            .unwrap_or("Average")
            .to_string();
        let dimensions = extract_dimensions_from_json(metric_src, "Dimensions");

        let detector = crate::types::AnomalyDetector {
            namespace,
            metric_name,
            stat,
            dimensions,
            state_value: "TRAINED_INSUFFICIENT_DATA".to_string(),
        };
        let mut state = state.write().await;
        state.put_anomaly_detector(detector);
        wire::serialize_put_anomaly_detector_response(&wire::PutAnomalyDetectorOutput::default())
    }

    async fn handle_delete_anomaly_detector(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let single = body.get("SingleMetricAnomalyDetector");
        let metric_src = single.unwrap_or(body);
        let namespace = metric_src
            .get("Namespace")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let metric_name = metric_src
            .get("MetricName")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let stat = metric_src
            .get("Stat")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let mut state = state.write().await;
        match state.delete_anomaly_detector(namespace, metric_name, stat) {
            Ok(()) => cbor_response(200, json!({})),
            Err(e) => cw_error_response(&e),
        }
    }

    async fn handle_describe_anomaly_detectors(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        _body: &Value,
    ) -> MockResponse {
        let state = state.read().await;
        let detectors = state.describe_anomaly_detectors();
        let entries = detectors.iter().map(anomaly_detector_to_model).collect();
        wire::serialize_describe_anomaly_detectors_response(&wire::DescribeAnomalyDetectorsOutput {
            anomaly_detectors: Some(entries),
            ..Default::default()
        })
    }

    // --- PutMetricStream / DeleteMetricStream / GetMetricStream / ListMetricStreams ---

    async fn handle_put_metric_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match body.get("Name").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'Name'");
            }
        };
        let firehose_arn = body
            .get("FirehoseArn")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let role_arn = body.get("RoleArn").and_then(|v| v.as_str()).unwrap_or("");
        let output_format = body
            .get("OutputFormat")
            .and_then(|v| v.as_str())
            .unwrap_or("json");

        let include_filters = extract_metric_stream_filters(body, "IncludeFilters");
        let exclude_filters = extract_metric_stream_filters(body, "ExcludeFilters");

        let mut state = state.write().await;
        let arn = state.put_metric_stream(
            name,
            firehose_arn,
            role_arn,
            output_format,
            include_filters,
            exclude_filters,
            account_id,
            region,
        );
        wire::serialize_put_metric_stream_response(&wire::PutMetricStreamOutput { arn: Some(arn) })
    }

    async fn handle_delete_metric_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match body.get("Name").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'Name'");
            }
        };
        let mut state = state.write().await;
        match state.delete_metric_stream(name) {
            Ok(()) => cbor_response(200, json!({})),
            Err(e) => cw_error_response(&e),
        }
    }

    async fn handle_get_metric_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match body.get("Name").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'Name'");
            }
        };
        let state = state.read().await;
        match state.get_metric_stream(name) {
            Ok(s) => wire::serialize_get_metric_stream_response(&metric_stream_to_get_output(s)),
            Err(e) => cw_error_response(&e),
        }
    }

    async fn handle_list_metric_streams(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        _body: &Value,
    ) -> MockResponse {
        let state = state.read().await;
        let streams = state.list_metric_streams();
        let entries = streams
            .iter()
            .map(|stream| metric_stream_to_entry(stream))
            .collect();
        wire::serialize_list_metric_streams_response(&wire::ListMetricStreamsOutput {
            entries: Some(entries),
            ..Default::default()
        })
    }

    async fn handle_start_metric_streams(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let names = extract_string_list(body, "Names");
        let mut state = state.write().await;
        match state.start_metric_streams(&names) {
            Ok(()) => wire::serialize_start_metric_streams_response(
                &wire::StartMetricStreamsOutput::default(),
            ),
            Err(e) => cw_error_response(&e),
        }
    }

    async fn handle_stop_metric_streams(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let names = extract_string_list(body, "Names");
        let mut state = state.write().await;
        match state.stop_metric_streams(&names) {
            Ok(()) => cbor_response(200, json!({})),
            Err(e) => cw_error_response(&e),
        }
    }

    // --- AlarmMuteRule handlers ---

    async fn handle_put_alarm_mute_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let name = match body.get("Name").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'Name'");
            }
        };
        let mute_type = body
            .get("MuteType")
            .and_then(|v| v.as_str())
            .unwrap_or("ALARM_NAMES");
        let alarm_names = body
            .get("MuteTargets")
            .and_then(|t| t.get("AlarmNames"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let description = body
            .get("Description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let mut state = state.write().await;
        state.put_alarm_mute_rule(
            name,
            mute_type,
            alarm_names,
            description,
            account_id,
            region,
        );
        wire::serialize_put_alarm_mute_rule_response()
    }

    async fn handle_get_alarm_mute_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match body.get("Name").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'Name'");
            }
        };
        let state = state.read().await;
        match state.get_alarm_mute_rule(name) {
            Ok(r) => wire::serialize_get_alarm_mute_rule_response(&alarm_mute_rule_to_output(r)),
            Err(e) => cw_error_response(&e),
        }
    }

    async fn handle_delete_alarm_mute_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let name = match body.get("Name").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => {
                return wire::cbor_error_response(400, "MissingParameter", "Missing 'Name'");
            }
        };
        let mut state = state.write().await;
        match state.delete_alarm_mute_rule(name) {
            Ok(()) => cbor_response(200, json!({})),
            Err(e) => cw_error_response(&e),
        }
    }

    async fn handle_list_alarm_mute_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        _body: &Value,
    ) -> MockResponse {
        let state = state.read().await;
        let rules = state.list_alarm_mute_rules();
        let entries = rules
            .iter()
            .map(|rule| alarm_mute_rule_to_summary(rule))
            .collect();
        wire::serialize_list_alarm_mute_rules_response(&wire::ListAlarmMuteRulesOutput {
            alarm_mute_rule_summaries: Some(entries),
            ..Default::default()
        })
    }

    // --- GetMetricWidgetImage ---

    // STUB[no-engine]: Metric widget image rendering requires a real charting engine;
    //   always returns a minimal 1×1 placeholder PNG.
    async fn handle_get_metric_widget_image(
        &self,
        _state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        _body: &Value,
    ) -> MockResponse {
        // Minimal valid 1x1 PNG, base64-encoded
        let mock_png_b64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";
        wire::serialize_get_metric_widget_image_response(&wire::GetMetricWidgetImageOutput {
            metric_widget_image: Some(mock_png_b64.to_string()),
        })
    }

    // --- GetInsightRuleReport ---

    // STUB[no-telemetry]: Insight rule reports are derived from real CloudWatch Logs
    //   ingestion and Contributor Insights evaluation; always returns empty results.
    async fn handle_get_insight_rule_report(
        &self,
        _state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        _body: &Value,
    ) -> MockResponse {
        wire::serialize_get_insight_rule_report_response(&wire::GetInsightRuleReportOutput {
            approximate_unique_count: Some(0),
            contributors: Some(Vec::new()),
            key_labels: Some(Vec::new()),
            metric_datapoints: Some(Vec::new()),
            ..Default::default()
        })
    }

    // --- ListManagedInsightRules / PutManagedInsightRules ---

    async fn handle_list_managed_insight_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let resource_arn = body
            .get("ResourceARN")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let state = state.read().await;
        // Return managed rules associated with this resource ARN
        let managed_rules = state
            .managed_insight_rules
            .iter()
            .filter(|r| resource_arn.is_empty() || r.resource_arn == resource_arn)
            .map(managed_insight_rule_to_model)
            .collect();
        wire::serialize_list_managed_insight_rules_response(&wire::ListManagedInsightRulesOutput {
            managed_rules: Some(managed_rules),
            ..Default::default()
        })
    }

    async fn handle_put_managed_insight_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<CloudWatchState>>,
        body: &Value,
    ) -> MockResponse {
        let managed_rules = body
            .get("ManagedRules")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let mut state = state.write().await;
        for rule in &managed_rules {
            let template_name = rule
                .get("TemplateName")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let resource_arn = rule
                .get("ResourceARN")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let rule_name = format!(
                "{}-{}",
                template_name,
                resource_arn.split(':').next_back().unwrap_or("rule")
            );
            state.managed_insight_rules.push(ManagedInsightRule {
                template_name,
                resource_arn,
                rule_name,
            });
        }
        wire::serialize_put_managed_insight_rules_response(&wire::PutManagedInsightRulesOutput {
            failures: Some(Vec::new()),
        })
    }
}

// --- Alarm serialization helper ---

fn serialize_alarm(a: &crate::types::MetricAlarm) -> Value {
    let dims: Vec<Value> = a
        .dimensions
        .iter()
        .map(|d| json!({"Name": d.name, "Value": d.value}))
        .collect();
    let mut v = json!({
        "AlarmName": a.alarm_name,
        "AlarmArn": a.alarm_arn,
        "MetricName": a.metric_name,
        "Namespace": a.namespace,
        "Threshold": a.threshold,
        "ComparisonOperator": a.comparison_operator,
        "EvaluationPeriods": a.evaluation_periods,
        "Period": a.period,
        "Statistic": a.statistic,
        "StateValue": a.state_value.as_str(),
        "StateReason": a.state_reason,
        "ActionsEnabled": a.actions_enabled,
        "AlarmActions": a.alarm_actions,
        "OKActions": a.ok_actions,
        "InsufficientDataActions": a.insufficient_data_actions,
        "Dimensions": dims,
    });
    if let Some(ref desc) = a.alarm_description {
        v["AlarmDescription"] = json!(desc);
    }
    if let Some(ref unit) = a.unit {
        v["Unit"] = json!(unit);
    }
    v
}

// --- Composite alarm serialization helper ---

fn serialize_composite_alarm(a: &crate::types::CompositeAlarm) -> Value {
    let mut v = json!({
        "AlarmName": a.alarm_name,
        "AlarmArn": a.alarm_arn,
        "AlarmRule": a.alarm_rule,
        "StateValue": a.state_value.as_str(),
        "StateReason": a.state_reason,
        "ActionsEnabled": a.actions_enabled,
        "AlarmActions": a.alarm_actions,
        "OKActions": a.ok_actions,
        "InsufficientDataActions": a.insufficient_data_actions,
    });
    if let Some(ref desc) = a.alarm_description {
        v["AlarmDescription"] = json!(desc);
    }
    v
}

fn metric_to_model(metric: &crate::types::Metric) -> wire::Metric {
    wire::Metric {
        namespace: Some(metric.namespace.clone()),
        metric_name: Some(metric.metric_name.clone()),
        dimensions: Some(metric.dimensions.iter().map(dimension_to_model).collect()),
    }
}

fn dimension_to_model(dimension: &Dimension) -> wire::Dimension {
    wire::Dimension {
        name: dimension.name.clone(),
        value: dimension.value.clone(),
    }
}

fn alarm_to_model(alarm: &crate::types::MetricAlarm) -> wire::MetricAlarm {
    serde_json::from_value(serialize_alarm(alarm)).unwrap_or_default()
}

fn composite_alarm_to_model(alarm: &crate::types::CompositeAlarm) -> wire::CompositeAlarm {
    serde_json::from_value(serialize_composite_alarm(alarm)).unwrap_or_default()
}

fn insight_rule_to_model(rule: &crate::types::InsightRule) -> wire::InsightRule {
    wire::InsightRule {
        name: Some(rule.name.clone()),
        state: Some(rule.state.as_str().to_string()),
        schema: Some(rule.schema.clone()),
        definition: Some(rule.definition.clone()),
        ..Default::default()
    }
}

fn anomaly_detector_to_model(detector: &crate::types::AnomalyDetector) -> wire::AnomalyDetector {
    wire::AnomalyDetector {
        namespace: Some(detector.namespace.clone()),
        metric_name: Some(detector.metric_name.clone()),
        stat: Some(detector.stat.clone()),
        state_value: Some(detector.state_value.clone()),
        dimensions: Some(detector.dimensions.iter().map(dimension_to_model).collect()),
        ..Default::default()
    }
}

fn metric_stream_filter_to_model(
    filter: &crate::types::MetricStreamFilter,
) -> wire::MetricStreamFilter {
    wire::MetricStreamFilter {
        namespace: Some(filter.namespace.clone()),
        metric_names: Some(filter.metric_names.clone()),
    }
}

fn metric_stream_to_get_output(stream: &crate::types::MetricStream) -> wire::GetMetricStreamOutput {
    wire::GetMetricStreamOutput {
        arn: Some(stream.arn.clone()),
        name: Some(stream.name.clone()),
        firehose_arn: Some(stream.firehose_arn.clone()),
        role_arn: Some(stream.role_arn.clone()),
        state: Some(stream.state.clone()),
        output_format: Some(stream.output_format.clone()),
        creation_date: Some(stream.creation_date as f64),
        last_update_date: Some(stream.last_update_date as f64),
        include_filters: Some(
            stream
                .include_filters
                .iter()
                .map(metric_stream_filter_to_model)
                .collect(),
        ),
        exclude_filters: Some(
            stream
                .exclude_filters
                .iter()
                .map(metric_stream_filter_to_model)
                .collect(),
        ),
        ..Default::default()
    }
}

fn metric_stream_to_entry(stream: &crate::types::MetricStream) -> wire::MetricStreamEntry {
    wire::MetricStreamEntry {
        arn: Some(stream.arn.clone()),
        name: Some(stream.name.clone()),
        firehose_arn: Some(stream.firehose_arn.clone()),
        state: Some(stream.state.clone()),
        output_format: Some(stream.output_format.clone()),
        creation_date: Some(stream.creation_date as f64),
        last_update_date: Some(stream.last_update_date as f64),
    }
}

fn alarm_mute_rule_to_output(rule: &crate::types::AlarmMuteRule) -> wire::GetAlarmMuteRuleOutput {
    wire::GetAlarmMuteRuleOutput {
        name: Some(rule.name.clone()),
        alarm_mute_rule_arn: Some(rule.arn.clone()),
        description: rule.description.clone(),
        mute_type: Some(rule.mute_type.clone()),
        mute_targets: Some(wire::MuteTargets {
            alarm_names: rule.alarm_names.clone(),
        }),
        status: Some(rule.status.clone()),
        last_updated_timestamp: Some(rule.last_updated_timestamp as f64),
        ..Default::default()
    }
}

fn alarm_mute_rule_to_summary(rule: &crate::types::AlarmMuteRule) -> wire::AlarmMuteRuleSummary {
    wire::AlarmMuteRuleSummary {
        alarm_mute_rule_arn: Some(rule.arn.clone()),
        mute_type: Some(rule.mute_type.clone()),
        status: Some(rule.status.clone()),
        last_updated_timestamp: Some(rule.last_updated_timestamp as f64),
        ..Default::default()
    }
}

fn managed_insight_rule_to_model(rule: &ManagedInsightRule) -> wire::ManagedRuleDescription {
    wire::ManagedRuleDescription {
        template_name: Some(rule.template_name.clone()),
        resource_a_r_n: Some(rule.resource_arn.clone()),
        rule_state: Some(wire::ManagedRuleState {
            rule_name: Some(rule.rule_name.clone()),
            ..Default::default()
        }),
    }
}

// --- String list extraction helper ---

fn extract_string_list(body: &Value, key: &str) -> Vec<String> {
    body.get(key)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

// --- Tag extraction helper ---

fn extract_tags_from_json(body: &Value) -> std::collections::HashMap<String, String> {
    let mut tags = std::collections::HashMap::new();
    if let Some(arr) = body.get("Tags").and_then(|v| v.as_array()) {
        for tag in arr {
            if let (Some(key), Some(value)) = (
                tag.get("Key").and_then(|v| v.as_str()),
                tag.get("Value").and_then(|v| v.as_str()),
            ) {
                tags.insert(key.to_string(), value.to_string());
            }
        }
    }
    tags
}

// --- Metric stream filter extraction helper ---

fn extract_metric_stream_filters(body: &Value, key: &str) -> Vec<crate::types::MetricStreamFilter> {
    body.get(key)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .map(|item| {
                    let namespace = item
                        .get("Namespace")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let metric_names = item
                        .get("MetricNames")
                        .and_then(|v| v.as_array())
                        .map(|a| {
                            a.iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .unwrap_or_default();
                    crate::types::MetricStreamFilter {
                        namespace,
                        metric_names,
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

// --- CBOR utilities ---

/// Extract action name from URL path.
/// Path format: /service/GraniteServiceVersion20100801/operation/{Action}
fn extract_action_from_path(uri: &str) -> Option<String> {
    let path = extract_path(uri);
    let idx = path.find("/operation/")?;
    let action = &path[idx + "/operation/".len()..];
    let action = action.split('?').next().unwrap_or(action);
    if action.is_empty() {
        None
    } else {
        Some(action.to_string())
    }
}

fn extract_path(uri: &str) -> String {
    let after_scheme = uri
        .strip_prefix("https://")
        .or_else(|| uri.strip_prefix("http://"))
        .unwrap_or(uri);
    let slash_pos = after_scheme.find('/').unwrap_or(after_scheme.len());
    after_scheme[slash_pos..].to_string()
}

/// Create a CBOR response from a JSON value, converting via `wire::json_to_cbor`.
fn cbor_response(status: u16, body: Value) -> MockResponse {
    let cbor_val = wire::json_to_cbor(&body);
    let mut buf = Vec::new();
    ciborium::into_writer(&cbor_val, &mut buf).unwrap();
    MockResponse::cbor(status, buf)
}

fn cw_error_response(err: &CloudWatchError) -> MockResponse {
    let (status, code) = match err {
        CloudWatchError::NamespaceRequired => (400, "InvalidParameterValue"),
        CloudWatchError::InvalidStateValue { .. } => (400, "InvalidParameterValue"),
        CloudWatchError::DashboardNotFound { .. } => (404, "ResourceNotFound"),
        CloudWatchError::AlarmNotFound { .. } => (404, "ResourceNotFound"),
        CloudWatchError::InsightRuleNotFound { .. } => (404, "ResourceNotFoundException"),
        CloudWatchError::ResourceNotFound => (404, "ResourceNotFoundException"),
        CloudWatchError::AnomalyDetectorNotFound { .. } => (404, "ResourceNotFoundException"),
        CloudWatchError::MetricStreamNotFound { .. } => (404, "ResourceNotFoundException"),
        CloudWatchError::AlarmMuteRuleNotFound { .. } => (404, "ResourceNotFoundException"),
    };
    wire::cbor_error_response(status, code, &err.to_string())
}

// --- JSON extraction helpers ---

fn extract_metric_data_from_json(body: &Value) -> Vec<MetricDatum> {
    let mut data = Vec::new();
    if let Some(arr) = body.get("MetricData").and_then(|v| v.as_array()) {
        for item in arr {
            let metric_name = item
                .get("MetricName")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let value = item.get("Value").and_then(|v| v.as_f64());
            let unit = item
                .get("Unit")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let dimensions = extract_dimensions_from_json(item, "Dimensions");

            data.push(MetricDatum {
                namespace: String::new(), // Will be set by put_metric_data
                metric_name,
                dimensions,
                value,
                timestamp: Utc::now(),
                unit,
            });
        }
    }
    data
}

fn extract_dimensions_from_json(body: &Value, key: &str) -> Vec<Dimension> {
    body.get(key)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|d| {
                    let name = d.get("Name")?.as_str()?.to_string();
                    let value = d
                        .get("Value")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    Some(Dimension { name, value })
                })
                .collect()
        })
        .unwrap_or_default()
}

// --- Query protocol support ---

const CW_NAMESPACE: &str = "http://monitoring.amazonaws.com/doc/2010-08-01/";

/// Convert flat AWS query parameters (with `member.N` notation) to nested JSON.
fn query_params_to_json(params: &std::collections::HashMap<String, String>) -> Value {
    // Group params by top-level key and build nested structure
    // e.g. "Dimensions.member.1.Name" → path ["Dimensions", "member", "1", "Name"]
    let mut root = serde_json::Map::new();

    for (key, value) in params {
        if key == "Action" || key == "Version" {
            continue;
        }
        let parts: Vec<&str> = key.split('.').collect();
        insert_nested(&mut root, &parts, value);
    }

    Value::Object(root)
}

/// Recursively insert a value into a JSON map following the dotted path.
/// Handles `member.N` notation by converting to arrays.
fn insert_nested(map: &mut serde_json::Map<String, Value>, parts: &[&str], value: &str) {
    if parts.is_empty() {
        return;
    }
    if parts.len() == 1 {
        // Try to parse as number or boolean, otherwise store as string
        let val = if parts[0].parse::<i64>().is_ok() {
            // Don't convert standalone numeric keys
            Value::String(value.to_string())
        } else if value == "true" {
            Value::Bool(true)
        } else if value == "false" {
            Value::Bool(false)
        } else if let Ok(f) = value.parse::<f64>() {
            serde_json::json!(f)
        } else {
            Value::String(value.to_string())
        };
        map.insert(parts[0].to_string(), val);
        return;
    }

    let key = parts[0];
    let rest = &parts[1..];

    // If next part is "member", this is a list
    if rest.first() == Some(&"member") && rest.len() >= 2 {
        let idx_str = rest[1];
        let idx: usize = idx_str.parse().unwrap_or(1);
        let remaining = &rest[2..];

        let arr = map
            .entry(key.to_string())
            .or_insert_with(|| Value::Array(vec![]));
        if let Value::Array(vec) = arr {
            // Ensure array is large enough
            while vec.len() < idx {
                vec.push(Value::Null);
            }
            if remaining.is_empty() {
                // Simple list member: Foo.member.1=bar
                vec[idx - 1] = Value::String(value.to_string());
            } else {
                // Nested list member: Foo.member.1.Key=bar
                if vec[idx - 1].is_null() {
                    vec[idx - 1] = Value::Object(serde_json::Map::new());
                }
                if let Value::Object(ref mut inner) = vec[idx - 1] {
                    insert_nested(inner, remaining, value);
                }
            }
        }
    } else {
        // Nested object
        let entry = map
            .entry(key.to_string())
            .or_insert_with(|| Value::Object(serde_json::Map::new()));
        if let Value::Object(inner) = entry {
            insert_nested(inner, rest, value);
        }
    }
}

/// Build a query-protocol XML error response for CloudWatch.
fn query_error_response(status: u16, code: &str, message: &str) -> MockResponse {
    winterbaume_core::protocol::aws_query_error_response(status, code, message, CW_NAMESPACE)
}

/// Convert a CBOR-encoded MockResponse to an XML query-protocol response.
fn cbor_response_to_xml(action: &str, response: MockResponse) -> MockResponse {
    if response.status < 200 || response.status >= 300 {
        // Error response: decode CBOR to get error code and message
        if let Ok(cbor_val) = ciborium::from_reader::<ciborium::Value, _>(&response.body[..]) {
            let json_val = wire::cbor_to_json(cbor_val);
            let code = json_val
                .get("__type")
                .and_then(|v| v.as_str())
                .unwrap_or("InternalError");
            let message = json_val
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            return query_error_response(response.status, code, message);
        }
        return query_error_response(response.status, "InternalError", "Unknown error");
    }

    // Success response: decode CBOR body to JSON, convert to XML
    let json_val = if response.body.is_empty() {
        json!({})
    } else {
        match ciborium::from_reader::<ciborium::Value, _>(&response.body[..]) {
            Ok(cbor_val) => cbor_to_json_for_xml(cbor_val),
            Err(_) => json!({}),
        }
    };

    let inner_xml = json_to_xml_inner(&json_val);
    winterbaume_core::protocol::aws_query_response(action, CW_NAMESPACE, &inner_xml)
}

/// Variant of `cbor_to_json` for the awsQuery XML response path.
///
/// CBOR Tag 1 is an epoch-based date/time (RFC 8949 §3.4.2). awsQuery XML
/// expects timestamps as ISO 8601 strings, so we render Tag 1 nodes as
/// RFC 3339 strings rather than stripping the tag and leaving a raw number,
/// which the AWS SDK cannot parse as a date.
fn cbor_to_json_for_xml(cbor: ciborium::Value) -> Value {
    match cbor {
        ciborium::Value::Tag(1, inner) => {
            let secs = match *inner {
                ciborium::Value::Integer(i) => {
                    let n: i128 = i.into();
                    n as f64
                }
                ciborium::Value::Float(f) => f,
                other => return cbor_to_json_for_xml(other),
            };
            let whole = secs.trunc() as i64;
            let nanos = ((secs - secs.trunc()) * 1_000_000_000.0).round() as u32;
            let dt = chrono::DateTime::<Utc>::from_timestamp(whole, nanos).unwrap_or_else(Utc::now);
            Value::String(dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true))
        }
        ciborium::Value::Tag(_, inner) => cbor_to_json_for_xml(*inner),
        ciborium::Value::Array(arr) => {
            Value::Array(arr.into_iter().map(cbor_to_json_for_xml).collect())
        }
        ciborium::Value::Map(map) => {
            let obj = map
                .into_iter()
                .filter_map(|(k, v)| {
                    let key = match k {
                        ciborium::Value::Text(s) => s,
                        _ => return None,
                    };
                    Some((key, cbor_to_json_for_xml(v)))
                })
                .collect();
            Value::Object(obj)
        }
        other => wire::cbor_to_json(other),
    }
}

/// Convert a JSON value to XML elements for AWS query-protocol responses.
fn json_to_xml_inner(val: &Value) -> String {
    use winterbaume_core::protocol::xml_escape;

    match val {
        Value::Object(map) => {
            let mut xml = String::new();
            for (key, value) in map {
                match value {
                    Value::Array(arr) => {
                        xml.push_str(&format!("<{key}>"));
                        for item in arr {
                            xml.push_str("<member>");
                            xml.push_str(&json_to_xml_inner(item));
                            xml.push_str("</member>");
                        }
                        xml.push_str(&format!("</{key}>"));
                    }
                    Value::Object(_) => {
                        xml.push_str(&format!("<{key}>"));
                        xml.push_str(&json_to_xml_inner(value));
                        xml.push_str(&format!("</{key}>"));
                    }
                    Value::String(s) => {
                        xml.push_str(&format!("<{key}>{}</{key}>", xml_escape(s)));
                    }
                    Value::Number(n) => {
                        xml.push_str(&format!("<{key}>{n}</{key}>"));
                    }
                    Value::Bool(b) => {
                        xml.push_str(&format!("<{key}>{b}</{key}>"));
                    }
                    Value::Null => {}
                }
            }
            xml
        }
        Value::String(s) => xml_escape(s),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        _ => String::new(),
    }
}
