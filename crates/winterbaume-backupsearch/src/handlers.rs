use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, percent_decode, rest_json_error,
};

use crate::state::{BackupSearchError, BackupSearchState};
use crate::types::{SearchJob, SearchResultExportJob};
use crate::views::BackupSearchStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct BackupSearchService {
    pub(crate) state: Arc<BackendState<BackupSearchState>>,
    pub(crate) notifier: StateChangeNotifier<BackupSearchStateView>,
}

impl BackupSearchService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for BackupSearchService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for BackupSearchService {
    fn service_name(&self) -> &str {
        "backup-search"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://backup-search\..*\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

type SharedState = Arc<tokio::sync::RwLock<BackupSearchState>>;

impl BackupSearchService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str().to_uppercase();
        let query = winterbaume_core::extract_query_string(&request.uri);
        let query_map: HashMap<String, String> = winterbaume_core::parse_query_string(query);

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        let response = match (method.as_str(), segments.as_slice()) {
            ("PUT", ["search-jobs"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_start_search_job(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            ("GET", ["search-jobs"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_list_search_jobs(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["search-jobs", id]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("SearchJobIdentifier", id_decoded.as_str())];
                self.handle_get_search_job(&state, &request, labels, &query_map)
                    .await
            }
            ("PUT", ["search-jobs", id, "actions", "cancel"]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("SearchJobIdentifier", id_decoded.as_str())];
                self.handle_stop_search_job(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["search-jobs", id, "backups"]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("SearchJobIdentifier", id_decoded.as_str())];
                self.handle_list_search_job_backups(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["search-jobs", id, "search-results"]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("SearchJobIdentifier", id_decoded.as_str())];
                self.handle_list_search_job_results(&state, &request, labels, &query_map)
                    .await
            }
            ("PUT", ["export-search-jobs"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_start_export_job(
                    &state, &request, labels, &query_map, account_id, &region,
                )
                .await
            }
            ("GET", ["export-search-jobs"]) => {
                let labels: &[(&str, &str)] = &[];
                self.handle_list_export_jobs(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["export-search-jobs", id]) => {
                let id_decoded = percent_decode(id);
                let labels: &[(&str, &str)] = &[("ExportJobIdentifier", id_decoded.as_str())];
                self.handle_get_export_job(&state, &request, labels, &query_map)
                    .await
            }
            ("POST", ["tags", arn]) => {
                let arn_decoded = percent_decode(arn);
                let labels: &[(&str, &str)] = &[("ResourceArn", arn_decoded.as_str())];
                self.handle_tag_resource(&state, &request, labels, &query_map)
                    .await
            }
            ("DELETE", ["tags", arn]) => {
                let arn_decoded = percent_decode(arn);
                let labels: &[(&str, &str)] = &[("ResourceArn", arn_decoded.as_str())];
                self.handle_untag_resource(&state, &request, labels, &query_map)
                    .await
            }
            ("GET", ["tags", arn]) => {
                let arn_decoded = percent_decode(arn);
                let labels: &[(&str, &str)] = &[("ResourceArn", arn_decoded.as_str())];
                self.handle_list_tags_for_resource(&state, &request, labels, &query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };

        use winterbaume_core::StatefulService;
        if response.status / 100 == 2
            && matches!(method.as_str(), "POST" | "PATCH" | "DELETE" | "PUT")
        {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_start_search_job(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_search_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!(
            "arn:aws:backup-search:{}:{}:search-job/{}",
            region, account_id, id
        );
        let now = chrono::Utc::now().timestamp();
        let job = SearchJob {
            identifier: id.clone(),
            arn: arn.clone(),
            name: input.name.clone(),
            status: "RUNNING".to_string(),
            status_message: None,
            encryption_key_arn: input.encryption_key_arn.clone(),
            search_scope: serde_json::to_value(&input.search_scope).ok(),
            item_filters: input
                .item_filters
                .as_ref()
                .and_then(|v| serde_json::to_value(v).ok()),
            creation_time: now,
            completion_time: None,
            items_matched: 0,
            items_scanned: 0,
            recovery_points_scanned: 0,
            tags: input.tags.clone().unwrap_or_default(),
        };
        let mut state = state.write().await;
        state.create_search_job(job);
        wire::serialize_start_search_job_response(&wire::StartSearchJobOutput {
            creation_time: Some(now as f64),
            search_job_arn: Some(arn),
            search_job_identifier: Some(id),
        })
    }

    async fn handle_get_search_job(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_search_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_search_job(&input.search_job_identifier) {
            Ok(j) => {
                let response = wire::GetSearchJobOutput {
                    completion_time: j.completion_time.map(|t| t as f64),
                    creation_time: Some(j.creation_time as f64),
                    current_search_progress: Some(wire::CurrentSearchProgress {
                        items_matched_count: Some(j.items_matched),
                        items_scanned_count: Some(j.items_scanned),
                        recovery_points_scanned_count: Some(j.recovery_points_scanned),
                    }),
                    encryption_key_arn: j.encryption_key_arn.clone(),
                    item_filters: j.item_filters.as_ref().and_then(parse_value),
                    name: j.name.clone(),
                    search_job_arn: Some(j.arn.clone()),
                    search_job_identifier: Some(j.identifier.clone()),
                    search_scope: j.search_scope.as_ref().and_then(parse_value),
                    search_scope_summary: None,
                    status: Some(j.status.clone()),
                    status_message: j.status_message.clone(),
                };
                wire::serialize_get_search_job_response(&response)
            }
            Err(e) => bs_error_response(&e),
        }
    }

    async fn handle_stop_search_job(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_stop_search_job_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.stop_search_job(&input.search_job_identifier) {
            Ok(_) => wire::serialize_stop_search_job_response(&wire::StopSearchJobOutput {}),
            Err(e) => bs_error_response(&e),
        }
    }

    async fn handle_list_search_jobs(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_list_search_jobs_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let items: Vec<wire::SearchJobSummary> = state
            .list_search_jobs()
            .into_iter()
            .map(|j| wire::SearchJobSummary {
                completion_time: j.completion_time.map(|t| t as f64),
                creation_time: Some(j.creation_time as f64),
                name: j.name.clone(),
                search_job_arn: Some(j.arn.clone()),
                search_job_identifier: Some(j.identifier.clone()),
                search_scope_summary: None,
                status: Some(j.status.clone()),
                status_message: j.status_message.clone(),
            })
            .collect();
        wire::serialize_list_search_jobs_response(&wire::ListSearchJobsOutput {
            next_token: None,
            search_jobs: Some(items),
        })
    }

    async fn handle_list_search_job_backups(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_search_job_backups_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_search_job(&input.search_job_identifier) {
            Ok(_) => wire::serialize_list_search_job_backups_response(
                &wire::ListSearchJobBackupsOutput {
                    next_token: None,
                    results: Some(vec![]),
                },
            ),
            Err(e) => bs_error_response(&e),
        }
    }

    async fn handle_list_search_job_results(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_search_job_results_request(request, labels, query)
        {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_search_job(&input.search_job_identifier) {
            Ok(_) => wire::serialize_list_search_job_results_response(
                &wire::ListSearchJobResultsOutput {
                    next_token: None,
                    results: Some(vec![]),
                },
            ),
            Err(e) => bs_error_response(&e),
        }
    }

    async fn handle_start_export_job(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_start_search_result_export_job_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.search_job_identifier.is_empty() {
            return rest_json_error(
                400,
                "ValidationException",
                "SearchJobIdentifier is required",
            );
        }
        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!(
            "arn:aws:backup-search:{}:{}:export-job/{}",
            region, account_id, id
        );
        let now = chrono::Utc::now().timestamp();
        let job = SearchResultExportJob {
            identifier: id.clone(),
            arn: arn.clone(),
            search_job_identifier: input.search_job_identifier.clone(),
            status: "RUNNING".to_string(),
            status_message: None,
            export_specification: serde_json::to_value(&input.export_specification).ok(),
            role_arn: input.role_arn.clone(),
            creation_time: now,
            completion_time: None,
            tags: input.tags.clone().unwrap_or_default(),
        };

        let mut state = state.write().await;
        match state.create_export_job(job) {
            Ok(_) => wire::serialize_start_search_result_export_job_response(
                &wire::StartSearchResultExportJobOutput {
                    export_job_arn: Some(arn),
                    export_job_identifier: Some(id),
                },
            ),
            Err(e) => bs_error_response(&e),
        }
    }

    async fn handle_get_export_job(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_search_result_export_job_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        match state.get_export_job(&input.export_job_identifier) {
            Ok(j) => {
                let response = wire::GetSearchResultExportJobOutput {
                    completion_time: j.completion_time.map(|t| t as f64),
                    creation_time: Some(j.creation_time as f64),
                    export_job_arn: Some(j.arn.clone()),
                    export_job_identifier: Some(j.identifier.clone()),
                    export_specification: j.export_specification.as_ref().and_then(parse_value),
                    search_job_arn: Some(j.search_job_identifier.clone()),
                    status: Some(j.status.clone()),
                    status_message: j.status_message.clone(),
                };
                wire::serialize_get_search_result_export_job_response(&response)
            }
            Err(e) => bs_error_response(&e),
        }
    }

    async fn handle_list_export_jobs(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let _input = match wire::deserialize_list_search_result_export_jobs_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let items: Vec<wire::ExportJobSummary> = state
            .list_export_jobs()
            .into_iter()
            .map(|j| wire::ExportJobSummary {
                completion_time: j.completion_time.map(|t| t as f64),
                creation_time: Some(j.creation_time as f64),
                export_job_arn: Some(j.arn.clone()),
                export_job_identifier: Some(j.identifier.clone()),
                search_job_arn: Some(j.search_job_identifier.clone()),
                status: Some(j.status.clone()),
                status_message: j.status_message.clone(),
            })
            .collect();
        wire::serialize_list_search_result_export_jobs_response(
            &wire::ListSearchResultExportJobsOutput {
                export_jobs: Some(items),
                next_token: None,
            },
        )
    }

    async fn handle_tag_resource(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tags.is_empty() {
            return rest_json_error(400, "ValidationException", "Tags is required");
        }
        let mut state = state.write().await;
        match state.tag_resource(&input.resource_arn, input.tags.clone()) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => bs_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        if input.tag_keys.is_empty() {
            return rest_json_error(400, "ValidationException", "tagKeys is required");
        }
        let mut state = state.write().await;
        match state.untag_resource(&input.resource_arn, &input.tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => bs_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &SharedState,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.list_tags(&input.resource_arn) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse {
                    tags: if tags.is_empty() { None } else { Some(tags) },
                },
            ),
            Err(e) => bs_error_response(&e),
        }
    }
}

fn parse_value<T: serde::de::DeserializeOwned>(v: &Value) -> Option<T> {
    serde_json::from_value(v.clone()).ok()
}

fn bs_error_response(err: &BackupSearchError) -> MockResponse {
    let (status, error_type) = match err {
        BackupSearchError::SearchJobNotFound { .. } => (404, "ResourceNotFoundException"),
        BackupSearchError::ExportJobNotFound { .. } => (404, "ResourceNotFoundException"),
        BackupSearchError::ResourceNotFound { .. } => (404, "ResourceNotFoundException"),
        BackupSearchError::SearchJobNotRunning { .. } => (409, "ConflictException"),
        BackupSearchError::Validation { .. } => (400, "ValidationException"),
    };
    let body = json!({"Message": err.to_string()});
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
