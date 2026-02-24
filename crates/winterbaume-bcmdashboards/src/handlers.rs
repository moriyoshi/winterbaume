use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::Value;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    json_error_response,
};

use crate::state::{BcmDashboardsError, BcmDashboardsState};
use crate::types::Dashboard;
use crate::views::BcmDashboardsStateView;
use crate::wire;

pub struct BcmDashboardsService {
    pub(crate) state: Arc<BackendState<BcmDashboardsState>>,
    pub(crate) notifier: StateChangeNotifier<BcmDashboardsStateView>,
}

impl BcmDashboardsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for BcmDashboardsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for BcmDashboardsService {
    fn service_name(&self) -> &str {
        "bcm-dashboards"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://bcm-dashboards\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<BcmDashboardsState>>;

impl BcmDashboardsService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;

        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => return json_error_response(400, "MissingAction", "Missing X-Amz-Target"),
        };

        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return json_error_response(400, "SerializationException", "Invalid JSON body");
        }
        let body_owned: Vec<u8> = if request.body.is_empty() {
            b"{}".to_vec()
        } else {
            request.body.to_vec()
        };
        let body_bytes: &[u8] = &body_owned;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateDashboard" => {
                self.handle_create_dashboard(&state, account_id, &region, body_bytes)
                    .await
            }
            "DeleteDashboard" => self.handle_delete_dashboard(&state, body_bytes).await,
            "GetDashboard" => self.handle_get_dashboard(&state, body_bytes).await,
            "GetResourcePolicy" => self.handle_get_resource_policy(body_bytes).await,
            "ListDashboards" => self.handle_list_dashboards(&state, body_bytes).await,
            "ListTagsForResource" => self.handle_list_tags(&state, body_bytes).await,
            "TagResource" => self.handle_tag_resource(&state, body_bytes).await,
            "UntagResource" => self.handle_untag_resource(&state, body_bytes).await,
            "UpdateDashboard" => self.handle_update_dashboard(&state, body_bytes).await,
            other => json_error_response(
                400,
                "UnknownOperationException",
                &format!("Unknown action: {other}"),
            ),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2
            && matches!(
                action.as_str(),
                "CreateDashboard"
                    | "DeleteDashboard"
                    | "TagResource"
                    | "UntagResource"
                    | "UpdateDashboard"
            )
        {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_dashboard(
        &self,
        state: &SharedState,
        account_id: &str,
        region: &str,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_dashboard_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.name.is_empty() {
            return json_error_response(400, "ValidationException", "name is required");
        }
        let name = input.name.clone();
        let description = input.description.clone();
        let widgets: Vec<Value> = input
            .widgets
            .iter()
            .filter_map(|w| serde_json::to_value(w).ok())
            .collect();
        let resource_tags: HashMap<String, String> = input
            .resource_tags
            .as_deref()
            .map(|tags| {
                tags.iter()
                    .map(|t| (t.key.clone(), t.value.clone()))
                    .collect()
            })
            .unwrap_or_default();
        let now = chrono::Utc::now().timestamp();
        let arn = format!(
            "arn:aws:bcm-dashboards:{}:{}:dashboard/{}",
            region,
            account_id,
            uuid::Uuid::new_v4()
        );
        let dashboard = Dashboard {
            arn: arn.clone(),
            name,
            description,
            r#type: "CUSTOM".to_string(),
            widgets,
            created_at: now,
            updated_at: now,
            tags: resource_tags,
        };

        let mut state = state.write().await;
        match state.create_dashboard(dashboard) {
            Ok(_) => wire::serialize_create_dashboard_response(&wire::CreateDashboardResponse {
                arn: Some(arn),
            }),
            Err(e) => bcm_error_response(&e),
        }
    }

    async fn handle_delete_dashboard(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_delete_dashboard_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.arn.is_empty() {
            return json_error_response(400, "ValidationException", "arn is required");
        }
        let arn = input.arn.clone();
        let mut state = state.write().await;
        match state.delete_dashboard(&arn) {
            Ok(()) => wire::serialize_delete_dashboard_response(&wire::DeleteDashboardResponse {
                arn: Some(arn),
            }),
            Err(e) => bcm_error_response(&e),
        }
    }

    async fn handle_get_dashboard(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_dashboard_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.arn.is_empty() {
            return json_error_response(400, "ValidationException", "arn is required");
        }
        let arn = input.arn.clone();
        let state = state.read().await;
        match state.get_dashboard(&arn) {
            Ok(d) => {
                let widgets: Option<Vec<wire::Widget>> = if d.widgets.is_empty() {
                    None
                } else {
                    Some(
                        d.widgets
                            .iter()
                            .filter_map(|v| serde_json::from_value(v.clone()).ok())
                            .collect(),
                    )
                };
                wire::serialize_get_dashboard_response(&wire::GetDashboardResponse {
                    arn: Some(d.arn.clone()),
                    created_at: Some(d.created_at as f64),
                    description: d.description.clone(),
                    name: Some(d.name.clone()),
                    r#type: Some(d.r#type.clone()),
                    updated_at: Some(d.updated_at as f64),
                    widgets,
                })
            }
            Err(e) => bcm_error_response(&e),
        }
    }

    async fn handle_get_resource_policy(&self, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_get_resource_policy_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "resourceArn is required");
        }
        let resource_arn = input.resource_arn.clone();
        // The mock returns an empty policy document for any valid ARN; AWS Billing
        // dashboards inherit organization-level policies that are not represented here.
        wire::serialize_get_resource_policy_response(&wire::GetResourcePolicyResponse {
            policy_document: Some("{\"Version\":\"2012-10-17\",\"Statement\":[]}".to_string()),
            resource_arn: Some(resource_arn),
        })
    }

    async fn handle_list_dashboards(&self, state: &SharedState, _body: &[u8]) -> MockResponse {
        let state = state.read().await;
        let dashboards: Vec<wire::DashboardReference> = state
            .list_dashboards()
            .into_iter()
            .map(|d| wire::DashboardReference {
                arn: Some(d.arn.clone()),
                created_at: Some(d.created_at as f64),
                description: d.description.clone(),
                name: Some(d.name.clone()),
                r#type: Some(d.r#type.clone()),
                updated_at: Some(d.updated_at as f64),
            })
            .collect();
        wire::serialize_list_dashboards_response(&wire::ListDashboardsResponse {
            dashboards: Some(dashboards),
            next_token: None,
        })
    }

    async fn handle_tag_resource(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "resourceArn is required");
        }
        let arn = input.resource_arn.clone();
        let tags: HashMap<String, String> = input
            .resource_tags
            .iter()
            .map(|t| (t.key.clone(), t.value.clone()))
            .collect();
        if tags.is_empty() {
            return json_error_response(400, "ValidationException", "resourceTags is required");
        }
        let mut state = state.write().await;
        match state.tag_resource(&arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => bcm_error_response(&e),
        }
    }

    async fn handle_untag_resource(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "resourceArn is required");
        }
        let arn = input.resource_arn.clone();
        let keys = input.resource_tag_keys.clone();
        if keys.is_empty() {
            return json_error_response(400, "ValidationException", "resourceTagKeys is required");
        }
        let mut state = state.write().await;
        match state.untag_resource(&arn, &keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => bcm_error_response(&e),
        }
    }

    async fn handle_list_tags(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.resource_arn.is_empty() {
            return json_error_response(400, "ValidationException", "resourceArn is required");
        }
        let arn = input.resource_arn.clone();
        let state = state.read().await;
        match state.list_tags(&arn) {
            Ok(tags) => {
                let resource_tags: Vec<wire::ResourceTag> = tags
                    .into_iter()
                    .map(|(k, v)| wire::ResourceTag { key: k, value: v })
                    .collect();
                wire::serialize_list_tags_for_resource_response(
                    &wire::ListTagsForResourceResponse {
                        resource_tags: if resource_tags.is_empty() {
                            None
                        } else {
                            Some(resource_tags)
                        },
                    },
                )
            }
            Err(e) => bcm_error_response(&e),
        }
    }

    async fn handle_update_dashboard(&self, state: &SharedState, body: &[u8]) -> MockResponse {
        let raw: Value = match serde_json::from_slice(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e.to_string()),
        };
        let input = match wire::deserialize_update_dashboard_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.arn.is_empty() {
            return json_error_response(400, "ValidationException", "arn is required");
        }
        let arn = input.arn.clone();
        let new_name = if raw.get("name").is_some() {
            Some(input.name.clone())
        } else {
            None
        };
        let new_description = input.description.clone();
        let new_widgets = if raw.get("widgets").is_some() {
            Some(
                input
                    .widgets
                    .as_deref()
                    .unwrap_or(&[])
                    .iter()
                    .filter_map(|w| serde_json::to_value(w).ok())
                    .collect::<Vec<Value>>(),
            )
        } else {
            None
        };

        let mut state = state.write().await;
        match state.update_dashboard(&arn, |d| {
            if let Some(n) = new_name {
                d.name = n;
            }
            if let Some(desc) = new_description {
                d.description = Some(desc);
            }
            if let Some(w) = new_widgets {
                d.widgets = w;
            }
        }) {
            Ok(_) => wire::serialize_update_dashboard_response(&wire::UpdateDashboardResponse {
                arn: Some(arn),
            }),
            Err(e) => bcm_error_response(&e),
        }
    }
}

fn bcm_error_response(err: &BcmDashboardsError) -> MockResponse {
    let (status, error_type) = match err {
        BcmDashboardsError::NotFound { .. } => (404, "ResourceNotFoundException"),
        BcmDashboardsError::AlreadyExists { .. } => (409, "ConflictException"),
        BcmDashboardsError::Validation { .. } => (400, "ValidationException"),
    };
    json_error_response(status, error_type, &err.to_string())
}
