use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path, extract_query_string, parse_query_string, percent_decode, rest_json_error,
};

use crate::model;
use crate::state::{DlmError, DlmState};
use crate::views::DlmStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct DlmService {
    pub(crate) state: Arc<BackendState<DlmState>>,
    pub(crate) notifier: StateChangeNotifier<DlmStateView>,
}

impl DlmService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for DlmService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for DlmService {
    fn service_name(&self) -> &str {
        "dlm"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://dlm\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<DlmState>>;

impl DlmService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
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

        let segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(percent_decode)
            .collect();
        let segs: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();

        let response = match (method.as_str(), segs.as_slice()) {
            ("POST", ["policies"]) => self.handle_create_policy(&state, &body).await,
            ("GET", ["policies"]) => self.handle_list_policies(&state, &request.uri).await,
            ("GET", ["policies", id]) => self.handle_get_policy(&state, id).await,
            ("PATCH", ["policies", id]) => self.handle_update_policy(&state, id, &body).await,
            ("DELETE", ["policies", id]) => self.handle_delete_policy(&state, id).await,
            ("GET", ["tags", arn]) => self.handle_list_tags(&state, arn).await,
            ("POST", ["tags", arn]) => self.handle_tag_resource(&state, arn, &body).await,
            ("DELETE", ["tags", arn]) => {
                self.handle_untag_resource(&state, arn, &request.uri).await
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

    async fn handle_create_policy(&self, state: &SharedState, body: &Value) -> MockResponse {
        let id = format!("policy-{}", uuid::Uuid::new_v4().simple());
        let now = chrono::Utc::now().to_rfc3339();
        let mut policy: model::LifecyclePolicy =
            serde_json::from_value(body.clone()).unwrap_or_default();
        policy.policy_id = Some(id.clone());
        policy.date_created = Some(now.clone());
        policy.date_modified = Some(now);
        if policy.state.is_none() {
            policy.state = Some("ENABLED".to_string());
        }

        let mut state = state.write().await;
        state.create_policy(policy);
        wire::serialize_create_lifecycle_policy_response(&wire::CreateLifecyclePolicyResponse {
            policy_id: Some(id),
        })
    }

    async fn handle_get_policy(&self, state: &SharedState, id: &str) -> MockResponse {
        let state = state.read().await;
        match state.get_policy(id) {
            Ok(p) => {
                wire::serialize_get_lifecycle_policy_response(&wire::GetLifecyclePolicyResponse {
                    policy: Some(p.clone()),
                })
            }
            Err(e) => err_response(&e),
        }
    }

    async fn handle_update_policy(
        &self,
        state: &SharedState,
        id: &str,
        body: &Value,
    ) -> MockResponse {
        let new_description = body
            .get("Description")
            .and_then(|v| v.as_str())
            .map(String::from);
        let new_state = body.get("State").and_then(|v| v.as_str()).map(String::from);
        let new_role = body
            .get("ExecutionRoleArn")
            .and_then(|v| v.as_str())
            .map(String::from);
        let new_details = body.get("PolicyDetails").cloned();
        let mut state = state.write().await;
        match state.update_policy(id, |p| {
            if let Some(d) = new_description {
                p.description = Some(d);
            }
            if let Some(s) = new_state {
                p.state = Some(s);
            }
            if let Some(r) = new_role {
                p.execution_role_arn = Some(r);
            }
            if let Some(d) = new_details {
                p.policy_details = serde_json::from_value(d).ok();
            }
        }) {
            Ok(_) => wire::serialize_update_lifecycle_policy_response(
                &wire::UpdateLifecyclePolicyResponse {},
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_delete_policy(&self, state: &SharedState, id: &str) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_policy(id) {
            Ok(()) => wire::serialize_delete_lifecycle_policy_response(
                &wire::DeleteLifecyclePolicyResponse {},
            ),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_list_policies(&self, state: &SharedState, _uri: &str) -> MockResponse {
        let state = state.read().await;
        let summaries: Vec<wire::LifecyclePolicySummary> = state
            .list_policies()
            .into_iter()
            .map(|p| wire::LifecyclePolicySummary {
                default_policy: p.default_policy,
                description: p.description.clone(),
                policy_id: p.policy_id.clone(),
                policy_type: p
                    .policy_details
                    .as_ref()
                    .and_then(|d| d.policy_type.clone()),
                state: p.state.clone(),
                tags: if p.tags.as_ref().is_some_and(|t| !t.is_empty()) {
                    p.tags.clone()
                } else {
                    None
                },
            })
            .collect();
        wire::serialize_get_lifecycle_policies_response(&wire::GetLifecyclePoliciesResponse {
            policies: Some(summaries),
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &SharedState,
        arn: &str,
        body: &Value,
    ) -> MockResponse {
        let tags = parse_tag_map(body.get("Tags"));
        if tags.is_empty() {
            return rest_json_error(400, "ValidationException", "Tags is required");
        }
        let mut state = state.write().await;
        state.tag_resource(arn, tags);
        wire::serialize_tag_resource_response(&wire::TagResourceResponse {})
    }

    async fn handle_untag_resource(
        &self,
        state: &SharedState,
        arn: &str,
        uri: &str,
    ) -> MockResponse {
        let qs = parse_query_string(extract_query_string(uri));
        let keys: Vec<String> = qs
            .iter()
            .filter(|(k, _)| *k == "tagKeys")
            .map(|(_, v)| v.clone())
            .collect();
        let mut state = state.write().await;
        state.untag_resource(arn, &keys);
        wire::serialize_untag_resource_response(&wire::UntagResourceResponse {})
    }

    async fn handle_list_tags(&self, state: &SharedState, arn: &str) -> MockResponse {
        let state = state.read().await;
        let tags = state.list_tags(arn);
        wire::serialize_list_tags_for_resource_response(&wire::ListTagsForResourceResponse {
            tags: if tags.is_empty() { None } else { Some(tags) },
        })
    }
}

fn parse_tag_map(v: Option<&Value>) -> HashMap<String, String> {
    v.and_then(|v| v.as_object())
        .map(|m| {
            m.iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                .collect()
        })
        .unwrap_or_default()
}

fn err_response(err: &DlmError) -> MockResponse {
    let (status, error_type) = match err {
        DlmError::PolicyNotFound { .. } => (404, "ResourceNotFoundException"),
        DlmError::ResourceNotFound { .. } => (404, "ResourceNotFoundException"),
        DlmError::Validation { .. } => (400, "InvalidRequestException"),
    };
    let body = json!({"Message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
