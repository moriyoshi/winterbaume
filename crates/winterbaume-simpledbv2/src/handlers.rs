use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{SdbError, SdbState};
use crate::views::SdbStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct SimpleDbV2Service {
    pub(crate) state: Arc<BackendState<SdbState>>,
    pub(crate) notifier: StateChangeNotifier<SdbStateView>,
}

impl SimpleDbV2Service {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }

    /// Pre-register a domain as existing in the mock state.
    /// This is useful for tests that need domains to exist before starting exports.
    pub async fn with_domain(self, region: &str, domain_name: &str) -> Self {
        let state = self.state.get(default_account_id(), region);
        state.write().await.add_domain(domain_name);
        self
    }
}

impl Default for SimpleDbV2Service {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SimpleDbV2Service {
    fn service_name(&self) -> &str {
        "sdb"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://sdb\.(.+)\.amazonaws\.com",
            r"https?://sdb\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl SimpleDbV2Service {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        // Parse URL path for restJson1 routing
        let path = extract_path(&request.uri);
        let raw_query = extract_query_string(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&raw_query);

        let response = match path.as_str() {
            "/v2/StartDomainExport" => {
                self.handle_start_domain_export(
                    &state,
                    &request,
                    &[],
                    &query_map,
                    account_id,
                    &region,
                )
                .await
            }
            "/v2/GetExport" => {
                self.handle_get_export(&state, &request, &[], &query_map)
                    .await
            }
            "/v2/ListExports" => {
                self.handle_list_exports(&state, &request, &[], &query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_start_domain_export(
        &self,
        state: &Arc<tokio::sync::RwLock<SdbState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_domain_export_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON body"),
        };
        if input.domain_name.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterValueException",
                "Missing 'domainName'",
            );
        }
        if input.s3_bucket.is_empty() {
            return rest_json_error(400, "InvalidParameterValueException", "Missing 's3Bucket'");
        }
        let client_token = match input.client_token.as_deref() {
            Some(t) if !t.is_empty() => t,
            _ => {
                return rest_json_error(
                    400,
                    "InvalidParameterValueException",
                    "clientToken is required",
                );
            }
        };

        let mut state = state.write().await;
        match state.start_domain_export(
            &input.domain_name,
            &input.s3_bucket,
            input.s3_key_prefix.as_deref(),
            input.s3_sse_algorithm.as_deref(),
            input.s3_sse_kms_key_id.as_deref(),
            input.s3_bucket_owner.as_deref(),
            Some(client_token),
            account_id,
            region,
        ) {
            Ok(export) => {
                wire::serialize_start_domain_export_response(&wire::StartDomainExportResponse {
                    client_token: Some(export.client_token.clone()),
                    export_arn: Some(export.export_arn.clone()),
                    requested_at: Some(export.requested_at.timestamp() as f64),
                })
            }
            Err(e) => sdb_error_response(&e),
        }
    }

    async fn handle_get_export(
        &self,
        state: &Arc<tokio::sync::RwLock<SdbState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_export_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON body"),
        };
        if input.export_arn.is_empty() {
            return rest_json_error(400, "InvalidParameterValueException", "Missing 'exportArn'");
        }

        let state = state.read().await;
        match state.get_export(&input.export_arn) {
            Ok(export) => wire::serialize_get_export_response(&wire::GetExportResponse {
                export_arn: Some(export.export_arn.clone()),
                client_token: Some(export.client_token.clone()),
                export_status: Some(export.export_status.clone()),
                domain_name: Some(export.domain_name.clone()),
                requested_at: Some(export.requested_at.timestamp() as f64),
                s3_bucket: Some(export.s3_bucket.clone()),
                s3_key_prefix: export.s3_key_prefix.clone(),
                s3_sse_algorithm: export.s3_sse_algorithm.clone(),
                s3_sse_kms_key_id: export.s3_sse_kms_key_id.clone(),
                s3_bucket_owner: export.s3_bucket_owner.clone(),
                failure_code: export.failure_code.clone(),
                failure_message: export.failure_message.clone(),
                export_manifest: export.export_manifest.clone(),
                items_count: export.items_count,
                export_data_cutoff_time: export
                    .export_data_cutoff_time
                    .map(|dt| dt.timestamp() as f64),
            }),
            Err(e) => sdb_error_response(&e),
        }
    }

    async fn handle_list_exports(
        &self,
        state: &Arc<tokio::sync::RwLock<SdbState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_exports_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON body"),
        };

        let state = state.read().await;
        match state.list_exports(
            input.domain_name.as_deref(),
            input.max_results,
            input.next_token.as_deref(),
        ) {
            Ok((summaries, next_token)) => {
                let entries: Vec<wire::ExportSummary> = summaries
                    .iter()
                    .map(|s| wire::ExportSummary {
                        export_arn: Some(s.export_arn.clone()),
                        export_status: Some(s.export_status.clone()),
                        requested_at: Some(s.requested_at.timestamp() as f64),
                        domain_name: Some(s.domain_name.clone()),
                    })
                    .collect();

                wire::serialize_list_exports_response(&wire::ListExportsResponse {
                    export_summaries: Some(entries),
                    next_token,
                })
            }
            Err(e) => sdb_error_response(&e),
        }
    }
}

fn sdb_error_response(err: &SdbError) -> MockResponse {
    let (status, error_type) = match err {
        SdbError::NoSuchDomain { .. } => (400, "NoSuchDomainException"),
        SdbError::NoSuchExport { .. } => (400, "NoSuchExportException"),
        SdbError::Conflict => (400, "ConflictException"),
    };
    let body = json!({
        "Type": "User",
        "Message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
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

fn extract_path(uri: &str) -> String {
    // Parse the URI and extract the path component
    if let Ok(parsed) = uri.parse::<http::Uri>() {
        parsed.path().to_string()
    } else {
        // Fallback: find path after host
        if let Some(pos) = uri.find("amazonaws.com") {
            let rest = &uri[pos + "amazonaws.com".len()..];
            rest.split('?').next().unwrap_or("/").to_string()
        } else {
            "/".to_string()
        }
    }
}

fn extract_query_string(uri: &str) -> String {
    if let Ok(parsed) = uri.parse::<http::Uri>() {
        parsed.query().unwrap_or("").to_string()
    } else if let Some(idx) = uri.find('?') {
        uri[idx + 1..].to_string()
    } else {
        String::new()
    }
}
