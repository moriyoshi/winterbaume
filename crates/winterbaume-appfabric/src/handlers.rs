use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path, extract_query_string, percent_decode, rest_json_error,
};

use crate::state::{AppFabricError, AppFabricState};
use crate::types::AppBundle;
use crate::views::AppFabricStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct AppFabricService {
    pub(crate) state: Arc<BackendState<AppFabricState>>,
    pub(crate) notifier: StateChangeNotifier<AppFabricStateView>,
}

impl AppFabricService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AppFabricService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AppFabricService {
    fn service_name(&self) -> &str {
        "appfabric"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://appfabric\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl AppFabricService {
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
            .map(percent_decode)
            .collect();
        let segs: Vec<&str> = segments.iter().map(String::as_str).collect();

        let response = match (method.as_str(), segs.as_slice()) {
            ("POST", ["appbundles"]) => {
                self.handle_create_app_bundle(&state, account_id, &region, &body)
                    .await
            }
            ("GET", ["appbundles"]) => self.handle_list_app_bundles(&state).await,
            ("GET", ["appbundles", id]) => self.handle_get_app_bundle(&state, id).await,
            ("DELETE", ["appbundles", id]) => self.handle_delete_app_bundle(&state, id).await,
            ("POST", ["tags", arn]) => self.handle_tag_resource(&state, arn, &body).await,
            ("DELETE", ["tags", arn]) => {
                self.handle_untag_resource(&state, arn, &request.uri).await
            }
            ("GET", ["tags", arn]) => self.handle_list_tags(&state, arn).await,
            _ => rest_json_error(
                501,
                "InternalServerException",
                &format!("Operation not implemented: {} {}", method, path),
            ),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2
            && matches!(method.as_str(), "POST" | "DELETE" | "PATCH" | "PUT")
        {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_app_bundle(
        &self,
        state: &Arc<tokio::sync::RwLock<AppFabricState>>,
        account_id: &str,
        region: &str,
        body: &Value,
    ) -> MockResponse {
        let cmk = body
            .get("customerManagedKeyIdentifier")
            .and_then(|v| v.as_str())
            .map(String::from);
        let tags = parse_tag_list(body.get("tags"));

        let mut state = state.write().await;
        let bundle = state.create_app_bundle(account_id, region, cmk, tags);
        let response = wire::CreateAppBundleResponse {
            app_bundle: Some(bundle_to_wire(bundle)),
        };
        wire::serialize_create_app_bundle_response(&response)
    }

    async fn handle_get_app_bundle(
        &self,
        state: &Arc<tokio::sync::RwLock<AppFabricState>>,
        identifier: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_app_bundle(identifier) {
            Ok(b) => {
                let response = wire::GetAppBundleResponse {
                    app_bundle: Some(bundle_to_wire(b)),
                };
                wire::serialize_get_app_bundle_response(&response)
            }
            Err(e) => appfabric_error_response(&e),
        }
    }

    async fn handle_delete_app_bundle(
        &self,
        state: &Arc<tokio::sync::RwLock<AppFabricState>>,
        identifier: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_app_bundle(identifier) {
            Ok(()) => wire::serialize_delete_app_bundle_response(&wire::DeleteAppBundleResponse {}),
            Err(e) => appfabric_error_response(&e),
        }
    }

    async fn handle_list_app_bundles(
        &self,
        state: &Arc<tokio::sync::RwLock<AppFabricState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let summaries: Vec<wire::AppBundleSummary> = state
            .list_app_bundles()
            .iter()
            .map(|b| wire::AppBundleSummary {
                arn: Some(b.arn.clone()),
            })
            .collect();
        let response = wire::ListAppBundlesResponse {
            app_bundle_summary_list: Some(summaries),
            next_token: None,
        };
        wire::serialize_list_app_bundles_response(&response)
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AppFabricState>>,
        arn: &str,
        body: &Value,
    ) -> MockResponse {
        let tags = parse_tag_list(body.get("tags"));
        if tags.is_empty() {
            return rest_json_error(400, "BadRequestException", "tags is required");
        }
        let mut state = state.write().await;
        match state.tag_resource(arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => appfabric_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AppFabricState>>,
        arn: &str,
        uri: &str,
    ) -> MockResponse {
        let qs = extract_query_string(uri);
        let mut keys: Vec<String> = Vec::new();
        for pair in qs.split('&') {
            if let Some((k, v)) = pair.split_once('=') {
                if k == "tagKeys" {
                    keys.push(percent_decode(v));
                }
            }
        }
        if keys.is_empty() {
            return rest_json_error(400, "BadRequestException", "tagKeys is required");
        }
        let mut state = state.write().await;
        match state.untag_resource(arn, &keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => appfabric_error_response(&e),
        }
    }

    async fn handle_list_tags(
        &self,
        state: &Arc<tokio::sync::RwLock<AppFabricState>>,
        arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_tags(arn) {
            Ok(tags) => {
                let tag_list: Vec<wire::Tag> = tags
                    .into_iter()
                    .map(|(k, v)| wire::Tag { key: k, value: v })
                    .collect();
                let response = wire::ListTagsForResourceResponse {
                    tags: if tag_list.is_empty() {
                        None
                    } else {
                        Some(tag_list)
                    },
                };
                wire::serialize_list_tags_for_resource_response(&response)
            }
            Err(e) => appfabric_error_response(&e),
        }
    }
}

fn bundle_to_wire(b: &AppBundle) -> wire::AppBundle {
    wire::AppBundle {
        arn: Some(b.arn.clone()),
        customer_managed_key_arn: b.customer_managed_key_arn.clone(),
    }
}

fn parse_tag_list(v: Option<&Value>) -> HashMap<String, String> {
    v.and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|t| {
                    let k = t.get("key").and_then(|v| v.as_str())?;
                    let val = t.get("value").and_then(|v| v.as_str())?;
                    Some((k.to_string(), val.to_string()))
                })
                .collect()
        })
        .unwrap_or_default()
}

fn appfabric_error_response(err: &AppFabricError) -> MockResponse {
    let (status, error_type) = match err {
        AppFabricError::AppBundleNotFound { .. } => (404, "ResourceNotFoundException"),
        AppFabricError::ResourceNotFound { .. } => (404, "ResourceNotFoundException"),
        AppFabricError::Validation { .. } => (400, "BadRequestException"),
    };
    let body = json!({"message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
