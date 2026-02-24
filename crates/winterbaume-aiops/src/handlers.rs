use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path, parse_query_string, percent_decode, rest_json_error,
};

use crate::state::{AIOpsError, AIOpsState};
use crate::types::{CrossAccountConfiguration, EncryptionConfiguration, InvestigationGroup};
use crate::views::AIOpsStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct AIOpsService {
    pub(crate) state: Arc<BackendState<AIOpsState>>,
    pub(crate) notifier: StateChangeNotifier<AIOpsStateView>,
}

impl AIOpsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AIOpsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AIOpsService {
    fn service_name(&self) -> &str {
        "aiops"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://aiops\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl AIOpsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str().to_uppercase();

        if !request.body.is_empty()
            && serde_json::from_slice::<serde_json::Value>(&request.body).is_err()
        {
            return rest_json_error(400, "BadRequestException", "Invalid JSON body");
        }

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();
        let query_string = winterbaume_core::extract_query_string(&request.uri).to_string();
        let query_map: HashMap<String, String> = parse_query_string(&query_string);

        let response = match (method.as_str(), segments.as_slice()) {
            ("POST", ["investigationGroups"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_create_investigation_group(
                    &state, account_id, &region, &request, labels, &query_map,
                )
                .await
            }
            ("GET", ["investigationGroups"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_list_investigation_groups(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["investigationGroups", ident]) => {
                let decoded = percent_decode(ident);
                let labels: &[(&str, &str)] = &[("identifier", decoded.as_str())];
                self.handle_get_investigation_group(&state, &request, labels, &query_map)
                    .await
            }
            ("PATCH", ["investigationGroups", ident]) => {
                let decoded = percent_decode(ident);
                let labels: &[(&str, &str)] = &[("identifier", decoded.as_str())];
                self.handle_update_investigation_group(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["investigationGroups", ident]) => {
                let decoded = percent_decode(ident);
                let labels: &[(&str, &str)] = &[("identifier", decoded.as_str())];
                self.handle_delete_investigation_group(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["investigationGroups", ident, "policy"]) => {
                let decoded = percent_decode(ident);
                let labels: &[(&str, &str)] = &[("identifier", decoded.as_str())];
                self.handle_put_investigation_group_policy(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["investigationGroups", ident, "policy"]) => {
                let decoded = percent_decode(ident);
                let labels: &[(&str, &str)] = &[("identifier", decoded.as_str())];
                self.handle_get_investigation_group_policy(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["investigationGroups", ident, "policy"]) => {
                let decoded = percent_decode(ident);
                let labels: &[(&str, &str)] = &[("identifier", decoded.as_str())];
                self.handle_delete_investigation_group_policy(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["tags", arn]) => {
                let decoded = percent_decode(arn);
                let labels: &[(&str, &str)] = &[("resourceArn", decoded.as_str())];
                self.handle_tag_resource(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["tags", arn]) => {
                let decoded = percent_decode(arn);
                let labels: &[(&str, &str)] = &[("resourceArn", decoded.as_str())];
                self.handle_untag_resource(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["tags", arn]) => {
                let decoded = percent_decode(arn);
                let labels: &[(&str, &str)] = &[("resourceArn", decoded.as_str())];
                self.handle_list_tags_for_resource(&state, &request, labels, &query_map)
                    .await
            }
            // --- Unimplemented operations (auto-generated stubs) ---
            // All operations are implemented!
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

    async fn handle_create_investigation_group(
        &self,
        state: &Arc<tokio::sync::RwLock<AIOpsState>>,
        account_id: &str,
        region: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_investigation_group_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        if input.name.is_empty() {
            return rest_json_error(400, "ValidationException", "name is required");
        }
        if input.role_arn.is_empty() {
            return rest_json_error(400, "ValidationException", "roleArn is required");
        }
        let name = input.name.clone();
        let role_arn = input.role_arn.clone();

        let now = chrono::Utc::now().timestamp();
        let arn = format!(
            "arn:aws:aiops:{}:{}:investigation-group/{}",
            region, account_id, name
        );

        let group = InvestigationGroup {
            name: name.clone(),
            arn: arn.clone(),
            role_arn,
            encryption_configuration: input.encryption_configuration.map(|e| {
                EncryptionConfiguration {
                    r#type: e.r#type,
                    kms_key_id: e.kms_key_id,
                }
            }),
            retention_in_days: input.retention_in_days,
            chatbot_notification_channel: input.chatbot_notification_channel,
            tag_key_boundaries: input.tag_key_boundaries,
            is_cloud_trail_event_history_enabled: input.is_cloud_trail_event_history_enabled,
            cross_account_configurations: input.cross_account_configurations.map(|cs| {
                cs.into_iter()
                    .map(|c| CrossAccountConfiguration {
                        source_role_arn: c.source_role_arn,
                    })
                    .collect()
            }),
            created_by: format!("arn:aws:iam::{}:user/mock-user", account_id),
            created_at: now,
            last_modified_by: format!("arn:aws:iam::{}:user/mock-user", account_id),
            last_modified_at: now,
            policy: None,
            tags: input.tags.unwrap_or_default(),
        };

        let mut state = state.write().await;
        match state.create_investigation_group(group) {
            Ok(_) => {
                let response = wire::CreateInvestigationGroupOutput { arn: Some(arn) };
                wire::serialize_create_investigation_group_response(&response)
            }
            Err(e) => aiops_error_response(&e),
        }
    }

    async fn handle_get_investigation_group(
        &self,
        state: &Arc<tokio::sync::RwLock<AIOpsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_investigation_group_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let identifier = input.identifier.as_str();
        let state = state.read().await;
        match state.get_investigation_group(identifier) {
            Ok(g) => wire::serialize_get_investigation_group_response(&group_to_get_response(g)),
            Err(e) => aiops_error_response(&e),
        }
    }

    async fn handle_update_investigation_group(
        &self,
        state: &Arc<tokio::sync::RwLock<AIOpsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_investigation_group_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let identifier = input.identifier.clone();
        let role_arn = input.role_arn;
        let encryption_configuration =
            input
                .encryption_configuration
                .map(|e| EncryptionConfiguration {
                    r#type: e.r#type,
                    kms_key_id: e.kms_key_id,
                });
        let chatbot_notification_channel = input.chatbot_notification_channel;
        let tag_key_boundaries = input.tag_key_boundaries;
        let is_cloud_trail_event_history_enabled = input.is_cloud_trail_event_history_enabled;
        let cross_account_configurations = input.cross_account_configurations.map(|cs| {
            cs.into_iter()
                .map(|c| CrossAccountConfiguration {
                    source_role_arn: c.source_role_arn,
                })
                .collect()
        });

        let now = chrono::Utc::now().timestamp();
        let mut state = state.write().await;
        match state.update_investigation_group(&identifier, |g| {
            if let Some(v) = role_arn {
                g.role_arn = v;
            }
            if let Some(v) = encryption_configuration {
                g.encryption_configuration = Some(v);
            }
            if let Some(v) = chatbot_notification_channel {
                g.chatbot_notification_channel = Some(v);
            }
            if let Some(v) = tag_key_boundaries {
                g.tag_key_boundaries = Some(v);
            }
            if let Some(v) = is_cloud_trail_event_history_enabled {
                g.is_cloud_trail_event_history_enabled = Some(v);
            }
            if let Some(v) = cross_account_configurations {
                g.cross_account_configurations = Some(v);
            }
            g.last_modified_at = now;
        }) {
            Ok(_) => wire::serialize_update_investigation_group_response(
                &wire::UpdateInvestigationGroupOutput {},
            ),
            Err(e) => aiops_error_response(&e),
        }
    }

    async fn handle_delete_investigation_group(
        &self,
        state: &Arc<tokio::sync::RwLock<AIOpsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_delete_investigation_group_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let identifier = input.identifier.as_str();
        let mut state = state.write().await;
        match state.delete_investigation_group(identifier) {
            Ok(()) => wire::serialize_delete_investigation_group_response(),
            Err(e) => aiops_error_response(&e),
        }
    }

    async fn handle_list_investigation_groups(
        &self,
        state: &Arc<tokio::sync::RwLock<AIOpsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_investigation_groups_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let max_results: usize = input.max_results.map(|v| v as usize).unwrap_or(50);
        let next_token: usize = input
            .next_token
            .as_deref()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);

        let state = state.read().await;
        let all_groups = state.list_investigation_groups();
        let total = all_groups.len();
        let end = (next_token + max_results).min(total);
        let page: Vec<wire::ListInvestigationGroupsModel> = all_groups[next_token..end]
            .iter()
            .map(|g| wire::ListInvestigationGroupsModel {
                arn: Some(g.arn.clone()),
                name: Some(g.name.clone()),
            })
            .collect();
        let next = if end < total {
            Some(end.to_string())
        } else {
            None
        };

        let response = wire::ListInvestigationGroupsOutput {
            investigation_groups: if page.is_empty() { None } else { Some(page) },
            next_token: next,
        };
        wire::serialize_list_investigation_groups_response(&response)
    }

    async fn handle_put_investigation_group_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<AIOpsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_investigation_group_policy_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.policy.is_empty() {
            return rest_json_error(400, "ValidationException", "policy is required");
        }
        let identifier = input.identifier.clone();
        let policy = input.policy;

        let mut state = state.write().await;
        match state.put_policy(&identifier, policy) {
            Ok(arn) => {
                let response = wire::PutInvestigationGroupPolicyResponse {
                    investigation_group_arn: Some(arn),
                };
                wire::serialize_put_investigation_group_policy_response(&response)
            }
            Err(e) => aiops_error_response(&e),
        }
    }

    async fn handle_get_investigation_group_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<AIOpsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_investigation_group_policy_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let identifier = input.identifier.as_str();
        let state = state.read().await;
        match state.get_policy(identifier) {
            Ok((arn, policy)) => {
                let response = wire::GetInvestigationGroupPolicyResponse {
                    investigation_group_arn: Some(arn),
                    policy: Some(policy),
                };
                wire::serialize_get_investigation_group_policy_response(&response)
            }
            Err(e) => aiops_error_response(&e),
        }
    }

    async fn handle_delete_investigation_group_policy(
        &self,
        state: &Arc<tokio::sync::RwLock<AIOpsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_investigation_group_policy_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let identifier = input.identifier.as_str();
        let mut state = state.write().await;
        match state.delete_policy(identifier) {
            Ok(()) => wire::serialize_delete_investigation_group_policy_response(
                &wire::DeleteInvestigationGroupPolicyOutput {},
            ),
            Err(e) => aiops_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AIOpsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tags.is_empty() {
            return rest_json_error(400, "ValidationException", "tags is required");
        }
        let resource_arn = input.resource_arn.clone();
        let tags = input.tags;
        let mut state = state.write().await;
        match state.tag_resource(&resource_arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => aiops_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AIOpsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tag_keys.is_empty() {
            return rest_json_error(400, "ValidationException", "tagKeys is required");
        }
        let resource_arn = input.resource_arn.clone();
        let keys: Vec<String> = input.tag_keys;
        let mut state = state.write().await;
        match state.untag_resource(&resource_arn, &keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => aiops_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AIOpsState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let arn = input.resource_arn.as_str();
        let state = state.read().await;
        match state.list_tags(arn) {
            Ok(tags) => {
                let response = wire::ListTagsForResourceOutput {
                    tags: if tags.is_empty() { None } else { Some(tags) },
                };
                wire::serialize_list_tags_for_resource_response(&response)
            }
            Err(e) => aiops_error_response(&e),
        }
    }
}

fn group_to_get_response(g: &InvestigationGroup) -> wire::GetInvestigationGroupResponse {
    wire::GetInvestigationGroupResponse {
        arn: Some(g.arn.clone()),
        name: Some(g.name.clone()),
        role_arn: Some(g.role_arn.clone()),
        encryption_configuration: g.encryption_configuration.as_ref().map(|e| {
            wire::EncryptionConfiguration {
                r#type: e.r#type.clone(),
                kms_key_id: e.kms_key_id.clone(),
            }
        }),
        retention_in_days: g.retention_in_days,
        chatbot_notification_channel: g.chatbot_notification_channel.clone(),
        is_cloud_trail_event_history_enabled: g.is_cloud_trail_event_history_enabled,
        cross_account_configurations: g.cross_account_configurations.as_ref().map(|cs| {
            cs.iter()
                .map(|c| wire::CrossAccountConfiguration {
                    source_role_arn: c.source_role_arn.clone(),
                })
                .collect()
        }),
        tag_key_boundaries: g.tag_key_boundaries.clone(),
        created_by: Some(g.created_by.clone()),
        created_at: Some(g.created_at),
        last_modified_by: Some(g.last_modified_by.clone()),
        last_modified_at: Some(g.last_modified_at),
    }
}

fn aiops_error_response(err: &AIOpsError) -> MockResponse {
    let (status, error_type) = match err {
        AIOpsError::NotFound { .. } => (404, "ResourceNotFoundException"),
        AIOpsError::AlreadyExists { .. } => (409, "ConflictException"),
        AIOpsError::ResourceNotFound { .. } => (404, "ResourceNotFoundException"),
        AIOpsError::PolicyNotFound { .. } => (404, "ResourceNotFoundException"),
        AIOpsError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
