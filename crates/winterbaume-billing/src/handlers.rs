use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
};

use crate::state::{BillingError, BillingState};
use crate::views::BillingStateView;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct BillingService {
    pub(crate) state: Arc<BackendState<BillingState>>,
    pub(crate) notifier: StateChangeNotifier<BillingStateView>,
}

impl BillingService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for BillingService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for BillingService {
    fn service_name(&self) -> &str {
        "billing"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://billing\..*\.api\.aws",
            r"https?://billing\..*\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<BillingState>>;

const MUTATING_ACTIONS: &[&str] = &[
    "CreateBillingView",
    "UpdateBillingView",
    "DeleteBillingView",
    "AssociateSourceViews",
    "DisassociateSourceViews",
    "TagResource",
    "UntagResource",
];

impl BillingService {
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
            None => return aws_json_error(400, "MissingAction", "Missing X-Amz-Target"),
        };

        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            match serde_json::from_slice(&request.body) {
                Ok(v) => v,
                Err(_) => {
                    return aws_json_error(400, "SerializationException", "Invalid JSON body");
                }
            }
        };

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateBillingView" => self.handle_create(&state, &body, &region, account_id).await,
            "GetBillingView" => self.handle_get(&state, &body).await,
            "UpdateBillingView" => self.handle_update(&state, &body).await,
            "DeleteBillingView" => self.handle_delete(&state, &body).await,
            "ListBillingViews" => self.handle_list(&state).await,
            "AssociateSourceViews" => self.handle_associate(&state, &body).await,
            "DisassociateSourceViews" => self.handle_disassociate(&state, &body).await,
            "ListSourceViewsForBillingView" => self.handle_list_sources(&state, &body).await,
            "GetResourcePolicy" => self.handle_get_policy(&state, &body).await,
            "ListTagsForResource" => self.handle_list_tags(&state, &body).await,
            "TagResource" => self.handle_tag(&state, &body).await,
            "UntagResource" => self.handle_untag(&state, &body).await,
            other => aws_json_error(
                400,
                "UnknownOperationException",
                &format!("Unknown action: {other}"),
            ),
        };

        if response.status / 100 == 2 && MUTATING_ACTIONS.contains(&action.as_str()) {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create(
        &self,
        state: &SharedState,
        body: &Value,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let name = match body.get("name").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => return aws_json_error(400, "ValidationException", "name is required"),
        };
        let id = uuid::Uuid::new_v4().simple().to_string();
        let arn = format!("arn:aws:billing::{account_id}:billingview/{id}");
        let now = epoch_secs();
        let source_views = body.get("sourceViews").cloned().unwrap_or(json!([]));
        let view = json!({
            "arn": arn,
            "name": name,
            "description": body.get("description").cloned().unwrap_or(Value::Null),
            "billingViewType": "CUSTOM",
            "ownerAccountId": account_id,
            "createdAt": now,
            "updatedAt": now,
            "sourceViews": source_views,
            "dataFilterExpression": body.get("dataFilterExpression").cloned().unwrap_or(Value::Null),
        });
        let mut state = state.write().await;
        state.put_view(arn.clone(), view);
        let tag_map = parse_tag_object(body.get("resourceTags"));
        if !tag_map.is_empty() {
            state.tag_resource(&arn, tag_map);
        }
        let _ = region;
        aws_json_response(json!({"arn": arn, "createdAt": now}))
    }

    async fn handle_get(&self, state: &SharedState, body: &Value) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => return aws_json_error(400, "ValidationException", "arn is required"),
        };
        let state = state.read().await;
        match state.billing_views.get(&arn) {
            Some(v) => aws_json_response(json!({"billingView": v})),
            None => err_response(&BillingError::NotFound { arn }),
        }
    }

    async fn handle_update(&self, state: &SharedState, body: &Value) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => return aws_json_error(400, "ValidationException", "arn is required"),
        };
        let mut state = state.write().await;
        let view = match state.billing_views.get_mut(&arn) {
            Some(v) => v,
            None => return err_response(&BillingError::NotFound { arn }),
        };
        if let Value::Object(map) = view {
            for field in ["name", "description", "dataFilterExpression"] {
                if let Some(v) = body.get(field) {
                    map.insert(field.into(), v.clone());
                }
            }
            map.insert("updatedAt".into(), json!(epoch_secs()));
        }
        aws_json_response(json!({"arn": arn, "updatedAt": epoch_secs()}))
    }

    async fn handle_delete(&self, state: &SharedState, body: &Value) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => return aws_json_error(400, "ValidationException", "arn is required"),
        };
        let mut state = state.write().await;
        match state.billing_views.remove(&arn) {
            Some(_) => {
                state.tags.remove(&arn);
                state.policies.remove(&arn);
                aws_json_response(json!({"arn": arn}))
            }
            None => err_response(&BillingError::NotFound { arn }),
        }
    }

    async fn handle_list(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let summaries: Vec<Value> = state
            .list_views()
            .into_iter()
            .map(|v| {
                json!({
                    "arn": v.get("arn").cloned().unwrap_or(Value::Null),
                    "name": v.get("name").cloned().unwrap_or(Value::Null),
                    "description": v.get("description").cloned().unwrap_or(Value::Null),
                    "billingViewType": v.get("billingViewType").cloned().unwrap_or(Value::Null),
                    "ownerAccountId": v.get("ownerAccountId").cloned().unwrap_or(Value::Null),
                })
            })
            .collect();
        aws_json_response(json!({"billingViews": summaries, "nextToken": Value::Null}))
    }

    async fn handle_associate(&self, state: &SharedState, body: &Value) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => return aws_json_error(400, "ValidationException", "arn is required"),
        };
        let new_sources: Vec<Value> = body
            .get("sourceViews")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let mut state = state.write().await;
        let view = match state.billing_views.get_mut(&arn) {
            Some(v) => v,
            None => return err_response(&BillingError::NotFound { arn }),
        };
        if let Value::Object(map) = view {
            let existing = map
                .get("sourceViews")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();
            let mut combined = existing;
            for src in new_sources {
                if !combined.iter().any(|c| c == &src) {
                    combined.push(src);
                }
            }
            map.insert("sourceViews".into(), Value::Array(combined));
        }
        aws_json_response(json!({"arn": arn}))
    }

    async fn handle_disassociate(&self, state: &SharedState, body: &Value) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => return aws_json_error(400, "ValidationException", "arn is required"),
        };
        let to_remove: Vec<Value> = body
            .get("sourceViews")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let mut state = state.write().await;
        let view = match state.billing_views.get_mut(&arn) {
            Some(v) => v,
            None => return err_response(&BillingError::NotFound { arn }),
        };
        if let Value::Object(map) = view {
            if let Some(arr) = map.get("sourceViews").and_then(|v| v.as_array()).cloned() {
                let filtered: Vec<Value> =
                    arr.into_iter().filter(|s| !to_remove.contains(s)).collect();
                map.insert("sourceViews".into(), Value::Array(filtered));
            }
        }
        aws_json_response(json!({"arn": arn}))
    }

    async fn handle_list_sources(&self, state: &SharedState, body: &Value) -> MockResponse {
        let arn = match body.get("arn").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => return aws_json_error(400, "ValidationException", "arn is required"),
        };
        let state = state.read().await;
        let view = match state.billing_views.get(&arn) {
            Some(v) => v,
            None => return err_response(&BillingError::NotFound { arn }),
        };
        let sources = view.get("sourceViews").cloned().unwrap_or(json!([]));
        aws_json_response(json!({"sourceViews": sources, "nextToken": Value::Null}))
    }

    async fn handle_get_policy(&self, state: &SharedState, body: &Value) -> MockResponse {
        let arn = match body.get("resourceArn").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => return aws_json_error(400, "ValidationException", "resourceArn is required"),
        };
        let state = state.read().await;
        if !state.billing_views.contains_key(&arn) {
            return err_response(&BillingError::NotFound { arn });
        }
        let policy = state.policies.get(&arn).cloned().unwrap_or_default();
        aws_json_response(json!({"resourceArn": arn, "policy": policy}))
    }

    async fn handle_list_tags(&self, state: &SharedState, body: &Value) -> MockResponse {
        let arn = match body.get("resourceArn").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => return aws_json_error(400, "ValidationException", "resourceArn is required"),
        };
        let state = state.read().await;
        let map = state.list_tags(&arn);
        let tags: Vec<Value> = map
            .into_iter()
            .map(|(k, v)| json!({"key": k, "value": v}))
            .collect();
        aws_json_response(json!({"resourceTags": tags}))
    }

    async fn handle_tag(&self, state: &SharedState, body: &Value) -> MockResponse {
        let arn = match body.get("resourceArn").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => return aws_json_error(400, "ValidationException", "resourceArn is required"),
        };
        let tags: HashMap<String, String> = body
            .get("resourceTags")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|t| {
                        let k = t.get("key").and_then(|v| v.as_str())?;
                        let v = t.get("value").and_then(|v| v.as_str())?;
                        Some((k.to_string(), v.to_string()))
                    })
                    .collect()
            })
            .unwrap_or_default();
        let mut state = state.write().await;
        state.tag_resource(&arn, tags);
        aws_json_response(json!({}))
    }

    async fn handle_untag(&self, state: &SharedState, body: &Value) -> MockResponse {
        let arn = match body.get("resourceArn").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => return aws_json_error(400, "ValidationException", "resourceArn is required"),
        };
        let keys: Vec<String> = body
            .get("resourceTagKeys")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let mut state = state.write().await;
        state.untag_resource(&arn, &keys);
        aws_json_response(json!({}))
    }
}

fn parse_tag_object(v: Option<&Value>) -> HashMap<String, String> {
    let Some(arr) = v.and_then(|v| v.as_array()) else {
        return HashMap::new();
    };
    arr.iter()
        .filter_map(|t| {
            let k = t.get("key").and_then(|v| v.as_str())?;
            let val = t.get("value").and_then(|v| v.as_str())?;
            Some((k.to_string(), val.to_string()))
        })
        .collect()
}

fn epoch_secs() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0)
}

fn aws_json_response(body: Value) -> MockResponse {
    MockResponse::json(200, body.to_string())
}

fn aws_json_error(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({"__type": error_type, "Message": message});
    let mut resp = MockResponse::json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn err_response(err: &BillingError) -> MockResponse {
    let (status, error_type) = match err {
        BillingError::NotFound { .. } => (404, "ResourceNotFoundException"),
        BillingError::Validation { .. } => (400, "ValidationException"),
    };
    aws_json_error(status, error_type, &err.to_string())
}
