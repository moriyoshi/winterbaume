use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id,
};

use crate::state::{SageMakerRuntimeError, SageMakerRuntimeState};
use crate::views::SageMakerRuntimeStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct SageMakerRuntimeService {
    pub(crate) state: Arc<BackendState<SageMakerRuntimeState>>,
    pub(crate) notifier: StateChangeNotifier<SageMakerRuntimeStateView>,
    /// Optional shared `winterbaume-sagemaker` state. When wired,
    /// `InvokeEndpoint*` calls resolve the request's `EndpointName`
    /// against the parent crate's `endpoints` map and reject unknown
    /// names with a `ValidationError` ( "Endpoint X of account Y not
    /// found" ), matching real AWS behaviour. When `None` ( the default
    /// for `Self::new()` ), the legacy auto-create-on-first-invocation
    /// behaviour is preserved so unit tests that exercise only the
    /// runtime crate still work without standing up the control plane.
    pub(crate) sagemaker_state: Option<Arc<BackendState<winterbaume_sagemaker::SageMakerState>>>,
}

impl SageMakerRuntimeService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
            sagemaker_state: None,
        }
    }

    /// Construct a `SageMakerRuntimeService` that validates
    /// `InvokeEndpoint*` calls against the parent
    /// `winterbaume-sagemaker` state.
    ///
    /// Pass the same `Arc` returned by
    /// [`winterbaume_sagemaker::SageMakerService::shared_state`] so the
    /// control plane ( `winterbaume-sagemaker`'s `CreateModel` +
    /// `CreateEndpointConfig` + `CreateEndpoint` ) and the data plane
    /// ( `winterbaume-sagemakerruntime`'s `InvokeEndpoint*` ) agree on
    /// which endpoints exist:
    ///
    /// ```no_run
    /// use winterbaume_core::MockAws;
    /// use winterbaume_sagemaker::SageMakerService;
    /// use winterbaume_sagemakerruntime::SageMakerRuntimeService;
    ///
    /// let sagemaker = SageMakerService::new();
    /// let runtime = SageMakerRuntimeService::with_sagemaker_state(
    ///     sagemaker.shared_state(),
    /// );
    /// let _mock = MockAws::builder()
    ///     .with_service(sagemaker)
    ///     .with_service(runtime)
    ///     .build();
    /// ```
    pub fn with_sagemaker_state(
        sagemaker_state: Arc<BackendState<winterbaume_sagemaker::SageMakerState>>,
    ) -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
            sagemaker_state: Some(sagemaker_state),
        }
    }
}

impl Default for SageMakerRuntimeService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for SageMakerRuntimeService {
    fn service_name(&self) -> &str {
        "sagemaker"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![r"https?://runtime\.sagemaker\.(.+)\.amazonaws\.com"]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl SageMakerRuntimeService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        // SageMaker Runtime routes:
        // POST /endpoints/{EndpointName}/invocations - InvokeEndpoint
        // POST /endpoints/{EndpointName}/async-invocations - InvokeEndpointAsync
        // POST /endpoints/{EndpointName}/invocations-response-stream - InvokeEndpointWithResponseStream

        if segments.len() < 3 || segments[0] != "endpoints" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        let endpoint_name = percent_decode(segments[1]);

        // When the parent `winterbaume-sagemaker` state is wired, every
        // invocation must reference an endpoint that exists on the
        // control plane; real AWS rejects unknown endpoints with
        // `ValidationError` ( "Endpoint X of account Y not found" ).
        // When `sagemaker_state` is `None`, the legacy
        // auto-create-on-first-invocation path is preserved for
        // backward compatibility with unit tests that exercise only the
        // runtime crate.
        if let Some(sagemaker_state) = self.sagemaker_state.as_ref() {
            let sm_state_arc = sagemaker_state.get(account_id, &region);
            let sm_state = sm_state_arc.read().await;
            if !sm_state.endpoints.contains_key(&endpoint_name) {
                return rest_json_error(
                    400,
                    "ValidationError",
                    &format!("Endpoint {endpoint_name} of account {account_id} not found"),
                );
            }
        }

        let response = match (method, segments[2]) {
            // POST /endpoints/{EndpointName}/invocations
            ("POST", "invocations") => {
                self.handle_invoke_endpoint(&state, &endpoint_name, &request)
                    .await
            }
            // POST /endpoints/{EndpointName}/async-invocations
            ("POST", "async-invocations") => {
                self.handle_invoke_endpoint_async(&state, &endpoint_name, &request)
                    .await
            }
            // POST /endpoints/{EndpointName}/invocations-response-stream
            ("POST", "invocations-response-stream") => {
                self.handle_invoke_endpoint_with_response_stream(&state, &endpoint_name, &request)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_invoke_endpoint(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerRuntimeState>>,
        endpoint_name: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let content_type = request
            .headers
            .get(http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let accept = request
            .headers
            .get(http::header::ACCEPT)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let custom_attributes = get_header(&request.headers, "x-amzn-sagemaker-custom-attributes");
        let target_model = get_header(&request.headers, "x-amzn-sagemaker-target-model");
        let target_variant = get_header(&request.headers, "x-amzn-sagemaker-target-variant");
        let target_container_hostname = get_header(
            &request.headers,
            "x-amzn-sagemaker-target-container-hostname",
        );
        let inference_id = get_header(&request.headers, "x-amzn-sagemaker-inference-id");
        let inference_component_name =
            get_header(&request.headers, "x-amzn-sagemaker-inference-component");

        let mut state = state.write().await;
        match state.invoke_endpoint(
            endpoint_name,
            content_type.clone(),
            accept,
            custom_attributes.clone(),
            target_model,
            target_variant,
            target_container_hostname,
            inference_id,
            inference_component_name,
        ) {
            Ok(_record) => {
                let output = wire::InvokeEndpointOutput {
                    body: None,
                    content_type: content_type.clone(),
                    custom_attributes: custom_attributes.clone(),
                    invoked_production_variant: Some("AllTraffic".to_string()),
                    ..Default::default()
                };
                let mut resp = wire::serialize_invoke_endpoint_response(&output);
                // Override Content-Type to match the request content type (the
                // wire serializer sets application/json by default).
                let response_content_type = content_type.as_deref().unwrap_or("application/json");
                resp.headers.insert(
                    http::header::CONTENT_TYPE,
                    response_content_type.parse().unwrap(),
                );
                if let Some(attrs) = custom_attributes {
                    resp.headers.insert(
                        HeaderName::from_static("x-amzn-sagemaker-custom-attributes"),
                        attrs.parse().unwrap(),
                    );
                }
                resp.headers.insert(
                    HeaderName::from_static("x-amzn-invoked-production-variant"),
                    "AllTraffic".parse().unwrap(),
                );
                // Clear body — sync invoke returns an empty body in our mock.
                resp.body = Vec::<u8>::new().into();
                resp
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_invoke_endpoint_async(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerRuntimeState>>,
        endpoint_name: &str,
        request: &MockRequest,
    ) -> MockResponse {
        let content_type = request
            .headers
            .get(http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let accept = request
            .headers
            .get(http::header::ACCEPT)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let custom_attributes = get_header(&request.headers, "x-amzn-sagemaker-custom-attributes");
        let inference_id = get_header(&request.headers, "x-amzn-sagemaker-inference-id");
        let input_location = get_header(&request.headers, "x-amzn-sagemaker-inputlocation");

        let mut state = state.write().await;
        match state.invoke_endpoint_async(
            endpoint_name,
            content_type,
            accept,
            custom_attributes,
            inference_id,
            input_location,
        ) {
            Ok(record) => {
                let output = wire::InvokeEndpointAsyncOutput {
                    output_location: Some(record.output_location.clone()),
                    inference_id: record.inference_id.clone(),
                    failure_location: None,
                };
                let mut resp = wire::serialize_invoke_endpoint_async_response(&output);
                resp.headers.insert(
                    HeaderName::from_static("x-amzn-sagemaker-outputlocation"),
                    record.output_location.parse().unwrap(),
                );
                resp
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }

    async fn handle_invoke_endpoint_with_response_stream(
        &self,
        state: &Arc<tokio::sync::RwLock<SageMakerRuntimeState>>,
        endpoint_name: &str,
        request: &MockRequest,
    ) -> MockResponse {
        // For the mock, we treat this similarly to InvokeEndpoint
        // but return the response as if it were a single-chunk stream response.
        let content_type = request
            .headers
            .get(http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let accept = request
            .headers
            .get(http::header::ACCEPT)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let custom_attributes = get_header(&request.headers, "x-amzn-sagemaker-custom-attributes");
        let target_model = get_header(&request.headers, "x-amzn-sagemaker-target-model");
        let target_variant = get_header(&request.headers, "x-amzn-sagemaker-target-variant");
        let target_container_hostname = get_header(
            &request.headers,
            "x-amzn-sagemaker-target-container-hostname",
        );
        let inference_id = get_header(&request.headers, "x-amzn-sagemaker-inference-id");
        let inference_component_name =
            get_header(&request.headers, "x-amzn-sagemaker-inference-component");

        let mut state = state.write().await;
        match state.invoke_endpoint(
            endpoint_name,
            content_type.clone(),
            accept,
            custom_attributes.clone(),
            target_model,
            target_variant,
            target_container_hostname,
            inference_id,
            inference_component_name,
        ) {
            Ok(_record) => {
                let response_content_type = content_type.as_deref().unwrap_or("application/json");
                let output = wire::InvokeEndpointWithResponseStreamOutput {
                    body: None,
                    content_type: content_type.clone(),
                    custom_attributes: custom_attributes.clone(),
                    invoked_production_variant: Some("AllTraffic".to_string()),
                };
                let mut resp =
                    wire::serialize_invoke_endpoint_with_response_stream_response(&output);
                // The SDK deserializes ContentType from X-Amzn-SageMaker-Content-Type for
                // the streaming operation (not the standard Content-Type header).
                resp.headers.insert(
                    HeaderName::from_static("x-amzn-sagemaker-content-type"),
                    response_content_type.parse().unwrap(),
                );
                if let Some(attrs) = custom_attributes {
                    resp.headers.insert(
                        HeaderName::from_static("x-amzn-sagemaker-custom-attributes"),
                        attrs.parse().unwrap(),
                    );
                }
                resp.headers.insert(
                    HeaderName::from_static("x-amzn-invoked-production-variant"),
                    "AllTraffic".parse().unwrap(),
                );
                // Return an empty body for the streaming response.  The SDK's
                // EventStream deserializer tries to read an `initial-response` message
                // from the body before returning to the caller.  The mock does not
                // produce real EventStream-framed data, so returning an empty body lets
                // `try_recv_initial_response` return `None` (no initial message) and
                // subsequent `recv()` calls will immediately return `None` (stream
                // ended).  Headers still carry the meaningful metadata.
                resp.body = Vec::<u8>::new().into();
                resp
            }
            Err(e) => sagemaker_error_response(&e),
        }
    }
}

fn get_header(headers: &http::HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

fn extract_path(uri: &str) -> String {
    if let Some(idx) = uri.find("amazonaws.com") {
        let after_host = &uri[idx + "amazonaws.com".len()..];
        if let Some(q) = after_host.find('?') {
            after_host[..q].to_string()
        } else {
            after_host.to_string()
        }
    } else {
        uri.split('?').next().unwrap_or(uri).to_string()
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

fn sagemaker_error_response(err: &SageMakerRuntimeError) -> MockResponse {
    let (status, error_type) = match err {
        SageMakerRuntimeError::ModelError {
            model_id: _,
            status,
            ..
        } => (*status, "ModelError"),
        SageMakerRuntimeError::ModelNotReady { .. } => (429, "ModelNotReadyException"),
        SageMakerRuntimeError::ValidationError { .. } => (400, "ValidationError"),
        SageMakerRuntimeError::InternalFailure => (500, "InternalFailure"),
        SageMakerRuntimeError::ServiceUnavailable => (503, "ServiceUnavailable"),
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
