use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
};

use crate::state::{OsisError, OsisState};
use crate::views::OsisStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct OsisService {
    pub(crate) state: Arc<BackendState<OsisState>>,
    pub(crate) notifier: StateChangeNotifier<OsisStateView>,
}

impl OsisService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for OsisService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for OsisService {
    fn service_name(&self) -> &str {
        "osis"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://osis\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl OsisService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        // Expected routes (OSIS REST API):
        // POST   /2022-01-01/osis/createPipeline
        // GET    /2022-01-01/osis/getPipeline/{PipelineName}
        // DELETE /2022-01-01/osis/deletePipeline/{PipelineName}
        // GET    /2022-01-01/osis/listPipelines
        // PUT    /2022-01-01/osis/startPipeline/{PipelineName}
        // PUT    /2022-01-01/osis/stopPipeline/{PipelineName}
        // PUT    /2022-01-01/osis/updatePipeline/{PipelineName}
        // GET    /2022-01-01/osis/listTagsForResource?arn={Arn}
        // POST   /2022-01-01/osis/tagResource
        // POST   /2022-01-01/osis/untagResource

        if segments.len() < 3 || segments[0] != "2022-01-01" || segments[1] != "osis" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        match (method, segments[2], segments.len()) {
            // POST /2022-01-01/osis/createPipeline
            ("POST", "createPipeline", 3) => {
                let body: Value = match serde_json::from_slice(&request.body) {
                    Ok(v) => v,
                    Err(_) => {
                        return rest_json_error(400, "ValidationException", "Invalid JSON body");
                    }
                };
                self.handle_create_pipeline(&state, &body, account_id, &region)
                    .await
            }
            // GET /2022-01-01/osis/getPipeline/{PipelineName}
            ("GET", "getPipeline", 4) => {
                let pipeline_name = percent_decode(segments[3]);
                self.handle_get_pipeline(&state, &pipeline_name).await
            }
            // DELETE /2022-01-01/osis/deletePipeline/{PipelineName}
            ("DELETE", "deletePipeline", 4) => {
                let pipeline_name = percent_decode(segments[3]);
                self.handle_delete_pipeline(&state, &pipeline_name).await
            }
            // GET /2022-01-01/osis/listPipelines
            ("GET", "listPipelines", 3) => self.handle_list_pipelines(&state).await,
            // PUT /2022-01-01/osis/startPipeline/{PipelineName}
            ("PUT", "startPipeline", 4) => {
                let pipeline_name = percent_decode(segments[3]);
                self.handle_start_pipeline(&state, &pipeline_name).await
            }
            // PUT /2022-01-01/osis/stopPipeline/{PipelineName}
            ("PUT", "stopPipeline", 4) => {
                let pipeline_name = percent_decode(segments[3]);
                self.handle_stop_pipeline(&state, &pipeline_name).await
            }
            // PUT /2022-01-01/osis/updatePipeline/{PipelineName}
            ("PUT", "updatePipeline", 4) => {
                let pipeline_name = percent_decode(segments[3]);
                let body: Value = match serde_json::from_slice(&request.body) {
                    Ok(v) => v,
                    Err(_) => {
                        return rest_json_error(400, "ValidationException", "Invalid JSON body");
                    }
                };
                self.handle_update_pipeline(&state, &pipeline_name, &body)
                    .await
            }
            // GET /2022-01-01/osis/listTagsForResource?arn={Arn}
            ("GET", "listTagsForResource", 3) => {
                let arn = extract_query_param(&request.uri, "arn").unwrap_or_default();
                self.handle_list_tags_for_resource(&state, &arn).await
            }
            // POST /2022-01-01/osis/tagResource?arn={Arn}
            ("POST", "tagResource", 3) => {
                let arn = match extract_query_param(&request.uri, "arn") {
                    Some(a) => a,
                    None => return rest_json_error(400, "ValidationException", "Arn is required"),
                };
                let body: Value = match serde_json::from_slice(&request.body) {
                    Ok(v) => v,
                    Err(_) => {
                        return rest_json_error(400, "ValidationException", "Invalid JSON body");
                    }
                };
                self.handle_tag_resource(&state, &arn, &body).await
            }
            // POST /2022-01-01/osis/untagResource?arn={Arn}
            ("POST", "untagResource", 3) => {
                let arn = match extract_query_param(&request.uri, "arn") {
                    Some(a) => a,
                    None => return rest_json_error(400, "ValidationException", "Arn is required"),
                };
                let body: Value = match serde_json::from_slice(&request.body) {
                    Ok(v) => v,
                    Err(_) => {
                        return rest_json_error(400, "ValidationException", "Invalid JSON body");
                    }
                };
                self.handle_untag_resource(&state, &arn, &body).await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn handle_create_pipeline(
        &self,
        state: &Arc<tokio::sync::RwLock<OsisState>>,
        body: &Value,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let pipeline_name = match body.get("PipelineName").and_then(|v| v.as_str()) {
            Some(n) => n,
            None => return rest_json_error(400, "ValidationException", "PipelineName is required"),
        };

        let min_units = body.get("MinUnits").and_then(|v| v.as_i64()).unwrap_or(1) as i32;

        let max_units = body.get("MaxUnits").and_then(|v| v.as_i64()).unwrap_or(1) as i32;

        let pipeline_configuration_body = body
            .get("PipelineConfigurationBody")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let initial_tags: std::collections::HashMap<String, String> = body
            .get("Tags")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|t| {
                        let key = t.get("Key").and_then(|v| v.as_str())?;
                        let value = t.get("Value").and_then(|v| v.as_str())?;
                        Some((key.to_string(), value.to_string()))
                    })
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.create_pipeline(
            pipeline_name,
            min_units,
            max_units,
            pipeline_configuration_body,
            account_id,
            region,
            initial_tags,
        ) {
            Ok(pipeline) => {
                let resp = wire::CreatePipelineResponse {
                    pipeline: Some(pipeline_to_wire(pipeline)),
                };
                wire::serialize_create_pipeline_response(&resp)
            }
            Err(e) => osis_error_response(&e),
        }
    }

    async fn handle_get_pipeline(
        &self,
        state: &Arc<tokio::sync::RwLock<OsisState>>,
        pipeline_name: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.get_pipeline(pipeline_name) {
            Ok(pipeline) => {
                let resp = wire::GetPipelineResponse {
                    pipeline: Some(pipeline_to_wire(pipeline)),
                };
                wire::serialize_get_pipeline_response(&resp)
            }
            Err(e) => osis_error_response(&e),
        }
    }

    async fn handle_delete_pipeline(
        &self,
        state: &Arc<tokio::sync::RwLock<OsisState>>,
        pipeline_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.delete_pipeline(pipeline_name) {
            Ok(()) => {
                let resp = wire::DeletePipelineResponse {};
                wire::serialize_delete_pipeline_response(&resp)
            }
            Err(e) => osis_error_response(&e),
        }
    }

    async fn handle_list_pipelines(
        &self,
        state: &Arc<tokio::sync::RwLock<OsisState>>,
    ) -> MockResponse {
        let state = state.read().await;
        let pipelines = state.list_pipelines();
        let summaries: Vec<wire::PipelineSummary> =
            pipelines.iter().map(|p| pipeline_to_summary(p)).collect();
        let resp = wire::ListPipelinesResponse {
            next_token: None,
            pipelines: Some(summaries),
        };
        wire::serialize_list_pipelines_response(&resp)
    }

    async fn handle_start_pipeline(
        &self,
        state: &Arc<tokio::sync::RwLock<OsisState>>,
        pipeline_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.start_pipeline(pipeline_name) {
            Ok(pipeline) => {
                let resp = wire::StartPipelineResponse {
                    pipeline: Some(pipeline_to_wire(pipeline)),
                };
                wire::serialize_start_pipeline_response(&resp)
            }
            Err(e) => osis_error_response(&e),
        }
    }

    async fn handle_stop_pipeline(
        &self,
        state: &Arc<tokio::sync::RwLock<OsisState>>,
        pipeline_name: &str,
    ) -> MockResponse {
        let mut state = state.write().await;
        match state.stop_pipeline(pipeline_name) {
            Ok(pipeline) => {
                let resp = wire::StopPipelineResponse {
                    pipeline: Some(pipeline_to_wire(pipeline)),
                };
                wire::serialize_stop_pipeline_response(&resp)
            }
            Err(e) => osis_error_response(&e),
        }
    }

    async fn handle_update_pipeline(
        &self,
        state: &Arc<tokio::sync::RwLock<OsisState>>,
        pipeline_name: &str,
        body: &Value,
    ) -> MockResponse {
        let min_units = body
            .get("MinUnits")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let max_units = body
            .get("MaxUnits")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let pipeline_configuration_body = body
            .get("PipelineConfigurationBody")
            .and_then(|v| v.as_str());

        let mut state = state.write().await;
        match state.update_pipeline(
            pipeline_name,
            min_units,
            max_units,
            pipeline_configuration_body,
        ) {
            Ok(pipeline) => {
                let resp = wire::UpdatePipelineResponse {
                    pipeline: Some(pipeline_to_wire(pipeline)),
                };
                wire::serialize_update_pipeline_response(&resp)
            }
            Err(e) => osis_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<OsisState>>,
        arn: &str,
    ) -> MockResponse {
        let state = state.read().await;
        match state.list_tags_for_resource(arn) {
            Ok(tags) => {
                let wire_tags: Vec<wire::Tag> = tags
                    .iter()
                    .map(|(k, v)| wire::Tag {
                        key: k.clone(),
                        value: v.clone(),
                    })
                    .collect();
                let resp = wire::ListTagsForResourceResponse {
                    tags: Some(wire_tags),
                };
                wire::serialize_list_tags_for_resource_response(&resp)
            }
            Err(e) => osis_error_response(&e),
        }
    }

    async fn handle_tag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<OsisState>>,
        arn: &str,
        body: &Value,
    ) -> MockResponse {
        let tags: std::collections::HashMap<String, String> = body
            .get("Tags")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|t| {
                        let key = t.get("Key").and_then(|v| v.as_str())?;
                        let value = t.get("Value").and_then(|v| v.as_str())?;
                        Some((key.to_string(), value.to_string()))
                    })
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.tag_resource(arn, tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => osis_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        state: &Arc<tokio::sync::RwLock<OsisState>>,
        arn: &str,
        body: &Value,
    ) -> MockResponse {
        let tag_keys: Vec<String> = body
            .get("TagKeys")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let mut state = state.write().await;
        match state.untag_resource(arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => osis_error_response(&e),
        }
    }
}

/// Convert a state `Pipeline` to the wire `Pipeline` model.
fn pipeline_to_wire(pipeline: &crate::types::Pipeline) -> wire::Pipeline {
    let tags: Vec<wire::Tag> = pipeline
        .tags
        .iter()
        .map(|(k, v)| wire::Tag {
            key: k.clone(),
            value: v.clone(),
        })
        .collect();
    wire::Pipeline {
        pipeline_name: Some(pipeline.pipeline_name.clone()),
        pipeline_arn: Some(pipeline.pipeline_arn.clone()),
        min_units: Some(pipeline.min_units),
        max_units: Some(pipeline.max_units),
        status: Some(pipeline.status.clone()),
        pipeline_configuration_body: Some(pipeline.pipeline_configuration_body.clone()),
        created_at: Some(pipeline.created_at.timestamp() as f64),
        last_updated_at: Some(pipeline.last_updated_at.timestamp() as f64),
        ingest_endpoint_urls: Some(pipeline.ingest_endpoint_urls.clone()),
        tags: if tags.is_empty() { None } else { Some(tags) },
        ..Default::default()
    }
}

/// Convert a state `Pipeline` to the wire `PipelineSummary` model.
fn pipeline_to_summary(pipeline: &crate::types::Pipeline) -> wire::PipelineSummary {
    wire::PipelineSummary {
        pipeline_name: Some(pipeline.pipeline_name.clone()),
        pipeline_arn: Some(pipeline.pipeline_arn.clone()),
        status: Some(pipeline.status.clone()),
        min_units: Some(pipeline.min_units),
        max_units: Some(pipeline.max_units),
        created_at: Some(pipeline.created_at.timestamp() as f64),
        last_updated_at: Some(pipeline.last_updated_at.timestamp() as f64),
        ..Default::default()
    }
}

fn extract_query_param(uri: &str, key: &str) -> Option<String> {
    let query = uri.split('?').nth(1)?;
    for pair in query.split('&') {
        let mut kv = pair.splitn(2, '=');
        let k = kv.next()?;
        let v = kv.next().unwrap_or("");
        if k == key {
            return Some(percent_decode(v));
        }
    }
    None
}

fn extract_path(uri: &str) -> String {
    // Delegate to the shared core helper, which correctly strips the scheme
    // and host (including custom-endpoint hostnames like `127.0.0.1:PORT`)
    // before returning the path. The previous implementation only matched on
    // `amazonaws.com` and returned the entire URI for non-AWS endpoints,
    // causing dispatch to fail with 404 against the in-process mock server.
    winterbaume_core::extract_path(uri)
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

fn osis_error_response(err: &OsisError) -> MockResponse {
    let (status, error_type) = match err {
        OsisError::PipelineAlreadyExists { .. } => (409, "ResourceAlreadyExistsException"),
        OsisError::PipelineNotFound { .. } => (404, "ResourceNotFoundException"),
        OsisError::ResourceNotFound { .. } => (404, "ResourceNotFoundException"),
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
