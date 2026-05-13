use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id, extract_path, rest_json_error,
};

use crate::state::{AccessAnalyzerError, AccessAnalyzerState};
use crate::types::CriterionValue;
use crate::views::AccessAnalyzerStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct AccessAnalyzerService {
    pub(crate) state: Arc<BackendState<AccessAnalyzerState>>,
    pub(crate) notifier: StateChangeNotifier<AccessAnalyzerStateView>,
}

impl AccessAnalyzerService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for AccessAnalyzerService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for AccessAnalyzerService {
    fn service_name(&self) -> &str {
        "access-analyzer"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://access-analyzer\..*\.amazonaws\.com",
            r"https?://access-analyzer\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl AccessAnalyzerService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let raw_query = match request.uri.find('?') {
            Some(idx) => request.uri[idx + 1..].to_string(),
            None => String::new(),
        };
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(&raw_query);
        let method = request.method.as_str();

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        if segments.is_empty() {
            return rest_json_error(404, "NotFoundException", "Not found");
        }

        // --- Tags: /tags/{ResourceArn+} ---
        if segments[0] == "tags" && segments.len() >= 2 {
            let resource_arn = decode_resource_arn(&segments[1..].join("/"));
            let response = match method {
                "GET" => {
                    self.handle_list_tags_for_resource(&state, &resource_arn)
                        .await
                }
                "POST" => {
                    let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
                    self.handle_tag_resource(&state, &request, labels, &query_map)
                        .await
                }
                "DELETE" => {
                    let tag_keys = extract_tag_keys_from_uri(&request.uri);
                    self.handle_untag_resource(&state, &resource_arn, &tag_keys)
                        .await
                }
                // --- Unimplemented operations (auto-generated stubs) ---
                // PUT /archive-rule => ApplyArchiveRule (not implemented)
                // PUT /policy/generation/{jobId} => CancelPolicyGeneration (not implemented)
                // POST /policy/check-access-not-granted => CheckAccessNotGranted (not implemented)
                // POST /policy/check-no-new-access => CheckNoNewAccess (not implemented)
                // POST /policy/check-no-public-access => CheckNoPublicAccess (not implemented)
                // PUT /access-preview => CreateAccessPreview (not implemented)
                // POST /recommendation/{id} => GenerateFindingRecommendation (not implemented)
                // GET /access-preview/{accessPreviewId} => GetAccessPreview (not implemented)
                // GET /analyzed-resource => GetAnalyzedResource (not implemented)
                // GET /finding/{id} => GetFinding (not implemented)
                // GET /recommendation/{id} => GetFindingRecommendation (not implemented)
                // GET /findingv2/{id} => GetFindingV2 (not implemented)
                // POST /analyzer/findings/statistics => GetFindingsStatistics (not implemented)
                // GET /policy/generation/{jobId} => GetGeneratedPolicy (not implemented)
                // POST /access-preview/{accessPreviewId} => ListAccessPreviewFindings (not implemented)
                // GET /access-preview => ListAccessPreviews (not implemented)
                // POST /analyzed-resource => ListAnalyzedResources (not implemented)
                // POST /finding => ListFindings (not implemented)
                // POST /findingv2 => ListFindingsV2 (not implemented)
                // GET /policy/generation => ListPolicyGenerations (not implemented)
                // PUT /policy/generation => StartPolicyGeneration (not implemented)
                // POST /resource/scan => StartResourceScan (not implemented)
                // PUT /analyzer/{analyzerName} => UpdateAnalyzer (not implemented)
                // PUT /analyzer/{analyzerName}/archive-rule/{ruleName} => UpdateArchiveRule (not implemented)
                // PUT /finding => UpdateFindings (not implemented)
                // POST /policy/validation => ValidatePolicy (not implemented)

                // 26 unimplemented operations above
                _ => rest_json_error(404, "NotFoundException", "Not found"),
            };
            if matches!(method, "POST" | "DELETE") && response.status / 100 == 2 {
                self.notify_state_changed(account_id, &region).await;
            }
            return response;
        }

        let response = match (method, segments.as_slice()) {
            // --- Analyzers ---
            // PUT /analyzer -> CreateAnalyzer
            ("PUT", ["analyzer"]) => {
                self.handle_create_analyzer(&state, &request, &[], &query_map, &region, account_id)
                    .await
            }
            // GET /analyzer -> ListAnalyzers
            ("GET", ["analyzer"]) => self.handle_list_analyzers(&state, &request.uri).await,
            // GET /analyzer/{analyzerName} -> GetAnalyzer
            ("GET", ["analyzer", analyzer_name]) => {
                self.handle_get_analyzer(&state, analyzer_name).await
            }
            // DELETE /analyzer/{analyzerName} -> DeleteAnalyzer
            ("DELETE", ["analyzer", analyzer_name]) => {
                self.handle_delete_analyzer(&state, analyzer_name).await
            }

            // --- Archive Rules ---
            // PUT /analyzer/{analyzerName}/archive-rule -> CreateArchiveRule
            ("PUT", ["analyzer", analyzer_name, "archive-rule"]) => {
                let labels: &[(&str, &str)] = &[("analyzerName", analyzer_name)];
                self.handle_create_archive_rule(&state, &request, labels, &query_map)
                    .await
            }
            // GET /analyzer/{analyzerName}/archive-rule -> ListArchiveRules
            ("GET", ["analyzer", analyzer_name, "archive-rule"]) => {
                self.handle_list_archive_rules(&state, analyzer_name).await
            }
            // GET /analyzer/{analyzerName}/archive-rule/{ruleName} -> GetArchiveRule
            ("GET", ["analyzer", analyzer_name, "archive-rule", rule_name]) => {
                self.handle_get_archive_rule(&state, analyzer_name, rule_name)
                    .await
            }
            // DELETE /analyzer/{analyzerName}/archive-rule/{ruleName} -> DeleteArchiveRule
            ("DELETE", ["analyzer", analyzer_name, "archive-rule", rule_name]) => {
                self.handle_delete_archive_rule(&state, analyzer_name, rule_name)
                    .await
            }

            // --- Unimplemented operations (auto-generated stubs) ---
            // PUT /archive-rule => ApplyArchiveRule
            // PUT /policy/generation/{jobId} => CancelPolicyGeneration
            // POST /policy/check-access-not-granted => CheckAccessNotGranted
            // POST /policy/check-no-new-access => CheckNoNewAccess
            // POST /policy/check-no-public-access => CheckNoPublicAccess
            // PUT /access-preview => CreateAccessPreview
            // POST /recommendation/{id} => GenerateFindingRecommendation
            // GET /access-preview/{accessPreviewId} => GetAccessPreview
            // GET /analyzed-resource => GetAnalyzedResource
            // GET /finding/{id} => GetFinding
            // GET /recommendation/{id} => GetFindingRecommendation
            // GET /findingv2/{id} => GetFindingV2
            // POST /analyzer/findings/statistics => GetFindingsStatistics
            // GET /policy/generation/{jobId} => GetGeneratedPolicy
            // POST /access-preview/{accessPreviewId} => ListAccessPreviewFindings
            // GET /access-preview => ListAccessPreviews
            // POST /analyzed-resource => ListAnalyzedResources
            // POST /finding => ListFindings
            // POST /findingv2 => ListFindingsV2
            // GET /policy/generation => ListPolicyGenerations
            // PUT /policy/generation => StartPolicyGeneration
            // POST /resource/scan => StartResourceScan
            // PUT /analyzer/{analyzerName} => UpdateAnalyzer
            // PUT /analyzer/{analyzerName}/archive-rule/{ruleName} => UpdateArchiveRule
            // PUT /finding => UpdateFindings
            // POST /policy/validation => ValidatePolicy
            _ => rest_json_error(501, "NotImplementedException", "Operation not implemented"),
        };

        // Notify on successful mutations
        if matches!(method, "PUT" | "POST" | "DELETE") && response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }

        response
    }

    // -------------------------------------------------------------------------
    // Analyzer handlers
    // -------------------------------------------------------------------------

    async fn handle_create_analyzer(
        &self,
        state: &Arc<tokio::sync::RwLock<AccessAnalyzerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_analyzer_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.analyzer_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'analyzerName'");
        }
        if input.r#type.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'type'");
        }
        let tags = input.tags.clone().unwrap_or_default();

        let mut state = state.write().await;
        match state.create_analyzer(
            &input.analyzer_name,
            &input.r#type,
            tags,
            region,
            account_id,
        ) {
            Ok(analyzer) => {
                wire::serialize_create_analyzer_response(&wire::CreateAnalyzerResponse {
                    arn: Some(analyzer.arn.clone()),
                })
            }
            Err(e) => aa_error_response(&e),
        }
    }

    async fn handle_get_analyzer(
        &self,
        state: &Arc<tokio::sync::RwLock<AccessAnalyzerState>>,
        analyzer_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_analyzer(analyzer_name) {
            Ok(analyzer_state) => {
                wire::serialize_get_analyzer_response(&wire::GetAnalyzerResponse {
                    analyzer: Some(analyzer_to_wire(&analyzer_state.analyzer)),
                })
            }
            Err(e) => aa_error_response(&e),
        }
    }

    async fn handle_delete_analyzer(
        &self,
        state: &Arc<tokio::sync::RwLock<AccessAnalyzerState>>,
        analyzer_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_analyzer(analyzer_name) {
            Ok(()) => wire::serialize_delete_analyzer_response(),
            Err(e) => aa_error_response(&e),
        }
    }

    async fn handle_list_analyzers(
        &self,
        state: &Arc<tokio::sync::RwLock<AccessAnalyzerState>>,
        uri: &str,
    ) -> MockResponse {
        let type_filter = extract_query_param(uri, "type");
        let state = state.read().await;
        let analyzers = state.list_analyzers(type_filter.as_deref());
        let wire_analyzers: Vec<wire::AnalyzerSummary> =
            analyzers.iter().map(|a| analyzer_to_wire(a)).collect();
        wire::serialize_list_analyzers_response(&wire::ListAnalyzersResponse {
            analyzers: Some(wire_analyzers),
            next_token: None,
        })
    }

    // -------------------------------------------------------------------------
    // Archive Rule handlers
    // -------------------------------------------------------------------------

    async fn handle_create_archive_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<AccessAnalyzerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_create_archive_rule_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.rule_name.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'ruleName'");
        }
        if input.filter.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'filter'");
        }
        let filter: HashMap<String, CriterionValue> = input
            .filter
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    CriterionValue {
                        eq: v.eq.clone(),
                        neq: v.neq.clone(),
                        contains: v.contains.clone(),
                        exists: v.exists,
                    },
                )
            })
            .collect();

        let mut state = state.write().await;
        match state.create_archive_rule(&input.analyzer_name, &input.rule_name, filter) {
            Ok(()) => wire::serialize_create_archive_rule_response(),
            Err(e) => aa_error_response(&e),
        }
    }

    async fn handle_get_archive_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<AccessAnalyzerState>>,
        analyzer_name: &str,
        rule_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_archive_rule(analyzer_name, rule_name) {
            Ok(rule) => wire::serialize_get_archive_rule_response(&wire::GetArchiveRuleResponse {
                archive_rule: Some(archive_rule_to_wire(rule)),
            }),
            Err(e) => aa_error_response(&e),
        }
    }

    async fn handle_delete_archive_rule(
        &self,
        state: &Arc<tokio::sync::RwLock<AccessAnalyzerState>>,
        analyzer_name: &str,
        rule_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_archive_rule(analyzer_name, rule_name) {
            Ok(()) => wire::serialize_delete_archive_rule_response(),
            Err(e) => aa_error_response(&e),
        }
    }

    async fn handle_list_archive_rules(
        &self,
        state: &Arc<tokio::sync::RwLock<AccessAnalyzerState>>,
        analyzer_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_archive_rules(analyzer_name) {
            Ok(rules) => {
                let wire_rules: Vec<wire::ArchiveRuleSummary> =
                    rules.iter().map(|r| archive_rule_to_wire(r)).collect();
                wire::serialize_list_archive_rules_response(&wire::ListArchiveRulesResponse {
                    archive_rules: Some(wire_rules),
                    next_token: None,
                })
            }
            Err(e) => aa_error_response(&e),
        }
    }

    // -------------------------------------------------------------------------
    // Tag handlers
    // -------------------------------------------------------------------------

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AccessAnalyzerState>>,
        resource_arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_tags_for_resource(resource_arn) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse {
                    tags: if tags.is_empty() {
                        None
                    } else {
                        Some(tags.clone())
                    },
                },
            ),
            Err(e) => aa_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AccessAnalyzerState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tags.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'tags'");
        }
        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, input.tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => aa_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<AccessAnalyzerState>>,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.untag_resource(resource_arn, tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => aa_error_response(&e),
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn analyzer_to_wire(a: &crate::types::Analyzer) -> wire::AnalyzerSummary {
    wire::AnalyzerSummary {
        arn: Some(a.arn.clone()),
        name: Some(a.name.clone()),
        r#type: Some(a.analyzer_type.clone()),
        status: Some(a.status.clone()),
        created_at: Some(a.created_at.clone()),
        tags: if a.tags.is_empty() {
            None
        } else {
            Some(a.tags.clone())
        },
        ..Default::default()
    }
}

fn archive_rule_to_wire(r: &crate::types::ArchiveRule) -> wire::ArchiveRuleSummary {
    let filter: HashMap<String, wire::Criterion> = r
        .filter
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                wire::Criterion {
                    eq: v.eq.clone(),
                    neq: v.neq.clone(),
                    contains: v.contains.clone(),
                    exists: v.exists,
                },
            )
        })
        .collect();
    wire::ArchiveRuleSummary {
        rule_name: Some(r.rule_name.clone()),
        filter: Some(filter),
        created_at: Some(r.created_at.clone()),
        updated_at: Some(r.updated_at.clone()),
    }
}

fn decode_resource_arn(encoded: &str) -> String {
    encoded
        .replace("%3A", ":")
        .replace("%2F", "/")
        .replace("%3a", ":")
        .replace("%2f", "/")
}

fn extract_tag_keys_from_uri(uri: &str) -> Vec<String> {
    if let Some(query_start) = uri.find('?') {
        let query = &uri[query_start + 1..];
        query
            .split('&')
            .filter_map(|param| {
                let (key, value) = param.split_once('=')?;
                if key == "tagKeys" {
                    Some(
                        value
                            .replace("%3A", ":")
                            .replace("%2F", "/")
                            .replace("%3a", ":")
                            .replace("%2f", "/"),
                    )
                } else {
                    None
                }
            })
            .collect()
    } else {
        vec![]
    }
}

fn extract_query_param(uri: &str, param_name: &str) -> Option<String> {
    let query_start = uri.find('?')?;
    let query = &uri[query_start + 1..];
    for pair in query.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
            if key == param_name {
                return Some(value.to_string());
            }
        }
    }
    None
}

fn aa_error_response(err: &AccessAnalyzerError) -> MockResponse {
    let (status, error_type) = match err {
        AccessAnalyzerError::AnalyzerAlreadyExists { .. } => (409, "ConflictException"),
        AccessAnalyzerError::AnalyzerNotFound { .. } => (404, "ResourceNotFoundException"),
        AccessAnalyzerError::ArchiveRuleAlreadyExists { .. } => (409, "ConflictException"),
        AccessAnalyzerError::ArchiveRuleNotFound { .. } => (404, "ResourceNotFoundException"),
        AccessAnalyzerError::ResourceNotFound { .. } => (404, "ResourceNotFoundException"),
        AccessAnalyzerError::Validation { .. } => (400, "ValidationException"),
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
