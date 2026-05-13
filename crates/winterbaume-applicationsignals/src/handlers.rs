use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, extract_query_string, parse_query_string, percent_decode, rest_json_error,
};

use crate::state::{ApplicationSignalsError, ApplicationSignalsState};
use crate::types::{GroupingConfiguration, ServiceLevelObjective};
use crate::views::ApplicationSignalsStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct ApplicationSignalsService {
    pub(crate) state: Arc<BackendState<ApplicationSignalsState>>,
    pub(crate) notifier: StateChangeNotifier<ApplicationSignalsStateView>,
}

impl ApplicationSignalsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for ApplicationSignalsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for ApplicationSignalsService {
    fn service_name(&self) -> &str {
        "application-signals"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://application-signals\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<ApplicationSignalsState>>;

impl ApplicationSignalsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str().to_uppercase();

        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
            }
        };

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        let response = match (method.as_str(), segments.as_slice()) {
            ("POST", ["budget-report"]) => self.handle_batch_get_budget_report(&state, &body).await,
            ("PATCH", ["exclusion-windows"]) => {
                self.handle_batch_update_exclusion_windows(&state, &body)
                    .await
            }
            ("POST", ["slo"]) => {
                self.handle_create_slo(&state, account_id, &region, &body)
                    .await
            }
            ("DELETE", ["grouping-configuration"]) => {
                self.handle_delete_grouping_configuration(&state).await
            }
            ("PUT", ["grouping-configuration"]) => {
                self.handle_put_grouping_configuration(&state, &body).await
            }
            ("DELETE", ["slo", id]) => self.handle_delete_slo(&state, &percent_decode(id)).await,
            ("GET", ["slo", id]) => self.handle_get_slo(&state, &percent_decode(id)).await,
            ("PATCH", ["slo", id]) => {
                self.handle_update_slo(&state, &percent_decode(id), &body)
                    .await
            }
            ("GET", ["slo", id, "exclusion-windows"]) => {
                self.handle_list_slo_exclusion_windows(&state, &percent_decode(id))
                    .await
            }
            ("POST", ["slos"]) => self.handle_list_slos(&state, &body).await,
            ("POST", ["service"]) => self.handle_get_service(&state, &body).await,
            ("GET", ["services"]) => self.handle_list_services(&state, &request.uri).await,
            ("POST", ["service-dependencies"]) => self.handle_list_dependencies(&state).await,
            ("POST", ["service-dependents"]) => self.handle_list_dependents(&state).await,
            ("POST", ["service-operations"]) => self.handle_list_operations(&state).await,
            ("POST", ["service", "states"]) => self.handle_list_service_states(&state).await,
            ("POST", ["auditFindings"]) => self.handle_list_audit_findings(&state).await,
            ("POST", ["events"]) => self.handle_list_entity_events(&state).await,
            ("POST", ["grouping-attribute-definitions"]) => {
                self.handle_list_grouping_attribute_definitions(&state)
                    .await
            }
            ("POST", ["start-discovery"]) => self.handle_start_discovery(&state).await,
            ("POST", ["tag-resource"]) => self.handle_tag_resource(&state, &body).await,
            ("POST", ["untag-resource"]) => self.handle_untag_resource(&state, &body).await,
            ("GET", ["tags"]) => {
                self.handle_list_tags_for_resource(&state, &request.uri)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2
            && matches!(method.as_str(), "POST" | "PATCH" | "DELETE" | "PUT")
        {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_slo(
        &self,
        state: &SharedState,
        account_id: &str,
        region: &str,
        body: &Value,
    ) -> MockResponse {
        let name = match require_str(body, "Name") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!(
            "arn:aws:application-signals:{}:{}:slo/{}",
            region, account_id, name
        );
        let now = chrono::Utc::now().timestamp();
        let sli = body.get("SliConfig").cloned();
        let request_based_sli = body.get("RequestBasedSliConfig").cloned();
        let metric_source_type = if request_based_sli.is_some() {
            "RequestBasedServiceOperation"
        } else {
            "ServiceOperation"
        };

        let slo = ServiceLevelObjective {
            id: id.clone(),
            arn: arn.clone(),
            name,
            description: opt_str(body, "Description"),
            evaluation_type: "PeriodBased".to_string(),
            metric_source_type: metric_source_type.to_string(),
            created_time: now,
            last_updated_time: now,
            sli,
            request_based_sli,
            goal: body.get("Goal").cloned(),
            burn_rate_configurations: parse_value_list(body.get("BurnRateConfigurations")),
            exclusion_windows: vec![],
            tags: parse_tag_list(body.get("Tags")),
        };

        let mut state = state.write().await;
        match state.create_slo(slo) {
            Ok(s) => wire::serialize_create_service_level_objective_response(
                &wire::CreateServiceLevelObjectiveOutput {
                    slo: Some(slo_to_wire(s)),
                },
            ),
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_get_slo(&self, state: &SharedState, id: &str) -> MockResponse {
        let state = state.read().await;
        match state.get_slo(id) {
            Ok(s) => wire::serialize_get_service_level_objective_response(
                &wire::GetServiceLevelObjectiveOutput {
                    slo: Some(slo_to_wire(s)),
                },
            ),
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_update_slo(&self, state: &SharedState, id: &str, body: &Value) -> MockResponse {
        let new_description = opt_str(body, "Description");
        let new_goal = body.get("Goal").cloned();
        let new_sli = body.get("SliConfig").cloned();
        let new_request_sli = body.get("RequestBasedSliConfig").cloned();
        let new_burn_rate = body.get("BurnRateConfigurations").map(parse_value_list_v);

        let mut state = state.write().await;
        match state.update_slo(id, |s| {
            if let Some(d) = new_description {
                s.description = Some(d);
            }
            if let Some(g) = new_goal {
                s.goal = Some(g);
            }
            if let Some(sli) = new_sli {
                s.sli = Some(sli);
                s.request_based_sli = None;
            } else if let Some(req) = new_request_sli {
                s.request_based_sli = Some(req);
                s.sli = None;
            }
            if let Some(b) = new_burn_rate {
                s.burn_rate_configurations = b;
            }
        }) {
            Ok(s) => wire::serialize_update_service_level_objective_response(
                &wire::UpdateServiceLevelObjectiveOutput {
                    slo: Some(slo_to_wire(s)),
                },
            ),
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_delete_slo(&self, state: &SharedState, id: &str) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_slo(id) {
            Ok(()) => wire::serialize_delete_service_level_objective_response(
                &wire::DeleteServiceLevelObjectiveOutput {},
            ),
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_list_slos(&self, state: &SharedState, _body: &Value) -> MockResponse {
        let state = state.read().await;
        let summaries: Vec<wire::ServiceLevelObjectiveSummary> = state
            .list_slos()
            .into_iter()
            .map(|s| wire::ServiceLevelObjectiveSummary {
                arn: Some(s.arn.clone()),
                created_time: Some(s.created_time as f64),
                dependency_config: None,
                evaluation_type: Some(s.evaluation_type.clone()),
                key_attributes: None,
                metric_source_type: Some(s.metric_source_type.clone()),
                name: Some(s.name.clone()),
                operation_name: None,
            })
            .collect();
        wire::serialize_list_service_level_objectives_response(
            &wire::ListServiceLevelObjectivesOutput {
                next_token: None,
                slo_summaries: Some(summaries),
            },
        )
    }

    async fn handle_list_slo_exclusion_windows(
        &self,
        state: &SharedState,
        id: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_slo(id) {
            Ok(s) => {
                let windows: Vec<wire::ExclusionWindow> = s
                    .exclusion_windows
                    .iter()
                    .filter_map(|v| serde_json::from_value(v.clone()).ok())
                    .collect();
                wire::serialize_list_service_level_objective_exclusion_windows_response(
                    &wire::ListServiceLevelObjectiveExclusionWindowsOutput {
                        exclusion_windows: Some(windows),
                        next_token: None,
                    },
                )
            }
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_batch_update_exclusion_windows(
        &self,
        state: &SharedState,
        body: &Value,
    ) -> MockResponse {
        let slo_ids = body
            .get("SloIds")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|x| x.as_str().map(String::from))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let add_windows = body
            .get("AddExclusionWindows")
            .map(parse_value_list_v)
            .unwrap_or_default();
        let remove_windows = body
            .get("RemoveExclusionWindows")
            .map(parse_value_list_v)
            .unwrap_or_default();

        if slo_ids.is_empty() {
            return rest_json_error(400, "ValidationException", "SloIds is required");
        }

        let mut state = state.write().await;
        let mut errors: Vec<wire::BatchUpdateExclusionWindowsError> = vec![];
        let mut success_arns: Vec<String> = vec![];
        for id in &slo_ids {
            let result = state.update_slo(id, |s| {
                for w in &add_windows {
                    s.exclusion_windows.push(w.clone());
                }
                if !remove_windows.is_empty() {
                    s.exclusion_windows.retain(|w| !remove_windows.contains(w));
                }
            });
            match result {
                Ok(s) => success_arns.push(s.arn.clone()),
                Err(_) => {
                    errors.push(wire::BatchUpdateExclusionWindowsError {
                        error_code: Some("ResourceNotFoundException".to_string()),
                        error_message: Some(format!("SLO {id} not found")),
                        slo_id: Some(id.clone()),
                    });
                }
            }
        }
        wire::serialize_batch_update_exclusion_windows_response(
            &wire::BatchUpdateExclusionWindowsOutput {
                errors: if errors.is_empty() {
                    None
                } else {
                    Some(errors)
                },
                slo_ids: Some(success_arns),
            },
        )
    }

    async fn handle_batch_get_budget_report(
        &self,
        state: &SharedState,
        body: &Value,
    ) -> MockResponse {
        let now = body
            .get("Timestamp")
            .and_then(|v| v.as_f64())
            .unwrap_or_else(|| chrono::Utc::now().timestamp() as f64);
        let identifiers: Vec<String> = body
            .get("SloIds")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|x| x.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let state = state.read().await;
        let mut reports: Vec<wire::ServiceLevelObjectiveBudgetReport> = vec![];
        let mut errors: Vec<wire::ServiceLevelObjectiveBudgetReportError> = vec![];
        for id in identifiers {
            match state.get_slo(&id) {
                Ok(s) => reports.push(wire::ServiceLevelObjectiveBudgetReport {
                    arn: Some(s.arn.clone()),
                    attainment: None,
                    budget_requests_remaining: None,
                    budget_seconds_remaining: None,
                    budget_status: Some("OK".to_string()),
                    evaluation_type: Some(s.evaluation_type.clone()),
                    goal: s.goal.as_ref().and_then(parse_value),
                    name: Some(s.name.clone()),
                    request_based_sli: s.request_based_sli.as_ref().and_then(parse_value),
                    sli: s.sli.as_ref().and_then(parse_value),
                    total_budget_requests: None,
                    total_budget_seconds: None,
                }),
                Err(_) => errors.push(wire::ServiceLevelObjectiveBudgetReportError {
                    arn: Some(id.clone()),
                    error_code: Some("ResourceNotFoundException".to_string()),
                    error_message: Some(format!("SLO {id} not found")),
                    name: Some(id),
                }),
            }
        }
        wire::serialize_batch_get_service_level_objective_budget_report_response(
            &wire::BatchGetServiceLevelObjectiveBudgetReportOutput {
                errors: Some(errors),
                reports: Some(reports),
                timestamp: Some(now),
            },
        )
    }

    async fn handle_get_service(&self, state: &SharedState, body: &Value) -> MockResponse {
        let key_attrs = body
            .get("KeyAttributes")
            .and_then(|v| v.as_object())
            .map(|m| {
                m.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect::<HashMap<String, String>>()
            })
            .unwrap_or_default();
        let now = chrono::Utc::now().timestamp() as f64;
        let start_time = body
            .get("StartTime")
            .and_then(|v| v.as_f64())
            .unwrap_or(now - 3600.0);
        let end_time = body.get("EndTime").and_then(|v| v.as_f64()).unwrap_or(now);
        let state = state.read().await;
        let service = state
            .services
            .iter()
            .find(|s| s.key_attributes == key_attrs)
            .map(|s| wire::Service {
                attribute_maps: Some(s.attribute_maps.clone()),
                key_attributes: Some(s.key_attributes.clone()),
                log_group_references: Some(s.log_group_references.clone()),
                metric_references: parse_value_list_to_typed(&s.metric_references),
                service_groups: parse_value_list_to_typed(&s.service_groups),
            });
        if service.is_none() {
            return rest_json_error(404, "ResourceNotFoundException", "Service not found");
        }
        wire::serialize_get_service_response(&wire::GetServiceOutput {
            end_time: Some(end_time),
            log_group_references: None,
            service,
            start_time: Some(start_time),
        })
    }

    async fn handle_list_services(&self, state: &SharedState, uri: &str) -> MockResponse {
        let qs = parse_query_string(extract_query_string(uri));
        let now = chrono::Utc::now().timestamp() as f64;
        let start_time = qs
            .get("StartTime")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(now - 3600.0);
        let end_time = qs
            .get("EndTime")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(now);
        let state = state.read().await;
        let summaries: Vec<wire::ServiceSummary> = state
            .services
            .iter()
            .map(|s| wire::ServiceSummary {
                attribute_maps: Some(s.attribute_maps.clone()),
                key_attributes: Some(s.key_attributes.clone()),
                metric_references: parse_value_list_to_typed(&s.metric_references),
                service_groups: parse_value_list_to_typed(&s.service_groups),
            })
            .collect();
        wire::serialize_list_services_response(&wire::ListServicesOutput {
            end_time: Some(end_time),
            next_token: None,
            service_summaries: Some(summaries),
            start_time: Some(start_time),
        })
    }

    // STUB[no-telemetry]: Service dependencies are derived from real infrastructure telemetry; the mock has no such data.
    async fn handle_list_dependencies(&self, _state: &SharedState) -> MockResponse {
        let now = chrono::Utc::now().timestamp() as f64;
        wire::serialize_list_service_dependencies_response(&wire::ListServiceDependenciesOutput {
            end_time: Some(now),
            next_token: None,
            service_dependencies: Some(vec![]),
            start_time: Some(now - 3600.0),
        })
    }

    // STUB[no-telemetry]: Service dependents are derived from real infrastructure telemetry; the mock has no such data.
    async fn handle_list_dependents(&self, _state: &SharedState) -> MockResponse {
        let now = chrono::Utc::now().timestamp() as f64;
        wire::serialize_list_service_dependents_response(&wire::ListServiceDependentsOutput {
            end_time: Some(now),
            next_token: None,
            service_dependents: Some(vec![]),
            start_time: Some(now - 3600.0),
        })
    }

    // STUB[no-telemetry]: Service operations are derived from real infrastructure telemetry; the mock has no such data.
    async fn handle_list_operations(&self, _state: &SharedState) -> MockResponse {
        let now = chrono::Utc::now().timestamp() as f64;
        wire::serialize_list_service_operations_response(&wire::ListServiceOperationsOutput {
            end_time: Some(now),
            next_token: None,
            service_operations: Some(vec![]),
            start_time: Some(now - 3600.0),
        })
    }

    // STUB[no-telemetry]: Service states are derived from real infrastructure telemetry; the mock has no such data.
    async fn handle_list_service_states(&self, _state: &SharedState) -> MockResponse {
        let now = chrono::Utc::now().timestamp() as f64;
        wire::serialize_list_service_states_response(&wire::ListServiceStatesOutput {
            end_time: Some(now),
            next_token: None,
            service_states: Some(vec![]),
            start_time: Some(now - 3600.0),
        })
    }

    // STUB[no-telemetry]: Audit findings are derived from real infrastructure telemetry; the mock has no such data.
    async fn handle_list_audit_findings(&self, _state: &SharedState) -> MockResponse {
        let now = chrono::Utc::now().timestamp() as f64;
        wire::serialize_list_audit_findings_response(&wire::ListAuditFindingsOutput {
            audit_findings: Some(vec![]),
            end_time: Some(now),
            next_token: None,
            start_time: Some(now - 3600.0),
        })
    }

    // STUB[no-telemetry]: Entity events are derived from real infrastructure telemetry; the mock has no such data.
    async fn handle_list_entity_events(&self, _state: &SharedState) -> MockResponse {
        let now = chrono::Utc::now().timestamp() as f64;
        wire::serialize_list_entity_events_response(&wire::ListEntityEventsOutput {
            change_events: Some(vec![]),
            end_time: Some(now),
            next_token: None,
            start_time: Some(now - 3600.0),
        })
    }

    async fn handle_list_grouping_attribute_definitions(
        &self,
        state: &SharedState,
    ) -> MockResponse {
        let state = state.read().await;
        let defs = state
            .grouping_configuration
            .as_ref()
            .map(|g| {
                g.grouping_attribute_definitions
                    .iter()
                    .filter_map(|v| serde_json::from_value(v.clone()).ok())
                    .collect::<Vec<wire::GroupingAttributeDefinition>>()
            })
            .unwrap_or_default();
        wire::serialize_list_grouping_attribute_definitions_response(
            &wire::ListGroupingAttributeDefinitionsOutput {
                grouping_attribute_definitions: Some(defs),
                next_token: None,
                updated_at: state
                    .grouping_configuration
                    .as_ref()
                    .map(|g| g.updated_at as f64),
            },
        )
    }

    async fn handle_put_grouping_configuration(
        &self,
        state: &SharedState,
        body: &Value,
    ) -> MockResponse {
        let defs = parse_value_list(body.get("GroupingAttributeDefinitions"));
        let mut state = state.write().await;
        let now = chrono::Utc::now().timestamp();
        state.grouping_configuration = Some(GroupingConfiguration {
            grouping_attribute_definitions: defs.clone(),
            updated_at: now,
        });
        let typed_defs: Vec<wire::GroupingAttributeDefinition> = defs
            .iter()
            .filter_map(|v| serde_json::from_value(v.clone()).ok())
            .collect();
        wire::serialize_put_grouping_configuration_response(&wire::PutGroupingConfigurationOutput {
            grouping_configuration: Some(wire::GroupingConfiguration {
                grouping_attribute_definitions: Some(typed_defs),
                updated_at: Some(now as f64),
            }),
        })
    }

    async fn handle_delete_grouping_configuration(&self, state: &SharedState) -> MockResponse {
        let mut state = state.write().await;
        state.grouping_configuration = None;
        wire::serialize_delete_grouping_configuration_response(
            &wire::DeleteGroupingConfigurationOutput {},
        )
    }

    async fn handle_start_discovery(&self, state: &SharedState) -> MockResponse {
        let mut state = state.write().await;
        state.discovery_started = true;
        wire::serialize_start_discovery_response(&wire::StartDiscoveryOutput {})
    }

    async fn handle_tag_resource(&self, state: &SharedState, body: &Value) -> MockResponse {
        let arn = match require_str(body, "ResourceArn") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let tags = parse_tag_list(body.get("Tags"));
        if tags.is_empty() {
            return rest_json_error(400, "ValidationException", "Tags is required");
        }
        let mut state = state.write().await;
        match state.tag_resource(&arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_untag_resource(&self, state: &SharedState, body: &Value) -> MockResponse {
        let arn = match require_str(body, "ResourceArn") {
            Ok(v) => v,
            Err(r) => return r,
        };
        let keys: Vec<String> = body
            .get("TagKeys")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        if keys.is_empty() {
            return rest_json_error(400, "ValidationException", "TagKeys is required");
        }
        let mut state = state.write().await;
        match state.untag_resource(&arn, &keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => app_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(&self, state: &SharedState, uri: &str) -> MockResponse {
        let qs = parse_query_string(extract_query_string(uri));
        let arn = match qs.get("ResourceArn") {
            Some(s) if !s.is_empty() => s.clone(),
            _ => {
                return rest_json_error(400, "ValidationException", "ResourceArn is required");
            }
        };
        let state = state.read().await;
        match state.list_tags(&arn) {
            Ok(tags) => {
                let tags_wire: Vec<wire::Tag> = tags
                    .into_iter()
                    .map(|(k, v)| wire::Tag { key: k, value: v })
                    .collect();
                wire::serialize_list_tags_for_resource_response(
                    &wire::ListTagsForResourceResponse {
                        tags: if tags_wire.is_empty() {
                            None
                        } else {
                            Some(tags_wire)
                        },
                    },
                )
            }
            Err(e) => app_error_response(&e),
        }
    }
}

fn slo_to_wire(s: &ServiceLevelObjective) -> wire::ServiceLevelObjective {
    wire::ServiceLevelObjective {
        arn: Some(s.arn.clone()),
        burn_rate_configurations: parse_value_list_to_typed(&s.burn_rate_configurations),
        created_time: Some(s.created_time as f64),
        description: s.description.clone(),
        evaluation_type: Some(s.evaluation_type.clone()),
        goal: s.goal.as_ref().and_then(parse_value),
        last_updated_time: Some(s.last_updated_time as f64),
        metric_source_type: Some(s.metric_source_type.clone()),
        name: Some(s.name.clone()),
        request_based_sli: s.request_based_sli.as_ref().and_then(parse_value),
        sli: s.sli.as_ref().and_then(parse_value),
    }
}

fn parse_value_list_to_typed<T: serde::de::DeserializeOwned>(items: &[Value]) -> Option<Vec<T>> {
    let v: Vec<T> = items
        .iter()
        .filter_map(|x| serde_json::from_value(x.clone()).ok())
        .collect();
    if v.is_empty() { None } else { Some(v) }
}

fn require_str(body: &Value, field: &str) -> Result<String, MockResponse> {
    body.get(field)
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from)
        .ok_or_else(|| rest_json_error(400, "ValidationException", &format!("{field} is required")))
}

fn opt_str(body: &Value, field: &str) -> Option<String> {
    body.get(field).and_then(|v| v.as_str()).map(String::from)
}

fn parse_value_list(v: Option<&Value>) -> Vec<Value> {
    v.and_then(|v| v.as_array())
        .map(|a| a.to_vec())
        .unwrap_or_default()
}

fn parse_value_list_v(v: &Value) -> Vec<Value> {
    parse_value_list(Some(v))
}

fn parse_value<T: serde::de::DeserializeOwned>(v: &Value) -> Option<T> {
    serde_json::from_value(v.clone()).ok()
}

fn parse_tag_list(v: Option<&Value>) -> HashMap<String, String> {
    v.and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|x| {
                    let key = x.get("Key")?.as_str()?.to_string();
                    let value = x.get("Value")?.as_str()?.to_string();
                    Some((key, value))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn app_error_response(err: &ApplicationSignalsError) -> MockResponse {
    let (status, error_type) = match err {
        ApplicationSignalsError::SloNotFound { .. } => (404, "ResourceNotFoundException"),
        ApplicationSignalsError::SloAlreadyExists { .. } => (409, "ConflictException"),
        ApplicationSignalsError::ResourceNotFound { .. } => (404, "ResourceNotFoundException"),
        ApplicationSignalsError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"Message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
