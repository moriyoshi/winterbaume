use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    StatefulService,
    protocol::common::{extract_path, extract_query_string, percent_decode},
};

use crate::model;
use crate::state::{CodeGuruSecurityError, CodeGuruSecurityState};
use crate::views::CodeGuruSecurityStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct CodeGuruSecurityService {
    pub(crate) state: Arc<BackendState<CodeGuruSecurityState>>,
    pub(crate) notifier: StateChangeNotifier<CodeGuruSecurityStateView>,
}

impl CodeGuruSecurityService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for CodeGuruSecurityService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for CodeGuruSecurityService {
    fn service_name(&self) -> &str {
        "codeguru-security"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://codeguru-security\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<CodeGuruSecurityState>>;

impl CodeGuruSecurityService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let raw_segments: Vec<String> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .map(percent_decode)
            .collect();
        let segments: Vec<&str> = raw_segments.iter().map(|s| s.as_str()).collect();
        let query = extract_query_string(&request.uri);
        let method = request.method.as_str().to_string();
        let body: Value = if request.body.is_empty() {
            json!({})
        } else {
            serde_json::from_slice(&request.body).unwrap_or(json!({}))
        };

        let (response, mutating) = match (method.as_str(), segments.as_slice()) {
            ("POST", ["batchGetFindings"]) => {
                (self.handle_batch_get_findings(&state, &body).await, false)
            }
            ("POST", ["scans"]) => (
                self.handle_create_scan(&state, &body, &region, account_id)
                    .await,
                true,
            ),
            ("POST", ["uploadUrl"]) => (self.handle_create_upload_url(&body, &region).await, false),
            ("GET", ["accountConfiguration", "get"]) => {
                (self.handle_get_account_configuration(&state).await, false)
            }
            ("PUT", ["updateAccountConfiguration"]) => (
                self.handle_update_account_configuration(&state, &body)
                    .await,
                true,
            ),
            ("GET", ["findings", scan_name]) => {
                (self.handle_get_findings(&state, scan_name).await, false)
            }
            ("GET", ["metrics", "summary"]) => (self.handle_get_metrics_summary().await, false),
            ("GET", ["scans", scan_name]) => (self.handle_get_scan(&state, scan_name).await, false),
            ("GET", ["metrics", "findings"]) => (self.handle_list_findings_metrics().await, false),
            ("GET", ["scans"]) => (self.handle_list_scans(&state).await, false),
            ("GET", ["tags", arn]) => (self.handle_list_tags(&state, arn).await, false),
            ("POST", ["tags", arn]) => (self.handle_tag_resource(&state, arn, &body).await, true),
            ("DELETE", ["tags", arn]) => {
                let keys: Vec<String> = url_query_get_all(query, "tagKeys")
                    .into_iter()
                    .map(percent_decode)
                    .collect();
                (self.handle_untag_resource(&state, arn, &keys).await, true)
            }
            _ => (
                rest_json_error(404, "ResourceNotFoundException", "No route matches"),
                false,
            ),
        };

        if response.status / 100 == 2 && mutating {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_batch_get_findings(&self, state: &SharedState, body: &Value) -> MockResponse {
        let identifiers = body
            .get("findingIdentifiers")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let state = state.read().await;
        let mut findings: Vec<model::Finding> = vec![];
        let mut failed: Vec<model::BatchGetFindingsError> = vec![];
        for ident in identifiers {
            let scan_name = ident
                .get("scanName")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let finding_id = ident
                .get("findingId")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            let found = state.findings.get(&scan_name).and_then(|fs| {
                fs.iter()
                    .find(|f| f.get("id").and_then(|v| v.as_str()) == Some(&finding_id))
                    .cloned()
            });
            match found {
                Some(f) => findings.push(from_value_or_default(f)),
                None => failed.push(model::BatchGetFindingsError {
                    scan_name: Some(scan_name),
                    finding_id: Some(finding_id),
                    error_code: Some("ResourceNotFound".to_string()),
                    message: Some("Finding not found".to_string()),
                }),
            }
        }
        wire::serialize_batch_get_findings_response(&model::BatchGetFindingsResponse {
            findings: Some(findings),
            failed_findings: Some(failed),
        })
    }

    async fn handle_create_scan(
        &self,
        state: &SharedState,
        body: &Value,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let scan_name = match body.get("scanName").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => return rest_json_error(400, "ValidationException", "scanName is required"),
        };
        let scan_type = body
            .get("scanType")
            .and_then(|v| v.as_str())
            .unwrap_or("Standard")
            .to_string();
        let analysis_type = body
            .get("analysisType")
            .and_then(|v| v.as_str())
            .unwrap_or("Security")
            .to_string();
        let now = chrono::Utc::now().timestamp() as f64;
        let run_id = uuid::Uuid::new_v4().to_string();
        let scan_arn = format!("arn:aws:codeguru-security:{region}:{account_id}:scan/{scan_name}");
        let resource_id = body
            .get("resourceId")
            .cloned()
            .unwrap_or(json!({"codeArtifactId": uuid::Uuid::new_v4().to_string()}));
        let scan = json!({
            "scanName": scan_name,
            "scanArn": scan_arn,
            "scanNameArn": scan_arn,
            "scanType": scan_type,
            "analysisType": analysis_type,
            "scanState": "Successful",
            "runId": run_id,
            "resourceId": resource_id,
            "createdAt": now,
            "updatedAt": now,
        });
        let mut state = state.write().await;
        state.put_scan(scan_name.clone(), scan.clone());
        let tag_map = parse_tags(body.get("tags"));
        if !tag_map.is_empty() {
            state.tag_resource(&scan_arn, tag_map);
        }
        let response_resource_id: Option<model::ResourceId> =
            scan.get("resourceId").cloned().map(from_value_or_default);
        wire::serialize_create_scan_response(&model::CreateScanResponse {
            scan_name: Some(scan_name),
            scan_name_arn: Some(scan_arn),
            run_id: Some(run_id),
            resource_id: response_resource_id,
            scan_state: Some("Successful".to_string()),
        })
    }

    async fn handle_create_upload_url(&self, body: &Value, region: &str) -> MockResponse {
        let scan_name = body
            .get("scanName")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let id = uuid::Uuid::new_v4().to_string();
        let bucket = format!("codeguru-security-mock-{region}");
        let url = format!("https://{bucket}.s3.{region}.amazonaws.com/uploads/{scan_name}/{id}");
        let mut request_headers = HashMap::new();
        request_headers.insert("Content-Type".to_string(), "application/zip".to_string());
        wire::serialize_create_upload_url_response(&model::CreateUploadUrlResponse {
            code_artifact_id: Some(id),
            s3_url: Some(url),
            request_headers: Some(request_headers),
        })
    }

    async fn handle_get_account_configuration(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let cfg: model::EncryptionConfig = if state.account_configuration.is_null() {
            model::EncryptionConfig::default()
        } else {
            from_value_or_default(state.account_configuration.clone())
        };
        wire::serialize_get_account_configuration_response(
            &model::GetAccountConfigurationResponse {
                encryption_config: Some(cfg),
            },
        )
    }

    async fn handle_update_account_configuration(
        &self,
        state: &SharedState,
        body: &Value,
    ) -> MockResponse {
        let cfg = body.get("encryptionConfig").cloned().unwrap_or(json!({}));
        let mut state = state.write().await;
        state.account_configuration = cfg.clone();
        let typed_cfg: model::EncryptionConfig = from_value_or_default(cfg);
        wire::serialize_update_account_configuration_response(
            &model::UpdateAccountConfigurationResponse {
                encryption_config: Some(typed_cfg),
            },
        )
    }

    async fn handle_get_findings(&self, state: &SharedState, scan_name: &str) -> MockResponse {
        let state = state.read().await;
        if !state.scans.contains_key(scan_name) {
            return err_response(&CodeGuruSecurityError::NotFound {
                name: scan_name.to_string(),
            });
        }
        let findings: Vec<model::Finding> = state
            .findings
            .get(scan_name)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(from_value_or_default)
            .collect();
        wire::serialize_get_findings_response(&model::GetFindingsResponse {
            findings: Some(findings),
            next_token: None,
        })
    }

    async fn handle_get_metrics_summary(&self) -> MockResponse {
        let summary = model::MetricsSummary {
            date: Some(chrono::Utc::now().timestamp() as f64),
            open_findings: Some(model::FindingMetricsValuePerSeverity {
                critical: Some(0.0),
                high: Some(0.0),
                info: Some(0.0),
                low: Some(0.0),
                medium: Some(0.0),
            }),
            categories_with_most_findings: Some(vec![]),
            scans_with_most_open_findings: Some(vec![]),
            scans_with_most_open_critical_findings: Some(vec![]),
        };
        wire::serialize_get_metrics_summary_response(&model::GetMetricsSummaryResponse {
            metrics_summary: Some(summary),
        })
    }

    async fn handle_get_scan(&self, state: &SharedState, scan_name: &str) -> MockResponse {
        let state = state.read().await;
        match state.scans.get(scan_name) {
            Some(record) => {
                let resp: model::GetScanResponse = from_value_or_default(record.scan.clone());
                wire::serialize_get_scan_response(&resp)
            }
            None => err_response(&CodeGuruSecurityError::NotFound {
                name: scan_name.to_string(),
            }),
        }
    }

    async fn handle_list_findings_metrics(&self) -> MockResponse {
        wire::serialize_list_findings_metrics_response(&model::ListFindingsMetricsResponse {
            findings_metrics: Some(vec![]),
            next_token: None,
        })
    }

    async fn handle_list_scans(&self, state: &SharedState) -> MockResponse {
        let state = state.read().await;
        let summaries: Vec<model::ScanSummary> = state
            .list_scans()
            .into_iter()
            .map(|s| model::ScanSummary {
                scan_name: s
                    .get("scanName")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                scan_name_arn: s
                    .get("scanArn")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                scan_state: s
                    .get("scanState")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                created_at: s.get("createdAt").and_then(|v| v.as_f64()),
                updated_at: s.get("updatedAt").and_then(|v| v.as_f64()),
                run_id: s
                    .get("runId")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
            })
            .collect();
        wire::serialize_list_scans_response(&model::ListScansResponse {
            summaries: Some(summaries),
            next_token: None,
        })
    }

    async fn handle_list_tags(&self, state: &SharedState, arn: &str) -> MockResponse {
        let state = state.read().await;
        let tags = state.list_tags(arn);
        wire::serialize_list_tags_for_resource_response(&model::ListTagsForResourceResponse {
            tags: Some(tags),
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &SharedState,
        arn: &str,
        body: &Value,
    ) -> MockResponse {
        let tag_map = parse_tags(body.get("tags"));
        let mut state = state.write().await;
        state.tag_resource(arn, tag_map);
        wire::serialize_tag_resource_response(&model::TagResourceResponse {})
    }

    async fn handle_untag_resource(
        &self,
        state: &SharedState,
        arn: &str,
        keys: &[String],
    ) -> MockResponse {
        let mut state = state.write().await;
        state.untag_resource(arn, keys);
        wire::serialize_untag_resource_response(&model::UntagResourceResponse {})
    }
}

fn parse_tags(v: Option<&Value>) -> HashMap<String, String> {
    let Some(map) = v.and_then(|v| v.as_object()) else {
        return HashMap::new();
    };
    map.iter()
        .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
        .collect()
}

fn url_query_get_all<'a>(query: &'a str, key: &str) -> Vec<&'a str> {
    query
        .split('&')
        .filter_map(|kv| {
            let (k, v) = kv.split_once('=').unwrap_or((kv, ""));
            if k == key { Some(v) } else { None }
        })
        .collect()
}

fn from_value_or_default<T: DeserializeOwned + Default>(v: Value) -> T {
    serde_json::from_value(v).unwrap_or_default()
}

fn rest_json_error(status: u16, error_type: &str, message: &str) -> MockResponse {
    let body = json!({"__type": error_type, "Message": message});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn err_response(err: &CodeGuruSecurityError) -> MockResponse {
    let (status, error_type) = match err {
        CodeGuruSecurityError::NotFound { .. } => (404, "ResourceNotFoundException"),
        CodeGuruSecurityError::Validation { .. } => (400, "ValidationException"),
    };
    rest_json_error(status, error_type, &err.to_string())
}
