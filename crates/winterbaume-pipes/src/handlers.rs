use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, extract_query_string, parse_query_string, percent_decode, rest_json_error,
};

use crate::state::{PipesError, PipesState};
use crate::views::PipesStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct PipesService {
    pub(crate) state: Arc<BackendState<PipesState>>,
    pub(crate) notifier: StateChangeNotifier<PipesStateView>,
}

impl PipesService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for PipesService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for PipesService {
    fn service_name(&self) -> &str {
        "pipes"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://pipes\..*\.amazonaws\.com",
            r"https?://pipes\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl PipesService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let raw_query = extract_query_string(&request.uri);
        let query_map: HashMap<String, String> = parse_query_string(raw_query);
        let method = request.method.as_str();

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        // Expected routes:
        // POST   /v1/pipes/{Name} - CreatePipe
        // GET    /v1/pipes/{Name} - DescribePipe
        // DELETE /v1/pipes/{Name} - DeletePipe
        // GET    /v1/pipes        - ListPipes

        // Handle tag operations: POST /tags/{ResourceArn}, DELETE /tags/{ResourceArn}, GET /tags/{ResourceArn}
        if segments.len() >= 2 && segments[0] == "tags" {
            let resource_arn = percent_decode(segments[1]);
            let labels: &[(&str, &str)] = &[("resourceArn", resource_arn.as_str())];
            match method {
                "POST" => {
                    return self
                        .handle_tag_resource(&state, &request, labels, &query_map)
                        .await;
                }
                "DELETE" => {
                    return self
                        .handle_untag_resource(&state, &request, labels, &query_map, &request.uri)
                        .await;
                }
                "GET" => {
                    return self
                        .handle_list_tags_for_resource(&state, &request, labels, &query_map)
                        .await;
                }
                _ => return rest_json_error(404, "UnknownOperationException", "Not found"),
            }
        }

        if segments.len() < 2 || segments[0] != "v1" || segments[1] != "pipes" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        match (method, segments.len()) {
            // GET /v1/pipes - ListPipes
            ("GET", 2) => {
                self.handle_list_pipes(&state, &request, &[], &query_map)
                    .await
            }
            // POST /v1/pipes/{Name} - CreatePipe
            ("POST", 3) => {
                let name = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_create_pipe(&state, &request, labels, &query_map, &region, account_id)
                    .await
            }
            // GET /v1/pipes/{Name} - DescribePipe
            ("GET", 3) => {
                let name = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_describe_pipe(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v1/pipes/{Name} - DeletePipe
            ("DELETE", 3) => {
                let name = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_delete_pipe(&state, &request, labels, &query_map)
                    .await
            }
            // PUT /v1/pipes/{Name} - UpdatePipe
            ("PUT", 3) => {
                let name = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_update_pipe(&state, &request, labels, &query_map)
                    .await
            }
            // POST /v1/pipes/{Name}/start - StartPipe
            ("POST", 4) if segments[3] == "start" => {
                let name = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_start_pipe(&state, &request, labels, &query_map)
                    .await
            }
            // POST /v1/pipes/{Name}/stop - StopPipe
            ("POST", 4) if segments[3] == "stop" => {
                let name = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_stop_pipe(&state, &request, labels, &query_map)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_create_pipe(
        &self,
        state: &Arc<tokio::sync::RwLock<PipesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_pipe_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "ValidationException", "Invalid JSON body"),
        };
        if input.source.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'Source'");
        }
        if input.target.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'Target'");
        }

        let mut state = state.write().await;
        match state.create_pipe(
            &input.name,
            &input.source,
            &input.target,
            region,
            account_id,
        ) {
            Ok(pipe) => wire::serialize_create_pipe_response(&wire::CreatePipeResponse {
                name: Some(pipe.name.clone()),
                arn: Some(pipe.arn.clone()),
                desired_state: Some(pipe.desired_state.clone()),
                current_state: Some(pipe.current_state.clone()),
                creation_time: Some(pipe.creation_time.timestamp() as f64),
                last_modified_time: Some(pipe.last_modified_time.timestamp() as f64),
            }),
            Err(e) => pipes_error_response(&e),
        }
    }

    async fn handle_describe_pipe(
        &self,
        state: &Arc<tokio::sync::RwLock<PipesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_pipe_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.describe_pipe(&input.name) {
            Ok(pipe) => wire::serialize_describe_pipe_response(&wire::DescribePipeResponse {
                name: Some(pipe.name.clone()),
                arn: Some(pipe.arn.clone()),
                source: Some(pipe.source.clone()),
                target: Some(pipe.target.clone()),
                desired_state: Some(pipe.desired_state.clone()),
                current_state: Some(pipe.current_state.clone()),
                creation_time: Some(pipe.creation_time.timestamp() as f64),
                last_modified_time: Some(pipe.last_modified_time.timestamp() as f64),
                description: pipe.description.clone(),
                ..Default::default()
            }),
            Err(e) => pipes_error_response(&e),
        }
    }

    async fn handle_delete_pipe(
        &self,
        state: &Arc<tokio::sync::RwLock<PipesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_pipe_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_pipe(&input.name) {
            Ok(pipe) => wire::serialize_delete_pipe_response(&wire::DeletePipeResponse {
                name: Some(pipe.name.clone()),
                arn: Some(pipe.arn.clone()),
                desired_state: Some("STOPPED".to_string()),
                current_state: Some("DELETING".to_string()),
                creation_time: Some(pipe.creation_time.timestamp() as f64),
                last_modified_time: Some(pipe.last_modified_time.timestamp() as f64),
            }),
            Err(e) => pipes_error_response(&e),
        }
    }

    async fn handle_start_pipe(
        &self,
        state: &Arc<tokio::sync::RwLock<PipesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_pipe_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.start_pipe(&input.name) {
            Ok(pipe) => wire::serialize_start_pipe_response(&wire::StartPipeResponse {
                arn: Some(pipe.arn.clone()),
                name: Some(pipe.name.clone()),
                creation_time: Some(pipe.creation_time.timestamp() as f64),
                last_modified_time: Some(pipe.last_modified_time.timestamp() as f64),
                current_state: Some(pipe.current_state.clone()),
                desired_state: Some(pipe.desired_state.clone()),
            }),
            Err(e) => pipes_error_response(&e),
        }
    }

    async fn handle_stop_pipe(
        &self,
        state: &Arc<tokio::sync::RwLock<PipesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_stop_pipe_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.stop_pipe(&input.name) {
            Ok(pipe) => wire::serialize_stop_pipe_response(&wire::StopPipeResponse {
                arn: Some(pipe.arn.clone()),
                name: Some(pipe.name.clone()),
                creation_time: Some(pipe.creation_time.timestamp() as f64),
                last_modified_time: Some(pipe.last_modified_time.timestamp() as f64),
                current_state: Some(pipe.current_state.clone()),
                desired_state: Some(pipe.desired_state.clone()),
            }),
            Err(e) => pipes_error_response(&e),
        }
    }

    async fn handle_update_pipe(
        &self,
        state: &Arc<tokio::sync::RwLock<PipesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_pipe_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "ValidationException", "Invalid JSON body"),
        };

        let role_arn = if input.role_arn.is_empty() {
            None
        } else {
            Some(input.role_arn.as_str())
        };
        let mut state = state.write().await;
        match state.update_pipe(
            &input.name,
            input.description.as_deref(),
            input.desired_state.as_deref(),
            input.enrichment.as_deref(),
            role_arn,
            // UpdatePipe Smithy model has no Source field; keep arg slot for parity.
            None,
            input.target.as_deref(),
        ) {
            Ok(pipe) => wire::serialize_update_pipe_response(&wire::UpdatePipeResponse {
                arn: Some(pipe.arn.clone()),
                name: Some(pipe.name.clone()),
                creation_time: Some(pipe.creation_time.timestamp() as f64),
                last_modified_time: Some(pipe.last_modified_time.timestamp() as f64),
                current_state: Some(pipe.current_state.clone()),
                desired_state: Some(pipe.desired_state.clone()),
            }),
            Err(e) => pipes_error_response(&e),
        }
    }

    async fn handle_list_pipes(
        &self,
        state: &Arc<tokio::sync::RwLock<PipesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_pipes_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let pipes = state.list_pipes();
        let entries: Vec<wire::Pipe> = pipes
            .iter()
            .map(|p| wire::Pipe {
                name: Some(p.name.clone()),
                arn: Some(p.arn.clone()),
                source: Some(p.source.clone()),
                target: Some(p.target.clone()),
                desired_state: Some(p.desired_state.clone()),
                current_state: Some(p.current_state.clone()),
                creation_time: Some(p.creation_time.timestamp() as f64),
                last_modified_time: Some(p.last_modified_time.timestamp() as f64),
                ..Default::default()
            })
            .collect();
        wire::serialize_list_pipes_response(&wire::ListPipesResponse {
            pipes: Some(entries),
            next_token: None,
        })
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<PipesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_tag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "ValidationException", "Invalid JSON body"),
        };
        if input.tags.is_empty() {
            return rest_json_error(400, "ValidationException", "Missing 'tags'");
        }

        let mut state = state.write().await;
        // Find the pipe by ARN
        let pipe = state
            .pipes
            .values_mut()
            .find(|p| p.arn == input.resource_arn);
        match pipe {
            Some(p) => {
                p.tags.extend(input.tags);
                wire::serialize_tag_resource_response(&wire::TagResourceResponse {})
            }
            None => rest_json_error(
                404,
                "NotFoundException",
                &format!("Pipe with ARN {} not found", input.resource_arn),
            ),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<PipesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        uri: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_untag_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };

        // SDKs often serialise repeating list query parameters as `?tagKeys=k1&tagKeys=k2`
        // rather than a comma-joined value. The wire deserialiser only sees the last value
        // because parse_query_string overwrites duplicate keys, so collect repeated keys
        // directly from the URI to preserve historical behaviour.
        let raw_query = uri.split('?').nth(1).unwrap_or("");
        let mut tag_keys: Vec<String> = raw_query
            .split('&')
            .filter_map(|pair| {
                let (k, v) = pair.split_once('=')?;
                if k == "tagKeys" {
                    Some(v.to_string())
                } else {
                    None
                }
            })
            .collect();
        if tag_keys.is_empty() {
            tag_keys = input.tag_keys.clone();
        }

        let mut state = state.write().await;
        let pipe = state
            .pipes
            .values_mut()
            .find(|p| p.arn == input.resource_arn);
        match pipe {
            Some(p) => {
                for key in &tag_keys {
                    p.tags.remove(key);
                }
                wire::serialize_untag_resource_response(&wire::UntagResourceResponse {})
            }
            None => rest_json_error(
                404,
                "NotFoundException",
                &format!("Pipe with ARN {} not found", input.resource_arn),
            ),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<PipesState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_tags_for_resource_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        let pipe = state.pipes.values().find(|p| p.arn == input.resource_arn);
        match pipe {
            Some(p) => {
                let tags: HashMap<String, String> = p.tags.clone();
                wire::serialize_list_tags_for_resource_response(
                    &wire::ListTagsForResourceResponse { tags: Some(tags) },
                )
            }
            None => rest_json_error(
                404,
                "NotFoundException",
                &format!("Pipe with ARN {} not found", input.resource_arn),
            ),
        }
    }
}

fn pipes_error_response(err: &PipesError) -> MockResponse {
    let (error_type, status) = match err {
        PipesError::ConflictException { .. } => ("ConflictException", 409),
        PipesError::NotFoundException { .. } => ("NotFoundException", 404),
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
