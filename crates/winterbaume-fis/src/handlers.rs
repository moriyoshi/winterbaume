use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService, extract_path, extract_query_string, parse_query_string, rest_json_error,
};

use crate::state::{FisError, FisState};
use crate::types::*;
use crate::views::FisStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct FisService {
    pub(crate) state: Arc<BackendState<FisState>>,
    pub(crate) notifier: StateChangeNotifier<FisStateView>,
}

impl FisService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for FisService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for FisService {
    fn service_name(&self) -> &str {
        "fis"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://fis\..*\.amazonaws\.com",
            r"https?://fis\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl FisService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let raw_query = extract_query_string(&request.uri);
        let query_map: HashMap<String, String> = parse_query_string(raw_query);
        let method = request.method.as_str();

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        if segments.is_empty() {
            return rest_json_error(404, "ResourceNotFoundException", "Not found");
        }

        // --- Tags: /tags/{ResourceArn+} ---
        if segments[0] == "tags" && segments.len() >= 2 {
            let resource_arn = decode_resource_arn(&segments[1..].join("/"));
            let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
            let response = match method {
                "GET" => {
                    self.handle_list_tags_for_resource(&state, &request, labels, &query_map)
                        .await
                }
                "POST" => {
                    self.handle_tag_resource(&state, &request, labels, &query_map)
                        .await
                }
                "DELETE" => {
                    self.handle_untag_resource(&state, &request, labels, &query_map, &request.uri)
                        .await
                }
                _ => rest_json_error(404, "ResourceNotFoundException", "Not found"),
            };
            if matches!(method, "POST" | "DELETE") && response.status / 100 == 2 {
                self.notify_state_changed(account_id, &region).await;
            }
            return response;
        }

        let response = match (method, segments.as_slice()) {
            // --- Experiment Templates ---
            ("POST", ["experimentTemplates"]) => {
                self.handle_create_experiment_template(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    &region,
                    account_id,
                )
                .await
            }
            ("GET", ["experimentTemplates"]) => {
                self.handle_list_experiment_templates(&state, &request, &[], &query_map)
                    .await
            }
            ("GET", ["experimentTemplates", id]) => {
                let labels: &[(&str, &str)] = &[("id", id)];
                self.handle_get_experiment_template(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["experimentTemplates", id]) => {
                let labels: &[(&str, &str)] = &[("id", id)];
                self.handle_delete_experiment_template(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["experimentTemplates", id]) => {
                let labels: &[(&str, &str)] = &[("id", id)];
                self.handle_update_experiment_template(&state, &request, labels, &query_map)
                    .await
            }

            _ => rest_json_error(501, "NotImplementedException", "Operation not implemented"),
        };

        // Notify on successful mutations
        if matches!(method, "POST" | "DELETE" | "PATCH" | "PUT") && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }

        response
    }

    // -------------------------------------------------------------------------
    // Experiment Template handlers
    // -------------------------------------------------------------------------

    #[allow(clippy::too_many_arguments)]
    async fn handle_create_experiment_template(
        &self,
        state: &Arc<tokio::sync::RwLock<FisState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_experiment_template_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };

        // Required fields use empty defaults from serde, so check explicitly to
        // preserve historical 400 responses when callers omit them.
        if !body_has_field(&request.body, "description") {
            return rest_json_error(400, "ValidationException", "Missing 'description'");
        }
        if !body_has_field(&request.body, "roleArn") {
            return rest_json_error(400, "ValidationException", "Missing 'roleArn'");
        }
        if !body_has_field(&request.body, "actions") {
            return rest_json_error(400, "ValidationException", "Missing 'actions'");
        }
        if !body_has_field(&request.body, "stopConditions") {
            return rest_json_error(400, "ValidationException", "Missing 'stopConditions'");
        }

        let targets = wire_targets_to_internal(input.targets.as_ref());
        let actions = wire_actions_to_internal(&input.actions);
        let stop_conditions = wire_stop_conditions_to_internal(&input.stop_conditions);
        let tags = input.tags.unwrap_or_default();

        let mut state = state.write().await;
        match state.create_experiment_template(
            &input.description,
            &input.role_arn,
            targets,
            actions,
            stop_conditions,
            tags,
            region,
            account_id,
        ) {
            Ok(template) => wire::serialize_create_experiment_template_response(
                &wire::CreateExperimentTemplateResponse {
                    experiment_template: Some(template_to_wire(template)),
                },
            ),
            Err(e) => fis_error_response(&e),
        }
    }

    async fn handle_get_experiment_template(
        &self,
        state: &Arc<tokio::sync::RwLock<FisState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_experiment_template_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_experiment_template(&input.id) {
            Ok(template) => wire::serialize_get_experiment_template_response(
                &wire::GetExperimentTemplateResponse {
                    experiment_template: Some(template_to_wire(template)),
                },
            ),
            Err(e) => fis_error_response(&e),
        }
    }

    async fn handle_delete_experiment_template(
        &self,
        state: &Arc<tokio::sync::RwLock<FisState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_experiment_template_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let mut state = state.write().await;
        match state.delete_experiment_template(&input.id) {
            Ok(template) => wire::serialize_delete_experiment_template_response(
                &wire::DeleteExperimentTemplateResponse {
                    experiment_template: Some(template_to_wire(&template)),
                },
            ),
            Err(e) => fis_error_response(&e),
        }
    }

    async fn handle_update_experiment_template(
        &self,
        state: &Arc<tokio::sync::RwLock<FisState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_experiment_template_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };

        let description = input.description.as_deref();
        let role_arn = input.role_arn.as_deref();
        let targets = input.targets.as_ref().map(wire_update_targets_to_internal);
        let actions = input.actions.as_ref().map(wire_update_actions_to_internal);
        let stop_conditions = input
            .stop_conditions
            .as_ref()
            .map(|arr| wire_update_stop_conditions_to_internal(arr));

        let mut state = state.write().await;
        match state.update_experiment_template(
            &input.id,
            description,
            role_arn,
            targets,
            actions,
            stop_conditions,
        ) {
            Ok(template) => wire::serialize_update_experiment_template_response(
                &wire::UpdateExperimentTemplateResponse {
                    experiment_template: Some(template_to_wire(template)),
                },
            ),
            Err(e) => fis_error_response(&e),
        }
    }

    async fn handle_list_experiment_templates(
        &self,
        state: &Arc<tokio::sync::RwLock<FisState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_experiment_templates_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let templates: Vec<wire::ExperimentTemplateSummary> = state
            .list_experiment_templates()
            .iter()
            .map(|t| wire::ExperimentTemplateSummary {
                id: Some(t.id.clone()),
                arn: Some(t.arn.clone()),
                description: Some(t.description.clone()),
                creation_time: Some(t.creation_time.timestamp() as f64),
                last_update_time: Some(t.last_update_time.timestamp() as f64),
                tags: if t.tags.is_empty() {
                    None
                } else {
                    Some(t.tags.clone())
                },
            })
            .collect();
        wire::serialize_list_experiment_templates_response(&wire::ListExperimentTemplatesResponse {
            experiment_templates: Some(templates),
            next_token: None,
        })
    }

    // -------------------------------------------------------------------------
    // Tag handlers
    // -------------------------------------------------------------------------

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<FisState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_tags_for_resource(&input.resource_arn) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse {
                    tags: if tags.is_empty() { None } else { Some(tags) },
                },
            ),
            Err(e) => fis_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<FisState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tags.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'tags'");
        }
        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, input.tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => fis_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<FisState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        uri: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        // SDKs often serialise repeating list query parameters as `?tagKeys=k1&tagKeys=k2`
        // rather than a comma-joined value. The wire deserialiser only sees the last value
        // because parse_query_string overwrites duplicate keys, so collect repeated keys
        // directly from the URI to preserve historical behaviour.
        let raw_query = uri.split('?').nth(1).unwrap_or("");
        let mut tag_keys: Vec<String> = raw_query
            .split('&')
            .filter_map(|pair| {
                let (k, v) = pair.split_once('=')?;
                if k == "tagKeys" {
                    Some(decode_resource_arn(v))
                } else {
                    None
                }
            })
            .collect();
        if tag_keys.is_empty() {
            tag_keys = input.tag_keys.unwrap_or_default();
        }

        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => fis_error_response(&e),
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn template_to_wire(t: &ExperimentTemplate) -> wire::ExperimentTemplate {
    wire::ExperimentTemplate {
        id: Some(t.id.clone()),
        arn: Some(t.arn.clone()),
        description: Some(t.description.clone()),
        role_arn: Some(t.role_arn.clone()),
        targets: if t.targets.is_empty() {
            None
        } else {
            Some(
                t.targets
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            wire::ExperimentTemplateTarget {
                                resource_type: Some(v.resource_type.clone()),
                                resource_arns: if v.resource_arns.is_empty() {
                                    None
                                } else {
                                    Some(v.resource_arns.clone())
                                },
                                resource_tags: if v.resource_tags.is_empty() {
                                    None
                                } else {
                                    Some(v.resource_tags.clone())
                                },
                                filters: if v.filters.is_empty() {
                                    None
                                } else {
                                    Some(
                                        v.filters
                                            .iter()
                                            .map(|f| wire::ExperimentTemplateTargetFilter {
                                                path: Some(f.path.clone()),
                                                values: Some(f.values.clone()),
                                            })
                                            .collect(),
                                    )
                                },
                                selection_mode: Some(v.selection_mode.clone()),
                                parameters: if v.parameters.is_empty() {
                                    None
                                } else {
                                    Some(v.parameters.clone())
                                },
                            },
                        )
                    })
                    .collect(),
            )
        },
        actions: if t.actions.is_empty() {
            None
        } else {
            Some(
                t.actions
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            wire::ExperimentTemplateAction {
                                action_id: Some(v.action_id.clone()),
                                description: v.description.clone(),
                                parameters: if v.parameters.is_empty() {
                                    None
                                } else {
                                    Some(v.parameters.clone())
                                },
                                targets: if v.targets.is_empty() {
                                    None
                                } else {
                                    Some(v.targets.clone())
                                },
                                start_after: if v.start_after.is_empty() {
                                    None
                                } else {
                                    Some(v.start_after.clone())
                                },
                            },
                        )
                    })
                    .collect(),
            )
        },
        stop_conditions: if t.stop_conditions.is_empty() {
            None
        } else {
            Some(
                t.stop_conditions
                    .iter()
                    .map(|sc| wire::ExperimentTemplateStopCondition {
                        source: Some(sc.source.clone()),
                        value: sc.value.clone(),
                    })
                    .collect(),
            )
        },
        creation_time: Some(t.creation_time.timestamp() as f64),
        last_update_time: Some(t.last_update_time.timestamp() as f64),
        tags: if t.tags.is_empty() {
            None
        } else {
            Some(t.tags.clone())
        },
        ..Default::default()
    }
}

fn body_has_field(body: &[u8], field: &str) -> bool {
    if body.is_empty() {
        return false;
    }
    let parsed: Result<serde_json::Value, _> = serde_json::from_slice(body);
    match parsed {
        Ok(serde_json::Value::Object(map)) => map.contains_key(field),
        _ => false,
    }
}

fn wire_targets_to_internal(
    targets: Option<&HashMap<String, wire::CreateExperimentTemplateTargetInput>>,
) -> HashMap<String, ExperimentTemplateTarget> {
    targets
        .map(|m| {
            m.iter()
                .map(|(name, t)| {
                    (
                        name.clone(),
                        ExperimentTemplateTarget {
                            resource_type: t.resource_type.clone(),
                            resource_arns: t.resource_arns.clone().unwrap_or_default(),
                            resource_tags: t.resource_tags.clone().unwrap_or_default(),
                            filters: t
                                .filters
                                .as_ref()
                                .map(|fs| {
                                    fs.iter()
                                        .map(|f| ExperimentTemplateTargetFilter {
                                            path: f.path.clone(),
                                            values: f.values.clone(),
                                        })
                                        .collect()
                                })
                                .unwrap_or_default(),
                            selection_mode: if t.selection_mode.is_empty() {
                                "ALL".to_string()
                            } else {
                                t.selection_mode.clone()
                            },
                            parameters: t.parameters.clone().unwrap_or_default(),
                        },
                    )
                })
                .collect()
        })
        .unwrap_or_default()
}

fn wire_update_targets_to_internal(
    targets: &HashMap<String, wire::UpdateExperimentTemplateTargetInput>,
) -> HashMap<String, ExperimentTemplateTarget> {
    targets
        .iter()
        .map(|(name, t)| {
            (
                name.clone(),
                ExperimentTemplateTarget {
                    resource_type: t.resource_type.clone(),
                    resource_arns: t.resource_arns.clone().unwrap_or_default(),
                    resource_tags: t.resource_tags.clone().unwrap_or_default(),
                    filters: t
                        .filters
                        .as_ref()
                        .map(|fs| {
                            fs.iter()
                                .map(|f| ExperimentTemplateTargetFilter {
                                    path: f.path.clone(),
                                    values: f.values.clone(),
                                })
                                .collect()
                        })
                        .unwrap_or_default(),
                    selection_mode: if t.selection_mode.is_empty() {
                        "ALL".to_string()
                    } else {
                        t.selection_mode.clone()
                    },
                    parameters: t.parameters.clone().unwrap_or_default(),
                },
            )
        })
        .collect()
}

fn wire_actions_to_internal(
    actions: &HashMap<String, wire::CreateExperimentTemplateActionInput>,
) -> HashMap<String, ExperimentTemplateAction> {
    actions
        .iter()
        .map(|(name, a)| {
            (
                name.clone(),
                ExperimentTemplateAction {
                    action_id: a.action_id.clone(),
                    description: a.description.clone(),
                    parameters: a.parameters.clone().unwrap_or_default(),
                    targets: a.targets.clone().unwrap_or_default(),
                    start_after: a.start_after.clone().unwrap_or_default(),
                },
            )
        })
        .collect()
}

fn wire_update_actions_to_internal(
    actions: &HashMap<String, wire::UpdateExperimentTemplateActionInputItem>,
) -> HashMap<String, ExperimentTemplateAction> {
    actions
        .iter()
        .map(|(name, a)| {
            (
                name.clone(),
                ExperimentTemplateAction {
                    action_id: a.action_id.clone().unwrap_or_default(),
                    description: a.description.clone(),
                    parameters: a.parameters.clone().unwrap_or_default(),
                    targets: a.targets.clone().unwrap_or_default(),
                    start_after: a.start_after.clone().unwrap_or_default(),
                },
            )
        })
        .collect()
}

fn wire_stop_conditions_to_internal(
    conditions: &[wire::CreateExperimentTemplateStopConditionInput],
) -> Vec<ExperimentTemplateStopCondition> {
    conditions
        .iter()
        .map(|sc| ExperimentTemplateStopCondition {
            source: sc.source.clone(),
            value: sc.value.clone(),
        })
        .collect()
}

fn wire_update_stop_conditions_to_internal(
    conditions: &[wire::UpdateExperimentTemplateStopConditionInput],
) -> Vec<ExperimentTemplateStopCondition> {
    conditions
        .iter()
        .map(|sc| ExperimentTemplateStopCondition {
            source: sc.source.clone(),
            value: sc.value.clone(),
        })
        .collect()
}

fn decode_resource_arn(encoded: &str) -> String {
    encoded
        .replace("%3A", ":")
        .replace("%2F", "/")
        .replace("%3a", ":")
        .replace("%2f", "/")
}

fn fis_error_response(err: &FisError) -> MockResponse {
    let (status, error_type) = match err {
        FisError::ExperimentTemplateNotFound { .. } => (404, "ResourceNotFoundException"),
        FisError::ResourceNotFound { .. } => (404, "ResourceNotFoundException"),
        FisError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({
        "Type": "User",
        "Message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
