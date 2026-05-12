use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, extract_query_string, parse_query_string, percent_decode, rest_json_error,
};

use crate::state::{CloudFrontKvsError, CloudFrontKvsState};
use crate::views::CloudFrontKvsStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");
const ETAG_HEADER: HeaderName = HeaderName::from_static("etag");

fn set_etag(mut resp: MockResponse, etag: &str) -> MockResponse {
    if let Ok(value) = etag.parse() {
        resp.headers.insert(ETAG_HEADER, value);
    }
    resp
}

pub struct CloudFrontKvsService {
    pub(crate) state: Arc<BackendState<CloudFrontKvsState>>,
    pub(crate) notifier: StateChangeNotifier<CloudFrontKvsStateView>,
}

impl CloudFrontKvsService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CloudFrontKvsService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CloudFrontKvsService {
    fn service_name(&self) -> &str {
        "cloudfront-kvs"
    }

    fn url_patterns(&self) -> Vec<&str> {
        // CloudFront KVS uses an account-prefixed `*.cloudfront-kvs.global.api.aws` endpoint;
        // also allow the documented `cloudfront-keyvaluestore.{region}.amazonaws.com` form.
        vec![
            r"https?://[^/]*cloudfront-kvs\.global\.api\.aws",
            r"https?://cloudfront-keyvaluestore\..*\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<CloudFrontKvsState>>;

impl CloudFrontKvsService {
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

        let segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(percent_decode)
            .collect();
        let segs: Vec<&str> = segments.iter().map(|s| s.as_str()).collect();

        // Extract IfMatch from headers when present.
        let if_match = request
            .headers
            .get("If-Match")
            .or_else(|| request.headers.get("if-match"))
            .and_then(|v| v.to_str().ok())
            .map(String::from)
            .unwrap_or_default();

        let response = match (method.as_str(), segs.as_slice()) {
            ("GET", ["key-value-stores", arn]) => self.handle_describe(&state, arn).await,
            ("GET", ["key-value-stores", arn, "keys"]) => self.handle_list_keys(&state, arn).await,
            ("GET", ["key-value-stores", arn, "keys", key]) => {
                self.handle_get_key(&state, arn, key).await
            }
            ("PUT", ["key-value-stores", arn, "keys", key]) => {
                self.handle_put_key(&state, arn, key, &if_match, &body)
                    .await
            }
            ("DELETE", ["key-value-stores", arn, "keys", key]) => {
                self.handle_delete_key(&state, arn, key, &if_match).await
            }
            ("POST", ["key-value-stores", arn, "keys"]) => {
                self.handle_update_keys(&state, arn, &if_match, &body, &request.uri)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 && method != "GET" {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_describe(&self, state: &SharedState, arn: &str) -> MockResponse {
        let mut state = state.write().await;
        state.ensure_store(arn);
        let r = state.get_store(arn).expect("ensured").clone();
        let total = state.total_size(arn);
        let resp = wire::serialize_describe_key_value_store_response(
            &wire::DescribeKeyValueStoreResponse {
                created: Some(r.created),
                e_tag: Some(r.etag.clone()),
                failure_reason: None,
                item_count: Some(r.items.len() as i32),
                kvs_a_r_n: Some(r.arn.clone()),
                last_modified: Some(r.last_modified),
                status: Some(r.status.clone()),
                total_size_in_bytes: Some(total),
            },
        );
        set_etag(resp, &r.etag)
    }

    async fn handle_get_key(&self, state: &SharedState, arn: &str, key: &str) -> MockResponse {
        let state = state.read().await;
        match state.get_key(arn, key) {
            Ok(value) => wire::serialize_get_key_response(&wire::GetKeyResponse {
                item_count: Some(state.item_count(arn)),
                key: Some(key.to_string()),
                total_size_in_bytes: Some(state.total_size(arn)),
                value: Some(value.clone()),
            }),
            Err(e) => err_response(&e),
        }
    }

    async fn handle_put_key(
        &self,
        state: &SharedState,
        arn: &str,
        key: &str,
        if_match: &str,
        body: &Value,
    ) -> MockResponse {
        let value = match body.get("Value").and_then(|v| v.as_str()) {
            Some(v) => v.to_string(),
            None => return rest_json_error(400, "ValidationException", "Value is required"),
        };
        let mut state = state.write().await;
        match state.put_key(arn, if_match, key.to_string(), value) {
            Ok(()) => {
                let r = state.get_store(arn).expect("ensured").clone();
                let total = state.total_size(arn);
                let resp = wire::serialize_put_key_response(&wire::PutKeyResponse {
                    e_tag: Some(r.etag.clone()),
                    item_count: Some(r.items.len() as i32),
                    total_size_in_bytes: Some(total),
                });
                set_etag(resp, &r.etag)
            }
            Err(e) => err_response(&e),
        }
    }

    async fn handle_delete_key(
        &self,
        state: &SharedState,
        arn: &str,
        key: &str,
        if_match: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_key(arn, if_match, key) {
            Ok(()) => {
                let r = state.get_store(arn).expect("ensured").clone();
                let total = state.total_size(arn);
                let resp = wire::serialize_delete_key_response(&wire::DeleteKeyResponse {
                    e_tag: Some(r.etag.clone()),
                    item_count: Some(r.items.len() as i32),
                    total_size_in_bytes: Some(total),
                });
                set_etag(resp, &r.etag)
            }
            Err(e) => err_response(&e),
        }
    }

    /// Single-page response — pagination not implemented; full result set
    /// returned in one call. The `MaxResults` and `NextToken` query parameters
    /// from the AWS contract are intentionally ignored: every call returns the
    /// complete sorted item list with `next_token: None`.
    async fn handle_list_keys(&self, state: &SharedState, arn: &str) -> MockResponse {
        let state = state.read().await;
        let items: Vec<wire::ListKeysResponseListItem> = state
            .list_items(arn)
            .into_iter()
            .map(|(k, v)| wire::ListKeysResponseListItem {
                key: Some(k.clone()),
                value: Some(v.clone()),
            })
            .collect();
        wire::serialize_list_keys_response(&wire::ListKeysResponse {
            items: Some(items),
            next_token: None,
        })
    }

    async fn handle_update_keys(
        &self,
        state: &SharedState,
        arn: &str,
        body_if_match: &str,
        body: &Value,
        uri: &str,
    ) -> MockResponse {
        // IfMatch can be on body or query string.
        let qs = parse_query_string(extract_query_string(uri));
        let if_match = if !body_if_match.is_empty() {
            body_if_match.to_string()
        } else {
            qs.get("IfMatch")
                .or_else(|| qs.get("if_match"))
                .cloned()
                .unwrap_or_default()
        };
        let puts: Vec<(String, String)> = body
            .get("Puts")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|item| {
                        let k = item.get("Key").and_then(|v| v.as_str())?.to_string();
                        let v = item.get("Value").and_then(|v| v.as_str())?.to_string();
                        Some((k, v))
                    })
                    .collect()
            })
            .unwrap_or_default();
        let deletes: Vec<String> = body
            .get("Deletes")
            .and_then(|v| v.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|item| item.get("Key").and_then(|v| v.as_str()).map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let mut state = state.write().await;
        match state.batch_update(arn, &if_match, puts, deletes) {
            Ok(()) => {
                let r = state.get_store(arn).expect("ensured").clone();
                let total = state.total_size(arn);
                let resp = wire::serialize_update_keys_response(&wire::UpdateKeysResponse {
                    e_tag: Some(r.etag.clone()),
                    item_count: Some(r.items.len() as i32),
                    total_size_in_bytes: Some(total),
                });
                set_etag(resp, &r.etag)
            }
            Err(e) => err_response(&e),
        }
    }
}

fn err_response(err: &CloudFrontKvsError) -> MockResponse {
    let (status, error_type) = match err {
        CloudFrontKvsError::KeyNotFound { .. } => (404, "ResourceNotFoundException"),
        CloudFrontKvsError::EtagMismatch { .. } => (409, "ConflictException"),
        CloudFrontKvsError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"Message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
