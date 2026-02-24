use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{MediaStoreDataError, MediaStoreDataState};
use crate::types::ItemType;
use crate::views::MediaStoreDataStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct MediaStoreDataService {
    pub(crate) state: Arc<BackendState<MediaStoreDataState>>,
    pub(crate) notifier: StateChangeNotifier<MediaStoreDataStateView>,
}

impl MediaStoreDataService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for MediaStoreDataService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for MediaStoreDataService {
    fn service_name(&self) -> &str {
        "mediastore"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://data\.mediastore\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl MediaStoreDataService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        // MediaStore Data Plane routes:
        // PUT /{Path+}  -> PutObject
        // GET /{Path+}  -> GetObject
        // DELETE /{Path+}  -> DeleteObject
        // HEAD /{Path+}  -> DescribeObject
        // GET /  -> ListItems (with optional query param ?Path=...)

        // Check for the root path (ListItems)
        if path == "/" && method == "GET" {
            let query_path = extract_query_param(&request.uri, "Path");
            return self.handle_list_items(&state, query_path.as_deref()).await;
        }

        // For all other operations, the object path is the URL path (without leading /)
        let object_path = path.trim_start_matches('/');

        if object_path.is_empty() {
            return match method {
                "GET" => self.handle_list_items(&state, None).await,
                _ => rest_json_error(400, "ValidationException", "Invalid request"),
            };
        }

        match method {
            "PUT" => self.handle_put_object(&state, object_path, &request).await,
            "GET" => self.handle_get_object(&state, object_path).await,
            "DELETE" => self.handle_delete_object(&state, object_path).await,
            "HEAD" => self.handle_describe_object(&state, object_path).await,
            _ => rest_json_error(400, "ValidationException", "Unsupported method"),
        }
    }

    async fn handle_put_object(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaStoreDataState>>,
        path: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let content_type = request
            .headers
            .get(http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let cache_control = request
            .headers
            .get(http::header::CACHE_CONTROL)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let body = request.body.clone();

        let mut state = state.write().await;
        match state.put_object(
            path,
            body,
            content_type.as_deref(),
            cache_control.as_deref(),
        ) {
            Ok(obj) => {
                let result = wire::PutObjectResponse {
                    content_s_h_a256: Some(obj.etag.clone()),
                    e_tag: Some(obj.etag.clone()),
                    storage_class: Some("TEMPORAL".to_string()),
                };
                wire::serialize_put_object_response(&result)
            }
            Err(e) => mediastore_error_response(&e),
        }
    }

    async fn handle_get_object(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaStoreDataState>>,
        path: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_object(path) {
            Ok(obj) => {
                let mut resp = MockResponse {
                    status: 200,
                    headers: http::HeaderMap::new(),
                    body: obj.body.clone(),
                };
                resp.headers.insert(
                    http::header::CONTENT_TYPE,
                    obj.content_type.parse().unwrap(),
                );
                resp.headers.insert(
                    http::header::CONTENT_LENGTH,
                    obj.content_length.to_string().parse().unwrap(),
                );
                resp.headers
                    .insert(http::header::ETAG, obj.etag.parse().unwrap());
                resp.headers.insert(
                    http::header::LAST_MODIFIED,
                    obj.last_modified
                        .format("%a, %d %b %Y %H:%M:%S GMT")
                        .to_string()
                        .parse()
                        .unwrap(),
                );
                resp.headers
                    .insert(HeaderName::from_static("path"), obj.path.parse().unwrap());
                if let Some(ref cc) = obj.cache_control {
                    resp.headers
                        .insert(http::header::CACHE_CONTROL, cc.parse().unwrap());
                }
                resp
            }
            Err(e) => mediastore_error_response(&e),
        }
    }

    async fn handle_delete_object(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaStoreDataState>>,
        path: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_object(path) {
            Ok(()) => wire::serialize_delete_object_response(&wire::DeleteObjectResponse {}),
            Err(e) => mediastore_error_response(&e),
        }
    }

    async fn handle_describe_object(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaStoreDataState>>,
        path: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.describe_object(path) {
            Ok(obj) => {
                let result = wire::DescribeObjectResponse::default();
                let mut resp = wire::serialize_describe_object_response(&result);
                // Set header-bound fields per the restJson protocol
                resp.headers.insert(
                    http::header::CONTENT_TYPE,
                    obj.content_type.parse().unwrap(),
                );
                resp.headers.insert(
                    http::header::CONTENT_LENGTH,
                    obj.content_length.to_string().parse().unwrap(),
                );
                resp.headers
                    .insert(http::header::ETAG, obj.etag.parse().unwrap());
                resp.headers.insert(
                    http::header::LAST_MODIFIED,
                    obj.last_modified
                        .format("%a, %d %b %Y %H:%M:%S GMT")
                        .to_string()
                        .parse()
                        .unwrap(),
                );
                if let Some(ref cc) = obj.cache_control {
                    resp.headers
                        .insert(http::header::CACHE_CONTROL, cc.parse().unwrap());
                }
                resp
            }
            Err(e) => mediastore_error_response(&e),
        }
    }

    async fn handle_list_items(
        &self,
        state: &Arc<tokio::sync::RwLock<MediaStoreDataState>>,
        path: Option<&str>,
    ) -> MockResponse {
        let state = state.read().await;
        let items = state.list_items(path);

        let wire_items: Vec<wire::Item> = items
            .iter()
            .map(|item| {
                let mut wire_item = wire::Item {
                    name: Some(item.name.clone()),
                    r#type: Some(match item.item_type {
                        ItemType::Object => "OBJECT".to_string(),
                        ItemType::Folder => "FOLDER".to_string(),
                    }),
                    ..Default::default()
                };
                wire_item.content_type = item.content_type.clone();
                wire_item.content_length = item.content_length.map(|v| v as i64);
                wire_item.e_tag = item.etag.clone();
                wire_item.last_modified = item.last_modified.map(|lm| lm.timestamp() as f64);
                wire_item
            })
            .collect();

        let result = wire::ListItemsResponse {
            items: if wire_items.is_empty() {
                Some(vec![])
            } else {
                Some(wire_items)
            },
            next_token: None,
        };
        wire::serialize_list_items_response(&result)
    }
}

fn extract_path(uri: &str) -> String {
    let after_scheme = if let Some(idx) = uri.find("://") {
        &uri[idx + 3..]
    } else {
        uri
    };
    let path_start = after_scheme.find('/').unwrap_or(after_scheme.len());
    let path_and_query = &after_scheme[path_start..];
    match path_and_query.find('?') {
        Some(q) => path_and_query[..q].to_string(),
        None => path_and_query.to_string(),
    }
}

fn extract_query_param(uri: &str, param: &str) -> Option<String> {
    let query_start = uri.find('?')?;
    let query = &uri[query_start + 1..];
    for pair in query.split('&') {
        if let Some((key, value)) = pair.split_once('=')
            && key == param
        {
            return Some(urlencoding_decode(value));
        }
    }
    None
}

fn urlencoding_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                result.push(byte as char);
            }
        } else if c == '+' {
            result.push(' ');
        } else {
            result.push(c);
        }
    }
    result
}

fn mediastore_error_response(err: &MediaStoreDataError) -> MockResponse {
    match err {
        MediaStoreDataError::ObjectNotFound { .. } => {
            let body = json!({
                "Type": "User",
                "Message": err.to_string(),
            });
            let mut resp = MockResponse::rest_json(404, body.to_string());
            resp.headers
                .insert(X_AMZN_ERRORTYPE, "ObjectNotFoundException".parse().unwrap());
            resp
        }
    }
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "Type": "User",
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
