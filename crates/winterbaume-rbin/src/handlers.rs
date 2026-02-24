use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path, percent_decode, rest_json_error,
};

use crate::state::{RbinError, RbinState, RuleRecord};
use crate::views::RbinStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct RbinService {
    pub(crate) state: Arc<BackendState<RbinState>>,
    pub(crate) notifier: StateChangeNotifier<RbinStateView>,
}

impl RbinService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for RbinService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for RbinService {
    fn service_name(&self) -> &str {
        "rbin"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://rbin\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<RbinState>>;

impl RbinService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let query = winterbaume_core::extract_query_string(&request.uri);
        let method = request.method.as_str().to_uppercase();

        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(query);

        let segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(percent_decode)
            .collect();
        let segs: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();

        let response = match (method.as_str(), segs.as_slice()) {
            ("POST", ["rules"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_create_rule(&state, &request, labels, &query_map, account_id, &region)
                    .await
            }
            ("POST", ["list-rules"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_list_rules(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["rules", id]) => {
                let labels: &[(&str, &str)] = &[("Identifier", id)];
                self.handle_get_rule(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["rules", id]) => {
                let labels: &[(&str, &str)] = &[("Identifier", id)];
                self.handle_update_rule(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["rules", id]) => {
                let labels: &[(&str, &str)] = &[("Identifier", id)];
                self.handle_delete_rule(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["rules", id, "lock"]) => {
                let labels: &[(&str, &str)] = &[("Identifier", id)];
                self.handle_lock_rule(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["rules", id, "unlock"]) => {
                let labels: &[(&str, &str)] = &[("Identifier", id)];
                self.handle_unlock_rule(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["tags", arn]) => {
                let labels: &[(&str, &str)] = &[("ResourceArn", arn)];
                self.handle_list_tags(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["tags", arn]) => {
                let labels: &[(&str, &str)] = &[("ResourceArn", arn)];
                self.handle_tag_resource(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["tags", arn]) => {
                let labels: &[(&str, &str)] = &[("ResourceArn", arn)];
                self.handle_untag_resource(&state, &request, labels, &query_map)
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

    async fn handle_create_rule(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_rule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.resource_type.is_empty() {
            return rest_json_error(400, "ValidationException", "ResourceType is required");
        }
        let id = format!("rule-{}", uuid::Uuid::new_v4().simple());
        let arn = format!("arn:aws:rbin:{}:{}:rule/{}", region, account_id, id);
        let lock_config = input
            .lock_configuration
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());
        let lock_state = if lock_config.is_some() {
            Some("locked".to_string())
        } else {
            None
        };
        let rule = RuleRecord {
            identifier: id.clone(),
            description: input.description.clone(),
            resource_type: input.resource_type.clone(),
            retention_period: serde_json::to_value(&input.retention_period).ok(),
            resource_tags: input
                .resource_tags
                .as_ref()
                .map(|tags| {
                    tags.iter()
                        .filter_map(|t| serde_json::to_value(t).ok())
                        .collect()
                })
                .unwrap_or_default(),
            exclude_resource_tags: input
                .exclude_resource_tags
                .as_ref()
                .map(|tags| {
                    tags.iter()
                        .filter_map(|t| serde_json::to_value(t).ok())
                        .collect()
                })
                .unwrap_or_default(),
            status: "available".to_string(),
            lock_state,
            lock_configuration: lock_config,
            lock_end_time: None,
            rule_arn: arn,
        };
        let mut state = state.write().await;
        let r = state.create_rule(rule);
        wire::serialize_create_rule_response(&wire::CreateRuleResponse {
            description: r.description.clone(),
            exclude_resource_tags: parse_typed_list(&r.exclude_resource_tags),
            identifier: Some(r.identifier.clone()),
            lock_configuration: r.lock_configuration.as_ref().and_then(parse_value),
            lock_state: r.lock_state.clone(),
            resource_tags: parse_typed_list(&r.resource_tags),
            resource_type: Some(r.resource_type.clone()),
            retention_period: r.retention_period.as_ref().and_then(parse_value),
            rule_arn: Some(r.rule_arn.clone()),
            status: Some(r.status.clone()),
            tags: None,
        })
    }

    async fn handle_get_rule(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_rule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_rule(&input.identifier) {
            Ok(r) => wire::serialize_get_rule_response(&wire::GetRuleResponse {
                description: r.description.clone(),
                exclude_resource_tags: parse_typed_list(&r.exclude_resource_tags),
                identifier: Some(r.identifier.clone()),
                lock_configuration: r.lock_configuration.as_ref().and_then(parse_value),
                lock_end_time: r.lock_end_time,
                lock_state: r.lock_state.clone(),
                resource_tags: parse_typed_list(&r.resource_tags),
                resource_type: Some(r.resource_type.clone()),
                retention_period: r.retention_period.as_ref().and_then(parse_value),
                rule_arn: Some(r.rule_arn.clone()),
                status: Some(r.status.clone()),
            }),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_update_rule(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_rule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let new_description = input.description.clone();
        let new_retention = input
            .retention_period
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok());
        let new_resource_type = input.resource_type.clone();
        let new_resource_tags = input.resource_tags.as_ref().map(|tags| {
            tags.iter()
                .filter_map(|t| serde_json::to_value(t).ok())
                .collect::<Vec<Value>>()
        });
        let new_exclude = input.exclude_resource_tags.as_ref().map(|tags| {
            tags.iter()
                .filter_map(|t| serde_json::to_value(t).ok())
                .collect::<Vec<Value>>()
        });
        let mut state = state.write().await;
        match state.update_rule(&input.identifier, |r| {
            if let Some(d) = new_description {
                r.description = Some(d);
            }
            if let Some(rp) = new_retention {
                r.retention_period = Some(rp);
            }
            if let Some(rt) = new_resource_type {
                r.resource_type = rt;
            }
            if let Some(t) = new_resource_tags {
                r.resource_tags = t;
            }
            if let Some(t) = new_exclude {
                r.exclude_resource_tags = t;
            }
        }) {
            Ok(r) => wire::serialize_update_rule_response(&wire::UpdateRuleResponse {
                description: r.description.clone(),
                exclude_resource_tags: parse_typed_list(&r.exclude_resource_tags),
                identifier: Some(r.identifier.clone()),
                lock_end_time: r.lock_end_time,
                lock_state: r.lock_state.clone(),
                resource_tags: parse_typed_list(&r.resource_tags),
                resource_type: Some(r.resource_type.clone()),
                retention_period: r.retention_period.as_ref().and_then(parse_value),
                rule_arn: Some(r.rule_arn.clone()),
                status: Some(r.status.clone()),
            }),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_delete_rule(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_rule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_rule(&input.identifier) {
            Ok(()) => wire::serialize_delete_rule_response(&wire::DeleteRuleResponse {}),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_rules(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_rules_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let resource_type = if input.resource_type.is_empty() {
            None
        } else {
            Some(input.resource_type.as_str())
        };
        let state = state.read().await;
        let summaries: Vec<wire::RuleSummary> = state
            .list_rules(resource_type)
            .into_iter()
            .map(|r| wire::RuleSummary {
                description: r.description.clone(),
                identifier: Some(r.identifier.clone()),
                lock_state: r.lock_state.clone(),
                retention_period: r.retention_period.as_ref().and_then(parse_value),
                rule_arn: Some(r.rule_arn.clone()),
            })
            .collect();
        wire::serialize_list_rules_response(&wire::ListRulesResponse {
            next_token: None,
            rules: Some(summaries),
        })
    }

    async fn handle_lock_rule(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_lock_rule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let config = match serde_json::to_value(&input.lock_configuration) {
            Ok(v) if !v.is_null() => v,
            _ => {
                return rest_json_error(
                    400,
                    "ValidationException",
                    "LockConfiguration is required",
                );
            }
        };
        let mut state = state.write().await;
        match state.lock_rule(&input.identifier, config) {
            Ok(r) => wire::serialize_lock_rule_response(&wire::LockRuleResponse {
                description: r.description.clone(),
                exclude_resource_tags: parse_typed_list(&r.exclude_resource_tags),
                identifier: Some(r.identifier.clone()),
                lock_configuration: r.lock_configuration.as_ref().and_then(parse_value),
                lock_state: r.lock_state.clone(),
                resource_tags: parse_typed_list(&r.resource_tags),
                resource_type: Some(r.resource_type.clone()),
                retention_period: r.retention_period.as_ref().and_then(parse_value),
                rule_arn: Some(r.rule_arn.clone()),
                status: Some(r.status.clone()),
            }),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_unlock_rule(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_unlock_rule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.unlock_rule(&input.identifier) {
            Ok(r) => wire::serialize_unlock_rule_response(&wire::UnlockRuleResponse {
                description: r.description.clone(),
                exclude_resource_tags: parse_typed_list(&r.exclude_resource_tags),
                identifier: Some(r.identifier.clone()),
                lock_configuration: r.lock_configuration.as_ref().and_then(parse_value),
                lock_end_time: r.lock_end_time,
                lock_state: r.lock_state.clone(),
                resource_tags: parse_typed_list(&r.resource_tags),
                resource_type: Some(r.resource_type.clone()),
                retention_period: r.retention_period.as_ref().and_then(parse_value),
                rule_arn: Some(r.rule_arn.clone()),
                status: Some(r.status.clone()),
            }),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let tags: HashMap<String, String> = input
            .tags
            .iter()
            .map(|t| (t.key.clone(), t.value.clone()))
            .collect();
        if tags.is_empty() {
            return rest_json_error(400, "ValidationException", "Tags is required");
        }
        let mut state = state.write().await;
        state.tag_resource(&input.resource_arn, tags);
        wire::serialize_tag_resource_response(&wire::TagResourceResponse {})
    }

    async fn handle_untag_resource(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        state.untag_resource(&input.resource_arn, &input.tag_keys);
        wire::serialize_untag_resource_response(&wire::UntagResourceResponse {})
    }

    async fn handle_list_tags(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let tags_map = state.list_tags(&input.resource_arn);
        let tags_wire: Vec<wire::Tag> = tags_map
            .into_iter()
            .map(|(k, v)| wire::Tag { key: k, value: v })
            .collect();
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: if tags_wire.is_empty() {
                None
            } else {
                Some(tags_wire)
            },
        })
    }
}

fn parse_value<T: serde::de::DeserializeOwned>(v: &Value) -> Option<T> {
    serde_json::from_value(v.clone()).ok()
}

fn parse_typed_list<T: serde::de::DeserializeOwned>(items: &[Value]) -> Option<Vec<T>> {
    let v: Vec<T> = items
        .iter()
        .filter_map(|x| serde_json::from_value(x.clone()).ok())
        .collect();
    if v.is_empty() { None } else { Some(v) }
}

fn err_response(err: &RbinError) -> MockResponse {
    let (status, error_type) = match err {
        RbinError::NotFound { .. } => (404, "ResourceNotFoundException"),
        RbinError::LockStateError { .. } => (409, "ConflictException"),
        RbinError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"Message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
