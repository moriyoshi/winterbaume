use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id, extract_path, extract_query_string, parse_query_string, rest_json_error,
};

use crate::state::{XRayError, XRayState};
use crate::views::XRayStateView;
use crate::wire;

#[allow(clippy::upper_case_acronyms)]
pub struct XRayService {
    pub(crate) state: Arc<BackendState<XRayState>>,
    pub(crate) notifier: StateChangeNotifier<XRayStateView>,
}

impl XRayService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for XRayService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for XRayService {
    fn service_name(&self) -> &str {
        "xray"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://xray\..*\.amazonaws\.com",
            r"https?://xray\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl XRayService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        let path = extract_path(&request.uri);
        let query = extract_query_string(&request.uri);
        let query_map: HashMap<String, String> = parse_query_string(query);
        let method = request.method.as_str();

        let state = self.state.get(account_id, &region);

        let response = match (method, path.as_str()) {
            ("POST", "/TelemetryRecords") => {
                self.handle_put_telemetry_records(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/TraceSegments") | ("POST", "/PutTraceSegments") => {
                self.handle_put_trace_segments(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/CreateGroup") => {
                self.handle_create_group(&state, &request, &[], &query_map, account_id, &region)
                    .await
            }
            ("POST", "/GetGroup") => {
                self.handle_get_group(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/DeleteGroup") => {
                self.handle_delete_group(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/Traces") => {
                self.handle_batch_get_traces(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/CancelTraceRetrieval") => {
                self.handle_cancel_trace_retrieval(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/CreateSamplingRule") => {
                self.handle_create_sampling_rule(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            ("POST", "/DeleteResourcePolicy") => {
                self.handle_delete_resource_policy(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/DeleteSamplingRule") => {
                self.handle_delete_sampling_rule(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/EncryptionConfig") => self.handle_get_encryption_config(&state).await,
            ("POST", "/Groups") => self.handle_get_groups(&state).await,
            ("POST", "/GetIndexingRules") => self.handle_get_indexing_rules(&state).await,
            ("POST", "/Insight") => self.handle_get_insight().await,
            ("POST", "/InsightEvents") => self.handle_get_insight_events().await,
            ("POST", "/InsightImpactGraph") => self.handle_get_insight_impact_graph().await,
            ("POST", "/InsightSummaries") => self.handle_get_insight_summaries().await,
            ("POST", "/GetRetrievedTracesGraph") => {
                self.handle_get_retrieved_traces_graph(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/GetSamplingRules") => self.handle_get_sampling_rules(&state).await,
            ("POST", "/SamplingStatisticSummaries") => {
                self.handle_get_sampling_statistic_summaries(&state).await
            }
            ("POST", "/SamplingTargets") => {
                self.handle_get_sampling_targets(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/ServiceGraph") => {
                self.handle_get_service_graph(&request, &[], &query_map)
                    .await
            }
            ("POST", "/TimeSeriesServiceStatistics") => {
                self.handle_get_time_series_service_statistics(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/TraceGraph") => self.handle_get_trace_graph(&request, &[], &query_map).await,
            ("POST", "/GetTraceSegmentDestination") => {
                self.handle_get_trace_segment_destination(&state).await
            }
            ("POST", "/TraceSummaries") => {
                self.handle_get_trace_summaries(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/ListResourcePolicies") => self.handle_list_resource_policies(&state).await,
            ("POST", "/ListRetrievedTraces") => {
                self.handle_list_retrieved_traces(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/ListTagsForResource") => {
                self.handle_list_tags_for_resource(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/PutEncryptionConfig") => {
                self.handle_put_encryption_config(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/PutResourcePolicy") => {
                self.handle_put_resource_policy(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/StartTraceRetrieval") => {
                self.handle_start_trace_retrieval(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/TagResource") => {
                self.handle_tag_resource(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/UntagResource") => {
                self.handle_untag_resource(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/UpdateGroup") => {
                self.handle_update_group(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/UpdateIndexingRule") => {
                self.handle_update_indexing_rule(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/UpdateSamplingRule") => {
                self.handle_update_sampling_rule(&state, &request, &[], &query_map)
                    .await
            }
            ("POST", "/UpdateTraceSegmentDestination") => {
                self.handle_update_trace_segment_destination(&state, &request, &[], &query_map)
                    .await
            }
            _ => rest_json_error(
                400,
                "InvalidRequestException",
                &format!("Unknown route: {method} {path}"),
            ),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_put_trace_segments(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_trace_segments_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let documents = input.trace_segment_documents;

        let mut state = state.write().await;
        let bad = state.put_trace_segments(documents);

        let unprocessed: Vec<wire::UnprocessedTraceSegment> = bad
            .into_iter()
            .map(|(id, code, msg)| wire::UnprocessedTraceSegment {
                id: if id.is_empty() { None } else { Some(id) },
                error_code: Some(code),
                message: Some(msg),
            })
            .collect();

        wire::serialize_put_trace_segments_response(&wire::PutTraceSegmentsResult {
            unprocessed_trace_segments: if unprocessed.is_empty() {
                None
            } else {
                Some(unprocessed)
            },
        })
    }

    async fn handle_create_group(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let group_name = input.group_name.as_str();
        if group_name.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "GroupName is required");
        }

        let filter_expression = input.filter_expression.as_deref().unwrap_or("");
        let insights_enabled = input
            .insights_configuration
            .as_ref()
            .and_then(|c| c.insights_enabled)
            .unwrap_or(false);
        let notifications_enabled = input
            .insights_configuration
            .as_ref()
            .and_then(|c| c.notifications_enabled)
            .unwrap_or(false);

        let mut state = state.write().await;
        match state.create_group(
            group_name,
            filter_expression,
            insights_enabled,
            notifications_enabled,
            account_id,
            region,
        ) {
            Ok(group) => wire::serialize_create_group_response(&wire::CreateGroupResult {
                group: Some(wire::Group {
                    group_name: Some(group.group_name.clone()),
                    group_a_r_n: Some(group.group_arn.clone()),
                    filter_expression: Some(group.filter_expression.clone()),
                    insights_configuration: group.insights_configuration.as_ref().map(|ic| {
                        wire::InsightsConfiguration {
                            insights_enabled: Some(ic.insights_enabled),
                            notifications_enabled: Some(ic.notifications_enabled),
                        }
                    }),
                }),
            }),
            Err(e) => xray_error_response(&e),
        }
    }

    async fn handle_get_group(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let group_name = input.group_name.as_deref();
        let group_arn = input.group_a_r_n.as_deref();

        let state = state.read().await;
        match state.get_group_by_name_or_arn(group_name, group_arn) {
            Ok(group) => wire::serialize_get_group_response(&wire::GetGroupResult {
                group: Some(wire::Group {
                    group_name: Some(group.group_name.clone()),
                    group_a_r_n: Some(group.group_arn.clone()),
                    filter_expression: Some(group.filter_expression.clone()),
                    insights_configuration: group.insights_configuration.as_ref().map(|ic| {
                        wire::InsightsConfiguration {
                            insights_enabled: Some(ic.insights_enabled),
                            notifications_enabled: Some(ic.notifications_enabled),
                        }
                    }),
                }),
            }),
            Err(e) => xray_error_response(&e),
        }
    }

    async fn handle_put_telemetry_records(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_telemetry_records_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };

        let records: Vec<crate::types::TelemetryRecord> = input
            .telemetry_records
            .into_iter()
            .map(|r| crate::types::TelemetryRecord {
                timestamp: r.timestamp,
                segments_received_count: r.segments_received_count,
                segments_sent_count: r.segments_sent_count,
                segments_rejected_count: r.segments_rejected_count,
                segments_spillover_count: r.segments_spillover_count,
                backend_connection_errors: r.backend_connection_errors.as_ref().map(|_| 0),
            })
            .collect();

        let mut state = state.write().await;
        state.put_telemetry_records(records);
        wire::serialize_put_telemetry_records_response(&wire::PutTelemetryRecordsResult {})
    }

    async fn handle_batch_get_traces(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_batch_get_traces_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let trace_ids = input.trace_ids;

        let state = state.read().await;
        let (found, unprocessed) = state.batch_get_traces(&trace_ids);

        let traces: Vec<wire::Trace> = found
            .into_iter()
            .map(|t| wire::Trace {
                id: Some(t.id),
                duration: Some(t.duration),
                segments: if t.segments.is_empty() {
                    None
                } else {
                    Some(
                        t.segments
                            .into_iter()
                            .map(|s| wire::Segment {
                                id: Some(s.id),
                                document: Some(s.document),
                            })
                            .collect(),
                    )
                },
                ..Default::default()
            })
            .collect();

        wire::serialize_batch_get_traces_response(&wire::BatchGetTracesResult {
            traces: if traces.is_empty() {
                None
            } else {
                Some(traces)
            },
            unprocessed_trace_ids: if unprocessed.is_empty() {
                None
            } else {
                Some(unprocessed)
            },
            ..Default::default()
        })
    }

    async fn handle_cancel_trace_retrieval(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_cancel_trace_retrieval_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let token = input.retrieval_token;
        if token.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "RetrievalToken is required");
        }
        let mut state = state.write().await;
        match state.cancel_trace_retrieval(&token) {
            Ok(()) => wire::serialize_cancel_trace_retrieval_response(
                &wire::CancelTraceRetrievalResult {},
            ),
            Err(e) => xray_error_response(&e),
        }
    }

    async fn handle_create_sampling_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_sampling_rule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let sr = input.sampling_rule;
        let rule_name = sr.rule_name.unwrap_or_default();
        if rule_name.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "RuleName is required");
        }
        let resource_arn = if sr.resource_a_r_n.is_empty() {
            "*"
        } else {
            sr.resource_a_r_n.as_str()
        };
        let priority = if sr.priority == 0 { 1000 } else { sr.priority };
        let fixed_rate = if sr.fixed_rate == 0.0 {
            0.05
        } else {
            sr.fixed_rate
        };
        let reservoir_size = sr.reservoir_size;
        let service_name = if sr.service_name.is_empty() {
            "*"
        } else {
            sr.service_name.as_str()
        };
        let service_type = if sr.service_type.is_empty() {
            "*"
        } else {
            sr.service_type.as_str()
        };
        let host = if sr.host.is_empty() {
            "*"
        } else {
            sr.host.as_str()
        };
        let http_method = if sr.h_t_t_p_method.is_empty() {
            "*"
        } else {
            sr.h_t_t_p_method.as_str()
        };
        let url_path = if sr.u_r_l_path.is_empty() {
            "*"
        } else {
            sr.u_r_l_path.as_str()
        };
        let version = if sr.version == 0 { 1 } else { sr.version };

        let mut state = state.write().await;
        match state.create_sampling_rule(
            &rule_name,
            resource_arn,
            priority,
            fixed_rate,
            reservoir_size,
            service_name,
            service_type,
            host,
            http_method,
            url_path,
            version,
            account_id,
            region,
        ) {
            Ok(rule) => {
                wire::serialize_create_sampling_rule_response(&wire::CreateSamplingRuleResult {
                    sampling_rule_record: Some(wire::SamplingRuleRecord {
                        sampling_rule: Some(wire::SamplingRule {
                            rule_name: Some(rule.rule_name.clone()),
                            rule_a_r_n: Some(rule.rule_arn.clone()),
                            resource_a_r_n: rule.resource_arn.clone(),
                            priority: rule.priority,
                            fixed_rate: rule.fixed_rate,
                            reservoir_size: rule.reservoir_size,
                            service_name: rule.service_name.clone(),
                            service_type: rule.service_type.clone(),
                            host: rule.host.clone(),
                            h_t_t_p_method: rule.http_method.clone(),
                            u_r_l_path: rule.url_path.clone(),
                            version: rule.version,
                            ..Default::default()
                        }),
                        created_at: Some(rule.created_at),
                        modified_at: Some(rule.modified_at),
                    }),
                })
            }
            Err(e) => xray_error_response(&e),
        }
    }

    async fn handle_delete_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_resource_policy_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let policy_name = input.policy_name;
        if policy_name.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "PolicyName is required");
        }
        let policy_revision_id = input.policy_revision_id.unwrap_or_default();

        let mut state = state.write().await;
        match state.delete_resource_policy(&policy_name, &policy_revision_id) {
            Ok(()) => wire::serialize_delete_resource_policy_response(
                &wire::DeleteResourcePolicyResult {},
            ),
            Err(e) => xray_error_response(&e),
        }
    }

    async fn handle_delete_sampling_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_sampling_rule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let rule_name = match input.rule_name {
            Some(n) if !n.is_empty() => n,
            _ => match input.rule_a_r_n {
                Some(arn) if !arn.is_empty() => {
                    arn.split('/').next_back().unwrap_or("").to_string()
                }
                _ => {
                    return rest_json_error(
                        400,
                        "InvalidRequestException",
                        "RuleName or RuleARN is required",
                    );
                }
            },
        };

        let mut state = state.write().await;
        match state.delete_sampling_rule(&rule_name) {
            Ok(rule) => {
                wire::serialize_delete_sampling_rule_response(&wire::DeleteSamplingRuleResult {
                    sampling_rule_record: Some(wire::SamplingRuleRecord {
                        sampling_rule: Some(wire::SamplingRule {
                            rule_name: Some(rule.rule_name.clone()),
                            rule_a_r_n: Some(rule.rule_arn.clone()),
                            resource_a_r_n: rule.resource_arn.clone(),
                            priority: rule.priority,
                            fixed_rate: rule.fixed_rate,
                            reservoir_size: rule.reservoir_size,
                            service_name: rule.service_name.clone(),
                            service_type: rule.service_type.clone(),
                            host: rule.host.clone(),
                            h_t_t_p_method: rule.http_method.clone(),
                            u_r_l_path: rule.url_path.clone(),
                            version: rule.version,
                            ..Default::default()
                        }),
                        created_at: Some(rule.created_at),
                        modified_at: Some(rule.modified_at),
                    }),
                })
            }
            Err(e) => xray_error_response(&e),
        }
    }

    async fn handle_get_encryption_config(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let ec = state.get_encryption_config();
        wire::serialize_get_encryption_config_response(&wire::GetEncryptionConfigResult {
            encryption_config: Some(wire::EncryptionConfig {
                key_id: ec.key_id.clone(),
                status: Some(ec.status.clone()),
                r#type: Some(ec.encryption_type.clone()),
            }),
        })
    }

    async fn handle_get_groups(&self, state: &Arc<tokio::sync::RwLock<XRayState>>) -> MockResponse {
        let state = state.read().await;
        let groups: Vec<wire::GroupSummary> = state
            .groups
            .values()
            .map(|g| wire::GroupSummary {
                group_name: Some(g.group_name.clone()),
                group_a_r_n: Some(g.group_arn.clone()),
                filter_expression: Some(g.filter_expression.clone()),
                insights_configuration: g.insights_configuration.as_ref().map(|ic| {
                    wire::InsightsConfiguration {
                        insights_enabled: Some(ic.insights_enabled),
                        notifications_enabled: Some(ic.notifications_enabled),
                    }
                }),
            })
            .collect();
        wire::serialize_get_groups_response(&wire::GetGroupsResult {
            groups: if groups.is_empty() {
                None
            } else {
                Some(groups)
            },
            next_token: None,
        })
    }

    async fn handle_get_indexing_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let rules: Vec<wire::IndexingRule> = state
            .get_indexing_rules()
            .into_iter()
            .map(|r| wire::IndexingRule {
                name: Some(r.name.clone()),
                modified_at: Some(r.modified_at),
                rule: r
                    .desired_sampling_percentage
                    .map(|pct| wire::IndexingRuleValue {
                        probabilistic: Some(wire::ProbabilisticRuleValue {
                            desired_sampling_percentage: Some(pct),
                            actual_sampling_percentage: Some(pct),
                        }),
                    }),
            })
            .collect();
        wire::serialize_get_indexing_rules_response(&wire::GetIndexingRulesResult {
            indexing_rules: if rules.is_empty() { None } else { Some(rules) },
            ..Default::default()
        })
    }

    // Insights are internally generated by AWS and cannot be created via the API.
    // The mock returns empty results; state restore can populate them if needed.
    async fn handle_get_insight(&self) -> MockResponse {
        wire::serialize_get_insight_response(&wire::GetInsightResult::default())
    }

    // Insight events are internally generated by AWS.
    async fn handle_get_insight_events(&self) -> MockResponse {
        wire::serialize_get_insight_events_response(&wire::GetInsightEventsResult::default())
    }

    // Insight impact graphs are internally generated by AWS.
    async fn handle_get_insight_impact_graph(&self) -> MockResponse {
        wire::serialize_get_insight_impact_graph_response(
            &wire::GetInsightImpactGraphResult::default(),
        )
    }

    // Insight summaries are internally generated by AWS.
    async fn handle_get_insight_summaries(&self) -> MockResponse {
        wire::serialize_get_insight_summaries_response(&wire::GetInsightSummariesResult::default())
    }

    // Retrieved traces graph requires a completed retrieval job.
    async fn handle_get_retrieved_traces_graph(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input =
            match wire::deserialize_get_retrieved_traces_graph_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
            };
        let _state = state.read().await;
        // The graph is derived from trace data; the mock returns an empty graph.
        wire::serialize_get_retrieved_traces_graph_response(
            &wire::GetRetrievedTracesGraphResult::default(),
        )
    }

    async fn handle_get_sampling_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let rules = state.get_sampling_rules();
        let records: Vec<wire::SamplingRuleRecord> = rules
            .iter()
            .map(|r| wire::SamplingRuleRecord {
                sampling_rule: Some(wire::SamplingRule {
                    rule_name: Some(r.rule_name.clone()),
                    rule_a_r_n: Some(r.rule_arn.clone()),
                    resource_a_r_n: r.resource_arn.clone(),
                    priority: r.priority,
                    fixed_rate: r.fixed_rate,
                    reservoir_size: r.reservoir_size,
                    service_name: r.service_name.clone(),
                    service_type: r.service_type.clone(),
                    host: r.host.clone(),
                    h_t_t_p_method: r.http_method.clone(),
                    u_r_l_path: r.url_path.clone(),
                    version: r.version,
                    ..Default::default()
                }),
                created_at: Some(r.created_at),
                modified_at: Some(r.modified_at),
            })
            .collect();
        wire::serialize_get_sampling_rules_response(&wire::GetSamplingRulesResult {
            sampling_rule_records: if records.is_empty() {
                None
            } else {
                Some(records)
            },
            ..Default::default()
        })
    }

    async fn handle_get_sampling_statistic_summaries(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let summaries = state.get_sampling_statistic_summaries();
        let wire_summaries: Vec<wire::SamplingStatisticSummary> = summaries
            .iter()
            .map(|s| wire::SamplingStatisticSummary {
                rule_name: Some(s.rule_name.clone()),
                timestamp: Some(s.timestamp),
                request_count: Some(s.request_count),
                borrow_count: Some(s.borrow_count),
                sampled_count: Some(s.sampled_count),
            })
            .collect();
        wire::serialize_get_sampling_statistic_summaries_response(
            &wire::GetSamplingStatisticSummariesResult {
                sampling_statistic_summaries: if wire_summaries.is_empty() {
                    None
                } else {
                    Some(wire_summaries)
                },
                ..Default::default()
            },
        )
    }

    async fn handle_get_sampling_targets(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_sampling_targets_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };

        // Collect the rule names referenced in the statistics documents
        let rule_names: Vec<String> = input
            .sampling_statistics_documents
            .into_iter()
            .map(|doc| doc.rule_name)
            .collect();

        let state = state.read().await;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        // Build sampling target documents from the stored rules.
        // For each requested rule name, return the rule's fixed_rate and reservoir_size.
        let (targets, unprocessed): (
            Vec<wire::SamplingTargetDocument>,
            Vec<wire::UnprocessedStatistics>,
        ) = rule_names
            .iter()
            .fold((vec![], vec![]), |(mut tgts, mut unproc), rule_name| {
                if let Some(rule) = state.sampling_rules.get(rule_name.as_str()) {
                    tgts.push(wire::SamplingTargetDocument {
                        rule_name: Some(rule.rule_name.clone()),
                        fixed_rate: Some(rule.fixed_rate),
                        reservoir_quota: Some(rule.reservoir_size),
                        reservoir_quota_t_t_l: Some(now + 10.0),
                        interval: Some(10),
                        ..Default::default()
                    });
                } else {
                    unproc.push(wire::UnprocessedStatistics {
                        rule_name: Some(rule_name.clone()),
                        error_code: Some("404".to_string()),
                        message: Some(format!("Sampling rule '{rule_name}' not found")),
                    });
                }
                (tgts, unproc)
            });

        let last_modification = state
            .sampling_rules
            .values()
            .map(|r| r.modified_at)
            .fold(0.0_f64, f64::max);

        wire::serialize_get_sampling_targets_response(&wire::GetSamplingTargetsResult {
            sampling_target_documents: if targets.is_empty() {
                None
            } else {
                Some(targets)
            },
            unprocessed_statistics: if unprocessed.is_empty() {
                None
            } else {
                Some(unprocessed)
            },
            last_rule_modification: Some(if last_modification > 0.0 {
                last_modification
            } else {
                now
            }),
            ..Default::default()
        })
    }

    async fn handle_get_service_graph(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_service_graph_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let start_time = input.start_time;
        let end_time = input.end_time;
        wire::serialize_get_service_graph_response(&wire::GetServiceGraphResult {
            start_time: Some(start_time),
            end_time: Some(end_time),
            services: Some(vec![]),
            ..Default::default()
        })
    }

    // Time series service statistics are derived from trace data by AWS.
    // The mock returns an empty result; the trace data is stored in state.
    async fn handle_get_time_series_service_statistics(
        &self,
        _state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_get_time_series_service_statistics_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        wire::serialize_get_time_series_service_statistics_response(
            &wire::GetTimeSeriesServiceStatisticsResult::default(),
        )
    }

    async fn handle_get_trace_graph(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_trace_graph_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        if input.trace_ids.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "TraceIds is required");
        }
        wire::serialize_get_trace_graph_response(&wire::GetTraceGraphResult {
            services: Some(vec![]),
            ..Default::default()
        })
    }

    async fn handle_get_trace_segment_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let dest = state.get_trace_segment_destination();
        let destination = if dest.destination.is_empty() {
            None
        } else {
            Some(dest.destination.clone())
        };
        wire::serialize_get_trace_segment_destination_response(
            &wire::GetTraceSegmentDestinationResult {
                destination,
                status: Some("ACTIVE".to_string()),
            },
        )
    }

    async fn handle_get_trace_summaries(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_trace_summaries_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };

        let start_time = input.start_time;
        let end_time = input.end_time;

        let state = state.read().await;
        let summaries = state.get_trace_summaries(start_time, end_time);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        let wire_summaries: Vec<wire::TraceSummary> = summaries
            .into_iter()
            .map(|s| wire::TraceSummary {
                id: Some(s.id),
                duration: Some(s.duration),
                has_error: Some(s.has_error),
                has_fault: Some(s.has_fault),
                has_throttle: Some(s.has_throttle),
                is_partial: Some(false),
                ..Default::default()
            })
            .collect();

        let count = wire_summaries.len() as i64;
        wire::serialize_get_trace_summaries_response(&wire::GetTraceSummariesResult {
            approximate_time: Some(now),
            traces_processed_count: Some(count),
            trace_summaries: if wire_summaries.is_empty() {
                None
            } else {
                Some(wire_summaries)
            },
            ..Default::default()
        })
    }

    async fn handle_list_resource_policies(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let policies = state.list_resource_policies();
        let result: Vec<wire::ResourcePolicy> = policies
            .iter()
            .map(|p| wire::ResourcePolicy {
                policy_name: Some(p.policy_name.clone()),
                policy_document: Some(p.policy_document.clone()),
                policy_revision_id: Some(p.policy_revision_id.clone()),
                last_updated_time: Some(p.last_updated_time),
            })
            .collect();
        wire::serialize_list_resource_policies_response(&wire::ListResourcePoliciesResult {
            resource_policies: if result.is_empty() {
                None
            } else {
                Some(result)
            },
            ..Default::default()
        })
    }

    async fn handle_list_retrieved_traces(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_retrieved_traces_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let token = input.retrieval_token;
        if token.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "RetrievalToken is required");
        }
        let state = state.read().await;
        match state.list_retrieved_traces(&token) {
            Ok((segments, status)) => {
                let traces: Vec<wire::RetrievedTrace> = {
                    // Group segments by trace_id
                    let mut by_trace: HashMap<String, Vec<wire::Span>> = HashMap::new();
                    for seg in &segments {
                        by_trace
                            .entry(seg.trace_id.clone())
                            .or_default()
                            .push(wire::Span {
                                id: Some(seg.id.clone()),
                                document: Some(seg.document.clone()),
                            });
                    }
                    by_trace
                        .into_iter()
                        .map(|(trace_id, spans)| {
                            let duration = spans.len() as f64;
                            wire::RetrievedTrace {
                                id: Some(trace_id),
                                duration: Some(duration),
                                spans: if spans.is_empty() { None } else { Some(spans) },
                            }
                        })
                        .collect()
                };
                wire::serialize_list_retrieved_traces_response(&wire::ListRetrievedTracesResult {
                    trace_format: Some("XRAY".to_string()),
                    traces: if traces.is_empty() {
                        None
                    } else {
                        Some(traces)
                    },
                    retrieval_status: Some(status.to_string()),
                    ..Default::default()
                })
            }
            Err(e) => xray_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let arn = input.resource_a_r_n;
        if arn.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "ResourceARN is required");
        }
        let state = state.read().await;
        let pairs = state.list_tags_for_resource(&arn);
        let tags: Vec<wire::Tag> = pairs
            .into_iter()
            .map(|(k, v)| wire::Tag { key: k, value: v })
            .collect();
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: if tags.is_empty() { None } else { Some(tags) },
            next_token: None,
        })
    }

    async fn handle_put_encryption_config(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_encryption_config_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let encryption_type = if input.r#type.is_empty() {
            "NONE".to_string()
        } else {
            input.r#type
        };
        let key_id = input.key_id;
        let mut state = state.write().await;
        state.put_encryption_config(key_id, &encryption_type);
        let ec = state.get_encryption_config();
        wire::serialize_put_encryption_config_response(&wire::PutEncryptionConfigResult {
            encryption_config: Some(wire::EncryptionConfig {
                key_id: ec.key_id.clone(),
                status: Some(ec.status.clone()),
                r#type: Some(ec.encryption_type.clone()),
            }),
        })
    }

    async fn handle_put_resource_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_resource_policy_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let policy_name = input.policy_name;
        if policy_name.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "PolicyName is required");
        }
        let policy_document = input.policy_document;
        if policy_document.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "PolicyDocument is required");
        }
        let policy_revision_id = input.policy_revision_id;

        let mut state = state.write().await;
        match state.put_resource_policy(
            &policy_name,
            &policy_document,
            policy_revision_id.as_deref(),
        ) {
            Ok(policy) => {
                wire::serialize_put_resource_policy_response(&wire::PutResourcePolicyResult {
                    resource_policy: Some(wire::ResourcePolicy {
                        policy_name: Some(policy.policy_name.clone()),
                        policy_document: Some(policy.policy_document.clone()),
                        policy_revision_id: Some(policy.policy_revision_id.clone()),
                        last_updated_time: Some(policy.last_updated_time),
                    }),
                })
            }
            Err(e) => xray_error_response(&e),
        }
    }

    async fn handle_start_trace_retrieval(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_trace_retrieval_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let trace_ids = input.trace_ids;

        let mut state = state.write().await;
        let job = state.start_trace_retrieval(trace_ids);
        wire::serialize_start_trace_retrieval_response(&wire::StartTraceRetrievalResult {
            retrieval_token: Some(job.retrieval_token.clone()),
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let arn = input.resource_a_r_n;
        if arn.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "ResourceARN is required");
        }
        let tags: HashMap<String, String> =
            input.tags.into_iter().map(|t| (t.key, t.value)).collect();
        let mut state = state.write().await;
        state.tag_resource(&arn, tags);
        wire::serialize_tag_resource_response(&wire::TagResourceResponse {})
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let arn = input.resource_a_r_n;
        if arn.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "ResourceARN is required");
        }
        let tag_keys = input.tag_keys;
        let mut state = state.write().await;
        state.untag_resource(&arn, &tag_keys);
        wire::serialize_untag_resource_response(&wire::UntagResourceResponse {})
    }

    async fn handle_update_group(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };

        let group_name = input.group_name.as_deref();
        let group_arn = input.group_a_r_n.as_deref();
        let filter_expression = input.filter_expression.as_deref();
        let insights_enabled = input
            .insights_configuration
            .as_ref()
            .and_then(|c| c.insights_enabled);
        let notifications_enabled = input
            .insights_configuration
            .as_ref()
            .and_then(|c| c.notifications_enabled);

        let mut state = state.write().await;
        match state.update_group(
            group_name,
            group_arn,
            filter_expression,
            insights_enabled,
            notifications_enabled,
        ) {
            Ok(group) => wire::serialize_update_group_response(&wire::UpdateGroupResult {
                group: Some(wire::Group {
                    group_name: Some(group.group_name.clone()),
                    group_a_r_n: Some(group.group_arn.clone()),
                    filter_expression: Some(group.filter_expression.clone()),
                    insights_configuration: group.insights_configuration.as_ref().map(|ic| {
                        wire::InsightsConfiguration {
                            insights_enabled: Some(ic.insights_enabled),
                            notifications_enabled: Some(ic.notifications_enabled),
                        }
                    }),
                }),
            }),
            Err(e) => xray_error_response(&e),
        }
    }

    async fn handle_update_indexing_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_indexing_rule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let name = input.name;
        if name.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "Name is required");
        }
        let desired_pct = input
            .rule
            .probabilistic
            .as_ref()
            .map(|p| p.desired_sampling_percentage);
        let mut state = state.write().await;
        let rule = state.update_indexing_rule(&name, desired_pct);
        wire::serialize_update_indexing_rule_response(&wire::UpdateIndexingRuleResult {
            indexing_rule: Some(wire::IndexingRule {
                name: Some(rule.name.clone()),
                modified_at: Some(rule.modified_at),
                rule: rule
                    .desired_sampling_percentage
                    .map(|pct| wire::IndexingRuleValue {
                        probabilistic: Some(wire::ProbabilisticRuleValue {
                            desired_sampling_percentage: Some(pct),
                            actual_sampling_percentage: Some(pct),
                        }),
                    }),
            }),
        })
    }

    async fn handle_update_sampling_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_sampling_rule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let update = input.sampling_rule_update;
        let rule_name = match update.rule_name {
            Some(n) if !n.is_empty() => n,
            _ => match update.rule_a_r_n.as_deref() {
                Some(arn) if !arn.is_empty() => {
                    arn.split('/').next_back().unwrap_or("").to_string()
                }
                _ => {
                    return rest_json_error(
                        400,
                        "InvalidRequestException",
                        "RuleName or RuleARN is required",
                    );
                }
            },
        };
        let resource_arn = update.resource_a_r_n.as_deref();
        let priority = update.priority;
        let fixed_rate = update.fixed_rate;
        let reservoir_size = update.reservoir_size;
        let service_name = update.service_name.as_deref();
        let service_type = update.service_type.as_deref();
        let host = update.host.as_deref();
        let http_method = update.h_t_t_p_method.as_deref();
        let url_path = update.u_r_l_path.as_deref();

        let mut state = state.write().await;
        match state.update_sampling_rule(
            &rule_name,
            resource_arn,
            priority,
            fixed_rate,
            reservoir_size,
            service_name,
            service_type,
            host,
            http_method,
            url_path,
        ) {
            Ok(rule) => {
                wire::serialize_update_sampling_rule_response(&wire::UpdateSamplingRuleResult {
                    sampling_rule_record: Some(wire::SamplingRuleRecord {
                        sampling_rule: Some(wire::SamplingRule {
                            rule_name: Some(rule.rule_name.clone()),
                            rule_a_r_n: Some(rule.rule_arn.clone()),
                            resource_a_r_n: rule.resource_arn.clone(),
                            priority: rule.priority,
                            fixed_rate: rule.fixed_rate,
                            reservoir_size: rule.reservoir_size,
                            service_name: rule.service_name.clone(),
                            service_type: rule.service_type.clone(),
                            host: rule.host.clone(),
                            h_t_t_p_method: rule.http_method.clone(),
                            u_r_l_path: rule.url_path.clone(),
                            version: rule.version,
                            ..Default::default()
                        }),
                        created_at: Some(rule.created_at),
                        modified_at: Some(rule.modified_at),
                    }),
                })
            }
            Err(e) => xray_error_response(&e),
        }
    }

    async fn handle_update_trace_segment_destination(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_trace_segment_destination_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };
        let destination = input.destination.unwrap_or_default();
        if destination.is_empty() {
            return rest_json_error(400, "InvalidRequestException", "Destination is required");
        }
        let mut state = state.write().await;
        state.update_trace_segment_destination(&destination);
        wire::serialize_update_trace_segment_destination_response(
            &wire::UpdateTraceSegmentDestinationResult {
                destination: Some(destination),
                status: Some("ACTIVE".to_string()),
            },
        )
    }

    async fn handle_delete_group(
        &self,
        state: &Arc<tokio::sync::RwLock<XRayState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_group_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "InvalidRequestException", &e),
        };

        let group_name = input.group_name.as_deref();
        let group_arn = input.group_a_r_n.as_deref();

        let mut state = state.write().await;
        match state.delete_group_by_name_or_arn(group_name, group_arn) {
            Ok(()) => wire::serialize_delete_group_response(&wire::DeleteGroupResult {}),
            Err(e) => xray_error_response(&e),
        }
    }
}

fn xray_error_response(err: &XRayError) -> MockResponse {
    let (status, error_type) = match err {
        XRayError::GroupAlreadyExists { .. } => (400, "InvalidRequestException"),
        XRayError::InvalidGroupArn => (400, "InvalidRequestException"),
        XRayError::MissingGroupIdentifier => (400, "InvalidRequestException"),
        XRayError::GroupNotFound { .. } => (404, "InvalidRequestException"),
        XRayError::InvalidPolicyRevisionId { .. } => (400, "InvalidPolicyRevisionIdException"),
        XRayError::ResourcePolicyNotFound { .. } => (404, "ResourceNotFoundException"),
        XRayError::SamplingRuleAlreadyExists { .. } => (400, "RuleAlreadyExistsException"),
        XRayError::SamplingRuleNotFound { .. } => (404, "InvalidRequestException"),
        XRayError::TraceRetrievalNotFound { .. } => (404, "InvalidRequestException"),
    };
    rest_json_error(status, error_type, &err.to_string())
}
