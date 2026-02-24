use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
};

use crate::state::{DsqlError, DsqlState};
use crate::views::DsqlStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct DsqlService {
    pub(crate) state: Arc<BackendState<DsqlState>>,
    pub(crate) notifier: StateChangeNotifier<DsqlStateView>,
}

impl DsqlService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for DsqlService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for DsqlService {
    fn service_name(&self) -> &str {
        "dsql"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://dsql\.(.+)\.api\.aws"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl DsqlService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = extract_region_from_dsql_uri(&request.uri)
            .or_else(|| winterbaume_core::auth::extract_region_from_headers(&request.headers))
            .unwrap_or_else(|| "us-east-1".to_string());
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        let query = extract_query(&request.uri);

        // DSQL API routes:
        // POST   /cluster                   - CreateCluster
        // GET    /cluster                   - ListClusters
        // GET    /cluster/{identifier}      - GetCluster
        // DELETE /cluster/{identifier}      - DeleteCluster
        // PUT    /cluster/{identifier}      - UpdateCluster (not implemented)
        // DELETE /cluster/{identifier}/policy  - DeleteClusterPolicy (not implemented)
        // GET    /cluster/{identifier}/policy  - GetClusterPolicy (not implemented)
        // PUT    /cluster/{identifier}/policy  - PutClusterPolicy (not implemented)
        // GET    /clusters/{identifier}/vpc-endpoint-service-name - GetVpcEndpointServiceName (not implemented)
        // GET    /tags/{resourceArn}         - ListTagsForResource (not implemented)
        // POST   /tags/{resourceArn}         - TagResource (not implemented)
        // DELETE /tags/{resourceArn}         - UntagResource (not implemented)

        if segments.is_empty() {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        use winterbaume_core::StatefulService;
        let response = match (method, segments[0], segments.len()) {
            // POST /cluster - CreateCluster
            ("POST", "cluster", 1) => {
                let body: Value = match serde_json::from_slice(&request.body) {
                    Ok(v) => v,
                    Err(_) => {
                        return rest_json_error(400, "ValidationException", "Invalid JSON body");
                    }
                };
                self.handle_create_cluster(&state, &body, account_id, &region)
                    .await
            }
            // GET /cluster - ListClusters
            ("GET", "cluster", 1) => self.handle_list_clusters(&state, &query).await,
            // GET /cluster/{identifier} - GetCluster
            ("GET", "cluster", 2) => {
                let identifier = percent_decode(segments[1]);
                self.handle_get_cluster(&state, &identifier).await
            }
            // DELETE /cluster/{identifier} - DeleteCluster
            ("DELETE", "cluster", 2) => {
                let identifier = percent_decode(segments[1]);
                self.handle_delete_cluster(&state, &identifier).await
            }
            // --- Unimplemented operations ---
            // PUT /cluster/{identifier} => UpdateCluster (not implemented)
            // DELETE /cluster/{identifier}/policy => DeleteClusterPolicy (not implemented)
            // GET /cluster/{identifier}/policy => GetClusterPolicy (not implemented)
            // PUT /cluster/{identifier}/policy => PutClusterPolicy (not implemented)
            // GET /clusters/{identifier}/vpc-endpoint-service-name => GetVpcEndpointServiceName (not implemented)
            // GET /tags/{resourceArn} => ListTagsForResource (not implemented)
            // POST /tags/{resourceArn} => TagResource (not implemented)
            // DELETE /tags/{resourceArn} => UntagResource (not implemented)

            // 8 unimplemented operations above
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<DsqlState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let deletion_protection_enabled = body
            .get("deletionProtectionEnabled")
            .and_then(|v| v.as_bool());
        let client_token = body.get("clientToken").and_then(|v| v.as_str());
        let tags: HashMap<String, String> = body
            .get("tags")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| Some((k.clone(), v.as_str()?.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.create_cluster(
            account_id,
            region,
            deletion_protection_enabled,
            client_token,
            tags,
        ) {
            Ok(cluster) => {
                wire::serialize_create_cluster_response(&cluster_to_create_output(&cluster))
            }
            Err(e) => dsql_error_response(&e),
        }
    }

    async fn handle_get_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<DsqlState>>,
        identifier: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_cluster(identifier) {
            Ok(cluster) => wire::serialize_get_cluster_response(&cluster_to_get_output(cluster)),
            Err(e) => dsql_error_response(&e),
        }
    }

    async fn handle_delete_cluster(
        &self,
        state: &Arc<tokio::sync::RwLock<DsqlState>>,
        identifier: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_cluster(identifier) {
            Ok(cluster) => {
                wire::serialize_delete_cluster_response(&cluster_to_delete_output(&cluster))
            }
            Err(e) => dsql_error_response(&e),
        }
    }

    async fn handle_list_clusters(
        &self,
        state: &Arc<tokio::sync::RwLock<DsqlState>>,
        query: &[(String, String)],
    ) -> MockResponse {
        let max_results = query
            .iter()
            .find(|(k, _)| k == "max-results")
            .and_then(|(_, v)| v.parse::<usize>().ok());
        let next_token = query
            .iter()
            .find(|(k, _)| k == "next-token")
            .map(|(_, v)| v.as_str());

        let state = state.read().await;
        match state.list_clusters(max_results, next_token) {
            Ok((clusters, new_token)) => {
                let output = wire::ListClustersOutput {
                    clusters: Some(
                        clusters
                            .iter()
                            .map(|c| wire::ClusterSummary {
                                identifier: Some(c.identifier.clone()),
                                arn: Some(c.arn.clone()),
                            })
                            .collect::<Vec<_>>(),
                    ),
                    next_token: new_token,
                };
                wire::serialize_list_clusters_response(&output)
            }
            Err(e) => dsql_error_response(&e),
        }
    }
}

fn cluster_to_create_output(cluster: &crate::types::Cluster) -> wire::CreateClusterOutput {
    wire::CreateClusterOutput {
        identifier: Some(cluster.identifier.clone()),
        arn: Some(cluster.arn.clone()),
        status: Some(cluster.status.clone()),
        creation_time: Some(cluster.creation_time.timestamp() as f64),
        deletion_protection_enabled: Some(cluster.deletion_protection_enabled),
        ..Default::default()
    }
}

fn cluster_to_get_output(cluster: &crate::types::Cluster) -> wire::GetClusterOutput {
    let tags = if cluster.tags.is_empty() {
        None
    } else {
        Some(cluster.tags.clone())
    };
    wire::GetClusterOutput {
        identifier: Some(cluster.identifier.clone()),
        arn: Some(cluster.arn.clone()),
        status: Some(cluster.status.clone()),
        creation_time: Some(cluster.creation_time.timestamp() as f64),
        deletion_protection_enabled: Some(cluster.deletion_protection_enabled),
        tags,
        ..Default::default()
    }
}

fn cluster_to_delete_output(cluster: &crate::types::Cluster) -> wire::DeleteClusterOutput {
    wire::DeleteClusterOutput {
        identifier: Some(cluster.identifier.clone()),
        arn: Some(cluster.arn.clone()),
        status: Some(cluster.status.clone()),
        creation_time: Some(cluster.creation_time.timestamp() as f64),
        ..Default::default()
    }
}

/// Extract region from DSQL-style URI: https://dsql.{region}.api.aws
fn extract_region_from_dsql_uri(uri: &str) -> Option<String> {
    let host = extract_host(uri)?;
    // Pattern: dsql.{region}.api.aws
    let parts: Vec<&str> = host.split('.').collect();
    if parts.len() >= 4
        && parts[0] == "dsql"
        && parts[parts.len() - 2] == "api"
        && parts[parts.len() - 1] == "aws"
    {
        // Region is everything between "dsql." and ".api.aws"
        let region = parts[1..parts.len() - 2].join(".");
        if !region.is_empty() {
            return Some(region);
        }
    }
    None
}

fn extract_host(uri: &str) -> Option<&str> {
    let after_scheme = uri
        .strip_prefix("https://")
        .or_else(|| uri.strip_prefix("http://"))?;
    let host = after_scheme.split('/').next()?;
    let host = host.split(':').next()?;
    if host.is_empty() {
        return None;
    }
    Some(host)
}

fn extract_path(uri: &str) -> String {
    if let Some(after_scheme) = uri
        .strip_prefix("https://")
        .or_else(|| uri.strip_prefix("http://"))
    {
        let path = after_scheme
            .find('/')
            .map(|i| &after_scheme[i..])
            .unwrap_or("/");
        path.split('?').next().unwrap_or(path).to_string()
    } else {
        "/".to_string()
    }
}

fn extract_query(uri: &str) -> Vec<(String, String)> {
    if let Some(q) = uri.find('?') {
        uri[q + 1..]
            .split('&')
            .filter_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                let key = parts.next()?;
                let value = parts.next().unwrap_or("");
                Some((percent_decode(key), percent_decode(value)))
            })
            .collect()
    } else {
        Vec::new()
    }
}

fn percent_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut bytes = s.bytes();
    while let Some(b) = bytes.next() {
        match b {
            b'%' => {
                let hi = bytes.next().and_then(hex_val);
                let lo = bytes.next().and_then(hex_val);
                if let (Some(hi), Some(lo)) = (hi, lo) {
                    result.push((hi << 4 | lo) as char);
                }
            }
            b'+' => result.push(' '),
            _ => result.push(b as char),
        }
    }
    result
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn dsql_error_response(err: &DsqlError) -> MockResponse {
    let (status, error_type) = match err {
        DsqlError::ResourceNotFound { .. } => (404, "ResourceNotFoundException"),
        DsqlError::DeletionProtectionEnabled { .. } => (409, "ConflictException"),
        DsqlError::InvalidNextToken => (400, "ValidationException"),
    };
    let body = json!({
        "message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}
