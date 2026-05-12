use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, extract_query_string, parse_query_string, rest_json_error,
};

use crate::state::{CloudSearchDomainState, Document};
use crate::views::CloudSearchDomainStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct CloudSearchDomainService {
    pub(crate) state: Arc<BackendState<CloudSearchDomainState>>,
    pub(crate) notifier: StateChangeNotifier<CloudSearchDomainStateView>,
}

impl CloudSearchDomainService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CloudSearchDomainService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CloudSearchDomainService {
    fn service_name(&self) -> &str {
        "cloudsearch-domain"
    }

    fn url_patterns(&self) -> Vec<&str> {
        // CloudSearch domains have per-domain hostnames like
        // `search-{name}-{id}.{region}.cloudsearch.amazonaws.com`. Allow any
        // subdomain that contains `cloudsearch` before `amazonaws.com`.
        vec![r"https?://[^/]*\.cloudsearch\.[^/]*amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<CloudSearchDomainState>>;

impl CloudSearchDomainService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let path = path.trim_start_matches('/');
        let method = request.method.as_str().to_uppercase();

        let response = match (method.as_str(), path) {
            ("GET", "2013-01-01/search") => self.handle_search(&state, &request.uri).await,
            ("GET", "2013-01-01/suggest") => self.handle_suggest(&state, &request.uri).await,
            ("POST", "2013-01-01/documents/batch") => self.handle_upload(&state, &request).await,
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2 && method != "GET" {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_search(&self, state: &SharedState, uri: &str) -> MockResponse {
        let qs = parse_query_string(extract_query_string(uri));
        let query = qs.get("q").cloned().unwrap_or_default();
        let state = state.read().await;
        let docs = state.search(&query);
        let hits: Vec<wire::Hit> = docs
            .iter()
            .map(|d| wire::Hit {
                exprs: None,
                fields: Some(
                    d.fields
                        .iter()
                        .map(|(k, v)| (k.clone(), vec![v.clone()]))
                        .collect(),
                ),
                highlights: None,
                id: Some(d.id.clone()),
            })
            .collect();
        let total = hits.len() as i64;
        wire::serialize_search_response(&wire::SearchResponse {
            facets: None,
            hits: Some(wire::Hits {
                cursor: None,
                found: Some(total),
                hit: Some(hits),
                start: Some(0),
            }),
            stats: None,
            status: Some(wire::SearchStatus {
                rid: Some(format!("rid-{}", uuid::Uuid::new_v4().simple())),
                timems: Some(1),
            }),
        })
    }

    async fn handle_suggest(&self, state: &SharedState, uri: &str) -> MockResponse {
        let qs = parse_query_string(extract_query_string(uri));
        let query = qs.get("q").cloned().unwrap_or_default();
        let state = state.read().await;
        let docs = state.suggest(&query);
        let suggestions: Vec<wire::SuggestionMatch> = docs
            .iter()
            .map(|d| wire::SuggestionMatch {
                id: Some(d.id.clone()),
                score: Some(1),
                suggestion: d.fields.values().next().cloned(),
            })
            .collect();
        let total = suggestions.len() as i64;
        wire::serialize_suggest_response(&wire::SuggestResponse {
            status: Some(wire::SuggestStatus {
                rid: Some(format!("rid-{}", uuid::Uuid::new_v4().simple())),
                timems: Some(1),
            }),
            suggest: Some(wire::SuggestModel {
                found: Some(total),
                query: Some(query),
                suggestions: Some(suggestions),
            }),
        })
    }

    async fn handle_upload(&self, state: &SharedState, request: &MockRequest) -> MockResponse {
        let content_type = request
            .headers
            .get("content-type")
            .or_else(|| request.headers.get("Content-Type"))
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/json")
            .to_lowercase();
        let body_bytes = request.body.to_vec();
        let documents = if content_type.contains("xml") {
            return rest_json_error(
                400,
                "InvalidContent",
                "XML SDF is not supported by this mock; use application/json",
            );
        } else {
            match serde_json::from_slice::<Vec<Value>>(&body_bytes) {
                Ok(v) => v,
                Err(_) => {
                    return rest_json_error(400, "InvalidContent", "Body must be a JSON array");
                }
            }
        };
        let mut batch: Vec<Document> = vec![];
        for d in documents {
            let id = d
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            if id.is_empty() {
                continue;
            }
            let op = d
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("add")
                .to_string();
            let fields = d
                .get("fields")
                .and_then(|v| v.as_object())
                .map(|m| {
                    m.iter()
                        .map(|(k, v)| {
                            let value = match v {
                                Value::String(s) => s.clone(),
                                Value::Array(arr) => arr
                                    .iter()
                                    .filter_map(|x| x.as_str().map(String::from))
                                    .collect::<Vec<_>>()
                                    .join(" "),
                                other => other.to_string(),
                            };
                            (k.clone(), value)
                        })
                        .collect::<HashMap<String, String>>()
                })
                .unwrap_or_default();
            batch.push(Document { id, op, fields });
        }
        let mut state = state.write().await;
        let (adds, deletes) = state.upload(batch);
        let warnings: Vec<wire::DocumentServiceWarning> = vec![];
        wire::serialize_upload_documents_response(&wire::UploadDocumentsResponse {
            adds: Some(adds),
            deletes: Some(deletes),
            status: Some("success".to_string()),
            warnings: if warnings.is_empty() {
                None
            } else {
                Some(warnings)
            },
        })
    }
}

#[allow(dead_code)]
fn err_response(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({"Message": message});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
